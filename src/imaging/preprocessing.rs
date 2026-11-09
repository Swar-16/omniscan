//! Grayscale conversion, Otsu thresholding, and adaptive Gaussian thresholding engines.
//!
//! All image-processing algorithms use native OpenCV through isolated internal modules.
//! Neither `cv::Mat` nor raw OpenCV types or errors are exposed across this boundary.
//!
//! # Grayscale Conversion
//! Converts color buffers (`PixelFormat::Rgb8` or `PixelFormat::Bgr8`) to `GrayImage` using
//! the ITU-R BT.601 standard luma formula:
//!
//! $$Y = 0.299 \times R + 0.587 \times G + 0.114 \times B$$
//!
//! With OpenCV's integer fixed-point approximation:
//!
//! $$Y = \left\lfloor\frac{299 \times R + 587 \times G + 114 \times B + 500}{1000}\right\rfloor$$
//!
//! Grayscale inputs (`PixelFormat::Gray8`) pass through without redundant computation.
//!
//! # Binary Polarity Convention
//! In accordance with the OMNISCAN domain contract:
//! - **Foreground = 0** (dark ink, filled pencil/pen marks).
//! - **Background = 255** (unmarked white examination sheet paper).
//!
//! # Otsu Global Thresholding
//! Evaluates the bimodal pixel histogram to automatically determine the optimal global threshold $T^*$
//! minimizing intra-class variance.
//! - Pixels $\le T^*$ are mapped to 0 (foreground mark).
//! - Pixels $> T^*$ are mapped to 255 (background paper).
//!
//! # Adaptive Gaussian Thresholding
//! Computes a spatially varying threshold for each pixel using a Gaussian-weighted sum over a
//! $(block\_size \times block\_size)$ neighborhood minus a constant $C$:
//!
//! $$T(x, y) = \text{GaussianMean}(x, y, block\_size) - C$$
//!
//! - Pixels $\le T(x, y)$ become 0 (foreground mark).
//! - Pixels $> T(x, y)$ become 255 (background paper).

use opencv::core::{Mat, Scalar, CV_8UC1, CV_8UC3};
use opencv::imgproc::{
    adaptive_threshold, cvt_color, threshold, ADAPTIVE_THRESH_GAUSSIAN_C, COLOR_BGR2GRAY,
    COLOR_RGB2GRAY, THRESH_BINARY, THRESH_OTSU,
};
use opencv::prelude::*;

use crate::error::ImageError;
use crate::imaging::buffer::{BinaryImage, GrayImage, ImageBuffer, PixelFormat};

/// Parameters for adaptive Gaussian thresholding.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AdaptiveGaussianParams {
    /// Neighborhood block size (must be odd, > 1, and <= min(width, height)).
    pub block_size: i32,
    /// Constant subtracted from the Gaussian-weighted mean.
    pub c: f64,
}

impl AdaptiveGaussianParams {
    /// Creates and validates a new set of adaptive Gaussian threshold parameters.
    ///
    /// # Errors
    /// Returns [`ImageError::InvalidAdaptiveParams`] if:
    /// - `block_size <= 1`
    /// - `block_size % 2 == 0` (block size must be odd)
    /// - `c` is NaN or infinite
    pub fn new(block_size: i32, c: f64) -> Result<Self, ImageError> {
        if block_size <= 1 {
            return Err(ImageError::InvalidAdaptiveParams {
                message: format!("Block size must be > 1, got {block_size}"),
            });
        }
        if block_size % 2 == 0 {
            return Err(ImageError::InvalidAdaptiveParams {
                message: format!("Block size must be odd, got {block_size}"),
            });
        }
        if c.is_nan() || c.is_infinite() {
            return Err(ImageError::InvalidAdaptiveParams {
                message: "Constant C cannot be NaN or infinite".to_string(),
            });
        }
        Ok(Self { block_size, c })
    }
}

impl Default for AdaptiveGaussianParams {
    /// Default parameters optimized for scanned OMR answer sheets:
    /// - `block_size`: 21 (covers typical bubble radius and margins)
    /// - `c`: 10.0 (provides robust contrast against paper texture and non-uniform lighting)
    fn default() -> Self {
        Self {
            block_size: 21,
            c: 10.0,
        }
    }
}

/// Converts any valid [`ImageBuffer`] into a [`GrayImage`].
///
/// If the input buffer is already [`PixelFormat::Gray8`], it is returned directly as a [`GrayImage`].
/// For [`PixelFormat::Rgb8`] or [`PixelFormat::Bgr8`], pixels are transformed using ITU-R BT.601
/// luma weights via OpenCV.
///
/// # Errors
/// Returns an [`ImageError`] if memory allocation fails or dimensions are invalid.
pub fn to_grayscale(buffer: &ImageBuffer) -> Result<GrayImage, ImageError> {
    let width = buffer.width();
    let height = buffer.height();
    let width_usize = width as usize;
    let height_usize = height as usize;
    let channels = buffer.channels();
    let row_bytes = width_usize * channels;
    let stride = buffer.stride();

    match buffer.format() {
        PixelFormat::Gray8 => {
            if stride == width_usize {
                GrayImage::new(width, height, buffer.as_slice().to_vec()).map_err(|e| {
                    ImageError::PreprocessingFailed(format!("Failed to wrap Gray8 buffer: {e}"))
                })
            } else {
                // If the input buffer has row padding, extract active pixels row-by-row
                let mut packed = Vec::with_capacity(width_usize * height_usize);
                let src = buffer.as_slice();
                for r in 0..height_usize {
                    let row_start = r * stride;
                    packed.extend_from_slice(&src[row_start..row_start + width_usize]);
                }
                GrayImage::new(width, height, packed).map_err(|e| {
                    ImageError::PreprocessingFailed(format!(
                        "Failed to create GrayImage from padded buffer: {e}"
                    ))
                })
            }
        }
        PixelFormat::Rgb8 | PixelFormat::Bgr8 => {
            let mut src_mat = Mat::new_rows_cols_with_default(
                height as i32,
                width as i32,
                CV_8UC3,
                Scalar::all(0.0),
            )
            .map_err(|e| {
                ImageError::PreprocessingFailed(format!("Failed to allocate OpenCV Mat: {e}"))
            })?;

            let dst_slice = src_mat.data_bytes_mut().map_err(|e| {
                ImageError::PreprocessingFailed(format!("Failed to access Mat data bytes: {e}"))
            })?;

            if stride == row_bytes {
                dst_slice.copy_from_slice(buffer.as_slice());
            } else {
                // Unpad row-by-row into tightly packed Mat storage
                let src = buffer.as_slice();
                for r in 0..height_usize {
                    let src_start = r * stride;
                    let dst_start = r * row_bytes;
                    dst_slice[dst_start..dst_start + row_bytes]
                        .copy_from_slice(&src[src_start..src_start + row_bytes]);
                }
            }

            let conversion_code = if buffer.format() == PixelFormat::Rgb8 {
                COLOR_RGB2GRAY
            } else {
                COLOR_BGR2GRAY
            };

            let mut gray_mat = Mat::default();
            cvt_color(&src_mat, &mut gray_mat, conversion_code, 0).map_err(|e| {
                ImageError::PreprocessingFailed(format!("OpenCV cvtColor failed: {e}"))
            })?;

            let gray_bytes = gray_mat.data_bytes().map_err(|e| {
                ImageError::PreprocessingFailed(format!("Failed to read gray bytes: {e}"))
            })?;

            GrayImage::new(width, height, gray_bytes.to_vec()).map_err(|e| {
                ImageError::PreprocessingFailed(format!("Failed to create GrayImage: {e}"))
            })
        }
    }
}

/// Applies Otsu global thresholding to an 8-bit grayscale image, returning a binary image.
///
/// Enforces the canonical OMNISCAN polarity:
/// - **Foreground (dark mark) = 0**
/// - **Background (paper sheet) = 255**
pub fn otsu_threshold(gray: &GrayImage) -> Result<BinaryImage, ImageError> {
    otsu_threshold_with_level(gray).map(|(bin, _)| bin)
}

/// Applies Otsu global thresholding and returns both the resulting [`BinaryImage`] and the
/// optimal threshold value computed by the algorithm.
///
/// # Degenerate Histograms
/// - **Uniform all-white (255):** Computed threshold is 0.0; all pixels become 255 (background).
/// - **Uniform all-black (0):** Computed threshold is 0.0; all pixels become 0 (foreground).
/// - **Uniform gray (e.g. 128):** Computed threshold is 0.0; all pixels become 255 (background).
pub fn otsu_threshold_with_level(gray: &GrayImage) -> Result<(BinaryImage, f64), ImageError> {
    let width = gray.width();
    let height = gray.height();

    let mut src_mat =
        Mat::new_rows_cols_with_default(height as i32, width as i32, CV_8UC1, Scalar::all(0.0))
            .map_err(|e| ImageError::PreprocessingFailed(format!("Failed to allocate Mat: {e}")))?;

    src_mat
        .data_bytes_mut()
        .map_err(|e| ImageError::PreprocessingFailed(format!("Failed to access Mat data: {e}")))?
        .copy_from_slice(gray.as_slice());

    let mut dst_mat = Mat::default();
    let computed_threshold = threshold(
        &src_mat,
        &mut dst_mat,
        0.0,
        255.0,
        THRESH_BINARY | THRESH_OTSU,
    )
    .map_err(|e| ImageError::PreprocessingFailed(format!("OpenCV Otsu threshold failed: {e}")))?;

    let bin_bytes = dst_mat.data_bytes().map_err(|e| {
        ImageError::PreprocessingFailed(format!("Failed to read binary bytes: {e}"))
    })?;

    let binary_image = BinaryImage::new(width, height, bin_bytes.to_vec()).map_err(|e| {
        ImageError::PreprocessingFailed(format!("Binary image validation failed: {e}"))
    })?;

    Ok((binary_image, computed_threshold))
}

/// Applies adaptive Gaussian thresholding to an 8-bit grayscale image.
///
/// Enforces the canonical OMNISCAN polarity:
/// - **Foreground (dark mark) = 0**
/// - **Background (paper sheet) = 255**
///
/// # Errors
/// Returns [`ImageError::InvalidAdaptiveParams`] if:
/// - `params.block_size` exceeds `min(width, height)`.
/// - `params.block_size <= 1` or is even.
pub fn adaptive_gaussian_threshold(
    gray: &GrayImage,
    params: &AdaptiveGaussianParams,
) -> Result<BinaryImage, ImageError> {
    let width = gray.width();
    let height = gray.height();
    let min_dim = width.min(height) as i32;

    if params.block_size > min_dim {
        return Err(ImageError::InvalidAdaptiveParams {
            message: format!(
                "Adaptive threshold block size ({}) exceeds minimum image dimension ({min_dim}) for image {}x{}",
                params.block_size, width, height
            ),
        });
    }

    let mut src_mat =
        Mat::new_rows_cols_with_default(height as i32, width as i32, CV_8UC1, Scalar::all(0.0))
            .map_err(|e| ImageError::PreprocessingFailed(format!("Failed to allocate Mat: {e}")))?;

    src_mat
        .data_bytes_mut()
        .map_err(|e| ImageError::PreprocessingFailed(format!("Failed to access Mat data: {e}")))?
        .copy_from_slice(gray.as_slice());

    let mut dst_mat = Mat::default();
    adaptive_threshold(
        &src_mat,
        &mut dst_mat,
        255.0,
        ADAPTIVE_THRESH_GAUSSIAN_C,
        THRESH_BINARY,
        params.block_size,
        params.c,
    )
    .map_err(|e| {
        ImageError::PreprocessingFailed(format!("OpenCV adaptive threshold failed: {e}"))
    })?;

    let bin_bytes = dst_mat.data_bytes().map_err(|e| {
        ImageError::PreprocessingFailed(format!("Failed to read binary bytes: {e}"))
    })?;

    BinaryImage::new(width, height, bin_bytes.to_vec()).map_err(|e| {
        ImageError::PreprocessingFailed(format!("Binary image validation failed: {e}"))
    })
}

/// Applies adaptive Gaussian thresholding using the default parameters.
pub fn adaptive_gaussian_threshold_default(gray: &GrayImage) -> Result<BinaryImage, ImageError> {
    adaptive_gaussian_threshold(gray, &AdaptiveGaussianParams::default())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_grayscale_primary_colors_rgb() {
        // Red: [255, 0, 0] -> BT.601 luma ~ 76
        let red_buf = ImageBuffer::new(1, 1, PixelFormat::Rgb8, vec![255, 0, 0]).unwrap();
        let red_gray = to_grayscale(&red_buf).unwrap();
        assert_eq!(red_gray.as_slice()[0], 76);

        // Green: [0, 255, 0] -> BT.601 luma ~ 150
        let green_buf = ImageBuffer::new(1, 1, PixelFormat::Rgb8, vec![0, 255, 0]).unwrap();
        let green_gray = to_grayscale(&green_buf).unwrap();
        assert_eq!(green_gray.as_slice()[0], 150);

        // Blue: [0, 0, 255] -> BT.601 luma ~ 29
        let blue_buf = ImageBuffer::new(1, 1, PixelFormat::Rgb8, vec![0, 0, 255]).unwrap();
        let blue_gray = to_grayscale(&blue_buf).unwrap();
        assert_eq!(blue_gray.as_slice()[0], 29);

        // White: [255, 255, 255] -> 255
        let white_buf = ImageBuffer::new(1, 1, PixelFormat::Rgb8, vec![255, 255, 255]).unwrap();
        let white_gray = to_grayscale(&white_buf).unwrap();
        assert_eq!(white_gray.as_slice()[0], 255);

        // Black: [0, 0, 0] -> 0
        let black_buf = ImageBuffer::new(1, 1, PixelFormat::Rgb8, vec![0, 0, 0]).unwrap();
        let black_gray = to_grayscale(&black_buf).unwrap();
        assert_eq!(black_gray.as_slice()[0], 0);

        // Gray: [128, 128, 128] -> 128
        let gray_buf = ImageBuffer::new(1, 1, PixelFormat::Rgb8, vec![128, 128, 128]).unwrap();
        let gray_gray = to_grayscale(&gray_buf).unwrap();
        assert_eq!(gray_gray.as_slice()[0], 128);
    }

    #[test]
    fn test_to_grayscale_bgr_ordering() {
        // In BGR, [255, 0, 0] is Pure Blue -> BT.601 luma ~ 29
        let bgr_blue = ImageBuffer::new(1, 1, PixelFormat::Bgr8, vec![255, 0, 0]).unwrap();
        let gray_blue = to_grayscale(&bgr_blue).unwrap();
        assert_eq!(gray_blue.as_slice()[0], 29);

        // In BGR, [0, 0, 255] is Pure Red -> BT.601 luma ~ 76
        let bgr_red = ImageBuffer::new(1, 1, PixelFormat::Bgr8, vec![0, 0, 255]).unwrap();
        let gray_red = to_grayscale(&bgr_red).unwrap();
        assert_eq!(gray_red.as_slice()[0], 76);
    }

    #[test]
    fn test_to_grayscale_gray8_passthrough() {
        let original_data = vec![10, 20, 30, 40, 50, 60];
        let gray_buf = ImageBuffer::new(3, 2, PixelFormat::Gray8, original_data.clone()).unwrap();
        let gray_img = to_grayscale(&gray_buf).unwrap();

        assert_eq!(gray_img.width(), 3);
        assert_eq!(gray_img.height(), 2);
        assert_eq!(gray_img.as_slice(), original_data.as_slice());
    }

    #[test]
    fn test_otsu_threshold_bimodal_polarity() {
        // 4x4 image: left half dark marks (30), right half clean paper (220)
        let mut pixels = vec![0u8; 16];
        for r in 0..4 {
            for c in 0..4 {
                pixels[r * 4 + c] = if c < 2 { 30 } else { 220 };
            }
        }
        let gray = GrayImage::new(4, 4, pixels).unwrap();
        let (binary, level) = otsu_threshold_with_level(&gray).unwrap();

        assert!((30.0..220.0).contains(&level));
        assert_eq!(binary.width(), 4);
        assert_eq!(binary.height(), 4);

        // Left half (30 <= level) must be 0 (foreground mark)
        assert_eq!(binary.get_pixel(0, 0), Some(0));
        assert_eq!(binary.get_pixel(1, 3), Some(0));
        // Right half (220 > level) must be 255 (background paper)
        assert_eq!(binary.get_pixel(2, 0), Some(255));
        assert_eq!(binary.get_pixel(3, 3), Some(255));

        assert!(binary.is_foreground(0, 0).unwrap());
        assert!(binary.is_background(2, 0).unwrap());
    }

    #[test]
    fn test_otsu_degenerate_all_white() {
        let gray = GrayImage::new(4, 4, vec![255; 16]).unwrap();
        let (binary, level) = otsu_threshold_with_level(&gray).unwrap();
        assert_eq!(level, 0.0);
        // All pixels 255 > 0 -> all become 255 (background)
        assert!(binary.as_slice().iter().all(|&b| b == 255));
    }

    #[test]
    fn test_otsu_degenerate_all_black() {
        let gray = GrayImage::new(4, 4, vec![0; 16]).unwrap();
        let (binary, level) = otsu_threshold_with_level(&gray).unwrap();
        assert_eq!(level, 0.0);
        // All pixels 0 <= 0 -> all become 0 (foreground)
        assert!(binary.as_slice().iter().all(|&b| b == 0));
    }

    #[test]
    fn test_otsu_degenerate_uniform_gray() {
        let gray = GrayImage::new(4, 4, vec![128; 16]).unwrap();
        let (binary, level) = otsu_threshold_with_level(&gray).unwrap();
        assert_eq!(level, 0.0);
        // All pixels 128 > 0 -> all become 255 (background)
        assert!(binary.as_slice().iter().all(|&b| b == 255));
    }

    #[test]
    fn test_otsu_synthetic_circular_mark_sheet() {
        // 10x10 sheet: white background (240), dark circular mark of radius 2 at center (5,5) with gray 30
        let mut pixels = vec![240u8; 100];
        let cx = 5.0f64;
        let cy = 5.0f64;
        let r_mark = 2.0f64;

        for y in 0..10 {
            for x in 0..10 {
                let dist = ((x as f64 - cx).powi(2) + (y as f64 - cy).powi(2)).sqrt();
                if dist <= r_mark {
                    pixels[y * 10 + x] = 30;
                }
            }
        }

        let gray = GrayImage::new(10, 10, pixels).unwrap();
        let binary = otsu_threshold(&gray).unwrap();

        // Center must be 0 (dark mark foreground)
        assert_eq!(binary.get_pixel(5, 5), Some(0));
        assert!(binary.is_foreground(5, 5).unwrap());

        // Corner must be 255 (background paper)
        assert_eq!(binary.get_pixel(0, 0), Some(255));
        assert!(binary.is_background(0, 0).unwrap());
    }

    #[test]
    fn test_adaptive_gaussian_params_validation() {
        // Block size <= 1 rejected
        let err1 = AdaptiveGaussianParams::new(1, 5.0).unwrap_err();
        assert!(matches!(err1, ImageError::InvalidAdaptiveParams { .. }));

        let err_neg = AdaptiveGaussianParams::new(-3, 5.0).unwrap_err();
        assert!(matches!(err_neg, ImageError::InvalidAdaptiveParams { .. }));

        // Even block size rejected
        let err_even = AdaptiveGaussianParams::new(4, 5.0).unwrap_err();
        assert!(matches!(err_even, ImageError::InvalidAdaptiveParams { .. }));

        // NaN or infinite C rejected
        let err_nan = AdaptiveGaussianParams::new(5, f64::NAN).unwrap_err();
        assert!(matches!(err_nan, ImageError::InvalidAdaptiveParams { .. }));

        let err_inf = AdaptiveGaussianParams::new(5, f64::INFINITY).unwrap_err();
        assert!(matches!(err_inf, ImageError::InvalidAdaptiveParams { .. }));

        // Valid odd block size >= 3 accepted
        let valid = AdaptiveGaussianParams::new(5, 2.5).unwrap();
        assert_eq!(valid.block_size, 5);
        assert_eq!(valid.c, 2.5);
    }

    #[test]
    fn test_adaptive_gaussian_image_too_small_rejection() {
        // 4x4 image with block size 7 (> 4)
        let gray = GrayImage::new(4, 4, vec![100; 16]).unwrap();
        let params = AdaptiveGaussianParams::new(7, 2.0).unwrap();
        let err = adaptive_gaussian_threshold(&gray, &params).unwrap_err();
        assert!(matches!(err, ImageError::InvalidAdaptiveParams { .. }));
        assert!(err.to_string().contains("exceeds minimum image dimension"));
    }

    #[test]
    fn test_adaptive_gaussian_polarity_and_gradient() {
        // 11x11 image with illumination gradient across rows (100 to 200)
        // plus a localized dark mark at (5, 5)
        let mut pixels = vec![0u8; 121];
        for y in 0..11 {
            for x in 0..11 {
                // Background gradient
                let bg = 100 + (y * 10) as u8;
                pixels[y * 11 + x] = if x == 5 && y == 5 {
                    // Dark mark significantly darker than local neighborhood
                    20
                } else {
                    bg
                };
            }
        }

        let gray = GrayImage::new(11, 11, pixels).unwrap();
        let params = AdaptiveGaussianParams::new(5, 5.0).unwrap();
        let binary = adaptive_gaussian_threshold(&gray, &params).unwrap();

        // Dark mark at (5,5) should be detected as 0 (foreground)
        assert_eq!(binary.get_pixel(5, 5), Some(0));
        assert!(binary.is_foreground(5, 5).unwrap());

        // Background area should be 255
        assert_eq!(binary.get_pixel(0, 0), Some(255));
        assert!(binary.is_background(0, 0).unwrap());
    }
}

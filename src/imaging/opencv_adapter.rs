//! Native OpenCV image decoding adapter for OMNISCAN.
//!
//! This module isolates OpenCV's decoding subsystem behind project-owned abstractions.
//! Neither `cv::Mat` nor OpenCV FFI types or errors are exposed across this boundary.
//!
//! # Supported Formats
//! - **PNG**: 8-bit grayscale, 8-bit 3-channel RGB/BGR, and 8-bit 4-channel BGRA (composited against white).
//! - **JPEG**: 8-bit grayscale, 8-bit 3-channel RGB/BGR.
//! - **TIFF**: Single-page 8-bit grayscale and 8-bit 3-channel RGB/BGR. Multi-page TIFFs are rejected.
//!
//! # Memory & Resource Bounds
//! All dimensions and allocations are validated before allocating the destination project-owned buffer:
//! - Dimensions must be `> 0` and `<= MAX_IMAGE_DIMENSION` (15,000 px).
//! - Total decompressed buffer size must be `<= MAX_IMAGE_PIXEL_BYTES` (120 MiB).
//! - All stride and allocation calculations use checked arithmetic to prevent overflow.

use std::path::Path;

use opencv::core::{Vector, CV_8U};
use opencv::imgcodecs::{imread, imreadmulti, IMREAD_UNCHANGED};
use opencv::prelude::*;

use crate::error::ImageError;
use crate::imaging::buffer::{
    BinaryImage, GrayImage, ImageBuffer, PixelFormat, MAX_IMAGE_DIMENSION, MAX_IMAGE_PIXEL_BYTES,
};

/// Color channel ordering preference for decoded color images.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ColorOrder {
    /// Standard RGB ordering (R, G, B) — default for OMNISCAN.
    #[default]
    Rgb,
    /// Native OpenCV BGR ordering (B, G, R).
    Bgr,
}

/// Checks whether the given file path has a supported image extension.
///
/// Supported extensions (case-insensitive): `.png`, `.jpg`, `.jpeg`, `.tif`, `.tiff`.
pub fn is_supported_format(path: &Path) -> bool {
    path.extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| {
            let lower = ext.to_ascii_lowercase();
            matches!(lower.as_str(), "png" | "jpg" | "jpeg" | "tif" | "tiff")
        })
        .unwrap_or(false)
}

/// Decodes an image file into a project-owned `ImageBuffer`.
///
/// For color images, channels are converted to standard RGB ordering by default.
/// Grayscale images are decoded directly into `PixelFormat::Gray8`.
///
/// # Errors
/// Returns an [`ImageError`] if:
/// - The file is not found or cannot be opened.
/// - The file format is unsupported.
/// - The image is corrupt or OpenCV fails to decode it.
/// - A multi-page TIFF is detected.
/// - The bit depth is not 8-bit unsigned (`CV_8U`).
/// - The channel count is not 1 (grayscale), 3 (color), or 4 (alpha-composited).
/// - Dimensions exceed `MAX_IMAGE_DIMENSION` (15,000 px).
/// - The required memory allocation exceeds `MAX_IMAGE_PIXEL_BYTES` (120 MiB).
pub fn decode_image(path: &Path) -> Result<ImageBuffer, ImageError> {
    decode_image_with_order(path, ColorOrder::Rgb)
}

/// Decodes an image file into a project-owned `ImageBuffer` with explicit color ordering.
pub fn decode_image_with_order(
    path: &Path,
    color_order: ColorOrder,
) -> Result<ImageBuffer, ImageError> {
    // 1. Path existence and file verification
    if !path.exists() {
        return Err(ImageError::FileNotFound {
            path: path.display().to_string(),
        });
    }

    if !path.is_file() {
        return Err(ImageError::IoError {
            path: path.display().to_string(),
            message: "Path is not a regular file".to_string(),
        });
    }

    // 2. Format extension verification
    let ext = path
        .extension()
        .and_then(|s| s.to_str())
        .map(|s| s.to_ascii_lowercase())
        .unwrap_or_default();

    if !matches!(ext.as_str(), "png" | "jpg" | "jpeg" | "tif" | "tiff") {
        return Err(ImageError::UnsupportedFormat {
            path: path.display().to_string(),
            format: if ext.is_empty() {
                "none".to_string()
            } else {
                ext
            },
        });
    }

    let path_str = path.to_str().ok_or_else(|| ImageError::IoError {
        path: path.display().to_string(),
        message: "Path contains invalid UTF-8 characters".to_string(),
    })?;

    // 3. Decoding via OpenCV
    let mat = if matches!(ext.as_str(), "tif" | "tiff") {
        // Multi-page TIFF detection
        let mut pages = Vector::<opencv::core::Mat>::new();
        let read_ok = imreadmulti(path_str, &mut pages, IMREAD_UNCHANGED).map_err(|e| {
            ImageError::DecodeFailed {
                path: path.display().to_string(),
                message: format!("OpenCV imreadmulti failed: {e}"),
            }
        })?;

        if pages.len() > 1 {
            return Err(ImageError::MultiPageNotSupported {
                path: path.display().to_string(),
                pages: pages.len(),
            });
        }

        if pages.is_empty() || !read_ok {
            return Err(ImageError::DecodeFailed {
                path: path.display().to_string(),
                message: "OpenCV failed to decode TIFF file (empty or corrupt)".to_string(),
            });
        }

        pages.get(0).map_err(|e| ImageError::DecodeFailed {
            path: path.display().to_string(),
            message: format!("Failed to read page 0 from decoded TIFF: {e}"),
        })?
    } else {
        // PNG or JPEG
        let m = imread(path_str, IMREAD_UNCHANGED).map_err(|e| ImageError::DecodeFailed {
            path: path.display().to_string(),
            message: format!("OpenCV imread failed: {e}"),
        })?;

        if m.empty() {
            return Err(ImageError::DecodeFailed {
                path: path.display().to_string(),
                message: "OpenCV failed to decode image (empty or corrupt file)".to_string(),
            });
        }
        m
    };

    // 4. Validate decoded dimensions before allocating destination memory
    let rows = mat.rows();
    let cols = mat.cols();
    if rows <= 0 || cols <= 0 {
        return Err(ImageError::DecodeFailed {
            path: path.display().to_string(),
            message: format!("Decoded image has non-positive dimensions: {cols}x{rows}"),
        });
    }

    let width = cols as u32;
    let height = rows as u32;

    if width > MAX_IMAGE_DIMENSION || height > MAX_IMAGE_DIMENSION {
        return Err(ImageError::InvalidDimensions {
            width,
            height,
            max: MAX_IMAGE_DIMENSION,
        });
    }

    // 5. Validate bit depth (must be CV_8U = 0)
    let depth = mat.depth();
    if depth != CV_8U {
        return Err(ImageError::UnsupportedDepth { depth });
    }

    // 6. Validate channel count
    let channels = mat.channels();
    if channels != 1 && channels != 3 && channels != 4 {
        return Err(ImageError::UnsupportedChannels { channels });
    }

    // 7. Determine output format, channels, and validated memory bounds
    let (target_format, target_channels) = match channels {
        1 => (PixelFormat::Gray8, 1),
        3 => match color_order {
            ColorOrder::Rgb => (PixelFormat::Rgb8, 3),
            ColorOrder::Bgr => (PixelFormat::Bgr8, 3),
        },
        4 => match color_order {
            ColorOrder::Rgb => (PixelFormat::Rgb8, 3),
            ColorOrder::Bgr => (PixelFormat::Bgr8, 3),
        },
        _ => unreachable!(),
    };

    let stride = width
        .checked_mul(target_channels as u32)
        .ok_or(ImageError::DimensionOverflow {
            width,
            height,
            channels: target_channels,
        })? as usize;

    let total_bytes = stride
        .checked_mul(height as usize)
        .ok_or(ImageError::DimensionOverflow {
            width,
            height,
            channels: target_channels,
        })?;

    if total_bytes > MAX_IMAGE_PIXEL_BYTES {
        return Err(ImageError::AllocationTooLarge {
            bytes: total_bytes,
            max: MAX_IMAGE_PIXEL_BYTES,
        });
    }

    // 8. Copy and transform pixel data into tightly-packed project-owned buffer
    let is_continuous = mat.is_continuous();
    let contiguous_mat = if is_continuous { mat } else { mat.clone() };

    let raw_bytes = contiguous_mat
        .data_bytes()
        .map_err(|e| ImageError::DecodeFailed {
            path: path.display().to_string(),
            message: format!("Failed to access decoded pixel bytes: {e}"),
        })?;

    let pixel_count = width as usize * height as usize;

    let buffer: Vec<u8> = match channels {
        1 => {
            if raw_bytes.len() < total_bytes {
                return Err(ImageError::BufferLengthMismatch {
                    expected: total_bytes,
                    actual: raw_bytes.len(),
                });
            }
            raw_bytes[..total_bytes].to_vec()
        }
        3 => {
            let src_bytes = total_bytes;
            if raw_bytes.len() < src_bytes {
                return Err(ImageError::BufferLengthMismatch {
                    expected: src_bytes,
                    actual: raw_bytes.len(),
                });
            }
            if color_order == ColorOrder::Bgr {
                raw_bytes[..total_bytes].to_vec()
            } else {
                // Convert OpenCV BGR to standard RGB
                let mut rgb = Vec::with_capacity(total_bytes);
                for chunk in raw_bytes[..total_bytes].as_chunks::<3>().0 {
                    rgb.push(chunk[2]); // R
                    rgb.push(chunk[1]); // G
                    rgb.push(chunk[0]); // B
                }
                rgb
            }
        }
        4 => {
            // 4-channel BGRA: alpha compositing against pure white (255, 255, 255)
            let src_expected = pixel_count
                .checked_mul(4)
                .ok_or(ImageError::DimensionOverflow {
                    width,
                    height,
                    channels: 4,
                })?;

            if raw_bytes.len() < src_expected {
                return Err(ImageError::BufferLengthMismatch {
                    expected: src_expected,
                    actual: raw_bytes.len(),
                });
            }

            let mut out = Vec::with_capacity(total_bytes);
            for chunk in raw_bytes[..src_expected].as_chunks::<4>().0 {
                let b_src = chunk[0] as u32;
                let g_src = chunk[1] as u32;
                let r_src = chunk[2] as u32;
                let a_src = chunk[3] as u32;
                let inv_a = 255 - a_src;

                // Integer blending against white: (Color * Alpha + 255 * (255 - Alpha) + 127) / 255
                let b_out = ((b_src * a_src + 255 * inv_a + 127) / 255) as u8;
                let g_out = ((g_src * a_src + 255 * inv_a + 127) / 255) as u8;
                let r_out = ((r_src * a_src + 255 * inv_a + 127) / 255) as u8;

                if color_order == ColorOrder::Rgb {
                    out.push(r_out);
                    out.push(g_out);
                    out.push(b_out);
                } else {
                    out.push(b_out);
                    out.push(g_out);
                    out.push(r_out);
                }
            }
            out
        }
        _ => unreachable!(),
    };

    ImageBuffer::from_raw_parts(width, height, target_format, stride, buffer).map_err(|e| {
        ImageError::DecodeFailed {
            path: path.display().to_string(),
            message: e.to_string(),
        }
    })
}

/// Decodes an image file and guarantees an 8-bit grayscale [`GrayImage`].
///
/// If the input image is a color image (3 or 4 channels), this function returns
/// [`ImageError::ExpectedGrayscale`]. Full color-to-grayscale conversion belongs
/// to subsequent preprocessing phases.
pub fn decode_gray_image(path: &Path) -> Result<GrayImage, ImageError> {
    let buffer = decode_image(path)?;
    if buffer.format() != PixelFormat::Gray8 {
        return Err(ImageError::ExpectedGrayscale {
            path: path.display().to_string(),
            channels: buffer.channels(),
        });
    }
    GrayImage::new(buffer.width(), buffer.height(), buffer.into_vec()).map_err(|e| {
        ImageError::DecodeFailed {
            path: path.display().to_string(),
            message: e.to_string(),
        }
    })
}

/// Saves a [`GrayImage`] as an 8-bit single-channel PNG file at the given path.
pub fn save_gray_image(path: &Path, gray: &GrayImage) -> Result<(), ImageError> {
    save_raw_gray_png(path, gray.width(), gray.height(), gray.as_slice())
}

/// Saves a [`BinaryImage`] as an 8-bit single-channel PNG file at the given path.
pub fn save_binary_image(path: &Path, binary: &BinaryImage) -> Result<(), ImageError> {
    save_raw_gray_png(path, binary.width(), binary.height(), binary.as_slice())
}

fn save_raw_gray_png(path: &Path, width: u32, height: u32, data: &[u8]) -> Result<(), ImageError> {
    let path_str = path.to_str().ok_or_else(|| ImageError::IoError {
        path: path.display().to_string(),
        message: "Path contains invalid UTF-8 characters".to_string(),
    })?;

    let mut mat = opencv::core::Mat::new_rows_cols_with_default(
        height as i32,
        width as i32,
        opencv::core::CV_8UC1,
        opencv::core::Scalar::all(0.0),
    )
    .map_err(|e| ImageError::PreprocessingFailed(format!("Failed to allocate Mat: {e}")))?;

    mat.data_bytes_mut()
        .map_err(|e| ImageError::PreprocessingFailed(format!("Failed to access Mat data: {e}")))?
        .copy_from_slice(data);

    opencv::imgcodecs::imwrite(path_str, &mat, &Vector::new()).map_err(|e| {
        ImageError::IoError {
            path: path.display().to_string(),
            message: format!("OpenCV imwrite failed: {e}"),
        }
    })?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use opencv::core::{Mat, Scalar, Vector, CV_16UC1, CV_8UC1, CV_8UC3, CV_8UC4};
    use opencv::imgcodecs::{imwrite, imwritemulti};
    use std::fs;
    use std::sync::atomic::{AtomicUsize, Ordering};

    static FIXTURE_COUNTER: AtomicUsize = AtomicUsize::new(0);

    struct TempFixture {
        path: std::path::PathBuf,
    }

    impl TempFixture {
        fn new(extension: &str) -> Self {
            let id = FIXTURE_COUNTER.fetch_add(1, Ordering::SeqCst);
            let dir = std::env::temp_dir().join("omniscan_phase2b_tests");
            let _ = fs::create_dir_all(&dir);
            let filename = format!("test_fixture_{}_{}.{}", std::process::id(), id, extension);
            let path = dir.join(filename);
            Self { path }
        }
    }

    impl Drop for TempFixture {
        fn drop(&mut self) {
            let _ = fs::remove_file(&self.path);
        }
    }

    #[test]
    fn test_is_supported_format() {
        assert!(is_supported_format(Path::new("sheet.png")));
        assert!(is_supported_format(Path::new("sheet.PNG")));
        assert!(is_supported_format(Path::new("sheet.jpg")));
        assert!(is_supported_format(Path::new("sheet.JPEG")));
        assert!(is_supported_format(Path::new("sheet.tif")));
        assert!(is_supported_format(Path::new("sheet.TIFF")));

        assert!(!is_supported_format(Path::new("sheet.bmp")));
        assert!(!is_supported_format(Path::new("sheet.gif")));
        assert!(!is_supported_format(Path::new("sheet.webp")));
        assert!(!is_supported_format(Path::new("sheet.pdf")));
        assert!(!is_supported_format(Path::new("sheet")));
    }

    #[test]
    fn test_missing_file_rejection() {
        let missing = Path::new("non_existent_file_omniscan_12345.png");
        let err = decode_image(missing).unwrap_err();
        assert_eq!(
            err,
            ImageError::FileNotFound {
                path: missing.display().to_string()
            }
        );
    }

    #[test]
    fn test_unsupported_format_rejection() {
        let fixture = TempFixture::new("bmp");
        fs::write(&fixture.path, b"not a supported bmp").unwrap();
        let err = decode_image(&fixture.path).unwrap_err();
        assert!(matches!(err, ImageError::UnsupportedFormat { .. }));
    }

    #[test]
    fn test_corrupt_file_rejection() {
        let fixture = TempFixture::new("png");
        fs::write(&fixture.path, b"not a valid png binary content").unwrap();
        let err = decode_image(&fixture.path).unwrap_err();
        assert!(matches!(err, ImageError::DecodeFailed { .. }));
    }

    #[test]
    fn test_decode_grayscale_png() {
        let fixture = TempFixture::new("png");
        let mat = Mat::new_rows_cols_with_default(8, 12, CV_8UC1, Scalar::all(180.0)).unwrap();
        imwrite(fixture.path.to_str().unwrap(), &mat, &Vector::new()).unwrap();

        let img = decode_image(&fixture.path).unwrap();
        assert_eq!(img.width(), 12);
        assert_eq!(img.height(), 8);
        assert_eq!(img.format(), PixelFormat::Gray8);
        assert_eq!(img.channels(), 1);
        assert_eq!(img.stride(), 12);
        assert_eq!(img.as_slice().len(), 96);
        assert!(img.as_slice().iter().all(|&b| b == 180));

        let gray = decode_gray_image(&fixture.path).unwrap();
        assert_eq!(gray.width(), 12);
        assert_eq!(gray.height(), 8);
    }

    #[test]
    fn test_decode_color_png_rgb_and_bgr() {
        let fixture = TempFixture::new("png");
        let mut mat = Mat::new_rows_cols_with_default(2, 2, CV_8UC3, Scalar::all(0.0)).unwrap();
        // OpenCV stores BGR: set (0,0) to pure Blue [255, 0, 0]
        *mat.at_2d_mut::<opencv::core::Vec3b>(0, 0).unwrap() =
            opencv::core::Vec3b::from([255, 0, 0]);
        // set (0,1) to pure Red [0, 0, 255]
        *mat.at_2d_mut::<opencv::core::Vec3b>(0, 1).unwrap() =
            opencv::core::Vec3b::from([0, 0, 255]);
        imwrite(fixture.path.to_str().unwrap(), &mat, &Vector::new()).unwrap();

        // 1. Decoded as default RGB
        let img_rgb = decode_image(&fixture.path).unwrap();
        assert_eq!(img_rgb.format(), PixelFormat::Rgb8);
        assert_eq!(img_rgb.channels(), 3);
        assert_eq!(img_rgb.stride(), 6);
        // (0,0) was BGR [255, 0, 0] -> in RGB must be [0, 0, 255]
        assert_eq!(img_rgb.get_pixel(0, 0), Some(&[0, 0, 255][..]));
        // (0,1) was BGR [0, 0, 255] -> in RGB must be [255, 0, 0]
        assert_eq!(img_rgb.get_pixel(1, 0), Some(&[255, 0, 0][..]));

        // 2. Decoded as explicit BGR
        let img_bgr = decode_image_with_order(&fixture.path, ColorOrder::Bgr).unwrap();
        assert_eq!(img_bgr.format(), PixelFormat::Bgr8);
        assert_eq!(img_bgr.get_pixel(0, 0), Some(&[255, 0, 0][..]));
        assert_eq!(img_bgr.get_pixel(1, 0), Some(&[0, 0, 255][..]));

        // 3. decode_gray_image on color image returns ExpectedGrayscale error
        let err = decode_gray_image(&fixture.path).unwrap_err();
        assert!(matches!(
            err,
            ImageError::ExpectedGrayscale { channels: 3, .. }
        ));
    }

    #[test]
    fn test_decode_jpeg_color_and_grayscale() {
        // Color JPEG
        let color_fixture = TempFixture::new("jpg");
        let mat_color = Mat::new_rows_cols_with_default(6, 6, CV_8UC3, Scalar::all(120.0)).unwrap();
        imwrite(
            color_fixture.path.to_str().unwrap(),
            &mat_color,
            &Vector::new(),
        )
        .unwrap();

        let img_color = decode_image(&color_fixture.path).unwrap();
        assert_eq!(img_color.width(), 6);
        assert_eq!(img_color.height(), 6);
        assert_eq!(img_color.format(), PixelFormat::Rgb8);

        // Grayscale JPEG
        let gray_fixture = TempFixture::new("jpeg");
        let mat_gray = Mat::new_rows_cols_with_default(6, 6, CV_8UC1, Scalar::all(200.0)).unwrap();
        imwrite(
            gray_fixture.path.to_str().unwrap(),
            &mat_gray,
            &Vector::new(),
        )
        .unwrap();

        let img_gray = decode_image(&gray_fixture.path).unwrap();
        assert_eq!(img_gray.format(), PixelFormat::Gray8);
        assert_eq!(img_gray.channels(), 1);
    }

    #[test]
    fn test_decode_single_page_tiff() {
        let fixture = TempFixture::new("tiff");
        let mat = Mat::new_rows_cols_with_default(4, 4, CV_8UC1, Scalar::all(75.0)).unwrap();
        imwrite(fixture.path.to_str().unwrap(), &mat, &Vector::new()).unwrap();

        let img = decode_image(&fixture.path).unwrap();
        assert_eq!(img.width(), 4);
        assert_eq!(img.height(), 4);
        assert_eq!(img.format(), PixelFormat::Gray8);
        assert_eq!(img.as_slice(), vec![75; 16].as_slice());
    }

    #[test]
    fn test_multipage_tiff_rejection() {
        let fixture = TempFixture::new("tiff");
        let p1 = Mat::new_rows_cols_with_default(4, 4, CV_8UC1, Scalar::all(50.0)).unwrap();
        let p2 = Mat::new_rows_cols_with_default(4, 4, CV_8UC1, Scalar::all(100.0)).unwrap();
        let mut pages = Vector::<Mat>::new();
        pages.push(p1);
        pages.push(p2);

        imwritemulti(fixture.path.to_str().unwrap(), &pages, &Vector::new()).unwrap();

        let err = decode_image(&fixture.path).unwrap_err();
        assert_eq!(
            err,
            ImageError::MultiPageNotSupported {
                path: fixture.path.display().to_string(),
                pages: 2,
            }
        );
    }

    #[test]
    fn test_alpha_compositing_against_white() {
        let fixture = TempFixture::new("png");
        let mut mat = Mat::new_rows_cols_with_default(1, 2, CV_8UC4, Scalar::all(0.0)).unwrap();
        // Pixel 0: Pure Black with Alpha = 0 (completely transparent)
        // Expected blended with white: Pure White [255, 255, 255]
        *mat.at_2d_mut::<opencv::core::Vec4b>(0, 0).unwrap() =
            opencv::core::Vec4b::from([0, 0, 0, 0]);

        // Pixel 1: Pure Blue [255, 0, 0] with Alpha = 255 (completely opaque)
        // In RGB: Pure Blue is [0, 0, 255]
        *mat.at_2d_mut::<opencv::core::Vec4b>(0, 1).unwrap() =
            opencv::core::Vec4b::from([255, 0, 0, 255]);

        imwrite(fixture.path.to_str().unwrap(), &mat, &Vector::new()).unwrap();

        let img = decode_image(&fixture.path).unwrap();
        assert_eq!(img.format(), PixelFormat::Rgb8);
        assert_eq!(img.get_pixel(0, 0), Some(&[255, 255, 255][..]));
        assert_eq!(img.get_pixel(1, 0), Some(&[0, 0, 255][..]));
    }

    #[test]
    fn test_unsupported_depth_16bit_rejection() {
        let fixture = TempFixture::new("tiff");
        let mat_16u =
            Mat::new_rows_cols_with_default(4, 4, CV_16UC1, Scalar::all(30000.0)).unwrap();
        imwrite(fixture.path.to_str().unwrap(), &mat_16u, &Vector::new()).unwrap();

        let err = decode_image(&fixture.path).unwrap_err();
        assert!(matches!(err, ImageError::UnsupportedDepth { depth: 2 }));
    }
}

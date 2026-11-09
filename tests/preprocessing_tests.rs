//! Integration tests for OMNISCAN grayscale conversion and thresholding engines.

use omniscan::imaging::{
    adaptive_gaussian_threshold, adaptive_gaussian_threshold_default, otsu_threshold,
    otsu_threshold_with_level, to_grayscale, AdaptiveGaussianParams, GrayImage, ImageBuffer,
    PixelFormat,
};
use omniscan::ImageError;

#[test]
fn test_integration_rgb_and_bgr_to_grayscale() {
    // 2x2 RGB image with 4 distinct colors:
    // (0,0): Pure Red   [255, 0, 0]   -> Y ~ 76
    // (1,0): Pure Green [0, 255, 0]   -> Y ~ 150
    // (0,1): Pure Blue  [0, 0, 255]   -> Y ~ 29
    // (1,1): Pure White [255, 255, 255] -> Y = 255
    let rgb_data = vec![
        255, 0, 0, // (0,0) Red
        0, 255, 0, // (1,0) Green
        0, 0, 255, // (0,1) Blue
        255, 255, 255, // (1,1) White
    ];
    let rgb_buf = ImageBuffer::new(2, 2, PixelFormat::Rgb8, rgb_data).unwrap();
    let gray_from_rgb = to_grayscale(&rgb_buf).unwrap();

    assert_eq!(gray_from_rgb.width(), 2);
    assert_eq!(gray_from_rgb.height(), 2);
    assert_eq!(gray_from_rgb.as_slice().len(), 4);
    assert_eq!(gray_from_rgb.get_pixel(0, 0), Some(76));
    assert_eq!(gray_from_rgb.get_pixel(1, 0), Some(150));
    assert_eq!(gray_from_rgb.get_pixel(0, 1), Some(29));
    assert_eq!(gray_from_rgb.get_pixel(1, 1), Some(255));

    // 2x2 BGR image with pure colors:
    // (0,0): Pure Blue  [255, 0, 0] in BGR -> Y ~ 29
    // (1,0): Pure Red   [0, 0, 255] in BGR -> Y ~ 76
    let bgr_data = vec![
        255, 0, 0, // (0,0) Blue in BGR
        0, 0, 255, // (1,0) Red in BGR
        0, 0, 0, // (0,1) Black
        128, 128, 128, // (1,1) Mid-gray
    ];
    let bgr_buf = ImageBuffer::new(2, 2, PixelFormat::Bgr8, bgr_data).unwrap();
    let gray_from_bgr = to_grayscale(&bgr_buf).unwrap();

    assert_eq!(gray_from_bgr.get_pixel(0, 0), Some(29));
    assert_eq!(gray_from_bgr.get_pixel(1, 0), Some(76));
    assert_eq!(gray_from_bgr.get_pixel(0, 1), Some(0));
    assert_eq!(gray_from_bgr.get_pixel(1, 1), Some(128));
}

#[test]
fn test_integration_grayscale_passthrough() {
    let gray_data = vec![50, 100, 150, 200];
    let buf = ImageBuffer::new(2, 2, PixelFormat::Gray8, gray_data.clone()).unwrap();
    let res = to_grayscale(&buf).unwrap();

    assert_eq!(res.width(), 2);
    assert_eq!(res.height(), 2);
    assert_eq!(res.as_slice(), gray_data.as_slice());
}

#[test]
fn test_integration_otsu_polarity_and_synthetic_omr_sheet() {
    // 20x20 synthetic sheet:
    // Background: Clean paper (luminance = 230)
    // Marks: Two filled bubbles centered at (5, 5) and (15, 15) with radius 3 (luminance = 35)
    let width = 20;
    let height = 20;
    let mut pixels = vec![230u8; width * height];

    let marks = [(5.0, 5.0, 3.0), (15.0, 15.0, 3.0)];
    for (cx, cy, r) in marks {
        for y in 0..height {
            for x in 0..width {
                let dist = ((x as f64 - cx).powi(2) + (y as f64 - cy).powi(2)).sqrt();
                if dist <= r {
                    pixels[y * width + x] = 35;
                }
            }
        }
    }

    let gray = GrayImage::new(width as u32, height as u32, pixels).unwrap();
    let (binary, threshold_level) = otsu_threshold_with_level(&gray).unwrap();

    // Verify computed threshold splits the two populations (OpenCV Otsu returns lower class bound 35.0)
    assert!(
        (35.0..230.0).contains(&threshold_level),
        "Threshold must split marks and paper, got {threshold_level}"
    );

    // Verify dimensions
    assert_eq!(binary.width(), width as u32);
    assert_eq!(binary.height(), height as u32);

    let binary_direct = otsu_threshold(&gray).unwrap();
    assert_eq!(binary_direct.as_slice(), binary.as_slice());

    // Verify canonical polarity:
    // Bubble centers must be 0 (dark mark / foreground)
    assert_eq!(binary.get_pixel(5, 5), Some(0));
    assert_eq!(binary.get_pixel(15, 15), Some(0));
    assert!(binary.is_foreground(5, 5).unwrap());
    assert!(binary.is_foreground(15, 15).unwrap());

    // Paper background at corners must be 255 (background paper)
    assert_eq!(binary.get_pixel(0, 0), Some(255));
    assert_eq!(binary.get_pixel(19, 0), Some(255));
    assert_eq!(binary.get_pixel(0, 19), Some(255));
    assert_eq!(binary.get_pixel(19, 19), Some(255));
    assert!(binary.is_background(0, 0).unwrap());
    assert!(binary.is_background(19, 19).unwrap());

    // Invariant: every single pixel must be strictly 0 or 255
    assert!(binary.as_slice().iter().all(|&b| b == 0 || b == 255));
}

#[test]
fn test_integration_otsu_degenerate_cases() {
    // 1. Uniform all-white
    let all_white = GrayImage::new(6, 6, vec![255; 36]).unwrap();
    let (bin_white, lvl_white) = otsu_threshold_with_level(&all_white).unwrap();
    assert_eq!(lvl_white, 0.0);
    assert!(bin_white.as_slice().iter().all(|&b| b == 255));

    // 2. Uniform all-black
    let all_black = GrayImage::new(6, 6, vec![0; 36]).unwrap();
    let (bin_black, lvl_black) = otsu_threshold_with_level(&all_black).unwrap();
    assert_eq!(lvl_black, 0.0);
    assert!(bin_black.as_slice().iter().all(|&b| b == 0));

    // 3. Uniform intermediate gray (128)
    let uniform_gray = GrayImage::new(6, 6, vec![128; 36]).unwrap();
    let (bin_gray, lvl_gray) = otsu_threshold_with_level(&uniform_gray).unwrap();
    assert_eq!(lvl_gray, 0.0);
    assert!(bin_gray.as_slice().iter().all(|&b| b == 255));
}

#[test]
fn test_integration_adaptive_gaussian_thresholding() {
    // 25x25 image with a steep illumination gradient from top (60) to bottom (220),
    // and two dark pencil marks (contrast: 40 lower than local background).
    let width = 25;
    let height = 25;
    let mut pixels = vec![0u8; width * height];

    for y in 0..height {
        let row_bg = 60 + ((y * 160) / height) as u8;
        for x in 0..width {
            pixels[y * width + x] = row_bg;
        }
    }

    // Place dark marks in both dark and bright gradient regions:
    pixels[5 * width + 5] = 20; // in dark top region
    pixels[20 * width + 20] = 130; // in bright bottom region

    let gray = GrayImage::new(width as u32, height as u32, pixels).unwrap();
    let params = AdaptiveGaussianParams::new(7, 8.0).unwrap();
    let binary = adaptive_gaussian_threshold(&gray, &params).unwrap();

    // Both marks should be detected as 0 (foreground mark) despite the non-uniform background gradient
    assert_eq!(binary.get_pixel(5, 5), Some(0));
    assert_eq!(binary.get_pixel(20, 20), Some(0));
    assert!(binary.is_foreground(5, 5).unwrap());
    assert!(binary.is_foreground(20, 20).unwrap());

    // Surrounding background must be 255 (background paper)
    assert_eq!(binary.get_pixel(0, 0), Some(255));
    assert_eq!(binary.get_pixel(24, 24), Some(255));
    assert!(binary.is_background(0, 0).unwrap());
    assert!(binary.is_background(24, 24).unwrap());
}

#[test]
fn test_integration_adaptive_threshold_invalid_parameters() {
    let gray = GrayImage::new(8, 8, vec![150; 64]).unwrap();

    // Even block size rejected
    let even_err = AdaptiveGaussianParams::new(4, 5.0).unwrap_err();
    assert!(matches!(even_err, ImageError::InvalidAdaptiveParams { .. }));

    // Block size <= 1 rejected
    let one_err = AdaptiveGaussianParams::new(1, 5.0).unwrap_err();
    assert!(matches!(one_err, ImageError::InvalidAdaptiveParams { .. }));

    // NaN C rejected
    let nan_err = AdaptiveGaussianParams::new(5, f64::NAN).unwrap_err();
    assert!(matches!(nan_err, ImageError::InvalidAdaptiveParams { .. }));

    // Block size exceeding image dimensions rejected
    let params_too_large = AdaptiveGaussianParams::new(11, 5.0).unwrap();
    let dim_err = adaptive_gaussian_threshold(&gray, &params_too_large).unwrap_err();
    assert!(matches!(dim_err, ImageError::InvalidAdaptiveParams { .. }));
    assert!(dim_err
        .to_string()
        .contains("exceeds minimum image dimension"));
}

#[test]
fn test_integration_adaptive_threshold_default_convenience() {
    // 30x30 image with default parameters (block_size: 21, c: 10.0)
    let gray = GrayImage::new(30, 30, vec![200; 900]).unwrap();
    let binary = adaptive_gaussian_threshold_default(&gray).unwrap();

    assert_eq!(binary.width(), 30);
    assert_eq!(binary.height(), 30);
    assert!(binary.as_slice().iter().all(|&b| b == 0 || b == 255));
}

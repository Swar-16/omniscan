//! Integration tests for native OpenCV image decoding adapter in OMNISCAN.

use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};

use opencv::core::{Mat, Scalar, Vector, CV_16UC1, CV_8UC1, CV_8UC3, CV_8UC4};
use opencv::imgcodecs::{imwrite, imwritemulti};
use opencv::prelude::*;

use omniscan::imaging::{
    decode_gray_image, decode_image, decode_image_with_order, is_supported_format, ColorOrder,
    PixelFormat,
};
use omniscan::ImageError;

static TEST_COUNTER: AtomicUsize = AtomicUsize::new(0);

struct TempImage {
    path: PathBuf,
}

impl TempImage {
    fn new(ext: &str) -> Self {
        let count = TEST_COUNTER.fetch_add(1, Ordering::SeqCst);
        let dir = std::env::temp_dir().join("omniscan_integration_img_tests");
        let _ = fs::create_dir_all(&dir);
        let path = dir.join(format!("integ_{}_{}.{}", std::process::id(), count, ext));
        Self { path }
    }
}

impl Drop for TempImage {
    fn drop(&mut self) {
        let _ = fs::remove_file(&self.path);
    }
}

#[test]
fn test_integration_supported_formats_filter() {
    assert!(is_supported_format(Path::new("scan.png")));
    assert!(is_supported_format(Path::new("scan.PNG")));
    assert!(is_supported_format(Path::new("scan.jpg")));
    assert!(is_supported_format(Path::new("scan.jpeg")));
    assert!(is_supported_format(Path::new("scan.tif")));
    assert!(is_supported_format(Path::new("scan.tiff")));

    assert!(!is_supported_format(Path::new("scan.bmp")));
    assert!(!is_supported_format(Path::new("scan.gif")));
    assert!(!is_supported_format(Path::new("scan.pdf")));
}

#[test]
fn test_integration_missing_file() {
    let missing = Path::new("completely_nonexistent_omniscan_test_file.png");
    let err = decode_image(missing).unwrap_err();
    assert_eq!(
        err,
        ImageError::FileNotFound {
            path: missing.display().to_string(),
        }
    );
}

#[test]
fn test_integration_corrupt_files() {
    let png = TempImage::new("png");
    fs::write(&png.path, b"corrupted random data not a valid png").unwrap();
    let err_png = decode_image(&png.path).unwrap_err();
    assert!(matches!(err_png, ImageError::DecodeFailed { .. }));

    let jpg = TempImage::new("jpg");
    fs::write(&jpg.path, b"corrupted random data not a valid jpg").unwrap();
    let err_jpg = decode_image(&jpg.path).unwrap_err();
    assert!(matches!(err_jpg, ImageError::DecodeFailed { .. }));

    // Empty 0-byte image file must return DecodeFailed
    let empty_png = TempImage::new("png");
    fs::write(&empty_png.path, b"").unwrap();
    let err_empty = decode_image(&empty_png.path).unwrap_err();
    assert!(matches!(err_empty, ImageError::DecodeFailed { .. }));
}

#[test]
fn test_integration_decode_png_grayscale_and_color() {
    // 1. Grayscale PNG
    let gray_temp = TempImage::new("png");
    let gray_mat = Mat::new_rows_cols_with_default(5, 7, CV_8UC1, Scalar::all(160.0)).unwrap();
    imwrite(gray_temp.path.to_str().unwrap(), &gray_mat, &Vector::new()).unwrap();

    let decoded_gray = decode_image(&gray_temp.path).unwrap();
    assert_eq!(decoded_gray.width(), 7);
    assert_eq!(decoded_gray.height(), 5);
    assert_eq!(decoded_gray.format(), PixelFormat::Gray8);
    assert_eq!(decoded_gray.stride(), 7);
    assert_eq!(decoded_gray.channels(), 1);
    assert_eq!(decoded_gray.as_slice().len(), 35);
    assert!(decoded_gray.as_slice().iter().all(|&b| b == 160));

    // 2. Color PNG with known color pixels
    let color_temp = TempImage::new("png");
    let mut color_mat = Mat::new_rows_cols_with_default(2, 2, CV_8UC3, Scalar::all(0.0)).unwrap();
    // BGR: Green is [0, 255, 0]
    *color_mat.at_2d_mut::<opencv::core::Vec3b>(0, 0).unwrap() =
        opencv::core::Vec3b::from([0, 255, 0]);
    // BGR: Red is [0, 0, 255]
    *color_mat.at_2d_mut::<opencv::core::Vec3b>(1, 1).unwrap() =
        opencv::core::Vec3b::from([0, 0, 255]);
    imwrite(
        color_temp.path.to_str().unwrap(),
        &color_mat,
        &Vector::new(),
    )
    .unwrap();

    // Default RGB decode
    let decoded_rgb = decode_image(&color_temp.path).unwrap();
    assert_eq!(decoded_rgb.format(), PixelFormat::Rgb8);
    assert_eq!(decoded_rgb.stride(), 6);
    assert_eq!(decoded_rgb.as_slice().len(), 12);
    // Green in RGB: [0, 255, 0]
    assert_eq!(decoded_rgb.get_pixel(0, 0), Some(&[0, 255, 0][..]));
    // Red in RGB: [255, 0, 0]
    assert_eq!(decoded_rgb.get_pixel(1, 1), Some(&[255, 0, 0][..]));

    // Explicit BGR decode
    let decoded_bgr = decode_image_with_order(&color_temp.path, ColorOrder::Bgr).unwrap();
    assert_eq!(decoded_bgr.format(), PixelFormat::Bgr8);
    // Red in BGR: [0, 0, 255]
    assert_eq!(decoded_bgr.get_pixel(1, 1), Some(&[0, 0, 255][..]));
}

#[test]
fn test_integration_decode_jpeg() {
    let temp = TempImage::new("jpg");
    let mat = Mat::new_rows_cols_with_default(8, 8, CV_8UC3, Scalar::all(100.0)).unwrap();
    imwrite(temp.path.to_str().unwrap(), &mat, &Vector::new()).unwrap();

    let img = decode_image(&temp.path).unwrap();
    assert_eq!(img.width(), 8);
    assert_eq!(img.height(), 8);
    assert_eq!(img.format(), PixelFormat::Rgb8);
    assert_eq!(img.stride(), 24);
    assert_eq!(img.as_slice().len(), 192);
}

#[test]
fn test_integration_single_page_tiff() {
    let temp = TempImage::new("tiff");
    let mat = Mat::new_rows_cols_with_default(3, 4, CV_8UC1, Scalar::all(99.0)).unwrap();
    imwrite(temp.path.to_str().unwrap(), &mat, &Vector::new()).unwrap();

    let img = decode_image(&temp.path).unwrap();
    assert_eq!(img.width(), 4);
    assert_eq!(img.height(), 3);
    assert_eq!(img.format(), PixelFormat::Gray8);
    assert_eq!(img.stride(), 4);
    assert_eq!(img.as_slice(), vec![99; 12].as_slice());
}

#[test]
fn test_integration_reject_multipage_tiff() {
    let temp = TempImage::new("tiff");
    let p1 = Mat::new_rows_cols_with_default(4, 4, CV_8UC1, Scalar::all(10.0)).unwrap();
    let p2 = Mat::new_rows_cols_with_default(4, 4, CV_8UC1, Scalar::all(20.0)).unwrap();
    let p3 = Mat::new_rows_cols_with_default(4, 4, CV_8UC1, Scalar::all(30.0)).unwrap();

    let mut pages = Vector::<Mat>::new();
    pages.push(p1);
    pages.push(p2);
    pages.push(p3);

    imwritemulti(temp.path.to_str().unwrap(), &pages, &Vector::new()).unwrap();

    let err = decode_image(&temp.path).unwrap_err();
    assert_eq!(
        err,
        ImageError::MultiPageNotSupported {
            path: temp.path.display().to_string(),
            pages: 3,
        }
    );
}

#[test]
fn test_integration_alpha_compositing_white() {
    let temp = TempImage::new("png");
    let mut mat = Mat::new_rows_cols_with_default(1, 1, CV_8UC4, Scalar::all(0.0)).unwrap();
    // 50% transparent pure red in BGR: [0, 0, 200, 128]
    // Blended against white: (200 * 128 + 255 * 127 + 127) / 255 = ~227
    *mat.at_2d_mut::<opencv::core::Vec4b>(0, 0).unwrap() =
        opencv::core::Vec4b::from([0, 0, 200, 128]);
    imwrite(temp.path.to_str().unwrap(), &mat, &Vector::new()).unwrap();

    let img = decode_image(&temp.path).unwrap();
    assert_eq!(img.format(), PixelFormat::Rgb8);
    let p = img.get_pixel(0, 0).unwrap();
    // Red channel should be blended with white
    assert!(p[0] >= 220 && p[0] <= 230);
    // Green and Blue channels should be blended from 0 to light grey/white
    assert!(p[1] >= 120 && p[1] <= 135);
    assert!(p[2] >= 120 && p[2] <= 135);
}

#[test]
fn test_integration_reject_16bit_depth() {
    let temp = TempImage::new("tiff");
    let mat = Mat::new_rows_cols_with_default(2, 2, CV_16UC1, Scalar::all(50000.0)).unwrap();
    imwrite(temp.path.to_str().unwrap(), &mat, &Vector::new()).unwrap();

    let err = decode_image(&temp.path).unwrap_err();
    assert!(matches!(err, ImageError::UnsupportedDepth { depth: 2 }));
}

#[test]
fn test_integration_decode_gray_image_contract() {
    // Grayscale succeeds
    let temp_gray = TempImage::new("png");
    let mat_gray = Mat::new_rows_cols_with_default(3, 3, CV_8UC1, Scalar::all(50.0)).unwrap();
    imwrite(temp_gray.path.to_str().unwrap(), &mat_gray, &Vector::new()).unwrap();
    let gray_img = decode_gray_image(&temp_gray.path).unwrap();
    assert_eq!(gray_img.width(), 3);
    assert_eq!(gray_img.height(), 3);

    // Color image rejected with ExpectedGrayscale
    let temp_color = TempImage::new("png");
    let mat_color = Mat::new_rows_cols_with_default(3, 3, CV_8UC3, Scalar::all(50.0)).unwrap();
    imwrite(
        temp_color.path.to_str().unwrap(),
        &mat_color,
        &Vector::new(),
    )
    .unwrap();
    let err = decode_gray_image(&temp_color.path).unwrap_err();
    assert!(matches!(
        err,
        ImageError::ExpectedGrayscale { channels: 3, .. }
    ));
}

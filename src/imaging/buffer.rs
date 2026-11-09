//! Foundational project-owned image abstractions, memory models, and validation.
//!
//! In accordance with ADR-002 and ADR-005, these abstractions remain completely independent
//! of OpenCV's `Mat` or FFI primitives. Concrete types with private fields enforce dimensional
//! sanity, checked arithmetic limits, and binary pixel conventions at construction time.

use crate::error::{ImageError, Result};
use serde::{Deserialize, Serialize};

/// Maximum permitted pixel dimension (width or height) for an ingested scan.
pub const MAX_IMAGE_DIMENSION: u32 = 15_000;

/// Maximum permitted resident byte allocation for a single decoded pixel buffer (120 MB).
/// A 600 DPI A4 color scan (approx. 4960 x 7016 x 3) is ~104 MB.
pub const MAX_IMAGE_PIXEL_BYTES: usize = 120 * 1024 * 1024;

/// Canonical pixel format specifying channel count and byte ordering.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum PixelFormat {
    /// Single-channel 8-bit grayscale (1 byte per pixel: luminance 0..255).
    Gray8,

    /// Three-channel 24-bit sRGB:
    /// - Byte 0: Red
    /// - Byte 1: Green
    /// - Byte 2: Blue
    Rgb8,

    /// Three-channel 24-bit BGR (native byte ordering of OpenCV decoded matrices):
    /// - Byte 0: Blue
    /// - Byte 1: Green
    /// - Byte 2: Red
    Bgr8,
}

impl PixelFormat {
    /// Returns the number of channels per pixel for this format.
    #[inline]
    pub const fn channels(&self) -> usize {
        match self {
            Self::Gray8 => 1,
            Self::Rgb8 | Self::Bgr8 => 3,
        }
    }
}

/// Project-owned contiguous raster image buffer.
///
/// Encapsulates row-major unpadded pixel memory. All fields are private to guarantee
/// dimensional, stride, and byte-length invariants.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImageBuffer {
    width: u32,
    height: u32,
    format: PixelFormat,
    stride: usize,
    data: Vec<u8>,
}

impl ImageBuffer {
    /// Constructs a tightly-packed `ImageBuffer`, validating all dimensions and memory bounds.
    ///
    /// Row stride is automatically computed as `width * format.channels()`.
    pub fn new(width: u32, height: u32, format: PixelFormat, data: Vec<u8>) -> Result<Self> {
        let channels = format.channels();
        let stride = compute_and_validate_stride(width, channels)?;
        validate_buffer_capacity(width, height, stride, data.len())?;

        Ok(Self {
            width,
            height,
            format,
            stride,
            data,
        })
    }

    /// Constructs an `ImageBuffer` with an explicit row stride.
    ///
    /// For the MVP baseline, row stride must equal `width * format.channels()` (tightly packed).
    /// Non-contiguous OpenCV `Mat` padding must be copied row-by-row into tightly-packed storage
    /// by the adapter layer before calling this constructor.
    pub fn from_raw_parts(
        width: u32,
        height: u32,
        format: PixelFormat,
        stride: usize,
        data: Vec<u8>,
    ) -> Result<Self> {
        let expected_stride = compute_and_validate_stride(width, format.channels())?;
        if stride != expected_stride {
            return Err(ImageError::InvalidStride {
                expected: expected_stride,
                actual: stride,
            }
            .into());
        }
        validate_buffer_capacity(width, height, stride, data.len())?;

        Ok(Self {
            width,
            height,
            format,
            stride,
            data,
        })
    }

    /// Width of the image in pixels.
    #[inline]
    pub fn width(&self) -> u32 {
        self.width
    }

    /// Height of the image in pixels.
    #[inline]
    pub fn height(&self) -> u32 {
        self.height
    }

    /// Pixel format and channel ordering of the buffer.
    #[inline]
    pub fn format(&self) -> PixelFormat {
        self.format
    }

    /// Number of color channels in this image.
    #[inline]
    pub fn channels(&self) -> usize {
        self.format.channels()
    }

    /// Row stride in bytes (bytes per row).
    #[inline]
    pub fn stride(&self) -> usize {
        self.stride
    }

    /// Immutable reference to the underlying contiguous pixel bytes.
    #[inline]
    pub fn as_slice(&self) -> &[u8] {
        &self.data
    }

    /// Consumes the buffer and returns ownership of the raw pixel vector without cloning.
    #[inline]
    pub fn into_vec(self) -> Vec<u8> {
        self.data
    }

    /// Returns the pixel slice at coordinates `(x, y)` if within bounds.
    ///
    /// Returns a slice of length `self.channels()`.
    pub fn get_pixel(&self, x: u32, y: u32) -> Option<&[u8]> {
        if x >= self.width || y >= self.height {
            return None;
        }
        let channels = self.channels();
        let offset = (y as usize) * self.stride + (x as usize) * channels;
        Some(&self.data[offset..offset + channels])
    }
}

/// Dedicated single-channel 8-bit grayscale image buffer.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GrayImage {
    width: u32,
    height: u32,
    stride: usize,
    data: Vec<u8>,
}

impl GrayImage {
    /// Constructs a tightly-packed `GrayImage` validating dimensions and buffer length.
    pub fn new(width: u32, height: u32, data: Vec<u8>) -> Result<Self> {
        let stride = compute_and_validate_stride(width, 1)?;
        validate_buffer_capacity(width, height, stride, data.len())?;

        Ok(Self {
            width,
            height,
            stride,
            data,
        })
    }

    /// Constructs a `GrayImage` with an explicit row stride.
    pub fn from_raw_parts(width: u32, height: u32, stride: usize, data: Vec<u8>) -> Result<Self> {
        let expected_stride = compute_and_validate_stride(width, 1)?;
        if stride != expected_stride {
            return Err(ImageError::InvalidStride {
                expected: expected_stride,
                actual: stride,
            }
            .into());
        }
        validate_buffer_capacity(width, height, stride, data.len())?;

        Ok(Self {
            width,
            height,
            stride,
            data,
        })
    }

    /// Width of the image in pixels.
    #[inline]
    pub fn width(&self) -> u32 {
        self.width
    }

    /// Height of the image in pixels.
    #[inline]
    pub fn height(&self) -> u32 {
        self.height
    }

    /// Row stride in bytes (equals `width` for tightly packed grayscale).
    #[inline]
    pub fn stride(&self) -> usize {
        self.stride
    }

    /// Immutable reference to the raw grayscale pixel bytes.
    #[inline]
    pub fn as_slice(&self) -> &[u8] {
        &self.data
    }

    /// Consumes the buffer and yields the underlying byte vector.
    #[inline]
    pub fn into_vec(self) -> Vec<u8> {
        self.data
    }

    /// Returns the 8-bit grayscale intensity at `(x, y)` if within bounds.
    pub fn get_pixel(&self, x: u32, y: u32) -> Option<u8> {
        if x >= self.width || y >= self.height {
            return None;
        }
        let offset = (y as usize) * self.stride + (x as usize);
        Some(self.data[offset])
    }
}

/// Dedicated single-channel binary image buffer.
///
/// **OMNISCAN Canonical Binarization Convention:**
/// - `0`: Foreground mark (dark pencil/pen stroke, bubble fill, corner registration fiducial).
/// - `255`: Background (clean reflective exam paper).
///
/// Constructors strictly validate that every byte is either `0` or `255`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BinaryImage {
    width: u32,
    height: u32,
    stride: usize,
    data: Vec<u8>,
}

impl BinaryImage {
    /// Constructs a `BinaryImage`, enforcing dimensional validity and the 0 / 255 pixel invariant.
    pub fn new(width: u32, height: u32, data: Vec<u8>) -> Result<Self> {
        let stride = compute_and_validate_stride(width, 1)?;
        validate_buffer_capacity(width, height, stride, data.len())?;
        validate_binary_pixels(&data)?;

        Ok(Self {
            width,
            height,
            stride,
            data,
        })
    }

    /// Constructs a `BinaryImage` with an explicit row stride.
    pub fn from_raw_parts(width: u32, height: u32, stride: usize, data: Vec<u8>) -> Result<Self> {
        let expected_stride = compute_and_validate_stride(width, 1)?;
        if stride != expected_stride {
            return Err(ImageError::InvalidStride {
                expected: expected_stride,
                actual: stride,
            }
            .into());
        }
        validate_buffer_capacity(width, height, stride, data.len())?;
        validate_binary_pixels(&data)?;

        Ok(Self {
            width,
            height,
            stride,
            data,
        })
    }

    /// Width of the binary image in pixels.
    #[inline]
    pub fn width(&self) -> u32 {
        self.width
    }

    /// Height of the binary image in pixels.
    #[inline]
    pub fn height(&self) -> u32 {
        self.height
    }

    /// Row stride in bytes.
    #[inline]
    pub fn stride(&self) -> usize {
        self.stride
    }

    /// Immutable reference to the binary pixel bytes (all bytes are 0 or 255).
    #[inline]
    pub fn as_slice(&self) -> &[u8] {
        &self.data
    }

    /// Consumes the buffer and yields the underlying byte vector.
    #[inline]
    pub fn into_vec(self) -> Vec<u8> {
        self.data
    }

    /// Returns the binary pixel value at `(x, y)` if within bounds (either `0` or `255`).
    pub fn get_pixel(&self, x: u32, y: u32) -> Option<u8> {
        if x >= self.width || y >= self.height {
            return None;
        }
        let offset = (y as usize) * self.stride + (x as usize);
        Some(self.data[offset])
    }

    /// Returns `Some(true)` if the pixel at `(x, y)` is a foreground mark (`value == 0`),
    /// `Some(false)` if background (`value == 255`), or `None` if out of bounds.
    pub fn is_foreground(&self, x: u32, y: u32) -> Option<bool> {
        self.get_pixel(x, y).map(|val| val == 0)
    }

    /// Returns `Some(true)` if the pixel at `(x, y)` is background paper (`value == 255`),
    /// `Some(false)` if foreground (`value == 0`), or `None` if out of bounds.
    pub fn is_background(&self, x: u32, y: u32) -> Option<bool> {
        self.get_pixel(x, y).map(|val| val == 255)
    }
}

// ---------------------------------------------------------------------------
// Private Helper Validation Functions (Guaranteed No Unchecked Arithmetic)
// ---------------------------------------------------------------------------

fn compute_and_validate_stride(width: u32, channels: usize) -> Result<usize> {
    if width == 0 || width > MAX_IMAGE_DIMENSION {
        return Err(ImageError::InvalidDimensions {
            width,
            height: 1,
            max: MAX_IMAGE_DIMENSION,
        }
        .into());
    }
    let stride_u32 = width
        .checked_mul(channels as u32)
        .ok_or(ImageError::DimensionOverflow {
            width,
            height: 1,
            channels,
        })?;
    Ok(stride_u32 as usize)
}

fn validate_buffer_capacity(
    width: u32,
    height: u32,
    stride: usize,
    actual_len: usize,
) -> Result<()> {
    if width == 0 || height == 0 || width > MAX_IMAGE_DIMENSION || height > MAX_IMAGE_DIMENSION {
        return Err(ImageError::InvalidDimensions {
            width,
            height,
            max: MAX_IMAGE_DIMENSION,
        }
        .into());
    }

    let expected_len =
        stride
            .checked_mul(height as usize)
            .ok_or_else(|| ImageError::DimensionOverflow {
                width,
                height,
                channels: stride / (width as usize),
            })?;

    if expected_len > MAX_IMAGE_PIXEL_BYTES {
        return Err(ImageError::AllocationTooLarge {
            bytes: expected_len,
            max: MAX_IMAGE_PIXEL_BYTES,
        }
        .into());
    }

    if actual_len != expected_len {
        return Err(ImageError::BufferLengthMismatch {
            expected: expected_len,
            actual: actual_len,
        }
        .into());
    }

    Ok(())
}

fn validate_binary_pixels(data: &[u8]) -> Result<()> {
    for (offset, &byte) in data.iter().enumerate() {
        if byte != 0 && byte != 255 {
            return Err(ImageError::InvalidBinaryPixel {
                value: byte,
                offset,
            }
            .into());
        }
    }
    Ok(())
}

// ---------------------------------------------------------------------------
// Unit Tests Covering All Required Invariants & Failure Modes
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_gray8_construction() {
        let data = vec![100, 150, 200, 250];
        let img = ImageBuffer::new(2, 2, PixelFormat::Gray8, data.clone()).unwrap();
        assert_eq!(img.width(), 2);
        assert_eq!(img.height(), 2);
        assert_eq!(img.format(), PixelFormat::Gray8);
        assert_eq!(img.channels(), 1);
        assert_eq!(img.stride(), 2);
        assert_eq!(img.as_slice(), data.as_slice());
        assert_eq!(img.get_pixel(0, 0), Some(&[100][..]));
        assert_eq!(img.get_pixel(1, 1), Some(&[250][..]));
    }

    #[test]
    fn test_valid_rgb8_construction() {
        let data = vec![
            255, 0, 0, // Red
            0, 255, 0, // Green
            0, 0, 255, // Blue
            255, 255, 255, // White
        ];
        let img = ImageBuffer::new(2, 2, PixelFormat::Rgb8, data).unwrap();
        assert_eq!(img.channels(), 3);
        assert_eq!(img.stride(), 6);
        assert_eq!(img.get_pixel(0, 0), Some(&[255, 0, 0][..]));
        assert_eq!(img.get_pixel(1, 0), Some(&[0, 255, 0][..]));
        assert_eq!(img.get_pixel(0, 1), Some(&[0, 0, 255][..]));
        assert_eq!(img.get_pixel(1, 1), Some(&[255, 255, 255][..]));
    }

    #[test]
    fn test_valid_bgr8_construction() {
        let data = vec![
            255, 0, 0, // Blue in BGR
            0, 255, 0, // Green in BGR
            0, 0, 255, // Red in BGR
            0, 0, 0, // Black
        ];
        let img = ImageBuffer::new(2, 2, PixelFormat::Bgr8, data).unwrap();
        assert_eq!(img.format(), PixelFormat::Bgr8);
        assert_eq!(img.channels(), 3);
        assert_eq!(img.stride(), 6);
        assert_eq!(img.get_pixel(0, 0), Some(&[255, 0, 0][..]));
    }

    #[test]
    fn test_valid_gray_image_construction() {
        let data = vec![10, 20, 30, 40, 50, 60];
        let gray = GrayImage::new(3, 2, data).unwrap();
        assert_eq!(gray.width(), 3);
        assert_eq!(gray.height(), 2);
        assert_eq!(gray.stride(), 3);
        assert_eq!(gray.get_pixel(0, 0), Some(10));
        assert_eq!(gray.get_pixel(2, 1), Some(60));
        assert_eq!(gray.get_pixel(3, 0), None);
    }

    #[test]
    fn test_valid_binary_image_construction() {
        let data = vec![
            0, 255, // Foreground, Background
            255, 0, // Background, Foreground
        ];
        let bin = BinaryImage::new(2, 2, data).unwrap();
        assert_eq!(bin.width(), 2);
        assert_eq!(bin.height(), 2);
        assert_eq!(bin.get_pixel(0, 0), Some(0));
        assert_eq!(bin.is_foreground(0, 0), Some(true));
        assert_eq!(bin.is_background(0, 0), Some(false));
        assert_eq!(bin.is_foreground(1, 0), Some(false));
        assert_eq!(bin.is_background(1, 0), Some(true));
    }

    #[test]
    fn test_zero_width_rejection() {
        let err = ImageBuffer::new(0, 10, PixelFormat::Gray8, vec![]).unwrap_err();
        assert!(err.to_string().contains("Invalid image dimensions"));

        let gray_err = GrayImage::new(0, 10, vec![]).unwrap_err();
        assert!(gray_err.to_string().contains("Invalid image dimensions"));

        let bin_err = BinaryImage::new(0, 10, vec![]).unwrap_err();
        assert!(bin_err.to_string().contains("Invalid image dimensions"));
    }

    #[test]
    fn test_zero_height_rejection() {
        let err = ImageBuffer::new(10, 0, PixelFormat::Gray8, vec![]).unwrap_err();
        assert!(err.to_string().contains("Invalid image dimensions"));

        let gray_err = GrayImage::new(10, 0, vec![]).unwrap_err();
        assert!(gray_err.to_string().contains("Invalid image dimensions"));

        let bin_err = BinaryImage::new(10, 0, vec![]).unwrap_err();
        assert!(bin_err.to_string().contains("Invalid image dimensions"));
    }

    #[test]
    fn test_invalid_buffer_length_rejection() {
        // Expected 2x2x1 = 4 bytes, provided 3 bytes
        let err = ImageBuffer::new(2, 2, PixelFormat::Gray8, vec![0, 1, 2]).unwrap_err();
        assert!(err.to_string().contains("Buffer length mismatch"));

        let gray_err = GrayImage::new(2, 2, vec![0, 1, 2, 3, 4]).unwrap_err();
        assert!(gray_err.to_string().contains("Buffer length mismatch"));

        let bin_err = BinaryImage::new(2, 2, vec![0, 255]).unwrap_err();
        assert!(bin_err.to_string().contains("Buffer length mismatch"));
    }

    #[test]
    fn test_invalid_stride_rejection() {
        // Expected stride for 2 px Gray8 is 2, passed 3
        let err = ImageBuffer::from_raw_parts(2, 2, PixelFormat::Gray8, 3, vec![0; 6]).unwrap_err();
        assert!(err.to_string().contains("Invalid row stride"));

        let gray_err = GrayImage::from_raw_parts(2, 2, 4, vec![0; 8]).unwrap_err();
        assert!(gray_err.to_string().contains("Invalid row stride"));

        let bin_err = BinaryImage::from_raw_parts(2, 2, 4, vec![0; 8]).unwrap_err();
        assert!(bin_err.to_string().contains("Invalid row stride"));
    }

    #[test]
    fn test_arithmetic_overflow_rejection() {
        // u32::MAX width should cleanly fail validation without panic
        let err = ImageBuffer::new(u32::MAX, 1, PixelFormat::Gray8, vec![]).unwrap_err();
        assert!(err.to_string().contains("Invalid image dimensions"));
    }

    #[test]
    fn test_maximum_dimension_rejection() {
        let err = ImageBuffer::new(
            MAX_IMAGE_DIMENSION + 1,
            100,
            PixelFormat::Gray8,
            vec![0; 100],
        )
        .unwrap_err();
        assert!(err.to_string().contains("Invalid image dimensions"));

        let err2 = ImageBuffer::new(
            100,
            MAX_IMAGE_DIMENSION + 1,
            PixelFormat::Gray8,
            vec![0; 100],
        )
        .unwrap_err();
        assert!(err2.to_string().contains("Invalid image dimensions"));
    }

    #[test]
    fn test_maximum_allocation_rejection() {
        // 10,000 x 10,000 x 3 = 300 MB > 120 MB MAX_IMAGE_PIXEL_BYTES
        let err = ImageBuffer::new(10_000, 10_000, PixelFormat::Rgb8, vec![0; 1000]).unwrap_err();
        assert!(err.to_string().contains("exceeds maximum limit"));
    }

    #[test]
    fn test_pixel_indexing_and_out_of_bounds() {
        let data = vec![1, 2, 3, 4];
        let img = ImageBuffer::new(2, 2, PixelFormat::Gray8, data).unwrap();
        assert_eq!(img.get_pixel(0, 0), Some(&[1][..]));
        assert_eq!(img.get_pixel(1, 0), Some(&[2][..]));
        assert_eq!(img.get_pixel(0, 1), Some(&[3][..]));
        assert_eq!(img.get_pixel(1, 1), Some(&[4][..]));

        // Out of bounds
        assert_eq!(img.get_pixel(2, 0), None);
        assert_eq!(img.get_pixel(0, 2), None);
        assert_eq!(img.get_pixel(99, 99), None);
    }

    #[test]
    fn test_binary_value_validation() {
        // Pixel value 128 is neither 0 nor 255 -> must be rejected
        let invalid_data = vec![0, 255, 128, 0];
        let err = BinaryImage::new(2, 2, invalid_data).unwrap_err();
        assert!(err
            .to_string()
            .contains("Invalid binary pixel value: 128 at byte offset 2"));

        // Pixel value 1 is neither 0 nor 255
        let invalid_data2 = vec![1, 255, 0, 0];
        let err2 = BinaryImage::new(2, 2, invalid_data2).unwrap_err();
        assert!(err2
            .to_string()
            .contains("Invalid binary pixel value: 1 at byte offset 0"));
    }

    #[test]
    fn test_channel_count_correctness() {
        assert_eq!(PixelFormat::Gray8.channels(), 1);
        assert_eq!(PixelFormat::Rgb8.channels(), 3);
        assert_eq!(PixelFormat::Bgr8.channels(), 3);
    }
}

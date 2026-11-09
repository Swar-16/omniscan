//! Typed top-level error definitions for OMNISCAN.

use thiserror::Error;

/// Detailed error causes for image abstraction and ingestion validation.
#[derive(Debug, Error, PartialEq, Eq)]
pub enum ImageError {
    #[error("Invalid image dimensions: {width}x{height} (must be > 0 and <= {max})")]
    InvalidDimensions { width: u32, height: u32, max: u32 },

    #[error("Image dimension arithmetic overflow ({width}x{height}x{channels})")]
    DimensionOverflow {
        width: u32,
        height: u32,
        channels: usize,
    },

    #[error("Decoded image allocation ({bytes} bytes) exceeds maximum limit ({max} bytes)")]
    AllocationTooLarge { bytes: usize, max: usize },

    #[error("Buffer length mismatch: expected {expected} bytes, received {actual} bytes")]
    BufferLengthMismatch { expected: usize, actual: usize },

    #[error("Invalid row stride: expected {expected} bytes, received {actual} bytes")]
    InvalidStride { expected: usize, actual: usize },

    #[error(
        "Invalid binary pixel value: {value} at byte offset {offset} (must be 0 for foreground or 255 for background)"
    )]
    InvalidBinaryPixel { value: u8, offset: usize },

    #[error("Image file not found: '{path}'")]
    FileNotFound { path: String },

    #[error("Image read I/O failure on '{path}': {message}")]
    IoError { path: String, message: String },

    #[error("Failed to decode image from '{path}': {message}")]
    DecodeFailed { path: String, message: String },

    #[error("Unsupported image format '{format}' for file '{path}'")]
    UnsupportedFormat { path: String, format: String },

    #[error(
        "Unsupported pixel depth: expected 8-bit unsigned (CV_8U), got OpenCV depth code {depth}"
    )]
    UnsupportedDepth { depth: i32 },

    #[error("Unsupported channel count: expected 1 (Grayscale) or 3 (Color), got {channels}")]
    UnsupportedChannels { channels: i32 },

    #[error(
        "Multi-page image detected on '{path}' ({pages} pages): OMNISCAN only supports single-page sheets"
    )]
    MultiPageNotSupported { path: String, pages: usize },

    #[error("Expected grayscale image but received {channels}-channel color image from '{path}'")]
    ExpectedGrayscale { path: String, channels: usize },

    #[error("Image preprocessing error: {0}")]
    PreprocessingFailed(String),

    #[error("Invalid adaptive threshold parameters: {message}")]
    InvalidAdaptiveParams { message: String },
}

impl From<ImageError> for OmniscanError {
    fn from(err: ImageError) -> Self {
        OmniscanError::ImageProcessing(err.to_string())
    }
}

/// Root error type representing all failure modes in OMNISCAN.
#[derive(Debug, Error)]
pub enum OmniscanError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Image processing error: {0}")]
    ImageProcessing(String),

    #[error("Template validation error: {0}")]
    Template(String),

    #[error("Manifest validation error: {0}")]
    Manifest(String),

    #[error("Ground truth parsing error: {0}")]
    GroundTruth(String),

    #[error("Alignment failure: {0}")]
    Alignment(String),

    #[error("Recognition ambiguity: {0}")]
    Ambiguity(String),

    #[error("Scoring error: {0}")]
    Scoring(String),

    #[error("CLI argument error: {0}")]
    Cli(String),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
}

/// Specialized Result alias for OMNISCAN operations.
pub type Result<T> = std::result::Result<T, OmniscanError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display_formatting() {
        let err = OmniscanError::Config("missing required field 'version'".to_string());
        assert_eq!(
            err.to_string(),
            "Configuration error: missing required field 'version'"
        );

        let template_err = OmniscanError::Template("Question Q1 out of bounds".to_string());
        assert_eq!(
            template_err.to_string(),
            "Template validation error: Question Q1 out of bounds"
        );

        let manifest_err = OmniscanError::Manifest("Path traversal detected".to_string());
        assert_eq!(
            manifest_err.to_string(),
            "Manifest validation error: Path traversal detected"
        );

        let ground_truth_err = OmniscanError::GroundTruth("Line 5: invalid token".to_string());
        assert_eq!(
            ground_truth_err.to_string(),
            "Ground truth parsing error: Line 5: invalid token"
        );

        let img_err: OmniscanError = ImageError::InvalidDimensions {
            width: 0,
            height: 100,
            max: 15_000,
        }
        .into();
        assert_eq!(
            img_err.to_string(),
            "Image processing error: Invalid image dimensions: 0x100 (must be > 0 and <= 15000)"
        );

        let fnf_err = ImageError::FileNotFound {
            path: "test.png".to_string(),
        };
        assert_eq!(fnf_err.to_string(), "Image file not found: 'test.png'");

        let multi_err = ImageError::MultiPageNotSupported {
            path: "sheet.tiff".to_string(),
            pages: 3,
        };
        assert_eq!(
            multi_err.to_string(),
            "Multi-page image detected on 'sheet.tiff' (3 pages): OMNISCAN only supports single-page sheets"
        );

        let prep_err = ImageError::PreprocessingFailed("cvtColor failed".to_string());
        assert_eq!(
            prep_err.to_string(),
            "Image preprocessing error: cvtColor failed"
        );

        let adapt_err = ImageError::InvalidAdaptiveParams {
            message: "Block size must be odd".to_string(),
        };
        assert_eq!(
            adapt_err.to_string(),
            "Invalid adaptive threshold parameters: Block size must be odd"
        );
    }

    #[test]
    fn test_io_error_conversion() {
        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
        let omniscan_err: OmniscanError = io_err.into();
        assert!(matches!(omniscan_err, OmniscanError::Io(_)));
    }
}

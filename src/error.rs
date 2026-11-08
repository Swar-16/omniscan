//! Typed top-level error definitions for OMNISCAN.

use thiserror::Error;

/// Root error type representing all failure modes in OMNISCAN.
#[derive(Debug, Error)]
pub enum OmniscanError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Image processing error: {0}")]
    ImageProcessing(String),

    #[error("Template error: {0}")]
    Template(String),

    #[error("Alignment failure: {0}")]
    Alignment(String),

    #[error("Recognition ambiguity: {0}")]
    Ambiguity(String),

    #[error("Scoring error: {0}")]
    Scoring(String),

    #[error("CLI argument error: {0}")]
    Cli(String),
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

        let ambiguity = OmniscanError::Ambiguity("Question 4 has multiple marks".to_string());
        assert_eq!(
            ambiguity.to_string(),
            "Recognition ambiguity: Question 4 has multiple marks"
        );
    }

    #[test]
    fn test_io_error_conversion() {
        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
        let omniscan_err: OmniscanError = io_err.into();
        assert!(matches!(omniscan_err, OmniscanError::Io(_)));
    }
}


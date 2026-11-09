//! OMNISCAN: Local-first, configurable Optical Mark Recognition (OMR) engine.
//!
//! This library crate exposes the foundational domain types, errors, CLI definitions,
//! normalized layout templates, dataset manifests, ground-truth parsers, and imaging abstractions.

pub mod cli;
pub mod core;
pub mod error;
pub mod imaging;
pub mod labels;
pub mod layout;
pub mod manifest;

pub use cli::{Cli, Commands, PreprocessMethod};
pub use error::{ImageError, OmniscanError, Result};
pub use imaging::{
    adaptive_gaussian_threshold, adaptive_gaussian_threshold_default, compute_latency_stats,
    decode_gray_image, decode_image, decode_image_with_order, execute_preprocess,
    is_supported_format, otsu_threshold, otsu_threshold_with_level, save_binary_image,
    save_gray_image, to_grayscale, AdaptiveGaussianParams, BenchmarkSummary, BinaryImage,
    ColorOrder, GrayImage, ImageBuffer, LatencyStats, PixelFormat, PreprocessConfig,
    PreprocessResult, PreprocessingDiagnostics, TimingBreakdown, MAX_IMAGE_DIMENSION,
    MAX_IMAGE_PIXEL_BYTES, PREPROCESSING_DIAGNOSTICS_SCHEMA_VERSION,
};
pub use layout::Template;

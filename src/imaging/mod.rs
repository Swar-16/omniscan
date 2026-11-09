//! Imaging module for OMNISCAN.
//!
//! Exposes project-owned raster image abstractions, pixel formats, and memory-safe buffers.
//! All OpenCV operations are strictly isolated behind imaging adapters and never leak
//! into domain contracts or public interfaces (ADR-002, ADR-005).

pub mod buffer;
pub mod diagnostics;
pub mod opencv_adapter;
pub mod pipeline;
pub mod preprocessing;

pub use buffer::{
    BinaryImage, GrayImage, ImageBuffer, PixelFormat, MAX_IMAGE_DIMENSION, MAX_IMAGE_PIXEL_BYTES,
};
pub use diagnostics::{
    compute_latency_stats, BenchmarkSummary, LatencyStats, PreprocessingDiagnostics,
    TimingBreakdown, PREPROCESSING_DIAGNOSTICS_SCHEMA_VERSION,
};
pub use opencv_adapter::{
    decode_gray_image, decode_image, decode_image_with_order, is_supported_format,
    save_binary_image, save_gray_image, ColorOrder,
};
pub use pipeline::{execute_preprocess, PreprocessConfig, PreprocessResult};
pub use preprocessing::{
    adaptive_gaussian_threshold, adaptive_gaussian_threshold_default, otsu_threshold,
    otsu_threshold_with_level, to_grayscale, AdaptiveGaussianParams,
};

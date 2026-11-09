//! Versioned diagnostics model and benchmark telemetry for preprocessing pipelines.
//!
//! Diagnostic JSON outputs record structured execution telemetry, parameter configs,
//! per-stage timings (in microseconds), and optional benchmark statistics without
//! embedding large raw pixel buffers.

use std::collections::HashMap;
use std::fs;
use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::error::ImageError;

/// Current schema version string for preprocessing diagnostics.
pub const PREPROCESSING_DIAGNOSTICS_SCHEMA_VERSION: &str = "omniscan-preprocessing-diagnostics-v1";

/// Structured diagnostic telemetry record for an image preprocessing run.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PreprocessingDiagnostics {
    /// Schema version for forward/backward compatibility.
    pub schema_version: String,
    /// Absolute or normalized path to the input scan file.
    pub input_path: String,
    /// Size of the input file on disk in bytes (compressed container size).
    pub file_size_bytes: u64,
    /// Image width in pixels.
    pub width: u32,
    /// Image height in pixels.
    pub height: u32,
    /// Decoded pixel format (e.g. "Gray8", "Rgb8", "Bgr8").
    pub pixel_format: String,
    /// Number of channels in the decoded image.
    pub channels: usize,
    /// Row stride in bytes.
    pub stride_bytes: usize,
    /// Decoded raw buffer size in memory in bytes.
    pub decoded_buffer_bytes: usize,
    /// Preprocessing thresholding method applied ("otsu", "adaptive", "both").
    pub method: String,
    /// Otsu optimal threshold value (0.0 - 255.0) if Otsu thresholding was executed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub otsu_threshold: Option<f64>,
    /// Adaptive Gaussian block size if adaptive thresholding was executed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adaptive_block_size: Option<i32>,
    /// Adaptive Gaussian constant C if adaptive thresholding was executed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adaptive_c: Option<f64>,
    /// Paths to generated output PNG artifacts.
    pub output_artifacts: HashMap<String, String>,
    /// Microsecond timing breakdown across pipeline stages.
    pub timing_micros: TimingBreakdown,
    /// Overall success status of the pipeline.
    pub success: bool,
    /// Optional benchmark statistical summary if benchmark mode was active.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub benchmark: Option<BenchmarkSummary>,
}

impl PreprocessingDiagnostics {
    /// Serializes the diagnostics struct to JSON and writes it atomically to the target path.
    ///
    /// Uses a temporary sibling file followed by atomic rename to prevent partial writes.
    pub fn write_to_file(&self, target_path: &Path) -> Result<(), ImageError> {
        let parent_dir = target_path.parent().unwrap_or_else(|| Path::new("."));
        fs::create_dir_all(parent_dir).map_err(|e| ImageError::IoError {
            path: parent_dir.display().to_string(),
            message: format!("Failed to create diagnostics output directory: {e}"),
        })?;

        let json_data = serde_json::to_string_pretty(self).map_err(|e| ImageError::IoError {
            path: target_path.display().to_string(),
            message: format!("Failed to serialize diagnostics to JSON: {e}"),
        })?;

        let temp_filename = format!(
            ".{}_{}.tmp",
            target_path
                .file_name()
                .and_then(|s| s.to_str())
                .unwrap_or("diag"),
            std::process::id()
        );
        let temp_path = parent_dir.join(temp_filename);

        fs::write(&temp_path, json_data.as_bytes()).map_err(|e| ImageError::IoError {
            path: temp_path.display().to_string(),
            message: format!("Failed to write temporary diagnostics file: {e}"),
        })?;

        fs::rename(&temp_path, target_path).map_err(|e| {
            let _ = fs::remove_file(&temp_path);
            ImageError::IoError {
                path: target_path.display().to_string(),
                message: format!("Failed to atomically rename diagnostics file: {e}"),
            }
        })?;

        Ok(())
    }
}

/// Execution durations for individual pipeline stages, recorded in microseconds.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TimingBreakdown {
    /// Duration of native image decoding from disk.
    pub decode_micros: u64,
    /// Duration of color-to-grayscale conversion.
    pub grayscale_micros: u64,
    /// Duration of Otsu thresholding, if executed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub otsu_micros: Option<u64>,
    /// Duration of adaptive Gaussian thresholding, if executed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adaptive_micros: Option<u64>,
    /// Duration of PNG artifact encoding and writing to disk.
    pub artifact_write_micros: u64,
    /// Total wall-clock duration of the pipeline execution.
    pub total_pipeline_micros: u64,
}

/// Statistical summary across measured benchmark iterations.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct BenchmarkSummary {
    /// Number of discarded warm-up runs.
    pub warmup_iterations: usize,
    /// Number of recorded measured runs.
    pub measured_iterations: usize,
    /// Estimated preprocessing throughput in images processed per second.
    pub throughput_images_per_sec: f64,
    /// Total preprocessing latency statistics (excluding decode and disk artifact I/O).
    pub latency_micros: LatencyStats,
    /// Grayscale conversion latency statistics.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grayscale_latency_micros: Option<LatencyStats>,
    /// Otsu thresholding latency statistics.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub otsu_latency_micros: Option<LatencyStats>,
    /// Adaptive Gaussian thresholding latency statistics.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adaptive_latency_micros: Option<LatencyStats>,
}

/// Descriptive latency statistics across measured iterations (in microseconds).
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LatencyStats {
    pub min_micros: f64,
    pub median_micros: f64,
    pub mean_micros: f64,
    pub max_micros: f64,
}

/// Computes min, median, mean, and max latency statistics from a list of durations in microseconds.
pub fn compute_latency_stats(durations: &[f64]) -> Option<LatencyStats> {
    if durations.is_empty() {
        return None;
    }
    let mut sorted = durations.to_vec();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));

    let len = sorted.len();
    let min_micros = sorted[0];
    let max_micros = sorted[len - 1];
    let sum: f64 = sorted.iter().sum();
    let mean_micros = sum / (len as f64);

    let median_micros = if len % 2 == 1 {
        sorted[len / 2]
    } else {
        (sorted[len / 2 - 1] + sorted[len / 2]) / 2.0
    };

    Some(LatencyStats {
        min_micros,
        median_micros,
        mean_micros,
        max_micros,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_latency_stats_odd() {
        let measurements = [100.0, 200.0, 300.0, 400.0, 500.0];
        let stats = compute_latency_stats(&measurements).unwrap();
        assert_eq!(stats.min_micros, 100.0);
        assert_eq!(stats.max_micros, 500.0);
        assert_eq!(stats.mean_micros, 300.0);
        assert_eq!(stats.median_micros, 300.0);
    }

    #[test]
    fn test_compute_latency_stats_even() {
        let measurements = [100.0, 200.0, 300.0, 400.0];
        let stats = compute_latency_stats(&measurements).unwrap();
        assert_eq!(stats.min_micros, 100.0);
        assert_eq!(stats.max_micros, 400.0);
        assert_eq!(stats.mean_micros, 250.0);
        assert_eq!(stats.median_micros, 250.0);
    }

    #[test]
    fn test_compute_latency_stats_empty() {
        assert!(compute_latency_stats(&[]).is_none());
    }

    #[test]
    fn test_diagnostics_roundtrip_json() {
        let diag = PreprocessingDiagnostics {
            schema_version: PREPROCESSING_DIAGNOSTICS_SCHEMA_VERSION.to_string(),
            input_path: "sheet.png".to_string(),
            file_size_bytes: 45000,
            width: 1000,
            height: 1200,
            pixel_format: "Gray8".to_string(),
            channels: 1,
            stride_bytes: 1000,
            decoded_buffer_bytes: 1200000,
            method: "both".to_string(),
            otsu_threshold: Some(125.0),
            adaptive_block_size: Some(21),
            adaptive_c: Some(10.0),
            output_artifacts: HashMap::from([
                ("grayscale".to_string(), "out/grayscale.png".to_string()),
                ("otsu".to_string(), "out/otsu.png".to_string()),
            ]),
            timing_micros: TimingBreakdown {
                decode_micros: 5000,
                grayscale_micros: 200,
                otsu_micros: Some(300),
                adaptive_micros: Some(1200),
                artifact_write_micros: 4000,
                total_pipeline_micros: 10700,
            },
            success: true,
            benchmark: None,
        };

        let serialized = serde_json::to_string(&diag).unwrap();
        let deserialized: PreprocessingDiagnostics = serde_json::from_str(&serialized).unwrap();
        assert_eq!(diag, deserialized);
    }
}

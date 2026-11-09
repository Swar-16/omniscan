//! End-to-end preprocessing pipeline execution, artifact export, and benchmark runner.
//!
//! Integrates decoding, grayscale conversion, and thresholding into a reproducible pipeline,
//! exporting diagnostic PNG artifacts and atomic JSON telemetry records.

use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::time::Instant;

use crate::cli::PreprocessMethod;
use crate::error::{ImageError, Result};
use crate::imaging::buffer::{BinaryImage, GrayImage};
use crate::imaging::diagnostics::{
    compute_latency_stats, BenchmarkSummary, PreprocessingDiagnostics, TimingBreakdown,
    PREPROCESSING_DIAGNOSTICS_SCHEMA_VERSION,
};
use crate::imaging::opencv_adapter::{decode_image, save_binary_image, save_gray_image};
use crate::imaging::preprocessing::{
    adaptive_gaussian_threshold, otsu_threshold_with_level, to_grayscale, AdaptiveGaussianParams,
};

/// Configuration for the preprocessing pipeline.
#[derive(Debug, Clone)]
pub struct PreprocessConfig {
    /// Path to the input image.
    pub input_path: PathBuf,
    /// Destination directory for generated artifacts and diagnostics.
    pub output_dir: PathBuf,
    /// Preprocessing thresholding method to apply.
    pub method: PreprocessMethod,
    /// Adaptive Gaussian block size.
    pub block_size: i32,
    /// Adaptive Gaussian constant C.
    pub c: f64,
    /// Whether to run in benchmark mode.
    pub benchmark: bool,
    /// Number of measured benchmark iterations.
    pub benchmark_iterations: usize,
}

/// Result of a preprocessing run containing generated in-memory images and diagnostics.
#[derive(Debug)]
pub struct PreprocessResult {
    /// Diagnostic telemetry record written to `diagnostics.json`.
    pub diagnostics: PreprocessingDiagnostics,
    /// Grayscale representation.
    pub grayscale: GrayImage,
    /// Otsu binary image, if requested.
    pub otsu: Option<BinaryImage>,
    /// Adaptive Gaussian binary image, if requested.
    pub adaptive: Option<BinaryImage>,
}

/// Executes the preprocessing pipeline according to the provided configuration.
///
/// # Output Artifacts & Overwrite Semantics
/// Produces output PNG artifacts in `config.output_dir`:
/// - `grayscale.png`: Single-channel 8-bit grayscale image.
/// - `otsu.png`: Binarized sheet via Otsu global thresholding (if `Otsu` or `Both`).
/// - `adaptive.png`: Binarized sheet via adaptive Gaussian thresholding (if `Adaptive` or `Both`).
/// - `diagnostics.json`: Atomic telemetry record adhering to `"omniscan-preprocessing-diagnostics-v1"`.
///
/// **Existing Destination Files:** If `config.output_dir` already contains artifacts from an
/// earlier execution, those files are overwritten in-place. Protection is strictly enforced against
/// overwriting the original input image file if it resides in the destination directory.
///
/// **Partial Write Failure:** Artifacts are written sequentially (`grayscale.png` -> `otsu.png` ->
/// `adaptive.png` -> `diagnostics.json`). If an I/O failure occurs while writing a later artifact,
/// previously written PNG files may remain on disk. However, `diagnostics.json` is only written upon
/// complete pipeline success; an incomplete run is never recorded as successful.
///
/// # Errors
/// Returns an [`OmniscanError`] if decoding fails, parameters are invalid, or I/O fails.
pub fn execute_preprocess(config: &PreprocessConfig) -> Result<PreprocessResult> {
    let pipeline_start = Instant::now();

    // 1. Validate input path existence
    if !config.input_path.exists() {
        return Err(ImageError::FileNotFound {
            path: config.input_path.display().to_string(),
        }
        .into());
    }

    if !config.input_path.is_file() {
        return Err(ImageError::IoError {
            path: config.input_path.display().to_string(),
            message: "Input path is not a regular file".to_string(),
        }
        .into());
    }

    // 2. Prevent accidental overwrite of input image
    if let (Ok(in_canon), Ok(out_canon)) = (
        config.input_path.canonicalize(),
        config.output_dir.canonicalize(),
    ) {
        if in_canon.parent() == Some(&out_canon) {
            let in_name = in_canon.file_name().and_then(|s| s.to_str()).unwrap_or("");
            if matches!(
                in_name,
                "grayscale.png" | "otsu.png" | "adaptive.png" | "diagnostics.json"
            ) {
                return Err(ImageError::IoError {
                    path: in_canon.display().to_string(),
                    message: "Output artifact would overwrite original input file".to_string(),
                }
                .into());
            }
        }
    }

    // 3. Create destination directory
    fs::create_dir_all(&config.output_dir).map_err(|e| ImageError::IoError {
        path: config.output_dir.display().to_string(),
        message: format!("Failed to create output directory: {e}"),
    })?;

    // 4. Validate adaptive parameters upfront if adaptive thresholding will be run
    let adaptive_params = match config.method {
        PreprocessMethod::Adaptive | PreprocessMethod::Both => {
            Some(AdaptiveGaussianParams::new(config.block_size, config.c)?)
        }
        PreprocessMethod::Otsu => None,
    };

    let file_size_bytes = fs::metadata(&config.input_path)
        .map(|m| m.len())
        .map_err(|e| ImageError::IoError {
            path: config.input_path.display().to_string(),
            message: format!("Failed to read input metadata: {e}"),
        })?;

    // 5. Decode image
    let decode_start = Instant::now();
    let decoded = decode_image(&config.input_path)?;
    let decode_micros = decode_start.elapsed().as_micros() as u64;

    let width = decoded.width();
    let height = decoded.height();
    let format_str = format!("{:?}", decoded.format());
    let channels = decoded.channels();
    let stride_bytes = decoded.stride();
    let decoded_buffer_bytes = decoded.as_slice().len();

    // 6. Convert to grayscale
    let gray_start = Instant::now();
    let gray = to_grayscale(&decoded)?;
    let grayscale_micros = gray_start.elapsed().as_micros() as u64;

    // 7. Apply selected thresholding methods
    let mut otsu_image = None;
    let mut otsu_level = None;
    let mut otsu_micros = None;

    if matches!(
        config.method,
        PreprocessMethod::Otsu | PreprocessMethod::Both
    ) {
        let o_start = Instant::now();
        let (bin, lvl) = otsu_threshold_with_level(&gray)?;
        otsu_micros = Some(o_start.elapsed().as_micros() as u64);
        otsu_level = Some(lvl);
        otsu_image = Some(bin);
    }

    let mut adaptive_image = None;
    let mut adaptive_micros = None;

    if let Some(ref params) = adaptive_params {
        let a_start = Instant::now();
        let bin = adaptive_gaussian_threshold(&gray, params)?;
        adaptive_micros = Some(a_start.elapsed().as_micros() as u64);
        adaptive_image = Some(bin);
    }

    // 8. Save output PNG artifacts
    let write_start = Instant::now();
    let mut output_artifacts = HashMap::new();

    let gray_path = config.output_dir.join("grayscale.png");
    save_gray_image(&gray_path, &gray)?;
    output_artifacts.insert(
        "grayscale".to_string(),
        gray_path.to_string_lossy().to_string(),
    );

    if let Some(ref bin) = otsu_image {
        let otsu_path = config.output_dir.join("otsu.png");
        save_binary_image(&otsu_path, bin)?;
        output_artifacts.insert("otsu".to_string(), otsu_path.to_string_lossy().to_string());
    }

    if let Some(ref bin) = adaptive_image {
        let adapt_path = config.output_dir.join("adaptive.png");
        save_binary_image(&adapt_path, bin)?;
        output_artifacts.insert(
            "adaptive".to_string(),
            adapt_path.to_string_lossy().to_string(),
        );
    }
    let artifact_write_micros = write_start.elapsed().as_micros() as u64;

    // 9. Run benchmark if requested
    let (benchmark_summary, benchmark_micros) = if config.benchmark {
        let bench_start = Instant::now();
        let iters = config.benchmark_iterations.max(1);
        let warmup = 2;

        // Warm-up runs (discarded)
        for _ in 0..warmup {
            let _g = to_grayscale(&decoded)?;
            if matches!(
                config.method,
                PreprocessMethod::Otsu | PreprocessMethod::Both
            ) {
                let _b = otsu_threshold_with_level(&_g)?;
            }
            if let Some(ref p) = adaptive_params {
                let _b = adaptive_gaussian_threshold(&_g, p)?;
            }
        }

        // Measured runs
        let mut total_durations = Vec::with_capacity(iters);
        let mut gray_durations = Vec::with_capacity(iters);
        let mut otsu_durations = Vec::with_capacity(iters);
        let mut adapt_durations = Vec::with_capacity(iters);

        for _ in 0..iters {
            let t_iter_start = Instant::now();

            let t_g = Instant::now();
            let iter_gray = to_grayscale(&decoded)?;
            gray_durations.push(t_g.elapsed().as_micros() as f64);

            if matches!(
                config.method,
                PreprocessMethod::Otsu | PreprocessMethod::Both
            ) {
                let t_o = Instant::now();
                let _ = otsu_threshold_with_level(&iter_gray)?;
                otsu_durations.push(t_o.elapsed().as_micros() as f64);
            }

            if let Some(ref p) = adaptive_params {
                let t_a = Instant::now();
                let _ = adaptive_gaussian_threshold(&iter_gray, p)?;
                adapt_durations.push(t_a.elapsed().as_micros() as f64);
            }

            total_durations.push(t_iter_start.elapsed().as_micros() as f64);
        }

        let total_stats = compute_latency_stats(&total_durations).unwrap();
        let throughput = if total_stats.mean_micros > 0.0 {
            1_000_000.0 / total_stats.mean_micros
        } else {
            0.0
        };

        (
            Some(BenchmarkSummary {
                warmup_iterations: warmup,
                measured_iterations: iters,
                throughput_images_per_sec: throughput,
                latency_micros: total_stats,
                grayscale_latency_micros: compute_latency_stats(&gray_durations),
                otsu_latency_micros: if otsu_durations.is_empty() {
                    None
                } else {
                    compute_latency_stats(&otsu_durations)
                },
                adaptive_latency_micros: if adapt_durations.is_empty() {
                    None
                } else {
                    compute_latency_stats(&adapt_durations)
                },
            }),
            Some(bench_start.elapsed().as_micros() as u64),
        )
    } else {
        (None, None)
    };

    let total_pipeline_micros = pipeline_start.elapsed().as_micros() as u64;

    // 10. Assemble and write diagnostics.json
    let diag_path = config.output_dir.join("diagnostics.json");
    output_artifacts.insert(
        "diagnostics".to_string(),
        diag_path.to_string_lossy().to_string(),
    );

    let method_name = match config.method {
        PreprocessMethod::Otsu => "otsu",
        PreprocessMethod::Adaptive => "adaptive",
        PreprocessMethod::Both => "both",
    };

    let diagnostics = PreprocessingDiagnostics {
        schema_version: PREPROCESSING_DIAGNOSTICS_SCHEMA_VERSION.to_string(),
        input_path: config.input_path.to_string_lossy().to_string(),
        file_size_bytes,
        width,
        height,
        pixel_format: format_str,
        channels,
        stride_bytes,
        decoded_buffer_bytes,
        method: method_name.to_string(),
        otsu_threshold: otsu_level,
        adaptive_block_size: adaptive_params.as_ref().map(|p| p.block_size),
        adaptive_c: adaptive_params.as_ref().map(|p| p.c),
        output_artifacts,
        timing_micros: TimingBreakdown {
            decode_micros,
            grayscale_micros,
            otsu_micros,
            adaptive_micros,
            artifact_write_micros,
            benchmark_micros,
            total_pipeline_micros,
        },
        success: true,
        benchmark: benchmark_summary,
    };

    diagnostics.write_to_file(&diag_path)?;

    Ok(PreprocessResult {
        diagnostics,
        grayscale: gray,
        otsu: otsu_image,
        adaptive: adaptive_image,
    })
}

//! CLI integration tests for `validate-image` and `preprocess` commands.

use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::sync::atomic::{AtomicUsize, Ordering};

use opencv::core::{Mat, Scalar, Vector, CV_8UC3};
use opencv::imgcodecs::imwrite;

use omniscan::imaging::{
    decode_image, PreprocessingDiagnostics, PREPROCESSING_DIAGNOSTICS_SCHEMA_VERSION,
};

static CLI_TEST_COUNTER: AtomicUsize = AtomicUsize::new(0);

struct TempWorkspace {
    dir: PathBuf,
}

impl TempWorkspace {
    fn new(name: &str) -> Self {
        let count = CLI_TEST_COUNTER.fetch_add(1, Ordering::SeqCst);
        let dir = std::env::temp_dir().join(format!(
            "omniscan_cli_{}_{}_{}",
            name,
            std::process::id(),
            count
        ));
        let _ = fs::create_dir_all(&dir);
        Self { dir }
    }

    fn file_path(&self, name: &str) -> PathBuf {
        self.dir.join(name)
    }

    fn create_synthetic_png(&self, filename: &str, width: i32, height: i32) -> PathBuf {
        let path = self.file_path(filename);
        let mat =
            Mat::new_rows_cols_with_default(height, width, CV_8UC3, Scalar::all(180.0)).unwrap();
        imwrite(path.to_str().unwrap(), &mat, &Vector::new()).unwrap();
        path
    }
}

impl Drop for TempWorkspace {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.dir);
    }
}

#[test]
fn test_cli_validate_image_success() {
    let ws = TempWorkspace::new("valid_png");
    let img_path = ws.create_synthetic_png("sheet.png", 60, 40);

    let binary = env!("CARGO_BIN_EXE_omniscan");
    let output = Command::new(binary)
        .args(["validate-image", img_path.to_str().unwrap()])
        .output()
        .expect("Failed to execute validate-image");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Image Validation: PASSED"));
    assert!(stdout.contains("60 x 40 px"));
    assert!(stdout.contains("Pixel Format:"));
    assert!(stdout.contains("Row Stride:"));
    assert!(stdout.contains("Container Size:"));
    assert!(stdout.contains("Decoded Memory:"));
    assert!(stdout.contains("Dimension Limits:   Satisfied"));
}

#[test]
fn test_cli_validate_image_missing_file() {
    let binary = env!("CARGO_BIN_EXE_omniscan");
    let output = Command::new(binary)
        .args(["validate-image", "completely_nonexistent_test_12345.png"])
        .output()
        .expect("Failed to execute validate-image");

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("not found") || stderr.contains("Image file not found"));
}

#[test]
fn test_cli_validate_image_corrupt_file() {
    let ws = TempWorkspace::new("corrupt");
    let corrupt_path = ws.file_path("corrupt.png");
    fs::write(&corrupt_path, b"not valid image data").unwrap();

    let binary = env!("CARGO_BIN_EXE_omniscan");
    let output = Command::new(binary)
        .args(["validate-image", corrupt_path.to_str().unwrap()])
        .output()
        .expect("Failed to execute validate-image");

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Failed to decode image"));
}

#[test]
fn test_cli_validate_image_unsupported_extension() {
    let ws = TempWorkspace::new("unsupported");
    let txt_path = ws.file_path("sheet.bmp");
    fs::write(&txt_path, b"bitmap").unwrap();

    let binary = env!("CARGO_BIN_EXE_omniscan");
    let output = Command::new(binary)
        .args(["validate-image", txt_path.to_str().unwrap()])
        .output()
        .expect("Failed to execute validate-image");

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Unsupported image format 'bmp'"));
}

#[test]
fn test_cli_preprocess_both_methods() {
    let ws = TempWorkspace::new("prep_both");
    let img_path = ws.create_synthetic_png("exam_sheet.png", 50, 50);
    let out_dir = ws.file_path("output_both");

    let binary = env!("CARGO_BIN_EXE_omniscan");
    let output = Command::new(binary)
        .args([
            "preprocess",
            img_path.to_str().unwrap(),
            "--output",
            out_dir.to_str().unwrap(),
            "--method",
            "both",
        ])
        .output()
        .expect("Failed to execute preprocess");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Preprocessing completed successfully"));

    // Verify output artifacts
    let gray_path = out_dir.join("grayscale.png");
    let otsu_path = out_dir.join("otsu.png");
    let adaptive_path = out_dir.join("adaptive.png");
    let diag_path = out_dir.join("diagnostics.json");

    assert!(gray_path.exists(), "grayscale.png must exist");
    assert!(otsu_path.exists(), "otsu.png must exist");
    assert!(adaptive_path.exists(), "adaptive.png must exist");
    assert!(diag_path.exists(), "diagnostics.json must exist");

    // Verify diagnostics JSON
    let diag_json = fs::read_to_string(&diag_path).unwrap();
    let diag: PreprocessingDiagnostics = serde_json::from_str(&diag_json).unwrap();
    assert_eq!(
        diag.schema_version,
        PREPROCESSING_DIAGNOSTICS_SCHEMA_VERSION
    );
    assert_eq!(diag.width, 50);
    assert_eq!(diag.height, 50);
    assert_eq!(diag.method, "both");
    assert!(diag.otsu_threshold.is_some());
    assert!(diag.adaptive_block_size.is_some());
    assert!(diag.adaptive_c.is_some());

    // Verify decoded output image dimensions and binary pixel values
    let decoded_otsu = decode_image(&otsu_path).unwrap();
    assert_eq!(decoded_otsu.width(), 50);
    assert_eq!(decoded_otsu.height(), 50);
    assert!(decoded_otsu.as_slice().iter().all(|&b| b == 0 || b == 255));

    let decoded_adapt = decode_image(&adaptive_path).unwrap();
    assert_eq!(decoded_adapt.width(), 50);
    assert_eq!(decoded_adapt.height(), 50);
    assert!(decoded_adapt.as_slice().iter().all(|&b| b == 0 || b == 255));

    // Verify timing fields are non-negative and benchmark_micros is absent when not benchmarked
    assert!(diag.timing_micros.decode_micros > 0);
    assert!(diag.timing_micros.grayscale_micros > 0);
    assert!(diag.timing_micros.total_pipeline_micros > 0);
    assert!(diag.timing_micros.benchmark_micros.is_none());
}

#[test]
fn test_cli_preprocess_otsu_only() {
    let ws = TempWorkspace::new("prep_otsu");
    let img_path = ws.create_synthetic_png("exam_otsu.png", 40, 40);
    let out_dir = ws.file_path("output_otsu");

    let binary = env!("CARGO_BIN_EXE_omniscan");
    let output = Command::new(binary)
        .args([
            "preprocess",
            img_path.to_str().unwrap(),
            "--output",
            out_dir.to_str().unwrap(),
            "--method",
            "otsu",
        ])
        .output()
        .expect("Failed to execute preprocess otsu");

    assert!(output.status.success());
    assert!(out_dir.join("grayscale.png").exists());
    assert!(out_dir.join("otsu.png").exists());
    assert!(!out_dir.join("adaptive.png").exists());
    assert!(out_dir.join("diagnostics.json").exists());
}

#[test]
fn test_cli_preprocess_adaptive_only() {
    let ws = TempWorkspace::new("prep_adapt");
    let img_path = ws.create_synthetic_png("exam_adapt.png", 40, 40);
    let out_dir = ws.file_path("output_adapt");

    let binary = env!("CARGO_BIN_EXE_omniscan");
    let output = Command::new(binary)
        .args([
            "preprocess",
            img_path.to_str().unwrap(),
            "--output",
            out_dir.to_str().unwrap(),
            "--method",
            "adaptive",
            "--block-size",
            "15",
            "--c",
            "8.5",
        ])
        .output()
        .expect("Failed to execute preprocess adaptive");

    assert!(output.status.success());
    assert!(out_dir.join("grayscale.png").exists());
    assert!(!out_dir.join("otsu.png").exists());
    assert!(out_dir.join("adaptive.png").exists());
    assert!(out_dir.join("diagnostics.json").exists());
}

#[test]
fn test_cli_preprocess_benchmark_mode() {
    let ws = TempWorkspace::new("prep_bench");
    let img_path = ws.create_synthetic_png("bench_sheet.png", 35, 35);
    let out_dir = ws.file_path("output_bench");

    let binary = env!("CARGO_BIN_EXE_omniscan");
    let output = Command::new(binary)
        .args([
            "preprocess",
            img_path.to_str().unwrap(),
            "--output",
            out_dir.to_str().unwrap(),
            "--benchmark",
            "--benchmark-iterations",
            "5",
        ])
        .output()
        .expect("Failed to execute preprocess with benchmark");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Benchmark Results"));
    assert!(stdout.contains("Throughput:"));
    assert!(stdout.contains("Total Latency:"));

    let diag_json = fs::read_to_string(out_dir.join("diagnostics.json")).unwrap();
    let diag: PreprocessingDiagnostics = serde_json::from_str(&diag_json).unwrap();
    assert!(diag.benchmark.is_some());
    let bench = diag.benchmark.unwrap();
    assert_eq!(bench.measured_iterations, 5);
    assert!(bench.throughput_images_per_sec > 0.0);
    assert!(bench.latency_micros.min_micros > 0.0);

    // Verify benchmark_micros is explicitly captured in TimingBreakdown
    assert!(diag.timing_micros.benchmark_micros.is_some());
    assert!(diag.timing_micros.benchmark_micros.unwrap() > 0);
    assert!(
        diag.timing_micros.total_pipeline_micros >= diag.timing_micros.benchmark_micros.unwrap()
    );
}

#[test]
fn test_cli_preprocess_invalid_adaptive_block_size() {
    let ws = TempWorkspace::new("invalid_block");
    let img_path = ws.create_synthetic_png("exam_invalid.png", 40, 40);
    let out_dir = ws.file_path("out_err");

    let binary = env!("CARGO_BIN_EXE_omniscan");
    let output = Command::new(binary)
        .args([
            "preprocess",
            img_path.to_str().unwrap(),
            "--output",
            out_dir.to_str().unwrap(),
            "--method",
            "adaptive",
            "--block-size",
            "4", // even number, invalid!
        ])
        .output()
        .expect("Failed to execute preprocess");

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Block size must be odd"));
}

#[test]
fn test_cli_preprocess_input_unchanged_and_collision_protection() {
    let ws = TempWorkspace::new("protection");
    let img_path = ws.create_synthetic_png("grayscale.png", 30, 30);
    let initial_content = fs::read(&img_path).unwrap();

    // Attempting to output into the same directory where input file is named grayscale.png must fail
    let binary = env!("CARGO_BIN_EXE_omniscan");
    let output = Command::new(binary)
        .args([
            "preprocess",
            img_path.to_str().unwrap(),
            "--output",
            ws.dir.to_str().unwrap(),
        ])
        .output()
        .expect("Failed to execute preprocess collision check");

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("overwrite original input file"));

    // Verify input file was not modified
    let after_content = fs::read(&img_path).unwrap();
    assert_eq!(initial_content, after_content);
}

#[test]
fn test_cli_preprocess_partial_artifact_write_failure_behavior() {
    let ws = TempWorkspace::new("partial_fail");
    let img_path = ws.create_synthetic_png("sheet.png", 30, 30);
    let out_dir = ws.file_path("out_partial");
    fs::create_dir_all(&out_dir).unwrap();

    // Create a directory where otsu.png would be written, causing save_binary_image to fail
    let conflicting_otsu_dir = out_dir.join("otsu.png");
    fs::create_dir(&conflicting_otsu_dir).unwrap();

    let binary = env!("CARGO_BIN_EXE_omniscan");
    let output = Command::new(binary)
        .args([
            "preprocess",
            img_path.to_str().unwrap(),
            "--output",
            out_dir.to_str().unwrap(),
            "--method",
            "both",
        ])
        .output()
        .expect("Failed to execute preprocess partial failure check");

    // Command must report failure to CLI
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Error: Image processing error:"));

    // grayscale.png was written before otsu.png, so it remains on disk
    assert!(out_dir.join("grayscale.png").exists());

    // Crucial guarantee: diagnostics.json must NEVER be emitted on failure
    assert!(
        !out_dir.join("diagnostics.json").exists(),
        "diagnostics.json must not exist when pipeline fails"
    );
}

#[test]
fn test_cli_preprocess_overwrites_existing_artifacts_in_output_dir() {
    let ws = TempWorkspace::new("overwrite_ok");
    let img_path = ws.create_synthetic_png("exam.png", 32, 32);
    let out_dir = ws.file_path("out_overwrite");

    let binary = env!("CARGO_BIN_EXE_omniscan");

    // Run 1: initial creation
    let output1 = Command::new(binary)
        .args([
            "preprocess",
            img_path.to_str().unwrap(),
            "--output",
            out_dir.to_str().unwrap(),
            "--method",
            "otsu",
        ])
        .output()
        .expect("Failed to execute initial preprocess");
    assert!(output1.status.success());
    assert!(out_dir.join("grayscale.png").exists());

    // Corrupt grayscale.png with dummy marker bytes
    let dummy_marker = b"STALE_OLD_ARTIFACT_DATA";
    fs::write(out_dir.join("grayscale.png"), dummy_marker).unwrap();
    assert_eq!(
        fs::read(out_dir.join("grayscale.png")).unwrap(),
        dummy_marker
    );

    // Run 2: rerun into same directory must overwrite in-place
    let output2 = Command::new(binary)
        .args([
            "preprocess",
            img_path.to_str().unwrap(),
            "--output",
            out_dir.to_str().unwrap(),
            "--method",
            "otsu",
        ])
        .output()
        .expect("Failed to execute second preprocess");
    assert!(output2.status.success());

    // Verify grayscale.png was cleanly overwritten with valid decodable image data
    let overwritten_data = fs::read(out_dir.join("grayscale.png")).unwrap();
    assert_ne!(overwritten_data, dummy_marker);
    let decoded = decode_image(&out_dir.join("grayscale.png")).expect("Must decode successfully");
    assert_eq!(decoded.width(), 32);
    assert_eq!(decoded.height(), 32);
}

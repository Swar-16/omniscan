//! Command-line interface definitions and parser for OMNISCAN.

use clap::{Parser, Subcommand, ValueEnum};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Preprocessing binarization method choice.
#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum, Default, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PreprocessMethod {
    /// Global Otsu binarization
    Otsu,
    /// Spatially varying adaptive Gaussian binarization
    Adaptive,
    /// Both Otsu and adaptive Gaussian thresholding
    #[default]
    Both,
}

/// OMNISCAN: A local-first, configurable Optical Mark Recognition (OMR) platform.
#[derive(Debug, Parser)]
#[command(
    name = "omniscan",
    author,
    version,
    about = "Local-first, configurable Optical Mark Recognition (OMR) platform",
    long_about = "OMNISCAN provides deterministic, explainable Optical Mark Recognition for academic and institutional assessment sheets."
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand, PartialEq)]
pub enum Commands {
    /// Validate an input image file for format, dimensions, and readability
    ValidateImage {
        /// Path to the scan/image file to validate
        image: PathBuf,
    },

    /// Preprocess an image: convert to grayscale, apply thresholding, and export diagnostics
    Preprocess {
        /// Path to the input image file
        image: PathBuf,

        /// Directory for diagnostic image and JSON output (existing artifact files will be overwritten in-place)
        #[arg(short, long)]
        output: PathBuf,

        /// Preprocessing thresholding method to apply (otsu, adaptive, both)
        #[arg(short, long, value_enum, default_value_t = PreprocessMethod::Both)]
        method: PreprocessMethod,

        /// Neighborhood block size for adaptive Gaussian thresholding (must be odd and > 1)
        #[arg(long, default_value_t = 21)]
        block_size: i32,

        /// Constant subtracted from the Gaussian-weighted mean in adaptive thresholding
        #[arg(long, default_value_t = 10.0)]
        c: f64,

        /// Run in benchmark mode over multiple iterations
        #[arg(long, default_value_t = false)]
        benchmark: bool,

        /// Number of measured benchmark iterations
        #[arg(long, default_value_t = 10)]
        benchmark_iterations: usize,
    },

    /// Inspect a sheet template definition and verify geometry bounds
    InspectTemplate {
        /// Path to the template JSON file
        template: PathBuf,
    },

    /// Process a single answer sheet image
    Process {
        /// Path to the input image file
        image: PathBuf,

        /// Path to the template configuration JSON
        #[arg(short, long)]
        template: PathBuf,

        /// Path to the answer key JSON
        #[arg(short, long)]
        answer_key: Option<PathBuf>,

        /// Directory for diagnostic image and JSON output
        #[arg(short, long)]
        output: Option<PathBuf>,
    },

    /// Batch process answer sheet images in a directory
    Batch {
        /// Directory containing answer sheet images
        input_dir: PathBuf,

        /// Path to the template configuration JSON
        #[arg(short, long)]
        template: PathBuf,

        /// Path to the answer key JSON
        #[arg(short, long)]
        answer_key: Option<PathBuf>,

        /// Directory for batch CSV/JSON reports and diagnostics
        #[arg(short, long)]
        output_dir: PathBuf,
    },

    /// Evaluate recognized responses against an answer key
    Evaluate {
        /// Path to responses JSON file
        responses: PathBuf,

        /// Path to the answer key JSON file
        #[arg(short, long)]
        answer_key: PathBuf,
    },

    /// Run benchmark over a dataset manifest
    Benchmark {
        /// Path to benchmark manifest JSON file
        manifest: PathBuf,

        /// Path to the template configuration JSON
        #[arg(short, long)]
        template: PathBuf,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_validate_image_command() {
        let args = ["omniscan", "validate-image", "sample.jpg"];
        let cli = Cli::try_parse_from(args).expect("Failed to parse validate-image");
        assert_eq!(
            cli.command,
            Commands::ValidateImage {
                image: PathBuf::from("sample.jpg")
            }
        );
    }

    #[test]
    fn test_parse_inspect_template_command() {
        let args = [
            "omniscan",
            "inspect-template",
            "configs/templates/sheet.json",
        ];
        let cli = Cli::try_parse_from(args).expect("Failed to parse inspect-template");
        assert_eq!(
            cli.command,
            Commands::InspectTemplate {
                template: PathBuf::from("configs/templates/sheet.json")
            }
        );
    }

    #[test]
    fn test_parse_process_command() {
        let args = [
            "omniscan",
            "process",
            "exam.png",
            "--template",
            "sheet.json",
            "--answer-key",
            "key.json",
            "--output",
            "out_dir",
        ];
        let cli = Cli::try_parse_from(args).expect("Failed to parse process command");
        assert_eq!(
            cli.command,
            Commands::Process {
                image: PathBuf::from("exam.png"),
                template: PathBuf::from("sheet.json"),
                answer_key: Some(PathBuf::from("key.json")),
                output: Some(PathBuf::from("out_dir")),
            }
        );
    }

    #[test]
    fn test_parse_preprocess_command_defaults() {
        let args = ["omniscan", "preprocess", "sheet.png", "--output", "out_dir"];
        let cli = Cli::try_parse_from(args).expect("Failed to parse preprocess command");
        assert_eq!(
            cli.command,
            Commands::Preprocess {
                image: PathBuf::from("sheet.png"),
                output: PathBuf::from("out_dir"),
                method: PreprocessMethod::Both,
                block_size: 21,
                c: 10.0,
                benchmark: false,
                benchmark_iterations: 10,
            }
        );
    }

    #[test]
    fn test_parse_preprocess_command_custom() {
        let args = [
            "omniscan",
            "preprocess",
            "scan.jpg",
            "--output",
            "artifacts",
            "--method",
            "adaptive",
            "--block-size",
            "31",
            "--c",
            "15.5",
            "--benchmark",
            "--benchmark-iterations",
            "25",
        ];
        let cli = Cli::try_parse_from(args).expect("Failed to parse custom preprocess command");
        assert_eq!(
            cli.command,
            Commands::Preprocess {
                image: PathBuf::from("scan.jpg"),
                output: PathBuf::from("artifacts"),
                method: PreprocessMethod::Adaptive,
                block_size: 31,
                c: 15.5,
                benchmark: true,
                benchmark_iterations: 25,
            }
        );
    }
}

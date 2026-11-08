//! Command-line interface definitions and parser for OMNISCAN.

use clap::{Parser, Subcommand};
use std::path::PathBuf;

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

#[derive(Debug, Subcommand, PartialEq, Eq)]
pub enum Commands {
    /// Validate an input image file for format, dimensions, and readability
    ValidateImage {
        /// Path to the scan/image file to validate
        image: PathBuf,
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
}

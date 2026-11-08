//! OMNISCAN CLI entry point.

use clap::Parser;
use omniscan::cli::{Cli, Commands};
use omniscan::error::Result;
use std::process::ExitCode;

fn run(cli: Cli) -> Result<()> {
    match cli.command {
        Commands::ValidateImage { image } => {
            eprintln!("[omniscan] Validating image: {}", image.display());
            println!(
                "Image validation stub: '{}' is queued for M2 implementation",
                image.display()
            );
            Ok(())
        }
        Commands::InspectTemplate { template } => {
            eprintln!("[omniscan] Inspecting template: {}", template.display());
            println!(
                "Template inspection stub: '{}' is queued for M1 implementation",
                template.display()
            );
            Ok(())
        }
        Commands::Process {
            image,
            template,
            answer_key,
            output,
        } => {
            eprintln!(
                "[omniscan] Processing image '{}' with template '{}'",
                image.display(),
                template.display()
            );
            if let Some(key) = answer_key {
                eprintln!("[omniscan] Answer key: '{}'", key.display());
            }
            if let Some(out) = output {
                eprintln!("[omniscan] Output directory: '{}'", out.display());
            }
            println!("Process stub: End-to-end processing pipeline scheduled for M7");
            Ok(())
        }
        Commands::Batch {
            input_dir,
            template,
            answer_key,
            output_dir,
        } => {
            eprintln!(
                "[omniscan] Batch processing directory '{}' with template '{}'",
                input_dir.display(),
                template.display()
            );
            if let Some(key) = answer_key {
                eprintln!("[omniscan] Answer key: '{}'", key.display());
            }
            eprintln!(
                "[omniscan] Batch output destination: '{}'",
                output_dir.display()
            );
            println!("Batch stub: Batch processing scheduled for M7");
            Ok(())
        }
        Commands::Evaluate {
            responses,
            answer_key,
        } => {
            eprintln!(
                "[omniscan] Evaluating responses '{}' against key '{}'",
                responses.display(),
                answer_key.display()
            );
            println!("Evaluation stub: Answer key scoring engine scheduled for M6");
            Ok(())
        }
        Commands::Benchmark { manifest, template } => {
            eprintln!(
                "[omniscan] Benchmarking manifest '{}' with template '{}'",
                manifest.display(),
                template.display()
            );
            println!("Benchmark stub: Dataset benchmarking scheduled for M8");
            Ok(())
        }
    }
}

fn main() -> ExitCode {
    let cli = Cli::parse();
    match run(cli) {
        Ok(()) => ExitCode::SUCCESS,
        Err(err) => {
            eprintln!("Error: {err}");
            ExitCode::FAILURE
        }
    }
}

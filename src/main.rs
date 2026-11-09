//! OMNISCAN CLI entry point.

use clap::Parser;
use omniscan::cli::{Cli, Commands};
use omniscan::error::{ImageError, OmniscanError, Result};
use omniscan::imaging::{
    decode_image, execute_preprocess, is_supported_format, PreprocessConfig, MAX_IMAGE_DIMENSION,
    MAX_IMAGE_PIXEL_BYTES,
};
use omniscan::layout::Template;
use std::fs;
use std::path::Path;
use std::process::ExitCode;

fn run(cli: Cli) -> Result<()> {
    match cli.command {
        Commands::ValidateImage { image } => {
            validate_image_command(&image)?;
            Ok(())
        }
        Commands::Preprocess {
            image,
            output,
            method,
            block_size,
            c,
            benchmark,
            benchmark_iterations,
        } => {
            let config = PreprocessConfig {
                input_path: image,
                output_dir: output,
                method,
                block_size,
                c,
                benchmark,
                benchmark_iterations,
            };
            preprocess_command(&config)?;
            Ok(())
        }
        Commands::InspectTemplate { template } => {
            let t = Template::from_file(&template)?;
            let summary = t.summary();
            println!("{summary}");
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

fn validate_image_command(image: &Path) -> Result<()> {
    if !is_supported_format(image) {
        let ext = image.extension().and_then(|s| s.to_str()).unwrap_or("none");
        return Err(OmniscanError::from(ImageError::UnsupportedFormat {
            path: image.display().to_string(),
            format: ext.to_string(),
        }));
    }

    if !image.exists() {
        return Err(OmniscanError::from(ImageError::FileNotFound {
            path: image.display().to_string(),
        }));
    }

    let file_size_bytes =
        fs::metadata(image)
            .map(|m| m.len())
            .map_err(|e| ImageError::IoError {
                path: image.display().to_string(),
                message: e.to_string(),
            })?;

    let decoded = decode_image(image)?;

    println!("Image Validation: PASSED");
    println!("  Path:               {}", image.display());
    println!(
        "  Dimensions:         {} x {} px",
        decoded.width(),
        decoded.height()
    );
    println!("  Pixel Format:       {:?}", decoded.format());
    println!("  Channels:           {}", decoded.channels());
    println!("  Row Stride:         {} bytes", decoded.stride());
    println!(
        "  Container Size:     {:.2} KB ({} bytes compressed)",
        file_size_bytes as f64 / 1024.0,
        file_size_bytes
    );
    println!(
        "  Decoded Memory:     {:.2} KB ({} bytes uncompressed)",
        decoded.as_slice().len() as f64 / 1024.0,
        decoded.as_slice().len()
    );
    println!(
        "  Dimension Limits:   Satisfied (max {} px)",
        MAX_IMAGE_DIMENSION
    );
    println!(
        "  Allocation Limit:   Satisfied (max {:.2} MB)",
        MAX_IMAGE_PIXEL_BYTES as f64 / (1024.0 * 1024.0)
    );
    Ok(())
}

fn preprocess_command(config: &PreprocessConfig) -> Result<()> {
    let result = execute_preprocess(config)?;

    println!("[omniscan] Preprocessing completed successfully.");
    println!("  Input:        {}", config.input_path.display());
    println!("  Method:       {:?}", config.method);
    println!(
        "  Dimensions:   {} x {} px",
        result.grayscale.width(),
        result.grayscale.height()
    );
    println!("  Output Dir:   {}", config.output_dir.display());
    println!("  Artifacts:");
    for (name, path) in &result.diagnostics.output_artifacts {
        println!("    - {}: {}", name, path);
    }

    if let Some(ref bench) = result.diagnostics.benchmark {
        println!();
        println!(
            "Benchmark Results ({} measured iterations, {} warm-up):",
            bench.measured_iterations, bench.warmup_iterations
        );
        println!(
            "  Throughput: {:.2} images/sec",
            bench.throughput_images_per_sec
        );
        println!(
            "  Total Latency:    min: {:.2} us | median: {:.2} us | mean: {:.2} us | max: {:.2} us",
            bench.latency_micros.min_micros,
            bench.latency_micros.median_micros,
            bench.latency_micros.mean_micros,
            bench.latency_micros.max_micros
        );
        if let Some(ref g) = bench.grayscale_latency_micros {
            println!(
                "  Grayscale:        min: {:.2} us | median: {:.2} us | mean: {:.2} us | max: {:.2} us",
                g.min_micros, g.median_micros, g.mean_micros, g.max_micros
            );
        }
        if let Some(ref o) = bench.otsu_latency_micros {
            println!(
                "  Otsu:             min: {:.2} us | median: {:.2} us | mean: {:.2} us | max: {:.2} us",
                o.min_micros, o.median_micros, o.mean_micros, o.max_micros
            );
        }
        if let Some(ref a) = bench.adaptive_latency_micros {
            println!(
                "  Adaptive:         min: {:.2} us | median: {:.2} us | mean: {:.2} us | max: {:.2} us",
                a.min_micros, a.median_micros, a.mean_micros, a.max_micros
            );
        }
    }

    Ok(())
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

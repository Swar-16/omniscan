use std::process::Command;

#[test]
fn test_cli_help() {
    let binary = env!("CARGO_BIN_EXE_omniscan");
    let output = Command::new(binary)
        .arg("--help")
        .output()
        .expect("Failed to execute omniscan --help");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(
        stdout.contains("OMNISCAN provides deterministic, explainable Optical Mark Recognition")
    );
    assert!(stdout.contains("validate-image"));
    assert!(stdout.contains("preprocess"));
    assert!(stdout.contains("inspect-template"));
    assert!(stdout.contains("process"));
    assert!(stdout.contains("batch"));
    assert!(stdout.contains("evaluate"));
    assert!(stdout.contains("benchmark"));
}

#[test]
fn test_cli_short_help() {
    let binary = env!("CARGO_BIN_EXE_omniscan");
    let output = Command::new(binary)
        .arg("-h")
        .output()
        .expect("Failed to execute omniscan -h");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Local-first, configurable Optical Mark Recognition"));
}

#[test]
fn test_cli_version() {
    let binary = env!("CARGO_BIN_EXE_omniscan");
    let output = Command::new(binary)
        .arg("--version")
        .output()
        .expect("Failed to execute omniscan --version");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("0.1.0"));
}

#[test]
fn test_subcommand_validate_image_help() {
    let binary = env!("CARGO_BIN_EXE_omniscan");
    let output = Command::new(binary)
        .args(["validate-image", "--help"])
        .output()
        .expect("Failed to execute omniscan validate-image --help");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Validate an input image file"));
    assert!(stdout.contains("<IMAGE>"));
}

#[test]
fn test_subcommand_process_help() {
    let binary = env!("CARGO_BIN_EXE_omniscan");
    let output = Command::new(binary)
        .args(["process", "--help"])
        .output()
        .expect("Failed to execute omniscan process --help");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("--template"));
    assert!(stdout.contains("--answer-key"));
    assert!(stdout.contains("--output"));
}

#[test]
fn test_subcommand_inspect_template_success() {
    let binary = env!("CARGO_BIN_EXE_omniscan");
    let output = Command::new(binary)
        .args([
            "inspect-template",
            "configs/templates/synthetic-standard-30q-v1.json",
        ])
        .output()
        .expect("Failed to execute inspect-template");

    assert!(output.status.success());
    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains("Template ID:       synthetic-standard-30q-v1"));
    assert!(stdout.contains("Questions Total:   30 questions"));
    assert!(stdout.contains("Status:            VALID"));
}

#[test]
fn test_subcommand_inspect_template_failure() {
    let binary = env!("CARGO_BIN_EXE_omniscan");
    let output = Command::new(binary)
        .args([
            "inspect-template",
            "tests/fixtures/templates/invalid_duplicate_question_id.json",
        ])
        .output()
        .expect("Failed to execute inspect-template on invalid template");

    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("Duplicate question ID: 'Q1'"));
}

#[test]
fn test_invalid_argument_fails() {
    let binary = env!("CARGO_BIN_EXE_omniscan");
    let output = Command::new(binary)
        .arg("--non-existent-flag")
        .output()
        .expect("Failed to execute omniscan with invalid argument");

    assert!(!output.status.success());
}

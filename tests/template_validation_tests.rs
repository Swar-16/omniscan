use omniscan::core::Orientation;
use omniscan::layout::Template;
use std::path::PathBuf;

fn fixture_path(subpath: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join("templates")
        .join(subpath)
}

#[test]
fn test_synthetic_reference_template_is_valid() {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("configs")
        .join("templates")
        .join("synthetic-standard-30q-v1.json");

    let template = Template::from_file(&path).expect("Synthetic reference template must be valid");
    assert_eq!(template.schema_version, 1);
    assert_eq!(template.template_id, "synthetic-standard-30q-v1");
    assert_eq!(template.page.orientation, Orientation::Portrait);
    assert_eq!(template.registration.markers.len(), 4);

    let summary = template.summary();
    assert_eq!(summary.total_questions, 30);
    assert_eq!(summary.single_choice_count, 30);
    assert_eq!(summary.multiple_select_count, 0);
    assert_eq!(summary.min_options, 4);
    assert_eq!(summary.max_options, 4);
    assert!(summary.candidate_id_configured);

    // Verify all 30 questions are present and ordered
    let mut question_numbers: Vec<u32> = template.sections[0]
        .questions
        .iter()
        .map(|q| q.question_number)
        .collect();
    question_numbers.sort_unstable();
    assert_eq!(question_numbers, (1..=30).collect::<Vec<u32>>());
}

#[test]
fn test_minimal_valid_template() {
    let path = fixture_path("valid_minimal_template.json");
    let template = Template::from_file(&path).expect("Minimal template should validate");
    assert_eq!(template.template_id, "valid-minimal-v1");
    assert_eq!(template.sections[0].questions.len(), 1);
}

#[test]
fn test_reject_out_of_bounds_template() {
    let path = fixture_path("invalid_out_of_bounds.json");
    let err = Template::from_file(&path).unwrap_err();
    let msg = err.to_string();
    assert!(
        msg.contains("exceeds page bounds") || msg.contains("boundaries exceed"),
        "Unexpected error: {msg}"
    );
}

#[test]
fn test_reject_invalid_radii_template() {
    let path = fixture_path("invalid_radii_and_dimensions.json");
    let err = Template::from_file(&path).unwrap_err();
    assert!(err.to_string().contains("radius must be positive"));
}

#[test]
fn test_reject_duplicate_question_id() {
    let path = fixture_path("invalid_duplicate_question_id.json");
    let err = Template::from_file(&path).unwrap_err();
    assert!(err.to_string().contains("Duplicate question ID: 'Q1'"));
}

#[test]
fn test_reject_duplicate_option_label() {
    let path = fixture_path("invalid_duplicate_option_label.json");
    let err = Template::from_file(&path).unwrap_err();
    assert!(err.to_string().contains("has duplicate option label 'A'"));
}

#[test]
fn test_reject_overlapping_bubbles() {
    let path = fixture_path("invalid_overlapping_bubbles.json");
    let err = Template::from_file(&path).unwrap_err();
    assert!(err
        .to_string()
        .contains("has overlapping options 'A' and 'B'"));
}

#[test]
fn test_reject_missing_required_fields() {
    let path = fixture_path("invalid_missing_required_fields.json");
    let err = Template::from_file(&path).unwrap_err();
    assert!(matches!(err, omniscan::error::OmniscanError::Json(_)));
}

#[test]
fn test_reject_unsupported_schema_version() {
    let path = fixture_path("invalid_unsupported_schema_version.json");
    let err = Template::from_file(&path).unwrap_err();
    assert!(err.to_string().contains("Unsupported schema_version 99"));
}

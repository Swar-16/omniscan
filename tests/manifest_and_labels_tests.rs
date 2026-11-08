use omniscan::labels::{GroundTruthSheet, ResponseMark};
use omniscan::manifest::{DatasetManifest, DatasetSplit};
use std::path::PathBuf;

fn fixture_manifest_path(subpath: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join("manifests")
        .join(subpath)
}

fn fixture_labels_path(subpath: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join("labels")
        .join(subpath)
}

#[test]
fn test_valid_manifest_parsing_and_validation() {
    let path = fixture_manifest_path("valid_manifest.json");
    let manifest = DatasetManifest::from_file(&path).expect("Manifest must be valid");
    assert_eq!(manifest.schema_version, 1);
    assert_eq!(manifest.dataset_id, "synthetic-smoke-v1");
    assert_eq!(manifest.records.len(), 2);
    assert_eq!(manifest.records[0].split, Some(DatasetSplit::Train));
    assert_eq!(manifest.records[1].split, Some(DatasetSplit::Test));
}

#[test]
fn test_reject_duplicate_record_id_in_manifest() {
    let path = fixture_manifest_path("invalid_duplicate_record_id.json");
    let err = DatasetManifest::from_file(&path).unwrap_err();
    assert!(err.to_string().contains("Duplicate record ID: 'DUP-001'"));
}

#[test]
fn test_reject_absolute_path_in_manifest() {
    let path = fixture_manifest_path("invalid_absolute_path.json");
    let err = DatasetManifest::from_file(&path).unwrap_err();
    assert!(err
        .to_string()
        .contains("Absolute paths are forbidden for 'image_path'"));
}

#[test]
fn test_reject_path_traversal_in_manifest() {
    let path = fixture_manifest_path("invalid_path_traversal.json");
    let err = DatasetManifest::from_file(&path).unwrap_err();
    assert!(err
        .to_string()
        .contains("Path traversal ('..') is forbidden for 'image_path'"));
}

#[test]
fn test_parse_valid_standard_labels() {
    let path = fixture_labels_path("valid_standard_labels.txt");
    let sheet = GroundTruthSheet::from_file(&path).expect("Standard labels should parse");
    assert_eq!(sheet.candidate_id, Some("CAND-7001".to_string()));
    assert_eq!(sheet.questions.len(), 10);
    assert_eq!(sheet.questions[0].question_id, "Q01");
    assert_eq!(sheet.questions[0].mark, ResponseMark::Single('A'));
    assert_eq!(sheet.questions[9].question_id, "Q10");
    assert_eq!(sheet.questions[9].mark, ResponseMark::Single('B'));
}

#[test]
fn test_parse_labels_with_blanks_and_multiple() {
    let path = fixture_labels_path("valid_with_blanks_and_multiple.txt");
    let sheet = GroundTruthSheet::from_file(&path).expect("Labels with blanks should parse");
    assert_eq!(sheet.candidate_id, Some("CAND-7002".to_string()));
    assert_eq!(sheet.questions.len(), 8);

    assert_eq!(sheet.questions[0].mark, ResponseMark::Single('A'));
    assert_eq!(sheet.questions[1].mark, ResponseMark::Blank);
    assert_eq!(
        sheet.questions[2].mark,
        ResponseMark::Multiple(vec!['A', 'B'])
    );
    assert_eq!(sheet.questions[3].mark, ResponseMark::Blank);
    assert_eq!(
        sheet.questions[4].mark,
        ResponseMark::Ambiguous("faint erasure".to_string())
    );
    assert_eq!(
        sheet.questions[5].mark,
        ResponseMark::Multiple(vec!['B', 'C'])
    );
    assert!(matches!(
        sheet.questions[6].mark,
        ResponseMark::Ambiguous(_)
    ));
    assert_eq!(sheet.questions[7].mark, ResponseMark::Single('D'));
}

#[test]
fn test_reject_malformed_token_in_labels() {
    let path = fixture_labels_path("malformed_token_labels.txt");
    let err = GroundTruthSheet::from_file(&path).unwrap_err();
    assert!(err
        .to_string()
        .contains("Line 3: Unknown or invalid mark token"));
}

#[test]
fn test_reject_duplicate_question_id_in_labels() {
    let path = fixture_labels_path("duplicate_question_labels.txt");
    let err = GroundTruthSheet::from_file(&path).unwrap_err();
    assert!(err
        .to_string()
        .contains("Line 4: Duplicate question identifier 'Q01'"));
}

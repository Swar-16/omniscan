//! Dataset manifest data structures and validation.

use crate::error::{OmniscanError, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::path::{Component, Path};

pub const CURRENT_MANIFEST_SCHEMA_VERSION: u32 = 1;

/// Dataset partition / split assignment.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DatasetSplit {
    Train,
    Validation,
    Test,
    Holdout,
}

/// An individual record in the dataset manifest.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ManifestRecord {
    pub id: String,
    pub image_path: String,
    #[serde(default)]
    pub ground_truth_path: Option<String>,
    #[serde(default)]
    pub template_id: Option<String>,
    #[serde(default)]
    pub cohort: Option<String>,
    #[serde(default)]
    pub split: Option<DatasetSplit>,
    #[serde(default)]
    pub metadata: serde_json::Map<String, serde_json::Value>,
}

/// Machine-readable manifest cataloging a dataset collection.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DatasetManifest {
    pub schema_version: u32,
    pub dataset_id: String,
    pub name: String,
    #[serde(default)]
    pub version: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub source_url: Option<String>,
    pub license: String,
    #[serde(default)]
    pub default_template_id: Option<String>,
    pub records: Vec<ManifestRecord>,
}

impl DatasetManifest {
    /// Deserializes a dataset manifest from a JSON string.
    pub fn from_json_str(json: &str) -> Result<Self> {
        let manifest: Self = serde_json::from_str(json)?;
        Ok(manifest)
    }

    /// Loads, parses, and validates a dataset manifest from a file.
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let manifest = Self::from_json_str(&content)?;
        manifest.validate()?;
        Ok(manifest)
    }

    /// Performs structural and schema validation of the manifest.
    pub fn validate(&self) -> Result<()> {
        if self.schema_version != CURRENT_MANIFEST_SCHEMA_VERSION {
            return Err(OmniscanError::Manifest(format!(
                "Unsupported manifest schema_version {}. Expected {}",
                self.schema_version, CURRENT_MANIFEST_SCHEMA_VERSION
            )));
        }

        if self.dataset_id.trim().is_empty() {
            return Err(OmniscanError::Manifest(
                "dataset_id cannot be empty".to_string(),
            ));
        }

        if self.license.trim().is_empty() {
            return Err(OmniscanError::Manifest(
                "license cannot be empty".to_string(),
            ));
        }

        let mut record_ids = HashSet::new();

        for record in &self.records {
            if record.id.trim().is_empty() {
                return Err(OmniscanError::Manifest(
                    "Record ID cannot be empty".to_string(),
                ));
            }
            if !record_ids.insert(&record.id) {
                return Err(OmniscanError::Manifest(format!(
                    "Duplicate record ID: '{}'",
                    record.id
                )));
            }

            Self::validate_relative_path(&record.image_path, "image_path")?;

            if let Some(gt_path) = &record.ground_truth_path {
                Self::validate_relative_path(gt_path, "ground_truth_path")?;
            }
        }

        Ok(())
    }

    /// Verifies that a path is strictly relative and contains no path traversal.
    fn validate_relative_path(path_str: &str, field_name: &str) -> Result<()> {
        let trimmed = path_str.trim();
        if trimmed.is_empty() {
            return Err(OmniscanError::Manifest(format!(
                "Path for '{}' cannot be empty",
                field_name
            )));
        }

        let path = Path::new(trimmed);

        // Reject absolute paths
        if path.is_absolute()
            || trimmed.starts_with('/')
            || trimmed.starts_with('\\')
            || (trimmed.len() >= 2 && trimmed.chars().nth(1) == Some(':'))
        {
            return Err(OmniscanError::Manifest(format!(
                "Absolute paths are forbidden for '{}': '{}'",
                field_name, trimmed
            )));
        }

        // Reject path traversal ('..')
        for component in path.components() {
            if component == Component::ParentDir {
                return Err(OmniscanError::Manifest(format!(
                    "Path traversal ('..') is forbidden for '{}': '{}'",
                    field_name, trimmed
                )));
            }
        }

        Ok(())
    }

    /// Validates that all referenced image and ground-truth files exist on disk relative to a base directory.
    pub fn validate_files_exist<P: AsRef<Path>>(&self, base_dir: P) -> Result<()> {
        let base = base_dir.as_ref();
        for record in &self.records {
            let img_full = base.join(&record.image_path);
            if !img_full.is_file() {
                return Err(OmniscanError::Manifest(format!(
                    "Record '{}' image file not found: '{}'",
                    record.id,
                    img_full.display()
                )));
            }

            if let Some(gt_rel) = &record.ground_truth_path {
                let gt_full = base.join(gt_rel);
                if !gt_full.is_file() {
                    return Err(OmniscanError::Manifest(format!(
                        "Record '{}' ground truth file not found: '{}'",
                        record.id,
                        gt_full.display()
                    )));
                }
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_manifest() -> DatasetManifest {
        DatasetManifest {
            schema_version: 1,
            dataset_id: "test-ds".to_string(),
            name: "Test Dataset".to_string(),
            version: Some("1.0".to_string()),
            description: None,
            source_url: None,
            license: "MIT".to_string(),
            default_template_id: None,
            records: vec![ManifestRecord {
                id: "REC01".to_string(),
                image_path: "images/sample.jpg".to_string(),
                ground_truth_path: Some("labels/sample.txt".to_string()),
                template_id: None,
                cohort: None,
                split: Some(DatasetSplit::Test),
                metadata: serde_json::Map::new(),
            }],
        }
    }

    #[test]
    fn test_valid_manifest() {
        let m = sample_manifest();
        assert!(m.validate().is_ok());
    }

    #[test]
    fn test_duplicate_record_id() {
        let mut m = sample_manifest();
        let dup = m.records[0].clone();
        m.records.push(dup);
        let err = m.validate().unwrap_err();
        assert!(err.to_string().contains("Duplicate record ID: 'REC01'"));
    }

    #[test]
    fn test_absolute_path_rejected() {
        let mut m = sample_manifest();
        m.records[0].image_path = "/var/data/image.png".to_string();
        let err = m.validate().unwrap_err();
        assert!(err.to_string().contains("Absolute paths are forbidden"));

        m.records[0].image_path = "C:\\data\\image.png".to_string();
        let err2 = m.validate().unwrap_err();
        assert!(err2.to_string().contains("Absolute paths are forbidden"));
    }

    #[test]
    fn test_path_traversal_rejected() {
        let mut m = sample_manifest();
        m.records[0].image_path = "images/../../secret.png".to_string();
        let err = m.validate().unwrap_err();
        assert!(err
            .to_string()
            .contains("Path traversal ('..') is forbidden"));
    }
}

//! Normalized OMR template representation and validation.

use crate::core::{Circle, Orientation, Point2D, Size2D};
use crate::error::{OmniscanError, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::fmt;
use std::path::Path;

pub const CURRENT_TEMPLATE_SCHEMA_VERSION: u32 = 1;

/// Shape of registration markers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MarkerShape {
    Square,
    Circle,
    Cross,
}

/// A physical registration marker or fiducial on the sheet.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MarkerLayout {
    pub id: String,
    pub center: Point2D,
    pub size: Size2D,
    pub shape: MarkerShape,
}

/// Strategy for detecting sheet alignment and orientation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RegistrationStrategy {
    FiducialMarkers,
    PageCorners,
    BoundingBox,
}

/// Registration configuration.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RegistrationLayout {
    pub strategy: RegistrationStrategy,
    pub markers: Vec<MarkerLayout>,
}

/// Page dimensions and reference aspect ratio.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct PageLayout {
    pub reference_width: u32,
    pub reference_height: u32,
    pub aspect_ratio: f64,
    pub orientation: Orientation,
}

/// Candidate identifier bubble grid layout.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CandidateIdLayout {
    pub id: String,
    pub digit_columns: usize,
    pub digits_per_column: usize,
    pub origin: Point2D,
    pub column_spacing: f64,
    pub row_spacing: f64,
    pub bubble_radius: f64,
}

/// Shape of answer option mark bubbles.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OptionShape {
    Circle,
    Oval,
    Rectangle,
}

/// A single response bubble option (e.g. 'A', 'B', 'C', 'D').
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OptionLayout {
    pub label: String,
    pub center: Point2D,
    pub radius: f64,
    pub shape: OptionShape,
}

/// Supported question response types.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum QuestionType {
    SingleChoice,
    MultipleSelect,
}

/// Geometry and properties for an individual assessment question.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct QuestionLayout {
    pub question_id: String,
    pub question_number: u32,
    pub question_type: QuestionType,
    pub options: Vec<OptionLayout>,
}

/// A logical section or column group of questions.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SectionLayout {
    pub section_id: String,
    pub name: String,
    pub columns: usize,
    pub questions: Vec<QuestionLayout>,
}

/// Complete, versioned OMR template definition in normalized coordinates.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Template {
    pub schema_version: u32,
    pub template_id: String,
    pub name: String,
    pub description: String,
    pub page: PageLayout,
    pub registration: RegistrationLayout,
    #[serde(default)]
    pub candidate_id: Option<CandidateIdLayout>,
    pub sections: Vec<SectionLayout>,
}

impl Template {
    /// Loads and parses a template from a JSON string.
    pub fn from_json_str(json: &str) -> Result<Self> {
        let template: Self = serde_json::from_str(json)?;
        Ok(template)
    }

    /// Loads, parses, and validates a template from a JSON file.
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let template = Self::from_json_str(&content)?;
        template.validate()?;
        Ok(template)
    }

    /// Validates all structural, geometric, and uniqueness constraints.
    pub fn validate(&self) -> Result<()> {
        // 1. Schema version check
        if self.schema_version != CURRENT_TEMPLATE_SCHEMA_VERSION {
            return Err(OmniscanError::Template(format!(
                "Unsupported schema_version {}. Expected {}",
                self.schema_version, CURRENT_TEMPLATE_SCHEMA_VERSION
            )));
        }

        if self.template_id.trim().is_empty() {
            return Err(OmniscanError::Template(
                "Template template_id cannot be empty".to_string(),
            ));
        }

        // 2. Page layout validation
        if self.page.reference_width == 0 || self.page.reference_height == 0 {
            return Err(OmniscanError::Template(
                "Page reference_width and reference_height must be strictly positive".to_string(),
            ));
        }
        if self.page.aspect_ratio <= 0.0 {
            return Err(OmniscanError::Template(
                "Page aspect_ratio must be strictly positive".to_string(),
            ));
        }
        let computed_ratio = self.page.reference_width as f64 / self.page.reference_height as f64;
        if (computed_ratio - self.page.aspect_ratio).abs() > 0.01 {
            return Err(OmniscanError::Template(format!(
                "Inconsistent page aspect_ratio: declared {:.4}, computed from dimensions ({}/{}) is {:.4}",
                self.page.aspect_ratio, self.page.reference_width, self.page.reference_height, computed_ratio
            )));
        }

        // 3. Registration markers validation
        let mut marker_ids = HashSet::new();
        for marker in &self.registration.markers {
            if marker.id.trim().is_empty() {
                return Err(OmniscanError::Template(
                    "Registration marker ID cannot be empty".to_string(),
                ));
            }
            if !marker_ids.insert(&marker.id) {
                return Err(OmniscanError::Template(format!(
                    "Duplicate registration marker ID: '{}'",
                    marker.id
                )));
            }
            if !marker.size.is_positive() {
                return Err(OmniscanError::Template(format!(
                    "Marker '{}' size dimensions must be strictly positive",
                    marker.id
                )));
            }
            let half_w = marker.size.width / 2.0;
            let half_h = marker.size.height / 2.0;
            if (marker.center.x - half_w) < -1e-9
                || (marker.center.x + half_w) > 1.0 + 1e-9
                || (marker.center.y - half_h) < -1e-9
                || (marker.center.y + half_h) > 1.0 + 1e-9
            {
                return Err(OmniscanError::Template(format!(
                    "Marker '{}' boundaries exceed normalized page range [0.0, 1.0]",
                    marker.id
                )));
            }
        }

        // 4. Candidate ID validation
        if let Some(cid) = &self.candidate_id {
            if cid.digit_columns != 6 {
                return Err(OmniscanError::Template(format!(
                    "Candidate ID digit_columns must be 6, found {}",
                    cid.digit_columns
                )));
            }
            if cid.digits_per_column != 10 {
                return Err(OmniscanError::Template(format!(
                    "Candidate ID digits_per_column must be 10 (digits 0-9), found {}",
                    cid.digits_per_column
                )));
            }
            if cid.bubble_radius <= 0.0 {
                return Err(OmniscanError::Template(
                    "Candidate ID bubble_radius must be strictly positive".to_string(),
                ));
            }
            if cid.column_spacing < 2.0 * cid.bubble_radius - 1e-7 {
                return Err(OmniscanError::Template(
                    "Candidate ID column_spacing causes overlapping bubbles".to_string(),
                ));
            }
            if cid.row_spacing < 2.0 * cid.bubble_radius - 1e-7 {
                return Err(OmniscanError::Template(
                    "Candidate ID row_spacing causes overlapping bubbles".to_string(),
                ));
            }
            // Check bounding box of candidate ID grid
            for col in 0..cid.digit_columns {
                for row in 0..cid.digits_per_column {
                    let cx = cid.origin.x + col as f64 * cid.column_spacing;
                    let cy = cid.origin.y + row as f64 * cid.row_spacing;
                    let circle = Circle::new(Point2D::new(cx, cy), cid.bubble_radius);
                    if !circle.is_within_bounds() {
                        return Err(OmniscanError::Template(format!(
                            "Candidate ID bubble at column {}, digit {} exceeds normalized boundaries",
                            col, row
                        )));
                    }
                }
            }
        }

        // 5. Section and question validation
        if self.sections.is_empty() {
            return Err(OmniscanError::Template(
                "Template must contain at least one section".to_string(),
            ));
        }

        let mut section_ids = HashSet::new();
        let mut question_ids = HashSet::new();

        for section in &self.sections {
            if section.section_id.trim().is_empty() {
                return Err(OmniscanError::Template(
                    "Section ID cannot be empty".to_string(),
                ));
            }
            if !section_ids.insert(&section.section_id) {
                return Err(OmniscanError::Template(format!(
                    "Duplicate section ID: '{}'",
                    section.section_id
                )));
            }

            for question in &section.questions {
                if question.question_id.trim().is_empty() {
                    return Err(OmniscanError::Template(
                        "Question ID cannot be empty".to_string(),
                    ));
                }
                if !question_ids.insert(&question.question_id) {
                    return Err(OmniscanError::Template(format!(
                        "Duplicate question ID: '{}'",
                        question.question_id
                    )));
                }

                if question.options.len() < 2 {
                    return Err(OmniscanError::Template(format!(
                        "Question '{}' has {} options; minimum required is 2",
                        question.question_id,
                        question.options.len()
                    )));
                }

                // Check option uniqueness and bounds
                let mut option_labels = HashSet::new();
                let mut circles: Vec<Circle> = Vec::with_capacity(question.options.len());

                for opt in &question.options {
                    if opt.label.trim().is_empty() {
                        return Err(OmniscanError::Template(format!(
                            "Question '{}' has an option with an empty label",
                            question.question_id
                        )));
                    }
                    if !option_labels.insert(&opt.label) {
                        return Err(OmniscanError::Template(format!(
                            "Question '{}' has duplicate option label '{}'",
                            question.question_id, opt.label
                        )));
                    }

                    if opt.radius <= 0.0 {
                        return Err(OmniscanError::Template(format!(
                            "Question '{}' option '{}' radius must be positive",
                            question.question_id, opt.label
                        )));
                    }

                    let circle = Circle::new(opt.center, opt.radius);
                    if !circle.is_within_bounds() {
                        return Err(OmniscanError::Template(format!(
                            "Question '{}' option '{}' bubble exceeds page bounds [0.0, 1.0]",
                            question.question_id, opt.label
                        )));
                    }

                    circles.push(circle);
                }

                // Check pairwise bubble overlap within the question
                for i in 0..circles.len() {
                    for j in (i + 1)..circles.len() {
                        if circles[i].overlaps_with(&circles[j]) {
                            return Err(OmniscanError::Template(format!(
                                "Question '{}' has overlapping options '{}' and '{}'",
                                question.question_id,
                                question.options[i].label,
                                question.options[j].label
                            )));
                        }
                    }
                }
            }
        }

        Ok(())
    }

    /// Computes summary statistics for inspection.
    pub fn summary(&self) -> TemplateSummary {
        let total_questions: usize = self.sections.iter().map(|s| s.questions.len()).sum();
        let single_choice_count: usize = self
            .sections
            .iter()
            .flat_map(|s| &s.questions)
            .filter(|q| q.question_type == QuestionType::SingleChoice)
            .count();
        let multiple_select_count = total_questions - single_choice_count;

        let min_options = self
            .sections
            .iter()
            .flat_map(|s| &s.questions)
            .map(|q| q.options.len())
            .min()
            .unwrap_or(0);
        let max_options = self
            .sections
            .iter()
            .flat_map(|s| &s.questions)
            .map(|q| q.options.len())
            .max()
            .unwrap_or(0);

        TemplateSummary {
            template_id: self.template_id.clone(),
            name: self.name.clone(),
            schema_version: self.schema_version,
            reference_width: self.page.reference_width,
            reference_height: self.page.reference_height,
            aspect_ratio: self.page.aspect_ratio,
            orientation: self.page.orientation,
            marker_count: self.registration.markers.len(),
            registration_strategy: self.registration.strategy,
            candidate_id_configured: self.candidate_id.is_some(),
            section_count: self.sections.len(),
            total_questions,
            single_choice_count,
            multiple_select_count,
            min_options,
            max_options,
        }
    }
}

/// Formatted summary metadata of a validated template.
#[derive(Debug, Clone)]
pub struct TemplateSummary {
    pub template_id: String,
    pub name: String,
    pub schema_version: u32,
    pub reference_width: u32,
    pub reference_height: u32,
    pub aspect_ratio: f64,
    pub orientation: Orientation,
    pub marker_count: usize,
    pub registration_strategy: RegistrationStrategy,
    pub candidate_id_configured: bool,
    pub section_count: usize,
    pub total_questions: usize,
    pub single_choice_count: usize,
    pub multiple_select_count: usize,
    pub min_options: usize,
    pub max_options: usize,
}

impl fmt::Display for TemplateSummary {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Template ID:       {}", self.template_id)?;
        writeln!(f, "Name:              {}", self.name)?;
        writeln!(f, "Schema Version:    {}", self.schema_version)?;
        writeln!(
            f,
            "Page Dimensions:   {} x {} (Aspect Ratio: {:.4}, {:?})",
            self.reference_width, self.reference_height, self.aspect_ratio, self.orientation
        )?;
        writeln!(
            f,
            "Registration:      {} markers (strategy: {:?})",
            self.marker_count, self.registration_strategy
        )?;
        writeln!(
            f,
            "Candidate ID:      {}",
            if self.candidate_id_configured {
                "Configured (6 columns x 10 rows, digits 0-9)"
            } else {
                "None"
            }
        )?;
        writeln!(f, "Sections:          {} section(s)", self.section_count)?;
        writeln!(
            f,
            "Questions Total:   {} questions (single_choice: {}, multiple_select: {})",
            self.total_questions, self.single_choice_count, self.multiple_select_count
        )?;
        writeln!(
            f,
            "Options Range:     {} to {} options per question",
            self.min_options, self.max_options
        )?;
        write!(
            f,
            "Status:            VALID (all constraints and bounds verified)"
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_valid_template() -> Template {
        Template {
            schema_version: 1,
            template_id: "test-template".to_string(),
            name: "Test Template".to_string(),
            description: "Test description".to_string(),
            page: PageLayout {
                reference_width: 1000,
                reference_height: 1414,
                aspect_ratio: 0.7072,
                orientation: Orientation::Portrait,
            },
            registration: RegistrationLayout {
                strategy: RegistrationStrategy::FiducialMarkers,
                markers: vec![MarkerLayout {
                    id: "top_left".to_string(),
                    center: Point2D::new(0.05, 0.05),
                    size: Size2D::new(0.02, 0.02),
                    shape: MarkerShape::Square,
                }],
            },
            candidate_id: Some(CandidateIdLayout {
                id: "roll".to_string(),
                digit_columns: 6,
                digits_per_column: 10,
                origin: Point2D::new(0.60, 0.10),
                column_spacing: 0.04,
                row_spacing: 0.02,
                bubble_radius: 0.008,
            }),
            sections: vec![SectionLayout {
                section_id: "sec1".to_string(),
                name: "Section 1".to_string(),
                columns: 1,
                questions: vec![QuestionLayout {
                    question_id: "Q1".to_string(),
                    question_number: 1,
                    question_type: QuestionType::SingleChoice,
                    options: vec![
                        OptionLayout {
                            label: "A".to_string(),
                            center: Point2D::new(0.10, 0.30),
                            radius: 0.01,
                            shape: OptionShape::Circle,
                        },
                        OptionLayout {
                            label: "B".to_string(),
                            center: Point2D::new(0.15, 0.30),
                            radius: 0.01,
                            shape: OptionShape::Circle,
                        },
                    ],
                }],
            }],
        }
    }

    #[test]
    fn test_valid_template_validation() {
        let t = sample_valid_template();
        assert!(t.validate().is_ok());
    }

    #[test]
    fn test_unsupported_schema_version() {
        let mut t = sample_valid_template();
        t.schema_version = 2;
        let err = t.validate().unwrap_err();
        assert!(err.to_string().contains("Unsupported schema_version 2"));
    }

    #[test]
    fn test_duplicate_question_id() {
        let mut t = sample_valid_template();
        let duplicate_q = t.sections[0].questions[0].clone();
        t.sections[0].questions.push(duplicate_q);
        let err = t.validate().unwrap_err();
        assert!(err.to_string().contains("Duplicate question ID: 'Q1'"));
    }

    #[test]
    fn test_overlapping_options() {
        let mut t = sample_valid_template();
        // Place option B overlapping with option A (centers separated by 0.01, radii sum to 0.02)
        t.sections[0].questions[0].options[1].center = Point2D::new(0.11, 0.30);
        let err = t.validate().unwrap_err();
        assert!(err.to_string().contains("overlapping options 'A' and 'B'"));
    }
}

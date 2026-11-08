//! Ground-truth domain model and text label parser.

use crate::error::{OmniscanError, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::path::Path;

/// Ground-truth response mark state observed by a human annotator.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ResponseMark {
    /// Single marked option, e.g. 'A'
    Single(char),
    /// Multiple options marked on the sheet, sorted uppercase, e.g. ['A', 'C']
    Multiple(Vec<char>),
    /// Explicitly blank response (no mark made by candidate)
    Blank,
    /// Ambiguous response flagged by human annotator (smudge, faint erase, unclear)
    Ambiguous(String),
}

/// A parsed ground-truth question record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct QuestionLabel {
    pub question_id: String,
    pub mark: ResponseMark,
    pub line_number: usize,
}

/// Complete ground-truth sheet record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GroundTruthSheet {
    pub candidate_id: Option<String>,
    pub questions: Vec<QuestionLabel>,
}

impl GroundTruthSheet {
    /// Parses ground-truth text annotations from a string.
    pub fn parse(content: &str) -> Result<Self> {
        let mut candidate_id = None;
        let mut questions = Vec::new();
        let mut seen_question_ids = HashSet::new();

        for (idx, line) in content.lines().enumerate() {
            let line_number = idx + 1;
            let trimmed = line.trim();

            if trimmed.is_empty() {
                continue;
            }

            // Check for candidate header directive in comment (e.g. # candidate_id: CAND123)
            if let Some(comment_content) = trimmed.strip_prefix('#') {
                let comment_trimmed = comment_content.trim();
                if let Some(rest) = comment_trimmed
                    .strip_prefix("candidate_id:")
                    .or_else(|| comment_trimmed.strip_prefix("candidate:"))
                {
                    let cid = rest.trim();
                    if !cid.is_empty() {
                        candidate_id = Some(cid.to_string());
                    }
                }
                // Comment lines are otherwise ignored
                continue;
            }

            // Check for candidate header directive without '#' (e.g. CANDIDATE: CAND123 or ID,CAND123)
            if let Some(rest) = trimmed
                .strip_prefix("CANDIDATE:")
                .or_else(|| trimmed.strip_prefix("candidate:"))
                .or_else(|| trimmed.strip_prefix("CANDIDATE,"))
                .or_else(|| trimmed.strip_prefix("ID,"))
            {
                let cid = rest.trim();
                if !cid.is_empty() {
                    candidate_id = Some(cid.to_string());
                    continue;
                }
            }

            // Parse question record
            let (q_id_raw, mark_raw) = Self::split_line(trimmed, line_number)?;
            let q_id = q_id_raw.trim().to_string();

            if q_id.is_empty() {
                return Err(OmniscanError::GroundTruth(format!(
                    "Line {}: Question identifier cannot be empty",
                    line_number
                )));
            }

            if !seen_question_ids.insert(q_id.clone()) {
                return Err(OmniscanError::GroundTruth(format!(
                    "Line {}: Duplicate question identifier '{}'",
                    line_number, q_id
                )));
            }

            let mark = Self::parse_mark(mark_raw.trim(), &q_id, line_number)?;
            questions.push(QuestionLabel {
                question_id: q_id,
                mark,
                line_number,
            });
        }

        Ok(GroundTruthSheet {
            candidate_id,
            questions,
        })
    }

    /// Loads and parses ground-truth text annotations from a file.
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self> {
        let content = std::fs::read_to_string(path)?;
        Self::parse(&content)
    }

    /// Splits a record line by comma, tab, or whitespace delimiter.
    fn split_line(line: &str, line_number: usize) -> Result<(&str, &str)> {
        // Tab delimited
        if let Some((first, rest)) = line.split_once('\t') {
            return Ok((first.trim(), rest.trim()));
        }

        // Comma delimited (if the token before comma does not contain spaces)
        if let Some((first, rest)) = line.split_once(',') {
            if !first.trim().contains(char::is_whitespace) {
                return Ok((first.trim(), rest.trim()));
            }
        }

        // Whitespace delimited
        let mut parts = line.split_whitespace();
        let first = parts.next().ok_or_else(|| {
            OmniscanError::GroundTruth(format!("Line {}: Empty record", line_number))
        })?;
        let remainder_start = line
            .find(first)
            .map(|idx| idx + first.len())
            .unwrap_or(first.len());
        let rest = line[remainder_start..].trim();
        if rest.is_empty() {
            return Err(OmniscanError::GroundTruth(format!(
                "Line {}: Missing response mark for question '{}'",
                line_number, first
            )));
        }
        Ok((first, rest))
    }

    /// Parses the mark string into a typed ResponseMark variant.
    fn parse_mark(raw: &str, q_id: &str, line_number: usize) -> Result<ResponseMark> {
        if raw.is_empty() {
            return Err(OmniscanError::GroundTruth(format!(
                "Line {}: Missing mark for question '{}'",
                line_number, q_id
            )));
        }

        // Blank indicators
        if raw == "-"
            || raw == "_"
            || raw.eq_ignore_ascii_case("BLANK")
            || raw.eq_ignore_ascii_case("NONE")
        {
            return Ok(ResponseMark::Blank);
        }

        // Ambiguous indicators
        if raw == "?" || raw == "*" || raw.eq_ignore_ascii_case("AMBIGUOUS") {
            return Ok(ResponseMark::Ambiguous(
                "Mark flagged as ambiguous".to_string(),
            ));
        }
        if let Some(reason) = raw
            .strip_prefix("AMBIGUOUS:")
            .or_else(|| raw.strip_prefix("ambiguous:"))
        {
            return Ok(ResponseMark::Ambiguous(reason.trim().to_string()));
        }

        // Multiple or single choices (e.g. 'A', 'AB', 'A,B', 'A+B', 'A B')
        let mut chars = Vec::new();
        let mut seen = HashSet::new();

        for ch in raw.chars() {
            if ch.is_whitespace() || ch == ',' || ch == '+' || ch == ';' || ch == '/' {
                continue;
            }
            if ch.is_ascii_alphabetic() {
                let upper = ch.to_ascii_uppercase();
                if !seen.insert(upper) {
                    return Err(OmniscanError::GroundTruth(format!(
                        "Line {}: Duplicate option '{}' in multiple-mark response for question '{}'",
                        line_number, upper, q_id
                    )));
                }
                chars.push(upper);
            } else {
                return Err(OmniscanError::GroundTruth(format!(
                    "Line {}: Unknown or invalid mark token '{}' for question '{}'",
                    line_number, raw, q_id
                )));
            }
        }

        chars.sort_unstable();

        match chars.len() {
            0 => Err(OmniscanError::GroundTruth(format!(
                "Line {}: Malformed mark token '{}' for question '{}'",
                line_number, raw, q_id
            ))),
            1 => Ok(ResponseMark::Single(chars[0])),
            _ => Ok(ResponseMark::Multiple(chars)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_comma_delimited_labels() {
        let content = "\
# Assessment ground truth
# candidate_id: CAND-9981
Q1,A
Q2,B
Q3,-
Q4,AB
Q5,?
";
        let sheet = GroundTruthSheet::parse(content).unwrap();
        assert_eq!(sheet.candidate_id, Some("CAND-9981".to_string()));
        assert_eq!(sheet.questions.len(), 5);
        assert_eq!(sheet.questions[0].mark, ResponseMark::Single('A'));
        assert_eq!(sheet.questions[1].mark, ResponseMark::Single('B'));
        assert_eq!(sheet.questions[2].mark, ResponseMark::Blank);
        assert_eq!(
            sheet.questions[3].mark,
            ResponseMark::Multiple(vec!['A', 'B'])
        );
        assert!(matches!(
            sheet.questions[4].mark,
            ResponseMark::Ambiguous(_)
        ));
    }

    #[test]
    fn test_parse_whitespace_delimited_labels() {
        let content = "\
1 A
2 C
3 BLANK
4 A,D
5 AMBIGUOUS: smudge
";
        let sheet = GroundTruthSheet::parse(content).unwrap();
        assert_eq!(sheet.questions.len(), 5);
        assert_eq!(sheet.questions[0].mark, ResponseMark::Single('A'));
        assert_eq!(sheet.questions[1].mark, ResponseMark::Single('C'));
        assert_eq!(sheet.questions[2].mark, ResponseMark::Blank);
        assert_eq!(
            sheet.questions[3].mark,
            ResponseMark::Multiple(vec!['A', 'D'])
        );
        assert_eq!(
            sheet.questions[4].mark,
            ResponseMark::Ambiguous("smudge".to_string())
        );
    }

    #[test]
    fn test_reject_duplicate_question_id() {
        let content = "\
Q1 A
Q2 B
Q1 C
";
        let err = GroundTruthSheet::parse(content).unwrap_err();
        assert!(err
            .to_string()
            .contains("Line 3: Duplicate question identifier 'Q1'"));
    }

    #[test]
    fn test_reject_duplicate_options_in_multi() {
        let content = "Q1 AA";
        let err = GroundTruthSheet::parse(content).unwrap_err();
        assert!(err
            .to_string()
            .contains("Line 1: Duplicate option 'A' in multiple-mark response"));
    }

    #[test]
    fn test_reject_invalid_token() {
        let content = "Q1 123";
        let err = GroundTruthSheet::parse(content).unwrap_err();
        assert!(err
            .to_string()
            .contains("Line 1: Unknown or invalid mark token"));
    }
}

//! Exact reverse-complement repeat helpers for `palindrome` and `einverted`.

use emboss_core::{MoleculeKind, reverse_complement_residues};
use emboss_diagnostics::{ErrorCategory, PlatformError};

use crate::sequence_stream::ToolExecutionError;

/// One exact palindromic window.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PalindromeRegion {
    /// Zero-based inclusive start.
    pub start: usize,
    /// Zero-based half-open end.
    pub end: usize,
    /// Matched palindromic sequence.
    pub matched: String,
}

/// One exact inverted repeat with a spacer.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InvertedRepeatRegion {
    /// Zero-based inclusive left-arm start.
    pub left_start: usize,
    /// Zero-based half-open left-arm end.
    pub left_end: usize,
    /// Zero-based inclusive right-arm start.
    pub right_start: usize,
    /// Zero-based half-open right-arm end.
    pub right_end: usize,
    /// Spacer length between the arms.
    pub gap_length: usize,
    /// Left-arm sequence.
    pub left_arm: String,
    /// Right-arm sequence.
    pub right_arm: String,
}

/// Validates reverse-complement-compatible molecule input.
pub fn validate_reverse_complement_molecule(
    tool: &str,
    molecule: MoleculeKind,
) -> Result<(), ToolExecutionError> {
    if matches!(molecule, MoleculeKind::Protein | MoleculeKind::Unknown) {
        return Err(PlatformError::new(
            ErrorCategory::Validation,
            format!("{tool} expects nucleotide input but the record was classified as {molecule}"),
        )
        .with_code(format!("tools.{tool}.input.not_nucleotide")));
    }

    Ok(())
}

/// Finds all exact reverse-complement palindromes in the requested length range.
pub fn exact_palindromes(
    tool: &str,
    molecule: MoleculeKind,
    residues: &str,
    min_length: usize,
    max_length: usize,
) -> Result<Vec<PalindromeRegion>, ToolExecutionError> {
    validate_reverse_complement_molecule(tool, molecule)?;
    let normalized = residues.to_ascii_uppercase();
    let mut hits = Vec::new();

    if min_length == 0 || max_length < min_length {
        return Ok(hits);
    }

    for start in 0..normalized.len() {
        for length in min_length..=max_length {
            if start + length > normalized.len() {
                break;
            }
            let window = &normalized[start..start + length];
            let reverse = reverse_complement_residues(molecule, window).map_err(map_revseq_error)?;
            if reverse == window {
                hits.push(PalindromeRegion {
                    start,
                    end: start + length,
                    matched: window.to_owned(),
                });
            }
        }
    }

    Ok(hits)
}

/// Finds all exact inverted repeats with bounded spacer length.
pub fn exact_inverted_repeats(
    tool: &str,
    molecule: MoleculeKind,
    residues: &str,
    min_arm_length: usize,
    max_gap_length: usize,
) -> Result<Vec<InvertedRepeatRegion>, ToolExecutionError> {
    validate_reverse_complement_molecule(tool, molecule)?;
    let normalized = residues.to_ascii_uppercase();
    let mut hits = Vec::new();

    if min_arm_length == 0 {
        return Ok(hits);
    }

    for left_start in 0..normalized.len() {
        for arm_length in min_arm_length..=normalized.len() {
            let left_end = left_start + arm_length;
            if left_end > normalized.len() {
                break;
            }

            let left_arm = &normalized[left_start..left_end];
            let reverse = reverse_complement_residues(molecule, left_arm).map_err(map_revseq_error)?;

            for gap_length in 0..=max_gap_length {
                let right_start = left_end + gap_length;
                let right_end = right_start + arm_length;
                if right_end > normalized.len() {
                    break;
                }
                let right_arm = &normalized[right_start..right_end];
                if reverse == right_arm {
                    hits.push(InvertedRepeatRegion {
                        left_start,
                        left_end,
                        right_start,
                        right_end,
                        gap_length,
                        left_arm: left_arm.to_owned(),
                        right_arm: right_arm.to_owned(),
                    });
                }
            }
        }
    }

    Ok(hits)
}

fn map_revseq_error(error: emboss_core::RevseqError) -> ToolExecutionError {
    PlatformError::new(ErrorCategory::Validation, error.to_string())
        .with_code("tools.pattern_tools.reverse_complement_invalid")
}

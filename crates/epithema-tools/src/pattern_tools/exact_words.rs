//! Exact shared-word helpers for `wordmatch` and `wordfinder`.

use epithema_core::{SequenceRecord, infer_alignment_mode};
use epithema_diagnostics::{ErrorCategory, PlatformError};

use crate::sequence_stream::ToolExecutionError;

/// One maximal exact shared region between two records.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExactWordRegion {
    /// Zero-based inclusive start in the left/query sequence.
    pub left_start: usize,
    /// Zero-based half-open end in the left/query sequence.
    pub left_end: usize,
    /// Zero-based inclusive start in the right/target sequence.
    pub right_start: usize,
    /// Zero-based half-open end in the right/target sequence.
    pub right_end: usize,
    /// Shared exact sequence.
    pub matched: String,
}

/// Validates compatibility and derives maximal exact shared regions.
pub fn maximal_exact_regions(
    tool: &str,
    left: &SequenceRecord,
    right: &SequenceRecord,
    min_length: usize,
) -> Result<Vec<ExactWordRegion>, ToolExecutionError> {
    infer_alignment_mode(left, right).map_err(|error| {
        PlatformError::new(ErrorCategory::Validation, error.to_string())
            .with_code(format!("tools.{tool}.input.incompatible_molecules"))
    })?;

    let left_bytes = left.residues().as_bytes();
    let right_bytes = right.residues().as_bytes();
    let mut regions = Vec::new();

    for left_start in 0..left_bytes.len() {
        for right_start in 0..right_bytes.len() {
            if left_bytes[left_start] != right_bytes[right_start] {
                continue;
            }

            if left_start > 0
                && right_start > 0
                && left_bytes[left_start - 1] == right_bytes[right_start - 1]
            {
                continue;
            }

            let mut length = 0usize;
            while left_start + length < left_bytes.len()
                && right_start + length < right_bytes.len()
                && left_bytes[left_start + length] == right_bytes[right_start + length]
            {
                length += 1;
            }

            if length < min_length {
                continue;
            }

            let matched = std::str::from_utf8(&left_bytes[left_start..left_start + length])
                .expect("sequence residues are normalized ASCII")
                .to_owned();

            regions.push(ExactWordRegion {
                left_start,
                left_end: left_start + length,
                right_start,
                right_end: right_start + length,
                matched,
            });
        }
    }

    regions.sort_by(|left, right| {
        left.left_start
            .cmp(&right.left_start)
            .then(left.right_start.cmp(&right.right_start))
            .then(left.matched.cmp(&right.matched))
    });

    Ok(regions)
}

//! `biosed` implementation.

use emboss_core::SequenceRecord;
use emboss_diagnostics::{ErrorCategory, PlatformError};

use crate::sequence_stream::{SequenceInput, ToolExecutionError, load_sequence_records};

use super::shared::rebuild_unannotated_record;

/// Typed parameters for `biosed`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BiosedParams {
    /// Input sequence stream.
    pub input: SequenceInput,
    /// 1-based inclusive start coordinate.
    pub start: usize,
    /// 1-based inclusive end coordinate.
    pub end: usize,
    /// Optional replacement residues. `None` means delete.
    pub replacement: Option<String>,
}
/// Structured `biosed` outcome.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BiosedOutcome {
    /// Source input.
    pub input: SequenceInput,
    /// 1-based inclusive start coordinate.
    pub start: usize,
    /// 1-based inclusive end coordinate.
    pub end: usize,
    /// Optional replacement residues.
    pub replacement: Option<String>,
    /// Edited records in input order.
    pub records: Vec<SequenceRecord>,
}

/// Returns `biosed` help text.
#[must_use]
pub fn biosed_help() -> &'static str {
    "Usage: emboss-rs biosed <input> <start> <end> [--replace <residues>]\n\nReplace or delete a 1-based inclusive section in every input record. Omitting --replace deletes the selected interval. v1 drops feature annotations after editing."
}

/// Executes `biosed`.
pub fn run_biosed(params: BiosedParams) -> Result<BiosedOutcome, ToolExecutionError> {
    let records = load_sequence_records(&params.input)?
        .into_iter()
        .map(|record| {
            edit_record(
                &record,
                params.start,
                params.end,
                params.replacement.as_deref(),
            )
        })
        .collect::<Result<Vec<_>, _>>()?;

    Ok(BiosedOutcome {
        input: params.input,
        start: params.start,
        end: params.end,
        replacement: params.replacement,
        records,
    })
}

fn edit_record(
    record: &SequenceRecord,
    start: usize,
    end: usize,
    replacement: Option<&str>,
) -> Result<SequenceRecord, ToolExecutionError> {
    if start == 0 || start > end {
        return Err(PlatformError::new(
            ErrorCategory::Validation,
            "biosed requires 1-based inclusive coordinates with start <= end",
        )
        .with_code("tools.biosed.coordinates.invalid"));
    }
    if end > record.len() {
        return Err(PlatformError::new(
            ErrorCategory::Validation,
            format!(
                "biosed interval {start}..{end} is out of range for sequence '{}' of length {}",
                record.identifier().accession(),
                record.len()
            ),
        )
        .with_code("tools.biosed.coordinates.out_of_range"));
    }

    let left = &record.residues()[..start - 1];
    let right = &record.residues()[end..];
    let replacement = replacement.unwrap_or_default();
    let residues = format!("{left}{replacement}{right}");
    if residues.is_empty() {
        return Err(PlatformError::new(
            ErrorCategory::Validation,
            format!(
                "biosed would remove all residues from sequence '{}'",
                record.identifier().accession()
            ),
        )
        .with_code("tools.biosed.sequence.empty"));
    }

    rebuild_unannotated_record("biosed", record, residues)
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::sequence_stream::SequenceInput;

    use super::{BiosedParams, run_biosed};

    fn fixture(name: &str) -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join(format!("../../crates/emboss-tools/tests/fixtures/{name}"))
    }

    #[test]
    fn replaces_interval_in_every_record() {
        let outcome = run_biosed(BiosedParams {
            input: SequenceInput::new(fixture("biosed_records.fasta")),
            start: 2,
            end: 3,
            replacement: Some("NN".to_owned()),
        })
        .expect("biosed should succeed");

        assert_eq!(outcome.records[0].residues(), "ANNG");
        assert_eq!(outcome.records[1].residues(), "TNNN");
    }

    #[test]
    fn deletes_interval_when_replacement_is_omitted() {
        let outcome = run_biosed(BiosedParams {
            input: SequenceInput::new(fixture("biosed_records.fasta")),
            start: 2,
            end: 3,
            replacement: None,
        })
        .expect("biosed should succeed");

        assert_eq!(outcome.records[0].residues(), "AG");
    }
}

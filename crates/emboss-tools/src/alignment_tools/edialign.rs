//! `edialign` implementation.

use emboss_core::{Alignment, AlignmentRow, SequenceRecord};
use emboss_diagnostics::{ErrorCategory, PlatformError};

use crate::sequence_stream::{SequenceInput, ToolExecutionError, load_sequence_records};

/// Typed parameters for `edialign`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EdialignParams {
    /// Sequence input containing two or more records.
    pub input: SequenceInput,
    /// Minimum exact shared block length.
    pub min_length: usize,
}

/// One aligned shared-block row summary.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EdialignRow {
    /// Row identifier.
    pub identifier: String,
    /// 1-based inclusive start coordinate in the source sequence.
    pub start: usize,
    /// 1-based inclusive end coordinate in the source sequence.
    pub end: usize,
}

/// Structured `edialign` outcome.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EdialignOutcome {
    /// Source input.
    pub input: SequenceInput,
    /// Minimum exact shared block length.
    pub min_length: usize,
    /// Shared exact block.
    pub block: String,
    /// Source-row span summaries.
    pub rows: Vec<EdialignRow>,
    /// Alignment payload.
    pub alignment: Alignment,
}

/// Returns `edialign` help text.
#[must_use]
pub fn edialign_help() -> &'static str {
    "Usage: emboss-rs edialign <input> [--min-length <length>]\n\nDerive a bounded local multiple alignment by finding the longest exact shared block across two or more sequence records in one input file. EMBOSS-RS v1 reports only the shared exact block and does not implement the broader historical dynamic-programming surface."
}

/// Executes `edialign`.
pub fn run_edialign(params: EdialignParams) -> Result<EdialignOutcome, ToolExecutionError> {
    if params.min_length == 0 {
        return Err(PlatformError::new(
            ErrorCategory::Validation,
            "edialign --min-length must be a positive integer",
        )
        .with_code("tools.edialign.min_length.invalid"));
    }

    let records = load_sequence_records(&params.input)?;
    if records.len() < 2 {
        return Err(PlatformError::new(
            ErrorCategory::Validation,
            "edialign requires at least two sequence records in one input file",
        )
        .with_code("tools.edialign.input.too_few_records"));
    }

    validate_molecule_compatibility(&records)?;
    let (block, starts) = longest_shared_block(&records, params.min_length).ok_or_else(|| {
        PlatformError::new(
            ErrorCategory::Validation,
            format!(
                "no exact shared block of length {} or greater was found",
                params.min_length
            ),
        )
        .with_code("tools.edialign.block.not_found")
    })?;

    let rows = records
        .iter()
        .zip(starts.iter())
        .map(|(record, start)| EdialignRow {
            identifier: record.identifier().accession().to_owned(),
            start: start + 1,
            end: start + block.len(),
        })
        .collect::<Vec<_>>();

    let alignment = Alignment::with_identifier(
        Some("edialign-shared-block"),
        records
            .iter()
            .map(|record| {
                AlignmentRow::new(
                    record.identifier().clone(),
                    record.molecule(),
                    block.clone(),
                )
                .map(|row| row.with_metadata(record.metadata().clone()))
            })
            .collect::<Result<Vec<_>, _>>()
            .map_err(|error| {
                PlatformError::new(ErrorCategory::Validation, error.to_string())
                    .with_code("tools.edialign.alignment.invalid")
            })?,
    )
    .map_err(|error| {
        PlatformError::new(ErrorCategory::Validation, error.to_string())
            .with_code("tools.edialign.alignment.invalid")
    })?;

    Ok(EdialignOutcome {
        input: params.input,
        min_length: params.min_length,
        block,
        rows,
        alignment,
    })
}

fn validate_molecule_compatibility(records: &[SequenceRecord]) -> Result<(), ToolExecutionError> {
    let first = records[0].molecule();
    let all_nucleotide = records
        .iter()
        .all(|record| record.molecule().is_nucleotide());
    let all_protein = records.iter().all(|record| record.molecule().is_protein());
    let all_unknown = records.iter().all(|record| record.molecule() == first);
    if all_nucleotide || all_protein || all_unknown {
        return Ok(());
    }

    Err(PlatformError::new(
        ErrorCategory::Validation,
        "edialign requires records from one compatible molecule class",
    )
    .with_code("tools.edialign.molecule_mismatch"))
}

fn longest_shared_block(
    records: &[SequenceRecord],
    min_length: usize,
) -> Option<(String, Vec<usize>)> {
    let seed = records.first()?.residues();
    for length in (min_length..=seed.len()).rev() {
        for start in 0..=seed.len() - length {
            let candidate = &seed[start..start + length];
            let mut starts = vec![start];
            let mut found = true;
            for record in records.iter().skip(1) {
                if let Some(position) = record.residues().find(candidate) {
                    starts.push(position);
                } else {
                    found = false;
                    break;
                }
            }
            if found {
                return Some((candidate.to_owned(), starts));
            }
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::sequence_stream::SequenceInput;

    use super::{EdialignParams, run_edialign};

    fn fixture(name: &str) -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join(format!("../emboss-tools/tests/fixtures/{name}"))
    }

    #[test]
    fn derives_longest_exact_shared_block_alignment() {
        let outcome = run_edialign(EdialignParams {
            input: SequenceInput::new(fixture("edialign_records.fasta")),
            min_length: 3,
        })
        .expect("edialign should execute");

        assert_eq!(outcome.block, "TACG");
        assert_eq!(outcome.rows.len(), 3);
        assert_eq!(outcome.rows[0].start, 4);
        assert_eq!(outcome.alignment.row_count(), 3);
        assert_eq!(outcome.alignment.column_count(), 4);
    }
}

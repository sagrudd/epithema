//! `maskambignuc` implementation.

use emboss_core::{SequenceRecord, mask_intervals};
use emboss_diagnostics::{ErrorCategory, PlatformError};

use crate::feature_tools::shared::contiguous_matching_intervals;
use crate::sequence_stream::{SequenceInput, ToolExecutionError, load_sequence_records};

/// Typed parameters for `maskambignuc`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MaskambignucParams {
    /// Local sequence input path.
    pub input: SequenceInput,
}

/// Structured `maskambignuc` outcome.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MaskambignucOutcome {
    /// Source input.
    pub input: SequenceInput,
    /// Total ambiguity residues masked across all records.
    pub masked_residue_count: usize,
    /// Masked records in input order.
    pub records: Vec<SequenceRecord>,
}

/// Returns `maskambignuc` help text.
#[must_use]
pub fn maskambignuc_help() -> &'static str {
    "Usage: emboss-rs maskambignuc <input>\n\nMask conservative nucleotide ambiguity residues with N in each input record. The EMBOSS-RS v1 implementation supports DNA and RNA inputs, preserves record order and annotations, and leaves canonical A/C/G/T/U residues unchanged."
}

/// Executes `maskambignuc`.
pub fn run_maskambignuc(
    params: MaskambignucParams,
) -> Result<MaskambignucOutcome, ToolExecutionError> {
    let mut masked_residue_count = 0usize;
    let mut records = Vec::new();

    for record in load_sequence_records(&params.input)? {
        if !record.molecule().is_nucleotide() {
            return Err(PlatformError::new(
                ErrorCategory::Validation,
                format!(
                    "maskambignuc requires nucleotide records; observed {} for '{}'",
                    record.molecule(),
                    record.identifier().accession()
                ),
            )
            .with_code("tools.maskambignuc.molecule.invalid"));
        }

        let intervals = contiguous_matching_intervals(&record, is_nucleotide_ambiguity);
        masked_residue_count += intervals
            .iter()
            .map(|interval| interval.len())
            .sum::<usize>();
        if intervals.is_empty() {
            records.push(record);
            continue;
        }

        records.push(mask_intervals(&record, &intervals, 'N').map_err(|error| {
            PlatformError::new(ErrorCategory::Validation, error.to_string())
                .with_code("tools.maskambignuc.mask.invalid")
        })?);
    }

    Ok(MaskambignucOutcome {
        input: params.input,
        masked_residue_count,
        records,
    })
}

fn is_nucleotide_ambiguity(symbol: char) -> bool {
    matches!(
        symbol.to_ascii_uppercase(),
        'N' | 'R' | 'Y' | 'S' | 'W' | 'K' | 'M' | 'B' | 'D' | 'H' | 'V'
    )
}

#[cfg(test)]
mod tests {
    use super::{MaskambignucParams, run_maskambignuc};
    use crate::sequence_stream::SequenceInput;

    #[test]
    fn masks_nucleotide_ambiguities_with_n() {
        let fixture = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests/fixtures/ambiguous_nucleotide_records.fasta");
        let outcome = run_maskambignuc(MaskambignucParams {
            input: SequenceInput::new(&fixture),
        })
        .expect("maskambignuc should execute");

        assert_eq!(outcome.masked_residue_count, 3);
        assert_eq!(outcome.records[0].residues(), "ACGTNNN");
    }

    #[test]
    fn rejects_non_nucleotide_input() {
        let fixture = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests/fixtures/protein_records.fasta");
        let error = run_maskambignuc(MaskambignucParams {
            input: SequenceInput::new(&fixture),
        })
        .expect_err("protein input should fail");

        assert_eq!(error.code(), Some("tools.maskambignuc.molecule.invalid"));
    }
}

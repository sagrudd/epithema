//! `maskambigprot` implementation.

use emboss_core::{SequenceRecord, mask_intervals};
use emboss_diagnostics::{ErrorCategory, PlatformError};

use crate::feature_tools::shared::contiguous_matching_intervals;
use crate::sequence_stream::{SequenceInput, ToolExecutionError, load_sequence_records};

/// Typed parameters for `maskambigprot`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MaskambigprotParams {
    /// Local sequence input path.
    pub input: SequenceInput,
}

/// Structured `maskambigprot` outcome.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MaskambigprotOutcome {
    /// Source input.
    pub input: SequenceInput,
    /// Total ambiguity residues masked across all records.
    pub masked_residue_count: usize,
    /// Masked records in input order.
    pub records: Vec<SequenceRecord>,
}

/// Returns `maskambigprot` help text.
#[must_use]
pub fn maskambigprot_help() -> &'static str {
    "Usage: emboss-rs maskambigprot <input>\n\nMask conservative protein ambiguity residues with X in each input record. The EMBOSS-RS v1 implementation treats B, J, X, and Z as ambiguity symbols, preserves record order and annotations, and leaves canonical amino-acid symbols unchanged."
}

/// Executes `maskambigprot`.
pub fn run_maskambigprot(
    params: MaskambigprotParams,
) -> Result<MaskambigprotOutcome, ToolExecutionError> {
    let mut masked_residue_count = 0usize;
    let mut records = Vec::new();

    for record in load_sequence_records(&params.input)? {
        if !record.molecule().is_protein() {
            return Err(PlatformError::new(
                ErrorCategory::Validation,
                format!(
                    "maskambigprot requires protein records; observed {} for '{}'",
                    record.molecule(),
                    record.identifier().accession()
                ),
            )
            .with_code("tools.maskambigprot.molecule.invalid"));
        }

        let intervals = contiguous_matching_intervals(&record, is_protein_ambiguity);
        masked_residue_count += intervals
            .iter()
            .map(|interval| interval.len())
            .sum::<usize>();
        if intervals.is_empty() {
            records.push(record);
            continue;
        }

        records.push(mask_intervals(&record, &intervals, 'X').map_err(|error| {
            PlatformError::new(ErrorCategory::Validation, error.to_string())
                .with_code("tools.maskambigprot.mask.invalid")
        })?);
    }

    Ok(MaskambigprotOutcome {
        input: params.input,
        masked_residue_count,
        records,
    })
}

fn is_protein_ambiguity(symbol: char) -> bool {
    matches!(symbol.to_ascii_uppercase(), 'B' | 'J' | 'X' | 'Z')
}

#[cfg(test)]
mod tests {
    use super::{MaskambigprotParams, run_maskambigprot};
    use crate::sequence_stream::SequenceInput;

    #[test]
    fn masks_protein_ambiguities_with_x() {
        let fixture = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests/fixtures/ambiguous_protein_records.fasta");
        let outcome = run_maskambigprot(MaskambigprotParams {
            input: SequenceInput::new(&fixture),
        })
        .expect("maskambigprot should execute");

        assert_eq!(outcome.masked_residue_count, 4);
        assert_eq!(outcome.records[0].residues(), "MXXXXUO-*");
    }

    #[test]
    fn rejects_non_protein_input() {
        let fixture = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("tests/fixtures/three_records.fasta");
        let error = run_maskambigprot(MaskambigprotParams {
            input: SequenceInput::new(&fixture),
        })
        .expect_err("nucleotide input should fail");

        assert_eq!(error.code(), Some("tools.maskambigprot.molecule.invalid"));
    }
}

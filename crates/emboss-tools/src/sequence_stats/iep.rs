//! `iep` implementation.

use emboss_core::{TitratableResidueCounts, estimate_protein_isoelectric_point};
use emboss_diagnostics::{ErrorCategory, PlatformError};

use crate::sequence_stats::protein_support::validate_protein_record;
use crate::sequence_stream::{SequenceInput, ToolExecutionError, load_sequence_records};

/// Typed parameters for `iep`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct IepParams {
    /// Local protein input path.
    pub input: SequenceInput,
}

/// One per-record isoelectric-point estimate.
#[derive(Clone, Debug, PartialEq)]
pub struct IepRecord {
    /// Record identifier.
    pub record_id: String,
    /// Number of non-gap, non-stop residues contributing to the estimate.
    pub residue_length: usize,
    /// Estimated pI from the fixed v1 pKa model.
    pub estimated_pi: f64,
    /// Net charge at pH 7.0 from the same model.
    pub net_charge_at_ph7: f64,
    /// Titratable side-chain counts used by the estimate.
    pub titratable_counts: TitratableResidueCounts,
}

/// Structured `iep` outcome.
#[derive(Clone, Debug, PartialEq)]
pub struct IepOutcome {
    /// Source input.
    pub input: SequenceInput,
    /// Per-record estimates.
    pub records: Vec<IepRecord>,
}

/// Returns `iep` help text.
#[must_use]
pub fn iep_help() -> &'static str {
    "Usage: emboss-rs iep <protein-input>\n\nEstimate a deterministic protein isoelectric point for each record. The v1 implementation uses a fixed explicit pKa model for termini plus D/E/C/Y/H/K/R side chains, reports net charge at pH 7.0, ignores stop symbols, and rejects unsupported ambiguous residues."
}

/// Executes `iep`.
pub fn run_iep(params: IepParams) -> Result<IepOutcome, ToolExecutionError> {
    let records = load_sequence_records(&params.input)?
        .into_iter()
        .map(|record| -> Result<IepRecord, ToolExecutionError> {
            validate_protein_record("iep", &record)?;
            let estimate =
                estimate_protein_isoelectric_point(record.residues()).map_err(|error| {
                    PlatformError::new(ErrorCategory::Validation, error.to_string())
                        .with_code("tools.iep.residue.unsupported")
                })?;

            Ok(IepRecord {
                record_id: record.identifier().accession().to_owned(),
                residue_length: estimate.residue_length,
                estimated_pi: estimate.isoelectric_point,
                net_charge_at_ph7: estimate.net_charge_at_ph7,
                titratable_counts: estimate.titratable_counts,
            })
        })
        .collect::<Result<Vec<_>, _>>()?;

    Ok(IepOutcome {
        input: params.input,
        records,
    })
}

#[cfg(test)]
mod tests {
    use super::{IepParams, run_iep};
    use crate::sequence_stream::SequenceInput;

    fn fixture(name: &str) -> std::path::PathBuf {
        std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join(format!("../../crates/emboss-tools/tests/fixtures/{name}"))
    }

    #[test]
    fn reports_pi_for_basic_and_mixed_records() {
        let outcome = run_iep(IepParams {
            input: SequenceInput::new(fixture("iep_records.fasta")),
        })
        .expect("iep should succeed");

        assert_eq!(outcome.records.len(), 2);
        assert!(outcome.records[0].estimated_pi > 9.0);
        assert!(outcome.records[0].net_charge_at_ph7 > 0.0);
        assert_eq!(outcome.records[1].titratable_counts.aspartate, 1);
        assert_eq!(outcome.records[1].titratable_counts.lysine, 1);
    }

    #[test]
    fn rejects_nucleotide_input() {
        let error = run_iep(IepParams {
            input: SequenceInput::new(fixture("nucleotide_pattern_records.fasta")),
        })
        .expect_err("nucleotide input should fail");

        assert!(error.to_string().contains("expects protein input"));
    }
}

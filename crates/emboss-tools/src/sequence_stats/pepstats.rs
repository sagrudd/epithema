//! `pepstats` implementation.

use emboss_core::{ResidueComposition, protein_molecular_weight};
use emboss_diagnostics::{ErrorCategory, PlatformError};

use crate::sequence_stream::{SequenceInput, ToolExecutionError, load_sequence_records};

/// Typed parameters for `pepstats`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PepstatsParams {
    /// Local protein input path.
    pub input: SequenceInput,
}

/// One per-record protein statistics summary.
#[derive(Clone, Debug, PartialEq)]
pub struct PepstatsRecord {
    /// Record identifier.
    pub record_id: String,
    /// Raw sequence length.
    pub sequence_length: usize,
    /// Number of non-gap, non-stop residues contributing to mass.
    pub residue_length: usize,
    /// Number of stop symbols present.
    pub stop_count: usize,
    /// Deterministic composition summary excluding gaps.
    pub composition: ResidueComposition,
    /// Estimated molecular weight in Daltons.
    pub molecular_weight: f64,
}

/// Structured `pepstats` outcome.
#[derive(Clone, Debug, PartialEq)]
pub struct PepstatsOutcome {
    /// Source input.
    pub input: SequenceInput,
    /// Per-record protein statistics.
    pub records: Vec<PepstatsRecord>,
}

/// Returns `pepstats` help text.
#[must_use]
pub fn pepstats_help() -> &'static str {
    "Usage: emboss-rs pepstats <protein-input>\n\nReport deterministic protein summary statistics for each sequence record. The v1 implementation includes raw sequence length, residue length excluding stop symbols, amino-acid composition counts and frequencies, and an average-residue molecular-weight estimate with water added once per chain. Isoelectric-point estimation is deferred."
}

/// Executes `pepstats`.
pub fn run_pepstats(params: PepstatsParams) -> Result<PepstatsOutcome, ToolExecutionError> {
    let records = load_sequence_records(&params.input)?
        .into_iter()
        .map(|record| {
            if record.molecule().is_nucleotide() {
                return Err(PlatformError::new(
                    ErrorCategory::Validation,
                    format!(
                        "pepstats expects protein input but '{}' was classified as {}",
                        record.identifier().accession(),
                        record.molecule()
                    ),
                )
                .with_code("tools.pepstats.input.not_protein"));
            }

            let composition = ResidueComposition::from_sequence(record.residues());
            let stop_count = composition.count_for('*');
            let residue_length = composition.counted_symbols().saturating_sub(stop_count);
            let molecular_weight =
                protein_molecular_weight(record.residues()).map_err(|error| {
                    PlatformError::new(ErrorCategory::Validation, error.to_string())
                        .with_code("tools.pepstats.residue.unsupported")
                })?;

            Ok(PepstatsRecord {
                record_id: record.identifier().accession().to_owned(),
                sequence_length: record.len(),
                residue_length,
                stop_count,
                composition,
                molecular_weight,
            })
        })
        .collect::<Result<Vec<_>, _>>()?;

    Ok(PepstatsOutcome {
        input: params.input,
        records,
    })
}

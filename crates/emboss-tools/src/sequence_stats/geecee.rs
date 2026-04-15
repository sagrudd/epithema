//! `geecee` implementation.

use emboss_core::GcSummary;
use emboss_diagnostics::{ErrorCategory, PlatformError};

use crate::sequence_stream::{SequenceInput, ToolExecutionError, load_sequence_records};

/// Typed parameters for `geecee`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GeeceeParams {
    /// Local nucleotide input path.
    pub input: SequenceInput,
}

/// One per-record GC summary.
#[derive(Clone, Debug, PartialEq)]
pub struct GeeceeRecord {
    /// Record identifier.
    pub record_id: String,
    /// Raw sequence length.
    pub sequence_length: usize,
    /// GC summary over the record.
    pub gc: GcSummary,
}

/// Structured `geecee` outcome.
#[derive(Clone, Debug, PartialEq)]
pub struct GeeceeOutcome {
    /// Source input.
    pub input: SequenceInput,
    /// Per-record GC summaries.
    pub records: Vec<GeeceeRecord>,
    /// Aggregate GC summary across all records.
    pub aggregate: GcSummary,
}

/// Returns `geecee` help text.
#[must_use]
pub fn geecee_help() -> &'static str {
    "Usage: emboss-rs geecee <nucleotide-input>\n\nReport deterministic GC counts and GC percentages for nucleotide sequence records. The v1 implementation reports both per-record rows and an aggregate summary across all records. GC percentage is computed over canonical A/C/G/T/U symbols only; ambiguous symbols such as N are excluded from the denominator and reported separately."
}

/// Executes `geecee`.
pub fn run_geecee(params: GeeceeParams) -> Result<GeeceeOutcome, ToolExecutionError> {
    let mut aggregate = GcSummary::default();
    let records = load_sequence_records(&params.input)?
        .into_iter()
        .map(|record| {
            if record.molecule().is_protein() {
                return Err(PlatformError::new(
                    ErrorCategory::Validation,
                    format!(
                        "geecee expects nucleotide input but '{}' was classified as {}",
                        record.identifier().accession(),
                        record.molecule()
                    ),
                )
                .with_code("tools.geecee.input.not_nucleotide"));
            }

            let gc = GcSummary::from_sequence(record.residues());
            aggregate.merge(&gc);
            Ok(GeeceeRecord {
                record_id: record.identifier().accession().to_owned(),
                sequence_length: record.len(),
                gc,
            })
        })
        .collect::<Result<Vec<_>, _>>()?;

    Ok(GeeceeOutcome {
        input: params.input,
        records,
        aggregate,
    })
}

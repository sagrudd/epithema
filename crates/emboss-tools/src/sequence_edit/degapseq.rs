//! `degapseq` implementation.

use emboss_core::SequenceRecord;
use emboss_diagnostics::{ErrorCategory, PlatformError};

use crate::sequence_stream::{SequenceInput, ToolExecutionError, load_sequence_records};

/// Typed parameters for `degapseq`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DegapseqParams {
    /// Local sequence input path.
    pub input: SequenceInput,
}

/// Structured `degapseq` outcome.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DegapseqOutcome {
    /// Source input.
    pub input: SequenceInput,
    /// Degapped records in input order.
    pub records: Vec<SequenceRecord>,
}

/// Returns `degapseq` help text.
#[must_use]
pub fn degapseq_help() -> &'static str {
    "Usage: emboss-rs degapseq <input>\n\nRemove '-' and '.' gap characters from each input sequence record and emit FASTA."
}

/// Executes `degapseq`.
pub fn run_degapseq(params: DegapseqParams) -> Result<DegapseqOutcome, ToolExecutionError> {
    let records = load_sequence_records(&params.input)?
        .into_iter()
        .map(degap_record)
        .collect::<Result<Vec<_>, _>>()?;

    Ok(DegapseqOutcome {
        input: params.input,
        records,
    })
}

fn degap_record(record: SequenceRecord) -> Result<SequenceRecord, ToolExecutionError> {
    let cleaned: String = record
        .residues()
        .chars()
        .filter(|symbol| !matches!(symbol, '-' | '.'))
        .collect();

    if cleaned.is_empty() {
        return Err(PlatformError::new(
            ErrorCategory::Validation,
            format!(
                "degapping removed all residues from sequence '{}'",
                record.identifier().accession()
            ),
        )
        .with_code("tools.degapseq.sequence.empty"));
    }

    let mut updated = SequenceRecord::new(record.identifier().clone(), record.molecule(), cleaned)
        .map_err(|error| {
            PlatformError::new(ErrorCategory::Validation, error.to_string())
                .with_code("tools.degapseq.sequence.invalid")
        })?;
    updated = updated.with_metadata(record.metadata().clone());
    Ok(updated)
}

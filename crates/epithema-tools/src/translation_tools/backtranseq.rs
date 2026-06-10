//! `backtranseq` implementation.

use epithema_core::{MoleculeKind, SequenceMetadata, SequenceRecord, backtranslate_representative};
use epithema_diagnostics::{ErrorCategory, PlatformError};

use crate::sequence_stream::{SequenceInput, ToolExecutionError, load_sequence_records};

/// Typed parameters for `backtranseq`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BacktranseqParams {
    /// Local protein input path.
    pub input: SequenceInput,
}

/// Structured `backtranseq` outcome.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BacktranseqOutcome {
    /// Source input.
    pub input: SequenceInput,
    /// Back-translated nucleotide records.
    pub records: Vec<SequenceRecord>,
}

/// Returns `backtranseq` help text.
#[must_use]
pub fn backtranseq_help() -> &'static str {
    "Usage: epithema backtranseq <protein-input>\n\nBack-translate protein sequence records into deterministic representative DNA codons using the standard genetic code. Stop symbols '*' are rendered as TAA. Output is emitted as FASTA DNA sequences."
}

/// Executes `backtranseq`.
pub fn run_backtranseq(
    params: BacktranseqParams,
) -> Result<BacktranseqOutcome, ToolExecutionError> {
    let records = load_sequence_records(&params.input)?
        .into_iter()
        .map(backtranslate_record)
        .collect::<Result<Vec<_>, _>>()?;

    Ok(BacktranseqOutcome {
        input: params.input,
        records,
    })
}

fn backtranslate_record(record: SequenceRecord) -> Result<SequenceRecord, ToolExecutionError> {
    validate_protein_record("backtranseq", &record)?;
    let residues = backtranslate_representative(record.residues()).map_err(|error| {
        PlatformError::new(ErrorCategory::Validation, error.to_string())
            .with_code("tools.backtranseq.residue.invalid")
    })?;

    let metadata = derived_metadata(record.metadata(), "representative back-translated DNA");
    SequenceRecord::new(record.identifier().clone(), MoleculeKind::Dna, residues)
        .map(|record| record.with_metadata(metadata))
        .map_err(|error| {
            PlatformError::new(ErrorCategory::Validation, error.to_string())
                .with_code("tools.backtranseq.sequence.invalid")
        })
}

fn validate_protein_record(tool: &str, record: &SequenceRecord) -> Result<(), ToolExecutionError> {
    if record.molecule().is_nucleotide() {
        return Err(PlatformError::new(
            ErrorCategory::Validation,
            format!(
                "{tool} expects protein-like input but '{}' was classified as {}",
                record.identifier().accession(),
                record.molecule()
            ),
        )
        .with_code(format!("tools.{tool}.input.not_protein")));
    }

    Ok(())
}

fn derived_metadata(metadata: &SequenceMetadata, suffix: &str) -> SequenceMetadata {
    let mut derived = metadata.clone();
    derived.description = Some(match metadata.description.as_deref() {
        Some(description) => format!("{description} ({suffix})"),
        None => suffix.to_owned(),
    });
    derived
}

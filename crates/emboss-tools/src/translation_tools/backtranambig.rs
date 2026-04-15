//! `backtranambig` implementation.

use emboss_core::{MoleculeKind, SequenceMetadata, SequenceRecord, backtranslate_ambiguous};
use emboss_diagnostics::{ErrorCategory, PlatformError};

use crate::sequence_stream::{SequenceInput, ToolExecutionError, load_sequence_records};

/// Typed parameters for `backtranambig`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BacktranambigParams {
    /// Local protein input path.
    pub input: SequenceInput,
}

/// Structured `backtranambig` outcome.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BacktranambigOutcome {
    /// Source input.
    pub input: SequenceInput,
    /// Back-translated ambiguous nucleotide records.
    pub records: Vec<SequenceRecord>,
}

/// Returns `backtranambig` help text.
#[must_use]
pub fn backtranambig_help() -> &'static str {
    "Usage: emboss-rs backtranambig <protein-input>\n\nBack-translate protein sequence records into ambiguous DNA codons using IUPAC nucleotide ambiguity and the standard genetic code. Stop symbols '*' are rendered as TAR. Output is emitted as FASTA DNA sequences."
}

/// Executes `backtranambig`.
pub fn run_backtranambig(
    params: BacktranambigParams,
) -> Result<BacktranambigOutcome, ToolExecutionError> {
    let records = load_sequence_records(&params.input)?
        .into_iter()
        .map(backtranslate_record)
        .collect::<Result<Vec<_>, _>>()?;

    Ok(BacktranambigOutcome {
        input: params.input,
        records,
    })
}

fn backtranslate_record(record: SequenceRecord) -> Result<SequenceRecord, ToolExecutionError> {
    validate_protein_record("backtranambig", &record)?;
    let residues = backtranslate_ambiguous(record.residues()).map_err(|error| {
        PlatformError::new(ErrorCategory::Validation, error.to_string())
            .with_code("tools.backtranambig.residue.invalid")
    })?;

    let metadata = derived_metadata(record.metadata(), "ambiguous back-translated DNA");
    SequenceRecord::new(record.identifier().clone(), MoleculeKind::Dna, residues)
        .map(|record| record.with_metadata(metadata))
        .map_err(|error| {
            PlatformError::new(ErrorCategory::Validation, error.to_string())
                .with_code("tools.backtranambig.sequence.invalid")
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

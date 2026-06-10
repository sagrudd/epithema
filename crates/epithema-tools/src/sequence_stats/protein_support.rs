//! Narrow shared protein-input validation for retained protein statistics tools.

use epithema_core::{Alphabet, MoleculeKind, SequenceRecord};
use epithema_diagnostics::{ErrorCategory, PlatformError};

use crate::sequence_stream::ToolExecutionError;

pub(crate) fn validate_protein_record(
    tool: &str,
    record: &SequenceRecord,
) -> Result<(), ToolExecutionError> {
    if record.molecule().is_nucleotide() {
        return Err(PlatformError::new(
            ErrorCategory::Validation,
            format!(
                "{tool} expects protein input but '{}' was classified as {}",
                record.identifier().accession(),
                record.molecule()
            ),
        )
        .with_code(format!("tools.{tool}.input.not_protein")));
    }

    if record.molecule().is_protein() || looks_like_protein_record(record) {
        return Ok(());
    }

    if looks_like_nucleotide_record(record) {
        return Err(PlatformError::new(
            ErrorCategory::Validation,
            format!(
                "{tool} expects protein input but '{}' was classified as {}",
                record.identifier().accession(),
                record.molecule()
            ),
        )
        .with_code(format!("tools.{tool}.input.not_protein")));
    }

    Err(PlatformError::new(
        ErrorCategory::Validation,
        format!(
            "{tool} expects protein input but '{}' could not be validated as protein",
            record.identifier().accession()
        ),
    )
    .with_code(format!("tools.{tool}.input.not_protein")))
}

fn looks_like_protein_record(record: &SequenceRecord) -> bool {
    Alphabet::Protein
        .validate(MoleculeKind::Protein, record.residues())
        .is_ok()
}

fn looks_like_nucleotide_record(record: &SequenceRecord) -> bool {
    Alphabet::Dna
        .validate(MoleculeKind::Dna, record.residues())
        .is_ok()
        || Alphabet::Rna
            .validate(MoleculeKind::Rna, record.residues())
            .is_ok()
}

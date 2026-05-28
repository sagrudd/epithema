//! Shared helpers for sequence-edit methods.

use emboss_core::{MoleculeKind, SequenceRecord};
use emboss_diagnostics::{ErrorCategory, PlatformError};

use crate::sequence_stream::ToolExecutionError;

pub(crate) fn rebuild_unannotated_record(
    tool: &str,
    record: &SequenceRecord,
    residues: String,
) -> Result<SequenceRecord, ToolExecutionError> {
    let rebuilt = SequenceRecord::new(record.identifier().clone(), record.molecule(), residues)
        .map_err(|error| {
            PlatformError::new(ErrorCategory::Validation, error.to_string())
                .with_code(format!("tools.{tool}.sequence.invalid"))
        })?
        .with_metadata(record.metadata().clone());
    Ok(rebuilt)
}

pub(crate) fn require_nucleotide_record(
    tool: &str,
    record: &SequenceRecord,
) -> Result<(), ToolExecutionError> {
    if matches!(record.molecule(), MoleculeKind::Dna | MoleculeKind::Rna)
        || (record.molecule() == MoleculeKind::Unknown && looks_nucleotide_like(record.residues()))
    {
        Ok(())
    } else {
        Err(PlatformError::new(
            ErrorCategory::Validation,
            format!(
                "{tool} requires nucleotide sequence records; observed {} for '{}'",
                record.molecule(),
                record.identifier().accession()
            ),
        )
        .with_code(format!("tools.{tool}.molecule.invalid")))
    }
}

pub(crate) fn compatible_nucleotide_kinds(left: &SequenceRecord, right: &SequenceRecord) -> bool {
    left.molecule() == right.molecule()
        || left.molecule() == MoleculeKind::Unknown
        || right.molecule() == MoleculeKind::Unknown
}

fn looks_nucleotide_like(residues: &str) -> bool {
    residues.chars().all(|symbol| {
        matches!(
            symbol,
            'A' | 'C'
                | 'G'
                | 'T'
                | 'U'
                | 'R'
                | 'Y'
                | 'S'
                | 'W'
                | 'K'
                | 'M'
                | 'B'
                | 'D'
                | 'H'
                | 'V'
                | 'N'
                | '-'
                | '.'
                | '*'
        )
    })
}

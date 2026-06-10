//! `pasteseq` implementation.

use epithema_core::SequenceRecord;
use epithema_diagnostics::{ErrorCategory, PlatformError};

use crate::sequence_stream::{SequenceInput, ToolExecutionError};
use crate::sequence_transform::shared::load_exactly_one_record;

/// Typed parameters for `pasteseq`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PasteseqParams {
    /// Main sequence input path.
    pub asequence: SequenceInput,
    /// Inserted sequence input path.
    pub bsequence: SequenceInput,
    /// 1-based insertion position after which the inserted sequence is placed.
    /// Zero inserts before the start of the main sequence.
    pub position: usize,
}

/// Structured `pasteseq` outcome.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PasteseqOutcome {
    /// Main sequence input.
    pub asequence: SequenceInput,
    /// Inserted sequence input.
    pub bsequence: SequenceInput,
    /// Requested insertion position.
    pub position: usize,
    /// Output merged sequence.
    pub record: SequenceRecord,
}

/// Returns `pasteseq` help text.
#[must_use]
pub fn pasteseq_help() -> &'static str {
    "Usage: epithema pasteseq <asequence> <bsequence> <position>\n\nInsert exactly one sequence record into another after a 1-based position in the main input. Position 0 inserts before the start and position length(asequence) inserts at the end. Epithema v1 requires matching molecule kinds and emits one unannotated merged sequence record."
}

/// Executes `pasteseq`.
pub fn run_pasteseq(params: PasteseqParams) -> Result<PasteseqOutcome, ToolExecutionError> {
    let main = load_exactly_one_record(&params.asequence, "pasteseq", "main")?;
    let inserted = load_exactly_one_record(&params.bsequence, "pasteseq", "inserted")?;
    let record = paste_records(main, inserted, params.position)?;

    Ok(PasteseqOutcome {
        asequence: params.asequence,
        bsequence: params.bsequence,
        position: params.position,
        record,
    })
}

fn paste_records(
    main: SequenceRecord,
    inserted: SequenceRecord,
    position: usize,
) -> Result<SequenceRecord, ToolExecutionError> {
    if main.molecule() != inserted.molecule() {
        return Err(PlatformError::new(
            ErrorCategory::Validation,
            format!(
                "pasteseq requires matching molecule kinds; observed {} and {}",
                main.molecule(),
                inserted.molecule()
            ),
        )
        .with_code("tools.pasteseq.molecule.mismatch"));
    }

    if position > main.len() {
        return Err(PlatformError::new(
            ErrorCategory::Validation,
            format!(
                "pasteseq position {position} is out of range for main sequence '{}' of length {}",
                main.identifier().accession(),
                main.len()
            ),
        )
        .with_code("tools.pasteseq.position.out_of_range"));
    }

    let residues = format!(
        "{}{}{}",
        &main.residues()[..position],
        inserted.residues(),
        &main.residues()[position..]
    );
    let mut record = SequenceRecord::new(main.identifier().clone(), main.molecule(), residues)
        .map_err(|error| {
            PlatformError::new(ErrorCategory::Validation, error.to_string())
                .with_code("tools.pasteseq.record.invalid")
        })?;
    record = record.with_metadata(main.metadata().clone());
    Ok(record)
}

#[cfg(test)]
mod tests {
    use epithema_core::{MoleculeKind, SequenceIdentifier};

    use super::paste_records;

    fn record(id: &str, molecule: MoleculeKind, residues: &str) -> epithema_core::SequenceRecord {
        epithema_core::SequenceRecord::new(
            SequenceIdentifier::new(id).expect("valid identifier"),
            molecule,
            residues,
        )
        .expect("valid sequence")
    }

    #[test]
    fn inserts_sequence_after_requested_position() {
        let merged = paste_records(
            record("main", MoleculeKind::Dna, "ACGT"),
            record("insert", MoleculeKind::Dna, "TT"),
            2,
        )
        .expect("pasteseq should merge");
        assert_eq!(merged.residues(), "ACTTGT");
    }

    #[test]
    fn supports_insertion_before_start() {
        let merged = paste_records(
            record("main", MoleculeKind::Dna, "ACGT"),
            record("insert", MoleculeKind::Dna, "TT"),
            0,
        )
        .expect("pasteseq should merge");
        assert_eq!(merged.residues(), "TTACGT");
    }

    #[test]
    fn rejects_molecule_mismatch() {
        let error = paste_records(
            record("main", MoleculeKind::Dna, "ACGT"),
            record("insert", MoleculeKind::Protein, "MA"),
            2,
        )
        .expect_err("molecule mismatch should fail");
        assert_eq!(error.code(), Some("tools.pasteseq.molecule.mismatch"));
    }
}

//! `vectorstrip` implementation.

use epithema_core::SequenceRecord;
use epithema_diagnostics::{ErrorCategory, PlatformError};

use crate::sequence_stream::{SequenceInput, ToolExecutionError, load_sequence_records};
use crate::sequence_transform::load_exactly_one_record;

use super::shared::{
    compatible_nucleotide_kinds, rebuild_unannotated_record, require_nucleotide_record,
};

/// Typed parameters for `vectorstrip`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VectorstripParams {
    /// Input sequence stream.
    pub input: SequenceInput,
    /// Vector sequence input.
    pub vector: SequenceInput,
}

/// Structured `vectorstrip` outcome.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VectorstripOutcome {
    /// Source input.
    pub input: SequenceInput,
    /// Vector sequence input.
    pub vector: SequenceInput,
    /// Stripped records in input order.
    pub records: Vec<SequenceRecord>,
}

/// Returns `vectorstrip` help text.
#[must_use]
pub fn vectorstrip_help() -> &'static str {
    "Usage: epithema vectorstrip <input> <vector-input>\n\nRemove exact vector-sequence matches from the left and right ends of each nucleotide input record. v1 requires exactly one vector record and strips only full-length exact terminal matches."
}

/// Executes `vectorstrip`.
pub fn run_vectorstrip(
    params: VectorstripParams,
) -> Result<VectorstripOutcome, ToolExecutionError> {
    let vector = load_exactly_one_record(&params.vector, "vectorstrip", "vector")?;
    require_nucleotide_record("vectorstrip", &vector)?;

    let records = load_sequence_records(&params.input)?
        .into_iter()
        .map(|record| strip_record(&record, &vector))
        .collect::<Result<Vec<_>, _>>()?;

    Ok(VectorstripOutcome {
        input: params.input,
        vector: params.vector,
        records,
    })
}

fn strip_record(
    record: &SequenceRecord,
    vector: &SequenceRecord,
) -> Result<SequenceRecord, ToolExecutionError> {
    require_nucleotide_record("vectorstrip", record)?;
    if !compatible_nucleotide_kinds(record, vector) {
        return Err(PlatformError::new(
            ErrorCategory::Validation,
            format!(
                "vectorstrip requires matching nucleotide molecule kinds; observed {} and {}",
                record.molecule(),
                vector.molecule()
            ),
        )
        .with_code("tools.vectorstrip.molecule.mismatch"));
    }

    let mut residues = record.residues().to_owned();
    let vector_residues = vector.residues();
    if residues.starts_with(vector_residues) {
        residues = residues[vector_residues.len()..].to_owned();
    }
    if residues.ends_with(vector_residues) {
        residues.truncate(residues.len() - vector_residues.len());
    }

    if residues.is_empty() {
        return Err(PlatformError::new(
            ErrorCategory::Validation,
            format!(
                "vectorstrip would remove all residues from sequence '{}'",
                record.identifier().accession()
            ),
        )
        .with_code("tools.vectorstrip.sequence.empty"));
    }

    rebuild_unannotated_record("vectorstrip", record, residues)
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::sequence_stream::SequenceInput;

    use super::{VectorstripParams, run_vectorstrip};

    fn fixture(name: &str) -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join(format!("../../crates/epithema-tools/tests/fixtures/{name}"))
    }

    #[test]
    fn strips_exact_terminal_vector_matches() {
        let outcome = run_vectorstrip(VectorstripParams {
            input: SequenceInput::new(fixture("vectorstrip_records.fasta")),
            vector: SequenceInput::new(fixture("vectorstrip_vector.fasta")),
        })
        .expect("vectorstrip should succeed");

        assert_eq!(outcome.records[0].residues(), "ACGT");
        assert_eq!(outcome.records[1].residues(), "TTAA");
        assert_eq!(outcome.records[2].residues(), "GGCC");
    }
}

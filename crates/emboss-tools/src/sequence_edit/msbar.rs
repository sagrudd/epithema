//! `msbar` implementation.

use std::collections::BTreeSet;

use emboss_core::SequenceRecord;
use emboss_diagnostics::{ErrorCategory, PlatformError};

use crate::sequence_stream::{SequenceInput, ToolExecutionError, load_sequence_records};

use super::shared::rebuild_unannotated_record;

/// One explicit point mutation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MsbarMutation {
    /// 1-based residue position.
    pub position: usize,
    /// Replacement residue.
    pub residue: char,
}
/// Typed parameters for `msbar`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MsbarParams {
    /// Input sequence stream.
    pub input: SequenceInput,
    /// Mutations applied to every input record.
    pub mutations: Vec<MsbarMutation>,
}

/// Structured `msbar` outcome.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MsbarOutcome {
    /// Source input.
    pub input: SequenceInput,
    /// Applied mutations.
    pub mutations: Vec<MsbarMutation>,
    /// Mutated records in input order.
    pub records: Vec<SequenceRecord>,
}

/// Returns `msbar` help text.
#[must_use]
pub fn msbar_help() -> &'static str {
    "Usage: emboss-rs msbar <input> <position:residue> [<position:residue> ...]\n\nApply one or more explicit 1-based point mutations to every input record. v1 supports substitution-only edits and drops feature annotations after mutation."
}

/// Executes `msbar`.
pub fn run_msbar(params: MsbarParams) -> Result<MsbarOutcome, ToolExecutionError> {
    if params.mutations.is_empty() {
        return Err(PlatformError::new(
            ErrorCategory::Validation,
            "msbar requires at least one mutation",
        )
        .with_code("tools.msbar.mutations.missing"));
    }
    let mut seen = BTreeSet::new();
    for mutation in &params.mutations {
        if mutation.position == 0 {
            return Err(PlatformError::new(
                ErrorCategory::Validation,
                "msbar mutation positions are 1-based and must be >= 1",
            )
            .with_code("tools.msbar.position.invalid"));
        }
        if !seen.insert(mutation.position) {
            return Err(PlatformError::new(
                ErrorCategory::Validation,
                format!(
                    "msbar received duplicate mutation for position {}",
                    mutation.position
                ),
            )
            .with_code("tools.msbar.position.duplicate"));
        }
    }

    let records = load_sequence_records(&params.input)?
        .into_iter()
        .map(|record| mutate_record(&record, &params.mutations))
        .collect::<Result<Vec<_>, _>>()?;

    Ok(MsbarOutcome {
        input: params.input,
        mutations: params.mutations,
        records,
    })
}

fn mutate_record(
    record: &SequenceRecord,
    mutations: &[MsbarMutation],
) -> Result<SequenceRecord, ToolExecutionError> {
    let mut residues: Vec<char> = record.residues().chars().collect();
    for mutation in mutations {
        let index = mutation.position - 1;
        if index >= residues.len() {
            return Err(PlatformError::new(
                ErrorCategory::Validation,
                format!(
                    "msbar position {} is out of range for sequence '{}' of length {}",
                    mutation.position,
                    record.identifier().accession(),
                    record.len()
                ),
            )
            .with_code("tools.msbar.position.out_of_range"));
        }
        residues[index] = mutation.residue.to_ascii_uppercase();
    }

    rebuild_unannotated_record("msbar", record, residues.into_iter().collect())
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::sequence_stream::SequenceInput;

    use super::{MsbarMutation, MsbarParams, run_msbar};

    fn fixture(name: &str) -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join(format!("../../crates/emboss-tools/tests/fixtures/{name}"))
    }

    #[test]
    fn applies_point_mutations() {
        let outcome = run_msbar(MsbarParams {
            input: SequenceInput::new(fixture("msbar_records.fasta")),
            mutations: vec![
                MsbarMutation {
                    position: 2,
                    residue: 'T',
                },
                MsbarMutation {
                    position: 4,
                    residue: 'A',
                },
            ],
        })
        .expect("msbar should succeed");

        assert_eq!(outcome.records[0].residues(), "ATGA");
    }
}

//! Shared helpers for sequence-transform methods.

use epithema_core::{MoleculeKind, SequenceIdentifier, SequenceRecord};
use epithema_diagnostics::{ErrorCategory, PlatformError};

use crate::sequence_stream::{SequenceInput, ToolExecutionError, load_sequence_records};

pub(crate) fn load_exactly_one_record(
    input: &SequenceInput,
    tool: &str,
    label: &str,
) -> Result<SequenceRecord, ToolExecutionError> {
    let mut records = load_sequence_records(input)?;
    if records.len() != 1 {
        return Err(PlatformError::new(
            ErrorCategory::Validation,
            format!(
                "{tool} requires exactly one sequence record in the {label} input '{}'",
                input.path.display()
            ),
        )
        .with_code(format!("tools.{tool}.input.record_count.invalid")));
    }

    Ok(records.remove(0))
}

pub(crate) fn longest_exact_suffix_prefix_overlap(left: &str, right: &str) -> usize {
    let max_overlap = left.len().min(right.len());
    for overlap in (1..=max_overlap).rev() {
        if left[left.len() - overlap..] == right[..overlap] {
            return overlap;
        }
    }

    0
}

pub(crate) fn merge_records_by_exact_overlap(
    tool: &str,
    left: SequenceRecord,
    right: SequenceRecord,
    required_molecule: Option<MoleculeKind>,
) -> Result<(SequenceRecord, usize), ToolExecutionError> {
    if let Some(expected) = required_molecule {
        if left.molecule() != expected || right.molecule() != expected {
            return Err(PlatformError::new(
                ErrorCategory::Validation,
                format!(
                    "{tool} requires {expected} inputs; observed {} and {}",
                    left.molecule(),
                    right.molecule()
                ),
            )
            .with_code(format!("tools.{tool}.molecule.invalid")));
        }
    } else if left.molecule() != right.molecule() {
        return Err(PlatformError::new(
            ErrorCategory::Validation,
            format!(
                "{tool} requires matching molecule kinds; observed {} and {}",
                left.molecule(),
                right.molecule()
            ),
        )
        .with_code(format!("tools.{tool}.molecule.mismatch")));
    }

    let overlap = longest_exact_suffix_prefix_overlap(left.residues(), right.residues());
    if overlap == 0 {
        return Err(PlatformError::new(
            ErrorCategory::Validation,
            format!(
                "{tool} did not find any positive exact suffix/prefix overlap between '{}' and '{}'",
                left.identifier().accession(),
                right.identifier().accession()
            ),
        )
        .with_code(format!("tools.{tool}.overlap.missing")));
    }

    let merged_residues = format!("{}{}", left.residues(), &right.residues()[overlap..]);
    let identifier = SequenceIdentifier::new(format!(
        "{}+{}",
        left.identifier().accession(),
        right.identifier().accession()
    ))
    .map_err(|error| {
        PlatformError::new(ErrorCategory::Validation, error.to_string())
            .with_code(format!("tools.{tool}.identifier.invalid"))
    })?;
    let merged = SequenceRecord::new(identifier, left.molecule(), merged_residues)
        .map_err(|error| {
            PlatformError::new(ErrorCategory::Validation, error.to_string())
                .with_code(format!("tools.{tool}.record.invalid"))
        })?
        .with_metadata(left.metadata().clone());

    Ok((merged, overlap))
}

pub(crate) fn derived_seed(base_seed: u64, ordinal: usize) -> u64 {
    base_seed ^ ((ordinal as u64 + 1).wrapping_mul(0x9E37_79B9_7F4A_7C15))
}

pub(crate) fn shuffled_residues(residues: &str, seed: u64) -> String {
    let mut chars: Vec<char> = residues.chars().collect();
    if chars.len() <= 1 {
        return residues.to_owned();
    }

    let mut rng = DeterministicRng::new(seed);
    for index in (1..chars.len()).rev() {
        let swap_with = rng.next_bounded(index + 1);
        chars.swap(index, swap_with);
    }

    chars.into_iter().collect()
}

#[derive(Clone, Debug)]
struct DeterministicRng {
    state: u64,
}

impl DeterministicRng {
    fn new(seed: u64) -> Self {
        let state = if seed == 0 {
            0xA5A5_A5A5_A5A5_A5A5
        } else {
            seed
        };
        Self { state }
    }

    fn next_u64(&mut self) -> u64 {
        let mut state = self.state;
        state ^= state >> 12;
        state ^= state << 25;
        state ^= state >> 27;
        self.state = state;
        state.wrapping_mul(0x2545_F491_4F6C_DD1D)
    }

    fn next_bounded(&mut self, upper: usize) -> usize {
        if upper <= 1 {
            return 0;
        }

        (self.next_u64() % upper as u64) as usize
    }
}

#[cfg(test)]
mod tests {
    use super::{derived_seed, longest_exact_suffix_prefix_overlap, shuffled_residues};

    #[test]
    fn detects_longest_exact_overlap() {
        assert_eq!(longest_exact_suffix_prefix_overlap("ACGTAA", "TAAGGG"), 3);
        assert_eq!(longest_exact_suffix_prefix_overlap("AAAA", "AAAT"), 3);
        assert_eq!(longest_exact_suffix_prefix_overlap("AAAA", "TTTT"), 0);
    }

    #[test]
    fn shuffling_is_deterministic_for_a_seed() {
        assert_eq!(
            shuffled_residues("ACGTAC", 7),
            shuffled_residues("ACGTAC", 7)
        );
        assert_ne!(
            shuffled_residues("ACGTAC", 7),
            shuffled_residues("ACGTAC", 8)
        );
    }

    #[test]
    fn derives_distinct_per_record_seeds() {
        assert_ne!(derived_seed(1, 0), derived_seed(1, 1));
    }
}

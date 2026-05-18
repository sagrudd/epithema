//! Shared deterministic set operations over sequence streams.

use std::collections::BTreeSet;

use emboss_core::{MoleculeKind, SequenceRecord};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SequenceSetOperator {
    /// Include records occurring in the first set, the second set, or both.
    Or,
    /// Include records occurring in both sets.
    And,
    /// Include records occurring in exactly one of the two sets.
    Xor,
    /// Include records occurring in the first set but not the second.
    Not,
}

impl SequenceSetOperator {
    /// Returns the stable user-facing operator label.
    #[must_use]
    pub fn label(self) -> &'static str {
        match self {
            Self::Or => "OR",
            Self::And => "AND",
            Self::Xor => "XOR",
            Self::Not => "NOT",
        }
    }
}

pub fn unique_records_in_order(records: Vec<SequenceRecord>) -> (Vec<SequenceRecord>, usize) {
    let mut seen = BTreeSet::new();
    let mut unique = Vec::new();
    let mut duplicate_count = 0usize;

    for record in records {
        let key = sequence_identity_key(&record);
        if seen.insert(key) {
            unique.push(record);
        } else {
            duplicate_count += 1;
        }
    }

    (unique, duplicate_count)
}

pub fn apply_sequence_set_operator(
    first: Vec<SequenceRecord>,
    second: Vec<SequenceRecord>,
    operator: SequenceSetOperator,
) -> (Vec<SequenceRecord>, usize, usize) {
    let (first_unique, first_duplicate_count) = unique_records_in_order(first);
    let (second_unique, second_duplicate_count) = unique_records_in_order(second);
    let first_keys = first_unique
        .iter()
        .map(sequence_identity_key)
        .collect::<BTreeSet<_>>();
    let second_keys = second_unique
        .iter()
        .map(sequence_identity_key)
        .collect::<BTreeSet<_>>();

    let records = match operator {
        SequenceSetOperator::Or => first_unique
            .into_iter()
            .chain(
                second_unique
                    .into_iter()
                    .filter(|record| !first_keys.contains(&sequence_identity_key(record))),
            )
            .collect(),
        SequenceSetOperator::And => first_unique
            .into_iter()
            .filter(|record| second_keys.contains(&sequence_identity_key(record)))
            .collect(),
        SequenceSetOperator::Xor => first_unique
            .into_iter()
            .filter(|record| !second_keys.contains(&sequence_identity_key(record)))
            .chain(
                second_unique
                    .into_iter()
                    .filter(|record| !first_keys.contains(&sequence_identity_key(record))),
            )
            .collect(),
        SequenceSetOperator::Not => first_unique
            .into_iter()
            .filter(|record| !second_keys.contains(&sequence_identity_key(record)))
            .collect(),
    };

    (records, first_duplicate_count, second_duplicate_count)
}

fn sequence_identity_key(record: &SequenceRecord) -> String {
    format!(
        "{}:{}",
        molecule_label(record.molecule()),
        record.residues().to_ascii_uppercase()
    )
}

fn molecule_label(molecule: MoleculeKind) -> &'static str {
    match molecule {
        MoleculeKind::Dna => "dna",
        MoleculeKind::Rna => "rna",
        MoleculeKind::Protein => "protein",
        MoleculeKind::Unknown => "unknown",
    }
}

#[cfg(test)]
mod tests {
    use super::{SequenceSetOperator, apply_sequence_set_operator, unique_records_in_order};
    use emboss_core::{MoleculeKind, SequenceIdentifier, SequenceRecord};

    fn record(id: &str, molecule: MoleculeKind, residues: &str) -> SequenceRecord {
        SequenceRecord::new(
            SequenceIdentifier::new(id).expect("valid identifier"),
            molecule,
            residues,
        )
        .expect("valid record")
    }

    #[test]
    fn removes_exact_duplicates_in_stable_order() {
        let (records, duplicates) = unique_records_in_order(vec![
            record("a", MoleculeKind::Dna, "ACGT"),
            record("b", MoleculeKind::Dna, "ACGT"),
            record("c", MoleculeKind::Dna, "TTTT"),
        ]);

        assert_eq!(duplicates, 1);
        assert_eq!(records.len(), 2);
        assert_eq!(records[0].identifier().accession(), "a");
        assert_eq!(records[1].identifier().accession(), "c");
    }

    #[test]
    fn computes_xor_with_stable_first_then_second_order() {
        let (records, _, _) = apply_sequence_set_operator(
            vec![
                record("a", MoleculeKind::Dna, "AAAA"),
                record("b", MoleculeKind::Dna, "CCCC"),
            ],
            vec![
                record("x", MoleculeKind::Dna, "CCCC"),
                record("y", MoleculeKind::Dna, "GGGG"),
            ],
            SequenceSetOperator::Xor,
        );

        let ids: Vec<_> = records
            .iter()
            .map(|record| record.identifier().accession().to_owned())
            .collect();
        assert_eq!(ids, vec!["a", "y"]);
    }
}

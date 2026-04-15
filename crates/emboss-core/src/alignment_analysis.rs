//! Deterministic alignment-derived analysis helpers.

use std::collections::{BTreeMap, BTreeSet};

use crate::{
    Alignment, AlignmentMode, AlignmentSymbol, MoleculeKind, SequenceIdentifier, SequenceRecord,
    global_alignment::infer_alignment_mode,
};

/// Summary of a direct ungapped positional comparison between two sequences.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DirectMatchSummary {
    /// Comparison mode inferred from the two records.
    pub mode: AlignmentMode,
    /// Query length in residues.
    pub query_length: usize,
    /// Target length in residues.
    pub target_length: usize,
    /// Number of positions compared over the shared overlap.
    pub compared_length: usize,
    /// Count of identical positions in the shared overlap.
    pub identity_count: usize,
    /// Count of mismatched positions in the shared overlap.
    pub mismatch_count: usize,
    /// Integer identity percentage over the compared length.
    pub identity_percent: usize,
    /// Signed target-minus-query length difference.
    pub length_difference: isize,
}

/// Distance matrix derived from a set of equal-length sequences.
#[derive(Clone, Debug, PartialEq)]
pub struct DistanceMatrix {
    /// Stable record identifiers in matrix row and column order.
    pub identifiers: Vec<String>,
    /// Comparison mode inferred for the matrix.
    pub mode: AlignmentMode,
    /// Shared sequence length required for all records.
    pub sequence_length: usize,
    /// Pairwise p-distance values.
    pub values: Vec<Vec<f64>>,
}

/// Deterministic consensus strategy.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ConsensusStrategy {
    /// Majority non-gap residue wins; ties fall back to a placeholder.
    Simple,
    /// Nucleotide consensus emits IUPAC ambiguity symbols where possible.
    Ambiguous,
}

/// Deterministic alignment-analysis errors.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AlignmentAnalysisError {
    /// Sequences could not be compared under a common nucleotide or protein mode.
    IncompatibleMolecules {
        /// Left molecule kind.
        left: MoleculeKind,
        /// Right molecule kind.
        right: MoleculeKind,
    },
    /// A distance matrix requires at least one input record.
    EmptySequenceSet,
    /// A distance matrix requires all records to share the same length.
    UnequalSequenceLengths {
        /// Expected sequence length.
        expected: usize,
        /// Observed sequence length.
        observed: usize,
        /// Record identifier with the unexpected length.
        identifier: String,
    },
    /// Alignment rows could not be interpreted under one broad molecule mode.
    MixedAlignmentMolecules,
}

impl std::fmt::Display for AlignmentAnalysisError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IncompatibleMolecules { left, right } => write!(
                f,
                "alignment analysis requires both inputs to be nucleotide or both protein: left {left}, right {right}"
            ),
            Self::EmptySequenceSet => {
                write!(
                    f,
                    "distance-matrix analysis requires at least one sequence record"
                )
            }
            Self::UnequalSequenceLengths {
                expected,
                observed,
                identifier,
            } => write!(
                f,
                "distance-matrix analysis requires equal-length sequences: expected {expected}, observed {observed} for '{identifier}'"
            ),
            Self::MixedAlignmentMolecules => write!(
                f,
                "consensus analysis requires an alignment that can be classified as nucleotide or protein"
            ),
        }
    }
}

impl std::error::Error for AlignmentAnalysisError {}

/// Computes a deterministic ungapped positional comparison over the shared overlap.
pub fn direct_match_summary(
    query: &SequenceRecord,
    target: &SequenceRecord,
) -> Result<DirectMatchSummary, AlignmentAnalysisError> {
    let mode = infer_alignment_mode(query, target).map_err(|_| {
        AlignmentAnalysisError::IncompatibleMolecules {
            left: query.molecule(),
            right: target.molecule(),
        }
    })?;

    let compared_length = query.len().min(target.len());
    let identity_count = query
        .residues()
        .chars()
        .zip(target.residues().chars())
        .take(compared_length)
        .filter(|(left, right)| left == right)
        .count();
    let mismatch_count = compared_length.saturating_sub(identity_count);
    let identity_percent = if compared_length == 0 {
        0
    } else {
        (identity_count * 100) / compared_length
    };

    Ok(DirectMatchSummary {
        mode,
        query_length: query.len(),
        target_length: target.len(),
        compared_length,
        identity_count,
        mismatch_count,
        identity_percent,
        length_difference: target.len() as isize - query.len() as isize,
    })
}

/// Computes a deterministic equal-length p-distance matrix for a sequence set.
pub fn p_distance_matrix(
    records: &[SequenceRecord],
) -> Result<DistanceMatrix, AlignmentAnalysisError> {
    let first = records
        .first()
        .ok_or(AlignmentAnalysisError::EmptySequenceSet)?;
    let sequence_length = first.len();

    for record in &records[1..] {
        if record.len() != sequence_length {
            return Err(AlignmentAnalysisError::UnequalSequenceLengths {
                expected: sequence_length,
                observed: record.len(),
                identifier: record.identifier().accession().to_owned(),
            });
        }
    }

    let mode = infer_record_set_mode(records)?;
    let mut values = vec![vec![0.0; records.len()]; records.len()];
    for left in 0..records.len() {
        for right in left + 1..records.len() {
            let summary = direct_match_summary(&records[left], &records[right])?;
            let distance = if sequence_length == 0 {
                0.0
            } else {
                summary.mismatch_count as f64 / sequence_length as f64
            };
            values[left][right] = distance;
            values[right][left] = distance;
        }
    }

    Ok(DistanceMatrix {
        identifiers: records
            .iter()
            .map(|record| record.identifier().accession().to_owned())
            .collect(),
        mode,
        sequence_length,
        values,
    })
}

/// Derives a consensus sequence for the supplied alignment.
pub fn consensus_sequence(
    alignment: &Alignment,
    strategy: ConsensusStrategy,
    identifier: SequenceIdentifier,
) -> Result<SequenceRecord, AlignmentAnalysisError> {
    let molecule = infer_alignment_molecule(alignment)?;
    let residues: String = (0..alignment.column_count())
        .map(|column| consensus_symbol(alignment, column, molecule, strategy))
        .collect();

    SequenceRecord::new(identifier, molecule, residues)
        .map_err(|_| AlignmentAnalysisError::MixedAlignmentMolecules)
}

fn infer_record_set_mode(
    records: &[SequenceRecord],
) -> Result<AlignmentMode, AlignmentAnalysisError> {
    let mut mode = None;
    for record in records {
        let inferred = if record.molecule().is_nucleotide()
            || (record.molecule() == MoleculeKind::Unknown
                && is_nucleotide_string(record.residues()))
        {
            AlignmentMode::Nucleotide
        } else if record.molecule().is_protein()
            || (record.molecule() == MoleculeKind::Unknown
                && !is_nucleotide_string(record.residues()))
        {
            AlignmentMode::Protein
        } else {
            return Err(AlignmentAnalysisError::IncompatibleMolecules {
                left: records[0].molecule(),
                right: record.molecule(),
            });
        };

        if let Some(existing) = mode {
            if existing != inferred {
                return Err(AlignmentAnalysisError::IncompatibleMolecules {
                    left: records[0].molecule(),
                    right: record.molecule(),
                });
            }
        } else {
            mode = Some(inferred);
        }
    }

    Ok(mode.expect("non-empty sequence sets have a mode"))
}

fn infer_alignment_molecule(alignment: &Alignment) -> Result<MoleculeKind, AlignmentAnalysisError> {
    let mut saw_protein = false;
    let mut saw_nucleotide = false;

    for row in alignment.rows() {
        let aligned = row.ungapped();
        if row.molecule().is_nucleotide() || is_nucleotide_string(&aligned) {
            saw_nucleotide = true;
        } else if row.molecule().is_protein() || !is_nucleotide_string(&aligned) {
            saw_protein = true;
        }
    }

    match (saw_nucleotide, saw_protein) {
        (true, false) => Ok(MoleculeKind::Dna),
        (false, true) => Ok(MoleculeKind::Protein),
        _ => Err(AlignmentAnalysisError::MixedAlignmentMolecules),
    }
}

fn consensus_symbol(
    alignment: &Alignment,
    column: usize,
    molecule: MoleculeKind,
    strategy: ConsensusStrategy,
) -> char {
    let mut counts: BTreeMap<char, usize> = BTreeMap::new();
    let mut exact_nucleotide_observations: BTreeSet<char> = BTreeSet::new();

    for symbol in alignment
        .column(column)
        .expect("validated alignment columns are in range")
    {
        if let AlignmentSymbol::Residue(residue) = symbol {
            *counts.entry(residue).or_insert(0) += 1;
            if molecule.is_nucleotide() && matches!(residue, 'A' | 'C' | 'G' | 'T' | 'U') {
                exact_nucleotide_observations.insert(residue);
            }
        }
    }

    if counts.is_empty() {
        return fallback_placeholder(molecule);
    }

    let max_count = counts.values().copied().max().unwrap_or(0);
    let winners: Vec<char> = counts
        .iter()
        .filter_map(|(residue, count)| (*count == max_count).then_some(*residue))
        .collect();

    if winners.len() == 1 {
        return winners[0];
    }

    match strategy {
        ConsensusStrategy::Simple => fallback_placeholder(molecule),
        ConsensusStrategy::Ambiguous if molecule.is_nucleotide() => {
            nucleotide_ambiguity_symbol(&exact_nucleotide_observations)
                .unwrap_or_else(|| fallback_placeholder(molecule))
        }
        ConsensusStrategy::Ambiguous => 'X',
    }
}

fn fallback_placeholder(molecule: MoleculeKind) -> char {
    if molecule.is_nucleotide() { 'N' } else { 'X' }
}

fn nucleotide_ambiguity_symbol(observations: &BTreeSet<char>) -> Option<char> {
    if observations.is_empty() {
        return Some('N');
    }

    let normalized: BTreeSet<char> = observations
        .iter()
        .map(|residue| if *residue == 'U' { 'T' } else { *residue })
        .collect();

    let exact_u_only = observations.len() == 1 && observations.contains(&'U');
    if exact_u_only {
        return Some('U');
    }

    match normalized.iter().copied().collect::<Vec<_>>().as_slice() {
        ['A'] => Some('A'),
        ['C'] => Some('C'),
        ['G'] => Some('G'),
        ['T'] => Some('T'),
        ['A', 'G'] => Some('R'),
        ['C', 'T'] => Some('Y'),
        ['C', 'G'] => Some('S'),
        ['A', 'T'] => Some('W'),
        ['G', 'T'] => Some('K'),
        ['A', 'C'] => Some('M'),
        ['C', 'G', 'T'] => Some('B'),
        ['A', 'G', 'T'] => Some('D'),
        ['A', 'C', 'T'] => Some('H'),
        ['A', 'C', 'G'] => Some('V'),
        ['A', 'C', 'G', 'T'] => Some('N'),
        _ => None,
    }
}

fn is_nucleotide_string(residues: &str) -> bool {
    residues.chars().all(|symbol| {
        matches!(
            symbol,
            'A' | 'C'
                | 'G'
                | 'T'
                | 'U'
                | 'N'
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
                | '*'
        )
    })
}

#[cfg(test)]
mod tests {
    use crate::{Alignment, AlignmentRow, MoleculeKind, SequenceIdentifier};

    use super::{ConsensusStrategy, consensus_sequence, direct_match_summary, p_distance_matrix};

    fn dna_record(id: &str, residues: &str) -> crate::SequenceRecord {
        crate::SequenceRecord::new(
            SequenceIdentifier::new(id).expect("valid identifier"),
            MoleculeKind::Dna,
            residues,
        )
        .expect("valid dna sequence")
    }

    #[test]
    fn summarizes_direct_overlap_matches() {
        let summary = direct_match_summary(&dna_record("q", "ACGT"), &dna_record("t", "ACT"))
            .expect("summary should compute");
        assert_eq!(summary.compared_length, 3);
        assert_eq!(summary.identity_count, 2);
        assert_eq!(summary.mismatch_count, 1);
        assert_eq!(summary.identity_percent, 66);
        assert_eq!(summary.length_difference, -1);
    }

    #[test]
    fn computes_equal_length_distance_matrix() {
        let matrix = p_distance_matrix(&[
            dna_record("alpha", "ACGT"),
            dna_record("beta", "TTTT"),
            dna_record("gamma", "GGCC"),
        ])
        .expect("matrix should compute");

        assert_eq!(matrix.identifiers, vec!["alpha", "beta", "gamma"]);
        assert_eq!(matrix.values[0][1], 0.75);
        assert_eq!(matrix.values[0][2], 1.0);
        assert_eq!(matrix.values[1][2], 1.0);
    }

    #[test]
    fn derives_simple_and_ambiguous_consensus() {
        let alignment = Alignment::with_identifier(
            Some("demo"),
            vec![
                AlignmentRow::new(
                    SequenceIdentifier::new("alpha").expect("valid identifier"),
                    MoleculeKind::Dna,
                    "AC-GT",
                )
                .expect("valid row"),
                AlignmentRow::new(
                    SequenceIdentifier::new("beta").expect("valid identifier"),
                    MoleculeKind::Dna,
                    "ACTGT",
                )
                .expect("valid row"),
                AlignmentRow::new(
                    SequenceIdentifier::new("gamma").expect("valid identifier"),
                    MoleculeKind::Dna,
                    "ACCGT",
                )
                .expect("valid row"),
            ],
        )
        .expect("alignment should be valid");

        let simple = consensus_sequence(
            &alignment,
            ConsensusStrategy::Simple,
            SequenceIdentifier::new("cons").expect("valid identifier"),
        )
        .expect("simple consensus should compute");
        let ambiguous = consensus_sequence(
            &alignment,
            ConsensusStrategy::Ambiguous,
            SequenceIdentifier::new("consambig").expect("valid identifier"),
        )
        .expect("ambiguous consensus should compute");

        assert_eq!(simple.residues(), "ACNGT");
        assert_eq!(ambiguous.residues(), "ACYGT");
    }
}

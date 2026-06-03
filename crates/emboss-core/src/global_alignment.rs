//! Deterministic global pairwise alignment primitives.

use crate::{Alignment, AlignmentRow, MoleculeKind, SequenceIdentifier, SequenceRecord};

const NEG_INF: i32 = i32::MIN / 4;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum TraceState {
    Match,
    GapInTarget,
    GapInQuery,
}

/// Scoring mode for global pairwise alignment.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AlignmentMode {
    /// Nucleotide-oriented exact-match scoring.
    Nucleotide,
    /// Protein-oriented exact-match scoring.
    Protein,
}

/// Simple affine-gap scoring configuration.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct GlobalAlignmentScoring {
    /// Score applied to exact symbol matches.
    pub match_score: i32,
    /// Score applied to mismatched symbols.
    pub mismatch_score: i32,
    /// Penalty for opening a gap.
    pub gap_open: i32,
    /// Penalty for extending an existing gap by one symbol.
    pub gap_extend: i32,
    /// Sequence scoring mode.
    pub mode: AlignmentMode,
}

impl GlobalAlignmentScoring {
    /// Returns the default nucleotide scoring convention.
    #[must_use]
    pub fn nucleotide_default() -> Self {
        Self {
            match_score: 1,
            mismatch_score: -1,
            gap_open: 5,
            gap_extend: 1,
            mode: AlignmentMode::Nucleotide,
        }
    }

    /// Returns the default protein scoring convention.
    #[must_use]
    pub fn protein_default() -> Self {
        Self {
            match_score: 2,
            mismatch_score: -1,
            gap_open: 8,
            gap_extend: 1,
            mode: AlignmentMode::Protein,
        }
    }

    fn substitution_score(self, left: char, right: char) -> i32 {
        if left == right {
            self.match_score
        } else {
            self.mismatch_score
        }
    }
}

/// Summary statistics for a completed pairwise alignment.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PairwiseAlignmentSummary {
    /// Total alignment score.
    pub score: i32,
    /// Aligned length in columns.
    pub aligned_length: usize,
    /// Count of identical nongap aligned positions.
    pub identity_count: usize,
    /// Percentage identity over aligned columns.
    pub identity_percent: usize,
    /// Query-row gap count.
    pub query_gap_count: usize,
    /// Target-row gap count.
    pub target_gap_count: usize,
    /// Scoring mode used.
    pub mode: AlignmentMode,
}

/// Completed global pairwise alignment.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PairwiseAlignmentResult {
    /// Pairwise alignment payload.
    pub alignment: Alignment,
    /// Summary statistics derived from the aligned rows.
    pub summary: PairwiseAlignmentSummary,
}

/// Deterministic global-alignment validation and execution errors.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum GlobalAlignmentError {
    /// Query and target could not be aligned under the same scoring mode.
    IncompatibleMolecules {
        /// Query molecule kind.
        query: MoleculeKind,
        /// Target molecule kind.
        target: MoleculeKind,
    },
    /// Gap penalties must be strictly positive.
    InvalidGapPenalty {
        /// Gap-open penalty value.
        gap_open: i32,
        /// Gap-extend penalty value.
        gap_extend: i32,
    },
}

impl std::fmt::Display for GlobalAlignmentError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IncompatibleMolecules { query, target } => write!(
                f,
                "global alignment requires both sequences to be nucleotide or both protein: query {query}, target {target}"
            ),
            Self::InvalidGapPenalty {
                gap_open,
                gap_extend,
            } => write!(
                f,
                "gap penalties must be positive integers: gap_open={gap_open}, gap_extend={gap_extend}"
            ),
        }
    }
}

impl std::error::Error for GlobalAlignmentError {}

/// Infers an alignment scoring mode from two sequence records.
pub fn infer_alignment_mode(
    query: &SequenceRecord,
    target: &SequenceRecord,
) -> Result<AlignmentMode, GlobalAlignmentError> {
    match (query.molecule(), target.molecule()) {
        (left, right) if left.is_nucleotide() && right.is_nucleotide() => {
            Ok(AlignmentMode::Nucleotide)
        }
        (MoleculeKind::Unknown, right) if right.is_nucleotide() => Ok(AlignmentMode::Nucleotide),
        (left, MoleculeKind::Unknown) if left.is_nucleotide() => Ok(AlignmentMode::Nucleotide),
        (left, right) if left.is_protein() && right.is_protein() => Ok(AlignmentMode::Protein),
        (MoleculeKind::Unknown, right) if right.is_protein() => Ok(AlignmentMode::Protein),
        (left, MoleculeKind::Unknown) if left.is_protein() => Ok(AlignmentMode::Protein),
        _ => Err(GlobalAlignmentError::IncompatibleMolecules {
            query: query.molecule(),
            target: target.molecule(),
        }),
    }
}

/// Performs deterministic Needleman-Wunsch-style global alignment with affine gaps.
pub fn global_align(
    query: &SequenceRecord,
    target: &SequenceRecord,
    scoring: GlobalAlignmentScoring,
) -> Result<PairwiseAlignmentResult, GlobalAlignmentError> {
    if scoring.gap_open <= 0 || scoring.gap_extend <= 0 {
        return Err(GlobalAlignmentError::InvalidGapPenalty {
            gap_open: scoring.gap_open,
            gap_extend: scoring.gap_extend,
        });
    }

    let expected_mode = infer_alignment_mode(query, target)?;
    if scoring.mode != expected_mode {
        return Err(GlobalAlignmentError::IncompatibleMolecules {
            query: query.molecule(),
            target: target.molecule(),
        });
    }

    let query_symbols: Vec<char> = query.residues().chars().collect();
    let target_symbols: Vec<char> = target.residues().chars().collect();
    let rows = query_symbols.len() + 1;
    let cols = target_symbols.len() + 1;

    let mut m = vec![vec![NEG_INF; cols]; rows];
    let mut x = vec![vec![NEG_INF; cols]; rows];
    let mut y = vec![vec![NEG_INF; cols]; rows];

    let mut trace_m = vec![vec![TraceState::Match; cols]; rows];
    let mut trace_x = vec![vec![TraceState::Match; cols]; rows];
    let mut trace_y = vec![vec![TraceState::Match; cols]; rows];

    m[0][0] = 0;
    for i in 1..rows {
        x[i][0] = -scoring.gap_open - ((i as i32 - 1) * scoring.gap_extend);
        trace_x[i][0] = TraceState::GapInTarget;
    }
    for j in 1..cols {
        y[0][j] = -scoring.gap_open - ((j as i32 - 1) * scoring.gap_extend);
        trace_y[0][j] = TraceState::GapInQuery;
    }

    for i in 1..rows {
        for j in 1..cols {
            let substitution =
                scoring.substitution_score(query_symbols[i - 1], target_symbols[j - 1]);
            let mut best_m = m[i - 1][j - 1];
            let mut best_state = TraceState::Match;
            if x[i - 1][j - 1] > best_m {
                best_m = x[i - 1][j - 1];
                best_state = TraceState::GapInTarget;
            }
            if y[i - 1][j - 1] > best_m {
                best_m = y[i - 1][j - 1];
                best_state = TraceState::GapInQuery;
            }
            m[i][j] = best_m + substitution;
            trace_m[i][j] = best_state;

            let open_x_from_m = m[i - 1][j] - scoring.gap_open;
            let extend_x = x[i - 1][j] - scoring.gap_extend;
            let open_x_from_y = y[i - 1][j] - scoring.gap_open;
            let (best_x, state_x) = if open_x_from_m >= extend_x && open_x_from_m >= open_x_from_y {
                (open_x_from_m, TraceState::Match)
            } else if extend_x >= open_x_from_y {
                (extend_x, TraceState::GapInTarget)
            } else {
                (open_x_from_y, TraceState::GapInQuery)
            };
            x[i][j] = best_x;
            trace_x[i][j] = state_x;

            let open_y_from_m = m[i][j - 1] - scoring.gap_open;
            let extend_y = y[i][j - 1] - scoring.gap_extend;
            let open_y_from_x = x[i][j - 1] - scoring.gap_open;
            let (best_y, state_y) = if open_y_from_m >= extend_y && open_y_from_m >= open_y_from_x {
                (open_y_from_m, TraceState::Match)
            } else if extend_y >= open_y_from_x {
                (extend_y, TraceState::GapInQuery)
            } else {
                (open_y_from_x, TraceState::GapInTarget)
            };
            y[i][j] = best_y;
            trace_y[i][j] = state_y;
        }
    }

    let mut state = TraceState::Match;
    let mut best_score = m[query_symbols.len()][target_symbols.len()];
    if x[query_symbols.len()][target_symbols.len()] > best_score {
        best_score = x[query_symbols.len()][target_symbols.len()];
        state = TraceState::GapInTarget;
    }
    if y[query_symbols.len()][target_symbols.len()] > best_score {
        best_score = y[query_symbols.len()][target_symbols.len()];
        state = TraceState::GapInQuery;
    }

    let mut i = query_symbols.len();
    let mut j = target_symbols.len();
    let mut aligned_query = String::new();
    let mut aligned_target = String::new();

    while i > 0 || j > 0 {
        match state {
            TraceState::Match => {
                aligned_query.push(query_symbols[i - 1]);
                aligned_target.push(target_symbols[j - 1]);
                state = trace_m[i][j];
                i -= 1;
                j -= 1;
            }
            TraceState::GapInTarget => {
                aligned_query.push(query_symbols[i - 1]);
                aligned_target.push('-');
                state = trace_x[i][j];
                i -= 1;
            }
            TraceState::GapInQuery => {
                aligned_query.push('-');
                aligned_target.push(target_symbols[j - 1]);
                state = trace_y[i][j];
                j -= 1;
            }
        }
    }

    let aligned_query: String = aligned_query.chars().rev().collect();
    let aligned_target: String = aligned_target.chars().rev().collect();

    let query_identifier = query.identifier().clone();
    let target_identifier = unique_target_identifier(&query_identifier, target.identifier());

    let query_row = AlignmentRow::new(query_identifier, query.molecule(), aligned_query)
        .expect("validated sequence residues should produce a valid alignment row")
        .with_metadata(query.metadata().clone());
    let target_row = AlignmentRow::new(target_identifier, target.molecule(), aligned_target)
        .expect("validated sequence residues should produce a valid alignment row")
        .with_metadata(target.metadata().clone());

    let alignment_identifier = Some(format!(
        "{}_vs_{}",
        query.identifier().accession(),
        target.identifier().accession()
    ));
    let alignment = Alignment::with_identifier(alignment_identifier, vec![query_row, target_row])
        .expect("constructed pairwise rows should always build a valid alignment");

    let identity_count = alignment.rows()[0]
        .aligned()
        .chars()
        .zip(alignment.rows()[1].aligned().chars())
        .filter(|(left, right)| *left != '-' && *right != '-' && left == right)
        .count();
    let aligned_length = alignment.column_count();
    let identity_percent = if aligned_length == 0 {
        0
    } else {
        (identity_count * 100) / aligned_length
    };
    let summary = PairwiseAlignmentSummary {
        score: best_score,
        aligned_length,
        identity_count,
        identity_percent,
        query_gap_count: alignment.rows()[0].gap_count(),
        target_gap_count: alignment.rows()[1].gap_count(),
        mode: scoring.mode,
    };

    Ok(PairwiseAlignmentResult { alignment, summary })
}

fn unique_target_identifier(
    query: &SequenceIdentifier,
    target: &SequenceIdentifier,
) -> SequenceIdentifier {
    if query.accession() != target.accession() {
        return target.clone();
    }

    SequenceIdentifier::new(format!("{}.target", target.accession()))
        .expect("suffixing a validated identifier should remain valid")
}

#[cfg(test)]
mod tests {
    use super::{global_align, infer_alignment_mode, AlignmentMode, GlobalAlignmentScoring};
    use crate::{MoleculeKind, SequenceIdentifier, SequenceRecord};

    #[test]
    fn infers_alignment_modes() {
        let dna = SequenceRecord::new(
            SequenceIdentifier::new("dna").expect("identifier"),
            MoleculeKind::Dna,
            "ACGT",
        )
        .expect("sequence");
        let protein = SequenceRecord::new(
            SequenceIdentifier::new("protein").expect("identifier"),
            MoleculeKind::Protein,
            "MA",
        )
        .expect("sequence");

        assert_eq!(
            infer_alignment_mode(&dna, &dna),
            Ok(AlignmentMode::Nucleotide)
        );
        assert_eq!(
            infer_alignment_mode(&protein, &protein),
            Ok(AlignmentMode::Protein)
        );
        assert!(infer_alignment_mode(&dna, &protein).is_err());
    }

    #[test]
    fn globally_aligns_simple_nucleotide_example() {
        let query = SequenceRecord::new(
            SequenceIdentifier::new("query").expect("identifier"),
            MoleculeKind::Dna,
            "ACGT",
        )
        .expect("sequence");
        let target = SequenceRecord::new(
            SequenceIdentifier::new("target").expect("identifier"),
            MoleculeKind::Dna,
            "ACT",
        )
        .expect("sequence");

        let result = global_align(
            &query,
            &target,
            GlobalAlignmentScoring::nucleotide_default(),
        )
        .expect("alignment");
        assert_eq!(result.alignment.row_count(), 2);
        assert_eq!(result.alignment.column_count(), 4);
        assert_eq!(result.alignment.rows()[0].aligned(), "ACGT");
        assert_eq!(result.alignment.rows()[1].aligned(), "AC-T");
        assert_eq!(result.summary.identity_count, 3);
    }

    #[test]
    fn globally_aligns_simple_protein_example() {
        let query = SequenceRecord::new(
            SequenceIdentifier::new("query").expect("identifier"),
            MoleculeKind::Protein,
            "MKT",
        )
        .expect("sequence");
        let target = SequenceRecord::new(
            SequenceIdentifier::new("target").expect("identifier"),
            MoleculeKind::Protein,
            "MNT",
        )
        .expect("sequence");

        let result = global_align(&query, &target, GlobalAlignmentScoring::protein_default())
            .expect("alignment");
        assert_eq!(result.alignment.rows()[0].aligned(), "MKT");
        assert_eq!(result.alignment.rows()[1].aligned(), "MNT");
        assert_eq!(result.summary.identity_count, 2);
    }
}

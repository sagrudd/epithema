//! Deterministic local pairwise alignment primitives.

use crate::{
    Alignment, AlignmentRow, AlignmentMode, MoleculeKind, SequenceIdentifier, SequenceRecord,
    infer_alignment_mode,
};

const NEG_INF: i32 = i32::MIN / 4;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum TraceState {
    Match,
    GapInTarget,
    GapInQuery,
    Stop,
}

/// Simple affine-gap scoring configuration for local alignment.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct LocalAlignmentScoring {
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

impl LocalAlignmentScoring {
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

/// Summary statistics for a completed local pairwise alignment.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LocalPairwiseAlignmentSummary {
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
    /// Zero-based inclusive start of the aligned query span.
    pub query_start: usize,
    /// Zero-based exclusive end of the aligned query span.
    pub query_end: usize,
    /// Zero-based inclusive start of the aligned target span.
    pub target_start: usize,
    /// Zero-based exclusive end of the aligned target span.
    pub target_end: usize,
}

/// Completed local pairwise alignment.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LocalPairwiseAlignmentResult {
    /// Pairwise alignment payload.
    pub alignment: Alignment,
    /// Summary statistics derived from the aligned rows.
    pub summary: LocalPairwiseAlignmentSummary,
}

/// Deterministic local-alignment validation and execution errors.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LocalAlignmentError {
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
    /// No local alignment with a positive score could be found.
    NoPositiveAlignment,
}

impl std::fmt::Display for LocalAlignmentError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::IncompatibleMolecules { query, target } => write!(
                f,
                "local alignment requires both sequences to be nucleotide or both protein: query {query}, target {target}"
            ),
            Self::InvalidGapPenalty {
                gap_open,
                gap_extend,
            } => write!(
                f,
                "gap penalties must be positive integers: gap_open={gap_open}, gap_extend={gap_extend}"
            ),
            Self::NoPositiveAlignment => {
                write!(f, "no positive-scoring local alignment could be found")
            }
        }
    }
}

impl std::error::Error for LocalAlignmentError {}

/// Performs deterministic Smith-Waterman-style local alignment with affine gaps.
pub fn local_align(
    query: &SequenceRecord,
    target: &SequenceRecord,
    scoring: LocalAlignmentScoring,
) -> Result<LocalPairwiseAlignmentResult, LocalAlignmentError> {
    if scoring.gap_open <= 0 || scoring.gap_extend <= 0 {
        return Err(LocalAlignmentError::InvalidGapPenalty {
            gap_open: scoring.gap_open,
            gap_extend: scoring.gap_extend,
        });
    }

    let expected_mode = infer_alignment_mode(query, target).map_err(|error| match error {
        crate::GlobalAlignmentError::IncompatibleMolecules { query, target } => {
            LocalAlignmentError::IncompatibleMolecules { query, target }
        }
        crate::GlobalAlignmentError::InvalidGapPenalty {
            gap_open,
            gap_extend,
        } => LocalAlignmentError::InvalidGapPenalty {
            gap_open,
            gap_extend,
        },
    })?;
    if scoring.mode != expected_mode {
        return Err(LocalAlignmentError::IncompatibleMolecules {
            query: query.molecule(),
            target: target.molecule(),
        });
    }

    let query_symbols: Vec<char> = query.residues().chars().collect();
    let target_symbols: Vec<char> = target.residues().chars().collect();
    let rows = query_symbols.len() + 1;
    let cols = target_symbols.len() + 1;

    let mut m = vec![vec![0; cols]; rows];
    let mut x = vec![vec![NEG_INF; cols]; rows];
    let mut y = vec![vec![NEG_INF; cols]; rows];

    let mut trace_m = vec![vec![TraceState::Stop; cols]; rows];
    let mut trace_x = vec![vec![TraceState::Stop; cols]; rows];
    let mut trace_y = vec![vec![TraceState::Stop; cols]; rows];

    let mut best_score = 0;
    let mut best_i = 0usize;
    let mut best_j = 0usize;
    let mut best_state = TraceState::Stop;

    for i in 1..rows {
        for j in 1..cols {
            let substitution =
                scoring.substitution_score(query_symbols[i - 1], target_symbols[j - 1]);

            let mut best_prev_m = 0;
            let mut best_prev_state = TraceState::Stop;
            if m[i - 1][j - 1] > best_prev_m {
                best_prev_m = m[i - 1][j - 1];
                best_prev_state = TraceState::Match;
            }
            if x[i - 1][j - 1] > best_prev_m {
                best_prev_m = x[i - 1][j - 1];
                best_prev_state = TraceState::GapInTarget;
            }
            if y[i - 1][j - 1] > best_prev_m {
                best_prev_m = y[i - 1][j - 1];
                best_prev_state = TraceState::GapInQuery;
            }
            let candidate_m = best_prev_m + substitution;
            if candidate_m > 0 {
                m[i][j] = candidate_m;
                trace_m[i][j] = best_prev_state;
            }

            let open_x_from_m = m[i - 1][j] - scoring.gap_open;
            let extend_x = x[i - 1][j] - scoring.gap_extend;
            let open_x_from_y = y[i - 1][j] - scoring.gap_open;
            let mut best_x = 0;
            let mut state_x = TraceState::Stop;
            if open_x_from_m > best_x {
                best_x = open_x_from_m;
                state_x = TraceState::Match;
            }
            if extend_x > best_x {
                best_x = extend_x;
                state_x = TraceState::GapInTarget;
            }
            if open_x_from_y > best_x {
                best_x = open_x_from_y;
                state_x = TraceState::GapInQuery;
            }
            if best_x > 0 {
                x[i][j] = best_x;
                trace_x[i][j] = state_x;
            }

            let open_y_from_m = m[i][j - 1] - scoring.gap_open;
            let extend_y = y[i][j - 1] - scoring.gap_extend;
            let open_y_from_x = x[i][j - 1] - scoring.gap_open;
            let mut best_y = 0;
            let mut state_y = TraceState::Stop;
            if open_y_from_m > best_y {
                best_y = open_y_from_m;
                state_y = TraceState::Match;
            }
            if extend_y > best_y {
                best_y = extend_y;
                state_y = TraceState::GapInQuery;
            }
            if open_y_from_x > best_y {
                best_y = open_y_from_x;
                state_y = TraceState::GapInTarget;
            }
            if best_y > 0 {
                y[i][j] = best_y;
                trace_y[i][j] = state_y;
            }

            if m[i][j] > best_score {
                best_score = m[i][j];
                best_i = i;
                best_j = j;
                best_state = TraceState::Match;
            }
            if x[i][j] > best_score {
                best_score = x[i][j];
                best_i = i;
                best_j = j;
                best_state = TraceState::GapInTarget;
            }
            if y[i][j] > best_score {
                best_score = y[i][j];
                best_i = i;
                best_j = j;
                best_state = TraceState::GapInQuery;
            }
        }
    }

    if best_score <= 0 {
        return Err(LocalAlignmentError::NoPositiveAlignment);
    }

    let mut i = best_i;
    let mut j = best_j;
    let mut state = best_state;
    let mut aligned_query = String::new();
    let mut aligned_target = String::new();

    while state != TraceState::Stop && i > 0 && j > 0 {
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
            TraceState::Stop => break,
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
    let summary = LocalPairwiseAlignmentSummary {
        score: best_score,
        aligned_length,
        identity_count,
        identity_percent,
        query_gap_count: alignment.rows()[0].gap_count(),
        target_gap_count: alignment.rows()[1].gap_count(),
        mode: scoring.mode,
        query_start: i,
        query_end: best_i,
        target_start: j,
        target_end: best_j,
    };

    Ok(LocalPairwiseAlignmentResult { alignment, summary })
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
    use super::{AlignmentMode, LocalAlignmentError, LocalAlignmentScoring, local_align};
    use crate::{MoleculeKind, SequenceIdentifier, SequenceRecord};

    #[test]
    fn locally_aligns_internal_nucleotide_region() {
        let query = SequenceRecord::new(
            SequenceIdentifier::new("query").expect("identifier"),
            MoleculeKind::Dna,
            "TTACGTAA",
        )
        .expect("sequence");
        let target = SequenceRecord::new(
            SequenceIdentifier::new("target").expect("identifier"),
            MoleculeKind::Dna,
            "GGACGTA",
        )
        .expect("sequence");

        let result = local_align(&query, &target, LocalAlignmentScoring::nucleotide_default())
            .expect("alignment");
        assert_eq!(result.alignment.rows()[0].aligned(), "ACGTA");
        assert_eq!(result.alignment.rows()[1].aligned(), "ACGTA");
        assert_eq!(result.summary.score, 5);
        assert_eq!(result.summary.query_start, 2);
        assert_eq!(result.summary.query_end, 7);
        assert_eq!(result.summary.target_start, 2);
        assert_eq!(result.summary.target_end, 7);
    }

    #[test]
    fn locally_aligns_internal_protein_region() {
        let query = SequenceRecord::new(
            SequenceIdentifier::new("query").expect("identifier"),
            MoleculeKind::Protein,
            "XXMKTYY",
        )
        .expect("sequence");
        let target = SequenceRecord::new(
            SequenceIdentifier::new("target").expect("identifier"),
            MoleculeKind::Protein,
            "PPMKTPP",
        )
        .expect("sequence");

        let result = local_align(&query, &target, LocalAlignmentScoring::protein_default())
            .expect("alignment");
        assert_eq!(result.alignment.rows()[0].aligned(), "MKT");
        assert_eq!(result.alignment.rows()[1].aligned(), "MKT");
        assert_eq!(result.summary.mode, AlignmentMode::Protein);
    }

    #[test]
    fn rejects_non_positive_gap_penalties() {
        let query = SequenceRecord::new(
            SequenceIdentifier::new("query").expect("identifier"),
            MoleculeKind::Dna,
            "ACGT",
        )
        .expect("sequence");
        let target = SequenceRecord::new(
            SequenceIdentifier::new("target").expect("identifier"),
            MoleculeKind::Dna,
            "ACGT",
        )
        .expect("sequence");

        let error = local_align(
            &query,
            &target,
            LocalAlignmentScoring {
                gap_open: 0,
                ..LocalAlignmentScoring::nucleotide_default()
            },
        )
        .expect_err("zero gap-open should fail");
        assert!(matches!(
            error,
            LocalAlignmentError::InvalidGapPenalty { .. }
        ));
    }

    #[test]
    fn rejects_no_positive_alignment() {
        let query = SequenceRecord::new(
            SequenceIdentifier::new("query").expect("identifier"),
            MoleculeKind::Dna,
            "AAAA",
        )
        .expect("sequence");
        let target = SequenceRecord::new(
            SequenceIdentifier::new("target").expect("identifier"),
            MoleculeKind::Dna,
            "TTTT",
        )
        .expect("sequence");

        let error = local_align(&query, &target, LocalAlignmentScoring::nucleotide_default())
            .expect_err("completely mismatching input should fail");
        assert_eq!(error, LocalAlignmentError::NoPositiveAlignment);
    }
}

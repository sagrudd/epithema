//! Deterministic nucleotide linguistic-complexity helpers.

use std::collections::BTreeSet;

use crate::{MoleculeKind, SequenceRecord};

/// Strict complexity-calculation parameters.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ComplexityParameters {
    /// Inclusive minimum k-mer size.
    pub k_min: usize,
    /// Inclusive maximum k-mer size.
    pub k_max: usize,
}

/// Whole-sequence complexity summary.
#[derive(Clone, Debug, PartialEq)]
pub struct SequenceComplexity {
    /// Record identifier.
    pub record_id: String,
    /// Sequence length.
    pub sequence_length: usize,
    /// Inclusive minimum k-mer size used.
    pub k_min: usize,
    /// Inclusive maximum k-mer size used.
    pub k_max: usize,
    /// Complexity ratio.
    pub complexity: f64,
}

/// One sliding-window complexity row.
#[derive(Clone, Debug, PartialEq)]
pub struct WindowComplexity {
    /// Record identifier.
    pub record_id: String,
    /// Zero-based window start.
    pub start: usize,
    /// Zero-based half-open window end.
    pub end: usize,
    /// Window length.
    pub window_length: usize,
    /// Inclusive minimum k-mer size used.
    pub k_min: usize,
    /// Inclusive maximum k-mer size used.
    pub k_max: usize,
    /// Complexity ratio.
    pub complexity: f64,
}

/// Complexity-analysis errors.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ComplexityError {
    /// Only canonical nucleotide inputs are supported in v1.
    NonCanonicalNucleotideInput {
        /// Record identifier.
        identifier: String,
        /// Molecule kind supplied by the record.
        molecule: MoleculeKind,
    },
    /// The sequence contains unsupported symbols for the strict A/C/G/T policy.
    UnsupportedSymbol {
        /// Record identifier.
        identifier: String,
        /// Unsupported symbol.
        symbol: char,
        /// Zero-based position.
        position: usize,
    },
    /// Invalid k-mer bounds were requested.
    InvalidKRange {
        /// Requested k-min.
        k_min: usize,
        /// Requested k-max.
        k_max: usize,
        /// Available sequence or window length.
        length: usize,
    },
    /// Invalid window configuration.
    InvalidWindow {
        /// Requested window length.
        window: usize,
        /// Requested step size.
        step: usize,
    },
    /// The sequence is shorter than the requested window length.
    WindowLongerThanSequence {
        /// Record identifier.
        identifier: String,
        /// Sequence length.
        sequence_length: usize,
        /// Window length.
        window: usize,
    },
}

impl std::fmt::Display for ComplexityError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NonCanonicalNucleotideInput {
                identifier,
                molecule,
            } => write!(
                f,
                "complex requires canonical nucleotide sequences in v1: '{identifier}' is classified as {molecule}"
            ),
            Self::UnsupportedSymbol {
                identifier,
                symbol,
                position,
            } => write!(
                f,
                "complex only supports canonical A/C/G/T residues in v1: '{identifier}' contains '{symbol}' at position {}",
                position + 1
            ),
            Self::InvalidKRange {
                k_min,
                k_max,
                length,
            } => write!(
                f,
                "invalid k-mer range for complexity calculation: k_min={k_min}, k_max={k_max}, available_length={length}"
            ),
            Self::InvalidWindow { window, step } => write!(
                f,
                "invalid sliding-window parameters for complexity calculation: window={window}, step={step}"
            ),
            Self::WindowLongerThanSequence {
                identifier,
                sequence_length,
                window,
            } => write!(
                f,
                "complex window length {window} exceeds sequence length {sequence_length} for '{identifier}'"
            ),
        }
    }
}

impl std::error::Error for ComplexityError {}

impl ComplexityParameters {
    /// Creates validated complexity parameters for a specific sequence or window length.
    pub fn validate(self, length: usize) -> Result<Self, ComplexityError> {
        if self.k_min == 0 || self.k_max == 0 || self.k_min > self.k_max || self.k_max > length {
            return Err(ComplexityError::InvalidKRange {
                k_min: self.k_min,
                k_max: self.k_max,
                length,
            });
        }
        Ok(self)
    }
}

/// Computes whole-sequence linguistic complexity for one record.
pub fn sequence_complexity(
    record: &SequenceRecord,
    parameters: ComplexityParameters,
) -> Result<SequenceComplexity, ComplexityError> {
    ensure_canonical_dna(record)?;
    let parameters = parameters.validate(record.len())?;
    let complexity = complexity_for_sequence(record.residues(), parameters);

    Ok(SequenceComplexity {
        record_id: record.identifier().accession().to_owned(),
        sequence_length: record.len(),
        k_min: parameters.k_min,
        k_max: parameters.k_max,
        complexity,
    })
}

/// Computes sliding-window linguistic complexity rows for one record.
pub fn sliding_window_complexity(
    record: &SequenceRecord,
    window: usize,
    step: usize,
    parameters: ComplexityParameters,
) -> Result<Vec<WindowComplexity>, ComplexityError> {
    ensure_canonical_dna(record)?;
    if window == 0 || step == 0 {
        return Err(ComplexityError::InvalidWindow { window, step });
    }
    if record.len() < window {
        return Err(ComplexityError::WindowLongerThanSequence {
            identifier: record.identifier().accession().to_owned(),
            sequence_length: record.len(),
            window,
        });
    }
    let parameters = parameters.validate(window)?;
    let residues = record.residues();
    let mut rows = Vec::new();
    let mut start = 0usize;
    while start + window <= record.len() {
        let end = start + window;
        rows.push(WindowComplexity {
            record_id: record.identifier().accession().to_owned(),
            start,
            end,
            window_length: window,
            k_min: parameters.k_min,
            k_max: parameters.k_max,
            complexity: complexity_for_sequence(&residues[start..end], parameters),
        });
        start += step;
    }
    Ok(rows)
}

fn ensure_canonical_dna(record: &SequenceRecord) -> Result<(), ComplexityError> {
    if !record.molecule().is_nucleotide() && record.molecule() != MoleculeKind::Unknown {
        return Err(ComplexityError::NonCanonicalNucleotideInput {
            identifier: record.identifier().accession().to_owned(),
            molecule: record.molecule(),
        });
    }

    for (position, symbol) in record.residues().chars().enumerate() {
        if !matches!(symbol, 'A' | 'C' | 'G' | 'T') {
            return Err(ComplexityError::UnsupportedSymbol {
                identifier: record.identifier().accession().to_owned(),
                symbol,
                position,
            });
        }
    }
    Ok(())
}

fn complexity_for_sequence(residues: &str, parameters: ComplexityParameters) -> f64 {
    let mut observed_total = 0usize;
    let mut possible_total = 0usize;
    for k in parameters.k_min..=parameters.k_max {
        let observed = distinct_kmers(residues, k);
        let possible = possible_distinct_kmers(residues.len(), k);
        observed_total += observed;
        possible_total += possible;
    }

    if possible_total == 0 {
        0.0
    } else {
        observed_total as f64 / possible_total as f64
    }
}

fn distinct_kmers(residues: &str, k: usize) -> usize {
    let mut seen = BTreeSet::new();
    for start in 0..=residues.len() - k {
        seen.insert(&residues[start..start + k]);
    }
    seen.len()
}

fn possible_distinct_kmers(length: usize, k: usize) -> usize {
    4usize.pow(k as u32).min(length - k + 1)
}

#[cfg(test)]
mod tests {
    use crate::{SequenceIdentifier, SequenceRecord};

    use super::{
        ComplexityError, ComplexityParameters, sequence_complexity, sliding_window_complexity,
    };

    fn dna(id: &str, residues: &str) -> SequenceRecord {
        SequenceRecord::new(
            SequenceIdentifier::new(id).expect("valid identifier"),
            crate::MoleculeKind::Dna,
            residues,
        )
        .expect("valid sequence")
    }

    #[test]
    fn scores_low_and_high_complexity_sequences() {
        let low = sequence_complexity(
            &dna("low", "AAAAAA"),
            ComplexityParameters { k_min: 1, k_max: 2 },
        )
        .expect("low complexity should compute");
        let high = sequence_complexity(
            &dna("high", "ACGTAC"),
            ComplexityParameters { k_min: 1, k_max: 2 },
        )
        .expect("high complexity should compute");

        assert!(low.complexity < high.complexity);
        assert!((low.complexity - (2.0 / 9.0)).abs() < 1e-9);
        assert!((high.complexity - (8.0 / 9.0)).abs() < 1e-9);
    }

    #[test]
    fn computes_sliding_window_rows() {
        let rows = sliding_window_complexity(
            &dna("demo", "ACGTAC"),
            4,
            2,
            ComplexityParameters { k_min: 1, k_max: 2 },
        )
        .expect("windowed complexity should compute");

        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0].start, 0);
        assert_eq!(rows[0].end, 4);
        assert_eq!(rows[1].start, 2);
        assert_eq!(rows[1].end, 6);
    }

    #[test]
    fn rejects_unsupported_symbols() {
        let error = sequence_complexity(
            &dna("ambig", "ACNT"),
            ComplexityParameters { k_min: 1, k_max: 2 },
        )
        .expect_err("ambiguous symbols should be rejected");

        assert!(matches!(error, ComplexityError::UnsupportedSymbol { .. }));
    }
}

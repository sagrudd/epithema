//! Sliding-window bounded nucleotide isochore profiles for `isochore`.

use crate::sequence::SequenceRecord;

/// Errors for `isochore` profile computation.
#[derive(Clone, Debug, PartialEq)]
pub enum NucleotideIsochoreError {
    /// The input sequence is not nucleotide-like.
    NonNucleotideSequence,
    /// The requested window length is invalid.
    InvalidWindow {
        /// Requested window length.
        window: usize,
    },
    /// The requested step size is invalid.
    InvalidStep {
        /// Requested step size.
        step: usize,
    },
    /// The sequence is shorter than the requested window.
    SequenceShorterThanWindow {
        /// Input sequence length.
        sequence_length: usize,
        /// Requested window length.
        window: usize,
    },
}

impl std::fmt::Display for NucleotideIsochoreError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NonNucleotideSequence => {
                write!(f, "isochore requires a nucleotide sequence input")
            }
            Self::InvalidWindow { window } => {
                write!(f, "window length must be >= 1, got {window}")
            }
            Self::InvalidStep { step } => write!(f, "step size must be >= 1, got {step}"),
            Self::SequenceShorterThanWindow {
                sequence_length,
                window,
            } => write!(
                f,
                "sequence length {sequence_length} is shorter than requested window {window}"
            ),
        }
    }
}

impl std::error::Error for NucleotideIsochoreError {}

/// Bounded GC-content band derived from one analytical window.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum IsochoreBand {
    /// GC-poorest bounded band.
    L1,
    /// Low-GC bounded band.
    L2,
    /// Moderately GC-rich bounded band.
    H1,
    /// High-GC bounded band.
    H2,
    /// GC-richest bounded band.
    H3,
}

/// One sliding-window bounded `isochore` row.
#[derive(Clone, Debug, PartialEq)]
pub struct IsochoreWindow {
    /// One-based inclusive window start.
    pub window_start: usize,
    /// One-based inclusive window end.
    pub window_end: usize,
    /// Window length in residues.
    pub window_length: usize,
    /// Canonical A/C/G/T/U symbols contributing to the fractions.
    pub canonical_symbols: usize,
    /// Non-gap ambiguous nucleotide symbols.
    pub ambiguous_symbols: usize,
    /// Gap or terminator symbols ignored by the bounded v1 model.
    pub ignored_gap_symbols: usize,
    /// Fraction of canonical A plus T/U residues in the window.
    pub at_fraction: f64,
    /// Fraction of canonical G plus C residues in the window.
    pub gc_fraction: f64,
    /// GC percentage derived from canonical symbols only.
    pub gc_percent: f64,
    /// Bounded GC-content band derived from `gc_fraction`.
    pub isochore_band: IsochoreBand,
}

/// Full bounded `isochore` profile for one nucleotide sequence.
#[derive(Clone, Debug, PartialEq)]
pub struct NucleotideIsochoreProfile {
    /// Stable sequence identifier.
    pub identifier: String,
    /// Sequence length.
    pub sequence_length: usize,
    /// Window length.
    pub window: usize,
    /// Step size.
    pub step: usize,
    /// Sliding-window rows.
    pub windows: Vec<IsochoreWindow>,
}

/// Computes a deterministic sliding-window bounded `isochore` profile.
pub fn nucleotide_isochore_profile(
    record: &SequenceRecord,
    window: usize,
    step: usize,
) -> Result<NucleotideIsochoreProfile, NucleotideIsochoreError> {
    if !record.molecule().is_nucleotide() || !record.alphabet().is_nucleotide() {
        return Err(NucleotideIsochoreError::NonNucleotideSequence);
    }
    if window == 0 {
        return Err(NucleotideIsochoreError::InvalidWindow { window });
    }
    if step == 0 {
        return Err(NucleotideIsochoreError::InvalidStep { step });
    }
    if record.len() < window {
        return Err(NucleotideIsochoreError::SequenceShorterThanWindow {
            sequence_length: record.len(),
            window,
        });
    }

    let residues: Vec<char> = record.residues().chars().collect();
    let mut windows = Vec::new();
    let last_start = record.len() - window;
    let mut start = 0usize;
    while start <= last_start {
        let slice = &residues[start..start + window];
        let counts = window_counts(slice);
        let denominator = counts.canonical_symbols as f64;
        let at_fraction = fraction(
            counts.adenine_symbols + counts.thymine_or_uracil_symbols,
            denominator,
        );
        let gc_fraction = fraction(counts.guanine_symbols + counts.cytosine_symbols, denominator);
        windows.push(IsochoreWindow {
            window_start: start + 1,
            window_end: start + window,
            window_length: window,
            canonical_symbols: counts.canonical_symbols,
            ambiguous_symbols: counts.ambiguous_symbols,
            ignored_gap_symbols: counts.ignored_gap_symbols,
            at_fraction,
            gc_fraction,
            gc_percent: gc_fraction * 100.0,
            isochore_band: band_for_gc_fraction(gc_fraction),
        });
        start += step;
    }

    Ok(NucleotideIsochoreProfile {
        identifier: record.identifier().accession().to_owned(),
        sequence_length: record.len(),
        window,
        step,
        windows,
    })
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
struct WindowCounts {
    canonical_symbols: usize,
    ambiguous_symbols: usize,
    ignored_gap_symbols: usize,
    adenine_symbols: usize,
    cytosine_symbols: usize,
    guanine_symbols: usize,
    thymine_or_uracil_symbols: usize,
}

fn window_counts(residues: &[char]) -> WindowCounts {
    let mut counts = WindowCounts::default();
    for residue in residues.iter().copied() {
        match residue {
            'A' => {
                counts.canonical_symbols += 1;
                counts.adenine_symbols += 1;
            }
            'C' => {
                counts.canonical_symbols += 1;
                counts.cytosine_symbols += 1;
            }
            'G' => {
                counts.canonical_symbols += 1;
                counts.guanine_symbols += 1;
            }
            'T' | 'U' => {
                counts.canonical_symbols += 1;
                counts.thymine_or_uracil_symbols += 1;
            }
            '-' | '*' => {
                counts.ignored_gap_symbols += 1;
            }
            _ => {
                counts.ambiguous_symbols += 1;
            }
        }
    }
    counts
}

fn fraction(count: usize, denominator: f64) -> f64 {
    if denominator == 0.0 {
        0.0
    } else {
        count as f64 / denominator
    }
}

fn band_for_gc_fraction(gc_fraction: f64) -> IsochoreBand {
    if gc_fraction < 0.37 {
        IsochoreBand::L1
    } else if gc_fraction < 0.41 {
        IsochoreBand::L2
    } else if gc_fraction < 0.46 {
        IsochoreBand::H1
    } else if gc_fraction < 0.53 {
        IsochoreBand::H2
    } else {
        IsochoreBand::H3
    }
}

#[cfg(test)]
mod tests {
    use crate::{MoleculeKind, SequenceIdentifier, SequenceRecord};

    use super::{IsochoreBand, NucleotideIsochoreError, nucleotide_isochore_profile};

    #[test]
    fn computes_expected_isochore_profile_for_dna() {
        let record = SequenceRecord::new(
            SequenceIdentifier::new("isochore1").expect("valid identifier"),
            MoleculeKind::Dna,
            "AAAAGGGGCCCC",
        )
        .expect("dna sequence should build");

        let profile = nucleotide_isochore_profile(&record, 4, 4).expect("profile should compute");
        assert_eq!(profile.windows.len(), 3);

        let first = &profile.windows[0];
        assert_eq!(first.window_start, 1);
        assert_eq!(first.window_end, 4);
        assert_eq!(first.canonical_symbols, 4);
        assert!((first.at_fraction - 1.0).abs() < 1e-12);
        assert!((first.gc_fraction - 0.0).abs() < 1e-12);
        assert!((first.gc_percent - 0.0).abs() < 1e-12);
        assert_eq!(first.isochore_band, IsochoreBand::L1);

        let second = &profile.windows[1];
        assert_eq!(second.window_start, 5);
        assert_eq!(second.window_end, 8);
        assert!((second.at_fraction - 0.0).abs() < 1e-12);
        assert!((second.gc_fraction - 1.0).abs() < 1e-12);
        assert!((second.gc_percent - 100.0).abs() < 1e-12);
        assert_eq!(second.isochore_band, IsochoreBand::H3);
    }

    #[test]
    fn classifies_boundary_gc_bands_stably() {
        let record = SequenceRecord::new(
            SequenceIdentifier::new("isochore2").expect("valid identifier"),
            MoleculeKind::Dna,
            "AAACGGGGAA",
        )
        .expect("dna sequence should build");

        let profile = nucleotide_isochore_profile(&record, 10, 1).expect("profile should compute");
        let only = &profile.windows[0];
        assert!((only.gc_fraction - 0.5).abs() < 1e-12);
        assert_eq!(only.isochore_band, IsochoreBand::H2);
    }

    #[test]
    fn handles_ambiguous_and_gap_symbols_honestly() {
        let record = SequenceRecord::new(
            SequenceIdentifier::new("isochore3").expect("valid identifier"),
            MoleculeKind::Rna,
            "GGUN-*",
        )
        .expect("rna sequence should build");

        let profile = nucleotide_isochore_profile(&record, 6, 1).expect("profile should compute");
        let only = &profile.windows[0];
        assert_eq!(only.canonical_symbols, 3);
        assert_eq!(only.ambiguous_symbols, 1);
        assert_eq!(only.ignored_gap_symbols, 2);
        assert!((only.gc_fraction - (2.0 / 3.0)).abs() < 1e-12);
        assert!((only.at_fraction - (1.0 / 3.0)).abs() < 1e-12);
        assert_eq!(only.isochore_band, IsochoreBand::H3);
    }

    #[test]
    fn rejects_non_nucleotide_input() {
        let record = SequenceRecord::new(
            SequenceIdentifier::new("isochore4").expect("valid identifier"),
            MoleculeKind::Protein,
            "MSTNPKPQR",
        )
        .expect("protein sequence should build");

        let error =
            nucleotide_isochore_profile(&record, 4, 1).expect_err("protein input should fail");
        assert_eq!(error, NucleotideIsochoreError::NonNucleotideSequence);
    }

    #[test]
    fn rejects_short_sequences() {
        let record = SequenceRecord::new(
            SequenceIdentifier::new("isochore5").expect("valid identifier"),
            MoleculeKind::Dna,
            "ACGT",
        )
        .expect("dna sequence should build");

        let error =
            nucleotide_isochore_profile(&record, 5, 1).expect_err("short sequence should fail");
        assert_eq!(
            error,
            NucleotideIsochoreError::SequenceShorterThanWindow {
                sequence_length: 4,
                window: 5,
            }
        );
    }
}

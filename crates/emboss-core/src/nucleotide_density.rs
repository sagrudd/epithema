//! Sliding-window bounded nucleotide-density profiles for `density`.

use crate::sequence::SequenceRecord;

/// Errors for `density` profile computation.
#[derive(Clone, Debug, PartialEq)]
pub enum NucleotideDensityError {
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

impl std::fmt::Display for NucleotideDensityError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NonNucleotideSequence => {
                write!(f, "density requires a nucleotide sequence input")
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

impl std::error::Error for NucleotideDensityError {}

/// One sliding-window nucleotide-density row.
#[derive(Clone, Debug, PartialEq)]
pub struct DensityWindow {
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
    /// Fraction of canonical A residues in the window.
    pub adenine_fraction: f64,
    /// Fraction of canonical C residues in the window.
    pub cytosine_fraction: f64,
    /// Fraction of canonical G residues in the window.
    pub guanine_fraction: f64,
    /// Fraction of canonical T/U residues in the window.
    pub thymine_or_uracil_fraction: f64,
    /// Fraction of canonical A plus T/U residues in the window.
    pub at_fraction: f64,
    /// Fraction of canonical G plus C residues in the window.
    pub gc_fraction: f64,
}

/// Full bounded `density` profile for one nucleotide sequence.
#[derive(Clone, Debug, PartialEq)]
pub struct NucleotideDensityProfile {
    /// Stable sequence identifier.
    pub identifier: String,
    /// Sequence length.
    pub sequence_length: usize,
    /// Window length.
    pub window: usize,
    /// Step size.
    pub step: usize,
    /// Sliding-window rows.
    pub windows: Vec<DensityWindow>,
}

/// Computes a deterministic sliding-window bounded nucleotide-density profile.
pub fn nucleotide_density_profile(
    record: &SequenceRecord,
    window: usize,
    step: usize,
) -> Result<NucleotideDensityProfile, NucleotideDensityError> {
    if !record.molecule().is_nucleotide() || !record.alphabet().is_nucleotide() {
        return Err(NucleotideDensityError::NonNucleotideSequence);
    }
    if window == 0 {
        return Err(NucleotideDensityError::InvalidWindow { window });
    }
    if step == 0 {
        return Err(NucleotideDensityError::InvalidStep { step });
    }
    if record.len() < window {
        return Err(NucleotideDensityError::SequenceShorterThanWindow {
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
        let counts = density_counts(slice);
        let denominator = counts.canonical_symbols as f64;
        let adenine_fraction = fraction(counts.adenine_symbols, denominator);
        let cytosine_fraction = fraction(counts.cytosine_symbols, denominator);
        let guanine_fraction = fraction(counts.guanine_symbols, denominator);
        let thymine_or_uracil_fraction = fraction(counts.thymine_or_uracil_symbols, denominator);
        windows.push(DensityWindow {
            window_start: start + 1,
            window_end: start + window,
            window_length: window,
            canonical_symbols: counts.canonical_symbols,
            ambiguous_symbols: counts.ambiguous_symbols,
            ignored_gap_symbols: counts.ignored_gap_symbols,
            adenine_fraction,
            cytosine_fraction,
            guanine_fraction,
            thymine_or_uracil_fraction,
            at_fraction: adenine_fraction + thymine_or_uracil_fraction,
            gc_fraction: cytosine_fraction + guanine_fraction,
        });
        start += step;
    }

    Ok(NucleotideDensityProfile {
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

fn density_counts(residues: &[char]) -> WindowCounts {
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

#[cfg(test)]
mod tests {
    use crate::{MoleculeKind, SequenceIdentifier, SequenceRecord};

    use super::{NucleotideDensityError, nucleotide_density_profile};

    #[test]
    fn computes_expected_density_profile_for_dna() {
        let record = SequenceRecord::new(
            SequenceIdentifier::new("density1").expect("valid identifier"),
            MoleculeKind::Dna,
            "AACGT",
        )
        .expect("dna sequence should build");

        let profile = nucleotide_density_profile(&record, 4, 1).expect("profile should compute");
        assert_eq!(profile.windows.len(), 2);

        let first = &profile.windows[0];
        assert_eq!(first.window_start, 1);
        assert_eq!(first.window_end, 4);
        assert_eq!(first.canonical_symbols, 4);
        assert_eq!(first.ambiguous_symbols, 0);
        assert!((first.adenine_fraction - 0.5).abs() < 1e-12);
        assert!((first.cytosine_fraction - 0.25).abs() < 1e-12);
        assert!((first.guanine_fraction - 0.25).abs() < 1e-12);
        assert!((first.thymine_or_uracil_fraction - 0.0).abs() < 1e-12);
        assert!((first.at_fraction - 0.5).abs() < 1e-12);
        assert!((first.gc_fraction - 0.5).abs() < 1e-12);

        let second = &profile.windows[1];
        assert_eq!(second.window_start, 2);
        assert_eq!(second.window_end, 5);
        assert!((second.adenine_fraction - 0.25).abs() < 1e-12);
        assert!((second.cytosine_fraction - 0.25).abs() < 1e-12);
        assert!((second.guanine_fraction - 0.25).abs() < 1e-12);
        assert!((second.thymine_or_uracil_fraction - 0.25).abs() < 1e-12);
        assert!((second.at_fraction - 0.5).abs() < 1e-12);
        assert!((second.gc_fraction - 0.5).abs() < 1e-12);
    }

    #[test]
    fn handles_ambiguous_and_gap_symbols_honestly() {
        let record = SequenceRecord::new(
            SequenceIdentifier::new("density2").expect("valid identifier"),
            MoleculeKind::Dna,
            "CGTN-",
        )
        .expect("dna sequence should build");

        let profile = nucleotide_density_profile(&record, 4, 1).expect("profile should compute");
        let first = &profile.windows[0];
        assert_eq!(first.canonical_symbols, 3);
        assert_eq!(first.ambiguous_symbols, 1);
        assert_eq!(first.ignored_gap_symbols, 0);
        assert!((first.cytosine_fraction - (1.0 / 3.0)).abs() < 1e-12);
        assert!((first.guanine_fraction - (1.0 / 3.0)).abs() < 1e-12);
        assert!((first.thymine_or_uracil_fraction - (1.0 / 3.0)).abs() < 1e-12);

        let second = &profile.windows[1];
        assert_eq!(second.canonical_symbols, 2);
        assert_eq!(second.ambiguous_symbols, 1);
        assert_eq!(second.ignored_gap_symbols, 1);
        assert!((second.at_fraction - 0.5).abs() < 1e-12);
        assert!((second.gc_fraction - 0.5).abs() < 1e-12);
    }

    #[test]
    fn supports_rna_windows() {
        let record = SequenceRecord::new(
            SequenceIdentifier::new("density3").expect("valid identifier"),
            MoleculeKind::Rna,
            "AACGU",
        )
        .expect("rna sequence should build");

        let profile = nucleotide_density_profile(&record, 5, 1).expect("profile should compute");
        let first = &profile.windows[0];
        assert!((first.thymine_or_uracil_fraction - 0.2).abs() < 1e-12);
        assert!((first.at_fraction - 0.6).abs() < 1e-12);
    }

    #[test]
    fn rejects_non_nucleotide_sequences() {
        let record = SequenceRecord::new(
            SequenceIdentifier::new("density4").expect("valid identifier"),
            MoleculeKind::Protein,
            "MSTN",
        )
        .expect("protein sequence should build");

        assert_eq!(
            nucleotide_density_profile(&record, 2, 1),
            Err(NucleotideDensityError::NonNucleotideSequence)
        );
    }

    #[test]
    fn rejects_short_sequences() {
        let record = SequenceRecord::new(
            SequenceIdentifier::new("density5").expect("valid identifier"),
            MoleculeKind::Dna,
            "ACG",
        )
        .expect("dna sequence should build");

        assert!(matches!(
            nucleotide_density_profile(&record, 4, 1),
            Err(NucleotideDensityError::SequenceShorterThanWindow { .. })
        ));
    }
}

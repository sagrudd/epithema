//! Bounded third-base-position variability profiles for `wobble`.

use crate::codon_usage::{CodonUsageError, summarize_coding_sequence};
use crate::sequence::SequenceRecord;

/// Errors for `wobble` profile computation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum NucleotideWobbleError {
    /// The input sequence is not nucleotide-like.
    NonNucleotideSequence,
    /// The coding sequence does not satisfy the bounded v1 wobble model.
    InvalidCodingSequence(CodonUsageError),
    /// The requested codon-window length is invalid.
    InvalidCodonWindow {
        /// Requested codon-window length.
        codon_window: usize,
    },
    /// The requested codon-step size is invalid.
    InvalidCodonStep {
        /// Requested codon-step size.
        codon_step: usize,
    },
    /// The sense-codon span is shorter than the requested codon window.
    SequenceShorterThanWindow {
        /// Count of sense codons available for profiling.
        sense_codon_count: usize,
        /// Requested codon-window length.
        codon_window: usize,
    },
}

impl std::fmt::Display for NucleotideWobbleError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NonNucleotideSequence => {
                write!(f, "wobble requires a nucleotide coding-sequence input")
            }
            Self::InvalidCodingSequence(error) => write!(f, "{error}"),
            Self::InvalidCodonWindow { codon_window } => {
                write!(f, "codon window length must be >= 1, got {codon_window}")
            }
            Self::InvalidCodonStep { codon_step } => {
                write!(f, "codon step size must be >= 1, got {codon_step}")
            }
            Self::SequenceShorterThanWindow {
                sense_codon_count,
                codon_window,
            } => write!(
                f,
                "sense codon count {sense_codon_count} is shorter than requested codon window {codon_window}"
            ),
        }
    }
}

impl std::error::Error for NucleotideWobbleError {}

/// One bounded `wobble` analytical row.
#[derive(Clone, Debug, PartialEq)]
pub struct WobbleWindow {
    /// One-based inclusive nucleotide start for the window.
    pub window_start: usize,
    /// One-based inclusive nucleotide end for the window.
    pub window_end: usize,
    /// Window length in nucleotides.
    pub window_length: usize,
    /// Window length in codons.
    pub codon_window_length: usize,
    /// Count of wobble positions contributing to the row.
    pub wobble_positions: usize,
    /// Fraction of wobble bases that are A.
    pub adenine_fraction: f64,
    /// Fraction of wobble bases that are C.
    pub cytosine_fraction: f64,
    /// Fraction of wobble bases that are G.
    pub guanine_fraction: f64,
    /// Fraction of wobble bases that are T.
    pub thymine_fraction: f64,
    /// Fraction contributed by the dominant wobble base in the window.
    pub dominant_wobble_fraction: f64,
    /// Bounded variability score, defined as `1 - dominant_wobble_fraction`.
    pub wobble_variability: f64,
}

/// Full bounded `wobble` profile for one coding nucleotide sequence.
#[derive(Clone, Debug, PartialEq)]
pub struct NucleotideWobbleProfile {
    /// Stable sequence identifier.
    pub identifier: String,
    /// Sequence length in nucleotides.
    pub sequence_length: usize,
    /// Codon-window length.
    pub codon_window: usize,
    /// Codon-step size.
    pub codon_step: usize,
    /// Sliding-window analytical rows.
    pub windows: Vec<WobbleWindow>,
}

/// Computes a deterministic codon-windowed bounded `wobble` profile.
pub fn nucleotide_wobble_profile(
    record: &SequenceRecord,
    codon_window: usize,
    codon_step: usize,
) -> Result<NucleotideWobbleProfile, NucleotideWobbleError> {
    if !record.molecule().is_nucleotide() || !record.alphabet().is_nucleotide() {
        return Err(NucleotideWobbleError::NonNucleotideSequence);
    }
    if codon_window == 0 {
        return Err(NucleotideWobbleError::InvalidCodonWindow { codon_window });
    }
    if codon_step == 0 {
        return Err(NucleotideWobbleError::InvalidCodonStep { codon_step });
    }

    let coding_summary = summarize_coding_sequence(record.residues())
        .map_err(NucleotideWobbleError::InvalidCodingSequence)?;
    if coding_summary.sense_codon_count < codon_window {
        return Err(NucleotideWobbleError::SequenceShorterThanWindow {
            sense_codon_count: coding_summary.sense_codon_count,
            codon_window,
        });
    }

    let codons =
        normalized_sense_codons(record.residues(), coding_summary.terminal_stop.as_deref());
    let last_start = codons.len() - codon_window;
    let mut start = 0usize;
    let mut windows = Vec::new();
    while start <= last_start {
        let slice = &codons[start..start + codon_window];
        let counts = wobble_counts(slice);
        let dominant_wobble_fraction = counts.dominant_fraction();
        windows.push(WobbleWindow {
            window_start: start * 3 + 1,
            window_end: (start + codon_window) * 3,
            window_length: codon_window * 3,
            codon_window_length: codon_window,
            wobble_positions: codon_window,
            adenine_fraction: counts.adenine_fraction(),
            cytosine_fraction: counts.cytosine_fraction(),
            guanine_fraction: counts.guanine_fraction(),
            thymine_fraction: counts.thymine_fraction(),
            dominant_wobble_fraction,
            wobble_variability: 1.0 - dominant_wobble_fraction,
        });
        start += codon_step;
    }

    Ok(NucleotideWobbleProfile {
        identifier: record.identifier().accession().to_owned(),
        sequence_length: record.len(),
        codon_window,
        codon_step,
        windows,
    })
}

fn normalized_sense_codons(sequence: &str, terminal_stop: Option<&str>) -> Vec<String> {
    let mut codons = sequence
        .as_bytes()
        .chunks(3)
        .map(|chunk| {
            std::str::from_utf8(chunk)
                .expect("sequence records are normalized ASCII residues")
                .to_ascii_uppercase()
                .replace('U', "T")
        })
        .collect::<Vec<_>>();
    if terminal_stop.is_some() {
        codons.pop();
    }
    codons
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
struct WobbleCounts {
    adenine: usize,
    cytosine: usize,
    guanine: usize,
    thymine: usize,
}

impl WobbleCounts {
    fn dominant_fraction(self) -> f64 {
        let dominant = self
            .adenine
            .max(self.cytosine)
            .max(self.guanine)
            .max(self.thymine);
        dominant as f64 / self.total() as f64
    }

    fn adenine_fraction(self) -> f64 {
        self.adenine as f64 / self.total() as f64
    }

    fn cytosine_fraction(self) -> f64 {
        self.cytosine as f64 / self.total() as f64
    }

    fn guanine_fraction(self) -> f64 {
        self.guanine as f64 / self.total() as f64
    }

    fn thymine_fraction(self) -> f64 {
        self.thymine as f64 / self.total() as f64
    }

    fn total(self) -> usize {
        self.adenine + self.cytosine + self.guanine + self.thymine
    }
}

fn wobble_counts(codons: &[String]) -> WobbleCounts {
    let mut counts = WobbleCounts::default();
    for codon in codons {
        let wobble_base = codon
            .as_bytes()
            .get(2)
            .copied()
            .expect("validated coding sequence codons are complete triplets")
            as char;
        match wobble_base {
            'A' => counts.adenine += 1,
            'C' => counts.cytosine += 1,
            'G' => counts.guanine += 1,
            'T' => counts.thymine += 1,
            _ => unreachable!("validated wobble base should be canonical"),
        }
    }
    counts
}

#[cfg(test)]
mod tests {
    use crate::{CodonUsageError, MoleculeKind, SequenceIdentifier, SequenceRecord};

    use super::{NucleotideWobbleError, nucleotide_wobble_profile};

    #[test]
    fn computes_expected_wobble_profile_for_coding_dna() {
        let record = SequenceRecord::new(
            SequenceIdentifier::new("wobble1").expect("valid identifier"),
            MoleculeKind::Dna,
            "GCTGCCGCA",
        )
        .expect("dna sequence should build");

        let profile =
            nucleotide_wobble_profile(&record, 2, 1).expect("profile should compute cleanly");
        assert_eq!(profile.windows.len(), 2);

        let first = &profile.windows[0];
        assert_eq!(first.window_start, 1);
        assert_eq!(first.window_end, 6);
        assert_eq!(first.window_length, 6);
        assert_eq!(first.codon_window_length, 2);
        assert_eq!(first.wobble_positions, 2);
        assert!((first.adenine_fraction - 0.0).abs() < 1e-12);
        assert!((first.cytosine_fraction - 0.5).abs() < 1e-12);
        assert!((first.guanine_fraction - 0.0).abs() < 1e-12);
        assert!((first.thymine_fraction - 0.5).abs() < 1e-12);
        assert!((first.dominant_wobble_fraction - 0.5).abs() < 1e-12);
        assert!((first.wobble_variability - 0.5).abs() < 1e-12);

        let second = &profile.windows[1];
        assert_eq!(second.window_start, 4);
        assert_eq!(second.window_end, 9);
        assert!((second.adenine_fraction - 0.5).abs() < 1e-12);
        assert!((second.cytosine_fraction - 0.5).abs() < 1e-12);
        assert!((second.guanine_fraction - 0.0).abs() < 1e-12);
        assert!((second.thymine_fraction - 0.0).abs() < 1e-12);
        assert!((second.dominant_wobble_fraction - 0.5).abs() < 1e-12);
        assert!((second.wobble_variability - 0.5).abs() < 1e-12);
    }

    #[test]
    fn accepts_rna_and_ignores_terminal_stop() {
        let record = SequenceRecord::new(
            SequenceIdentifier::new("wobble2").expect("valid identifier"),
            MoleculeKind::Rna,
            "GCUGCCUAA",
        )
        .expect("rna sequence should build");

        let profile =
            nucleotide_wobble_profile(&record, 2, 1).expect("profile should compute cleanly");
        assert_eq!(profile.windows.len(), 1);
        let only = &profile.windows[0];
        assert_eq!(only.window_start, 1);
        assert_eq!(only.window_end, 6);
        assert!((only.cytosine_fraction - 0.5).abs() < 1e-12);
        assert!((only.thymine_fraction - 0.5).abs() < 1e-12);
    }

    #[test]
    fn rejects_non_coding_length() {
        let record = SequenceRecord::new(
            SequenceIdentifier::new("wobble3").expect("valid identifier"),
            MoleculeKind::Dna,
            "ATGGC",
        )
        .expect("dna sequence should build");

        let error = nucleotide_wobble_profile(&record, 2, 1)
            .expect_err("non-coding length should be rejected");
        assert_eq!(
            error,
            NucleotideWobbleError::InvalidCodingSequence(CodonUsageError::NonCodingLength {
                length: 5,
            })
        );
    }

    #[test]
    fn rejects_ambiguous_coding_sequence() {
        let record = SequenceRecord::new(
            SequenceIdentifier::new("wobble4").expect("valid identifier"),
            MoleculeKind::Dna,
            "ATGNCC",
        )
        .expect("dna sequence should build");

        let error = nucleotide_wobble_profile(&record, 2, 1)
            .expect_err("ambiguous coding sequence should be rejected");
        assert_eq!(
            error,
            NucleotideWobbleError::InvalidCodingSequence(CodonUsageError::AmbiguousCodon(
                "NCC".to_owned()
            ))
        );
    }

    #[test]
    fn rejects_short_coding_sequences() {
        let record = SequenceRecord::new(
            SequenceIdentifier::new("wobble5").expect("valid identifier"),
            MoleculeKind::Dna,
            "GCTGCC",
        )
        .expect("dna sequence should build");

        assert!(matches!(
            nucleotide_wobble_profile(&record, 3, 1),
            Err(NucleotideWobbleError::SequenceShorterThanWindow {
                sense_codon_count: 2,
                codon_window: 3,
            })
        ));
    }
}

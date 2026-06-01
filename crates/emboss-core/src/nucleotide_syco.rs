//! Bounded codon-window synonymous codon preference profiles for `syco`.

use crate::codon_usage::{
    CodonUsageError, CodonUsageProfile, cai_for_profile, derive_cai_weights,
    summarize_coding_sequence,
};
use crate::sequence::SequenceRecord;

/// Errors for `syco` profile computation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum NucleotideSycoError {
    /// The input sequence is not nucleotide-like.
    NonNucleotideSequence,
    /// The coding sequence does not satisfy the bounded v1 `syco` model.
    InvalidCodingSequence(CodonUsageError),
    /// The reference codon profile is empty.
    EmptyReferenceProfile,
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

impl std::fmt::Display for NucleotideSycoError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NonNucleotideSequence => {
                write!(f, "syco requires a nucleotide coding-sequence input")
            }
            Self::InvalidCodingSequence(error) => write!(f, "{error}"),
            Self::EmptyReferenceProfile => write!(
                f,
                "syco requires a non-empty reference codon-usage profile"
            ),
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

impl std::error::Error for NucleotideSycoError {}

/// One bounded `syco` analytical row.
#[derive(Clone, Debug, PartialEq)]
pub struct SycoWindow {
    /// One-based inclusive nucleotide start for the window.
    pub window_start: usize,
    /// One-based inclusive nucleotide end for the window.
    pub window_end: usize,
    /// Window length in nucleotides.
    pub window_length: usize,
    /// Window length in codons.
    pub codon_window_length: usize,
    /// Count of sense codons contributing to the score.
    pub sense_codon_count: usize,
    /// Deterministic synonymous codon preference score for the window.
    pub syco_score: f64,
}

/// Full bounded `syco` profile for one coding nucleotide sequence.
#[derive(Clone, Debug, PartialEq)]
pub struct NucleotideSycoProfile {
    /// Stable sequence identifier.
    pub identifier: String,
    /// Sequence length in nucleotides.
    pub sequence_length: usize,
    /// Optional terminal stop codon excluded from profiling.
    pub terminal_stop: Option<String>,
    /// Codon-window length.
    pub codon_window: usize,
    /// Codon-step size.
    pub codon_step: usize,
    /// Sense codon count after terminal-stop exclusion.
    pub sense_codon_count: usize,
    /// Reference profile sense-codon count.
    pub reference_sense_codon_count: usize,
    /// Sliding-window analytical rows.
    pub windows: Vec<SycoWindow>,
}

/// Computes a deterministic bounded `syco` profile for one coding sequence
/// against a reference codon-usage profile.
pub fn nucleotide_syco_profile(
    record: &SequenceRecord,
    reference: &CodonUsageProfile,
    codon_window: usize,
    codon_step: usize,
) -> Result<NucleotideSycoProfile, NucleotideSycoError> {
    if !record.molecule().is_nucleotide() || !record.alphabet().is_nucleotide() {
        return Err(NucleotideSycoError::NonNucleotideSequence);
    }
    if reference.total_sense_codons() == 0 {
        return Err(NucleotideSycoError::EmptyReferenceProfile);
    }
    if codon_window == 0 {
        return Err(NucleotideSycoError::InvalidCodonWindow { codon_window });
    }
    if codon_step == 0 {
        return Err(NucleotideSycoError::InvalidCodonStep { codon_step });
    }

    let coding_summary = summarize_coding_sequence(record.residues())
        .map_err(NucleotideSycoError::InvalidCodingSequence)?;
    if coding_summary.sense_codon_count < codon_window {
        return Err(NucleotideSycoError::SequenceShorterThanWindow {
            sense_codon_count: coding_summary.sense_codon_count,
            codon_window,
        });
    }

    let codons =
        normalized_sense_codons(record.residues(), coding_summary.terminal_stop.as_deref());
    let weights = derive_cai_weights(reference);
    let last_start = codons.len() - codon_window;
    let mut start = 0usize;
    let mut windows = Vec::new();
    while start <= last_start {
        let slice = &codons[start..start + codon_window];
        let mut window_profile = CodonUsageProfile::new();
        for codon in slice {
            window_profile.add_codon(codon);
        }
        windows.push(SycoWindow {
            window_start: start * 3 + 1,
            window_end: (start + codon_window) * 3,
            window_length: codon_window * 3,
            codon_window_length: codon_window,
            sense_codon_count: codon_window,
            syco_score: cai_for_profile(&window_profile, &weights),
        });
        start += codon_step;
    }

    Ok(NucleotideSycoProfile {
        identifier: record.identifier().accession().to_owned(),
        sequence_length: record.len(),
        terminal_stop: coding_summary.terminal_stop,
        codon_window,
        codon_step,
        sense_codon_count: coding_summary.sense_codon_count,
        reference_sense_codon_count: reference.total_sense_codons(),
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

#[cfg(test)]
mod tests {
    use crate::{
        CodonUsageError, MoleculeKind, SequenceIdentifier, SequenceRecord, summarize_coding_sequence,
    };

    use super::{NucleotideSycoError, nucleotide_syco_profile};

    #[test]
    fn computes_expected_syco_profile_for_coding_dna() {
        let reference =
            summarize_coding_sequence("CTTCTTCTACTT").expect("reference profile should derive");
        let record = SequenceRecord::new(
            SequenceIdentifier::new("syco1").expect("valid identifier"),
            MoleculeKind::Dna,
            "CTACTACTT",
        )
        .expect("dna sequence should build");

        let profile = nucleotide_syco_profile(&record, &reference.profile, 2, 1)
            .expect("profile should compute cleanly");
        assert_eq!(profile.windows.len(), 2);
        assert_eq!(profile.sense_codon_count, 3);
        assert_eq!(profile.reference_sense_codon_count, 4);
        assert_eq!(profile.terminal_stop, None);

        let first = &profile.windows[0];
        assert_eq!(first.window_start, 1);
        assert_eq!(first.window_end, 6);
        assert_eq!(first.window_length, 6);
        assert_eq!(first.codon_window_length, 2);
        assert_eq!(first.sense_codon_count, 2);
        assert!((first.syco_score - (1.0_f64 / 3.0_f64)).abs() < 1e-12);

        let second = &profile.windows[1];
        assert_eq!(second.window_start, 4);
        assert_eq!(second.window_end, 9);
        assert!((second.syco_score - (1.0_f64 / 3.0_f64).sqrt()).abs() < 1e-12);
    }

    #[test]
    fn excludes_terminal_stop_from_syco_windows() {
        let reference =
            summarize_coding_sequence("CTTCTTCTACTT").expect("reference profile should derive");
        let record = SequenceRecord::new(
            SequenceIdentifier::new("syco_stop").expect("valid identifier"),
            MoleculeKind::Dna,
            "CTACTACTTTAA",
        )
        .expect("dna sequence should build");

        let profile = nucleotide_syco_profile(&record, &reference.profile, 2, 1)
            .expect("profile should compute cleanly");
        assert_eq!(profile.terminal_stop.as_deref(), Some("TAA"));
        assert_eq!(profile.sense_codon_count, 3);
        assert_eq!(profile.windows.len(), 2);
        assert_eq!(profile.windows[1].window_end, 9);
    }

    #[test]
    fn rejects_empty_reference_profile() {
        let record = SequenceRecord::new(
            SequenceIdentifier::new("syco_empty_ref").expect("valid identifier"),
            MoleculeKind::Dna,
            "CTACTACTT",
        )
        .expect("dna sequence should build");

        let error = nucleotide_syco_profile(&record, &crate::CodonUsageProfile::new(), 2, 1)
            .expect_err("empty reference should fail");
        assert_eq!(error, NucleotideSycoError::EmptyReferenceProfile);
    }

    #[test]
    fn rejects_non_coding_length() {
        let reference =
            summarize_coding_sequence("CTTCTTCTACTT").expect("reference profile should derive");
        let record = SequenceRecord::new(
            SequenceIdentifier::new("syco_bad_len").expect("valid identifier"),
            MoleculeKind::Dna,
            "CTACTACT",
        )
        .expect("dna sequence should build");

        let error = nucleotide_syco_profile(&record, &reference.profile, 2, 1)
            .expect_err("non-coding length should fail");
        assert_eq!(
            error,
            NucleotideSycoError::InvalidCodingSequence(CodonUsageError::NonCodingLength {
                length: 8,
            })
        );
    }

    #[test]
    fn rejects_sequence_shorter_than_requested_window() {
        let reference =
            summarize_coding_sequence("CTTCTTCTACTT").expect("reference profile should derive");
        let record = SequenceRecord::new(
            SequenceIdentifier::new("syco_short").expect("valid identifier"),
            MoleculeKind::Dna,
            "CTACTT",
        )
        .expect("dna sequence should build");

        let error = nucleotide_syco_profile(&record, &reference.profile, 3, 1)
            .expect_err("short coding span should fail");
        assert_eq!(
            error,
            NucleotideSycoError::SequenceShorterThanWindow {
                sense_codon_count: 2,
                codon_window: 3,
            }
        );
    }
}

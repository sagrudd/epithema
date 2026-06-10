//! Bounded siRNA-candidate discovery support for `sirna`.

use std::fmt::{Display, Formatter};

use crate::{SequenceRecord, Strand, reverse_complement_residues};

/// Errors for bounded `sirna` profile computation.
#[derive(Clone, Debug, PartialEq)]
pub enum SirnaError {
    /// The input sequence is not nucleotide-like.
    NonNucleotideSequence,
    /// The requested duplex length is invalid.
    InvalidDuplexLength {
        /// Requested duplex length.
        duplex_length: usize,
    },
    /// The requested step size is invalid.
    InvalidStep {
        /// Requested step size.
        step: usize,
    },
    /// The requested GC-fraction bounds are invalid.
    InvalidGcBounds {
        /// Requested minimum GC fraction.
        min_gc_fraction: f64,
        /// Requested maximum GC fraction.
        max_gc_fraction: f64,
    },
    /// The sequence is shorter than the requested duplex length.
    SequenceShorterThanDuplexLength {
        /// Input sequence length.
        sequence_length: usize,
        /// Requested duplex length.
        duplex_length: usize,
    },
    /// The requested minimum seed AU-count is invalid.
    InvalidSeedAuMinimum {
        /// Requested minimum seed AU-count.
        min_seed_au_count: usize,
    },
    /// The requested maximum homopolymer run is invalid.
    InvalidMaxHomopolymerRun {
        /// Requested maximum homopolymer run.
        max_homopolymer_run: usize,
    },
}

impl Display for SirnaError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NonNucleotideSequence => write!(f, "sirna requires a nucleotide sequence input"),
            Self::InvalidDuplexLength { duplex_length } => write!(
                f,
                "duplex length must be >= 8 for bounded sirna discovery, got {duplex_length}"
            ),
            Self::InvalidStep { step } => write!(f, "step size must be >= 1, got {step}"),
            Self::InvalidGcBounds {
                min_gc_fraction,
                max_gc_fraction,
            } => write!(
                f,
                "GC-fraction bounds must satisfy 0.0 <= min <= max <= 1.0, got [{min_gc_fraction}, {max_gc_fraction}]"
            ),
            Self::SequenceShorterThanDuplexLength {
                sequence_length,
                duplex_length,
            } => write!(
                f,
                "sequence length {sequence_length} is shorter than requested duplex length {duplex_length}"
            ),
            Self::InvalidSeedAuMinimum { min_seed_au_count } => write!(
                f,
                "minimum seed AU-count must be <= 7 for the bounded sirna seed window, got {min_seed_au_count}"
            ),
            Self::InvalidMaxHomopolymerRun {
                max_homopolymer_run,
            } => write!(
                f,
                "maximum homopolymer run must be >= 1, got {max_homopolymer_run}"
            ),
        }
    }
}

impl std::error::Error for SirnaError {}

/// Stable bounded parameters for one `sirna` analytical run.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SirnaParameters {
    /// Duplex length to consider.
    pub duplex_length: usize,
    /// Step size between successive target windows.
    pub step: usize,
    /// Minimum accepted GC fraction.
    pub min_gc_fraction: f64,
    /// Maximum accepted GC fraction.
    pub max_gc_fraction: f64,
    /// Minimum A/U content count in the guide seed positions 2-8.
    pub min_seed_au_count: usize,
    /// Maximum allowed homopolymer run in the sense strand.
    pub max_homopolymer_run: usize,
    /// Whether the guide 5' base must be A/U-like.
    pub require_guide_five_prime_au: bool,
}

impl Default for SirnaParameters {
    fn default() -> Self {
        Self {
            duplex_length: 21,
            step: 1,
            min_gc_fraction: 0.30,
            max_gc_fraction: 0.55,
            min_seed_au_count: 4,
            max_homopolymer_run: 3,
            require_guide_five_prime_au: true,
        }
    }
}

/// One bounded `sirna` candidate row.
#[derive(Clone, Debug, PartialEq)]
pub struct SirnaCandidate {
    /// Stable candidate identifier.
    pub candidate_id: String,
    /// Candidate orientation on the input sequence.
    pub strand: Strand,
    /// One-based inclusive target start on the input sequence.
    pub target_start: usize,
    /// One-based inclusive target end on the input sequence.
    pub target_end: usize,
    /// Duplex length in residues.
    pub duplex_length: usize,
    /// Sense-strand sequence in target orientation.
    pub sense_sequence: String,
    /// Guide-strand sequence paired against the target.
    pub guide_sequence: String,
    /// Canonical A/C/G/T/U count in the target window.
    pub canonical_symbols: usize,
    /// Ambiguous symbol count in the target window.
    pub ambiguous_symbols: usize,
    /// Fraction of canonical G/C residues in the target window.
    pub gc_fraction: f64,
    /// Guide-strand 5' terminal base.
    pub guide_five_prime_base: char,
    /// Count of A/U-like bases in guide positions 2-8.
    pub guide_seed_au_count: usize,
    /// Longest homopolymer run in the target window.
    pub max_homopolymer_run: usize,
}

/// Full bounded `sirna` profile for one nucleotide sequence.
#[derive(Clone, Debug, PartialEq)]
pub struct SirnaProfile {
    /// Stable target identifier.
    pub identifier: String,
    /// Target sequence length.
    pub sequence_length: usize,
    /// Parameters used to generate candidates.
    pub parameters: SirnaParameters,
    /// Stable ordered candidate rows.
    pub candidates: Vec<SirnaCandidate>,
}

/// Computes a deterministic bounded `sirna` candidate profile.
pub fn sirna_profile(
    record: &SequenceRecord,
    parameters: SirnaParameters,
) -> Result<SirnaProfile, SirnaError> {
    validate_parameters(record, parameters)?;

    let residues = record.residues();
    let duplex_length = parameters.duplex_length;
    let last_start = residues.len() - duplex_length;
    let mut start = 0usize;
    let mut candidates = Vec::new();

    while start <= last_start {
        let slice = &residues[start..start + duplex_length];
        if let Some(window) = candidate_window(slice) {
            let guide_sequence = reverse_complement_residues(record.molecule(), slice).expect(
                "validated canonical nucleotide windows always support reverse complements",
            );
            let guide_five_prime_base = char::from(guide_sequence.as_bytes()[0]);
            let guide_seed_au_count =
                count_au_like(&guide_sequence[1..guide_sequence.len().min(8)]);

            if candidate_passes_bounds(
                parameters,
                window.gc_fraction,
                guide_five_prime_base,
                guide_seed_au_count,
                window.max_homopolymer_run,
            ) {
                candidates.push(SirnaCandidate {
                    candidate_id: format!("sirna-{:05}", candidates.len() + 1),
                    strand: Strand::Forward,
                    target_start: start + 1,
                    target_end: start + duplex_length,
                    duplex_length,
                    sense_sequence: slice.to_owned(),
                    guide_sequence,
                    canonical_symbols: window.canonical_symbols,
                    ambiguous_symbols: window.ambiguous_symbols,
                    gc_fraction: window.gc_fraction,
                    guide_five_prime_base,
                    guide_seed_au_count,
                    max_homopolymer_run: window.max_homopolymer_run,
                });
            }
        }

        start += parameters.step;
    }

    Ok(SirnaProfile {
        identifier: record.identifier().accession().to_owned(),
        sequence_length: record.len(),
        parameters,
        candidates,
    })
}

fn validate_parameters(
    record: &SequenceRecord,
    parameters: SirnaParameters,
) -> Result<(), SirnaError> {
    if !record.molecule().is_nucleotide() || !record.alphabet().is_nucleotide() {
        return Err(SirnaError::NonNucleotideSequence);
    }
    if parameters.duplex_length < 8 {
        return Err(SirnaError::InvalidDuplexLength {
            duplex_length: parameters.duplex_length,
        });
    }
    if parameters.step == 0 {
        return Err(SirnaError::InvalidStep {
            step: parameters.step,
        });
    }
    if !(0.0..=1.0).contains(&parameters.min_gc_fraction)
        || !(0.0..=1.0).contains(&parameters.max_gc_fraction)
        || parameters.min_gc_fraction > parameters.max_gc_fraction
    {
        return Err(SirnaError::InvalidGcBounds {
            min_gc_fraction: parameters.min_gc_fraction,
            max_gc_fraction: parameters.max_gc_fraction,
        });
    }
    if parameters.min_seed_au_count > 7 {
        return Err(SirnaError::InvalidSeedAuMinimum {
            min_seed_au_count: parameters.min_seed_au_count,
        });
    }
    if parameters.max_homopolymer_run == 0 {
        return Err(SirnaError::InvalidMaxHomopolymerRun {
            max_homopolymer_run: parameters.max_homopolymer_run,
        });
    }
    if record.len() < parameters.duplex_length {
        return Err(SirnaError::SequenceShorterThanDuplexLength {
            sequence_length: record.len(),
            duplex_length: parameters.duplex_length,
        });
    }

    Ok(())
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct CandidateWindow {
    canonical_symbols: usize,
    ambiguous_symbols: usize,
    gc_fraction: f64,
    max_homopolymer_run: usize,
}

fn candidate_window(slice: &str) -> Option<CandidateWindow> {
    let canonical_symbols = slice
        .bytes()
        .filter(|base| is_canonical_nucleotide(*base))
        .count();
    let ambiguous_symbols = slice.len() - canonical_symbols;
    if ambiguous_symbols > 0 {
        return None;
    }

    let gc_symbols = slice.bytes().filter(|base| is_gc(*base)).count();
    let gc_fraction = gc_symbols as f64 / canonical_symbols as f64;

    Some(CandidateWindow {
        canonical_symbols,
        ambiguous_symbols,
        gc_fraction,
        max_homopolymer_run: longest_homopolymer_run(slice),
    })
}

fn candidate_passes_bounds(
    parameters: SirnaParameters,
    gc_fraction: f64,
    guide_five_prime_base: char,
    guide_seed_au_count: usize,
    max_homopolymer_run: usize,
) -> bool {
    if gc_fraction < parameters.min_gc_fraction || gc_fraction > parameters.max_gc_fraction {
        return false;
    }
    if parameters.require_guide_five_prime_au && !matches!(guide_five_prime_base, 'A' | 'T' | 'U') {
        return false;
    }
    if guide_seed_au_count < parameters.min_seed_au_count {
        return false;
    }
    if max_homopolymer_run > parameters.max_homopolymer_run {
        return false;
    }

    true
}

fn is_canonical_nucleotide(base: u8) -> bool {
    matches!(base, b'A' | b'C' | b'G' | b'T' | b'U')
}

fn is_gc(base: u8) -> bool {
    matches!(base, b'G' | b'C')
}

fn is_au_like(base: u8) -> bool {
    matches!(base, b'A' | b'T' | b'U')
}

fn count_au_like(slice: &str) -> usize {
    slice.bytes().filter(|base| is_au_like(*base)).count()
}

fn longest_homopolymer_run(slice: &str) -> usize {
    let mut longest = 0usize;
    let mut current = 0usize;
    let mut last = None;

    for base in slice.bytes() {
        if Some(base) == last {
            current += 1;
        } else {
            last = Some(base);
            current = 1;
        }
        longest = longest.max(current);
    }

    longest
}

#[cfg(test)]
mod tests {
    use crate::{MoleculeKind, SequenceIdentifier, SequenceRecord, reverse_complement_residues};

    use super::{SirnaError, SirnaParameters, sirna_profile};

    fn dna_record(id: &str, residues: &str) -> SequenceRecord {
        SequenceRecord::new(
            SequenceIdentifier::new(id).expect("identifier should be valid"),
            MoleculeKind::Dna,
            residues,
        )
        .expect("record should normalize")
    }

    #[test]
    fn computes_deterministic_sirna_candidates_for_one_target_window() {
        let record = dna_record("target1", "AATATCGCCATGCGATATATT");
        let parameters = SirnaParameters {
            duplex_length: 21,
            step: 1,
            min_gc_fraction: 0.30,
            max_gc_fraction: 0.55,
            min_seed_au_count: 4,
            max_homopolymer_run: 3,
            require_guide_five_prime_au: true,
        };

        let profile = sirna_profile(&record, parameters).expect("profile should compute");

        assert_eq!(profile.identifier, "target1");
        assert_eq!(profile.sequence_length, 21);
        assert_eq!(profile.parameters, parameters);
        assert_eq!(profile.candidates.len(), 1);

        let candidate = &profile.candidates[0];
        assert_eq!(candidate.candidate_id, "sirna-00001");
        assert_eq!(candidate.strand, crate::Strand::Forward);
        assert_eq!(candidate.target_start, 1);
        assert_eq!(candidate.target_end, 21);
        assert_eq!(candidate.duplex_length, 21);
        assert_eq!(candidate.sense_sequence, "AATATCGCCATGCGATATATT");
        assert_eq!(
            candidate.guide_sequence,
            reverse_complement_residues(MoleculeKind::Dna, candidate.sense_sequence.as_str())
                .expect("reverse complement should compute")
        );
        assert_eq!(candidate.canonical_symbols, 21);
        assert_eq!(candidate.ambiguous_symbols, 0);
        assert!((candidate.gc_fraction - (7.0 / 21.0)).abs() < 1e-12);
        assert_eq!(candidate.guide_five_prime_base, 'A');
        assert_eq!(candidate.guide_seed_au_count, 6);
        assert_eq!(candidate.max_homopolymer_run, 2);
    }

    #[test]
    fn skips_ambiguous_windows_without_error() {
        let record = dna_record("target2", "AATATCGCCATGCGATATATN");
        let profile = sirna_profile(
            &record,
            SirnaParameters {
                duplex_length: 21,
                ..SirnaParameters::default()
            },
        )
        .expect("profile should compute");

        assert!(profile.candidates.is_empty());
    }

    #[test]
    fn rejects_non_nucleotide_input() {
        let record = SequenceRecord::new(
            SequenceIdentifier::new("protein1").expect("identifier should be valid"),
            MoleculeKind::Protein,
            "MSTNPKPQ",
        )
        .expect("record should normalize");

        let error = sirna_profile(&record, SirnaParameters::default())
            .expect_err("protein input should fail");

        assert_eq!(error, SirnaError::NonNucleotideSequence);
    }

    #[test]
    fn rejects_invalid_bounded_parameters() {
        let record = dna_record("target3", "AATATCGCCATGCGATATATT");

        let error = sirna_profile(
            &record,
            SirnaParameters {
                duplex_length: 7,
                ..SirnaParameters::default()
            },
        )
        .expect_err("short duplex length should fail");
        assert_eq!(error, SirnaError::InvalidDuplexLength { duplex_length: 7 });

        let error = sirna_profile(
            &record,
            SirnaParameters {
                min_seed_au_count: 8,
                ..SirnaParameters::default()
            },
        )
        .expect_err("oversized seed AU minimum should fail");
        assert_eq!(
            error,
            SirnaError::InvalidSeedAuMinimum {
                min_seed_au_count: 8
            }
        );
    }
}

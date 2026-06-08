//! Bounded primer-and-oligo candidate generation support for `eprimer3`.

use std::fmt::{Display, Formatter};

use crate::{MoleculeKind, SequenceRecord, Strand, reverse_complement_residues};

/// Errors for bounded `eprimer3` profile computation.
#[derive(Clone, Debug, PartialEq)]
pub enum Eprimer3Error {
    /// The input sequence is not nucleotide-like.
    NonNucleotideSequence,
    /// The requested minimum oligo length is invalid.
    InvalidMinOligoLength {
        /// Requested minimum oligo length.
        min_oligo_length: usize,
    },
    /// The requested maximum oligo length is invalid.
    InvalidMaxOligoLength {
        /// Requested maximum oligo length.
        max_oligo_length: usize,
    },
    /// The requested oligo-length range is invalid.
    InvalidOligoLengthRange {
        /// Requested minimum oligo length.
        min_oligo_length: usize,
        /// Requested maximum oligo length.
        max_oligo_length: usize,
    },
    /// The requested step size is invalid.
    InvalidStep {
        /// Requested step size.
        step: usize,
    },
    /// The sequence is shorter than the requested minimum oligo length.
    SequenceShorterThanMinOligoLength {
        /// Input sequence length.
        sequence_length: usize,
        /// Requested minimum oligo length.
        min_oligo_length: usize,
    },
    /// The requested GC-fraction bounds are invalid.
    InvalidGcBounds {
        /// Requested minimum GC fraction.
        min_gc_fraction: f64,
        /// Requested maximum GC fraction.
        max_gc_fraction: f64,
    },
    /// The requested melting-temperature bounds are invalid.
    InvalidTmBounds {
        /// Requested minimum Tm.
        min_tm_celsius: f64,
        /// Requested maximum Tm.
        max_tm_celsius: f64,
    },
}

impl Display for Eprimer3Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NonNucleotideSequence => {
                write!(f, "eprimer3 requires a nucleotide sequence input")
            }
            Self::InvalidMinOligoLength { min_oligo_length } => write!(
                f,
                "minimum oligo length must be >= 1, got {min_oligo_length}"
            ),
            Self::InvalidMaxOligoLength { max_oligo_length } => write!(
                f,
                "maximum oligo length must be >= 1, got {max_oligo_length}"
            ),
            Self::InvalidOligoLengthRange {
                min_oligo_length,
                max_oligo_length,
            } => write!(
                f,
                "minimum oligo length {min_oligo_length} exceeds maximum oligo length {max_oligo_length}"
            ),
            Self::InvalidStep { step } => write!(f, "step size must be >= 1, got {step}"),
            Self::SequenceShorterThanMinOligoLength {
                sequence_length,
                min_oligo_length,
            } => write!(
                f,
                "sequence length {sequence_length} is shorter than requested minimum oligo length {min_oligo_length}"
            ),
            Self::InvalidGcBounds {
                min_gc_fraction,
                max_gc_fraction,
            } => write!(
                f,
                "GC-fraction bounds must satisfy 0.0 <= min <= max <= 1.0, got [{min_gc_fraction}, {max_gc_fraction}]"
            ),
            Self::InvalidTmBounds {
                min_tm_celsius,
                max_tm_celsius,
            } => write!(
                f,
                "Tm bounds must satisfy min <= max, got [{min_tm_celsius}, {max_tm_celsius}]"
            ),
        }
    }
}

impl std::error::Error for Eprimer3Error {}

/// Stable bounded design parameters for one `eprimer3` analytical run.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Eprimer3Parameters {
    /// Minimum oligo length to consider.
    pub min_oligo_length: usize,
    /// Maximum oligo length to consider.
    pub max_oligo_length: usize,
    /// Step size between successive window starts.
    pub step: usize,
    /// Minimum accepted GC fraction.
    pub min_gc_fraction: f64,
    /// Maximum accepted GC fraction.
    pub max_gc_fraction: f64,
    /// Minimum accepted melting estimate.
    pub min_tm_celsius: f64,
    /// Maximum accepted melting estimate.
    pub max_tm_celsius: f64,
}

impl Default for Eprimer3Parameters {
    fn default() -> Self {
        Self {
            min_oligo_length: 18,
            max_oligo_length: 22,
            step: 1,
            min_gc_fraction: 0.40,
            max_gc_fraction: 0.60,
            min_tm_celsius: 52.0,
            max_tm_celsius: 65.0,
        }
    }
}

/// One bounded `eprimer3` candidate row.
#[derive(Clone, Debug, PartialEq)]
pub struct Eprimer3Candidate {
    /// Stable candidate identifier.
    pub candidate_id: String,
    /// Candidate orientation on the input sequence.
    pub strand: Strand,
    /// One-based inclusive candidate start on the input sequence.
    pub oligo_start: usize,
    /// One-based inclusive candidate end on the input sequence.
    pub oligo_end: usize,
    /// Candidate length in residues.
    pub oligo_length: usize,
    /// Normalized oligo sequence in candidate orientation.
    pub oligo_sequence: String,
    /// Canonical A/C/G/T/U count in the underlying genomic window.
    pub canonical_symbols: usize,
    /// Ambiguous symbol count in the underlying genomic window.
    pub ambiguous_symbols: usize,
    /// Fraction of canonical G/C residues.
    pub gc_fraction: f64,
    /// Conservative melting estimate in Celsius.
    pub tm_celsius: f64,
    /// Count of G/C residues in the oligo's 3' terminal triplet, or shorter suffix if needed.
    pub three_prime_gc_count: usize,
}

/// Full bounded `eprimer3` profile for one nucleotide sequence.
#[derive(Clone, Debug, PartialEq)]
pub struct Eprimer3Profile {
    /// Stable sequence identifier.
    pub identifier: String,
    /// Input sequence length.
    pub sequence_length: usize,
    /// Parameters used to generate candidates.
    pub parameters: Eprimer3Parameters,
    /// Stable ordered candidate rows.
    pub candidates: Vec<Eprimer3Candidate>,
}

/// Computes a deterministic bounded `eprimer3` candidate profile.
pub fn eprimer3_profile(
    record: &SequenceRecord,
    parameters: Eprimer3Parameters,
) -> Result<Eprimer3Profile, Eprimer3Error> {
    validate_parameters(record, parameters)?;

    let residues = record.residues();
    let mut candidates = Vec::new();

    for oligo_length in parameters.min_oligo_length..=parameters.max_oligo_length {
        if oligo_length > residues.len() {
            continue;
        }

        let last_start = residues.len() - oligo_length;
        let mut start = 0usize;
        while start <= last_start {
            let slice = &residues[start..start + oligo_length];
            if let Some(window) = candidate_window(slice) {
                if candidate_passes_bounds(window.gc_fraction, window.tm_celsius, parameters) {
                    candidates.push(build_candidate(
                        record.identifier().accession(),
                        start,
                        oligo_length,
                        Strand::Forward,
                        slice.to_owned(),
                        &window,
                    ));

                    let reverse_sequence =
                        reverse_complement_residues(MoleculeKind::Dna, slice).expect(
                            "validated canonical nucleotide windows always support reverse complements",
                        );
                    candidates.push(build_candidate(
                        record.identifier().accession(),
                        start,
                        oligo_length,
                        Strand::Reverse,
                        reverse_sequence,
                        &window,
                    ));
                }
            }
            start += parameters.step;
        }
    }

    Ok(Eprimer3Profile {
        identifier: record.identifier().accession().to_owned(),
        sequence_length: record.len(),
        parameters,
        candidates,
    })
}

fn validate_parameters(
    record: &SequenceRecord,
    parameters: Eprimer3Parameters,
) -> Result<(), Eprimer3Error> {
    if !record.molecule().is_nucleotide() || !record.alphabet().is_nucleotide() {
        return Err(Eprimer3Error::NonNucleotideSequence);
    }
    if parameters.min_oligo_length == 0 {
        return Err(Eprimer3Error::InvalidMinOligoLength {
            min_oligo_length: parameters.min_oligo_length,
        });
    }
    if parameters.max_oligo_length == 0 {
        return Err(Eprimer3Error::InvalidMaxOligoLength {
            max_oligo_length: parameters.max_oligo_length,
        });
    }
    if parameters.min_oligo_length > parameters.max_oligo_length {
        return Err(Eprimer3Error::InvalidOligoLengthRange {
            min_oligo_length: parameters.min_oligo_length,
            max_oligo_length: parameters.max_oligo_length,
        });
    }
    if parameters.step == 0 {
        return Err(Eprimer3Error::InvalidStep {
            step: parameters.step,
        });
    }
    if record.len() < parameters.min_oligo_length {
        return Err(Eprimer3Error::SequenceShorterThanMinOligoLength {
            sequence_length: record.len(),
            min_oligo_length: parameters.min_oligo_length,
        });
    }
    if !(0.0..=1.0).contains(&parameters.min_gc_fraction)
        || !(0.0..=1.0).contains(&parameters.max_gc_fraction)
        || parameters.min_gc_fraction > parameters.max_gc_fraction
    {
        return Err(Eprimer3Error::InvalidGcBounds {
            min_gc_fraction: parameters.min_gc_fraction,
            max_gc_fraction: parameters.max_gc_fraction,
        });
    }
    if parameters.min_tm_celsius > parameters.max_tm_celsius {
        return Err(Eprimer3Error::InvalidTmBounds {
            min_tm_celsius: parameters.min_tm_celsius,
            max_tm_celsius: parameters.max_tm_celsius,
        });
    }

    Ok(())
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct CandidateWindow {
    canonical_symbols: usize,
    ambiguous_symbols: usize,
    gc_fraction: f64,
    tm_celsius: f64,
    three_prime_gc_count: usize,
}

fn candidate_window(slice: &str) -> Option<CandidateWindow> {
    let mut canonical_symbols = 0usize;
    let mut gc_symbols = 0usize;
    let mut ambiguous_symbols = 0usize;

    for residue in slice.chars() {
        match residue {
            'A' | 'T' | 'U' => {
                canonical_symbols += 1;
            }
            'C' | 'G' => {
                canonical_symbols += 1;
                gc_symbols += 1;
            }
            _ => {
                ambiguous_symbols += 1;
            }
        }
    }

    if canonical_symbols == 0 || ambiguous_symbols > 0 {
        return None;
    }

    let gc_fraction = gc_symbols as f64 / canonical_symbols as f64;
    let tm_celsius = estimate_tm_celsius(canonical_symbols, gc_symbols);
    let three_prime_gc_count = slice
        .chars()
        .rev()
        .take(3)
        .filter(|residue| matches!(residue, 'G' | 'C'))
        .count();

    Some(CandidateWindow {
        canonical_symbols,
        ambiguous_symbols,
        gc_fraction,
        tm_celsius,
        three_prime_gc_count,
    })
}

fn candidate_passes_bounds(
    gc_fraction: f64,
    tm_celsius: f64,
    parameters: Eprimer3Parameters,
) -> bool {
    gc_fraction >= parameters.min_gc_fraction
        && gc_fraction <= parameters.max_gc_fraction
        && tm_celsius >= parameters.min_tm_celsius
        && tm_celsius <= parameters.max_tm_celsius
}

fn estimate_tm_celsius(length: usize, gc_symbols: usize) -> f64 {
    let at_symbols = length - gc_symbols;
    if length < 14 {
        (2 * at_symbols + 4 * gc_symbols) as f64
    } else {
        64.9 + 41.0 * (gc_symbols as f64 - 16.4) / length as f64
    }
}

fn build_candidate(
    identifier: &str,
    start: usize,
    oligo_length: usize,
    strand: Strand,
    oligo_sequence: String,
    window: &CandidateWindow,
) -> Eprimer3Candidate {
    let oligo_start = start + 1;
    let oligo_end = start + oligo_length;
    let strand_label = match strand {
        Strand::Forward => "forward",
        Strand::Reverse => "reverse",
        Strand::Unstranded => "unstranded",
        Strand::Unknown => "unknown",
    };

    Eprimer3Candidate {
        candidate_id: format!("{identifier}:{strand_label}:{oligo_start}-{oligo_end}"),
        strand,
        oligo_start,
        oligo_end,
        oligo_length,
        oligo_sequence,
        canonical_symbols: window.canonical_symbols,
        ambiguous_symbols: window.ambiguous_symbols,
        gc_fraction: window.gc_fraction,
        tm_celsius: window.tm_celsius,
        three_prime_gc_count: window.three_prime_gc_count,
    }
}

#[cfg(test)]
mod tests {
    use crate::{MoleculeKind, SequenceIdentifier, SequenceRecord, Strand};

    use super::{Eprimer3Error, Eprimer3Parameters, eprimer3_profile};

    fn dna_record(id: &str, residues: &str) -> SequenceRecord {
        SequenceRecord::new(
            SequenceIdentifier::new(id).expect("valid identifier"),
            MoleculeKind::Dna,
            residues,
        )
        .expect("dna record should build")
    }

    #[test]
    fn computes_deterministic_forward_and_reverse_candidates() {
        let record = dna_record("ep3a", "ATGCGCGCAT");
        let parameters = Eprimer3Parameters {
            min_oligo_length: 4,
            max_oligo_length: 4,
            step: 1,
            min_gc_fraction: 0.5,
            max_gc_fraction: 1.0,
            min_tm_celsius: 12.0,
            max_tm_celsius: 16.0,
        };

        let profile = eprimer3_profile(&record, parameters).expect("profile should compute");
        assert_eq!(profile.candidates.len(), 14);

        let first = &profile.candidates[0];
        assert_eq!(first.candidate_id, "ep3a:forward:1-4");
        assert_eq!(first.strand, Strand::Forward);
        assert_eq!(first.oligo_sequence, "ATGC");
        assert_eq!(first.oligo_start, 1);
        assert_eq!(first.oligo_end, 4);
        assert!((first.gc_fraction - 0.5).abs() < 1e-12);
        assert!((first.tm_celsius - 12.0).abs() < 1e-12);
        assert_eq!(first.three_prime_gc_count, 2);

        let second = &profile.candidates[1];
        assert_eq!(second.candidate_id, "ep3a:reverse:1-4");
        assert_eq!(second.strand, Strand::Reverse);
        assert_eq!(second.oligo_sequence, "GCAT");
        assert_eq!(second.oligo_start, 1);
        assert_eq!(second.oligo_end, 4);
    }

    #[test]
    fn skips_windows_with_ambiguous_symbols_honestly() {
        let record = dna_record("ep3b", "ATGNCCGC");
        let parameters = Eprimer3Parameters {
            min_oligo_length: 4,
            max_oligo_length: 4,
            step: 1,
            min_gc_fraction: 0.0,
            max_gc_fraction: 1.0,
            min_tm_celsius: 0.0,
            max_tm_celsius: 100.0,
        };

        let profile = eprimer3_profile(&record, parameters).expect("profile should compute");
        assert!(
            profile
                .candidates
                .iter()
                .all(|candidate| candidate.ambiguous_symbols == 0)
        );
        assert_eq!(profile.candidates.len(), 2);
        assert_eq!(profile.candidates[0].candidate_id, "ep3b:forward:5-8");
    }

    #[test]
    fn rejects_non_nucleotide_inputs() {
        let record = SequenceRecord::new(
            SequenceIdentifier::new("ep3c").expect("valid identifier"),
            MoleculeKind::Protein,
            "MSTNPK",
        )
        .expect("protein record should build");

        let error = eprimer3_profile(&record, Eprimer3Parameters::default())
            .expect_err("protein input should fail");
        assert_eq!(error, Eprimer3Error::NonNucleotideSequence);
    }

    #[test]
    fn rejects_invalid_length_and_bound_parameters() {
        let record = dna_record("ep3d", "ATGCGCGCAT");

        let error = eprimer3_profile(
            &record,
            Eprimer3Parameters {
                min_oligo_length: 0,
                ..Eprimer3Parameters::default()
            },
        )
        .expect_err("zero minimum length should fail");
        assert_eq!(
            error,
            Eprimer3Error::InvalidMinOligoLength {
                min_oligo_length: 0
            }
        );

        let error = eprimer3_profile(
            &record,
            Eprimer3Parameters {
                min_oligo_length: 10,
                max_oligo_length: 4,
                ..Eprimer3Parameters::default()
            },
        )
        .expect_err("invalid length range should fail");
        assert_eq!(
            error,
            Eprimer3Error::InvalidOligoLengthRange {
                min_oligo_length: 10,
                max_oligo_length: 4,
            }
        );

        let error = eprimer3_profile(
            &record,
            Eprimer3Parameters {
                min_oligo_length: 4,
                max_oligo_length: 4,
                min_gc_fraction: 0.8,
                max_gc_fraction: 0.2,
                ..Eprimer3Parameters::default()
            },
        )
        .expect_err("invalid gc bounds should fail");
        assert_eq!(
            error,
            Eprimer3Error::InvalidGcBounds {
                min_gc_fraction: 0.8,
                max_gc_fraction: 0.2,
            }
        );

        let error = eprimer3_profile(
            &record,
            Eprimer3Parameters {
                min_oligo_length: 4,
                max_oligo_length: 4,
                min_tm_celsius: 70.0,
                max_tm_celsius: 60.0,
                ..Eprimer3Parameters::default()
            },
        )
        .expect_err("invalid tm bounds should fail");
        assert_eq!(
            error,
            Eprimer3Error::InvalidTmBounds {
                min_tm_celsius: 70.0,
                max_tm_celsius: 60.0,
            }
        );
    }
}

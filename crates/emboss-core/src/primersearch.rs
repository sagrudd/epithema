//! Bounded primer-pair matching support for `primersearch`.

use std::fmt::{Display, Formatter};

use crate::{
    MoleculeKind, NucleotidePattern, PatternError, PatternMatch, SequenceRecord, Strand,
    reverse_complement_residues,
};

/// Errors for bounded `primersearch` profile computation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum PrimersearchError {
    /// The input sequence is not nucleotide-like.
    NonNucleotideSequence,
    /// At least one primer pair is required.
    EmptyPrimerPairs,
    /// Primer-pair names must not be blank.
    EmptyPrimerPairName,
    /// One primer sequence failed bounded nucleotide-pattern validation.
    InvalidPrimerPattern {
        /// Which primer role failed validation.
        role: PrimerRole,
        /// Underlying validation error.
        error: PatternError,
    },
    /// Reverse-complement construction failed unexpectedly for a validated primer.
    ReverseComplementUnsupported {
        /// Which primer role failed reverse-complement construction.
        role: PrimerRole,
        /// Underlying reverse-complement error.
        message: String,
    },
}

impl Display for PrimersearchError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NonNucleotideSequence => {
                write!(f, "primersearch requires a nucleotide sequence input")
            }
            Self::EmptyPrimerPairs => write!(f, "primersearch requires at least one primer pair"),
            Self::EmptyPrimerPairName => write!(f, "primer pair names must not be empty"),
            Self::InvalidPrimerPattern { role, error } => {
                write!(f, "invalid {role} primer pattern: {error}")
            }
            Self::ReverseComplementUnsupported { role, message } => {
                write!(
                    f,
                    "validated {role} primer could not be reverse-complemented: {message}"
                )
            }
        }
    }
}

impl std::error::Error for PrimersearchError {}

/// Stable role label for one primer within a pair.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PrimerRole {
    /// The pair's forward primer.
    Forward,
    /// The pair's reverse primer.
    Reverse,
}

impl Display for PrimerRole {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Forward => f.write_str("forward"),
            Self::Reverse => f.write_str("reverse"),
        }
    }
}

/// One validated primer pair used for bounded `primersearch`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrimerPair {
    name: String,
    forward_primer: String,
    reverse_primer: String,
    forward_pattern: NucleotidePattern,
    reverse_pattern: NucleotidePattern,
    reverse_complement_forward_pattern: NucleotidePattern,
    reverse_complement_reverse_pattern: NucleotidePattern,
}

impl PrimerPair {
    /// Creates one validated bounded primer pair.
    pub fn new(
        name: impl Into<String>,
        forward_primer: impl AsRef<str>,
        reverse_primer: impl AsRef<str>,
    ) -> Result<Self, PrimersearchError> {
        let name = name.into().trim().to_owned();
        if name.is_empty() {
            return Err(PrimersearchError::EmptyPrimerPairName);
        }

        let forward_pattern =
            NucleotidePattern::parse(forward_primer.as_ref()).map_err(|error| {
                PrimersearchError::InvalidPrimerPattern {
                    role: PrimerRole::Forward,
                    error,
                }
            })?;
        let reverse_pattern =
            NucleotidePattern::parse(reverse_primer.as_ref()).map_err(|error| {
                PrimersearchError::InvalidPrimerPattern {
                    role: PrimerRole::Reverse,
                    error,
                }
            })?;

        let reverse_complement_forward =
            reverse_complement_residues(MoleculeKind::Dna, forward_pattern.raw()).map_err(
                |error| PrimersearchError::ReverseComplementUnsupported {
                    role: PrimerRole::Forward,
                    message: error.to_string(),
                },
            )?;
        let reverse_complement_reverse =
            reverse_complement_residues(MoleculeKind::Dna, reverse_pattern.raw()).map_err(
                |error| PrimersearchError::ReverseComplementUnsupported {
                    role: PrimerRole::Reverse,
                    message: error.to_string(),
                },
            )?;

        let reverse_complement_forward_pattern =
            NucleotidePattern::parse(&reverse_complement_forward).map_err(|error| {
                PrimersearchError::InvalidPrimerPattern {
                    role: PrimerRole::Forward,
                    error,
                }
            })?;
        let reverse_complement_reverse_pattern =
            NucleotidePattern::parse(&reverse_complement_reverse).map_err(|error| {
                PrimersearchError::InvalidPrimerPattern {
                    role: PrimerRole::Reverse,
                    error,
                }
            })?;

        Ok(Self {
            name,
            forward_primer: forward_pattern.raw().to_owned(),
            reverse_primer: reverse_pattern.raw().to_owned(),
            forward_pattern,
            reverse_pattern,
            reverse_complement_forward_pattern,
            reverse_complement_reverse_pattern,
        })
    }

    /// Returns the stable primer-pair name.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the normalized forward primer text.
    #[must_use]
    pub fn forward_primer(&self) -> &str {
        &self.forward_primer
    }

    /// Returns the normalized reverse primer text.
    #[must_use]
    pub fn reverse_primer(&self) -> &str {
        &self.reverse_primer
    }
}

/// One bounded `primersearch` hit row.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrimersearchHit {
    /// Stable primer-pair name.
    pub primer_pair_name: String,
    /// Deterministic amplicon orientation on the input sequence.
    pub strand: Strand,
    /// One-based inclusive left-primer start on the input sequence.
    pub left_primer_start: usize,
    /// One-based inclusive left-primer end on the input sequence.
    pub left_primer_end: usize,
    /// One-based inclusive right-primer start on the input sequence.
    pub right_primer_start: usize,
    /// One-based inclusive right-primer end on the input sequence.
    pub right_primer_end: usize,
    /// One-based inclusive amplicon start on the input sequence.
    pub amplicon_start: usize,
    /// One-based inclusive amplicon end on the input sequence.
    pub amplicon_end: usize,
    /// Amplicon span length in residues.
    pub amplicon_length: usize,
    /// Matched left-primer slice on the input sequence.
    pub left_matched: String,
    /// Matched right-primer slice on the input sequence.
    pub right_matched: String,
}

/// Full bounded `primersearch` profile for one sequence record.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrimersearchProfile {
    /// Stable target identifier.
    pub identifier: String,
    /// Target sequence length.
    pub sequence_length: usize,
    /// Primer pairs considered in stable input order.
    pub primer_pair_count: usize,
    /// Stable ordered complete-pair hits.
    pub hits: Vec<PrimersearchHit>,
}

/// Computes a deterministic bounded primer-pair search profile.
pub fn primersearch_profile(
    record: &SequenceRecord,
    primer_pairs: &[PrimerPair],
) -> Result<PrimersearchProfile, PrimersearchError> {
    if !record.molecule().is_nucleotide() || !record.alphabet().is_nucleotide() {
        return Err(PrimersearchError::NonNucleotideSequence);
    }
    if primer_pairs.is_empty() {
        return Err(PrimersearchError::EmptyPrimerPairs);
    }

    let residues = record.residues();
    let mut hits = Vec::new();

    for primer_pair in primer_pairs {
        let forward_left_hits = primer_pair.forward_pattern.scan(residues);
        let forward_right_hits = primer_pair
            .reverse_complement_reverse_pattern
            .scan(residues);
        collect_complete_hits(
            &mut hits,
            primer_pair.name(),
            Strand::Forward,
            &forward_left_hits,
            &forward_right_hits,
        );

        let reverse_left_hits = primer_pair
            .reverse_complement_forward_pattern
            .scan(residues);
        let reverse_right_hits = primer_pair.reverse_pattern.scan(residues);
        collect_complete_hits(
            &mut hits,
            primer_pair.name(),
            Strand::Reverse,
            &reverse_left_hits,
            &reverse_right_hits,
        );
    }

    Ok(PrimersearchProfile {
        identifier: record.identifier().accession().to_owned(),
        sequence_length: record.len(),
        primer_pair_count: primer_pairs.len(),
        hits,
    })
}

fn collect_complete_hits(
    hits: &mut Vec<PrimersearchHit>,
    primer_pair_name: &str,
    strand: Strand,
    left_hits: &[PatternMatch],
    right_hits: &[PatternMatch],
) {
    for left_hit in left_hits {
        for right_hit in right_hits {
            if right_hit.start() < left_hit.start() {
                continue;
            }

            let amplicon_start = left_hit.start() + 1;
            let amplicon_end = right_hit.end();
            hits.push(PrimersearchHit {
                primer_pair_name: primer_pair_name.to_owned(),
                strand,
                left_primer_start: left_hit.start() + 1,
                left_primer_end: left_hit.end(),
                right_primer_start: right_hit.start() + 1,
                right_primer_end: right_hit.end(),
                amplicon_start,
                amplicon_end,
                amplicon_length: amplicon_end - left_hit.start(),
                left_matched: left_hit.matched().to_owned(),
                right_matched: right_hit.matched().to_owned(),
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{MoleculeKind, SequenceIdentifier, SequenceRecord};

    use super::{PrimerPair, PrimersearchError, primersearch_profile};

    fn dna_record(id: &str, residues: &str) -> SequenceRecord {
        SequenceRecord::new(
            SequenceIdentifier::new(id).expect("valid identifier"),
            MoleculeKind::Dna,
            residues,
        )
        .expect("dna record should build")
    }

    #[test]
    fn computes_forward_orientation_hits() {
        let record = dna_record("target", "CCCATGCGGGTTACCCC");
        let primer_pair =
            PrimerPair::new("pair1", "ATGC", "GTAA").expect("primer pair should build");

        let profile =
            primersearch_profile(&record, &[primer_pair]).expect("profile should compute");

        assert_eq!(profile.hits.len(), 1);
        let hit = &profile.hits[0];
        assert_eq!(hit.primer_pair_name, "pair1");
        assert_eq!(hit.strand, crate::Strand::Forward);
        assert_eq!(hit.left_primer_start, 4);
        assert_eq!(hit.left_primer_end, 7);
        assert_eq!(hit.right_primer_start, 11);
        assert_eq!(hit.right_primer_end, 14);
        assert_eq!(hit.amplicon_start, 4);
        assert_eq!(hit.amplicon_end, 14);
        assert_eq!(hit.amplicon_length, 11);
    }

    #[test]
    fn computes_reverse_orientation_hits() {
        let record = dna_record("target", "CCCGCATGGGTTAAAC");
        let primer_pair =
            PrimerPair::new("pair2", "ATGC", "TTAA").expect("primer pair should build");

        let profile =
            primersearch_profile(&record, &[primer_pair]).expect("profile should compute");

        assert_eq!(profile.hits.len(), 1);
        let hit = &profile.hits[0];
        assert_eq!(hit.primer_pair_name, "pair2");
        assert_eq!(hit.strand, crate::Strand::Reverse);
        assert_eq!(hit.left_primer_start, 4);
        assert_eq!(hit.left_primer_end, 7);
        assert_eq!(hit.right_primer_start, 11);
        assert_eq!(hit.right_primer_end, 14);
        assert_eq!(hit.left_matched, "GCAT");
        assert_eq!(hit.right_matched, "TTAA");
    }

    #[test]
    fn supports_iupac_ambiguity_in_primer_patterns() {
        let record = dna_record("target", "ATGCAAGT");
        let primer_pair =
            PrimerPair::new("pair3", "ATGC", "ACTT").expect("primer pair should build");
        let ambiguous = PrimerPair::new("pair4", "ATGN", "ACTT").expect("primer pair should build");

        let exact_profile =
            primersearch_profile(&record, &[primer_pair]).expect("exact profile should compute");
        let ambiguous_profile =
            primersearch_profile(&record, &[ambiguous]).expect("ambiguous profile should compute");

        assert_eq!(exact_profile.hits.len(), 1);
        assert_eq!(ambiguous_profile.hits.len(), 1);
        assert_eq!(ambiguous_profile.hits[0].left_matched, "ATGC");
    }

    #[test]
    fn rejects_protein_input() {
        let record = SequenceRecord::new(
            SequenceIdentifier::new("protein").expect("valid identifier"),
            MoleculeKind::Protein,
            "MSTN",
        )
        .expect("protein record should build");
        let primer_pair =
            PrimerPair::new("pair", "ATGC", "ACTT").expect("primer pair should build");

        let error =
            primersearch_profile(&record, &[primer_pair]).expect_err("protein input should fail");

        assert_eq!(error, PrimersearchError::NonNucleotideSequence);
    }

    #[test]
    fn rejects_empty_primer_pair_sets() {
        let record = dna_record("target", "ATGCAAGT");
        let error =
            primersearch_profile(&record, &[]).expect_err("empty primer-pair set should fail");
        assert_eq!(error, PrimersearchError::EmptyPrimerPairs);
    }

    #[test]
    fn rejects_invalid_primer_symbols() {
        let error = PrimerPair::new("pair", "ATG1", "ACTT")
            .expect_err("invalid primer symbols should fail");

        assert!(matches!(
            error,
            PrimersearchError::InvalidPrimerPattern {
                role: super::PrimerRole::Forward,
                ..
            }
        ));
    }
}

//! Small reusable codon-usage and coding-bias helpers.
//!
//! This module intentionally supports a narrow deterministic v1 scope:
//! - standard-code codon normalization and counting
//! - strict in-frame coding-sequence validation
//! - normalized codon-usage profile derivation
//! - CAI-style weight derivation and score calculation
//! - simple profile comparison helpers

use std::collections::BTreeMap;
use std::error::Error;
use std::fmt::{Display, Formatter};

/// Errors produced by codon-usage helpers.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum CodonUsageError {
    /// Coding length was not divisible by three.
    NonCodingLength {
        /// Observed sequence length.
        length: usize,
    },
    /// Encountered a codon with non-canonical content.
    InvalidCodon(String),
    /// Encountered an ambiguous codon that cannot be classified safely.
    AmbiguousCodon(String),
    /// Encountered an internal stop codon.
    InternalStopCodon(String),
}

impl Display for CodonUsageError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NonCodingLength { length } => {
                write!(
                    f,
                    "coding sequence length {length} is not divisible by three"
                )
            }
            Self::InvalidCodon(codon) => write!(f, "invalid codon '{codon}'"),
            Self::AmbiguousCodon(codon) => write!(f, "ambiguous codon '{codon}' is not supported"),
            Self::InternalStopCodon(codon) => {
                write!(f, "internal stop codon '{codon}' is not allowed")
            }
        }
    }
}

impl Error for CodonUsageError {}

/// Deterministic codon-usage profile over sense codons.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct CodonUsageProfile {
    counts: BTreeMap<String, usize>,
    total_sense_codons: usize,
}

impl CodonUsageProfile {
    /// Creates an empty codon-usage profile.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns sense-codon counts in stable sorted order.
    #[must_use]
    pub fn counts(&self) -> &BTreeMap<String, usize> {
        &self.counts
    }

    /// Returns the total number of counted sense codons.
    #[must_use]
    pub fn total_sense_codons(&self) -> usize {
        self.total_sense_codons
    }

    /// Returns the count for one codon.
    #[must_use]
    pub fn count_for(&self, codon: &str) -> usize {
        self.counts.get(codon).copied().unwrap_or_default()
    }

    /// Returns the frequency for one codon.
    #[must_use]
    pub fn frequency_for(&self, codon: &str) -> f64 {
        if self.total_sense_codons == 0 {
            return 0.0;
        }

        self.count_for(codon) as f64 / self.total_sense_codons as f64
    }

    /// Adds one counted sense codon.
    pub fn add_codon(&mut self, codon: &str) {
        *self.counts.entry(codon.to_owned()).or_insert(0) += 1;
        self.total_sense_codons += 1;
    }

    /// Merges another profile into this one.
    pub fn merge(&mut self, other: &Self) {
        for (codon, count) in &other.counts {
            *self.counts.entry(codon.clone()).or_insert(0) += count;
        }
        self.total_sense_codons += other.total_sense_codons;
    }
}

/// One validated coding-sequence codon summary.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CodingSequenceSummary {
    /// Profile over sense codons in the sequence.
    pub profile: CodonUsageProfile,
    /// Number of sense codons counted.
    pub sense_codon_count: usize,
    /// Optional terminal stop codon excluded from the profile.
    pub terminal_stop: Option<String>,
}

/// Counts codons in a strict coding sequence.
pub fn summarize_coding_sequence(
    nucleotide_sequence: &str,
) -> Result<CodingSequenceSummary, CodonUsageError> {
    if nucleotide_sequence.len() % 3 != 0 {
        return Err(CodonUsageError::NonCodingLength {
            length: nucleotide_sequence.len(),
        });
    }

    let mut profile = CodonUsageProfile::new();
    let codons = nucleotide_sequence
        .as_bytes()
        .chunks(3)
        .map(|chunk| {
            let codon = std::str::from_utf8(chunk)
                .expect("sequence records are normalized ASCII residues")
                .to_ascii_uppercase()
                .replace('U', "T");
            validate_codon(&codon)?;
            Ok(codon)
        })
        .collect::<Result<Vec<_>, CodonUsageError>>()?;

    let mut terminal_stop = None;
    for (index, codon) in codons.iter().enumerate() {
        if is_stop_codon(codon) {
            if index + 1 == codons.len() {
                terminal_stop = Some(codon.clone());
                continue;
            }

            return Err(CodonUsageError::InternalStopCodon(codon.clone()));
        }

        profile.add_codon(codon);
    }

    Ok(CodingSequenceSummary {
        sense_codon_count: profile.total_sense_codons(),
        profile,
        terminal_stop,
    })
}

/// Derives deterministic CAI weights from a reference profile.
#[must_use]
pub fn derive_cai_weights(reference: &CodonUsageProfile) -> BTreeMap<String, f64> {
    let mut grouped: BTreeMap<char, Vec<&'static str>> = BTreeMap::new();
    for (codon, amino_acid) in sense_codon_table() {
        grouped.entry(*amino_acid).or_default().push(*codon);
    }

    let mut weights = BTreeMap::new();
    for codons in grouped.values() {
        let max_count = codons
            .iter()
            .map(|codon| reference.count_for(codon))
            .max()
            .unwrap_or_default();

        for codon in codons {
            let weight = if max_count == 0 {
                0.0
            } else {
                reference.count_for(codon) as f64 / max_count as f64
            };
            weights.insert((*codon).to_owned(), weight);
        }
    }

    weights
}

/// Computes a deterministic CAI-like score for a target profile.
#[must_use]
pub fn cai_for_profile(profile: &CodonUsageProfile, weights: &BTreeMap<String, f64>) -> f64 {
    if profile.total_sense_codons() == 0 {
        return 0.0;
    }

    let mut weighted_count = 0usize;
    let mut log_sum = 0.0_f64;
    for (codon, count) in profile.counts() {
        let weight = weights.get(codon).copied().unwrap_or(0.0);
        if weight <= 0.0 {
            return 0.0;
        }
        log_sum += (*count as f64) * weight.ln();
        weighted_count += count;
    }

    if weighted_count == 0 {
        0.0
    } else {
        (log_sum / weighted_count as f64).exp()
    }
}

/// Computes total variation distance between two normalized profiles.
#[must_use]
pub fn total_variation_distance(left: &CodonUsageProfile, right: &CodonUsageProfile) -> f64 {
    sense_codon_table()
        .iter()
        .map(|(codon, _)| (left.frequency_for(codon) - right.frequency_for(codon)).abs())
        .sum::<f64>()
        * 0.5
}

/// Returns the amino-acid symbol for a sense codon.
#[must_use]
pub fn amino_acid_for_sense_codon(codon: &str) -> Option<char> {
    sense_codon_table()
        .iter()
        .find_map(|(entry_codon, amino_acid)| (*entry_codon == codon).then_some(*amino_acid))
}

/// Returns the canonical sense codons in stable order.
#[must_use]
pub fn sense_codons() -> Vec<&'static str> {
    sense_codon_table()
        .iter()
        .map(|(codon, _)| *codon)
        .collect()
}

fn validate_codon(codon: &str) -> Result<(), CodonUsageError> {
    if codon.len() != 3 {
        return Err(CodonUsageError::InvalidCodon(codon.to_owned()));
    }
    if codon
        .chars()
        .any(|symbol| !matches!(symbol, 'A' | 'C' | 'G' | 'T'))
    {
        if codon.chars().all(|symbol| {
            matches!(
                symbol,
                'A' | 'C'
                    | 'G'
                    | 'T'
                    | 'N'
                    | 'R'
                    | 'Y'
                    | 'S'
                    | 'W'
                    | 'K'
                    | 'M'
                    | 'B'
                    | 'D'
                    | 'H'
                    | 'V'
            )
        }) {
            return Err(CodonUsageError::AmbiguousCodon(codon.to_owned()));
        }
        return Err(CodonUsageError::InvalidCodon(codon.to_owned()));
    }
    Ok(())
}

fn is_stop_codon(codon: &str) -> bool {
    matches!(codon, "TAA" | "TAG" | "TGA")
}

fn sense_codon_table() -> &'static [(&'static str, char)] {
    &[
        ("GCT", 'A'),
        ("GCC", 'A'),
        ("GCA", 'A'),
        ("GCG", 'A'),
        ("CGT", 'R'),
        ("CGC", 'R'),
        ("CGA", 'R'),
        ("CGG", 'R'),
        ("AGA", 'R'),
        ("AGG", 'R'),
        ("AAT", 'N'),
        ("AAC", 'N'),
        ("GAT", 'D'),
        ("GAC", 'D'),
        ("TGT", 'C'),
        ("TGC", 'C'),
        ("CAA", 'Q'),
        ("CAG", 'Q'),
        ("GAA", 'E'),
        ("GAG", 'E'),
        ("GGT", 'G'),
        ("GGC", 'G'),
        ("GGA", 'G'),
        ("GGG", 'G'),
        ("CAT", 'H'),
        ("CAC", 'H'),
        ("ATT", 'I'),
        ("ATC", 'I'),
        ("ATA", 'I'),
        ("TTA", 'L'),
        ("TTG", 'L'),
        ("CTT", 'L'),
        ("CTC", 'L'),
        ("CTA", 'L'),
        ("CTG", 'L'),
        ("AAA", 'K'),
        ("AAG", 'K'),
        ("ATG", 'M'),
        ("TTT", 'F'),
        ("TTC", 'F'),
        ("CCT", 'P'),
        ("CCC", 'P'),
        ("CCA", 'P'),
        ("CCG", 'P'),
        ("TCT", 'S'),
        ("TCC", 'S'),
        ("TCA", 'S'),
        ("TCG", 'S'),
        ("AGT", 'S'),
        ("AGC", 'S'),
        ("ACT", 'T'),
        ("ACC", 'T'),
        ("ACA", 'T'),
        ("ACG", 'T'),
        ("TGG", 'W'),
        ("TAT", 'Y'),
        ("TAC", 'Y'),
        ("GTT", 'V'),
        ("GTC", 'V'),
        ("GTA", 'V'),
        ("GTG", 'V'),
    ]
}

#[cfg(test)]
mod tests {
    use super::{
        cai_for_profile, derive_cai_weights, sense_codons, summarize_coding_sequence,
        total_variation_distance, CodonUsageError, CodonUsageProfile,
    };

    #[test]
    fn summarizes_coding_sequence_and_excludes_terminal_stop() {
        let summary = summarize_coding_sequence("ATGGCTTAA").expect("coding sequence");
        assert_eq!(summary.sense_codon_count, 2);
        assert_eq!(summary.profile.count_for("ATG"), 1);
        assert_eq!(summary.profile.count_for("GCT"), 1);
        assert_eq!(summary.terminal_stop.as_deref(), Some("TAA"));
    }

    #[test]
    fn rejects_internal_stop() {
        let error = summarize_coding_sequence("ATGTAAGCT").expect_err("internal stop should fail");
        assert_eq!(error, CodonUsageError::InternalStopCodon("TAA".to_owned()));
    }

    #[test]
    fn derives_weights_and_scores_cai() {
        let reference = summarize_coding_sequence("CTTCTTTTACTA").expect("reference");
        let query_preferred = summarize_coding_sequence("CTTCTT").expect("query");
        let query_rare = summarize_coding_sequence("TTATTA").expect("query");
        let weights = derive_cai_weights(&reference.profile);

        let preferred_score = cai_for_profile(&query_preferred.profile, &weights);
        let rare_score = cai_for_profile(&query_rare.profile, &weights);

        assert!(preferred_score > rare_score);
        assert!(preferred_score > 0.0);
    }

    #[test]
    fn computes_total_variation_distance() {
        let left = summarize_coding_sequence("CTTCTTATG").expect("left");
        let right =
            summarize_coding_sequence("TTATT AATG".replace(' ', "").as_str()).expect("right");
        let distance = total_variation_distance(&left.profile, &right.profile);
        assert!(distance > 0.0);
    }

    #[test]
    fn exposes_stable_sense_codon_catalogue() {
        let codons = sense_codons();
        assert_eq!(codons.len(), 61);
        assert_eq!(codons[0], "GCT");
    }

    #[test]
    fn zero_weight_reference_produces_zero_cai() {
        let weights = derive_cai_weights(&CodonUsageProfile::new());
        let query = summarize_coding_sequence("ATGGCT").expect("query");
        assert_eq!(cai_for_profile(&query.profile, &weights), 0.0);
    }
}

//! `codcmp` implementation.

use std::path::PathBuf;

use epithema_core::{
    CodonUsageProfile, amino_acid_for_sense_codon, sense_codons, total_variation_distance,
};

use crate::codon_tools::shared::load_profile_source;
use crate::sequence_stream::ToolExecutionError;

/// Parameters for a `codcmp` execution.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CodcmpParams {
    /// Left coding-sequence or normalized codon-profile input.
    pub left: PathBuf,
    /// Right coding-sequence or normalized codon-profile input.
    pub right: PathBuf,
}

/// One codon comparison row in a `codcmp` report.
#[derive(Clone, Debug, PartialEq)]
pub struct CodcmpRow {
    /// Sense codon being compared.
    pub codon: String,
    /// Amino acid encoded by the codon.
    pub amino_acid: char,
    /// Observed codon count in the left profile.
    pub left_count: usize,
    /// Codon frequency in the left profile.
    pub left_frequency: f64,
    /// Observed codon count in the right profile.
    pub right_count: usize,
    /// Codon frequency in the right profile.
    pub right_frequency: f64,
    /// Left frequency minus right frequency.
    pub delta_frequency: f64,
}

/// Complete outcome from a `codcmp` execution.
#[derive(Clone, Debug, PartialEq)]
pub struct CodcmpOutcome {
    /// Left input used for comparison.
    pub left: PathBuf,
    /// Right input used for comparison.
    pub right: PathBuf,
    /// Per-codon comparison rows over sense codons.
    pub rows: Vec<CodcmpRow>,
    /// Aggregate total variation distance over sense codon frequencies.
    pub total_variation_distance: f64,
}

/// Returns command-line help text for `codcmp`.
#[must_use]
pub fn codcmp_help() -> &'static str {
    "Usage: epithema codcmp <left-input> <right-input>\n\nCompare normalized codon usage between two strict coding-sequence inputs or normalized codon-profile inputs. The v1 report includes codon-by-codon counts, frequencies, frequency deltas, and aggregate total variation distance over the 61 sense codons."
}

/// Runs codon-profile comparison for two coding or profile inputs.
pub fn run_codcmp(params: CodcmpParams) -> Result<CodcmpOutcome, ToolExecutionError> {
    let left_profile = load_profile_source(&params.left)?;
    let right_profile = load_profile_source(&params.right)?;
    let rows = compare_profiles(&left_profile, &right_profile);
    let total_variation_distance = total_variation_distance(&left_profile, &right_profile);

    Ok(CodcmpOutcome {
        left: params.left,
        right: params.right,
        rows,
        total_variation_distance,
    })
}

fn compare_profiles(left: &CodonUsageProfile, right: &CodonUsageProfile) -> Vec<CodcmpRow> {
    sense_codons()
        .into_iter()
        .map(|codon| {
            let left_frequency = left.frequency_for(codon);
            let right_frequency = right.frequency_for(codon);
            CodcmpRow {
                codon: codon.to_owned(),
                amino_acid: amino_acid_for_sense_codon(codon)
                    .expect("sense codon should have amino acid"),
                left_count: left.count_for(codon),
                left_frequency,
                right_count: right.count_for(codon),
                right_frequency,
                delta_frequency: left_frequency - right_frequency,
            }
        })
        .collect()
}

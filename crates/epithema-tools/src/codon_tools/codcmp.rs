//! `codcmp` implementation.

use std::path::PathBuf;

use epithema_core::{
    CodonUsageProfile, amino_acid_for_sense_codon, sense_codons, total_variation_distance,
};

use crate::codon_tools::shared::load_profile_source;
use crate::sequence_stream::ToolExecutionError;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CodcmpParams {
    pub left: PathBuf,
    pub right: PathBuf,
}

#[derive(Clone, Debug, PartialEq)]
pub struct CodcmpRow {
    pub codon: String,
    pub amino_acid: char,
    pub left_count: usize,
    pub left_frequency: f64,
    pub right_count: usize,
    pub right_frequency: f64,
    pub delta_frequency: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct CodcmpOutcome {
    pub left: PathBuf,
    pub right: PathBuf,
    pub rows: Vec<CodcmpRow>,
    pub total_variation_distance: f64,
}

#[must_use]
pub fn codcmp_help() -> &'static str {
    "Usage: epithema codcmp <left-input> <right-input>\n\nCompare normalized codon usage between two strict coding-sequence inputs or normalized codon-profile inputs. The v1 report includes codon-by-codon counts, frequencies, frequency deltas, and aggregate total variation distance over the 61 sense codons."
}

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

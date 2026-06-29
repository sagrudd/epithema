//! `cai` implementation.

use std::path::PathBuf;

use epithema_core::{cai_for_profile, derive_cai_weights};

use crate::codon_tools::shared::{derive_coding_profile_records, load_profile_source};
use crate::sequence_stream::{SequenceInput, ToolExecutionError};

/// Parameters for a `cai` execution.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CaiParams {
    /// Coding nucleotide sequence input to score.
    pub input: SequenceInput,
    /// Reference coding-sequence or normalized codon-profile input.
    pub reference: PathBuf,
}

/// CAI result for one input record.
#[derive(Clone, Debug, PartialEq)]
pub struct CaiCase {
    /// Stable record identifier from the source sequence.
    pub record_id: String,
    /// Number of sense codons contributing to the score.
    pub sense_codon_count: usize,
    /// Optional terminal stop codon excluded from the score.
    pub terminal_stop: Option<String>,
    /// Deterministic CAI-like score derived from reference weights.
    pub cai: f64,
}

/// Complete outcome from a `cai` execution.
#[derive(Clone, Debug, PartialEq)]
pub struct CaiOutcome {
    /// Coding nucleotide sequence input that was scored.
    pub input: SequenceInput,
    /// Reference input used to derive codon weights.
    pub reference: PathBuf,
    /// Per-record CAI results.
    pub cases: Vec<CaiCase>,
}

/// Returns command-line help text for `cai`.
#[must_use]
pub fn cai_help() -> &'static str {
    "Usage: epithema cai <coding-input> <reference-input>\n\nReport deterministic CAI-like values for strict in-frame coding nucleotide sequences. The reference input may be either another coding-sequence file or a normalized codon profile emitted by codcopy. CAI weights are derived as codon_count / max_synonymous_count per amino-acid class, stop codons are excluded, and any target codon with zero reference weight yields CAI 0.0."
}

/// Runs deterministic CAI scoring for strict coding nucleotide sequences.
pub fn run_cai(params: CaiParams) -> Result<CaiOutcome, ToolExecutionError> {
    let reference_profile = load_profile_source(&params.reference)?;
    let weights = derive_cai_weights(&reference_profile);
    let cases = derive_coding_profile_records(&params.input)?
        .into_iter()
        .map(|record| CaiCase {
            record_id: record.record_id,
            sense_codon_count: record.sense_codon_count,
            terminal_stop: record.terminal_stop,
            cai: cai_for_profile(&record.profile, &weights),
        })
        .collect();

    Ok(CaiOutcome {
        input: params.input,
        reference: params.reference,
        cases,
    })
}

//! `cai` implementation.

use std::path::PathBuf;

use epithema_core::{cai_for_profile, derive_cai_weights};

use crate::codon_tools::shared::{derive_coding_profile_records, load_profile_source};
use crate::sequence_stream::{SequenceInput, ToolExecutionError};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CaiParams {
    pub input: SequenceInput,
    pub reference: PathBuf,
}

#[derive(Clone, Debug, PartialEq)]
pub struct CaiCase {
    pub record_id: String,
    pub sense_codon_count: usize,
    pub terminal_stop: Option<String>,
    pub cai: f64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct CaiOutcome {
    pub input: SequenceInput,
    pub reference: PathBuf,
    pub cases: Vec<CaiCase>,
}

#[must_use]
pub fn cai_help() -> &'static str {
    "Usage: epithema cai <coding-input> <reference-input>\n\nReport deterministic CAI-like values for strict in-frame coding nucleotide sequences. The reference input may be either another coding-sequence file or a normalized codon profile emitted by codcopy. CAI weights are derived as codon_count / max_synonymous_count per amino-acid class, stop codons are excluded, and any target codon with zero reference weight yields CAI 0.0."
}

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

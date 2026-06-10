//! `chips` implementation.

use epithema_core::CodonUsageProfile;

use crate::codon_tools::shared::{
    CodingProfileRecord, aggregate_profile, derive_coding_profile_records,
};
use crate::sequence_stream::{SequenceInput, ToolExecutionError};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ChipsParams {
    pub input: SequenceInput,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ChipsOutcome {
    pub input: SequenceInput,
    pub records: Vec<CodingProfileRecord>,
    pub aggregate: CodonUsageProfile,
}

#[must_use]
pub fn chips_help() -> &'static str {
    "Usage: epithema chips <coding-input>\n\nReport deterministic codon-usage counts and frequencies for strict in-frame coding nucleotide sequences. A single terminal stop codon is allowed and excluded from codon-profile counts. Internal stops, ambiguous codons, and non-triplet sequence lengths are rejected."
}

pub fn run_chips(params: ChipsParams) -> Result<ChipsOutcome, ToolExecutionError> {
    let records = derive_coding_profile_records(&params.input)?;
    let aggregate = aggregate_profile(&records);
    Ok(ChipsOutcome {
        input: params.input,
        records,
        aggregate,
    })
}

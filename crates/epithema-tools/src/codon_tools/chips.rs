//! `chips` implementation.

use epithema_core::CodonUsageProfile;

use crate::codon_tools::shared::{
    CodingProfileRecord, aggregate_profile, derive_coding_profile_records,
};
use crate::sequence_stream::{SequenceInput, ToolExecutionError};

/// Parameters for a `chips` execution.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ChipsParams {
    /// Coding nucleotide sequence input to profile.
    pub input: SequenceInput,
}

/// Complete outcome from a `chips` execution.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ChipsOutcome {
    /// Coding nucleotide sequence input that was profiled.
    pub input: SequenceInput,
    /// Per-record coding profile summaries.
    pub records: Vec<CodingProfileRecord>,
    /// Aggregate codon-usage profile across all records.
    pub aggregate: CodonUsageProfile,
}

/// Returns command-line help text for `chips`.
#[must_use]
pub fn chips_help() -> &'static str {
    "Usage: epithema chips <coding-input>\n\nReport deterministic codon-usage counts and frequencies for strict in-frame coding nucleotide sequences. A single terminal stop codon is allowed and excluded from codon-profile counts. Internal stops, ambiguous codons, and non-triplet sequence lengths are rejected."
}

/// Runs codon-usage profiling for strict coding nucleotide sequences.
pub fn run_chips(params: ChipsParams) -> Result<ChipsOutcome, ToolExecutionError> {
    let records = derive_coding_profile_records(&params.input)?;
    let aggregate = aggregate_profile(&records);
    Ok(ChipsOutcome {
        input: params.input,
        records,
        aggregate,
    })
}

//! `cusp` implementation.

use emboss_core::CodonUsageProfile;

use crate::codon_tools::shared::{
    CodingProfileRecord, aggregate_profile, derive_coding_profile_records,
};
use crate::sequence_stream::{SequenceInput, ToolExecutionError};

/// Typed parameters for `cusp`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CuspParams {
    /// Local coding-sequence input path.
    pub input: SequenceInput,
}

/// Structured `cusp` outcome.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CuspOutcome {
    /// Source input.
    pub input: SequenceInput,
    /// Per-record codon profiles.
    pub records: Vec<CodingProfileRecord>,
    /// Aggregate codon profile across all records.
    pub aggregate: CodonUsageProfile,
}

/// Returns `cusp` help text.
#[must_use]
pub fn cusp_help() -> &'static str {
    "Usage: emboss-rs cusp <coding-input>\n\nCreate a deterministic codon-usage table from strict in-frame coding nucleotide sequences. v1 reports one complete 61-sense-codon table for each input record plus one aggregate table across all records, allows at most one terminal stop codon per record, and rejects ambiguous or internally stopped codons."
}

/// Executes `cusp`.
pub fn run_cusp(params: CuspParams) -> Result<CuspOutcome, ToolExecutionError> {
    let records = derive_coding_profile_records(&params.input)?;
    let aggregate = aggregate_profile(&records);
    Ok(CuspOutcome {
        input: params.input,
        records,
        aggregate,
    })
}

#[cfg(test)]
mod tests {
    use super::{CuspParams, run_cusp};
    use crate::sequence_stream::SequenceInput;

    #[test]
    fn reports_per_record_and_aggregate_codon_profiles() {
        let outcome = run_cusp(CuspParams {
            input: SequenceInput::new(
                "/Users/stephen/Projects/emboss-rs/crates/emboss-tools/tests/fixtures/codon_reference.fasta",
            ),
        })
        .expect("cusp should execute");

        assert_eq!(outcome.records.len(), 2);
        assert_eq!(outcome.records[0].record_id, "ref_pref");
        assert_eq!(outcome.records[0].profile.count_for("CTT"), 3);
        assert_eq!(outcome.records[1].profile.count_for("CTA"), 1);
        assert_eq!(outcome.aggregate.count_for("ATG"), 2);
    }
}

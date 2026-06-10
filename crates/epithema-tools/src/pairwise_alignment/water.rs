//! `water` implementation.

use epithema_core::AlignmentMode;

use super::shared::{load_singleton_record, local_align_pair};
use crate::sequence_stream::{SequenceInput, ToolExecutionError};

/// Typed parameters for `water`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WaterParams {
    /// Singleton query sequence input.
    pub query: SequenceInput,
    /// Singleton target sequence input.
    pub target: SequenceInput,
    /// Optional gap-open penalty override.
    pub gap_open: Option<i32>,
    /// Optional gap-extend penalty override.
    pub gap_extend: Option<i32>,
}

/// Structured `water` outcome.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WaterOutcome {
    /// Source query input.
    pub query: SequenceInput,
    /// Source target input.
    pub target: SequenceInput,
    /// Gap-open penalty used.
    pub gap_open: i32,
    /// Gap-extend penalty used.
    pub gap_extend: i32,
    /// Completed local pairwise alignment result.
    pub result: epithema_core::LocalPairwiseAlignmentResult,
}

/// Returns `water` help text.
#[must_use]
pub fn water_help() -> &'static str {
    "Usage: epithema water <query-input> <target-input> [--gap-open <penalty>] [--gap-extend <penalty>]\n\nPerform deterministic local pairwise alignment between exactly one query record and one target record. Epithema v1 uses a Smith-Waterman-style affine-gap alignment core, reports the highest-scoring local region only, and renders the primary payload as Stockholm."
}

/// Executes `water`.
pub fn run_water(params: WaterParams) -> Result<WaterOutcome, ToolExecutionError> {
    let query = load_singleton_record(&params.query, "water", "query")?;
    let target = load_singleton_record(&params.target, "water", "target")?;
    let result = local_align_pair(&query, &target, params.gap_open, params.gap_extend)?;
    let defaults = default_penalties(result.summary.mode);

    Ok(WaterOutcome {
        query: params.query,
        target: params.target,
        gap_open: params.gap_open.unwrap_or(defaults.0),
        gap_extend: params.gap_extend.unwrap_or(defaults.1),
        result,
    })
}

fn default_penalties(mode: AlignmentMode) -> (i32, i32) {
    match mode {
        AlignmentMode::Nucleotide => (5, 1),
        AlignmentMode::Protein => (8, 1),
    }
}

#[cfg(test)]
mod tests {
    use super::{WaterParams, run_water};
    use crate::sequence_stream::SequenceInput;

    fn fixture(name: &str) -> SequenceInput {
        SequenceInput::new(
            std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("tests/fixtures")
                .join(name)
                .display()
                .to_string(),
        )
    }

    #[test]
    fn computes_best_local_alignment_for_singleton_inputs() {
        let outcome = run_water(WaterParams {
            query: fixture("water_query.fasta"),
            target: fixture("water_target.fasta"),
            gap_open: None,
            gap_extend: None,
        })
        .expect("water should succeed");

        assert_eq!(outcome.result.alignment.rows()[0].aligned(), "ACGTA");
        assert_eq!(outcome.result.alignment.rows()[1].aligned(), "ACGTA");
        assert_eq!(outcome.result.summary.query_start, 2);
        assert_eq!(outcome.result.summary.query_end, 7);
        assert_eq!(outcome.result.summary.target_start, 2);
        assert_eq!(outcome.result.summary.target_end, 7);
    }

    #[test]
    fn rejects_incompatible_input_molecules() {
        let error = run_water(WaterParams {
            query: fixture("matcher_query.fasta"),
            target: fixture("protein_records.fasta"),
            gap_open: None,
            gap_extend: None,
        })
        .expect_err("incompatible molecules should fail");
        assert!(error.code().is_some());
    }
}

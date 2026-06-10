//! `needleall` implementation.

use epithema_core::AlignmentMode;
use epithema_diagnostics::{ErrorCategory, PlatformError};

use super::shared::align_pair;
use crate::sequence_stream::{SequenceInput, ToolExecutionError, load_sequence_records};

/// Typed parameters for `needleall`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NeedleallParams {
    /// Query sequence-set input.
    pub query: SequenceInput,
    /// Target sequence-set input.
    pub target: SequenceInput,
    /// Optional gap-open penalty override.
    pub gap_open: Option<i32>,
    /// Optional gap-extend penalty override.
    pub gap_extend: Option<i32>,
}

/// One comparison row from `needleall`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NeedleallCase {
    /// Query record identifier.
    pub query_id: String,
    /// Target record identifier.
    pub target_id: String,
    /// Scoring mode used.
    pub mode: AlignmentMode,
    /// Alignment score.
    pub score: i32,
    /// Aligned length.
    pub aligned_length: usize,
    /// Identity count.
    pub identity_count: usize,
    /// Integer identity percentage over aligned columns.
    pub identity_percent: usize,
    /// Query gap count.
    pub query_gap_count: usize,
    /// Target gap count.
    pub target_gap_count: usize,
}

/// Structured `needleall` outcome.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NeedleallOutcome {
    /// Source query input.
    pub query: SequenceInput,
    /// Source target input.
    pub target: SequenceInput,
    /// Gap-open penalty override used when present.
    pub gap_open: Option<i32>,
    /// Gap-extend penalty override used when present.
    pub gap_extend: Option<i32>,
    /// Deterministic query-major comparison cases.
    pub cases: Vec<NeedleallCase>,
}

/// Returns `needleall` help text.
#[must_use]
pub fn needleall_help() -> &'static str {
    "Usage: epithema needleall <query-input> <target-input> [--gap-open <penalty>] [--gap-extend <penalty>]\n\nPerform deterministic many-vs-many global pairwise alignment between all records in the query input and all records in the target input. Comparisons run in query-major then target-major order. The CLI reports a structured summary table rather than emitting every alignment."
}

/// Executes `needleall`.
pub fn run_needleall(params: NeedleallParams) -> Result<NeedleallOutcome, ToolExecutionError> {
    let query_records = load_sequence_records(&params.query)?;
    let target_records = load_sequence_records(&params.target)?;

    if query_records.is_empty() || target_records.is_empty() {
        return Err(PlatformError::new(
            ErrorCategory::Validation,
            "needleall requires non-empty query and target sequence sets",
        )
        .with_code("tools.needleall.inputs.empty"));
    }

    let mut cases = Vec::new();
    for query in &query_records {
        for target in &target_records {
            let result = align_pair(query, target, params.gap_open, params.gap_extend)?;
            cases.push(NeedleallCase {
                query_id: query.identifier().accession().to_owned(),
                target_id: target.identifier().accession().to_owned(),
                mode: result.summary.mode,
                score: result.summary.score,
                aligned_length: result.summary.aligned_length,
                identity_count: result.summary.identity_count,
                identity_percent: result.summary.identity_percent,
                query_gap_count: result.summary.query_gap_count,
                target_gap_count: result.summary.target_gap_count,
            });
        }
    }

    Ok(NeedleallOutcome {
        query: params.query,
        target: params.target,
        gap_open: params.gap_open,
        gap_extend: params.gap_extend,
        cases,
    })
}

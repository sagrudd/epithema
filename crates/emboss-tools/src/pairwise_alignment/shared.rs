use emboss_core::{
    AlignmentMode, GlobalAlignmentError, GlobalAlignmentScoring, PairwiseAlignmentResult,
    SequenceRecord, global_align, infer_alignment_mode,
};
use emboss_diagnostics::{ErrorCategory, PlatformError};

use crate::sequence_stream::{SequenceInput, ToolExecutionError, load_sequence_records};

pub fn scoring_for_pair(
    query: &SequenceRecord,
    target: &SequenceRecord,
    gap_open: Option<i32>,
    gap_extend: Option<i32>,
) -> Result<GlobalAlignmentScoring, ToolExecutionError> {
    let mode = infer_alignment_mode(query, target).map_err(map_global_alignment_error)?;
    let mut scoring = match mode {
        AlignmentMode::Nucleotide => GlobalAlignmentScoring::nucleotide_default(),
        AlignmentMode::Protein => GlobalAlignmentScoring::protein_default(),
    };
    if let Some(gap_open) = gap_open {
        scoring.gap_open = gap_open;
    }
    if let Some(gap_extend) = gap_extend {
        scoring.gap_extend = gap_extend;
    }
    Ok(scoring)
}

pub fn align_pair(
    query: &SequenceRecord,
    target: &SequenceRecord,
    gap_open: Option<i32>,
    gap_extend: Option<i32>,
) -> Result<PairwiseAlignmentResult, ToolExecutionError> {
    let scoring = scoring_for_pair(query, target, gap_open, gap_extend)?;
    global_align(query, target, scoring).map_err(map_global_alignment_error)
}

pub fn load_singleton_record(
    input: &SequenceInput,
    tool: &str,
    side: &str,
) -> Result<SequenceRecord, ToolExecutionError> {
    let mut records = load_sequence_records(input)?;
    if records.len() != 1 {
        return Err(PlatformError::new(
            ErrorCategory::Validation,
            format!("{tool} requires exactly one {side} sequence record"),
        )
        .with_code(format!("tools.{tool}.{side}.singleton_required")));
    }
    Ok(records.remove(0))
}

pub fn map_global_alignment_error(error: GlobalAlignmentError) -> ToolExecutionError {
    let code = match error {
        GlobalAlignmentError::IncompatibleMolecules { .. } => {
            "tools.global_alignment.input.incompatible_molecules"
        }
        GlobalAlignmentError::InvalidGapPenalty { .. } => {
            "tools.global_alignment.gap_penalty.invalid"
        }
    };
    PlatformError::new(ErrorCategory::Validation, error.to_string()).with_code(code)
}

use std::path::Path;

use emboss_core::{
    Alignment, AlignmentAnalysisError, ConsensusStrategy, DistanceMatrix, SequenceIdentifier,
    SequenceRecord, consensus_sequence, direct_match_summary, p_distance_matrix,
};
use emboss_diagnostics::{ErrorCategory, PlatformError};

use crate::alignment_tools::{AlignmentInput, load_alignment};
use crate::sequence_stream::{SequenceInput, ToolExecutionError, load_sequence_records};

pub fn load_singleton_sequence(
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

pub fn compare_sequences(
    query: &SequenceRecord,
    target: &SequenceRecord,
) -> Result<emboss_core::DirectMatchSummary, ToolExecutionError> {
    direct_match_summary(query, target).map_err(map_analysis_error)
}

pub fn build_distance_matrix(
    records: &[SequenceRecord],
) -> Result<DistanceMatrix, ToolExecutionError> {
    p_distance_matrix(records).map_err(map_analysis_error)
}

pub fn load_consensus_input(input: &AlignmentInput) -> Result<Alignment, ToolExecutionError> {
    load_alignment(input)
}

pub fn derive_consensus(
    alignment: &Alignment,
    strategy: ConsensusStrategy,
    suffix: &str,
    path: &Path,
) -> Result<SequenceRecord, ToolExecutionError> {
    let stem = alignment
        .identifier()
        .map(ToOwned::to_owned)
        .or_else(|| {
            path.file_stem()
                .and_then(|value| value.to_str())
                .map(ToOwned::to_owned)
        })
        .unwrap_or_else(|| "alignment".to_owned());
    let identifier =
        SequenceIdentifier::new(format!("{stem}.{suffix}")).expect("tool-derived identifier valid");
    consensus_sequence(alignment, strategy, identifier).map_err(map_analysis_error)
}

pub fn map_analysis_error(error: AlignmentAnalysisError) -> ToolExecutionError {
    let code = match error {
        AlignmentAnalysisError::IncompatibleMolecules { .. } => {
            "tools.alignment_analysis.input.incompatible_molecules"
        }
        AlignmentAnalysisError::EmptySequenceSet => "tools.distmat.input.empty",
        AlignmentAnalysisError::UnequalSequenceLengths { .. } => {
            "tools.distmat.input.length_mismatch"
        }
        AlignmentAnalysisError::MixedAlignmentMolecules => "tools.consensus.input.mixed_molecules",
    };
    PlatformError::new(ErrorCategory::Validation, error.to_string()).with_code(code)
}

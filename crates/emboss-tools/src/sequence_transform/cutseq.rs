//! `cutseq` implementation.

use emboss_core::{SequenceIdentifier, SequenceRecord};
use emboss_diagnostics::{ErrorCategory, PlatformError};

use crate::sequence_stream::{SequenceInput, ToolExecutionError, load_sequence_records};

/// Typed parameters for `cutseq`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CutseqParams {
    /// Local sequence input path.
    pub input: SequenceInput,
    /// 1-based cut position after which the sequence is split.
    pub cut_position: usize,
}

/// Structured `cutseq` outcome.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CutseqOutcome {
    /// Source input.
    pub input: SequenceInput,
    /// 1-based cut position.
    pub cut_position: usize,
    /// Left/right fragments in input order.
    pub records: Vec<SequenceRecord>,
}

/// Returns `cutseq` help text.
#[must_use]
pub fn cutseq_help() -> &'static str {
    "Usage: emboss-rs cutseq <input> <position>\n\nCut each sequence record after the supplied 1-based interior position. The cut position belongs to the left fragment, and the position must be between 1 and length-1 for every record."
}

/// Executes `cutseq`.
pub fn run_cutseq(params: CutseqParams) -> Result<CutseqOutcome, ToolExecutionError> {
    if params.cut_position == 0 {
        return Err(PlatformError::new(
            ErrorCategory::Validation,
            "cutseq uses 1-based cut positions and requires position >= 1",
        )
        .with_code("tools.cutseq.position.invalid"));
    }

    let mut records = Vec::new();
    for record in load_sequence_records(&params.input)? {
        let [left, right] = split_record(record, params.cut_position)?;
        records.push(left);
        records.push(right);
    }

    Ok(CutseqOutcome {
        input: params.input,
        cut_position: params.cut_position,
        records,
    })
}

fn split_record(
    record: SequenceRecord,
    cut_position: usize,
) -> Result<[SequenceRecord; 2], ToolExecutionError> {
    if cut_position >= record.len() {
        return Err(PlatformError::new(
            ErrorCategory::Validation,
            format!(
                "cut position {cut_position} must be an interior position for sequence '{}' of length {}",
                record.identifier().accession(),
                record.len()
            ),
        )
        .with_code("tools.cutseq.position.out_of_range"));
    }

    let left = &record.residues()[..cut_position];
    let right = &record.residues()[cut_position..];
    let molecule = record.molecule();
    let metadata = record.metadata().clone();
    let base = record.identifier().accession();

    Ok([
        build_fragment(base, "left", molecule, left, metadata.clone())?,
        build_fragment(base, "right", molecule, right, metadata)?,
    ])
}

fn build_fragment(
    base: &str,
    suffix: &str,
    molecule: emboss_core::MoleculeKind,
    residues: &str,
    metadata: emboss_core::SequenceMetadata,
) -> Result<SequenceRecord, ToolExecutionError> {
    let identifier = SequenceIdentifier::new(format!("{base}.{suffix}")).map_err(|error| {
        PlatformError::new(ErrorCategory::Validation, error.to_string())
            .with_code("tools.cutseq.identifier.invalid")
    })?;
    let mut record = SequenceRecord::new(identifier, molecule, residues).map_err(|error| {
        PlatformError::new(ErrorCategory::Validation, error.to_string())
            .with_code("tools.cutseq.record.invalid")
    })?;
    record = record.with_metadata(metadata);
    Ok(record)
}

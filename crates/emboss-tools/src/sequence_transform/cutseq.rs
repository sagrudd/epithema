//! `cutseq` implementation.

use emboss_core::{Interval, SequenceIdentifier, SequenceRecord};
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
    "Usage: emboss-rs cutseq <input> <position>\n\nSplit each sequence record after the supplied 1-based interior position. The cut position belongs to the left fragment, and the same cut is applied to every input record. Position must be between 1 and length-1 for every record, so `cutseq` always emits exactly two non-empty fragments per input record."
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

    let left = subsequence_for_interval(
        &record,
        Interval::new(0, cut_position).map_err(|error| {
            PlatformError::new(ErrorCategory::Validation, error.to_string())
                .with_code("tools.cutseq.interval.invalid")
        })?,
    )?;
    let right = subsequence_for_interval(
        &record,
        Interval::new(cut_position, record.len()).map_err(|error| {
            PlatformError::new(ErrorCategory::Validation, error.to_string())
                .with_code("tools.cutseq.interval.invalid")
        })?,
    )?;
    let molecule = record.molecule();
    let metadata = record.metadata().clone();
    let base = record.identifier().accession();

    Ok([
        build_fragment(base, "left", molecule, &left, metadata.clone())?,
        build_fragment(base, "right", molecule, &right, metadata)?,
    ])
}

fn subsequence_for_interval(
    record: &SequenceRecord,
    interval: Interval,
) -> Result<String, ToolExecutionError> {
    record
        .subsequence(interval)
        .map(str::to_owned)
        .map_err(|error| {
            PlatformError::new(ErrorCategory::Validation, error.to_string())
                .with_code("tools.cutseq.interval.invalid")
        })
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

#[cfg(test)]
mod tests {
    use emboss_core::{MoleculeKind, SequenceIdentifier};

    use super::{CutseqParams, run_cutseq, split_record};
    use crate::sequence_stream::SequenceInput;

    fn record(id: &str, residues: &str) -> emboss_core::SequenceRecord {
        emboss_core::SequenceRecord::new(
            SequenceIdentifier::new(id).expect("valid identifier"),
            MoleculeKind::Dna,
            residues,
        )
        .expect("valid sequence")
    }

    #[test]
    fn splits_record_at_interior_position() {
        let [left, right] = split_record(record("seq1", "ACGT"), 2).expect("splits");
        assert_eq!(left.identifier().accession(), "seq1.left");
        assert_eq!(left.residues(), "AC");
        assert_eq!(right.identifier().accession(), "seq1.right");
        assert_eq!(right.residues(), "GT");
    }

    #[test]
    fn supports_boundary_cut_after_first_position() {
        let [left, right] = split_record(record("seq1", "ACGT"), 1).expect("splits");
        assert_eq!(left.residues(), "A");
        assert_eq!(right.residues(), "CGT");
    }

    #[test]
    fn supports_boundary_cut_before_last_position() {
        let [left, right] = split_record(record("seq1", "ACGT"), 3).expect("splits");
        assert_eq!(left.residues(), "ACG");
        assert_eq!(right.residues(), "T");
    }

    #[test]
    fn rejects_out_of_range_cut_position() {
        let error = split_record(record("seq1", "ACGT"), 4).expect_err("must fail");
        assert_eq!(error.code(), Some("tools.cutseq.position.out_of_range"));
    }

    #[test]
    fn rejects_zero_cut_position_before_reading_input() {
        let error = run_cutseq(CutseqParams {
            input: SequenceInput::new("unused.fa"),
            cut_position: 0,
        })
        .expect_err("must fail");
        assert_eq!(error.code(), Some("tools.cutseq.position.invalid"));
    }
}

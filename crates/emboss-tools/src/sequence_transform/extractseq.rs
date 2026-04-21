//! `extractseq` implementation.

use emboss_core::{Interval, SequenceRecord};
use emboss_diagnostics::{ErrorCategory, PlatformError};

use crate::sequence_stream::{SequenceInput, ToolExecutionError, load_sequence_records};

/// Typed parameters for `extractseq`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExtractseqParams {
    /// Local sequence input path.
    pub input: SequenceInput,
    /// 1-based inclusive start.
    pub start: usize,
    /// 1-based inclusive end.
    pub end: usize,
}

/// Structured `extractseq` outcome.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExtractseqOutcome {
    /// Source input.
    pub input: SequenceInput,
    /// 1-based inclusive start.
    pub start: usize,
    /// 1-based inclusive end.
    pub end: usize,
    /// Extracted subsequences in input order.
    pub records: Vec<SequenceRecord>,
}

/// Returns `extractseq` help text.
#[must_use]
pub fn extractseq_help() -> &'static str {
    "Usage: emboss-rs extractseq <input> <start> <end>\n\nExtract the same 1-based inclusive region from each sequence record in a local FASTA, FASTQ, EMBL, or GenBank file. The start and end coordinates are both inclusive. The requested interval must lie within every input record; otherwise the run fails clearly."
}

/// Executes `extractseq`.
pub fn run_extractseq(params: ExtractseqParams) -> Result<ExtractseqOutcome, ToolExecutionError> {
    if params.start == 0 || params.end == 0 || params.start > params.end {
        return Err(PlatformError::new(
            ErrorCategory::Validation,
            "extractseq requires 1-based inclusive coordinates with start <= end",
        )
        .with_code("tools.extractseq.coordinates.invalid"));
    }

    let records = load_sequence_records(&params.input)?
        .into_iter()
        .map(|record| extract_record(record, params.start, params.end))
        .collect::<Result<Vec<_>, _>>()?;

    Ok(ExtractseqOutcome {
        input: params.input,
        start: params.start,
        end: params.end,
        records,
    })
}

fn extract_record(
    record: SequenceRecord,
    start: usize,
    end: usize,
) -> Result<SequenceRecord, ToolExecutionError> {
    if end > record.len() {
        return Err(PlatformError::new(
            ErrorCategory::Validation,
            format!(
                "requested region {start}..{end} is out of range for sequence '{}' of length {}",
                record.identifier().accession(),
                record.len()
            ),
        )
        .with_code("tools.extractseq.coordinates.out_of_range"));
    }

    let interval = one_based_inclusive_interval(start, end)?;
    let subsequence = record.subsequence(interval).map_err(|error| {
        PlatformError::new(ErrorCategory::Validation, error.to_string())
            .with_code("tools.extractseq.interval.invalid")
    })?;

    let mut extracted =
        SequenceRecord::new(record.identifier().clone(), record.molecule(), subsequence).map_err(
            |error| {
                PlatformError::new(ErrorCategory::Validation, error.to_string())
                    .with_code("tools.extractseq.record.invalid")
            },
        )?;
    extracted = extracted.with_metadata(record.metadata().clone());
    Ok(extracted)
}

fn one_based_inclusive_interval(start: usize, end: usize) -> Result<Interval, ToolExecutionError> {
    Interval::from_one_based_inclusive(start, end).map_err(|error| {
        PlatformError::new(ErrorCategory::Validation, error.to_string())
            .with_code("tools.extractseq.interval.invalid")
    })
}

#[cfg(test)]
mod tests {
    use emboss_core::{MoleculeKind, SequenceIdentifier};

    use super::{ExtractseqParams, extract_record, one_based_inclusive_interval, run_extractseq};
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
    fn converts_one_based_inclusive_coordinates_to_core_interval() {
        let interval = one_based_inclusive_interval(2, 4).expect("interval should convert");
        assert_eq!(interval.start(), 1);
        assert_eq!(interval.end(), 4);
    }

    #[test]
    fn extracts_interior_region() {
        let extracted = extract_record(record("seq1", "ACGTAC"), 2, 4).expect("extracts");
        assert_eq!(extracted.residues(), "CGT");
    }

    #[test]
    fn extracts_full_length_region() {
        let extracted = extract_record(record("seq1", "ACGT"), 1, 4).expect("extracts");
        assert_eq!(extracted.residues(), "ACGT");
    }

    #[test]
    fn rejects_out_of_range_region() {
        let error = extract_record(record("seq1", "ACGT"), 2, 5).expect_err("must fail");
        assert_eq!(
            error.code(),
            Some("tools.extractseq.coordinates.out_of_range")
        );
    }

    #[test]
    fn rejects_invalid_coordinate_order() {
        let error = run_extractseq(ExtractseqParams {
            input: SequenceInput::new("unused.fa"),
            start: 4,
            end: 2,
        })
        .expect_err("must fail before reading input");
        assert_eq!(error.code(), Some("tools.extractseq.coordinates.invalid"));
    }
}

//! `sizeseq` implementation.

use emboss_core::SequenceRecord;

use crate::sequence_stream::{SequenceInput, ToolExecutionError, load_sequence_records};

/// Typed parameters for `sizeseq`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SizeseqParams {
    /// Input sequence stream.
    pub input: SequenceInput,
}

/// Structured `sizeseq` outcome.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SizeseqOutcome {
    /// Source input.
    pub input: SequenceInput,
    /// Records sorted by descending size with stable tie order.
    pub records: Vec<SequenceRecord>,
}

/// Returns `sizeseq` help text.
#[must_use]
pub fn sizeseq_help() -> &'static str {
    "Usage: emboss-rs sizeseq <input>\n\nSort input sequence records by descending length. Ties preserve input order."
}

/// Executes `sizeseq`.
pub fn run_sizeseq(params: SizeseqParams) -> Result<SizeseqOutcome, ToolExecutionError> {
    let mut records = load_sequence_records(&params.input)?;
    records.sort_by(|left, right| right.len().cmp(&left.len()));

    Ok(SizeseqOutcome {
        input: params.input,
        records,
    })
}

#[cfg(test)]
mod tests {
    use super::{SizeseqParams, run_sizeseq};
    use crate::sequence_stream::SequenceInput;
    use std::path::PathBuf;

    fn fixture(name: &str) -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(format!("../../crates/emboss-tools/tests/fixtures/{name}"))
    }

    #[test]
    fn sorts_records_by_descending_length_with_stable_ties() {
        let outcome = run_sizeseq(SizeseqParams {
            input: SequenceInput::new(fixture("sizeseq_records.fasta")),
        })
        .expect("sizeseq should succeed");

        let ids: Vec<_> = outcome
            .records
            .iter()
            .map(|record| record.identifier().accession().to_owned())
            .collect();
        assert_eq!(ids, vec!["long", "middle", "short", "short_tie"]);
    }
}

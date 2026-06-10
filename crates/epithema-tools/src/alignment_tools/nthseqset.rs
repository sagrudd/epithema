//! `nthseqset` implementation.

use epithema_core::Alignment;
use epithema_diagnostics::{ErrorCategory, PlatformError};

use super::shared::{AlignmentInput, AlignmentToolError, load_alignments};

/// Typed parameters for `nthseqset`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NthseqsetParams {
    /// Local alignment-set input path.
    pub input: AlignmentInput,
    /// 1-based alignment-set index to select.
    pub number: usize,
}

/// Structured `nthseqset` outcome.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NthseqsetOutcome {
    /// Source input.
    pub input: AlignmentInput,
    /// 1-based alignment-set index selected.
    pub number: usize,
    /// Total alignment-set count in the input.
    pub total_count: usize,
    /// Selected alignment.
    pub alignment: Alignment,
}

/// Returns `nthseqset` help text.
#[must_use]
pub fn nthseqset_help() -> &'static str {
    "Usage: epithema nthseqset <input> <number>\n\nSelect one 1-based alignment set from an input file containing one or more alignments. Epithema v1 accepts Stockholm multi-alignment inputs plus single aligned FASTA or Stockholm files, and emits the selected alignment as Stockholm."
}

/// Executes `nthseqset`.
pub fn run_nthseqset(params: NthseqsetParams) -> Result<NthseqsetOutcome, AlignmentToolError> {
    if params.number == 0 {
        return Err(PlatformError::new(
            ErrorCategory::Validation,
            "alignment-set number must be 1 or greater",
        )
        .with_code("tools.nthseqset.number.invalid"));
    }

    let mut alignments = load_alignments(&params.input)?;
    let total_count = alignments.len();
    if params.number > total_count {
        return Err(PlatformError::new(
            ErrorCategory::Validation,
            format!(
                "requested alignment-set number {} is out of range for {} alignment sets",
                params.number, total_count
            ),
        )
        .with_code("tools.nthseqset.number.out_of_range"));
    }

    Ok(NthseqsetOutcome {
        input: params.input,
        number: params.number,
        total_count,
        alignment: alignments.remove(params.number - 1),
    })
}

#[cfg(test)]
mod tests {
    use super::{NthseqsetParams, run_nthseqset};
    use crate::alignment_tools::AlignmentInput;

    fn fixture(name: &str) -> std::path::PathBuf {
        std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join(format!("../../crates/epithema-tools/tests/fixtures/{name}"))
    }

    #[test]
    fn selects_requested_stockholm_alignment_set() {
        let outcome = run_nthseqset(NthseqsetParams {
            input: AlignmentInput::new(fixture("nthseqset_alignments.sto")),
            number: 2,
        })
        .expect("nthseqset should succeed");

        assert_eq!(outcome.total_count, 2);
        assert_eq!(outcome.alignment.row_count(), 2);
        assert_eq!(outcome.alignment.column_count(), 5);
        assert_eq!(
            outcome.alignment.rows()[0].identifier().accession(),
            "gamma"
        );
    }

    #[test]
    fn accepts_single_alignment_input_as_first_set() {
        let outcome = run_nthseqset(NthseqsetParams {
            input: AlignmentInput::new(fixture("multiple_alignment.sto")),
            number: 1,
        })
        .expect("nthseqset should succeed");

        assert_eq!(outcome.total_count, 1);
        assert_eq!(outcome.alignment.row_count(), 3);
    }

    #[test]
    fn rejects_zero_number() {
        let error = run_nthseqset(NthseqsetParams {
            input: AlignmentInput::new(fixture("nthseqset_alignments.sto")),
            number: 0,
        })
        .expect_err("zero number should fail");

        assert_eq!(error.code(), Some("tools.nthseqset.number.invalid"));
    }

    #[test]
    fn rejects_out_of_range_number() {
        let error = run_nthseqset(NthseqsetParams {
            input: AlignmentInput::new(fixture("nthseqset_alignments.sto")),
            number: 3,
        })
        .expect_err("out of range number should fail");

        assert_eq!(error.code(), Some("tools.nthseqset.number.out_of_range"));
    }
}

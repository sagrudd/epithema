//! `revseq` implementation.

use epithema_core::{RevseqMode, SequenceRecord, transform_sequence_record};
use epithema_diagnostics::{ErrorCategory, PlatformError};

use crate::sequence_stream::{SequenceInput, ToolExecutionError, load_sequence_records};

/// Typed parameters for `revseq`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RevseqParams {
    /// Local sequence input path.
    pub input: SequenceInput,
    /// Requested reverse behavior.
    pub mode: RevseqMode,
}

/// Structured `revseq` outcome.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RevseqOutcome {
    /// Source input.
    pub input: SequenceInput,
    /// Applied reverse behavior.
    pub mode: RevseqMode,
    /// Reversed records.
    pub records: Vec<SequenceRecord>,
}

/// Returns `revseq` help text.
#[must_use]
pub fn revseq_help() -> &'static str {
    "Usage: epithema revseq <input> [--reverse-only | --complement]\n\nReverse sequence content for each input record.\n\nDefault behavior (`auto`):\n- DNA and RNA inputs are reverse-complemented\n- protein and unknown-molecule inputs are reversed only\n\nOptions:\n- `--reverse-only` always reverse residues without complementing\n- `--complement` require reverse-complement behavior and reject non-nucleotide inputs\n\nCurrent limitation: records with attached features are rejected because feature coordinate remapping is not yet implemented."
}

/// Executes `revseq`.
pub fn run_revseq(params: RevseqParams) -> Result<RevseqOutcome, ToolExecutionError> {
    let records = load_sequence_records(&params.input)?
        .into_iter()
        .map(|record| reverse_record(record, params.mode))
        .collect::<Result<Vec<_>, _>>()?;

    Ok(RevseqOutcome {
        input: params.input,
        mode: params.mode,
        records,
    })
}

fn reverse_record(
    record: SequenceRecord,
    mode: RevseqMode,
) -> Result<SequenceRecord, ToolExecutionError> {
    transform_sequence_record(&record, mode).map_err(|error| {
        let code = match error {
            epithema_core::RevseqError::UnsupportedReverseComplement { .. } => {
                "tools.revseq.complement.unsupported"
            }
            epithema_core::RevseqError::UnsupportedResidue { .. } => {
                "tools.revseq.complement.residue_unsupported"
            }
            epithema_core::RevseqError::UnsupportedAnnotatedRecord => {
                "tools.revseq.features.unsupported"
            }
            epithema_core::RevseqError::InvalidSequence(_) => "tools.revseq.sequence.invalid",
        };
        PlatformError::new(ErrorCategory::Validation, error.to_string()).with_code(code)
    })
}

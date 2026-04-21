//! `maskseq` implementation.

use emboss_core::{Interval, SequenceRecord, mask_intervals};
use emboss_diagnostics::{ErrorCategory, PlatformError};

use crate::feature_tools::shared::{effective_mask_symbol, map_feature_error};
use crate::sequence_stream::{SequenceInput, ToolExecutionError, load_sequence_records};

/// Typed parameters for `maskseq`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MaskseqParams {
    /// Local sequence input path.
    pub input: SequenceInput,
    /// 1-based inclusive masking intervals converted to zero-based half-open core intervals.
    pub intervals: Vec<Interval>,
    /// Optional explicit mask symbol.
    pub mask_char: Option<char>,
}

/// Structured `maskseq` outcome.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct MaskseqOutcome {
    /// Source input.
    pub input: SequenceInput,
    /// Applied intervals in core coordinates.
    pub intervals: Vec<Interval>,
    /// Optional explicit mask symbol.
    pub mask_char: Option<char>,
    /// Masked records in input order.
    pub records: Vec<SequenceRecord>,
}

/// Returns `maskseq` help text.
#[must_use]
pub fn maskseq_help() -> &'static str {
    "Usage: emboss-rs maskseq <input> <start:end> [start:end ...] [--mask-char <char>]\n\nMask one or more explicit 1-based inclusive intervals in each input sequence record. Overlapping intervals are allowed and are applied deterministically in place. The default mask symbol is N for nucleotide records and X for protein records."
}

/// Executes `maskseq`.
pub fn run_maskseq(params: MaskseqParams) -> Result<MaskseqOutcome, ToolExecutionError> {
    if params.intervals.is_empty() {
        return Err(PlatformError::new(
            ErrorCategory::Validation,
            "maskseq requires at least one interval",
        )
        .with_code("tools.maskseq.interval.missing"));
    }

    let records = load_sequence_records(&params.input)?
        .into_iter()
        .map(|record| {
            let mask_symbol = effective_mask_symbol("maskseq", &record, params.mask_char)?;
            mask_intervals(&record, &params.intervals, mask_symbol)
                .map_err(|error| map_feature_error("maskseq", error))
        })
        .collect::<Result<Vec<_>, _>>()?;

    Ok(MaskseqOutcome {
        input: params.input,
        intervals: params.intervals,
        mask_char: params.mask_char,
        records,
    })
}

#[cfg(test)]
mod tests {
    use emboss_core::{Interval, MoleculeKind, SequenceIdentifier, mask_intervals};

    use super::{MaskseqParams, run_maskseq};
    use crate::feature_tools::shared::effective_mask_symbol;
    use crate::sequence_stream::SequenceInput;

    fn record(id: &str, molecule: MoleculeKind, residues: &str) -> emboss_core::SequenceRecord {
        emboss_core::SequenceRecord::new(
            SequenceIdentifier::new(id).expect("valid identifier"),
            molecule,
            residues,
        )
        .expect("valid sequence")
    }

    #[test]
    fn defaults_to_n_for_nucleotide_sequences() {
        let symbol =
            effective_mask_symbol("maskseq", &record("dna1", MoleculeKind::Dna, "ACGT"), None)
                .expect("default mask should resolve");
        assert_eq!(symbol, 'N');
    }

    #[test]
    fn defaults_to_x_for_protein_sequences() {
        let symbol = effective_mask_symbol(
            "maskseq",
            &record("prot1", MoleculeKind::Protein, "MA*"),
            None,
        )
        .expect("default mask should resolve");
        assert_eq!(symbol, 'X');
    }

    #[test]
    fn accepts_valid_explicit_mask_character() {
        let symbol = effective_mask_symbol(
            "maskseq",
            &record("dna1", MoleculeKind::Dna, "ACGT"),
            Some('r'),
        )
        .expect("explicit mask should resolve");
        assert_eq!(symbol, 'R');
    }

    #[test]
    fn rejects_invalid_explicit_mask_character_for_molecule() {
        let error = effective_mask_symbol(
            "maskseq",
            &record("prot1", MoleculeKind::Protein, "MA*"),
            Some('?'),
        )
        .expect_err("invalid mask should fail");
        assert_eq!(
            error.code(),
            Some("tools.maskseq.mask_char.invalid_for_molecule")
        );
    }

    #[test]
    fn rejects_missing_intervals_before_reading_input() {
        let error = run_maskseq(MaskseqParams {
            input: SequenceInput::new("unused.fa"),
            intervals: Vec::new(),
            mask_char: None,
        })
        .expect_err("must fail");
        assert_eq!(error.code(), Some("tools.maskseq.interval.missing"));
    }

    #[test]
    fn masks_entire_sequence_when_interval_spans_record() {
        let masked = mask_intervals(
            &record("dna1", MoleculeKind::Dna, "ACGT"),
            &[Interval::new(0, 4).expect("valid interval")],
            effective_mask_symbol("maskseq", &record("dna1", MoleculeKind::Dna, "ACGT"), None)
                .expect("mask symbol"),
        )
        .expect("masking should succeed");
        assert_eq!(masked.residues(), "NNNN");
    }

    #[test]
    fn masks_protein_sequences_with_default_x() {
        let source = record("prot1", MoleculeKind::Protein, "MA*");
        let masked = mask_intervals(
            &source,
            &[Interval::new(1, 2).expect("valid interval")],
            effective_mask_symbol("maskseq", &source, None).expect("mask symbol"),
        )
        .expect("masking should succeed");
        assert_eq!(masked.residues(), "MX*");
    }
}

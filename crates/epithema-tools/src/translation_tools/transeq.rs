//! `transeq` implementation.

use epithema_core::{MoleculeKind, SequenceRecord};
use epithema_diagnostics::{ErrorCategory, PlatformError};

use crate::sequence_stream::{SequenceInput, ToolExecutionError, load_sequence_records};

use super::shared::{
    TranslationFrameSelection, derived_metadata, identifier_with_suffix, translate_record_frame,
};

/// Typed parameters for `transeq`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TranseqParams {
    /// Local nucleotide input path.
    pub input: SequenceInput,
    /// Forward frame selection.
    pub frame: TranslationFrameSelection,
}

/// Structured `transeq` outcome.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TranseqOutcome {
    /// Source input.
    pub input: SequenceInput,
    /// Frame selection.
    pub frame: TranslationFrameSelection,
    /// Translated protein records.
    pub records: Vec<SequenceRecord>,
}

/// Returns `transeq` help text.
#[must_use]
pub fn transeq_help() -> &'static str {
    "Usage: epithema transeq <nucleotide-input> [--frame <1|2|3|all>]\n\nTranslate nucleotide sequence records with the standard genetic code in forward frames only. v1 supports --frame 1, 2, 3, or all; ignores trailing partial codons; rejects ambiguous codons; emits protein FASTA; and names derived outputs as <identifier>.frameN."
}

/// Executes `transeq`.
pub fn run_transeq(params: TranseqParams) -> Result<TranseqOutcome, ToolExecutionError> {
    let mut records = Vec::new();

    for record in load_sequence_records(&params.input)? {
        for &frame_offset in params.frame.offsets() {
            let translated = translate_record_frame("transeq", &record, frame_offset)?;
            if translated.is_empty() {
                if params.frame == TranslationFrameSelection::AllForward {
                    continue;
                }
                return Err(PlatformError::new(
                    ErrorCategory::Validation,
                    format!(
                        "transeq frame {} did not contain a complete codon for '{}'",
                        frame_offset + 1,
                        record.identifier().accession()
                    ),
                )
                .with_code("tools.transeq.frame.empty"));
            }

            let frame_label = format!("frame{}", frame_offset + 1);
            let metadata = derived_metadata(
                record.metadata(),
                &format!(
                    "translated protein {}",
                    frame_label.replace("frame", "frame ")
                ),
            );
            let identifier = identifier_with_suffix(record.identifier(), &frame_label)?;
            let translated_record =
                SequenceRecord::new(identifier, MoleculeKind::Protein, translated)
                    .map(|derived| derived.with_metadata(metadata))
                    .map_err(|error| {
                        PlatformError::new(ErrorCategory::Validation, error.to_string())
                            .with_code("tools.transeq.sequence.invalid")
                    })?;
            records.push(translated_record);
        }
    }

    if records.is_empty() {
        return Err(PlatformError::new(
            ErrorCategory::Validation,
            "transeq did not produce any translated records",
        )
        .with_code("tools.transeq.output.empty"));
    }

    Ok(TranseqOutcome {
        input: params.input,
        frame: params.frame,
        records,
    })
}

#[cfg(test)]
mod tests {
    use super::{TranseqParams, run_transeq};
    use crate::sequence_stream::SequenceInput;
    use crate::translation_tools::shared::TranslationFrameSelection;
    use std::fs;

    fn write_temp_sequence_file(name: &str, contents: &str) -> std::path::PathBuf {
        let path = std::env::temp_dir().join(format!(
            "epithema-transeq-{name}-{}-{}.fasta",
            std::process::id(),
            std::thread::current().name().unwrap_or("main")
        ));
        fs::write(&path, contents).expect("temporary sequence fixture should be written");
        path
    }

    #[test]
    fn translates_forward_frame_one_records() {
        let input = write_temp_sequence_file("frame1", ">cds\nATGGCTTAA\n");
        let outcome = run_transeq(TranseqParams {
            input: SequenceInput::new(&input),
            frame: TranslationFrameSelection::Frame1,
        })
        .expect("transeq should translate frame 1");

        assert_eq!(outcome.records.len(), 1);
        assert_eq!(outcome.records[0].identifier().accession(), "cds.frame1");
        assert_eq!(outcome.records[0].residues(), "MA*");

        fs::remove_file(input).ok();
    }

    #[test]
    fn translates_all_forward_frames_and_skips_empty_frames() {
        let input = write_temp_sequence_file("all", ">cds\nAATGGCTTAA\n");
        let outcome = run_transeq(TranseqParams {
            input: SequenceInput::new(&input),
            frame: TranslationFrameSelection::AllForward,
        })
        .expect("transeq should translate all forward frames");

        assert_eq!(outcome.records.len(), 3);
        assert_eq!(outcome.records[0].identifier().accession(), "cds.frame1");
        assert_eq!(outcome.records[0].residues(), "NGL");
        assert_eq!(outcome.records[1].identifier().accession(), "cds.frame2");
        assert_eq!(outcome.records[1].residues(), "MA*");
        assert_eq!(outcome.records[2].identifier().accession(), "cds.frame3");
        assert_eq!(outcome.records[2].residues(), "WL");

        fs::remove_file(input).ok();
    }

    #[test]
    fn rejects_protein_input() {
        let input = write_temp_sequence_file("protein", ">pep\nMSTN\n");
        let error = run_transeq(TranseqParams {
            input: SequenceInput::new(&input),
            frame: TranslationFrameSelection::Frame1,
        })
        .expect_err("protein input should fail");

        assert!(error.code().is_some());
        fs::remove_file(input).ok();
    }
}

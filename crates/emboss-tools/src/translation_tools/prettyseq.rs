//! `prettyseq` implementation.

use emboss_diagnostics::{ErrorCategory, PlatformError};

use crate::sequence_stream::{SequenceInput, ToolExecutionError, load_sequence_records};

use super::shared::{TranslationFrameSelection, translate_record_frame};

/// Typed parameters for `prettyseq`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrettyseqParams {
    /// Local nucleotide input path.
    pub input: SequenceInput,
    /// Selected forward frame.
    pub frame: TranslationFrameSelection,
    /// Wrapped nucleotide line width.
    pub width: usize,
}

/// Structured `prettyseq` outcome.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PrettyseqOutcome {
    /// Source input.
    pub input: SequenceInput,
    /// Selected forward frame.
    pub frame: TranslationFrameSelection,
    /// Wrapped nucleotide line width.
    pub width: usize,
    /// Rendered deterministic text body.
    pub body: String,
    /// Number of rendered records.
    pub record_count: usize,
}

/// Returns `prettyseq` help text.
#[must_use]
pub fn prettyseq_help() -> &'static str {
    "Usage: emboss-rs prettyseq <nucleotide-input> [--frame <1|2|3>] [--width <count>]\n\nRender nucleotide sequence records and a translated forward frame in a deterministic text layout. v1 supports frames 1-3 only, defaults to frame 1 and width 60, ignores trailing partial codons in the translated view, and does not attempt reverse-strand presentation."
}

/// Executes `prettyseq`.
pub fn run_prettyseq(params: PrettyseqParams) -> Result<PrettyseqOutcome, ToolExecutionError> {
    if params.frame == TranslationFrameSelection::AllForward {
        return Err(PlatformError::new(
            ErrorCategory::Validation,
            "prettyseq supports one selected forward frame at a time in v1",
        )
        .with_code("tools.prettyseq.frame.all_not_supported"));
    }
    if params.width == 0 {
        return Err(PlatformError::new(
            ErrorCategory::Validation,
            "prettyseq width must be a positive integer",
        )
        .with_code("tools.prettyseq.width.invalid"));
    }

    let frame_offset = params.frame.offsets()[0];
    let frame_ordinal = frame_offset + 1;
    let records = load_sequence_records(&params.input)?;
    let mut blocks = Vec::new();

    for record in &records {
        let translated = translate_record_frame("prettyseq", record, frame_offset)?;
        if translated.is_empty() {
            return Err(PlatformError::new(
                ErrorCategory::Validation,
                format!(
                    "prettyseq frame {} did not contain a complete codon for '{}'",
                    frame_ordinal,
                    record.identifier().accession()
                ),
            )
            .with_code("tools.prettyseq.frame.empty"));
        }
        blocks.push(render_record_block(
            record.identifier().accession(),
            record.residues(),
            &translated,
            frame_offset,
            params.width,
        ));
    }

    Ok(PrettyseqOutcome {
        input: params.input,
        frame: params.frame,
        width: params.width,
        body: blocks.join("\n\n"),
        record_count: records.len(),
    })
}

fn render_record_block(
    identifier: &str,
    residues: &str,
    translated: &str,
    frame_offset: usize,
    width: usize,
) -> String {
    let mut lines = vec![
        format!(">{identifier}"),
        format!("FRAME {}", frame_offset + 1),
    ];
    let amino_chars: Vec<char> = translated.chars().collect();

    let mut start = 0usize;
    while start < residues.len() {
        let end = usize::min(start + width, residues.len());
        let nucleotide_chunk = &residues[start..end];
        let amino_start = start.saturating_sub(frame_offset) / 3;
        let amino_end = if end <= frame_offset {
            amino_start
        } else {
            usize::min(amino_chars.len(), (end - frame_offset) / 3)
        };
        let amino_chunk: String = amino_chars[amino_start..amino_end].iter().collect();

        lines.push(format!(
            "NT {:>5} {} {:>5}",
            start + 1,
            nucleotide_chunk,
            end
        ));
        if !amino_chunk.is_empty() {
            lines.push(format!(
                "AA {:>5} {} {:>5}",
                amino_start + 1,
                amino_chunk,
                amino_end
            ));
        }
        start = end;
    }

    lines.join("\n")
}

#[cfg(test)]
mod tests {
    use super::{PrettyseqParams, run_prettyseq};
    use crate::sequence_stream::SequenceInput;
    use crate::translation_tools::shared::TranslationFrameSelection;
    use std::fs;

    fn write_temp_sequence_file(name: &str, contents: &str) -> std::path::PathBuf {
        let path = std::env::temp_dir().join(format!(
            "emboss-rs-prettyseq-{name}-{}-{}.fasta",
            std::process::id(),
            std::thread::current().name().unwrap_or("main")
        ));
        fs::write(&path, contents).expect("temporary sequence fixture should be written");
        path
    }

    #[test]
    fn renders_deterministic_pretty_text() {
        let input = write_temp_sequence_file("pretty", ">cds\nATGGCTTAA\n");
        let outcome = run_prettyseq(PrettyseqParams {
            input: SequenceInput::new(&input),
            frame: TranslationFrameSelection::Frame1,
            width: 9,
        })
        .expect("prettyseq should execute");

        assert!(outcome.body.contains(">cds"));
        assert!(outcome.body.contains("FRAME 1"));
        assert!(outcome.body.contains("NT     1 ATGGCTTAA     9"));
        assert!(outcome.body.contains("AA     1 MA*     3"));

        fs::remove_file(input).ok();
    }
}

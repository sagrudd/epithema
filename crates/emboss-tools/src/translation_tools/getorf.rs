//! `getorf` implementation.

use emboss_core::{SequenceMetadata, SequenceRecord};
use emboss_diagnostics::{ErrorCategory, PlatformError};

use crate::sequence_stream::{SequenceInput, ToolExecutionError, load_sequence_records};

use super::shared::{
    derived_metadata, dna_equivalent_residues, identifier_with_suffix, validate_nucleotide_record,
};

/// Typed parameters for `getorf`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GetorfParams {
    /// Local nucleotide input path.
    pub input: SequenceInput,
}

/// One deterministic ORF extraction case.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GetorfCase {
    /// Source record identifier.
    pub source_id: String,
    /// Stable emitted ORF identifier.
    pub orf_id: String,
    /// Forward frame ordinal.
    pub frame: usize,
    /// One-based inclusive nucleotide start.
    pub start: usize,
    /// One-based inclusive nucleotide end.
    pub end: usize,
    /// Amino-acid length excluding the stop codon.
    pub amino_acid_length: usize,
}

/// Structured `getorf` outcome.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GetorfOutcome {
    /// Source input.
    pub input: SequenceInput,
    /// Extracted ORF records.
    pub records: Vec<SequenceRecord>,
    /// Stable extracted-case metadata.
    pub cases: Vec<GetorfCase>,
}

/// Returns `getorf` help text.
#[must_use]
pub fn getorf_help() -> &'static str {
    "Usage: emboss-rs getorf <nucleotide-input>\n\nExtract forward-frame ORFs from nucleotide sequence records. v1 scans frames 1-3 only, reports every ATG-started ORF that terminates at the first in-frame stop codon, includes the stop codon in the emitted nucleotide record, preserves input order, and emits FASTA output."
}

/// Executes `getorf`.
pub fn run_getorf(params: GetorfParams) -> Result<GetorfOutcome, ToolExecutionError> {
    let mut records = Vec::new();
    let mut cases = Vec::new();

    for record in load_sequence_records(&params.input)? {
        validate_nucleotide_record("getorf", &record)?;
        let source_id = record.identifier().accession().to_owned();
        let dna = dna_equivalent_residues(record.residues());
        let original = record.residues();
        let mut source_orf_index = 1usize;

        for frame_offset in 0..3 {
            let mut position = frame_offset;
            while position + 3 <= dna.len() {
                if &dna[position..position + 3] != "ATG" {
                    position += 3;
                    continue;
                }

                if let Some(stop_end) = first_in_frame_stop_end(&dna, position + 3) {
                    let frame = frame_offset + 1;
                    let start = position + 1;
                    let end = stop_end;
                    let amino_acid_length = (stop_end - position) / 3 - 1;
                    let suffix = format!("orf{source_orf_index}.frame{frame}.{start}-{end}");
                    let identifier = identifier_with_suffix(record.identifier(), &suffix)?;
                    let residues = original[position..stop_end].to_owned();
                    let metadata = orf_metadata(record.metadata(), frame, start, end);
                    let orf_record =
                        SequenceRecord::new(identifier.clone(), record.molecule(), residues)
                            .map(|derived| derived.with_metadata(metadata))
                            .map_err(|error| {
                                PlatformError::new(ErrorCategory::Validation, error.to_string())
                                    .with_code("tools.getorf.sequence.invalid")
                            })?;
                    records.push(orf_record);
                    cases.push(GetorfCase {
                        source_id: source_id.clone(),
                        orf_id: identifier.accession().to_owned(),
                        frame,
                        start,
                        end,
                        amino_acid_length,
                    });
                    source_orf_index += 1;
                }

                position += 3;
            }
        }
    }

    Ok(GetorfOutcome {
        input: params.input,
        records,
        cases,
    })
}

fn first_in_frame_stop_end(dna: &str, start: usize) -> Option<usize> {
    let mut position = start;
    while position + 3 <= dna.len() {
        if matches!(&dna[position..position + 3], "TAA" | "TAG" | "TGA") {
            return Some(position + 3);
        }
        position += 3;
    }
    None
}

fn orf_metadata(
    metadata: &SequenceMetadata,
    frame: usize,
    start: usize,
    end: usize,
) -> SequenceMetadata {
    derived_metadata(metadata, &format!("ORF frame {frame} {start}-{end}"))
}

#[cfg(test)]
mod tests {
    use super::{GetorfParams, run_getorf};
    use crate::sequence_stream::SequenceInput;
    use std::fs;

    fn write_temp_sequence_file(name: &str, contents: &str) -> std::path::PathBuf {
        let path = std::env::temp_dir().join(format!(
            "emboss-rs-getorf-{name}-{}-{}.fasta",
            std::process::id(),
            std::thread::current().name().unwrap_or("main")
        ));
        fs::write(&path, contents).expect("temporary sequence fixture should be written");
        path
    }

    #[test]
    fn extracts_forward_stop_bounded_orfs() {
        let input = write_temp_sequence_file("orfs", ">orfA\nAAAATGAAATAGATGCCCTAA\n");
        let outcome = run_getorf(GetorfParams {
            input: SequenceInput::new(&input),
        })
        .expect("getorf should execute");

        assert_eq!(outcome.records.len(), 2);
        assert_eq!(outcome.records[0].residues(), "ATGAAATAG");
        assert_eq!(outcome.records[1].residues(), "ATGCCCTAA");
        assert_eq!(outcome.cases[0].frame, 1);
        assert_eq!(outcome.cases[0].start, 4);
        assert_eq!(outcome.cases[0].end, 12);

        fs::remove_file(input).ok();
    }

    #[test]
    fn returns_empty_output_when_no_orf_is_present() {
        let input = write_temp_sequence_file("none", ">orfA\nACGACGACG\n");
        let outcome = run_getorf(GetorfParams {
            input: SequenceInput::new(&input),
        })
        .expect("getorf should execute");

        assert!(outcome.records.is_empty());
        assert!(outcome.cases.is_empty());
        fs::remove_file(input).ok();
    }
}

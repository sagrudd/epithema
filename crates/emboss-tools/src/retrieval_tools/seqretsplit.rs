//! `seqretsplit` implementation.

use std::collections::HashMap;

use emboss_core::SequenceRecord;
use emboss_diagnostics::{ErrorCategory, PlatformError};

use super::SeqretSource;

/// Shared execution error for retrieval tools.
pub type ToolExecutionError = PlatformError;

/// One deterministic split-output file for `seqretsplit`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SeqretsplitOutputFile {
    /// Deterministic output file name for the normalized record.
    pub file_name: String,
    /// Normalized record that would be written to the file.
    pub record: SequenceRecord,
}

/// Typed parameters for `seqretsplit`.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SeqretsplitParams {
    /// Resolved input source.
    pub source: SeqretSource,
    /// Normalized records produced from that source.
    pub records: Vec<SequenceRecord>,
}

/// Structured `seqretsplit` outcome.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SeqretsplitOutcome {
    /// Resolved input source.
    pub source: SeqretSource,
    /// Deterministic split-output file projections.
    pub outputs: Vec<SeqretsplitOutputFile>,
    /// Total normalized records preserved by the split-output plan.
    pub total_records: usize,
}

/// Returns the `seqretsplit` help text.
#[must_use]
pub fn seqretsplit_help() -> &'static str {
    "Usage: emboss-rs seqretsplit <input>\n\nNormalize one local or provider-backed sequence input and derive deterministic per-record output files."
}

/// Executes `seqretsplit`.
pub fn run_seqretsplit(
    params: SeqretsplitParams,
) -> Result<SeqretsplitOutcome, ToolExecutionError> {
    if params.records.is_empty() {
        return Err(PlatformError::new(
            ErrorCategory::Validation,
            "seqretsplit requires at least one sequence record",
        )
        .with_code("tools.seqretsplit.records.empty"));
    }

    let source = params.source;
    let mut emitted_names = HashMap::<String, usize>::new();
    let mut outputs = Vec::with_capacity(params.records.len());

    for record in params.records {
        let file_name = deterministic_output_file_name(&source, record.identifier().accession());
        let unique_file_name = uniquify_file_name(file_name, &mut emitted_names);
        outputs.push(SeqretsplitOutputFile {
            file_name: unique_file_name,
            record,
        });
    }

    Ok(SeqretsplitOutcome {
        source,
        total_records: outputs.len(),
        outputs,
    })
}

fn deterministic_output_file_name(source: &SeqretSource, accession: &str) -> String {
    let source_stem = match source {
        SeqretSource::LocalPath(path) => path
            .file_stem()
            .and_then(std::ffi::OsStr::to_str)
            .unwrap_or("seqretsplit"),
        SeqretSource::Retrieved {
            provider,
            accession,
        } => return format!("{}_{}.fasta", sanitize_filename_component(provider), sanitize_filename_component(accession)),
    };

    format!(
        "{}__{}.fasta",
        sanitize_filename_component(source_stem),
        sanitize_filename_component(accession)
    )
}

fn uniquify_file_name(file_name: String, emitted_names: &mut HashMap<String, usize>) -> String {
    let counter = emitted_names.entry(file_name.clone()).or_insert(0);
    *counter += 1;

    if *counter == 1 {
        return file_name;
    }

    match file_name.rsplit_once('.') {
        Some((stem, extension)) => format!("{stem}_{}.{}", *counter, extension),
        None => format!("{}_{}", file_name, *counter),
    }
}

fn sanitize_filename_component(raw: &str) -> String {
    let mut sanitized = String::with_capacity(raw.len());
    let mut previous_was_separator = false;

    for ch in raw.chars() {
        let mapped = if ch.is_ascii_alphanumeric() {
            previous_was_separator = false;
            ch
        } else if matches!(ch, '.' | '-' | '_') {
            previous_was_separator = false;
            ch
        } else if previous_was_separator {
            continue;
        } else {
            previous_was_separator = true;
            '_'
        };
        sanitized.push(mapped);
    }

    let trimmed = sanitized.trim_matches('_');
    if trimmed.is_empty() {
        "sequence".to_owned()
    } else {
        trimmed.to_owned()
    }
}

#[cfg(test)]
mod tests {
    use emboss_core::{MoleculeKind, SequenceIdentifier, SequenceRecord};

    use super::{SeqretsplitParams, run_seqretsplit};
    use crate::retrieval_tools::SeqretSource;
    use crate::sequence_stream::{SequenceInput, load_sequence_records};

    fn fixture(name: &str) -> std::path::PathBuf {
        std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join(format!("../../crates/emboss-tools/tests/fixtures/{name}"))
    }

    #[test]
    fn derives_deterministic_split_file_names_for_local_inputs() {
        let records = load_sequence_records(&SequenceInput::new(fixture("three_records.fasta")))
            .expect("fixture should load");
        let outcome = run_seqretsplit(SeqretsplitParams {
            source: SeqretSource::LocalPath(fixture("three_records.fasta")),
            records,
        })
        .expect("seqretsplit should succeed");

        assert_eq!(outcome.total_records, 3);
        assert_eq!(outcome.outputs.len(), 3);
        assert_eq!(outcome.outputs[0].file_name, "three_records__alpha.fasta");
        assert_eq!(outcome.outputs[1].file_name, "three_records__beta.fasta");
        assert_eq!(outcome.outputs[2].file_name, "three_records__gamma.fasta");
    }

    #[test]
    fn derives_provider_qualified_split_file_name_for_single_record() {
        let record = SequenceRecord::new(
            SequenceIdentifier::new("AB000263").expect("identifier should build"),
            MoleculeKind::Dna,
            "ACGT",
        )
        .expect("record should build");
        let outcome = run_seqretsplit(SeqretsplitParams {
            source: SeqretSource::Retrieved {
                provider: "ena".to_owned(),
                accession: "AB000263".to_owned(),
            },
            records: vec![record],
        })
        .expect("seqretsplit should succeed");

        assert_eq!(outcome.outputs.len(), 1);
        assert_eq!(outcome.outputs[0].file_name, "ena_AB000263.fasta");
    }

    #[test]
    fn uniquifies_duplicate_identifiers_deterministically() {
        let first = SequenceRecord::new(
            SequenceIdentifier::new("dup").expect("identifier should build"),
            MoleculeKind::Dna,
            "ACGT",
        )
        .expect("record should build");
        let second = SequenceRecord::new(
            SequenceIdentifier::new("dup").expect("identifier should build"),
            MoleculeKind::Dna,
            "TGCA",
        )
        .expect("record should build");
        let outcome = run_seqretsplit(SeqretsplitParams {
            source: SeqretSource::LocalPath(fixture("three_records.fasta")),
            records: vec![first, second],
        })
        .expect("seqretsplit should succeed");

        assert_eq!(outcome.outputs[0].file_name, "three_records__dup.fasta");
        assert_eq!(outcome.outputs[1].file_name, "three_records__dup_2.fasta");
    }

    #[test]
    fn rejects_empty_record_sets() {
        let error = run_seqretsplit(SeqretsplitParams {
            source: SeqretSource::LocalPath(fixture("three_records.fasta")),
            records: Vec::new(),
        })
        .expect_err("empty record sets should fail");

        assert!(error.to_string().contains("at least one sequence record"));
    }
}

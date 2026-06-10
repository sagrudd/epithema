//! Shared sequence-stream helpers.

use std::fs::File;
use std::io::{BufReader, Read};
use std::path::{Path, PathBuf};

use epithema_core::SequenceRecord;
use epithema_diagnostics::{ErrorCategory, PlatformError};
use epithema_io::{
    parse_embl_reader, parse_fasta_reader, parse_fastq_reader, parse_genbank_reader,
};

/// Local sequence input handled by the first shipped sequence-stream cohort.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SequenceInput {
    /// Canonical local path to the input data.
    pub path: PathBuf,
}

impl SequenceInput {
    /// Creates a new local sequence input wrapper.
    #[must_use]
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self { path: path.into() }
    }
}

/// Shared execution error type for sequence-stream tools.
pub type ToolExecutionError = PlatformError;

/// Loads sequence records from a supported local file.
pub fn load_sequence_records(
    input: &SequenceInput,
) -> Result<Vec<SequenceRecord>, ToolExecutionError> {
    let format = detect_sequence_format(&input.path)?;
    let file = File::open(&input.path).map_err(|error| {
        PlatformError::new(ErrorCategory::Validation, "failed to open sequence input")
            .with_code("tools.sequence_stream.input.open_failed")
            .with_detail(format!("{}: {error}", input.path.display()))
    })?;
    let reader = BufReader::new(file);

    match format.as_str() {
        "fasta" => parse_fasta_reader(reader).map_err(map_io_error),
        "fastq" => parse_fastq_reader(reader)
            .map(|records| {
                records
                    .into_iter()
                    .map(|record| record.sequence().clone())
                    .collect()
            })
            .map_err(map_io_error),
        "embl" => parse_embl_reader(reader).map_err(map_io_error),
        "genbank" => parse_genbank_reader(reader).map_err(map_io_error),
        other => Err(PlatformError::new(
            ErrorCategory::Validation,
            format!("unsupported sequence input format '{other}'"),
        )
        .with_code("tools.sequence_stream.input.unsupported_format")),
    }
}

fn detect_sequence_format(path: &Path) -> Result<String, ToolExecutionError> {
    if let Some(extension) = path.extension().and_then(|value| value.to_str()) {
        let lowered = extension.to_ascii_lowercase();
        let inferred = match lowered.as_str() {
            "fa" | "fasta" | "fna" | "faa" | "fas" => Some("fasta"),
            "fq" | "fastq" => Some("fastq"),
            "embl" | "dat" => Some("embl"),
            "gb" | "gbk" | "genbank" => Some("genbank"),
            _ => None,
        };

        if let Some(format) = inferred {
            return Ok(format.to_owned());
        }
    }

    let mut file = File::open(path).map_err(|error| {
        PlatformError::new(
            ErrorCategory::Configuration,
            "failed to inspect sequence input format",
        )
        .with_code("tools.sequence_stream.input.inspect_failed")
        .with_detail(format!("{}: {error}", path.display()))
    })?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer).map_err(|error| {
        PlatformError::new(
            ErrorCategory::Configuration,
            "failed to read sequence input for format detection",
        )
        .with_code("tools.sequence_stream.input.read_failed")
        .with_detail(format!("{}: {error}", path.display()))
    })?;

    let first_content = buffer
        .lines()
        .map(str::trim)
        .find(|line| !line.is_empty())
        .unwrap_or_default();

    let format = if first_content.starts_with('>') {
        "fasta"
    } else if first_content.starts_with('@') {
        "fastq"
    } else if first_content.starts_with("ID ") {
        "embl"
    } else if first_content.starts_with("LOCUS ") {
        "genbank"
    } else {
        return Err(PlatformError::new(
            ErrorCategory::Validation,
            "could not determine a supported sequence input format",
        )
        .with_code("tools.sequence_stream.input.unknown_format")
        .with_detail(path.display().to_string()));
    };

    Ok(format.to_owned())
}

fn map_io_error(error: epithema_io::IoError) -> ToolExecutionError {
    PlatformError::new(ErrorCategory::Validation, error.to_string())
        .with_code("tools.sequence_stream.input.parse_failed")
}

#[cfg(test)]
mod tests {
    use super::{SequenceInput, detect_sequence_format, load_sequence_records};

    fn fixture(path: &str) -> std::path::PathBuf {
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(path)
    }

    #[test]
    fn detects_fasta_by_extension() {
        let format = detect_sequence_format(&fixture(
            "../epithema-tools/tests/fixtures/three_records.fasta",
        ))
        .expect("format should detect");
        assert_eq!(format, "fasta");
    }

    #[test]
    fn loads_multi_record_fasta() {
        let records = load_sequence_records(&SequenceInput::new(fixture(
            "../epithema-tools/tests/fixtures/three_records.fasta",
        )))
        .expect("fixture should load");
        assert_eq!(records.len(), 3);
    }
}

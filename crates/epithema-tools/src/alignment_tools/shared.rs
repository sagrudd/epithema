use std::fs::File;
use std::io::{BufReader, Read};
use std::path::{Path, PathBuf};

use epithema_core::Alignment;
use epithema_diagnostics::{ErrorCategory, PlatformError};
use epithema_io::{parse_aligned_fasta_reader, parse_stockholm_reader};

/// Local alignment input handled by the alignment utility cohort.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AlignmentInput {
    /// Canonical local path to the alignment data.
    pub path: PathBuf,
}

impl AlignmentInput {
    /// Creates a new local alignment input wrapper.
    #[must_use]
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self { path: path.into() }
    }
}

/// Shared execution error type for alignment tools.
pub type AlignmentToolError = PlatformError;

/// Loads one or more alignments from a supported local file.
pub fn load_alignments(input: &AlignmentInput) -> Result<Vec<Alignment>, AlignmentToolError> {
    let format = detect_alignment_format(&input.path)?;
    let file = File::open(&input.path).map_err(|error| {
        PlatformError::new(ErrorCategory::Validation, "failed to open alignment input")
            .with_code("tools.alignment.input.open_failed")
            .with_detail(format!("{}: {error}", input.path.display()))
    })?;
    let reader = BufReader::new(file);

    match format.as_str() {
        "aligned-fasta" => parse_aligned_fasta_reader(reader)
            .map(|alignment| vec![alignment])
            .map_err(map_io_error),
        "stockholm" => parse_stockholm_reader(reader).map_err(map_io_error),
        other => Err(PlatformError::new(
            ErrorCategory::Validation,
            format!("unsupported alignment input format '{other}'"),
        )
        .with_code("tools.alignment.input.unsupported_format")),
    }
}

/// Loads a single alignment from a supported local file.
pub fn load_alignment(input: &AlignmentInput) -> Result<Alignment, AlignmentToolError> {
    let mut alignments = load_alignments(input)?;
    if alignments.len() != 1 {
        return Err(PlatformError::new(
            ErrorCategory::Validation,
            "alignment utility tools require exactly one alignment per input file",
        )
        .with_code("tools.alignment.input.multiple_alignments")
        .with_detail(input.path.display().to_string()));
    }

    Ok(alignments.remove(0))
}

fn detect_alignment_format(path: &Path) -> Result<String, AlignmentToolError> {
    if let Some(extension) = path.extension().and_then(|value| value.to_str()) {
        let lowered = extension.to_ascii_lowercase();
        let inferred = match lowered.as_str() {
            "sto" | "stockholm" => Some("stockholm"),
            "afa" | "afasta" => Some("aligned-fasta"),
            "fa" | "fasta" | "fas" => Some("aligned-fasta"),
            _ => None,
        };

        if let Some(format) = inferred {
            return Ok(format.to_owned());
        }
    }

    let mut file = File::open(path).map_err(|error| {
        PlatformError::new(
            ErrorCategory::Configuration,
            "failed to inspect alignment input format",
        )
        .with_code("tools.alignment.input.inspect_failed")
        .with_detail(format!("{}: {error}", path.display()))
    })?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer).map_err(|error| {
        PlatformError::new(
            ErrorCategory::Configuration,
            "failed to read alignment input for format detection",
        )
        .with_code("tools.alignment.input.read_failed")
        .with_detail(format!("{}: {error}", path.display()))
    })?;

    let first_content = buffer
        .lines()
        .map(str::trim)
        .find(|line| !line.is_empty())
        .unwrap_or_default();

    let format = if first_content.starts_with("# STOCKHOLM 1.0") {
        "stockholm"
    } else if first_content.starts_with('>') {
        "aligned-fasta"
    } else {
        return Err(PlatformError::new(
            ErrorCategory::Validation,
            "could not determine a supported alignment input format",
        )
        .with_code("tools.alignment.input.unknown_format")
        .with_detail(path.display().to_string()));
    };

    Ok(format.to_owned())
}

fn map_io_error(error: epithema_io::IoError) -> AlignmentToolError {
    PlatformError::new(ErrorCategory::Validation, error.to_string())
        .with_code("tools.alignment.input.parse_failed")
}

#[cfg(test)]
mod tests {
    use super::{AlignmentInput, detect_alignment_format, load_alignment, load_alignments};

    fn fixture(path: &str) -> std::path::PathBuf {
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(path)
    }

    #[test]
    fn detects_stockholm_by_extension() {
        let format = detect_alignment_format(&fixture(
            "../epithema-tools/tests/fixtures/pairwise_alignment.sto",
        ))
        .expect("format should detect");
        assert_eq!(format, "stockholm");
    }

    #[test]
    fn loads_single_alignment_fixture() {
        let alignment = load_alignment(&AlignmentInput::new(fixture(
            "../epithema-tools/tests/fixtures/multiple_alignment.sto",
        )))
        .expect("fixture should load");
        assert_eq!(alignment.row_count(), 3);
    }

    #[test]
    fn loads_multiple_stockholm_alignments() {
        let alignments = load_alignments(&AlignmentInput::new(fixture(
            "../epithema-tools/tests/fixtures/nthseqset_alignments.sto",
        )))
        .expect("fixture should load");
        assert_eq!(alignments.len(), 2);
    }
}

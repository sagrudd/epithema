//! Sequence file format parsing and writing for EMBOSS-RS.
//!
//! The current IO layer supports FASTA and FASTQ with explicit, readable APIs
//! built around the strengthened `emboss-core` biological model. FASTA records
//! map directly to `SequenceRecord`. FASTQ records use an IO-local
//! `FastqRecord`, which couples a `SequenceRecord` with decoded Phred+33
//! quality scores.

pub mod error;
pub mod fasta;
pub mod fastq;

pub use error::IoError;
pub use fasta::{
    DEFAULT_FASTA_LINE_WIDTH, parse_fasta_reader, parse_fasta_reader_with_molecule,
    parse_fasta_str, parse_fasta_str_with_molecule, write_fasta_string, write_fasta_writer,
    write_fasta_writer_wrapped,
};
pub use fastq::{
    FastqRecord, QualityScores, parse_fastq_reader, parse_fastq_reader_with_molecule,
    parse_fastq_str, parse_fastq_str_with_molecule, write_fastq_string, write_fastq_writer,
};

/// Minimal description of a supported data format.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DataFormat {
    /// Stable format identifier.
    pub name: &'static str,
    /// Short operational summary.
    pub summary: &'static str,
}

/// Registry of known formats in the current workspace state.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FormatCatalog {
    formats: Vec<DataFormat>,
}

impl Default for FormatCatalog {
    fn default() -> Self {
        Self {
            formats: vec![
                DataFormat {
                    name: "fasta",
                    summary: "FASTA sequence parsing and writing",
                },
                DataFormat {
                    name: "fastq",
                    summary: "FASTQ sequence-plus-quality parsing and writing",
                },
            ],
        }
    }
}

impl FormatCatalog {
    /// Creates the current built-in format catalog.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the currently registered formats.
    #[must_use]
    pub fn formats(&self) -> &[DataFormat] {
        &self.formats
    }
}

#[cfg(test)]
mod tests {
    use super::FormatCatalog;

    #[test]
    fn lists_supported_sequence_formats() {
        let catalog = FormatCatalog::new();
        assert_eq!(catalog.formats().len(), 2);
        assert_eq!(catalog.formats()[0].name, "fasta");
        assert_eq!(catalog.formats()[1].name, "fastq");
    }
}

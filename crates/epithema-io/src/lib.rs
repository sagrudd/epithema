//! Sequence file format parsing and writing for Epithema.
//!
//! The current IO layer supports FASTA and FASTQ with explicit, readable APIs
//! built around the strengthened `epithema-core` biological model. FASTA records
//! map directly to `SequenceRecord`. FASTQ records use an IO-local
//! `FastqRecord`, which couples a `SequenceRecord` with decoded Phred+33
//! quality scores. EMBL and GenBank support a practical annotated-record subset
//! with top-level metadata, simple feature tables, and stable writing. Aligned
//! FASTA and Stockholm support map directly to the shared `Alignment` model.

pub mod aligned_fasta;
pub mod embl;
pub mod error;
pub mod fasta;
pub mod fastq;
pub mod flatfile;
pub mod genbank;
pub mod stockholm;

pub use aligned_fasta::{
    DEFAULT_ALIGNED_FASTA_LINE_WIDTH, parse_aligned_fasta_reader,
    parse_aligned_fasta_reader_with_molecule, parse_aligned_fasta_str,
    parse_aligned_fasta_str_with_molecule, write_aligned_fasta_string, write_aligned_fasta_writer,
    write_aligned_fasta_writer_wrapped,
};
pub use embl::{parse_embl_reader, parse_embl_str, write_embl_string, write_embl_writer};
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
pub use genbank::{
    parse_genbank_reader, parse_genbank_str, write_genbank_string, write_genbank_writer,
};
pub use stockholm::{
    parse_stockholm_reader, parse_stockholm_str, write_stockholm_string, write_stockholm_writer,
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
                DataFormat {
                    name: "embl",
                    summary: "EMBL flat-file parsing and writing for annotated sequence records",
                },
                DataFormat {
                    name: "genbank",
                    summary: "GenBank flat-file parsing and writing for annotated sequence records",
                },
                DataFormat {
                    name: "aligned-fasta",
                    summary: "Aligned FASTA parsing and writing for sequence alignments",
                },
                DataFormat {
                    name: "stockholm",
                    summary: "Stockholm alignment parsing and writing for a practical subset",
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
        assert_eq!(catalog.formats().len(), 6);
        assert_eq!(catalog.formats()[0].name, "fasta");
        assert_eq!(catalog.formats()[1].name, "fastq");
        assert_eq!(catalog.formats()[2].name, "embl");
        assert_eq!(catalog.formats()[3].name, "genbank");
        assert_eq!(catalog.formats()[4].name, "aligned-fasta");
        assert_eq!(catalog.formats()[5].name, "stockholm");
    }
}

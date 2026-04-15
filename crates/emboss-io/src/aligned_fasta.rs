//! Aligned FASTA parsing and writing.
//!
//! Supported subset:
//! - one alignment per input
//! - FASTA-style headers with first-token identifier and optional description
//! - `-` and `.` gap symbols, normalized to `-`

use std::io::{BufRead, Write};

use emboss_core::{Alignment, AlignmentRow, MoleculeKind, SequenceIdentifier, SequenceMetadata};

use crate::error::IoError;
use crate::flatfile::infer_molecule_kind;

/// Default line width for aligned FASTA output.
pub const DEFAULT_ALIGNED_FASTA_LINE_WIDTH: usize = 80;

/// Parses an aligned FASTA document into a single validated alignment.
pub fn parse_aligned_fasta_reader<R: BufRead>(reader: R) -> Result<Alignment, IoError> {
    parse_aligned_fasta_reader_internal(reader, None)
}

/// Parses an aligned FASTA document using an explicit molecule kind for all rows.
pub fn parse_aligned_fasta_reader_with_molecule<R: BufRead>(
    reader: R,
    molecule: MoleculeKind,
) -> Result<Alignment, IoError> {
    parse_aligned_fasta_reader_internal(reader, Some(molecule))
}

/// Parses an aligned FASTA document from string content.
pub fn parse_aligned_fasta_str(input: &str) -> Result<Alignment, IoError> {
    parse_aligned_fasta_reader(std::io::Cursor::new(input.as_bytes()))
}

/// Parses an aligned FASTA document from string content using an explicit molecule kind.
pub fn parse_aligned_fasta_str_with_molecule(
    input: &str,
    molecule: MoleculeKind,
) -> Result<Alignment, IoError> {
    parse_aligned_fasta_reader_with_molecule(std::io::Cursor::new(input.as_bytes()), molecule)
}

/// Writes an alignment as aligned FASTA with the default line wrap.
pub fn write_aligned_fasta_writer<W: Write>(
    writer: W,
    alignment: &Alignment,
) -> Result<(), IoError> {
    write_aligned_fasta_writer_wrapped(writer, alignment, DEFAULT_ALIGNED_FASTA_LINE_WIDTH)
}

/// Writes an alignment as aligned FASTA with a configurable line wrap.
pub fn write_aligned_fasta_writer_wrapped<W: Write>(
    mut writer: W,
    alignment: &Alignment,
    line_width: usize,
) -> Result<(), IoError> {
    if line_width == 0 {
        return Err(IoError::parse(
            "aligned-fasta",
            None,
            "line width must be greater than zero",
        ));
    }

    for row in alignment.rows() {
        writeln!(writer, ">{}", format_header(row))?;
        for chunk in row.aligned().as_bytes().chunks(line_width) {
            writeln!(writer, "{}", String::from_utf8_lossy(chunk))?;
        }
    }

    Ok(())
}

/// Writes an alignment as aligned FASTA to a `String`.
pub fn write_aligned_fasta_string(alignment: &Alignment) -> Result<String, IoError> {
    let mut buffer = Vec::new();
    write_aligned_fasta_writer(&mut buffer, alignment)?;
    String::from_utf8(buffer)
        .map_err(|error| IoError::parse("aligned-fasta", None, format!("non-utf8 output: {error}")))
}

fn parse_aligned_fasta_reader_internal<R: BufRead>(
    reader: R,
    explicit_molecule: Option<MoleculeKind>,
) -> Result<Alignment, IoError> {
    let mut rows = Vec::new();
    let mut current_header: Option<(String, Option<String>, usize)> = None;
    let mut aligned = String::new();

    for (line_index, line_result) in reader.lines().enumerate() {
        let line_number = line_index + 1;
        let line = line_result?;
        let trimmed = line.trim();

        if trimmed.is_empty() {
            continue;
        }

        if let Some(header) = trimmed.strip_prefix('>') {
            if let Some((identifier, description, header_line)) = current_header.take() {
                rows.push(build_alignment_row(
                    identifier,
                    description,
                    &aligned,
                    explicit_molecule,
                    header_line,
                )?);
                aligned.clear();
            }

            let (identifier, description) = parse_header("aligned-fasta", header, line_number)?;
            current_header = Some((identifier, description, line_number));
            continue;
        }

        if current_header.is_none() {
            return Err(IoError::parse(
                "aligned-fasta",
                Some(line_number),
                "alignment data appeared before the first header",
            ));
        }

        aligned.push_str(trimmed);
    }

    if let Some((identifier, description, header_line)) = current_header.take() {
        rows.push(build_alignment_row(
            identifier,
            description,
            &aligned,
            explicit_molecule,
            header_line,
        )?);
    }

    if rows.is_empty() {
        return Err(IoError::parse(
            "aligned-fasta",
            None,
            "no aligned FASTA rows were found",
        ));
    }

    Alignment::new(rows).map_err(IoError::from)
}

fn build_alignment_row(
    identifier: String,
    description: Option<String>,
    aligned: &str,
    explicit_molecule: Option<MoleculeKind>,
    header_line: usize,
) -> Result<AlignmentRow, IoError> {
    if aligned.trim().is_empty() {
        return Err(IoError::parse(
            "aligned-fasta",
            Some(header_line),
            format!("row '{identifier}' has no aligned content"),
        ));
    }

    let sequence_identifier = SequenceIdentifier::new(identifier)?;
    let ungapped: String = aligned
        .chars()
        .filter(|symbol| !matches!(symbol, '-' | '.'))
        .collect();
    let molecule = explicit_molecule.unwrap_or_else(|| infer_molecule_kind(&ungapped));
    let mut metadata = SequenceMetadata::new();
    if let Some(description) = description {
        metadata = metadata.with_description(description);
    }

    AlignmentRow::new(sequence_identifier, molecule, aligned)
        .map(|row| row.with_metadata(metadata))
        .map_err(IoError::from)
}

fn parse_header(
    format: &'static str,
    header: &str,
    line_number: usize,
) -> Result<(String, Option<String>), IoError> {
    let trimmed = header.trim();
    if trimmed.is_empty() {
        return Err(IoError::parse(
            format,
            Some(line_number),
            "header line must contain an identifier",
        ));
    }

    let mut parts = trimmed.splitn(2, char::is_whitespace);
    let identifier = parts.next().unwrap_or_default().to_owned();
    let description = parts
        .next()
        .map(str::trim)
        .filter(|description| !description.is_empty())
        .map(ToOwned::to_owned);
    Ok((identifier, description))
}

fn format_header(row: &AlignmentRow) -> String {
    let mut header = row.identifier().accession().to_owned();
    if let Some(description) = row.metadata().description.as_deref() {
        if !description.is_empty() {
            header.push(' ');
            header.push_str(description);
        }
    }
    header
}

#[cfg(test)]
mod tests {
    use emboss_core::{AlignmentSymbol, MoleculeKind};

    use super::{
        DEFAULT_ALIGNED_FASTA_LINE_WIDTH, parse_aligned_fasta_str,
        parse_aligned_fasta_str_with_molecule, write_aligned_fasta_string,
        write_aligned_fasta_writer_wrapped,
    };

    #[test]
    fn parses_pairwise_aligned_fasta() {
        let alignment = parse_aligned_fasta_str(">seq1 first\nAC-GT\n>seq2 second\nACTGT\n")
            .expect("aligned fasta should parse");

        assert!(alignment.is_pairwise());
        assert_eq!(alignment.column_count(), 5);
        assert_eq!(
            alignment.rows()[0].metadata().description.as_deref(),
            Some("first")
        );
        assert_eq!(
            alignment.column(2).expect("column should exist")[0],
            AlignmentSymbol::Gap
        );
    }

    #[test]
    fn supports_explicit_molecule_override() {
        let alignment = parse_aligned_fasta_str_with_molecule(
            ">seq1\nACGU-\n>seq2\nACG-U\n",
            MoleculeKind::Rna,
        )
        .expect("aligned fasta should parse");
        assert_eq!(alignment.rows()[0].molecule(), MoleculeKind::Rna);
    }

    #[test]
    fn rejects_inconsistent_row_lengths() {
        let error = parse_aligned_fasta_str(">seq1\nAC-GT\n>seq2\nACTG\n")
            .expect_err("length mismatch should fail");
        assert!(error.to_string().contains("same aligned length"));
    }

    #[test]
    fn round_trips_aligned_fasta() {
        let alignment = parse_aligned_fasta_str(">seq1\nAC-GT\n>seq2\nACTGT\n")
            .expect("aligned fasta should parse");
        let rendered = write_aligned_fasta_string(&alignment).expect("aligned fasta should render");
        let reparsed = parse_aligned_fasta_str(&rendered).expect("round-trip should parse");
        assert_eq!(reparsed, alignment);
    }

    #[test]
    fn wraps_output_deterministically() {
        let alignment = parse_aligned_fasta_str(">seq1\nAC-GTAC-GT\n>seq2\nACTGTACAGT\n")
            .expect("aligned fasta should parse");
        let mut output = Vec::new();
        write_aligned_fasta_writer_wrapped(&mut output, &alignment, 4)
            .expect("writer should succeed");
        let rendered = String::from_utf8(output).expect("valid utf8");
        assert!(rendered.contains("AC-G\nTAC-\nGT"));
        assert_eq!(DEFAULT_ALIGNED_FASTA_LINE_WIDTH, 80);
    }
}

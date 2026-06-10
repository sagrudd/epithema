//! FASTA parsing and writing.

use std::io::{BufRead, Write};

use epithema_core::{Alphabet, MoleculeKind, SequenceIdentifier, SequenceMetadata, SequenceRecord};

use crate::error::IoError;

/// Default line width used when writing FASTA.
pub const DEFAULT_FASTA_LINE_WIDTH: usize = 80;

/// Parses FASTA from a buffered reader with conservative molecule inference.
pub fn parse_fasta_reader<R: BufRead>(reader: R) -> Result<Vec<SequenceRecord>, IoError> {
    parse_fasta_reader_internal(reader, None)
}

/// Parses FASTA from a buffered reader using an explicit molecule kind.
pub fn parse_fasta_reader_with_molecule<R: BufRead>(
    reader: R,
    molecule: MoleculeKind,
) -> Result<Vec<SequenceRecord>, IoError> {
    parse_fasta_reader_internal(reader, Some(molecule))
}

/// Parses FASTA from string content with conservative molecule inference.
pub fn parse_fasta_str(input: &str) -> Result<Vec<SequenceRecord>, IoError> {
    parse_fasta_reader(std::io::Cursor::new(input.as_bytes()))
}

/// Parses FASTA from string content using an explicit molecule kind.
pub fn parse_fasta_str_with_molecule(
    input: &str,
    molecule: MoleculeKind,
) -> Result<Vec<SequenceRecord>, IoError> {
    parse_fasta_reader_with_molecule(std::io::Cursor::new(input.as_bytes()), molecule)
}

/// Writes FASTA records with the default 80-column wrap.
pub fn write_fasta_writer<W: Write>(writer: W, records: &[SequenceRecord]) -> Result<(), IoError> {
    write_fasta_writer_wrapped(writer, records, DEFAULT_FASTA_LINE_WIDTH)
}

/// Writes FASTA records with a configurable line width.
pub fn write_fasta_writer_wrapped<W: Write>(
    mut writer: W,
    records: &[SequenceRecord],
    line_width: usize,
) -> Result<(), IoError> {
    if line_width == 0 {
        return Err(IoError::parse(
            "fasta",
            None,
            "line width must be greater than zero",
        ));
    }

    for record in records {
        writeln!(writer, ">{}", format_header(record))?;
        for chunk in record.residues().as_bytes().chunks(line_width) {
            writeln!(writer, "{}", String::from_utf8_lossy(chunk))?;
        }
    }

    Ok(())
}

/// Writes FASTA records to a `String`.
pub fn write_fasta_string(records: &[SequenceRecord]) -> Result<String, IoError> {
    let mut buffer = Vec::new();
    write_fasta_writer(&mut buffer, records)?;
    String::from_utf8(buffer)
        .map_err(|error| IoError::parse("fasta", None, format!("non-utf8 output: {error}")))
}

fn parse_fasta_reader_internal<R: BufRead>(
    reader: R,
    explicit_molecule: Option<MoleculeKind>,
) -> Result<Vec<SequenceRecord>, IoError> {
    let mut records = Vec::new();
    let mut current_header: Option<(String, Option<String>, usize)> = None;
    let mut sequence_lines = String::new();

    for (line_index, line_result) in reader.lines().enumerate() {
        let line_number = line_index + 1;
        let line = line_result?;
        let trimmed = line.trim();

        if trimmed.is_empty() {
            continue;
        }

        if let Some(header) = trimmed.strip_prefix('>') {
            if let Some((identifier, description, header_line)) = current_header.take() {
                records.push(build_fasta_record(
                    identifier,
                    description,
                    &sequence_lines,
                    explicit_molecule,
                    header_line,
                )?);
                sequence_lines.clear();
            }

            let (identifier, description) = parse_header(header, line_number)?;
            current_header = Some((identifier, description, line_number));
            continue;
        }

        if current_header.is_none() {
            return Err(IoError::parse(
                "fasta",
                Some(line_number),
                "sequence data appeared before the first header",
            ));
        }

        sequence_lines.push_str(trimmed);
    }

    if let Some((identifier, description, header_line)) = current_header.take() {
        records.push(build_fasta_record(
            identifier,
            description,
            &sequence_lines,
            explicit_molecule,
            header_line,
        )?);
    }

    if records.is_empty() {
        return Err(IoError::parse("fasta", None, "no FASTA records were found"));
    }

    Ok(records)
}

fn build_fasta_record(
    identifier: String,
    description: Option<String>,
    residues: &str,
    explicit_molecule: Option<MoleculeKind>,
    header_line: usize,
) -> Result<SequenceRecord, IoError> {
    if residues.trim().is_empty() {
        return Err(IoError::parse(
            "fasta",
            Some(header_line),
            format!("record '{identifier}' has no sequence residues"),
        ));
    }

    let sequence_identifier = SequenceIdentifier::new(identifier)?;
    let molecule = explicit_molecule.unwrap_or_else(|| infer_molecule_kind(residues));
    let mut metadata = SequenceMetadata::new();
    if let Some(description) = description {
        metadata = metadata.with_description(description);
    }

    SequenceRecord::new(sequence_identifier, molecule, residues)
        .map(|record| record.with_metadata(metadata))
        .map_err(IoError::from)
}

fn parse_header(header: &str, line_number: usize) -> Result<(String, Option<String>), IoError> {
    let trimmed = header.trim();
    if trimmed.is_empty() {
        return Err(IoError::parse(
            "fasta",
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

fn format_header(record: &SequenceRecord) -> String {
    let mut header = record.identifier().accession().to_owned();
    let description = record
        .metadata()
        .description
        .as_deref()
        .filter(|value| !value.is_empty())
        .or_else(|| {
            record
                .identifier()
                .display_name()
                .filter(|display_name| *display_name != record.identifier().accession())
        });

    if let Some(description) = description {
        header.push(' ');
        header.push_str(description);
    }

    header
}

fn infer_molecule_kind(residues: &str) -> MoleculeKind {
    let uppercase: String = residues
        .chars()
        .filter(|symbol| !symbol.is_whitespace())
        .map(|symbol| symbol.to_ascii_uppercase())
        .collect();

    let has_u = uppercase.contains('U');
    let has_t = uppercase.contains('T');

    if has_u
        && !has_t
        && Alphabet::Rna
            .validate(MoleculeKind::Rna, &uppercase)
            .is_ok()
    {
        return MoleculeKind::Rna;
    }

    if has_t
        && !has_u
        && Alphabet::Dna
            .validate(MoleculeKind::Dna, &uppercase)
            .is_ok()
    {
        return MoleculeKind::Dna;
    }

    let dna_like = Alphabet::Dna
        .validate(MoleculeKind::Dna, &uppercase)
        .is_ok();
    let rna_like = Alphabet::Rna
        .validate(MoleculeKind::Rna, &uppercase)
        .is_ok();
    if !dna_like
        && !rna_like
        && Alphabet::Protein
            .validate(MoleculeKind::Protein, &uppercase)
            .is_ok()
    {
        return MoleculeKind::Protein;
    }

    MoleculeKind::Unknown
}

#[cfg(test)]
mod tests {
    use epithema_core::MoleculeKind;

    use super::{
        DEFAULT_FASTA_LINE_WIDTH, parse_fasta_str, parse_fasta_str_with_molecule,
        write_fasta_string, write_fasta_writer_wrapped,
    };

    #[test]
    fn parses_minimal_fasta_record() {
        let records = parse_fasta_str(">seq1\nACGT\n").expect("fasta should parse");
        assert_eq!(records.len(), 1);
        assert_eq!(records[0].identifier().accession(), "seq1");
        assert_eq!(records[0].residues(), "ACGT");
        assert_eq!(records[0].molecule(), MoleculeKind::Dna);
    }

    #[test]
    fn parses_multi_record_fasta_with_descriptions() {
        let records = parse_fasta_str(">seq1 first record\nACGT\n>seq2 second\nMPEQ\n")
            .expect("fasta should parse");

        assert_eq!(records.len(), 2);
        assert_eq!(
            records[0].metadata().description.as_deref(),
            Some("first record")
        );
        assert_eq!(records[1].metadata().description.as_deref(), Some("second"));
        assert_eq!(records[1].molecule(), MoleculeKind::Protein);
    }

    #[test]
    fn supports_explicit_molecule_override() {
        let records = parse_fasta_str_with_molecule(">rna\nACGU\n", MoleculeKind::Rna)
            .expect("rna fasta should parse");
        assert_eq!(records[0].molecule(), MoleculeKind::Rna);
    }

    #[test]
    fn rejects_missing_header() {
        let error = parse_fasta_str("ACGT\n").expect_err("header should be required");
        assert!(error.to_string().contains("before the first header"));
    }

    #[test]
    fn round_trips_fasta_records() {
        let original = parse_fasta_str(">seq1 desc\nACGTACGT\n").expect("fasta should parse");
        let rendered = write_fasta_string(&original).expect("fasta should render");
        let reparsed = parse_fasta_str(&rendered).expect("round-trip should parse");

        assert_eq!(reparsed, original);
    }

    #[test]
    fn wraps_fasta_output_deterministically() {
        let records = parse_fasta_str(">seq1\nACGTACGTAC\n").expect("fasta should parse");
        let mut output = Vec::new();
        write_fasta_writer_wrapped(&mut output, &records, 4).expect("fasta should write");
        let rendered = String::from_utf8(output).expect("valid utf8");

        assert!(rendered.contains(">seq1"));
        assert!(rendered.contains("ACGT\nACGT\nAC"));
        assert_eq!(DEFAULT_FASTA_LINE_WIDTH, 80);
    }

    #[test]
    fn rejects_zero_width_wrapping() {
        let records = parse_fasta_str(">seq1\nACGT\n").expect("fasta should parse");
        let error =
            write_fasta_writer_wrapped(Vec::new(), &records, 0).expect_err("width must be valid");
        assert!(error.to_string().contains("line width"));
    }
}

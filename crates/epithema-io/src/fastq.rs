//! FASTQ parsing and writing.

use std::io::{BufRead, Write};

use epithema_core::{Alphabet, MoleculeKind, SequenceIdentifier, SequenceMetadata, SequenceRecord};

use crate::error::IoError;

/// Decoded Phred quality scores stored as integer values.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QualityScores {
    values: Vec<u8>,
}

impl QualityScores {
    /// Creates quality scores from decoded Phred values.
    pub fn new(values: Vec<u8>) -> Result<Self, IoError> {
        if values.iter().any(|value| *value > 93) {
            return Err(IoError::parse(
                "fastq",
                None,
                "quality scores must be in the inclusive range 0..=93",
            ));
        }

        Ok(Self { values })
    }

    /// Decodes Phred+33 ASCII quality text.
    pub fn from_phred33(text: &str) -> Result<Self, IoError> {
        let mut values = Vec::with_capacity(text.len());

        for (index, symbol) in text.chars().enumerate() {
            let codepoint = u32::from(symbol);
            if !(33..=126).contains(&codepoint) {
                return Err(IoError::parse(
                    "fastq",
                    None,
                    format!(
                        "quality symbol at position {} is outside printable FASTQ range",
                        index + 1
                    ),
                ));
            }

            let value = u8::try_from(codepoint - 33).map_err(|_| {
                IoError::parse("fastq", None, "quality value could not be represented")
            })?;
            values.push(value);
        }

        Self::new(values)
    }

    /// Returns the decoded Phred values.
    #[must_use]
    pub fn values(&self) -> &[u8] {
        &self.values
    }

    /// Returns the number of scores.
    #[must_use]
    pub fn len(&self) -> usize {
        self.values.len()
    }

    /// Returns `true` when no quality scores are present.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    /// Encodes the scores as canonical Phred+33 FASTQ text.
    pub fn to_phred33(&self) -> Result<String, IoError> {
        self.values
            .iter()
            .map(|value| {
                if *value > 93 {
                    return Err(IoError::parse(
                        "fastq",
                        None,
                        format!("quality score {value} is outside the writable Phred+33 range"),
                    ));
                }

                let codepoint = u32::from(*value) + 33;
                char::from_u32(codepoint).ok_or_else(|| {
                    IoError::parse("fastq", None, "quality score could not be encoded")
                })
            })
            .collect()
    }
}

/// FASTQ record coupling sequence data with per-base quality scores.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FastqRecord {
    sequence: SequenceRecord,
    qualities: QualityScores,
}

impl FastqRecord {
    /// Creates a typed FASTQ record from a sequence record and decoded qualities.
    pub fn new(sequence: SequenceRecord, qualities: QualityScores) -> Result<Self, IoError> {
        if sequence.len() != qualities.len() {
            return Err(IoError::parse(
                "fastq",
                None,
                format!(
                    "sequence length {} does not match quality length {}",
                    sequence.len(),
                    qualities.len()
                ),
            ));
        }

        Ok(Self {
            sequence,
            qualities,
        })
    }

    /// Returns the typed biological sequence record.
    #[must_use]
    pub fn sequence(&self) -> &SequenceRecord {
        &self.sequence
    }

    /// Returns the decoded quality scores.
    #[must_use]
    pub fn qualities(&self) -> &QualityScores {
        &self.qualities
    }
}

/// Parses FASTQ from a buffered reader with conservative molecule inference.
pub fn parse_fastq_reader<R: BufRead>(reader: R) -> Result<Vec<FastqRecord>, IoError> {
    parse_fastq_reader_internal(reader, None)
}

/// Parses FASTQ from a buffered reader using an explicit molecule kind.
pub fn parse_fastq_reader_with_molecule<R: BufRead>(
    reader: R,
    molecule: MoleculeKind,
) -> Result<Vec<FastqRecord>, IoError> {
    parse_fastq_reader_internal(reader, Some(molecule))
}

/// Parses FASTQ from string content with conservative molecule inference.
pub fn parse_fastq_str(input: &str) -> Result<Vec<FastqRecord>, IoError> {
    parse_fastq_reader(std::io::Cursor::new(input.as_bytes()))
}

/// Parses FASTQ from string content using an explicit molecule kind.
pub fn parse_fastq_str_with_molecule(
    input: &str,
    molecule: MoleculeKind,
) -> Result<Vec<FastqRecord>, IoError> {
    parse_fastq_reader_with_molecule(std::io::Cursor::new(input.as_bytes()), molecule)
}

/// Writes FASTQ records to a generic writer.
pub fn write_fastq_writer<W: Write>(mut writer: W, records: &[FastqRecord]) -> Result<(), IoError> {
    for record in records {
        let header = format_header(record.sequence());
        let qualities = record.qualities().to_phred33()?;
        writeln!(writer, "@{header}")?;
        writeln!(writer, "{}", record.sequence().residues())?;
        writeln!(writer, "+")?;
        writeln!(writer, "{qualities}")?;
    }

    Ok(())
}

/// Writes FASTQ records to a `String`.
pub fn write_fastq_string(records: &[FastqRecord]) -> Result<String, IoError> {
    let mut buffer = Vec::new();
    write_fastq_writer(&mut buffer, records)?;
    String::from_utf8(buffer)
        .map_err(|error| IoError::parse("fastq", None, format!("non-utf8 output: {error}")))
}

fn parse_fastq_reader_internal<R: BufRead>(
    reader: R,
    explicit_molecule: Option<MoleculeKind>,
) -> Result<Vec<FastqRecord>, IoError> {
    let mut records = Vec::new();
    let mut lines = reader.lines().enumerate();

    while let Some((header_index, header_line)) = lines.next() {
        let header_number = header_index + 1;
        let header = header_line?;
        if header.trim().is_empty() {
            continue;
        }

        let Some(header_body) = header.strip_prefix('@') else {
            return Err(IoError::parse(
                "fastq",
                Some(header_number),
                "record header must start with '@'",
            ));
        };

        let sequence_line = read_required_line(&mut lines, header_number + 1, "sequence line")?;
        let separator_line =
            read_required_line(&mut lines, header_number + 2, "quality separator line")?;
        let quality_line = read_required_line(&mut lines, header_number + 3, "quality line")?;

        if !separator_line.starts_with('+') {
            return Err(IoError::parse(
                "fastq",
                Some(header_number + 2),
                "quality separator line must start with '+'",
            ));
        }

        let (identifier, description) = parse_header(header_body, header_number)?;
        let qualities = QualityScores::from_phred33(quality_line.trim_end())?;
        let record = build_fastq_record(
            identifier,
            description,
            sequence_line.trim_end(),
            qualities,
            explicit_molecule,
            header_number,
        )?;
        records.push(record);
    }

    if records.is_empty() {
        return Err(IoError::parse("fastq", None, "no FASTQ records were found"));
    }

    Ok(records)
}

fn read_required_line<R: BufRead>(
    lines: &mut std::iter::Enumerate<std::io::Lines<R>>,
    expected_line_number: usize,
    label: &str,
) -> Result<String, IoError> {
    match lines.next() {
        Some((_, line)) => line.map_err(IoError::from),
        None => Err(IoError::parse(
            "fastq",
            Some(expected_line_number),
            format!("missing {label}"),
        )),
    }
}

fn build_fastq_record(
    identifier: String,
    description: Option<String>,
    residues: &str,
    qualities: QualityScores,
    explicit_molecule: Option<MoleculeKind>,
    header_line: usize,
) -> Result<FastqRecord, IoError> {
    if residues.trim().is_empty() {
        return Err(IoError::parse(
            "fastq",
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

    let sequence =
        SequenceRecord::new(sequence_identifier, molecule, residues)?.with_metadata(metadata);
    FastqRecord::new(sequence, qualities)
}

fn parse_header(header: &str, line_number: usize) -> Result<(String, Option<String>), IoError> {
    let trimmed = header.trim();
    if trimmed.is_empty() {
        return Err(IoError::parse(
            "fastq",
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
    if let Some(description) = record.metadata().description.as_deref() {
        if !description.is_empty() {
            header.push(' ');
            header.push_str(description);
        }
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
        FastqRecord, QualityScores, parse_fastq_str, parse_fastq_str_with_molecule,
        write_fastq_string,
    };

    #[test]
    fn parses_minimal_fastq_record() {
        let records = parse_fastq_str("@seq1\nACGT\n+\n!!!!\n").expect("fastq should parse");

        assert_eq!(records.len(), 1);
        assert_eq!(records[0].sequence().identifier().accession(), "seq1");
        assert_eq!(records[0].sequence().residues(), "ACGT");
        assert_eq!(records[0].qualities().values(), &[0, 0, 0, 0]);
        assert_eq!(records[0].sequence().molecule(), MoleculeKind::Dna);
    }

    #[test]
    fn parses_multi_record_fastq_with_descriptions() {
        let records = parse_fastq_str("@seq1 first\nACGT\n+\n!!!!\n@seq2 second\nMSNT\n+\nIIII\n")
            .expect("fastq should parse");

        assert_eq!(records.len(), 2);
        assert_eq!(
            records[0].sequence().metadata().description.as_deref(),
            Some("first")
        );
        assert_eq!(
            records[1].sequence().metadata().description.as_deref(),
            Some("second")
        );
        assert_eq!(records[1].qualities().values(), &[40, 40, 40, 40]);
    }

    #[test]
    fn supports_explicit_molecule_override() {
        let records = parse_fastq_str_with_molecule("@rna\nACGU\n+\n####\n", MoleculeKind::Rna)
            .expect("fastq should parse");
        assert_eq!(records[0].sequence().molecule(), MoleculeKind::Rna);
    }

    #[test]
    fn rejects_invalid_separator() {
        let error = parse_fastq_str("@seq1\nACGT\n?\n!!!!\n")
            .expect_err("separator line should be validated");
        assert!(error.to_string().contains("separator"));
    }

    #[test]
    fn rejects_quality_length_mismatch() {
        let error = parse_fastq_str("@seq1\nACGT\n+\n!!!\n")
            .expect_err("sequence and quality lengths must agree");
        assert!(error.to_string().contains("length"));
    }

    #[test]
    fn rejects_invalid_phred_symbol() {
        let error = parse_fastq_str("@seq1\nACGT\n+\n! \u{7f}!\n")
            .expect_err("quality characters must be printable");
        assert!(error.to_string().contains("quality symbol"));
    }

    #[test]
    fn round_trips_fastq_records() {
        let original = parse_fastq_str("@seq1 desc\nACGT\n+\n!!!!\n").expect("fastq should parse");
        let rendered = write_fastq_string(&original).expect("fastq should render");
        let reparsed = parse_fastq_str(&rendered).expect("round-trip should parse");

        assert_eq!(reparsed, original);
    }

    #[test]
    fn rejects_writing_out_of_range_quality_scores() {
        let qualities =
            QualityScores::new(vec![94]).expect_err("out-of-range qualities should fail");
        assert!(qualities.to_string().contains("inclusive range"));
    }

    #[test]
    fn validates_record_constructor_lengths() {
        let sequence = parse_fastq_str("@seq1\nACGT\n+\n!!!!\n")
            .expect("fastq should parse")
            .into_iter()
            .next()
            .expect("record should exist")
            .sequence()
            .clone();
        let qualities = QualityScores::new(vec![1, 2, 3]).expect("qualities should build");
        let error = FastqRecord::new(sequence, qualities).expect_err("lengths must match");
        assert!(error.to_string().contains("length"));
    }
}

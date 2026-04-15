//! Stockholm alignment parsing and writing for a practical subset.
//!
//! Supported subset:
//! - `# STOCKHOLM 1.0` header
//! - sequence rows
//! - `#=GF ID` alignment identifier
//! - other annotation lines are ignored
//! - `//` record terminator

use std::collections::BTreeMap;
use std::io::{BufRead, Write};

use emboss_core::{Alignment, AlignmentRow, SequenceIdentifier};

use crate::error::IoError;
use crate::flatfile::infer_molecule_kind;

/// Parses one or more Stockholm alignments from a buffered reader.
pub fn parse_stockholm_reader<R: BufRead>(reader: R) -> Result<Vec<Alignment>, IoError> {
    let mut alignments = Vec::new();
    let mut in_record = false;
    let mut current_identifier: Option<String> = None;
    let mut row_segments: BTreeMap<String, String> = BTreeMap::new();
    let mut row_order: Vec<String> = Vec::new();

    for (line_index, line_result) in reader.lines().enumerate() {
        let line_number = line_index + 1;
        let line = line_result?;
        let trimmed = line.trim();

        if trimmed.is_empty() {
            continue;
        }

        if trimmed == "# STOCKHOLM 1.0" {
            if in_record {
                return Err(IoError::parse(
                    "stockholm",
                    Some(line_number),
                    "encountered a new Stockholm header before closing the previous record",
                ));
            }

            in_record = true;
            current_identifier = None;
            row_segments.clear();
            row_order.clear();
            continue;
        }

        if !in_record {
            return Err(IoError::parse(
                "stockholm",
                Some(line_number),
                "missing '# STOCKHOLM 1.0' header",
            ));
        }

        if trimmed == "//" {
            let alignment =
                build_stockholm_alignment(current_identifier.take(), &row_order, &row_segments)?;
            alignments.push(alignment);
            in_record = false;
            row_segments.clear();
            row_order.clear();
            continue;
        }

        if let Some(rest) = trimmed.strip_prefix("#=GF") {
            let rest = rest.trim();
            if let Some(value) = rest.strip_prefix("ID") {
                let value = value.trim();
                if !value.is_empty() {
                    current_identifier = Some(value.to_owned());
                }
            }
            continue;
        }

        if trimmed.starts_with('#') {
            continue;
        }

        let mut parts = trimmed.split_whitespace();
        let identifier = parts.next().ok_or_else(|| {
            IoError::parse("stockholm", Some(line_number), "missing row identifier")
        })?;
        let segment = parts.next().ok_or_else(|| {
            IoError::parse(
                "stockholm",
                Some(line_number),
                "missing aligned row content",
            )
        })?;
        if parts.next().is_some() {
            return Err(IoError::parse(
                "stockholm",
                Some(line_number),
                "unsupported extra fields in Stockholm sequence row",
            ));
        }

        if !row_segments.contains_key(identifier) {
            row_order.push(identifier.to_owned());
            row_segments.insert(identifier.to_owned(), segment.to_owned());
        } else if let Some(existing) = row_segments.get_mut(identifier) {
            existing.push_str(segment);
        }
    }

    if in_record {
        return Err(IoError::parse(
            "stockholm",
            None,
            "record terminated without closing '//'",
        ));
    }

    if alignments.is_empty() {
        return Err(IoError::parse(
            "stockholm",
            None,
            "no Stockholm alignments were found",
        ));
    }

    Ok(alignments)
}

/// Parses Stockholm content from a string.
pub fn parse_stockholm_str(input: &str) -> Result<Vec<Alignment>, IoError> {
    parse_stockholm_reader(std::io::Cursor::new(input.as_bytes()))
}

/// Writes one alignment as Stockholm text.
pub fn write_stockholm_writer<W: Write>(
    mut writer: W,
    alignment: &Alignment,
) -> Result<(), IoError> {
    writeln!(writer, "# STOCKHOLM 1.0")?;
    if let Some(identifier) = alignment.identifier() {
        writeln!(writer, "#=GF ID {identifier}")?;
    }
    for row in alignment.rows() {
        writeln!(writer, "{} {}", row.identifier().accession(), row.aligned())?;
    }
    writeln!(writer, "//")?;
    Ok(())
}

/// Writes one alignment as a `String`.
pub fn write_stockholm_string(alignment: &Alignment) -> Result<String, IoError> {
    let mut buffer = Vec::new();
    write_stockholm_writer(&mut buffer, alignment)?;
    String::from_utf8(buffer)
        .map_err(|error| IoError::parse("stockholm", None, format!("non-utf8 output: {error}")))
}

fn build_stockholm_alignment(
    identifier: Option<String>,
    row_order: &[String],
    row_segments: &BTreeMap<String, String>,
) -> Result<Alignment, IoError> {
    if row_order.is_empty() {
        return Err(IoError::parse(
            "stockholm",
            None,
            "alignment record contains no sequence rows",
        ));
    }

    let rows = row_order
        .iter()
        .map(|row_identifier| {
            let aligned = row_segments
                .get(row_identifier)
                .expect("row order and segment map stay in sync");
            let sequence_identifier = SequenceIdentifier::new(row_identifier.clone())?;
            let ungapped: String = aligned
                .chars()
                .filter(|symbol| !matches!(symbol, '-' | '.'))
                .collect();
            let molecule = infer_molecule_kind(&ungapped);
            AlignmentRow::new(sequence_identifier, molecule, aligned).map_err(IoError::from)
        })
        .collect::<Result<Vec<_>, IoError>>()?;

    Alignment::with_identifier(identifier, rows).map_err(IoError::from)
}

#[cfg(test)]
mod tests {
    use super::{parse_stockholm_str, write_stockholm_string};

    #[test]
    fn parses_pairwise_stockholm_alignment() {
        let alignments =
            parse_stockholm_str("# STOCKHOLM 1.0\n#=GF ID pairwise\nseq1 AC-GT\nseq2 ACTGT\n//\n")
                .expect("stockholm should parse");

        assert_eq!(alignments.len(), 1);
        let alignment = &alignments[0];
        assert_eq!(alignment.identifier(), Some("pairwise"));
        assert!(alignment.is_pairwise());
        assert_eq!(alignment.column_count(), 5);
    }

    #[test]
    fn parses_multiple_alignment_with_ignored_annotations() {
        let alignments = parse_stockholm_str(
            "# STOCKHOLM 1.0\n#=GF ID msa\n#=GC SS_cons ....\nseq1 AC-GT\nseq2 ACTGT\nseq3 ACAGT\n//\n",
        )
        .expect("stockholm should parse");

        assert!(alignments[0].is_multiple());
        assert_eq!(alignments[0].row_count(), 3);
    }

    #[test]
    fn rejects_missing_header() {
        let error = parse_stockholm_str("seq1 AC-GT\nseq2 ACTGT\n//\n")
            .expect_err("header should be required");
        assert!(error.to_string().contains("header"));
    }

    #[test]
    fn rejects_inconsistent_row_lengths() {
        let error = parse_stockholm_str("# STOCKHOLM 1.0\nseq1 AC-GT\nseq2 ACTG\n//\n")
            .expect_err("length mismatch should fail");
        assert!(error.to_string().contains("same aligned length"));
    }

    #[test]
    fn round_trips_stockholm_alignment() {
        let alignment =
            parse_stockholm_str("# STOCKHOLM 1.0\n#=GF ID pairwise\nseq1 AC-GT\nseq2 ACTGT\n//\n")
                .expect("stockholm should parse")
                .into_iter()
                .next()
                .expect("alignment should exist");
        let rendered = write_stockholm_string(&alignment).expect("stockholm should render");
        let reparsed = parse_stockholm_str(&rendered)
            .expect("round-trip should parse")
            .into_iter()
            .next()
            .expect("alignment should exist");
        assert_eq!(reparsed, alignment);
    }
}

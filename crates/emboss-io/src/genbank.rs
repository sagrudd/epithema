//! GenBank flat-file parsing and writing for a practical annotated-record subset.
//!
//! Supported subset:
//! - `LOCUS`, `DEFINITION`, `ACCESSION`, `VERSION`, `SOURCE`, `ORGANISM`,
//!   `FEATURES`, `ORIGIN`, and `//`
//! - single-record and multi-record parsing
//! - simple feature locations, `complement(...)`, and `join(...)`
//! - basic qualifier parsing and stable writing for the same subset

use std::io::{BufRead, Write};

use emboss_core::{MoleculeKind, SequenceIdentifier, SequenceMetadata, SequenceRecord};

use crate::error::IoError;
use crate::flatfile::{
    feature_from_parts, finalize_record_features, format_feature_kind, format_feature_qualifiers,
    format_location, infer_molecule_kind, parse_location, parse_qualifiers,
    sequence_from_flatfile_line, write_sequence_blocks,
};

type RawQualifierLines = Vec<(usize, String)>;
type RawFeatureEntry = (String, String, usize, RawQualifierLines);

/// Parses one or more GenBank records from a buffered reader.
pub fn parse_genbank_reader<R: BufRead>(reader: R) -> Result<Vec<SequenceRecord>, IoError> {
    let mut records = Vec::new();
    let mut current = Vec::new();

    for line_result in reader.lines() {
        let line = line_result?;
        if line.trim() == "//" {
            if !current.is_empty() {
                records.push(parse_genbank_record(&current)?);
                current.clear();
            }
            continue;
        }

        if !line.trim().is_empty() {
            current.push(line);
        }
    }

    if !current.is_empty() {
        return Err(IoError::parse(
            "genbank",
            None,
            "record terminated without closing '//'",
        ));
    }

    if records.is_empty() {
        return Err(IoError::parse(
            "genbank",
            None,
            "no GenBank records were found",
        ));
    }

    Ok(records)
}

/// Parses GenBank content from a string.
pub fn parse_genbank_str(input: &str) -> Result<Vec<SequenceRecord>, IoError> {
    parse_genbank_reader(std::io::Cursor::new(input.as_bytes()))
}

/// Writes GenBank records to a generic writer.
pub fn write_genbank_writer<W: Write>(
    mut writer: W,
    records: &[SequenceRecord],
) -> Result<(), IoError> {
    for record in records {
        let molecule = match record.molecule() {
            MoleculeKind::Dna => "DNA",
            MoleculeKind::Rna => "RNA",
            MoleculeKind::Protein => "PROTEIN",
            MoleculeKind::Unknown => "DNA",
        };
        let topology = record
            .metadata()
            .topology
            .map(|value| match value {
                emboss_core::SequenceTopology::Linear => "linear",
                emboss_core::SequenceTopology::Circular => "circular",
            })
            .unwrap_or("linear");

        writeln!(
            writer,
            "LOCUS       {:<12} {:>6} bp    {:<7} {:<8} UNC 01-JAN-1980",
            record.identifier().accession(),
            record.len(),
            molecule,
            topology
        )?;
        if let Some(description) = &record.metadata().description {
            writeln!(writer, "DEFINITION  {description}")?;
        }
        writeln!(writer, "ACCESSION   {}", record.identifier().accession())?;
        writeln!(writer, "VERSION     {}.1", record.identifier().accession())?;
        let source = record
            .metadata()
            .source
            .as_deref()
            .or(record.metadata().organism.as_deref())
            .unwrap_or("unknown");
        writeln!(writer, "SOURCE      {source}")?;
        if let Some(organism) = &record.metadata().organism {
            writeln!(writer, "  ORGANISM  {organism}")?;
        }
        writeln!(writer, "FEATURES             Location/Qualifiers")?;
        for feature in record.features() {
            writeln!(
                writer,
                "     {:<15} {}",
                format_feature_kind(&feature.kind),
                format_location(&feature.location)
            )?;
            for (key, value) in format_feature_qualifiers(feature) {
                if value == "true" {
                    writeln!(writer, "                     /{key}")?;
                } else {
                    writeln!(writer, "                     /{key}=\"{value}\"")?;
                }
            }
        }
        writeln!(writer, "ORIGIN")?;
        for line in write_sequence_blocks(record.residues(), 60, 10) {
            writeln!(writer, "{line}")?;
        }
        writeln!(writer, "//")?;
    }

    Ok(())
}

/// Writes GenBank records to a `String`.
pub fn write_genbank_string(records: &[SequenceRecord]) -> Result<String, IoError> {
    let mut buffer = Vec::new();
    write_genbank_writer(&mut buffer, records)?;
    String::from_utf8(buffer)
        .map_err(|error| IoError::parse("genbank", None, format!("non-utf8 output: {error}")))
}

fn parse_genbank_record(lines: &[String]) -> Result<SequenceRecord, IoError> {
    let mut accession: Option<String> = None;
    let mut description_parts = Vec::new();
    let mut source_parts = Vec::new();
    let mut organism_parts = Vec::new();
    let mut in_features = false;
    let mut in_origin = false;
    let mut sequence = String::new();
    let mut molecule: Option<MoleculeKind> = None;
    let mut raw_features: Vec<RawFeatureEntry> = Vec::new();
    let mut current_header: Option<&'static str> = None;

    for (index, line) in lines.iter().enumerate() {
        let line_number = index + 1;

        if in_origin {
            sequence.push_str(&sequence_from_flatfile_line(line));
            continue;
        }

        if line.starts_with("FEATURES") {
            in_features = true;
            current_header = None;
            continue;
        }

        if line.starts_with("ORIGIN") {
            in_origin = true;
            in_features = false;
            current_header = None;
            continue;
        }

        if in_features {
            parse_genbank_feature_line(line, line_number, &mut raw_features)?;
            continue;
        }

        if let Some(value) = line.strip_prefix("LOCUS") {
            current_header = Some("LOCUS");
            let content = value.trim();
            accession = content.split_whitespace().next().map(ToOwned::to_owned);
            let upper = content.to_ascii_uppercase();
            molecule = if upper.contains(" RNA") {
                Some(MoleculeKind::Rna)
            } else if upper.contains("PROTEIN") || upper.contains(" AA") {
                Some(MoleculeKind::Protein)
            } else if upper.contains(" DNA") {
                Some(MoleculeKind::Dna)
            } else {
                molecule
            };
            continue;
        }

        if let Some(value) = line.strip_prefix("DEFINITION") {
            current_header = Some("DEFINITION");
            description_parts.push(value.trim().to_owned());
            continue;
        }

        if let Some(value) = line.strip_prefix("ACCESSION") {
            current_header = Some("ACCESSION");
            accession = value
                .split_whitespace()
                .next()
                .map(ToOwned::to_owned)
                .or(accession);
            continue;
        }

        if let Some(value) = line.strip_prefix("SOURCE") {
            current_header = Some("SOURCE");
            source_parts.push(value.trim().to_owned());
            continue;
        }

        if let Some(value) = line.strip_prefix("  ORGANISM") {
            current_header = Some("ORGANISM");
            organism_parts.push(value.trim().to_owned());
            continue;
        }

        if line.starts_with("            ") {
            match current_header {
                Some("DEFINITION") => description_parts.push(line.trim().to_owned()),
                Some("ORGANISM") => {
                    organism_parts.push(line.trim().trim_end_matches('.').to_owned())
                }
                _ => {}
            }
        }
    }

    if sequence.is_empty() {
        return Err(IoError::parse(
            "genbank",
            None,
            "record is missing an ORIGIN sequence section",
        ));
    }

    let accession = accession
        .ok_or_else(|| IoError::parse("genbank", None, "record is missing an accession"))?;
    let identifier = SequenceIdentifier::new(accession)?;
    let molecule = molecule.unwrap_or_else(|| infer_molecule_kind(&sequence));
    let mut metadata = SequenceMetadata::new();
    if !description_parts.is_empty() {
        metadata = metadata.with_description(description_parts.join(" "));
    }
    if !organism_parts.is_empty() {
        metadata = metadata.with_organism(organism_parts.join("; "));
    }
    if !source_parts.is_empty() {
        metadata = metadata.with_source(source_parts.join(" "));
    }

    let record = SequenceRecord::new(identifier, molecule, sequence)?.with_metadata(metadata);
    let features = raw_features
        .into_iter()
        .map(|(kind, location, line_number, qualifiers)| {
            let location = parse_location("genbank", &location, Some(line_number))?;
            let qualifiers = parse_qualifiers("genbank", &qualifiers)?;
            Ok(feature_from_parts(&kind, location, qualifiers))
        })
        .collect::<Result<Vec<_>, IoError>>()?;

    finalize_record_features(record, features)
}

fn parse_genbank_feature_line(
    line: &str,
    line_number: usize,
    raw_features: &mut Vec<RawFeatureEntry>,
) -> Result<(), IoError> {
    let trimmed = line.trim();
    if trimmed.is_empty() {
        return Ok(());
    }

    if let Some(qualifier) = trimmed.strip_prefix('/') {
        let current = raw_features.last_mut().ok_or_else(|| {
            IoError::parse(
                "genbank",
                Some(line_number),
                "feature continuation without a feature",
            )
        })?;
        current.3.push((line_number, format!("/{qualifier}")));
        return Ok(());
    }

    let content = line.trim_start();
    let key_end = content.find(char::is_whitespace).ok_or_else(|| {
        IoError::parse(
            "genbank",
            Some(line_number),
            "malformed FEATURES table line",
        )
    })?;
    let key_field = content[..key_end].trim();
    let value_field = content[key_end..].trim();

    if key_field.is_empty() || value_field.is_empty() {
        return Err(IoError::parse(
            "genbank",
            Some(line_number),
            "malformed FEATURES table line",
        ));
    }

    raw_features.push((
        key_field.to_owned(),
        value_field.to_owned(),
        line_number,
        Vec::new(),
    ));
    Ok(())
}

#[cfg(test)]
mod tests {
    use emboss_core::{FeatureKind, MoleculeKind, Strand};

    use super::{parse_genbank_str, write_genbank_string};

    #[test]
    fn parses_representative_genbank_record() {
        let records = parse_genbank_str(
            "LOCUS       SCU49845     40 bp    DNA     linear   PLN 01-JAN-2000\n\
DEFINITION  Example annotated sequence.\n\
ACCESSION   SCU49845\n\
VERSION     SCU49845.1\n\
SOURCE      Saccharomyces cerevisiae\n\
  ORGANISM  Saccharomyces cerevisiae\n\
            Eukaryota; Fungi.\n\
FEATURES             Location/Qualifiers\n\
     gene            1..20\n\
                     /gene=\"TCP1-beta\"\n\
     CDS             complement(5..18)\n\
                     /product=\"chaperonin\"\n\
ORIGIN\n\
        1 atgcatgcat gcatgcatgc atgcatgcat gcatgcatgc\n\
//\n",
        )
        .expect("genbank should parse");

        assert_eq!(records.len(), 1);
        let record = &records[0];
        assert_eq!(record.molecule(), MoleculeKind::Dna);
        assert_eq!(record.identifier().accession(), "SCU49845");
        assert_eq!(
            record.metadata().description.as_deref(),
            Some("Example annotated sequence.")
        );
        assert_eq!(record.features().len(), 2);
        assert!(matches!(record.features()[0].kind, FeatureKind::Gene));
        assert_eq!(
            record.features()[1].location.strand(),
            Some(Strand::Reverse)
        );
    }

    #[test]
    fn round_trips_supported_genbank_subset() {
        let parsed = parse_genbank_str(
            "LOCUS       TEST0001     12 bp    DNA     linear   UNC 01-JAN-2000\n\
DEFINITION  Example.\n\
ACCESSION   TEST0001\n\
SOURCE      Example source\n\
  ORGANISM  Example organism\n\
FEATURES             Location/Qualifiers\n\
     misc_feature    join(1..3,7..9)\n\
                     /note=\"joined\"\n\
ORIGIN\n\
        1 acgtacgtacgt\n\
//\n",
        )
        .expect("genbank should parse");

        let rendered = write_genbank_string(&parsed).expect("genbank should render");
        let reparsed = parse_genbank_str(&rendered).expect("rendered genbank should parse");
        assert_eq!(reparsed, parsed);
    }

    #[test]
    fn rejects_missing_origin_section() {
        let error = parse_genbank_str("LOCUS       TEST\nACCESSION   TEST\n//\n")
            .expect_err("origin should be required");
        assert!(error.to_string().contains("ORIGIN"));
    }

    #[test]
    fn rejects_unsupported_feature_syntax() {
        let error = parse_genbank_str(
            "LOCUS       TEST0001     12 bp    DNA     linear   UNC 01-JAN-2000\n\
ACCESSION   TEST0001\n\
FEATURES             Location/Qualifiers\n\
     gene            1^2\n\
ORIGIN\n\
        1 acgtacgtacgt\n\
//\n",
        )
        .expect_err("unsupported syntax should fail");
        assert!(error.to_string().contains("unsupported feature"));
    }
}

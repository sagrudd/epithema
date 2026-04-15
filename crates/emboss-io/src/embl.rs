//! EMBL flat-file parsing and writing for a practical annotated-record subset.
//!
//! Supported subset:
//! - `ID`, `AC`, `DE`, `OS`, `OC`, `FH`, `FT`, `SQ`, and `//`
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

/// Parses one or more EMBL records from a buffered reader.
pub fn parse_embl_reader<R: BufRead>(reader: R) -> Result<Vec<SequenceRecord>, IoError> {
    let mut records = Vec::new();
    let mut current = Vec::new();

    for line_result in reader.lines() {
        let line = line_result?;
        if line.trim() == "//" {
            if !current.is_empty() {
                records.push(parse_embl_record(&current)?);
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
            "embl",
            None,
            "record terminated without closing '//'",
        ));
    }

    if records.is_empty() {
        return Err(IoError::parse("embl", None, "no EMBL records were found"));
    }

    Ok(records)
}

/// Parses EMBL content from a string.
pub fn parse_embl_str(input: &str) -> Result<Vec<SequenceRecord>, IoError> {
    parse_embl_reader(std::io::Cursor::new(input.as_bytes()))
}

/// Writes EMBL records to a generic writer.
pub fn write_embl_writer<W: Write>(
    mut writer: W,
    records: &[SequenceRecord],
) -> Result<(), IoError> {
    for record in records {
        let topology = record
            .metadata()
            .topology
            .map(|value| match value {
                emboss_core::SequenceTopology::Linear => "linear",
                emboss_core::SequenceTopology::Circular => "circular",
            })
            .unwrap_or("linear");
        let molecule = match record.molecule() {
            MoleculeKind::Dna => "DNA",
            MoleculeKind::Rna => "RNA",
            MoleculeKind::Protein => "AA",
            MoleculeKind::Unknown => "DNA",
        };

        writeln!(
            writer,
            "ID   {}; SV 1; {}; {}; UNC; {} BP.",
            record.identifier().accession(),
            topology,
            molecule,
            record.len()
        )?;
        writeln!(writer, "AC   {};", record.identifier().accession())?;
        if let Some(description) = &record.metadata().description {
            writeln!(writer, "DE   {description}")?;
        }
        if let Some(organism) = &record.metadata().organism {
            writeln!(writer, "OS   {organism}")?;
        }
        if let Some(source) = &record.metadata().source {
            writeln!(writer, "OC   {source}")?;
        }
        writeln!(writer, "FH   Key             Location/Qualifiers")?;
        writeln!(writer, "FH")?;
        for feature in record.features() {
            writeln!(
                writer,
                "FT   {:<15} {}",
                format_feature_kind(&feature.kind),
                format_location(&feature.location)
            )?;
            for (key, value) in format_feature_qualifiers(feature) {
                if value == "true" {
                    writeln!(writer, "FT                   /{key}")?;
                } else {
                    writeln!(writer, "FT                   /{key}=\"{value}\"")?;
                }
            }
        }
        writeln!(writer, "SQ   Sequence {} BP;", record.len())?;
        for line in write_sequence_blocks(record.residues(), 60, 10) {
            writeln!(writer, "{line}")?;
        }
        writeln!(writer, "//")?;
    }

    Ok(())
}

/// Writes EMBL records to a `String`.
pub fn write_embl_string(records: &[SequenceRecord]) -> Result<String, IoError> {
    let mut buffer = Vec::new();
    write_embl_writer(&mut buffer, records)?;
    String::from_utf8(buffer)
        .map_err(|error| IoError::parse("embl", None, format!("non-utf8 output: {error}")))
}

fn parse_embl_record(lines: &[String]) -> Result<SequenceRecord, IoError> {
    let mut accession: Option<String> = None;
    let mut description_parts = Vec::new();
    let mut organism_parts = Vec::new();
    let mut taxonomy_parts = Vec::new();
    let mut molecule: Option<MoleculeKind> = None;
    let mut in_sequence = false;
    let mut sequence = String::new();
    let mut raw_features: Vec<RawFeatureEntry> = Vec::new();

    for (index, line) in lines.iter().enumerate() {
        let line_number = index + 1;
        let key = line.get(..2).unwrap_or("").trim();

        if in_sequence {
            sequence.push_str(&sequence_from_flatfile_line(line));
            continue;
        }

        match key {
            "ID" => {
                let content = line.get(5..).unwrap_or("").trim();
                if let Some((identifier, _)) = content.split_once(';') {
                    accession = Some(identifier.trim().to_owned());
                }

                let upper = content.to_ascii_uppercase();
                molecule = if upper.contains("RNA") {
                    Some(MoleculeKind::Rna)
                } else if upper.contains(" AA") {
                    Some(MoleculeKind::Protein)
                } else if upper.contains(" DNA") {
                    Some(MoleculeKind::Dna)
                } else {
                    molecule
                };
            }
            "AC" => {
                let content = line.get(5..).unwrap_or("").trim();
                if let Some(first) = content
                    .trim_end_matches(';')
                    .split(';')
                    .map(str::trim)
                    .find(|value| !value.is_empty())
                {
                    accession.get_or_insert_with(|| first.to_owned());
                }
            }
            "DE" => description_parts.push(line.get(5..).unwrap_or("").trim().to_owned()),
            "OS" => organism_parts.push(line.get(5..).unwrap_or("").trim().to_owned()),
            "OC" => {
                taxonomy_parts.push(line.get(5..).unwrap_or("").trim_end_matches('.').to_owned())
            }
            "FT" => parse_embl_feature_line(line, line_number, &mut raw_features)?,
            "SQ" => in_sequence = true,
            _ => {}
        }
    }

    if sequence.is_empty() {
        return Err(IoError::parse(
            "embl",
            None,
            "record is missing an SQ sequence section",
        ));
    }

    let accession =
        accession.ok_or_else(|| IoError::parse("embl", None, "record is missing an accession"))?;
    let identifier = SequenceIdentifier::new(accession)?;
    let molecule = molecule.unwrap_or_else(|| infer_molecule_kind(&sequence));
    let mut metadata = SequenceMetadata::new();
    if !description_parts.is_empty() {
        metadata = metadata.with_description(description_parts.join(" "));
    }
    if !organism_parts.is_empty() {
        metadata = metadata.with_organism(organism_parts.join(" "));
    }
    if !taxonomy_parts.is_empty() {
        metadata = metadata.with_source(taxonomy_parts.join("; "));
    }

    let record = SequenceRecord::new(identifier, molecule, sequence)?.with_metadata(metadata);
    let features = raw_features
        .into_iter()
        .map(|(kind, location, line_number, qualifiers)| {
            let location = parse_location("embl", &location, Some(line_number))?;
            let qualifiers = parse_qualifiers("embl", &qualifiers)?;
            Ok(feature_from_parts(&kind, location, qualifiers))
        })
        .collect::<Result<Vec<_>, IoError>>()?;

    finalize_record_features(record, features)
}

fn parse_embl_feature_line(
    line: &str,
    line_number: usize,
    raw_features: &mut Vec<RawFeatureEntry>,
) -> Result<(), IoError> {
    let key_field = line.get(5..20).unwrap_or("").trim();
    let value_field = line.get(21..).unwrap_or("").trim();

    if key_field.is_empty() {
        let current = raw_features.last_mut().ok_or_else(|| {
            IoError::parse(
                "embl",
                Some(line_number),
                "feature continuation without a feature",
            )
        })?;
        current.3.push((line_number, value_field.to_owned()));
        return Ok(());
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

    use super::{parse_embl_str, write_embl_string};

    #[test]
    fn parses_representative_embl_record() {
        let records = parse_embl_str(
            "ID   X56734; SV 1; linear; mRNA; STD; PLN; 1859 BP.\n\
AC   X56734;\n\
DE   Trifolium repens mRNA for non-cyanogenic beta-glucosidase\n\
OS   Trifolium repens\n\
OC   Eukaryota; Viridiplantae.\n\
FH   Key             Location/Qualifiers\n\
FH\n\
FT   gene            1..50\n\
FT                   /gene=\"bglA\"\n\
FT   CDS             complement(10..45)\n\
FT                   /product=\"beta-glucosidase\"\n\
SQ   Sequence 50 BP;\n\
     augcaugcau gcaugcaugc augcaugcau gcaugcaugc augcaugcau 50\n\
//\n",
        )
        .expect("embl should parse");

        assert_eq!(records.len(), 1);
        let record = &records[0];
        assert_eq!(record.molecule(), MoleculeKind::Rna);
        assert_eq!(record.identifier().accession(), "X56734");
        assert_eq!(
            record.metadata().organism.as_deref(),
            Some("Trifolium repens")
        );
        assert_eq!(record.features().len(), 2);
        assert!(matches!(record.features()[0].kind, FeatureKind::Gene));
        assert_eq!(
            record.features()[1].location.strand(),
            Some(Strand::Reverse)
        );
    }

    #[test]
    fn round_trips_supported_embl_subset() {
        let parsed = parse_embl_str(
            "ID   TEST1; SV 1; linear; DNA; UNC; 12 BP.\n\
AC   TEST1;\n\
DE   Example sequence\n\
OS   Example organism\n\
FH   Key             Location/Qualifiers\n\
FH\n\
FT   gene            join(1..3,7..9)\n\
FT                   /gene=\"abc\"\n\
SQ   Sequence 12 BP;\n\
     acgtacgtacgt 12\n\
//\n",
        )
        .expect("embl should parse");

        let rendered = write_embl_string(&parsed).expect("embl should render");
        let reparsed = parse_embl_str(&rendered).expect("rendered embl should parse");
        assert_eq!(reparsed, parsed);
    }

    #[test]
    fn rejects_missing_sequence_section() {
        let error = parse_embl_str("ID   TEST;\nAC   TEST;\n//\n")
            .expect_err("sequence section should be required");
        assert!(error.to_string().contains("SQ"));
    }

    #[test]
    fn rejects_unsupported_feature_syntax() {
        let error = parse_embl_str(
            "ID   TEST; SV 1; linear; DNA; UNC; 12 BP.\n\
AC   TEST;\n\
FH   Key             Location/Qualifiers\n\
FH\n\
FT   gene            order(1..3,7..9)\n\
SQ   Sequence 12 BP;\n\
     acgtacgtacgt 12\n\
//\n",
        )
        .expect_err("unsupported location should fail");
        assert!(error.to_string().contains("unsupported feature location"));
    }
}

//! Shared helpers for EMBL and GenBank flat-file support.

use std::collections::BTreeMap;

use emboss_core::{
    Feature, FeatureKind, FeatureLocation, FeatureSpan, Interval, MoleculeKind, SequenceRecord,
    Strand,
};

use crate::error::IoError;

pub(crate) fn infer_molecule_kind(residues: &str) -> MoleculeKind {
    let uppercase: String = residues
        .chars()
        .filter(|symbol| !symbol.is_whitespace())
        .map(|symbol| symbol.to_ascii_uppercase())
        .collect();

    let has_u = uppercase.contains('U');
    let has_t = uppercase.contains('T');

    if has_u
        && !has_t
        && emboss_core::Alphabet::Rna
            .validate(MoleculeKind::Rna, &uppercase)
            .is_ok()
    {
        return MoleculeKind::Rna;
    }

    if has_t
        && !has_u
        && emboss_core::Alphabet::Dna
            .validate(MoleculeKind::Dna, &uppercase)
            .is_ok()
    {
        return MoleculeKind::Dna;
    }

    let dna_like = emboss_core::Alphabet::Dna
        .validate(MoleculeKind::Dna, &uppercase)
        .is_ok();
    let rna_like = emboss_core::Alphabet::Rna
        .validate(MoleculeKind::Rna, &uppercase)
        .is_ok();
    if !dna_like
        && !rna_like
        && emboss_core::Alphabet::Protein
            .validate(MoleculeKind::Protein, &uppercase)
            .is_ok()
    {
        return MoleculeKind::Protein;
    }

    MoleculeKind::Unknown
}

pub(crate) fn sequence_from_flatfile_line(line: &str) -> String {
    line.chars()
        .filter(|symbol| symbol.is_ascii_alphabetic() || matches!(symbol, '*' | '-'))
        .collect()
}

pub(crate) fn parse_feature_kind(raw: &str) -> FeatureKind {
    match raw.trim() {
        "gene" => FeatureKind::Gene,
        "CDS" => FeatureKind::CodingSequence,
        "exon" => FeatureKind::Exon,
        "intron" => FeatureKind::Intron,
        "repeat_region" => FeatureKind::RepeatRegion,
        "misc_feature" => FeatureKind::MiscFeature,
        "region" => FeatureKind::Region,
        "motif" | "site" => FeatureKind::Motif,
        other => FeatureKind::Other(other.to_owned()),
    }
}

pub(crate) fn format_feature_kind(kind: &FeatureKind) -> String {
    match kind {
        FeatureKind::Gene => "gene".to_owned(),
        FeatureKind::CodingSequence => "CDS".to_owned(),
        FeatureKind::Exon => "exon".to_owned(),
        FeatureKind::Intron => "intron".to_owned(),
        FeatureKind::Region => "region".to_owned(),
        FeatureKind::Motif => "motif".to_owned(),
        FeatureKind::RepeatRegion => "repeat_region".to_owned(),
        FeatureKind::MiscFeature => "misc_feature".to_owned(),
        FeatureKind::Other(label) => label.clone(),
    }
}

pub(crate) fn parse_location(
    format: &'static str,
    location: &str,
    line: Option<usize>,
) -> Result<FeatureLocation, IoError> {
    let spans = parse_location_inner(format, location.trim(), line, Strand::Forward)?;
    FeatureLocation::from_spans(spans).map_err(IoError::from)
}

fn parse_location_inner(
    format: &'static str,
    location: &str,
    line: Option<usize>,
    strand: Strand,
) -> Result<Vec<FeatureSpan>, IoError> {
    let compact: String = location
        .chars()
        .filter(|symbol| !symbol.is_whitespace())
        .collect();
    if compact.is_empty() {
        return Err(IoError::parse(
            format,
            line,
            "feature location must not be empty",
        ));
    }

    if compact.contains('^') || compact.contains(':') || compact.starts_with("order(") {
        return Err(IoError::parse(
            format,
            line,
            format!("unsupported feature location syntax '{location}'"),
        ));
    }

    if let Some(inner) = strip_wrapper(&compact, "complement") {
        return parse_location_inner(format, inner, line, strand.opposite());
    }

    if let Some(inner) = strip_wrapper(&compact, "join") {
        let mut spans = Vec::new();
        for part in split_top_level(inner, ',')? {
            spans.extend(parse_location_inner(format, part, line, strand)?);
        }
        return Ok(spans);
    }

    let interval = parse_simple_interval(format, &compact, line)?;
    Ok(vec![FeatureSpan::new(interval, strand)])
}

fn strip_wrapper<'a>(value: &'a str, name: &str) -> Option<&'a str> {
    let prefix = format!("{name}(");
    value
        .strip_prefix(&prefix)
        .and_then(|rest| rest.strip_suffix(')'))
}

fn split_top_level(value: &str, delimiter: char) -> Result<Vec<&str>, IoError> {
    let mut parts = Vec::new();
    let mut depth = 0usize;
    let mut start = 0usize;

    for (index, symbol) in value.char_indices() {
        match symbol {
            '(' => depth += 1,
            ')' => depth = depth.saturating_sub(1),
            c if c == delimiter && depth == 0 => {
                parts.push(&value[start..index]);
                start = index + symbol.len_utf8();
            }
            _ => {}
        }
    }

    if depth != 0 {
        return Err(IoError::parse(
            "flatfile",
            None,
            "unbalanced parentheses in feature location",
        ));
    }

    parts.push(&value[start..]);
    Ok(parts)
}

fn parse_simple_interval(
    format: &'static str,
    value: &str,
    line: Option<usize>,
) -> Result<Interval, IoError> {
    if let Some((start, end)) = value.split_once("..") {
        let start = parse_position(start, format, line)?;
        let end = parse_position(end, format, line)?;
        if start > end {
            return Err(IoError::parse(
                format,
                line,
                format!("feature location start {start} exceeds end {end}"),
            ));
        }
        return Interval::new(start - 1, end).map_err(IoError::from);
    }

    let position = parse_position(value, format, line)?;
    Interval::new(position - 1, position).map_err(IoError::from)
}

fn parse_position(
    value: &str,
    format: &'static str,
    line: Option<usize>,
) -> Result<usize, IoError> {
    let trimmed = value.trim_matches(|symbol| matches!(symbol, '<' | '>'));
    trimmed.parse::<usize>().map_err(|_| {
        IoError::parse(
            format,
            line,
            format!("unsupported feature coordinate '{value}'"),
        )
    })
}

pub(crate) fn format_location(location: &FeatureLocation) -> String {
    let spans = location.spans();
    let forward_spans: Vec<String> = spans
        .iter()
        .map(|span| format_interval(span.interval()))
        .collect();

    let inner = if forward_spans.len() == 1 {
        forward_spans[0].clone()
    } else {
        format!("join({})", forward_spans.join(","))
    };

    match location.strand() {
        Some(Strand::Reverse) => format!("complement({inner})"),
        _ => inner,
    }
}

fn format_interval(interval: Interval) -> String {
    let start = interval.start() + 1;
    let end = interval.end();
    if interval.len() == 1 {
        start.to_string()
    } else {
        format!("{start}..{end}")
    }
}

pub(crate) fn parse_qualifiers(
    format: &'static str,
    lines: &[(usize, String)],
) -> Result<BTreeMap<String, String>, IoError> {
    let mut qualifiers = BTreeMap::new();
    let mut open_key: Option<String> = None;
    let mut open_value = String::new();

    for (line_number, line) in lines {
        let trimmed = line.trim();
        if let Some(rest) = trimmed.strip_prefix('/') {
            if let Some(key) = open_key.take() {
                qualifiers.insert(key, open_value.trim().to_owned());
                open_value.clear();
            }

            let (key, value) = match rest.split_once('=') {
                Some((key, value)) => (key.trim(), value.trim()),
                None => {
                    qualifiers.insert(rest.trim().to_owned(), "true".to_owned());
                    continue;
                }
            };

            if let Some(stripped) = value.strip_prefix('"') {
                if let Some(content) = stripped.strip_suffix('"') {
                    qualifiers.insert(key.to_owned(), content.to_owned());
                } else {
                    open_key = Some(key.to_owned());
                    open_value.push_str(stripped);
                }
            } else {
                qualifiers.insert(key.to_owned(), value.to_owned());
            }
        } else if open_key.is_some() {
            if let Some(content) = trimmed.strip_suffix('"') {
                if !open_value.is_empty() {
                    open_value.push(' ');
                }
                open_value.push_str(content);
                let key = open_key.take().expect("open key should exist");
                qualifiers.insert(key, open_value.trim().to_owned());
                open_value.clear();
            } else {
                if !open_value.is_empty() {
                    open_value.push(' ');
                }
                open_value.push_str(trimmed);
            }
        } else {
            return Err(IoError::parse(
                format,
                Some(*line_number),
                "unsupported feature qualifier continuation",
            ));
        }
    }

    if let Some(key) = open_key.take() {
        qualifiers.insert(key, open_value.trim().to_owned());
    }

    Ok(qualifiers)
}

pub(crate) fn feature_from_parts(
    kind_label: &str,
    location: FeatureLocation,
    qualifiers: BTreeMap<String, String>,
) -> Feature {
    let mut feature = Feature::new(parse_feature_kind(kind_label), location);

    if let Some(name) = qualifiers
        .get("gene")
        .or_else(|| qualifiers.get("locus_tag"))
        .or_else(|| qualifiers.get("label"))
        .filter(|value| !value.is_empty())
    {
        feature = feature.with_name(name.clone());
    }

    if let Some(note) = qualifiers.get("note").filter(|value| !value.is_empty()) {
        feature = feature.with_note(note.clone());
    }

    for (key, value) in qualifiers {
        feature = feature.with_qualifier(key, value);
    }

    feature
}

pub(crate) fn format_feature_qualifiers(feature: &Feature) -> BTreeMap<String, String> {
    let mut qualifiers = feature.qualifiers.clone();

    if let Some(name) = &feature.name {
        let default_name_key = if matches!(feature.kind, FeatureKind::Gene) {
            "gene"
        } else {
            "label"
        };
        qualifiers
            .entry(default_name_key.to_owned())
            .or_insert_with(|| name.clone());
    }

    if let Some(note) = &feature.note {
        qualifiers
            .entry("note".to_owned())
            .or_insert_with(|| note.clone());
    }

    qualifiers
}

pub(crate) fn write_sequence_blocks(
    sequence: &str,
    line_width: usize,
    group_width: usize,
) -> Vec<String> {
    let lowercase = sequence.to_ascii_lowercase();
    lowercase
        .as_bytes()
        .chunks(line_width)
        .enumerate()
        .map(|(index, chunk)| {
            let grouped = chunk
                .chunks(group_width)
                .map(|part| String::from_utf8_lossy(part).into_owned())
                .collect::<Vec<_>>()
                .join(" ");
            let count = ((index + 1) * line_width).min(sequence.len());
            format!(
                "     {:<width$} {}",
                grouped,
                count,
                width = line_width + (line_width / group_width).saturating_sub(1)
            )
        })
        .collect()
}

pub(crate) fn finalize_record_features(
    mut record: SequenceRecord,
    features: Vec<Feature>,
) -> Result<SequenceRecord, IoError> {
    for feature in features {
        record.add_feature(feature)?;
    }
    Ok(record)
}

#[cfg(test)]
mod tests {
    use emboss_core::{Interval, Strand};

    use super::{format_location, parse_location, parse_qualifiers};

    #[test]
    fn parses_simple_and_joined_locations() {
        let simple =
            parse_location("genbank", "5..10", None).expect("simple location should parse");
        assert_eq!(
            simple.spans()[0].interval(),
            Interval::new(4, 10).expect("valid interval")
        );

        let joined = parse_location("genbank", "complement(join(5..10,20..25))", None)
            .expect("join should parse");
        assert_eq!(joined.spans().len(), 2);
        assert_eq!(joined.spans()[0].strand(), Strand::Reverse);
        assert_eq!(format_location(&joined), "complement(join(5..10,20..25))");
    }

    #[test]
    fn parses_multiline_qualifiers() {
        let qualifiers = parse_qualifiers(
            "embl",
            &[
                (1, "/note=\"first".to_owned()),
                (2, "continued\"".to_owned()),
                (3, "/pseudo".to_owned()),
            ],
        )
        .expect("qualifiers should parse");

        assert_eq!(
            qualifiers.get("note").map(String::as_str),
            Some("first continued")
        );
        assert_eq!(qualifiers.get("pseudo").map(String::as_str), Some("true"));
    }
}

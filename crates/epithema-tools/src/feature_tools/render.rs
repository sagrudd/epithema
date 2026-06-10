use epithema_core::{Feature, FeatureKind, FeatureLocation, FeatureSpan, Strand};

pub fn render_feature_kind(kind: &FeatureKind) -> String {
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

pub fn render_feature_location(location: &FeatureLocation) -> String {
    match location.spans() {
        [span] => render_feature_span(*span),
        spans => format!(
            "join({})",
            spans
                .iter()
                .map(|span| render_feature_span(*span))
                .collect::<Vec<_>>()
                .join(",")
        ),
    }
}

pub fn render_feature_text(feature: &Feature) -> String {
    let mut lines = vec![format!(
        "     {:<15} {}",
        render_feature_kind(&feature.kind),
        render_feature_location(&feature.location)
    )];

    if let Some(name) = &feature.name {
        if !feature.qualifiers.values().any(|value| value == name) {
            lines.push(format!(
                "                     /name=\"{}\"",
                escape_qualifier_value(name)
            ));
        }
    }
    if let Some(note) = &feature.note {
        if feature.qualifiers.get("note") != Some(note) {
            lines.push(format!(
                "                     /note=\"{}\"",
                escape_qualifier_value(note)
            ));
        }
    }
    for (key, value) in &feature.qualifiers {
        lines.push(format!(
            "                     /{}=\"{}\"",
            key,
            escape_qualifier_value(value)
        ));
    }

    lines.join("\n")
}

pub fn render_feature_strand(strand: Option<Strand>) -> &'static str {
    match strand {
        Some(Strand::Forward) => "forward",
        Some(Strand::Reverse) => "reverse",
        Some(Strand::Unknown) | Some(Strand::Unstranded) | None => "unknown",
    }
}

fn render_feature_span(span: FeatureSpan) -> String {
    let bounds = format!("{}..{}", span.interval().start() + 1, span.interval().end());
    match span.strand() {
        Strand::Forward | Strand::Unknown | Strand::Unstranded => bounds,
        Strand::Reverse => format!("complement({bounds})"),
    }
}

fn escape_qualifier_value(value: &str) -> String {
    value.replace('"', "'")
}

#[cfg(test)]
mod tests {
    use epithema_core::{Feature, FeatureKind, FeatureLocation, FeatureSpan, Interval, Strand};

    use super::{render_feature_location, render_feature_text};

    #[test]
    fn renders_joined_reverse_aware_locations() {
        let location = FeatureLocation::from_spans(vec![
            FeatureSpan::new(
                Interval::new(1, 4).expect("valid interval"),
                Strand::Reverse,
            ),
            FeatureSpan::new(
                Interval::new(7, 9).expect("valid interval"),
                Strand::Reverse,
            ),
        ])
        .expect("valid location");

        assert_eq!(
            render_feature_location(&location),
            "join(complement(2..4),complement(8..9))"
        );
    }

    #[test]
    fn renders_normalized_feature_text() {
        let feature = Feature::new(
            FeatureKind::Gene,
            FeatureLocation::new(
                Interval::new(1, 6).expect("valid interval"),
                Strand::Forward,
            ),
        )
        .with_name("geneA")
        .with_note("example")
        .with_qualifier("gene", "geneA")
        .with_qualifier("product", "demo");

        let rendered = render_feature_text(&feature);
        assert!(rendered.contains("gene            2..6"));
        assert!(rendered.contains("/note=\"example\""));
        assert!(rendered.contains("/gene=\"geneA\""));
        assert!(rendered.contains("/product=\"demo\""));
    }
}

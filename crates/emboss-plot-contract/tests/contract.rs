//! Integration tests for the typed EMBOSS-RS plot contract.

use std::fs;
use std::path::PathBuf;

use emboss_plot_contract::{
    AxisScaleHint, DataVector, GeometryHint, PlotAnnotation, PlotAxis, PlotKind, PlotMetadata,
    PlotProvenance, PlotReferenceAxis, PlotReferenceLine, PlotSeries, PlotSpec, SeriesStyle,
};

fn fixture_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
        .join(name)
}

fn example_line_spec() -> PlotSpec {
    PlotSpec::new(
        PlotKind::Line,
        PlotMetadata::new("gc_profile", "GC profile")
            .with_subtitle("Windowed summary")
            .with_provenance(PlotProvenance {
                tool: Some("geecee".to_owned()),
                method: Some("windowed_gc".to_owned()),
                source_artifact_ids: vec!["table:gc".to_owned()],
            }),
        PlotAxis::new("Window start").with_scale_hint(AxisScaleHint::Linear),
        PlotAxis::new("GC proportion").with_scale_hint(AxisScaleHint::Linear),
        vec![
            PlotSeries::new(
                "sample_a",
                "Sample A",
                DataVector::Numeric(vec![1.0, 5.0, 9.0]),
                vec![0.42, 0.55, 0.47],
            )
            .with_legend_label("Sample A")
            .with_semantic_group("sample")
            .with_style(
                SeriesStyle::empty()
                    .with_geometry_hint(GeometryHint::Line)
                    .with_color_role("primary"),
            ),
            PlotSeries::new(
                "sample_b",
                "Sample B",
                DataVector::Numeric(vec![1.0, 5.0, 9.0]),
                vec![0.38, 0.44, 0.41],
            )
            .with_legend_label("Sample B")
            .with_semantic_group("sample")
            .with_style(
                SeriesStyle::empty()
                    .with_geometry_hint(GeometryHint::Line)
                    .with_color_role("secondary"),
            ),
        ],
    )
    .with_annotation(PlotAnnotation {
        label: Some("target".to_owned()),
        reference_line: Some(PlotReferenceLine {
            axis: PlotReferenceAxis::Y,
            value: 0.5,
        }),
    })
}

#[test]
fn serializes_to_stable_fixture() {
    let json = example_line_spec()
        .to_json_pretty()
        .expect("example plot should serialize");
    let fixture =
        fs::read_to_string(fixture_path("line_plot.json")).expect("line fixture should exist");
    assert_eq!(json.trim(), fixture.trim());
}

#[test]
fn deserializes_fixture_and_validates() {
    let fixture =
        fs::read_to_string(fixture_path("line_plot.json")).expect("line fixture should exist");
    let spec = PlotSpec::from_json_str(&fixture).expect("fixture should parse");
    assert_eq!(spec.kind, PlotKind::Line);
    assert_eq!(spec.series.len(), 2);
}

#[test]
fn rejects_geometry_kind_mismatch() {
    let spec = PlotSpec::new(
        PlotKind::Bar,
        PlotMetadata::new("bad_bar", "Bad bar"),
        PlotAxis::new("Category"),
        PlotAxis::new("Value"),
        vec![
            PlotSeries::new(
                "series",
                "Series",
                DataVector::Text(vec!["A".to_owned(), "B".to_owned()]),
                vec![1.0, 2.0],
            )
            .with_style(SeriesStyle::empty().with_geometry_hint(GeometryHint::Line)),
        ],
    );

    let error = spec
        .validate()
        .expect_err("mismatched geometry should fail");
    assert!(format!("{error}").contains("geometry hint"));
}

#[test]
fn rejects_inconsistent_vector_lengths() {
    let spec = PlotSpec::new(
        PlotKind::Scatter,
        PlotMetadata::new("scatter", "Scatter"),
        PlotAxis::new("X"),
        PlotAxis::new("Y"),
        vec![
            PlotSeries::new(
                "points",
                "Points",
                DataVector::Numeric(vec![1.0, 2.0]),
                vec![1.0],
            )
            .with_style(SeriesStyle::empty().with_geometry_hint(GeometryHint::Point)),
        ],
    );

    let error = spec.validate().expect_err("length mismatch should fail");
    assert!(format!("{error}").contains("inconsistent x/y lengths"));
}

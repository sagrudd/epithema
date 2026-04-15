use serde::{Deserialize, Serialize};

use crate::axis::PlotAxis;
use crate::error::PlotContractError;
use crate::facet::PlotFacet;
use crate::metadata::PlotMetadata;
use crate::series::{GeometryHint, PlotSeries};

/// Supported v1 plot families.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PlotKind {
    /// Connected line plot.
    Line,
    /// Scatter plot.
    Scatter,
    /// Bar plot.
    Bar,
}

impl PlotKind {
    /// Returns a stable plot-kind label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Line => "line",
            Self::Scatter => "scatter",
            Self::Bar => "bar",
        }
    }

    fn expected_geometry(self) -> GeometryHint {
        match self {
            Self::Line => GeometryHint::Line,
            Self::Scatter => GeometryHint::Point,
            Self::Bar => GeometryHint::Bar,
        }
    }
}

/// Axis selector for a reference-line annotation.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PlotReferenceAxis {
    /// Vertical reference line on the x axis.
    X,
    /// Horizontal reference line on the y axis.
    Y,
}

/// Optional reference-line annotation.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PlotReferenceLine {
    /// Axis the reference line belongs to.
    pub axis: PlotReferenceAxis,
    /// Numeric line position.
    pub value: f64,
}

/// Optional annotation block.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PlotAnnotation {
    /// Optional human-readable label.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub label: Option<String>,
    /// Optional reference line payload.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reference_line: Option<PlotReferenceLine>,
}

/// Full plot specification handed from Rust to R.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PlotSpec {
    /// Contract version for forward compatibility.
    pub version: u32,
    /// Plot family.
    pub kind: PlotKind,
    /// Display metadata.
    pub metadata: PlotMetadata,
    /// X-axis metadata.
    pub x_axis: PlotAxis,
    /// Y-axis metadata.
    pub y_axis: PlotAxis,
    /// One or more data series.
    pub series: Vec<PlotSeries>,
    /// Optional annotation blocks.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub annotations: Vec<PlotAnnotation>,
    /// Optional faceting metadata.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub facet: Option<PlotFacet>,
}

impl PlotSpec {
    /// Current plot-contract version.
    pub const VERSION: u32 = 1;

    /// Creates a new plot specification with no annotations or faceting.
    #[must_use]
    pub fn new(
        kind: PlotKind,
        metadata: PlotMetadata,
        x_axis: PlotAxis,
        y_axis: PlotAxis,
        series: Vec<PlotSeries>,
    ) -> Self {
        Self {
            version: Self::VERSION,
            kind,
            metadata,
            x_axis,
            y_axis,
            series,
            annotations: Vec::new(),
            facet: None,
        }
    }

    /// Creates an empty placeholder line plot with the supplied title.
    #[must_use]
    pub fn empty(title: impl Into<String>) -> Self {
        Self::new(
            PlotKind::Line,
            PlotMetadata::new("plot", title),
            PlotAxis::new("x"),
            PlotAxis::new("y"),
            vec![
                PlotSeries::new(
                    "placeholder",
                    "Placeholder",
                    crate::series::DataVector::Numeric(vec![0.0]),
                    vec![0.0],
                )
                .with_style(
                    crate::series::SeriesStyle::empty()
                        .with_geometry_hint(crate::series::GeometryHint::Line),
                ),
            ],
        )
    }

    /// Attaches an annotation.
    #[must_use]
    pub fn with_annotation(mut self, annotation: PlotAnnotation) -> Self {
        self.annotations.push(annotation);
        self
    }

    /// Attaches a facet descriptor.
    #[must_use]
    pub fn with_facet(mut self, facet: PlotFacet) -> Self {
        self.facet = Some(facet);
        self
    }

    /// Validates the plot contract for v1 constraints.
    pub fn validate(&self) -> Result<(), PlotContractError> {
        if self.metadata.title.trim().is_empty() {
            return Err(PlotContractError::EmptyTitle);
        }
        if self.x_axis.label.trim().is_empty() {
            return Err(PlotContractError::MissingAxisLabel { axis: "x" });
        }
        if self.y_axis.label.trim().is_empty() {
            return Err(PlotContractError::MissingAxisLabel { axis: "y" });
        }
        if self.series.is_empty() {
            return Err(PlotContractError::EmptySeriesSet);
        }

        let expected_geometry = self.kind.expected_geometry();
        let first_kind = self.series[0].x.kind_label();
        for series in &self.series {
            let x_len = series.x.len();
            let y_len = series.y.len();
            if series.x.is_empty() || series.y.is_empty() {
                return Err(PlotContractError::EmptySeriesData {
                    series_id: series.id.clone(),
                });
            }
            if x_len != y_len {
                return Err(PlotContractError::InconsistentSeriesLengths {
                    series_id: series.id.clone(),
                    x_len,
                    y_len,
                });
            }
            if series.x.kind_label() != first_kind {
                return Err(PlotContractError::MixedXValueKinds);
            }
            if let Some(style) = &series.style
                && let Some(geometry_hint) = style.geometry_hint
                && geometry_hint != expected_geometry
            {
                return Err(PlotContractError::GeometryKindMismatch {
                    series_id: series.id.clone(),
                    plot_kind: self.kind.as_str(),
                    geometry_hint: geometry_hint.as_str(),
                });
            }
        }

        Ok(())
    }

    /// Serializes the validated plot contract to pretty JSON.
    pub fn to_json_pretty(&self) -> Result<String, PlotContractError> {
        self.validate()?;
        serde_json::to_string_pretty(self)
            .map_err(|error| PlotContractError::Serialization(error.to_string()))
    }

    /// Deserializes a plot contract from JSON and validates it.
    pub fn from_json_str(json: &str) -> Result<Self, PlotContractError> {
        let spec = serde_json::from_str::<Self>(json)
            .map_err(|error| PlotContractError::Deserialization(error.to_string()))?;
        spec.validate()?;
        Ok(spec)
    }
}

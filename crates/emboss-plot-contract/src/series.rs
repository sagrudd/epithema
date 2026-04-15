use serde::{Deserialize, Serialize};

/// X-axis data vector supported by the v1 contract.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", content = "values", rename_all = "snake_case")]
pub enum DataVector {
    /// Continuous numeric x coordinates.
    Numeric(Vec<f64>),
    /// Discrete categorical x coordinates.
    Text(Vec<String>),
}

impl DataVector {
    /// Returns the vector length.
    #[must_use]
    pub fn len(&self) -> usize {
        match self {
            Self::Numeric(values) => values.len(),
            Self::Text(values) => values.len(),
        }
    }

    /// Returns whether the vector is empty.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns a stable kind label.
    #[must_use]
    pub const fn kind_label(&self) -> &'static str {
        match self {
            Self::Numeric(_) => "numeric",
            Self::Text(_) => "text",
        }
    }
}

/// Renderer hint for series geometry.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GeometryHint {
    /// Connected line geometry.
    Line,
    /// Point geometry.
    Point,
    /// Bar geometry.
    Bar,
}

impl GeometryHint {
    /// Returns a stable geometry label.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Line => "line",
            Self::Point => "point",
            Self::Bar => "bar",
        }
    }
}

/// Semantic style hints for a data series.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct SeriesStyle {
    /// Optional geometry hint aligned with the plot kind.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub geometry_hint: Option<GeometryHint>,
    /// Optional semantic color role such as `primary` or `reference`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub color_role: Option<String>,
}

impl SeriesStyle {
    /// Creates an empty style hint block.
    #[must_use]
    pub fn empty() -> Self {
        Self {
            geometry_hint: None,
            color_role: None,
        }
    }

    /// Attaches a geometry hint.
    #[must_use]
    pub fn with_geometry_hint(mut self, geometry_hint: GeometryHint) -> Self {
        self.geometry_hint = Some(geometry_hint);
        self
    }

    /// Attaches a semantic color role.
    #[must_use]
    pub fn with_color_role(mut self, color_role: impl Into<String>) -> Self {
        self.color_role = Some(color_role.into());
        self
    }
}

/// One named data series in a plot.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct PlotSeries {
    /// Stable series identifier.
    pub id: String,
    /// Human-readable series label.
    pub label: String,
    /// Optional legend label override.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub legend_label: Option<String>,
    /// Optional semantic grouping label.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub semantic_group: Option<String>,
    /// Optional facet value.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub facet: Option<String>,
    /// X-axis vector.
    pub x: DataVector,
    /// Y-axis vector.
    pub y: Vec<f64>,
    /// Optional style hints.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub style: Option<SeriesStyle>,
}

impl PlotSeries {
    /// Creates a plot series from typed x/y vectors.
    #[must_use]
    pub fn new(
        id: impl Into<String>,
        label: impl Into<String>,
        x: DataVector,
        y: Vec<f64>,
    ) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            legend_label: None,
            semantic_group: None,
            facet: None,
            x,
            y,
            style: None,
        }
    }

    /// Attaches a legend label override.
    #[must_use]
    pub fn with_legend_label(mut self, legend_label: impl Into<String>) -> Self {
        self.legend_label = Some(legend_label.into());
        self
    }

    /// Attaches a semantic grouping label.
    #[must_use]
    pub fn with_semantic_group(mut self, semantic_group: impl Into<String>) -> Self {
        self.semantic_group = Some(semantic_group.into());
        self
    }

    /// Attaches a facet value.
    #[must_use]
    pub fn with_facet(mut self, facet: impl Into<String>) -> Self {
        self.facet = Some(facet.into());
        self
    }

    /// Attaches style hints.
    #[must_use]
    pub fn with_style(mut self, style: SeriesStyle) -> Self {
        self.style = Some(style);
        self
    }
}

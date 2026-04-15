use serde::{Deserialize, Serialize};

/// Faceting scale policy hint for the renderer.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FacetScaleMode {
    /// All facets should share scales.
    Shared,
    /// Facets may use free scales.
    Free,
}

/// Optional faceting metadata for a plot.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct PlotFacet {
    /// Human-readable facet label.
    pub label: String,
    /// Semantic scale policy hint.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scale_mode: Option<FacetScaleMode>,
}

impl PlotFacet {
    /// Creates a facet descriptor.
    #[must_use]
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            scale_mode: None,
        }
    }

    /// Attaches a facet scale policy hint.
    #[must_use]
    pub fn with_scale_mode(mut self, scale_mode: FacetScaleMode) -> Self {
        self.scale_mode = Some(scale_mode);
        self
    }
}

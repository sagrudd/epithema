use serde::{Deserialize, Serialize};

/// Semantic scale hint for an axis. Rendering remains R-owned.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AxisScaleHint {
    /// Continuous numeric scaling.
    Linear,
    /// Discrete categorical scaling.
    Categorical,
}

/// Plot axis metadata.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct PlotAxis {
    /// Human-readable axis label.
    pub label: String,
    /// Optional semantic scale hint for the renderer.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scale_hint: Option<AxisScaleHint>,
}

impl PlotAxis {
    /// Creates a labeled axis.
    #[must_use]
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            scale_hint: None,
        }
    }

    /// Attaches a semantic scale hint.
    #[must_use]
    pub fn with_scale_hint(mut self, scale_hint: AxisScaleHint) -> Self {
        self.scale_hint = Some(scale_hint);
        self
    }
}

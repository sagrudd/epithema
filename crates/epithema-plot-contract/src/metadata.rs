use serde::{Deserialize, Serialize};

/// Provenance metadata describing where a plot-ready payload came from.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct PlotProvenance {
    /// Optional governing tool or method identifier.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tool: Option<String>,
    /// Optional internal method or workflow identifier.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub method: Option<String>,
    /// Optional source artefact identifiers for later cross-linking.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub source_artifact_ids: Vec<String>,
}

impl PlotProvenance {
    /// Creates an empty provenance block.
    #[must_use]
    pub fn empty() -> Self {
        Self {
            tool: None,
            method: None,
            source_artifact_ids: Vec::new(),
        }
    }
}

/// Human-readable metadata for a plot payload.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct PlotMetadata {
    /// Stable plot identifier.
    pub id: String,
    /// Display title.
    pub title: String,
    /// Optional subtitle.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subtitle: Option<String>,
    /// Optional caption or note.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub caption: Option<String>,
    /// Optional provenance block.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub provenance: Option<PlotProvenance>,
}

impl PlotMetadata {
    /// Creates required plot metadata.
    #[must_use]
    pub fn new(id: impl Into<String>, title: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            subtitle: None,
            caption: None,
            provenance: None,
        }
    }

    /// Attaches a subtitle.
    #[must_use]
    pub fn with_subtitle(mut self, subtitle: impl Into<String>) -> Self {
        self.subtitle = Some(subtitle.into());
        self
    }

    /// Attaches a caption.
    #[must_use]
    pub fn with_caption(mut self, caption: impl Into<String>) -> Self {
        self.caption = Some(caption.into());
        self
    }

    /// Attaches provenance.
    #[must_use]
    pub fn with_provenance(mut self, provenance: PlotProvenance) -> Self {
        self.provenance = Some(provenance);
        self
    }
}

//! Bridge-safe projection types for future R-facing wrappers.

/// Stable summary of a governed EMBOSS-RS tool for bridge exposure.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BridgeToolSummary {
    /// Stable tool identifier exposed through the governed binary surface.
    pub name: String,
    /// Short summary used for help, docs, and discovery.
    pub summary: String,
}

/// Stable summary of a platform diagnostic for bridge exposure.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BridgeDiagnosticSummary {
    /// Severity as a lower-case string.
    pub severity: String,
    /// Stable machine-oriented code when present.
    pub code: Option<String>,
    /// Human-readable message.
    pub message: String,
    /// Optional context detail.
    pub context: Option<String>,
    /// Optional scoped location.
    pub location: Option<String>,
}

/// Generic operation status summary suitable for thin bridge responses.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BridgeOperationStatus {
    /// Whether the operation reached a healthy or successful state.
    pub ok: bool,
    /// Human-readable status message.
    pub message: String,
}

/// Bridge-safe projection of a shared EMBOSS-RS method result summary.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BridgeResultSummary {
    /// Tool or method identifier.
    pub tool: String,
    /// Stable payload family label.
    pub payload_kind: String,
    /// Summary title.
    pub title: String,
    /// Ordered summary lines.
    pub lines: Vec<String>,
    /// Number of attached artefacts.
    pub artifact_count: usize,
    /// Number of attached diagnostics.
    pub diagnostic_count: usize,
}

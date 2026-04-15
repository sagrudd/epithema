//! Bridge-safe projection types for future R-facing wrappers.

use serde::{Deserialize, Serialize};

/// Compact provenance summary safe to marshal across the bridge.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BridgeProvenanceSummary {
    /// Stable origin kind label.
    pub origin_kind: String,
    /// Main locator or identifier.
    pub locator: String,
    /// Optional provider identity.
    pub provider: Option<String>,
    /// Optional descriptive label.
    pub description: Option<String>,
}

/// Bridge-safe summary of an auxiliary artefact.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BridgeArtifactSummary {
    /// Stable artefact identifier.
    pub id: String,
    /// Stable artefact kind label.
    pub kind: String,
    /// Optional human-readable label.
    pub label: Option<String>,
    /// Optional local path.
    pub local_path: Option<String>,
    /// Optional artefact provenance summary.
    pub provenance: Option<BridgeProvenanceSummary>,
}

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

/// Bridge-safe summary of a biological feature.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BridgeFeatureSummary {
    /// Stable feature kind label.
    pub kind: String,
    /// Optional feature name.
    pub name: Option<String>,
    /// Zero-based inclusive start coordinate of the spanning bounds.
    pub start: usize,
    /// Zero-based exclusive end coordinate of the spanning bounds.
    pub end: usize,
    /// Shared strand label when available.
    pub strand: Option<String>,
    /// Number of location spans.
    pub span_count: usize,
    /// Number of qualifiers.
    pub qualifier_count: usize,
}

/// Bridge-safe summary of a sequence record.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BridgeSequenceSummary {
    /// Stable sequence identifier.
    pub identifier: String,
    /// Optional display label.
    pub display_name: Option<String>,
    /// Stable molecule kind label.
    pub molecule: String,
    /// Stable alphabet label.
    pub alphabet: String,
    /// Residue length.
    pub length: usize,
    /// Optional description.
    pub description: Option<String>,
    /// Number of attached features.
    pub feature_count: usize,
}

/// Bridge-safe summary of an alignment.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BridgeAlignmentSummary {
    /// Optional alignment identifier.
    pub identifier: Option<String>,
    /// Number of rows.
    pub row_count: usize,
    /// Number of aligned columns.
    pub column_count: usize,
    /// Whether the alignment is pairwise.
    pub pairwise: bool,
    /// Whether the alignment is multiple.
    pub multiple: bool,
    /// Ordered row identifiers.
    pub row_identifiers: Vec<String>,
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
    /// Whether a typed plot payload is attached.
    pub plot_available: bool,
}

/// Bridge-safe tabular summary suitable for later data-frame conversion.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BridgeTableSummary {
    /// Optional table title.
    pub title: Option<String>,
    /// Ordered column names.
    pub columns: Vec<String>,
    /// Ordered row cell values as strings.
    pub rows: Vec<Vec<String>>,
    /// Cached row count.
    pub row_count: usize,
}

/// Bridge-safe summary of a typed Rust plot contract.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BridgePlotSummary {
    /// Stable plot identifier.
    pub id: String,
    /// Human-readable title.
    pub title: String,
    /// Stable plot kind label.
    pub kind: String,
    /// Number of series.
    pub series_count: usize,
}

/// Bridge-safe JSON handoff payload for the R plotting backend.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BridgePlotContract {
    /// Stable plot summary.
    pub summary: BridgePlotSummary,
    /// Serialized JSON contract.
    pub json: String,
}

/// Bridge-safe owned sequence record for the first analytical R surface.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct BridgeSequenceRecord {
    /// Stable sequence identifier.
    pub identifier: String,
    /// Normalized uppercase residue content.
    pub sequence: String,
    /// Optional description.
    pub description: Option<String>,
    /// Stable molecule label.
    pub molecule: String,
    /// Stable alphabet label.
    pub alphabet: String,
    /// Residue length.
    pub length: usize,
}

/// Bridge-safe input record for analytical method requests.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct BridgeSequenceInput {
    /// Optional stable sequence identifier.
    pub identifier: Option<String>,
    /// Residue content to normalize and validate.
    pub sequence: String,
    /// Optional free-text description.
    pub description: Option<String>,
    /// Optional explicit molecule label.
    pub molecule: Option<String>,
}

/// One bridge-safe charge-profile row.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BridgeChargeWindow {
    /// Stable sequence identifier.
    pub identifier: String,
    /// One-based inclusive window start.
    pub window_start: usize,
    /// One-based inclusive window end.
    pub window_end: usize,
    /// Window length in residues.
    pub window_length: usize,
    /// Mean charge across the window.
    pub mean_charge: f64,
}

/// Bridge-safe charge-profile response.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BridgeChargeProfile {
    /// Stable sequence identifier.
    pub identifier: String,
    /// Sequence length in residues.
    pub sequence_length: usize,
    /// Window length.
    pub window: usize,
    /// Step size.
    pub step: usize,
    /// Ordered sliding-window rows.
    pub windows: Vec<BridgeChargeWindow>,
    /// Typed plot contract JSON for the R plotting backend.
    pub plot_contract_json: String,
}

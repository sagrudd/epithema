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

/// Stable summary of a governed Epithema tool for bridge exposure.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
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
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
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

/// Bridge-safe projection of a shared Epithema method result summary.
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

/// Bridge-safe 1-based inclusive interval input.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct BridgeIntervalInput {
    /// 1-based inclusive start coordinate.
    pub start: usize,
    /// 1-based inclusive end coordinate.
    pub end: usize,
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
    /// Number of attached features.
    pub feature_count: usize,
    /// Stable feature summaries attached to the record.
    pub features: Vec<BridgeFeatureSummary>,
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

/// One bridge-safe pattern hit row.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct BridgePatternHit {
    /// Stable source record identifier.
    pub identifier: String,
    /// Searched pattern text.
    pub pattern: String,
    /// Optional strand label.
    pub strand: Option<String>,
    /// Optional translated frame label.
    pub frame: Option<usize>,
    /// Zero-based inclusive residue start.
    pub start: usize,
    /// Zero-based half-open residue end.
    pub end: usize,
    /// Optional zero-based inclusive amino-acid start.
    pub amino_start: Option<usize>,
    /// Optional zero-based half-open amino-acid end.
    pub amino_end: Option<usize>,
    /// Optional zero-based inclusive nucleotide start.
    pub nucleotide_start: Option<usize>,
    /// Optional zero-based half-open nucleotide end.
    pub nucleotide_end: Option<usize>,
    /// Matched text.
    pub matched: String,
}

/// One bridge-safe `descseq` row.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct BridgeDescseqRow {
    /// Stable source-order ordinal.
    pub ordinal: usize,
    /// Stable record identifier.
    pub identifier: String,
    /// Optional display name.
    pub display_name: Option<String>,
    /// Optional free-text description.
    pub description: Option<String>,
    /// Sequence length in residues.
    pub length: usize,
    /// Stable molecule label.
    pub molecule: String,
    /// Stable alphabet label.
    pub alphabet: String,
    /// Attached feature count.
    pub feature_count: usize,
    /// Optional source label.
    pub source: Option<String>,
    /// Optional organism label.
    pub organism: Option<String>,
    /// Optional topology label.
    pub topology: Option<String>,
}

/// One bridge-safe translation-check case.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct BridgeTranslationCheck {
    /// Nucleotide input identifier.
    pub nucleotide_id: String,
    /// Protein input identifier.
    pub protein_id: String,
    /// Whether the translated and expected proteins match after terminal-stop normalization.
    pub matches: bool,
    /// Translated protein sequence.
    pub translated_protein: String,
    /// Expected protein sequence.
    pub expected_protein: String,
    /// Whether the translated protein ended with a terminal stop.
    pub translated_terminal_stop: bool,
    /// Whether the expected protein ended with a terminal stop.
    pub expected_terminal_stop: bool,
    /// Stable detail text.
    pub detail: String,
}

/// One bridge-safe composition row.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BridgeCompositionRow {
    /// Stable scope label such as `record` or `aggregate`.
    pub scope: String,
    /// Optional record identifier.
    pub identifier: Option<String>,
    /// Optional molecule label.
    pub molecule: Option<String>,
    /// Optional raw sequence length.
    pub sequence_length: Option<usize>,
    /// Number of counted non-gap symbols.
    pub counted_symbols: usize,
    /// Number of ignored gap symbols.
    pub ignored_gap_symbols: usize,
    /// Residue symbol.
    pub residue: String,
    /// Residue count.
    pub count: usize,
    /// Residue frequency.
    pub frequency: f64,
}

/// One bridge-safe GC summary row.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BridgeGcRow {
    /// Stable scope label such as `record` or `aggregate`.
    pub scope: String,
    /// Optional record identifier.
    pub identifier: Option<String>,
    /// Raw sequence length.
    pub sequence_length: usize,
    /// Total non-gap symbols.
    pub counted_symbols: usize,
    /// Canonical A/C/G/T/U symbols in the denominator.
    pub canonical_symbols: usize,
    /// Canonical G/C symbols in the numerator.
    pub gc_symbols: usize,
    /// Ambiguous non-gap symbols.
    pub ambiguous_symbols: usize,
    /// Ignored gap symbols.
    pub ignored_gap_symbols: usize,
    /// GC percentage over canonical symbols.
    pub gc_percent: f64,
}

/// One bridge-safe pepstats summary row.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BridgePepstatsSummaryRow {
    /// Record identifier.
    pub identifier: String,
    /// Raw sequence length.
    pub sequence_length: usize,
    /// Number of non-gap, non-stop residues contributing to mass.
    pub residue_length: usize,
    /// Number of stop symbols.
    pub stop_count: usize,
    /// Deterministic molecular-weight estimate.
    pub molecular_weight: f64,
}

/// Bridge-safe pepstats result carrying summary and composition rows.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BridgePepstatsResult {
    /// Per-record summary rows.
    pub summary_rows: Vec<BridgePepstatsSummaryRow>,
    /// Per-record composition rows.
    pub composition_rows: Vec<BridgeCompositionRow>,
}

/// Bridge-safe whole-sequence complexity summary.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BridgeComplexitySummary {
    /// Record identifier.
    pub identifier: String,
    /// Sequence length.
    pub sequence_length: usize,
    /// Inclusive minimum k.
    pub k_min: usize,
    /// Inclusive maximum k.
    pub k_max: usize,
    /// Whole-sequence complexity ratio.
    pub complexity: f64,
}

/// Bridge-safe sliding-window complexity row.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BridgeComplexityWindow {
    /// Record identifier.
    pub identifier: String,
    /// One-based inclusive window start.
    pub window_start: usize,
    /// One-based inclusive window end.
    pub window_end: usize,
    /// Window length.
    pub window_length: usize,
    /// Complexity ratio.
    pub complexity: f64,
}

/// Bridge-safe complexity result.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BridgeComplexityResult {
    /// Whole-sequence summary.
    pub summary: BridgeComplexitySummary,
    /// Optional sliding-window rows.
    pub windows: Vec<BridgeComplexityWindow>,
}

/// Bridge-safe matcher summary row.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct BridgeMatcherSummary {
    /// Comparison mode label.
    pub mode: String,
    /// Query length.
    pub query_length: usize,
    /// Target length.
    pub target_length: usize,
    /// Compared overlap length.
    pub compared_length: usize,
    /// Identity count.
    pub identity_count: usize,
    /// Mismatch count.
    pub mismatch_count: usize,
    /// Integer identity percentage over the compared overlap.
    pub identity_percent: usize,
    /// Signed target-minus-query length difference.
    pub length_difference: isize,
}

/// Bridge-safe p-distance matrix.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BridgeDistanceMatrix {
    /// Ordered record identifiers.
    pub identifiers: Vec<String>,
    /// Comparison mode label.
    pub mode: String,
    /// Shared sequence length.
    pub sequence_length: usize,
    /// Pairwise p-distance values.
    pub values: Vec<Vec<f64>>,
}

/// Bridge-safe aligned-row input.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct BridgeAlignmentRowInput {
    /// Stable row identifier.
    pub identifier: String,
    /// Aligned row content including `-` gaps.
    pub aligned: String,
    /// Optional row description.
    pub description: Option<String>,
}

/// Bridge-safe alignment input.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct BridgeAlignmentInput {
    /// Optional alignment identifier.
    pub identifier: Option<String>,
    /// Optional explicit molecule label.
    pub molecule: Option<String>,
    /// Ordered aligned rows.
    pub rows: Vec<BridgeAlignmentRowInput>,
}

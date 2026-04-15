//! Error projections for bridge-facing surfaces.

/// Bridge-safe error summary for future R-facing wrappers.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BridgeErrorSummary {
    /// Stable platform error category.
    pub category: String,
    /// Stable machine-oriented code when present.
    pub code: Option<String>,
    /// Human-readable primary message.
    pub message: String,
    /// Optional human-readable detail.
    pub detail: Option<String>,
}

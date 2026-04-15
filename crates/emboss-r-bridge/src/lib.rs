//! Rust-side bridge scaffolding for integration with the sister `emboss-r` project.
//!
//! This crate defines the Rust-facing contract seam that future R wrappers will
//! expose. It projects stable, bridge-safe summaries from `emboss-service`,
//! `emboss-diagnostics`, and related crates without duplicating service logic or
//! embedding R package mechanics in the workspace.

pub mod api;
pub mod conversion;
pub mod error;
pub mod health;
pub mod types;
pub mod version;

pub use api::{
    bridge_version, health_check, health_check_with_service, list_tools, summarize_alignment,
    summarize_features, summarize_sequence, summarize_table_result, supports_plot_payload,
};
pub use error::BridgeErrorSummary;
pub use health::BridgeHealth;
pub use types::{
    BridgeAlignmentSummary, BridgeArtifactSummary, BridgeDiagnosticSummary, BridgeFeatureSummary,
    BridgeOperationStatus, BridgeProvenanceSummary, BridgeResultSummary, BridgeSequenceSummary,
    BridgeTableSummary, BridgeToolSummary,
};
pub use version::BridgeVersion;

//! Rust-side bridge scaffolding for integration with the sister `emboss-r` project.
//!
//! This crate defines the Rust-facing contract seam that `emboss-r` consumes.
//! It projects stable, bridge-safe summaries and a small first analytical method
//! subset from `emboss-service`, `emboss-diagnostics`, and related crates
//! without duplicating service logic or embedding R package mechanics in the
//! workspace.

pub mod api;
pub mod conversion;
pub mod error;
pub mod health;
pub mod methods;
pub mod protocol;
pub mod types;
pub mod version;

pub use api::{
    bridge_version, health_check, health_check_with_service, list_tools, serialize_plot_contract,
    summarize_alignment, summarize_features, summarize_plot_contract, summarize_sequence,
    summarize_table_result, supports_plot_payload,
};
pub use error::BridgeErrorSummary;
pub use health::BridgeHealth;
pub use methods::{
    charge_profile, new_sequence, not_sequence, nth_sequence, sequence_count, skip_sequences,
};
pub use protocol::{BridgeRequest, BridgeResponse};
pub use types::{
    BridgeAlignmentSummary, BridgeArtifactSummary, BridgeChargeProfile, BridgeChargeWindow,
    BridgeDiagnosticSummary, BridgeFeatureSummary, BridgeOperationStatus, BridgePlotContract,
    BridgePlotSummary, BridgeProvenanceSummary, BridgeResultSummary, BridgeSequenceInput,
    BridgeSequenceRecord, BridgeSequenceSummary, BridgeTableSummary, BridgeToolSummary,
};
pub use version::BridgeVersion;

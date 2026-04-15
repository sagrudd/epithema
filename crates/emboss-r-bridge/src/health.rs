//! Health and handshake summaries for the Rust-side bridge.

use crate::types::BridgeOperationStatus;

/// Bridge-facing health summary for the current runtime.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BridgeHealth {
    /// Sister package expected by the Rust workspace.
    pub sister_package: String,
    /// Plot backend expected by the Rust workspace.
    pub plot_backend: String,
    /// Number of currently governed tool descriptors.
    pub tools_registered: usize,
    /// Number of configured providers.
    pub providers_configured: usize,
    /// Human-readable shared service status.
    pub service_status: String,
    /// High-level bridge status summary.
    pub operation_status: BridgeOperationStatus,
}

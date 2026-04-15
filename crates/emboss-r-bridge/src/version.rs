//! Version and platform identity projections for the bridge surface.

/// Stable bridge-facing version and platform summary.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BridgeVersion {
    /// Rust workspace package version.
    pub package_version: String,
    /// Canonical CLI binary name.
    pub binary_name: String,
    /// First-class sister package name.
    pub sister_package: String,
    /// Plot backend owned by the R surface.
    pub plot_backend: String,
}

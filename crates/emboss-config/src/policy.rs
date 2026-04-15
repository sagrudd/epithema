//! Runtime policy controls for acquisition and autodoc behavior.

/// Acquisition behavior controls shared by tools, services, and autodoc.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AcquisitionPolicy {
    /// Allow filesystem-backed inputs.
    pub allow_local_files: bool,
    /// Allow provider-backed remote resolution.
    pub allow_remote_acquisition: bool,
    /// Prefer local cache roots before consulting remote providers.
    pub prefer_local_cache: bool,
}

impl Default for AcquisitionPolicy {
    fn default() -> Self {
        Self {
            allow_local_files: true,
            allow_remote_acquisition: true,
            prefer_local_cache: true,
        }
    }
}

/// Autodoc-specific acquisition policy controls.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AutodocPolicy {
    /// Allow local checked-in artefacts to satisfy documentation inputs.
    pub allow_local_declared_artifacts: bool,
    /// Allow governed fixture assets to satisfy documentation inputs.
    pub allow_fixture_assets: bool,
    /// Allow harvested legacy EMBOSS artefacts to satisfy documentation inputs.
    pub allow_legacy_harvest_artifacts: bool,
    /// Allow generated intermediate artefacts declared by the documentation flow.
    pub allow_generated_artifacts: bool,
    /// Require autodoc acquisition to use governed provider paths.
    pub acquire_through_providers: bool,
    /// Require provenance capture for autodoc inputs.
    pub record_provenance: bool,
}

impl Default for AutodocPolicy {
    fn default() -> Self {
        Self {
            allow_local_declared_artifacts: true,
            allow_fixture_assets: true,
            allow_legacy_harvest_artifacts: true,
            allow_generated_artifacts: true,
            acquire_through_providers: true,
            record_provenance: true,
        }
    }
}

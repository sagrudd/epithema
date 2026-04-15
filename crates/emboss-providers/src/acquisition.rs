//! Governed documentation-acquisition seam.
//!
//! Documentation workflows must use this interface for any external artefact
//! resolution. Local, fixture, legacy, and generated artefacts may be accepted
//! without remote acquisition, but provider-backed documentation inputs must
//! pass through this explicit gateway rather than using ad hoc download logic
//! inside docgen or Sphinx-generation code.

use std::path::PathBuf;

use emboss_diagnostics::ArtifactProvenance;
use emboss_diagnostics::PlatformError;

use crate::{AcquisitionRequest, ProviderId};

/// How a documentation artefact was resolved or accepted.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DocumentationAcquisitionRoute {
    /// The artefact was already available as a local path.
    LocalDeclared,
    /// The artefact was resolved from governed fixture inventory.
    FixtureAsset,
    /// The artefact was derived from harvested legacy EMBOSS material.
    LegacyHarvest,
    /// The artefact was generated within the documentation workflow.
    GeneratedArtifact,
    /// The artefact was acquired through a governed provider-backed path.
    GovernedProvider {
        /// Provider identity used by the governed acquisition route.
        provider: Option<ProviderId>,
    },
}

/// Typed request passed into the governed documentation-acquisition seam.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DocumentationAcquisitionRequest {
    /// Stable artefact identifier from the autodoc contract.
    pub artifact_id: String,
    /// Provider-mediated resolution request derived from the autodoc declaration.
    pub request: AcquisitionRequest,
}

impl DocumentationAcquisitionRequest {
    /// Creates a new documentation acquisition request.
    #[must_use]
    pub fn new(artifact_id: impl Into<String>, request: AcquisitionRequest) -> Self {
        Self {
            artifact_id: artifact_id.into(),
            request,
        }
    }
}

/// Provenance-rich resolution outcome for a documentation artefact.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DocumentationAcquisitionRecord {
    /// Stable artefact identifier from the autodoc contract.
    pub artifact_id: String,
    /// Route taken to resolve or accept the artefact.
    pub route: DocumentationAcquisitionRoute,
    /// Structured provenance for the resolved artefact.
    pub provenance: ArtifactProvenance,
    /// Optional local materialized path when the artefact is file-backed.
    pub local_path: Option<PathBuf>,
}

impl DocumentationAcquisitionRecord {
    /// Creates a documentation acquisition record.
    #[must_use]
    pub fn new(
        artifact_id: impl Into<String>,
        route: DocumentationAcquisitionRoute,
        provenance: ArtifactProvenance,
    ) -> Self {
        Self {
            artifact_id: artifact_id.into(),
            route,
            provenance,
            local_path: None,
        }
    }

    /// Attaches a local materialized path to the resolved artefact.
    #[must_use]
    pub fn with_local_path(mut self, path: impl Into<PathBuf>) -> Self {
        self.local_path = Some(path.into());
        self
    }
}

/// Formal gateway for provider-mediated documentation acquisition.
pub trait DocumentationAcquisitionGateway {
    /// Resolves a documentation artefact through a governed EMBOSS-RS path.
    fn acquire_documentation_artifact(
        &self,
        request: &DocumentationAcquisitionRequest,
    ) -> Result<DocumentationAcquisitionRecord, PlatformError>;
}

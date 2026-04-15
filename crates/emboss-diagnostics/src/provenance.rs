//! Provenance models for inputs, outputs, and related artefacts.

/// Stable classification of an input or generated artefact origin.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ArtifactOriginKind {
    /// Local filesystem input or output.
    LocalFile,
    /// Accession or remote biological identifier.
    Accession,
    /// Provider-derived asset retrieved from an external source.
    ProviderAsset,
    /// Fixture or generated test asset.
    GeneratedFixture,
    /// Generated output from platform execution.
    GeneratedOutput,
    /// Origin is not yet known.
    Unknown,
}

/// Provenance record for an artefact involved in a run.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ArtifactProvenance {
    /// Classified origin of the artefact.
    pub origin_kind: ArtifactOriginKind,
    locator: String,
    provider: Option<String>,
    description: Option<String>,
}

impl ArtifactProvenance {
    /// Creates a provenance record from an origin kind and locator.
    #[must_use]
    pub fn new(origin_kind: ArtifactOriginKind, locator: impl Into<String>) -> Self {
        Self {
            origin_kind,
            locator: locator.into(),
            provider: None,
            description: None,
        }
    }

    /// Creates a provenance record for a local file path.
    #[must_use]
    pub fn local_file(path: impl Into<String>) -> Self {
        Self::new(ArtifactOriginKind::LocalFile, path)
    }

    /// Creates a provenance record for an accession.
    #[must_use]
    pub fn accession(accession: impl Into<String>) -> Self {
        Self::new(ArtifactOriginKind::Accession, accession)
    }

    /// Creates a provenance record for a provider-backed asset.
    #[must_use]
    pub fn provider_asset(locator: impl Into<String>) -> Self {
        Self::new(ArtifactOriginKind::ProviderAsset, locator)
    }

    /// Creates a provenance record for a generated fixture.
    #[must_use]
    pub fn generated_fixture(locator: impl Into<String>) -> Self {
        Self::new(ArtifactOriginKind::GeneratedFixture, locator)
    }

    /// Creates a provenance record for generated output.
    #[must_use]
    pub fn generated_output(locator: impl Into<String>) -> Self {
        Self::new(ArtifactOriginKind::GeneratedOutput, locator)
    }

    /// Adds provider identity when the artefact was externally sourced.
    #[must_use]
    pub fn with_provider(mut self, provider: impl Into<String>) -> Self {
        self.provider = Some(provider.into());
        self
    }

    /// Adds a descriptive label for the artefact.
    #[must_use]
    pub fn with_description(mut self, description: impl Into<String>) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Returns the main locator for the artefact.
    #[must_use]
    pub fn locator(&self) -> &str {
        &self.locator
    }

    /// Returns the provider when present.
    #[must_use]
    pub fn provider(&self) -> Option<&str> {
        self.provider.as_deref()
    }

    /// Returns the descriptive label when present.
    #[must_use]
    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }
}

#[cfg(test)]
mod tests {
    use super::{ArtifactOriginKind, ArtifactProvenance};

    #[test]
    fn provider_asset_can_capture_provider_and_description() {
        let provenance = ArtifactProvenance::provider_asset("uniprot:P12345")
            .with_provider("UniProt")
            .with_description("retrieved sequence");

        assert_eq!(provenance.origin_kind, ArtifactOriginKind::ProviderAsset);
        assert_eq!(provenance.provider(), Some("UniProt"));
        assert_eq!(provenance.description(), Some("retrieved sequence"));
    }
}

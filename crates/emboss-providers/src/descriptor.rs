//! Static provider descriptors.

use crate::{ProviderCapability, ProviderId};

/// Descriptor for a known provider backend.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProviderDescriptor {
    /// Stable provider identity.
    pub id: ProviderId,
    /// Human-readable summary for diagnostics and docs.
    pub summary: String,
    capabilities: Vec<ProviderCapability>,
}

impl ProviderDescriptor {
    /// Creates a provider descriptor from an identity, summary, and capabilities.
    #[must_use]
    pub fn new(
        id: ProviderId,
        summary: impl Into<String>,
        capabilities: impl IntoIterator<Item = ProviderCapability>,
    ) -> Self {
        Self {
            id,
            summary: summary.into(),
            capabilities: capabilities.into_iter().collect(),
        }
    }

    /// Returns the declared capabilities for the provider.
    #[must_use]
    pub fn capabilities(&self) -> &[ProviderCapability] {
        &self.capabilities
    }

    /// Returns true when the provider advertises the supplied capability.
    #[must_use]
    pub fn supports(&self, capability: ProviderCapability) -> bool {
        self.capabilities.contains(&capability)
    }
}

#[cfg(test)]
mod tests {
    use super::ProviderDescriptor;
    use crate::{ProviderCapability, ProviderId};

    #[test]
    fn reports_declared_capabilities() {
        let descriptor = ProviderDescriptor::new(
            ProviderId::new("ena").expect("valid provider id"),
            "European Nucleotide Archive",
            [
                ProviderCapability::MetadataLookup,
                ProviderCapability::SequenceRetrieval,
            ],
        );

        assert!(descriptor.supports(ProviderCapability::MetadataLookup));
        assert!(!descriptor.supports(ProviderCapability::ArchiveAcquisition));
    }
}

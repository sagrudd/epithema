//! Registry of known provider descriptors.

use emboss_diagnostics::{ErrorCategory, PlatformError};

use crate::{ProviderCapability, ProviderDescriptor, ProviderId};

/// Mutable registry of known provider descriptors.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ProviderRegistry {
    descriptors: Vec<ProviderDescriptor>,
}

impl ProviderRegistry {
    /// Creates an empty provider registry.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Creates a registry populated with built-in sequence providers.
    #[must_use]
    pub fn builtin_defaults() -> Self {
        let mut registry = Self::new();
        for descriptor in builtin_provider_descriptors() {
            registry
                .register(descriptor)
                .expect("built-in provider descriptors must be unique");
        }
        registry
    }

    /// Registers a provider descriptor, rejecting duplicate identities.
    pub fn register(&mut self, descriptor: ProviderDescriptor) -> Result<(), PlatformError> {
        if self.find(&descriptor.id).is_some() {
            return Err(PlatformError::new(
                ErrorCategory::Registry,
                format!("provider '{}' is already registered", descriptor.id),
            )
            .with_code("providers.registry.duplicate_provider"));
        }

        self.descriptors.push(descriptor);
        Ok(())
    }

    /// Returns all registered providers.
    #[must_use]
    pub fn descriptors(&self) -> &[ProviderDescriptor] {
        &self.descriptors
    }

    /// Returns the descriptor for the supplied provider identity.
    #[must_use]
    pub fn find(&self, id: &ProviderId) -> Option<&ProviderDescriptor> {
        self.descriptors
            .iter()
            .find(|descriptor| &descriptor.id == id)
    }

    /// Returns provider descriptors advertising a given capability.
    #[must_use]
    pub fn supporting(&self, capability: ProviderCapability) -> Vec<&ProviderDescriptor> {
        self.descriptors
            .iter()
            .filter(|descriptor| descriptor.supports(capability))
            .collect()
    }

    /// Returns the number of registered providers.
    #[must_use]
    pub fn len(&self) -> usize {
        self.descriptors.len()
    }

    /// Returns true when no provider descriptors are registered.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.descriptors.is_empty()
    }
}

fn builtin_provider_descriptors() -> Vec<ProviderDescriptor> {
    vec![
        ProviderDescriptor::new(
            ProviderId::new("ena").expect("static provider id should be valid"),
            "European Nucleotide Archive single-sequence retrieval",
            [
                ProviderCapability::MetadataLookup,
                ProviderCapability::SequenceRetrieval,
            ],
        ),
        ProviderDescriptor::new(
            ProviderId::new("ncbi").expect("static provider id should be valid"),
            "NCBI E-utilities single-sequence retrieval",
            [
                ProviderCapability::MetadataLookup,
                ProviderCapability::SequenceRetrieval,
            ],
        ),
    ]
}

#[cfg(test)]
mod tests {
    use super::ProviderRegistry;
    use crate::{ProviderCapability, ProviderDescriptor, ProviderId};

    #[test]
    fn rejects_duplicate_provider_registration() {
        let mut registry = ProviderRegistry::new();
        let descriptor = ProviderDescriptor::new(
            ProviderId::new("ena").expect("valid provider id"),
            "European Nucleotide Archive",
            [ProviderCapability::SequenceRetrieval],
        );

        registry
            .register(descriptor.clone())
            .expect("first registration should succeed");
        assert!(registry.register(descriptor).is_err());
    }

    #[test]
    fn finds_providers_by_capability() {
        let mut registry = ProviderRegistry::new();
        registry
            .register(ProviderDescriptor::new(
                ProviderId::new("sra").expect("valid provider id"),
                "Sequence Read Archive",
                [ProviderCapability::ArchiveAcquisition],
            ))
            .expect("registration should succeed");

        assert_eq!(
            registry
                .supporting(ProviderCapability::ArchiveAcquisition)
                .len(),
            1
        );
    }

    #[test]
    fn builtin_defaults_include_ena_and_ncbi() {
        let registry = ProviderRegistry::builtin_defaults();

        assert!(
            registry
                .find(&ProviderId::new("ena").expect("valid id"))
                .is_some()
        );
        assert!(
            registry
                .find(&ProviderId::new("ncbi").expect("valid id"))
                .is_some()
        );
    }
}

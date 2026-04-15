//! Service-owned documentation acquisition façade.
//!
//! This module provides the governed acquisition boundary used by docgen and
//! future reporting paths. It intentionally does not perform hidden downloads:
//! provider-backed requests are routed through a formal service gateway and
//! return a clear not-yet-implemented error until real provider clients are
//! integrated.

use emboss_config::PlatformConfig;
use emboss_diagnostics::{ErrorCategory, PlatformError};
use emboss_providers::{
    DocumentationAcquisitionGateway, DocumentationAcquisitionRecord,
    DocumentationAcquisitionRequest, ProviderCapability, ProviderRegistry,
};

/// Service-backed documentation acquisition gateway.
#[derive(Clone, Copy, Debug)]
pub struct ServiceDocumentationAcquisition<'a> {
    config: &'a PlatformConfig,
    providers: &'a ProviderRegistry,
}

impl<'a> ServiceDocumentationAcquisition<'a> {
    /// Creates a service-backed documentation acquisition gateway.
    #[must_use]
    pub fn new(config: &'a PlatformConfig, providers: &'a ProviderRegistry) -> Self {
        Self { config, providers }
    }
}

impl DocumentationAcquisitionGateway for ServiceDocumentationAcquisition<'_> {
    fn acquire_documentation_artifact(
        &self,
        request: &DocumentationAcquisitionRequest,
    ) -> Result<DocumentationAcquisitionRecord, PlatformError> {
        if !self.config.autodoc.acquire_through_providers {
            return Err(PlatformError::new(
                ErrorCategory::Configuration,
                "formal documentation acquisition is disabled by platform policy",
            )
            .with_code("service.documentation_acquisition.disabled")
            .with_detail(request.artifact_id.clone()));
        }

        let preferred = request.request.preferred_provider.clone();

        if let Some(provider) = &preferred {
            let Some(descriptor) = self.providers.find(provider) else {
                return Err(PlatformError::new(
                    ErrorCategory::Registry,
                    "requested documentation provider is not registered",
                )
                .with_code("service.documentation_acquisition.unknown_provider")
                .with_detail(provider.as_str()));
            };

            if !descriptor.supports(ProviderCapability::DocumentationAssetAcquisition) {
                return Err(PlatformError::new(
                    ErrorCategory::Registry,
                    "requested provider does not advertise documentation retrieval capability",
                )
                .with_code("service.documentation_acquisition.unsupported_capability")
                .with_detail(provider.as_str()));
            }
        }

        Err(PlatformError::new(
            ErrorCategory::NotImplemented,
            "formal provider-backed documentation acquisition is not implemented yet",
        )
        .with_code("service.documentation_acquisition.not_implemented")
        .with_detail(request.artifact_id.clone()))
    }
}

#[cfg(test)]
mod tests {
    use emboss_config::PlatformConfig;
    use emboss_providers::{
        AcquisitionRequest, DocumentationAcquisitionRequest, InputReference, ProviderCapability,
        ProviderDescriptor, ProviderId, ProviderRegistry, ResolutionIntent,
    };

    use super::ServiceDocumentationAcquisition;
    use crate::DocumentationAcquisitionGateway;

    #[test]
    fn rejects_provider_requests_until_formal_implementation_exists() {
        let mut registry = ProviderRegistry::new();
        registry
            .register(ProviderDescriptor::new(
                ProviderId::new("ena").expect("valid id"),
                "European Nucleotide Archive",
                [ProviderCapability::DocumentationAssetAcquisition],
            ))
            .expect("registration should succeed");

        let config = PlatformConfig::default();
        let gateway = ServiceDocumentationAcquisition::new(&config, &registry);
        let request = DocumentationAcquisitionRequest::new(
            "ena-doc",
            AcquisitionRequest::new(
                ResolutionIntent::DocumentationAsset,
                InputReference::accession("AB000263"),
            )
            .with_preferred_provider(ProviderId::new("ena").expect("valid id")),
        );

        let error = gateway
            .acquire_documentation_artifact(&request)
            .expect_err("provider acquisition should be explicitly unimplemented");
        assert_eq!(
            error.code(),
            Some("service.documentation_acquisition.not_implemented")
        );
    }
}

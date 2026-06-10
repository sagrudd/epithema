//! Service-owned single-sequence retrieval façade.
//!
//! This module is the formal acquisition seam for provider-backed single
//! sequence retrieval. It keeps provider routing, configuration checks, and
//! provenance-aware results out of the CLI and out of individual tool modules.

use epithema_config::PlatformConfig;
use epithema_diagnostics::{ErrorCategory, PlatformError};
use epithema_providers::{
    AcquisitionRequest, ProviderCapability, ProviderHttpClient, ProviderRegistry,
    ProviderSequenceRouter, ReqwestHttpClient, RetrievedSequence, SequenceRequest,
};

/// Service-backed single-sequence retrieval gateway.
#[derive(Clone, Debug)]
pub struct ServiceSequenceRetrieval<'a, C> {
    config: &'a PlatformConfig,
    providers: &'a ProviderRegistry,
    client: C,
}

impl<'a> ServiceSequenceRetrieval<'a, ReqwestHttpClient> {
    /// Creates a service-backed retrieval gateway with the default HTTP client.
    pub fn new(
        config: &'a PlatformConfig,
        providers: &'a ProviderRegistry,
    ) -> Result<Self, PlatformError> {
        Ok(Self {
            config,
            providers,
            client: ReqwestHttpClient::new()?,
        })
    }
}

impl<'a, C> ServiceSequenceRetrieval<'a, C> {
    /// Creates a service-backed retrieval gateway with an injected HTTP client.
    #[must_use]
    pub fn with_client(
        config: &'a PlatformConfig,
        providers: &'a ProviderRegistry,
        client: C,
    ) -> Self {
        Self {
            config,
            providers,
            client,
        }
    }
}

impl<C: ProviderHttpClient> ServiceSequenceRetrieval<'_, C> {
    /// Retrieves a single provider-backed sequence record.
    pub fn retrieve_sequence(
        &self,
        request: &SequenceRequest,
    ) -> Result<RetrievedSequence, PlatformError> {
        if !self.config.acquisition.allow_remote_acquisition {
            return Err(PlatformError::new(
                ErrorCategory::Configuration,
                "remote provider acquisition is disabled by platform policy",
            )
            .with_code("service.sequence_retrieval.remote_disabled"));
        }

        let resolution = epithema_providers::SequenceProviderResolution::from_request(request)?;
        let Some(descriptor) = self.providers.find(&resolution.provider) else {
            return Err(PlatformError::new(
                ErrorCategory::Registry,
                "requested provider is not registered in the active service registry",
            )
            .with_code("service.sequence_retrieval.unknown_provider")
            .with_detail(resolution.provider.as_str().to_owned()));
        };

        if !descriptor.supports(ProviderCapability::SequenceRetrieval) {
            return Err(PlatformError::new(
                ErrorCategory::Registry,
                "requested provider does not advertise single-sequence retrieval capability",
            )
            .with_code("service.sequence_retrieval.unsupported_provider")
            .with_detail(resolution.provider.as_str().to_owned()));
        }

        if !provider_enabled(self.config, resolution.provider.as_str()) {
            return Err(PlatformError::new(
                ErrorCategory::Configuration,
                "requested provider is disabled in platform configuration",
            )
            .with_code("service.sequence_retrieval.provider_disabled")
            .with_detail(resolution.provider.as_str().to_owned()));
        }

        ProviderSequenceRouter::new().retrieve_with(request, &self.client)
    }

    /// Convenience alias for a single accession-style provider request.
    pub fn retrieve_single_sequence(
        &self,
        request: &AcquisitionRequest,
    ) -> Result<RetrievedSequence, PlatformError> {
        self.retrieve_sequence(request)
    }
}

fn provider_enabled(config: &PlatformConfig, provider: &str) -> bool {
    let settings = config.provider_settings();
    if settings.is_empty() {
        return true;
    }

    settings
        .iter()
        .find(|setting| setting.id.as_str() == provider)
        .map(|setting| setting.enabled)
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use epithema_config::{PlatformConfig, ProviderSettings};
    use epithema_diagnostics::PlatformError;
    use epithema_providers::{
        AcquisitionRequest, HttpRequest, HttpResponse, InputReference, ProviderHttpClient,
        ProviderId, ProviderRegistry, ResolutionIntent,
    };

    use super::ServiceSequenceRetrieval;

    #[derive(Clone, Debug, Default)]
    struct MockHttpClient {
        responses: HashMap<String, HttpResponse>,
    }

    impl MockHttpClient {
        fn with_response(mut self, url: impl Into<String>, response: HttpResponse) -> Self {
            self.responses.insert(url.into(), response);
            self
        }
    }

    impl ProviderHttpClient for MockHttpClient {
        fn get_text(&self, request: &HttpRequest) -> Result<HttpResponse, PlatformError> {
            self.responses.get(&request.url).cloned().ok_or_else(|| {
                PlatformError::new(
                    epithema_diagnostics::ErrorCategory::Invocation,
                    "mock response was not configured for provider request",
                )
                .with_code("service.sequence_retrieval.test.missing_response")
                .with_detail(request.url.clone())
            })
        }
    }

    #[test]
    fn retrieves_ena_sequence_through_service_gateway() {
        let config = PlatformConfig::default().with_provider(ProviderSettings::enabled(
            ProviderId::new("ena").expect("valid provider"),
        ));
        let registry = ProviderRegistry::builtin_defaults();
        let client = MockHttpClient::default().with_response(
            "https://www.ebi.ac.uk/ena/browser/api/fasta/AB000263",
            HttpResponse::new(200, ">AB000263 example\nACGT\n"),
        );
        let gateway = ServiceSequenceRetrieval::with_client(&config, &registry, client);
        let request = AcquisitionRequest::new(
            ResolutionIntent::SequenceInput,
            InputReference::provider_asset(
                Some(ProviderId::new("ena").expect("valid provider")),
                "AB000263",
            ),
        )
        .with_preferred_provider(ProviderId::new("ena").expect("valid provider"));

        let retrieved = gateway
            .retrieve_sequence(&request)
            .expect("ENA retrieval should succeed");

        assert_eq!(retrieved.provider.as_str(), "ena");
        assert_eq!(retrieved.record.identifier().accession(), "AB000263");
    }

    #[test]
    fn retrieves_ncbi_sequence_through_service_gateway() {
        let config = PlatformConfig::default().with_provider(ProviderSettings::enabled(
            ProviderId::new("ncbi").expect("valid provider"),
        ));
        let registry = ProviderRegistry::builtin_defaults();
        let client = MockHttpClient::default().with_response(
            "https://eutils.ncbi.nlm.nih.gov/entrez/eutils/efetch.fcgi?db=protein&id=NP_000537.3&rettype=fasta&retmode=text",
            HttpResponse::new(200, ">NP_000537.3 TP53\nMEEPQSDPSV\n"),
        );
        let gateway = ServiceSequenceRetrieval::with_client(&config, &registry, client);
        let request = AcquisitionRequest::new(
            ResolutionIntent::SequenceInput,
            InputReference::provider_asset(
                Some(ProviderId::new("ncbi").expect("valid provider")),
                "protein:NP_000537.3",
            ),
        )
        .with_preferred_provider(ProviderId::new("ncbi").expect("valid provider"));

        let retrieved = gateway
            .retrieve_sequence(&request)
            .expect("NCBI retrieval should succeed");

        assert_eq!(retrieved.provider.as_str(), "ncbi");
        assert_eq!(
            retrieved.record.molecule(),
            epithema_core::MoleculeKind::Protein
        );
    }
}

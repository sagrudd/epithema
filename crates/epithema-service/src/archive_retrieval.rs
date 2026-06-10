//! Service-owned archive metadata and public-run manifest façade.
//!
//! This module is the formal acquisition seam for provider-backed archive
//! metadata lookup and public-run manifest discovery. It keeps provider
//! routing, capability checks, and provenance-aware results out of the CLI and
//! out of individual tool implementations.

use epithema_config::PlatformConfig;
use epithema_diagnostics::{ErrorCategory, PlatformError};
use epithema_providers::{
    AcquisitionRequest, ProviderArchiveRouter, ProviderCapability, ProviderHttpClient,
    ProviderRegistry, ReqwestHttpClient, RetrievedArchiveManifest, RetrievedArchiveMetadata,
};

/// Service-backed archive retrieval gateway.
#[derive(Clone, Debug)]
pub struct ServiceArchiveRetrieval<'a, C> {
    config: &'a PlatformConfig,
    providers: &'a ProviderRegistry,
    client: C,
}

impl<'a> ServiceArchiveRetrieval<'a, ReqwestHttpClient> {
    /// Creates a service-backed archive gateway with the default HTTP client.
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

impl<'a, C> ServiceArchiveRetrieval<'a, C> {
    /// Creates a service-backed archive gateway with an injected HTTP client.
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

impl<C: ProviderHttpClient> ServiceArchiveRetrieval<'_, C> {
    /// Looks up normalized archive metadata.
    pub fn lookup_metadata(
        &self,
        request: &AcquisitionRequest,
    ) -> Result<RetrievedArchiveMetadata, PlatformError> {
        self.ensure_archive_provider_enabled(request)?;
        ProviderArchiveRouter::new().lookup_metadata_with(request, &self.client)
    }

    /// Looks up a normalized public-run manifest.
    pub fn retrieve_run_manifest(
        &self,
        request: &AcquisitionRequest,
    ) -> Result<RetrievedArchiveManifest, PlatformError> {
        self.ensure_archive_provider_enabled(request)?;
        ProviderArchiveRouter::new().manifest_with(request, &self.client)
    }

    fn ensure_archive_provider_enabled(
        &self,
        request: &AcquisitionRequest,
    ) -> Result<(), PlatformError> {
        if !self.config.acquisition.allow_remote_acquisition {
            return Err(PlatformError::new(
                ErrorCategory::Configuration,
                "remote provider acquisition is disabled by platform policy",
            )
            .with_code("service.archive_retrieval.remote_disabled"));
        }

        let resolution = epithema_providers::ArchiveProviderResolution::from_request(request)?;
        let Some(descriptor) = self.providers.find(&resolution.provider) else {
            return Err(PlatformError::new(
                ErrorCategory::Registry,
                "requested archive provider is not registered in the active service registry",
            )
            .with_code("service.archive_retrieval.unknown_provider")
            .with_detail(resolution.provider.as_str().to_owned()));
        };

        if !descriptor.supports(ProviderCapability::ArchiveAcquisition) {
            return Err(PlatformError::new(
                ErrorCategory::Registry,
                "requested provider does not advertise archive acquisition capability",
            )
            .with_code("service.archive_retrieval.unsupported_provider")
            .with_detail(resolution.provider.as_str().to_owned()));
        }

        let settings = self.config.provider_settings();
        if settings.is_empty() {
            return Ok(());
        }

        let enabled = settings
            .iter()
            .find(|setting| setting.id.as_str() == resolution.provider.as_str())
            .map(|setting| setting.enabled)
            .unwrap_or(false);
        if !enabled {
            return Err(PlatformError::new(
                ErrorCategory::Configuration,
                "requested archive provider is disabled in platform configuration",
            )
            .with_code("service.archive_retrieval.provider_disabled")
            .with_detail(resolution.provider.as_str().to_owned()));
        }

        Ok(())
    }
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

    use super::ServiceArchiveRetrieval;

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
                .with_code("service.archive_retrieval.test.missing_response")
                .with_detail(request.url.clone())
            })
        }
    }

    #[test]
    fn retrieves_ena_run_metadata_through_service_gateway() {
        let config = PlatformConfig::default().with_provider(ProviderSettings::enabled(
            ProviderId::new("ena").expect("valid provider"),
        ));
        let registry = ProviderRegistry::builtin_defaults();
        let client = MockHttpClient::default().with_response(
            "https://www.ebi.ac.uk/ena/portal/api/filereport?accession=ERR123456&result=read_run&fields=run_accession%2Cstudy_accession%2Cexperiment_accession%2Csample_accession%2Cinstrument_platform%2Cinstrument_model%2Clibrary_layout%2Clibrary_strategy%2Clibrary_source%2Cfastq_ftp%2Cfastq_md5%2Cfastq_bytes%2Csubmitted_ftp%2Csubmitted_md5%2Csubmitted_bytes%2Csra_ftp%2Csra_md5%2Csra_bytes&format=tsv&download=false",
            HttpResponse::new(200, "run_accession\tstudy_accession\texperiment_accession\tsample_accession\tinstrument_platform\tinstrument_model\tlibrary_layout\tlibrary_strategy\tlibrary_source\tfastq_ftp\tfastq_md5\tfastq_bytes\tsubmitted_ftp\tsubmitted_md5\tsubmitted_bytes\tsra_ftp\tsra_md5\tsra_bytes\nERR123456\tERP000001\tERX000001\tERS000001\tILLUMINA\tNovaSeq 6000\tPAIRED\tWGS\tGENOMIC\tftp.sra.ebi.ac.uk/vol1/fastq/ERR123/ERR123456/ERR123456_1.fastq.gz;ftp.sra.ebi.ac.uk/vol1/fastq/ERR123/ERR123456/ERR123456_2.fastq.gz\tmd51;md52\t10;12\t\t\t\t\t\t\n"),
        );
        let gateway = ServiceArchiveRetrieval::with_client(&config, &registry, client);
        let request = AcquisitionRequest::new(
            ResolutionIntent::ArchiveAsset,
            InputReference::provider_asset(
                Some(ProviderId::new("ena").expect("valid provider")),
                "ERR123456",
            ),
        )
        .with_preferred_provider(ProviderId::new("ena").expect("valid provider"));

        let metadata = gateway
            .lookup_metadata(&request)
            .expect("ENA metadata lookup should succeed");

        assert_eq!(metadata.provider.as_str(), "ena");
        assert_eq!(metadata.files.len(), 2);
    }

    #[test]
    fn retrieves_sra_run_metadata_through_service_gateway() {
        let config = PlatformConfig::default().with_provider(ProviderSettings::enabled(
            ProviderId::new("sra").expect("valid provider"),
        ));
        let registry = ProviderRegistry::builtin_defaults();
        let client = MockHttpClient::default().with_response(
            "https://trace.ncbi.nlm.nih.gov/Traces/sra-db-be/runinfo?acc=SRR123456",
            HttpResponse::new(200, "Run,ReleaseDate,LoadDate,spots,bases,spots_with_mates,avgLength,size_MB,AssemblyName,download_path,Experiment,LibraryName,LibraryStrategy,LibrarySelection,LibrarySource,LibraryLayout,InsertSize,InsertDev,Platform,Model,SRAStudy,BioProject,Study_Pubmed_id,ProjectID,Sample,BioSample,SampleType,TaxID,ScientificName,SampleName,CenterName,Submission,dbgap_study_accession,Consent,RunHash,ReadHash\nSRR123456,2024-01-01,2024-01-02,1,100,1,100,1,,https://example.invalid/SRR123456,SRX123456,,WGS,,GENOMIC,PAIRED,,,,ILLUMINA,NextSeq 2000,SRP000001,PRJNA1,,1,SRS123456,SAMN1,,9606,Homo sapiens,,NCBI,SRA000001,,,runhash,readhash\n"),
        );
        let gateway = ServiceArchiveRetrieval::with_client(&config, &registry, client);
        let request = AcquisitionRequest::new(
            ResolutionIntent::ArchiveAsset,
            InputReference::provider_asset(
                Some(ProviderId::new("sra").expect("valid provider")),
                "SRR123456",
            ),
        )
        .with_preferred_provider(ProviderId::new("sra").expect("valid provider"));

        let metadata = gateway
            .lookup_metadata(&request)
            .expect("SRA metadata lookup should succeed");

        assert_eq!(metadata.provider.as_str(), "sra");
        assert_eq!(metadata.run_accession.as_deref(), Some("SRR123456"));
    }
}

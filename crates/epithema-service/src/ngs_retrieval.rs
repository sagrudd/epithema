//! Service-owned NGS dataset acquisition facade.
//!
//! This module is the formal acquisition seam for provider-backed NGS dataset
//! manifest discovery. It centralizes provider routing, configuration checks,
//! and the future download/provenance method surface outside the CLI and tool
//! implementations.

use std::path::{Path, PathBuf};

use epithema_config::PlatformConfig;
use epithema_diagnostics::{ErrorCategory, PlatformError};
use epithema_providers::{
    EnaNgsAdapter, NgsDownloadPlan, NgsDownloadRecord, NgsManifest, NgsProvenance, NgsQuery,
    ProviderCapability, ProviderHttpClient, ProviderId, ProviderRegistry, ReqwestHttpClient,
    SraNgsAdapter,
};

/// Service-backed NGS dataset retrieval gateway.
#[derive(Clone, Debug)]
pub struct ServiceNgsRetrieval<'a, C> {
    config: &'a PlatformConfig,
    providers: &'a ProviderRegistry,
    client: C,
}

impl<'a> ServiceNgsRetrieval<'a, ReqwestHttpClient> {
    /// Creates a service-backed NGS gateway with the default HTTP client.
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

impl<'a, C> ServiceNgsRetrieval<'a, C> {
    /// Creates a service-backed NGS gateway with an injected HTTP client.
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

impl<C: ProviderHttpClient> ServiceNgsRetrieval<'_, C> {
    /// Retrieves a normalized run-level NGS manifest for a classified query.
    pub fn retrieve_manifest(&self, query: &NgsQuery) -> Result<NgsManifest, PlatformError> {
        let provider = self.ensure_ngs_provider_enabled(query)?;
        let routed_query = query.clone().with_provider(provider.clone());

        match provider.as_str() {
            "ena" => EnaNgsAdapter::new().manifest(&routed_query, &self.client),
            "sra" => SraNgsAdapter::new().manifest(&routed_query, &self.client),
            other => Err(PlatformError::new(
                ErrorCategory::Registry,
                "NGS manifest retrieval is not implemented for the requested provider",
            )
            .with_code("service.ngs_retrieval.unsupported_provider")
            .with_detail(other.to_owned())),
        }
    }

    /// Alias for manifest listing workflows such as the planned `ngslist`.
    pub fn list_manifest(&self, query: &NgsQuery) -> Result<NgsManifest, PlatformError> {
        self.retrieve_manifest(query)
    }

    /// Future materialization-plan entry point for the planned `ngsget`.
    pub fn plan_downloads(
        &self,
        _manifest: &NgsManifest,
        _output_root: impl Into<PathBuf>,
        _include_raw: bool,
    ) -> Result<NgsDownloadPlan, PlatformError> {
        Err(not_implemented(
            "download planning is not implemented by the NGS service gateway yet",
            "service.ngs_retrieval.download_planning_not_implemented",
        ))
    }

    /// Future file materialization entry point for the planned `ngsget`.
    pub fn materialize_download_plan(
        &self,
        _plan: &NgsDownloadPlan,
    ) -> Result<Vec<NgsDownloadRecord>, PlatformError> {
        Err(not_implemented(
            "NGS download materialization is not implemented by the service gateway yet",
            "service.ngs_retrieval.materialization_not_implemented",
        ))
    }

    /// Future verification entry point for materialized NGS assets.
    pub fn verify_materialized_assets(
        &self,
        _records: &[NgsDownloadRecord],
    ) -> Result<Vec<NgsDownloadRecord>, PlatformError> {
        Err(not_implemented(
            "NGS asset verification is not implemented by the service gateway yet",
            "service.ngs_retrieval.verification_not_implemented",
        ))
    }

    /// Future provenance writer entry point for NGS acquisition runs.
    pub fn write_provenance(
        &self,
        _provenance: &NgsProvenance,
        _path: &Path,
    ) -> Result<(), PlatformError> {
        Err(not_implemented(
            "NGS provenance serialization is not implemented by the service gateway yet",
            "service.ngs_retrieval.provenance_writing_not_implemented",
        ))
    }

    fn ensure_ngs_provider_enabled(&self, query: &NgsQuery) -> Result<ProviderId, PlatformError> {
        if !self.config.acquisition.allow_remote_acquisition {
            return Err(PlatformError::new(
                ErrorCategory::Configuration,
                "remote provider acquisition is disabled by platform policy",
            )
            .with_code("service.ngs_retrieval.remote_disabled"));
        }

        let provider = resolve_ngs_provider(query)?;
        let Some(descriptor) = self.providers.find(&provider) else {
            return Err(PlatformError::new(
                ErrorCategory::Registry,
                "requested NGS provider is not registered in the active service registry",
            )
            .with_code("service.ngs_retrieval.unknown_provider")
            .with_detail(provider.as_str().to_owned()));
        };

        if !descriptor.supports(ProviderCapability::ArchiveAcquisition) {
            return Err(PlatformError::new(
                ErrorCategory::Registry,
                "requested provider does not advertise archive acquisition capability",
            )
            .with_code("service.ngs_retrieval.unsupported_provider")
            .with_detail(provider.as_str().to_owned()));
        }

        if !provider_enabled(self.config, provider.as_str()) {
            return Err(PlatformError::new(
                ErrorCategory::Configuration,
                "requested NGS provider is disabled in platform configuration",
            )
            .with_code("service.ngs_retrieval.provider_disabled")
            .with_detail(provider.as_str().to_owned()));
        }

        Ok(provider)
    }
}

fn resolve_ngs_provider(query: &NgsQuery) -> Result<ProviderId, PlatformError> {
    if query.object_class.is_none() {
        return Err(PlatformError::new(
            ErrorCategory::Validation,
            "NGS manifest retrieval requires a classified query",
        )
        .with_code("service.ngs_retrieval.unclassified_query")
        .with_detail(query.accession.clone()));
    }

    if let Some(provider) = &query.provider {
        return Ok(provider.clone());
    }

    // Provider-neutral NGS queries use ENA as the deterministic auto route
    // because ENA exposes generated FASTQ URLs needed by the planned default
    // `ngsget` behavior. Callers can force SRA with `sra:<accession>`.
    Ok(ProviderId::new("ena").expect("static provider id should be valid"))
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

fn not_implemented(message: &str, code: &'static str) -> PlatformError {
    PlatformError::new(ErrorCategory::Invocation, message).with_code(code)
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use epithema_config::{PlatformConfig, ProviderSettings};
    use epithema_diagnostics::PlatformError;
    use epithema_providers::{
        ArchiveRoute, EnaNgsAdapter, HttpRequest, HttpResponse, NgsManifest, NgsQuery,
        ProviderHttpClient, ProviderId, ProviderRegistry, SraNgsAdapter,
    };

    use super::ServiceNgsRetrieval;

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
                .with_code("service.ngs_retrieval.test.missing_response")
                .with_detail(request.url.clone())
            })
        }
    }

    #[test]
    fn retrieves_ena_ngs_manifest_through_service_gateway() {
        let ena = ProviderId::new("ena").expect("valid provider");
        let config =
            PlatformConfig::default().with_provider(ProviderSettings::enabled(ena.clone()));
        let registry = ProviderRegistry::builtin_defaults();
        let query = NgsQuery::classify("PRJNA1011899").expect("query should classify");
        let routed_query = query.clone().with_provider(ena);
        let request = EnaNgsAdapter::new()
            .build_manifest_request(&routed_query)
            .expect("request should build");
        let body = concat!(
            "run_accession\tstudy_accession\tsecondary_study_accession\texperiment_accession\tsample_accession\tsecondary_sample_accession\tstudy_title\tsample_title\texperiment_title\tscientific_name\tinstrument_platform\tinstrument_model\tlibrary_strategy\tlibrary_source\tlibrary_selection\tlibrary_layout\tfastq_ftp\tfastq_md5\tfastq_bytes\tsubmitted_ftp\tsubmitted_md5\tsubmitted_bytes\tsra_ftp\tsra_md5\tsra_bytes\n",
            "ERR1\tERP1\tPRJNA1011899\tERX1\tERS1\tSAMN1\tStudy title\tSample one\tExperiment one\tHomo sapiens\tILLUMINA\tNovaSeq 6000\tWGS\tGENOMIC\tRANDOM\tPAIRED\tftp.sra.ebi.ac.uk/vol1/fastq/ERR1/ERR1_1.fastq.gz\tmd51\t10\t\t\t\t\t\t\n"
        );
        let client =
            MockHttpClient::default().with_response(request.url, HttpResponse::new(200, body));
        let gateway = ServiceNgsRetrieval::with_client(&config, &registry, client);

        let manifest = gateway
            .retrieve_manifest(&query)
            .expect("ENA NGS manifest retrieval should succeed");

        assert_eq!(manifest.provider.as_str(), "ena");
        assert_eq!(manifest.runs.len(), 1);
        assert_eq!(manifest.assets().len(), 1);
    }

    #[test]
    fn retrieves_sra_ngs_manifest_through_service_gateway() {
        let sra = ProviderId::new("sra").expect("valid provider");
        let config =
            PlatformConfig::default().with_provider(ProviderSettings::enabled(sra.clone()));
        let registry = ProviderRegistry::builtin_defaults();
        let query = NgsQuery::classify("sra:SRR123456").expect("query should classify");
        let request = SraNgsAdapter::new()
            .build_manifest_request(&query)
            .expect("request should build");
        let body = concat!(
            "Run,ReleaseDate,LoadDate,spots,bases,spots_with_mates,avgLength,size_MB,AssemblyName,download_path,Experiment,LibraryName,LibraryStrategy,LibrarySelection,LibrarySource,LibraryLayout,InsertSize,InsertDev,Platform,Model,SRAStudy,BioProject,StudyTitle,ProjectID,Sample,BioSample,SampleType,TaxID,ScientificName,SampleName,CenterName,Submission,dbgap_study_accession,Consent,RunHash,ReadHash\n",
            "SRR123456,2024-01-01,2024-01-02,1,100,1,100,1,,https://example.invalid/SRR123456.sra,SRX123456,,WGS,RANDOM,GENOMIC,PAIRED,,,ILLUMINA,NextSeq 2000,SRP1,PRJNA1,Study title,1,SRS123456,SAMN1,,9606,Homo sapiens,Sample one,NCBI,SRA1,,,runhash,readhash\n"
        );
        let client =
            MockHttpClient::default().with_response(request.url, HttpResponse::new(200, body));
        let gateway = ServiceNgsRetrieval::with_client(&config, &registry, client);

        let manifest = gateway
            .retrieve_manifest(&query)
            .expect("SRA NGS manifest retrieval should succeed");

        assert_eq!(manifest.provider.as_str(), "sra");
        assert_eq!(manifest.runs.len(), 1);
        assert_eq!(manifest.assets().len(), 2);
    }

    #[test]
    fn rejects_ngs_manifest_when_remote_acquisition_is_disabled() {
        let mut config = PlatformConfig::default();
        config.acquisition.allow_remote_acquisition = false;
        let registry = ProviderRegistry::builtin_defaults();
        let gateway =
            ServiceNgsRetrieval::with_client(&config, &registry, MockHttpClient::default());
        let query = NgsQuery::classify("ena:ERR123456").expect("query should classify");

        let error = gateway
            .retrieve_manifest(&query)
            .expect_err("remote-disabled policy should fail");

        assert_eq!(error.code(), Some("service.ngs_retrieval.remote_disabled"));
    }

    #[test]
    fn rejects_ngs_manifest_when_provider_is_disabled() {
        let config = PlatformConfig::default().with_provider(ProviderSettings {
            id: ProviderId::new("ena").expect("valid provider"),
            enabled: false,
        });
        let registry = ProviderRegistry::builtin_defaults();
        let gateway =
            ServiceNgsRetrieval::with_client(&config, &registry, MockHttpClient::default());
        let query = NgsQuery::classify("ena:ERR123456").expect("query should classify");

        let error = gateway
            .retrieve_manifest(&query)
            .expect_err("disabled provider should fail");

        assert_eq!(
            error.code(),
            Some("service.ngs_retrieval.provider_disabled")
        );
    }

    #[test]
    fn exposes_guarded_future_ngsget_methods() {
        let config = PlatformConfig::default();
        let registry = ProviderRegistry::builtin_defaults();
        let gateway =
            ServiceNgsRetrieval::with_client(&config, &registry, MockHttpClient::default());
        let provider = ProviderId::new("ena").expect("static provider id should be valid");
        let query = NgsQuery::classify("ena:ERR123456").expect("query should classify");
        let manifest = NgsManifest::new(
            query,
            provider.clone(),
            ArchiveRoute::new(provider, "ena.portal.filereport.read_run", "tsv"),
            Vec::new(),
        );

        let error = gateway
            .plan_downloads(&manifest, "ngs-out", false)
            .expect_err("download planning should be guarded");

        assert_eq!(
            error.code(),
            Some("service.ngs_retrieval.download_planning_not_implemented")
        );
    }
}

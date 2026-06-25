//! Service-owned NGS dataset acquisition facade.
//!
//! This module is the formal acquisition seam for provider-backed NGS dataset
//! manifest discovery. It centralizes provider routing, configuration checks,
//! and the future download/provenance method surface outside the CLI and tool
//! implementations.

use std::fs;
use std::path::{Path, PathBuf};

use epithema_config::PlatformConfig;
use epithema_diagnostics::{ErrorCategory, PlatformError};
use epithema_providers::{
    EnaNgsAdapter, HttpRequest, NgsAsset, NgsAssetRole, NgsDownloadPlan, NgsDownloadRecord,
    NgsManifest, NgsProvenance, NgsQuery, NgsVerificationStatus, ProviderCapability,
    ProviderHttpClient, ProviderId, ProviderRegistry, ReqwestHttpClient, SraNgsAdapter,
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

    /// Builds a deterministic materialization plan for the planned `ngsget`.
    pub fn plan_downloads(
        &self,
        manifest: &NgsManifest,
        output_root: impl Into<PathBuf>,
        include_raw: bool,
    ) -> Result<NgsDownloadPlan, PlatformError> {
        Ok(NgsDownloadPlan::new(
            manifest.clone(),
            output_root,
            include_raw,
            select_ngs_assets_for_download(manifest, include_raw),
        ))
    }

    /// Materializes directly downloadable ENA assets selected by `ngsget`.
    pub fn materialize_download_plan(
        &self,
        plan: &NgsDownloadPlan,
    ) -> Result<Vec<NgsDownloadRecord>, PlatformError> {
        if plan.manifest.provider.as_str() != "ena" {
            return Err(not_implemented(
                "NGS materialization is currently implemented for direct ENA downloads only",
                "service.ngs_retrieval.materialization_provider_not_implemented",
            ));
        }

        plan.selected_assets
            .iter()
            .map(|asset| materialize_direct_ngs_asset(&self.client, plan, asset))
            .collect()
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

fn select_ngs_assets_for_download(manifest: &NgsManifest, include_raw: bool) -> Vec<NgsAsset> {
    manifest
        .runs
        .iter()
        .flat_map(|run| run.assets.iter())
        .filter(|asset| should_select_ngs_asset(asset.role, include_raw))
        .cloned()
        .collect()
}

fn should_select_ngs_asset(role: NgsAssetRole, include_raw: bool) -> bool {
    role == NgsAssetRole::GeneratedFastq
        || (include_raw
            && matches!(
                role,
                NgsAssetRole::SubmittedRaw
                    | NgsAssetRole::SubmittedAlignment
                    | NgsAssetRole::SraArchive
                    | NgsAssetRole::Index
                    | NgsAssetRole::UnknownSubmitted
            ))
}

fn materialize_direct_ngs_asset<C: ProviderHttpClient>(
    client: &C,
    plan: &NgsDownloadPlan,
    asset: &NgsAsset,
) -> Result<NgsDownloadRecord, PlatformError> {
    let local_path = local_ngs_asset_path(&plan.output_root, asset);
    if let Some((observed_size, observed_checksum)) =
        verified_existing_asset_evidence(&local_path, asset)?
    {
        return Ok(NgsDownloadRecord::new(asset.clone(), local_path)
            .with_observed_evidence(Some(observed_size), Some(observed_checksum))
            .with_verification_status(NgsVerificationStatus::SkippedVerified));
    }

    let Some(download_url) = direct_download_url(&asset.source_url) else {
        return Ok(NgsDownloadRecord::new(asset.clone(), local_path)
            .with_verification_status(NgsVerificationStatus::Failed)
            .with_failure_reason("asset source is not a direct ENA download URL"));
    };

    let response = client
        .get_bytes(&HttpRequest::new(download_url).with_accept("application/octet-stream, */*"))?;
    if !(200..300).contains(&response.status) {
        return Ok(NgsDownloadRecord::new(asset.clone(), local_path)
            .with_verification_status(NgsVerificationStatus::Failed)
            .with_failure_reason(format!("download returned HTTP status {}", response.status)));
    }

    let partial_path = partial_ngs_asset_path(&local_path);
    if let Some(parent) = partial_path.parent() {
        fs::create_dir_all(parent)
            .map_err(|error| ngs_io_error("create output directory", error))?;
    }
    fs::write(&partial_path, &response.body)
        .map_err(|error| ngs_io_error("write partial NGS download", error))?;

    let observed_size = u64::try_from(response.body.len()).ok();
    let observed_checksum = Some(format!("{:x}", md5::compute(&response.body)));
    let verification_failure = ngs_verification_failure(asset, observed_size, &observed_checksum);
    let mut record = NgsDownloadRecord::new(asset.clone(), local_path.clone())
        .with_observed_evidence(observed_size, observed_checksum.clone());

    if let Some(reason) = verification_failure {
        return Ok(record
            .with_verification_status(NgsVerificationStatus::Failed)
            .with_failure_reason(reason));
    }

    if local_path.exists() {
        fs::remove_file(&local_path)
            .map_err(|error| ngs_io_error("replace existing NGS download", error))?;
    }
    fs::rename(&partial_path, &local_path)
        .map_err(|error| ngs_io_error("promote verified NGS download", error))?;

    let status = if asset.size_bytes.is_some() || asset.checksum_md5.is_some() {
        NgsVerificationStatus::Verified
    } else {
        NgsVerificationStatus::Unverified
    };
    record = record.with_verification_status(status);
    Ok(record)
}

fn direct_download_url(source_url: &str) -> Option<String> {
    if let Some(path) = source_url.strip_prefix("ftp://") {
        Some(format!("https://{path}"))
    } else if source_url.starts_with("http://") || source_url.starts_with("https://") {
        Some(source_url.to_owned())
    } else {
        None
    }
}

fn local_ngs_asset_path(output_root: &Path, asset: &NgsAsset) -> PathBuf {
    output_root
        .join("runs")
        .join(sanitize_path_component(&asset.run_accession))
        .join(local_ngs_asset_directory(asset.role))
        .join(local_ngs_asset_filename(asset))
}

fn local_ngs_asset_directory(role: NgsAssetRole) -> &'static str {
    match role {
        NgsAssetRole::GeneratedFastq => "fastq",
        NgsAssetRole::SraArchive => "sra",
        NgsAssetRole::SubmittedRaw
        | NgsAssetRole::SubmittedAlignment
        | NgsAssetRole::Index
        | NgsAssetRole::UnknownSubmitted => "raw",
    }
}

fn local_ngs_asset_filename(asset: &NgsAsset) -> String {
    let without_fragment = asset
        .source_url
        .split('#')
        .next()
        .unwrap_or(&asset.source_url);
    let without_query = without_fragment
        .split('?')
        .next()
        .unwrap_or(without_fragment);
    let candidate = without_query
        .rsplit('/')
        .next()
        .filter(|value| !value.is_empty())
        .unwrap_or(asset.source_url.as_str());
    let sanitized = sanitize_path_component(candidate);
    if sanitized.is_empty() {
        format!(
            "{}_{}.{}",
            sanitize_path_component(&asset.run_accession),
            asset.role.as_str(),
            sanitize_path_component(&asset.format)
        )
    } else {
        sanitized
    }
}

fn sanitize_path_component(value: &str) -> String {
    let sanitized: String = value
        .chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() || matches!(ch, '.' | '-' | '_') {
                ch
            } else {
                '_'
            }
        })
        .collect();

    if sanitized.is_empty() || sanitized == "." || sanitized == ".." {
        "_".to_owned()
    } else {
        sanitized
    }
}

fn partial_ngs_asset_path(local_path: &Path) -> PathBuf {
    let file_name = local_path
        .file_name()
        .and_then(|value| value.to_str())
        .unwrap_or("asset");
    local_path.with_file_name(format!("{file_name}.partial"))
}

fn verified_existing_asset_evidence(
    local_path: &Path,
    asset: &NgsAsset,
) -> Result<Option<(u64, String)>, PlatformError> {
    if !local_path.exists() || (asset.size_bytes.is_none() && asset.checksum_md5.is_none()) {
        return Ok(None);
    }

    let body =
        fs::read(local_path).map_err(|error| ngs_io_error("read existing NGS download", error))?;
    let observed_size = u64::try_from(body.len()).map_err(|error| {
        PlatformError::new(
            ErrorCategory::Invocation,
            "existing NGS download size could not be represented as u64",
        )
        .with_code("service.ngs_retrieval.size_overflow")
        .with_detail(error.to_string())
    })?;
    let observed_checksum = format!("{:x}", md5::compute(&body));
    let observed_checksum_option = Some(observed_checksum.clone());
    if ngs_verification_failure(asset, Some(observed_size), &observed_checksum_option).is_none() {
        Ok(Some((observed_size, observed_checksum)))
    } else {
        Ok(None)
    }
}

fn ngs_verification_failure(
    asset: &NgsAsset,
    observed_size: Option<u64>,
    observed_checksum: &Option<String>,
) -> Option<String> {
    if let (Some(expected), Some(observed)) = (asset.size_bytes, observed_size) {
        if expected != observed {
            return Some(format!(
                "byte count mismatch: expected {expected}, observed {observed}"
            ));
        }
    }

    if let (Some(expected), Some(observed)) = (&asset.checksum_md5, observed_checksum) {
        if !expected.eq_ignore_ascii_case(observed) {
            return Some(format!(
                "MD5 checksum mismatch: expected {}, observed {}",
                expected, observed
            ));
        }
    }

    None
}

fn ngs_io_error(action: &str, error: std::io::Error) -> PlatformError {
    PlatformError::new(ErrorCategory::Invocation, format!("failed to {action}"))
        .with_code("service.ngs_retrieval.io_failed")
        .with_detail(error.to_string())
}

fn not_implemented(message: &str, code: &'static str) -> PlatformError {
    PlatformError::new(ErrorCategory::Invocation, message).with_code(code)
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::fs;
    use std::path::PathBuf;
    use std::time::{SystemTime, UNIX_EPOCH};

    use epithema_config::{PlatformConfig, ProviderSettings};
    use epithema_diagnostics::PlatformError;
    use epithema_providers::{
        ArchiveRoute, EnaNgsAdapter, HttpBytesResponse, HttpRequest, HttpResponse, NgsAsset,
        NgsAssetRole, NgsDownloadPlan, NgsManifest, NgsManifestRun, NgsQuery, NgsRunMetadata,
        NgsVerificationStatus, ProviderHttpClient, ProviderId, ProviderRegistry, SraNgsAdapter,
    };

    use super::ServiceNgsRetrieval;

    #[derive(Clone, Debug, Default)]
    struct MockHttpClient {
        responses: HashMap<String, HttpResponse>,
        byte_responses: HashMap<String, HttpBytesResponse>,
    }

    impl MockHttpClient {
        fn with_response(mut self, url: impl Into<String>, response: HttpResponse) -> Self {
            self.responses.insert(url.into(), response);
            self
        }

        fn with_byte_response(
            mut self,
            url: impl Into<String>,
            response: HttpBytesResponse,
        ) -> Self {
            self.byte_responses.insert(url.into(), response);
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

        fn get_bytes(&self, request: &HttpRequest) -> Result<HttpBytesResponse, PlatformError> {
            self.byte_responses
                .get(&request.url)
                .cloned()
                .ok_or_else(|| {
                    PlatformError::new(
                        epithema_diagnostics::ErrorCategory::Invocation,
                        "mock byte response was not configured for provider request",
                    )
                    .with_code("service.ngs_retrieval.test.missing_byte_response")
                    .with_detail(request.url.clone())
                })
        }
    }

    fn planned_manifest() -> NgsManifest {
        let provider = ProviderId::new("ena").expect("static provider id should be valid");
        let query = NgsQuery::classify("ena:ERR123456").expect("query should classify");
        let metadata = NgsRunMetadata::new("ERR123456");
        let assets = vec![
            NgsAsset::new(
                "ERR123456",
                NgsAssetRole::GeneratedFastq,
                "fastq.gz",
                "ftp://example.invalid/ERR123456.fastq.gz",
            ),
            NgsAsset::new(
                "ERR123456",
                NgsAssetRole::SubmittedRaw,
                "pod5",
                "ftp://example.invalid/ERR123456.pod5",
            ),
            NgsAsset::new(
                "ERR123456",
                NgsAssetRole::SubmittedAlignment,
                "bam",
                "ftp://example.invalid/ERR123456.bam",
            ),
            NgsAsset::new(
                "ERR123456",
                NgsAssetRole::Index,
                "bai",
                "ftp://example.invalid/ERR123456.bam.bai",
            ),
            NgsAsset::new(
                "ERR123456",
                NgsAssetRole::SraArchive,
                "sra",
                "ftp://example.invalid/ERR123456.sra",
            ),
            NgsAsset::new(
                "ERR123456",
                NgsAssetRole::UnknownSubmitted,
                "submitted",
                "ftp://example.invalid/ERR123456.dat",
            ),
        ];
        NgsManifest::new(
            query,
            provider.clone(),
            ArchiveRoute::new(provider, "ena.portal.filereport.read_run", "tsv"),
            vec![NgsManifestRun::new(metadata, assets)],
        )
    }

    fn sra_planned_manifest() -> NgsManifest {
        let provider = ProviderId::new("sra").expect("static provider id should be valid");
        let query = NgsQuery::classify("sra:SRR123456").expect("query should classify");
        let metadata = NgsRunMetadata::new("SRR123456");
        let assets = vec![NgsAsset::new(
            "SRR123456",
            NgsAssetRole::GeneratedFastq,
            "fastq",
            "sra-convert://SRR123456/fastq",
        )];
        NgsManifest::new(
            query,
            provider.clone(),
            ArchiveRoute::new(provider, "sra.runinfo", "csv"),
            vec![NgsManifestRun::new(metadata, assets)],
        )
    }

    fn temp_ngs_output_root(label: &str) -> PathBuf {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time should be after unix epoch")
            .as_nanos();
        std::env::temp_dir().join(format!(
            "epithema-ngs-{label}-{}-{nanos}",
            std::process::id()
        ))
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
    fn plans_generated_fastq_assets_by_default() {
        let config = PlatformConfig::default();
        let registry = ProviderRegistry::builtin_defaults();
        let gateway =
            ServiceNgsRetrieval::with_client(&config, &registry, MockHttpClient::default());
        let manifest = planned_manifest();

        let plan = gateway
            .plan_downloads(&manifest, "ngs-out", false)
            .expect("default NGS download planning should succeed");

        assert_eq!(plan.output_root, PathBuf::from("ngs-out"));
        assert!(!plan.include_raw);
        assert_eq!(plan.manifest, manifest);
        assert_eq!(plan.selected_assets.len(), 1);
        assert_eq!(plan.selected_assets[0].role, NgsAssetRole::GeneratedFastq);
    }

    #[test]
    fn plans_raw_and_submitted_assets_when_requested() {
        let config = PlatformConfig::default();
        let registry = ProviderRegistry::builtin_defaults();
        let gateway =
            ServiceNgsRetrieval::with_client(&config, &registry, MockHttpClient::default());
        let manifest = planned_manifest();

        let plan = gateway
            .plan_downloads(&manifest, "ngs-out", true)
            .expect("raw-inclusive NGS download planning should succeed");

        assert!(plan.include_raw);
        assert_eq!(
            plan.selected_assets
                .iter()
                .map(|asset| asset.role.as_str())
                .collect::<Vec<_>>(),
            vec![
                "generated_fastq",
                "submitted_raw",
                "submitted_alignment",
                "index",
                "sra_archive",
                "unknown_submitted",
            ]
        );
    }

    #[test]
    fn materializes_direct_ena_downloads_with_verification() {
        let config = PlatformConfig::default();
        let registry = ProviderRegistry::builtin_defaults();
        let body = b"ACGT\n".to_vec();
        let checksum = format!("{:x}", md5::compute(&body));
        let asset = NgsAsset::new(
            "ERR123456",
            NgsAssetRole::GeneratedFastq,
            "fastq.gz",
            "ftp://example.invalid/ERR123456.fastq.gz",
        )
        .with_size_bytes(Some(5))
        .with_checksum_md5(Some(checksum.clone()));
        let mut manifest = planned_manifest();
        manifest.runs[0].assets = vec![asset.clone()];
        let output_root = temp_ngs_output_root("download");
        let plan = NgsDownloadPlan::new(manifest, output_root.clone(), false, vec![asset]);
        let client = MockHttpClient::default().with_byte_response(
            "https://example.invalid/ERR123456.fastq.gz",
            HttpBytesResponse::new(200, body.clone()),
        );
        let gateway = ServiceNgsRetrieval::with_client(&config, &registry, client);

        let records = gateway
            .materialize_download_plan(&plan)
            .expect("direct ENA download should materialize");

        assert_eq!(records.len(), 1);
        assert_eq!(
            records[0].verification_status,
            NgsVerificationStatus::Verified
        );
        assert_eq!(records[0].observed_size_bytes, Some(5));
        assert_eq!(
            records[0].observed_checksum_md5.as_deref(),
            Some(checksum.as_str())
        );
        assert_eq!(
            records[0].local_path,
            output_root.join("runs/ERR123456/fastq/ERR123456.fastq.gz")
        );
        assert_eq!(
            fs::read(&records[0].local_path).expect("downloaded file should be readable"),
            body
        );
        assert!(
            !output_root
                .join("runs/ERR123456/fastq/ERR123456.fastq.gz.partial")
                .exists()
        );
        fs::remove_dir_all(output_root).ok();
    }

    #[test]
    fn skips_existing_verified_ena_download() {
        let config = PlatformConfig::default();
        let registry = ProviderRegistry::builtin_defaults();
        let body = b"ACGT\n".to_vec();
        let checksum = format!("{:x}", md5::compute(&body));
        let asset = NgsAsset::new(
            "ERR123456",
            NgsAssetRole::GeneratedFastq,
            "fastq.gz",
            "ftp://example.invalid/ERR123456.fastq.gz",
        )
        .with_size_bytes(Some(5))
        .with_checksum_md5(Some(checksum));
        let mut manifest = planned_manifest();
        manifest.runs[0].assets = vec![asset.clone()];
        let output_root = temp_ngs_output_root("skip");
        let local_path = output_root.join("runs/ERR123456/fastq/ERR123456.fastq.gz");
        fs::create_dir_all(local_path.parent().expect("local path should have parent"))
            .expect("download directory should be created");
        fs::write(&local_path, &body).expect("existing verified download should be written");
        let plan = NgsDownloadPlan::new(manifest, output_root.clone(), false, vec![asset]);
        let gateway =
            ServiceNgsRetrieval::with_client(&config, &registry, MockHttpClient::default());

        let records = gateway
            .materialize_download_plan(&plan)
            .expect("verified existing file should be skipped");

        assert_eq!(records.len(), 1);
        assert_eq!(
            records[0].verification_status,
            NgsVerificationStatus::SkippedVerified
        );
        assert_eq!(records[0].local_path, local_path);
        fs::remove_dir_all(output_root).ok();
    }

    #[test]
    fn leaves_partial_file_when_ena_download_fails_verification() {
        let config = PlatformConfig::default();
        let registry = ProviderRegistry::builtin_defaults();
        let body = b"ACGT\n".to_vec();
        let asset = NgsAsset::new(
            "ERR123456",
            NgsAssetRole::GeneratedFastq,
            "fastq.gz",
            "ftp://example.invalid/ERR123456.fastq.gz",
        )
        .with_size_bytes(Some(999))
        .with_checksum_md5(Some("00000000000000000000000000000000".to_owned()));
        let mut manifest = planned_manifest();
        manifest.runs[0].assets = vec![asset.clone()];
        let output_root = temp_ngs_output_root("mismatch");
        let plan = NgsDownloadPlan::new(manifest, output_root.clone(), false, vec![asset]);
        let client = MockHttpClient::default().with_byte_response(
            "https://example.invalid/ERR123456.fastq.gz",
            HttpBytesResponse::new(200, body),
        );
        let gateway = ServiceNgsRetrieval::with_client(&config, &registry, client);

        let records = gateway
            .materialize_download_plan(&plan)
            .expect("verification failure should be recorded");

        assert_eq!(records.len(), 1);
        assert_eq!(
            records[0].verification_status,
            NgsVerificationStatus::Failed
        );
        assert!(
            records[0]
                .failure_reason
                .as_deref()
                .is_some_and(|reason| reason.contains("byte count mismatch"))
        );
        assert!(!records[0].local_path.exists());
        assert!(
            output_root
                .join("runs/ERR123456/fastq/ERR123456.fastq.gz.partial")
                .exists()
        );
        fs::remove_dir_all(output_root).ok();
    }

    #[test]
    fn keeps_sra_materialization_guarded_for_conversion_task() {
        let config = PlatformConfig::default();
        let registry = ProviderRegistry::builtin_defaults();
        let gateway =
            ServiceNgsRetrieval::with_client(&config, &registry, MockHttpClient::default());
        let manifest = sra_planned_manifest();
        let selected_assets = manifest.assets().into_iter().cloned().collect();
        let plan = NgsDownloadPlan::new(manifest, "ngs-out", false, selected_assets);

        let error = gateway
            .materialize_download_plan(&plan)
            .expect_err("SRA conversion remains a later task");

        assert_eq!(
            error.code(),
            Some("service.ngs_retrieval.materialization_provider_not_implemented")
        );
    }
}

//! Service-owned NGS dataset acquisition facade.
//!
//! This module is the formal acquisition seam for provider-backed NGS dataset
//! manifest discovery. It centralizes provider routing, configuration checks,
//! and the future download/provenance method surface outside the CLI and tool
//! implementations.

use std::io::Read;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::Mutex;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;
use std::time::Duration;
use std::{env, fs};

use epithema_config::PlatformConfig;
use epithema_diagnostics::{ArtifactOriginKind, ArtifactProvenance, ErrorCategory, PlatformError};
use epithema_providers::{
    EnaNgsAdapter, HttpDownloadProgress, HttpDownloadProgressState, HttpRequest, NgsAsset,
    NgsAssetRole, NgsDownloadPlan, NgsDownloadRecord, NgsManifest, NgsManifestRun, NgsProvenance,
    NgsQuery, NgsRunMetadata, NgsVerificationStatus, ProviderCapability, ProviderHttpClient,
    ProviderId, ProviderRegistry, ReqwestHttpClient, SraNgsAdapter,
};
use serde_json::{Value, json};

const DEFAULT_SRA_TOOLKIT_CONTAINER: &str = "docker.io/ncbi/sra-tools:3.1.1";
const DEFAULT_SRA_TOOLKIT_VERSION: &str = "3.1.1";
const MAX_NGS_DOWNLOAD_THREADS: usize = 20;
const DEFAULT_ASPERA_TARGET_RATE: &str = "300m";

/// Callback used to report streamed NGS file download progress.
pub type NgsDownloadProgressCallback = dyn Fn(HttpDownloadProgress) + Send + Sync + 'static;

/// Direct-download transport mode for NGS assets.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum NgsDownloadTransport {
    /// Use the existing HTTPS streaming downloader.
    Https,
    /// Prefer Aspera when possible, otherwise use HTTPS.
    Auto,
    /// Require Aspera for eligible ENA file URLs.
    Aspera,
}

impl NgsDownloadTransport {
    /// Returns the stable lowercase label for the transport.
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Https => "https",
            Self::Auto => "auto",
            Self::Aspera => "aspera",
        }
    }
}

/// Optional Aspera command configuration for ENA direct downloads.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NgsAsperaConfig {
    /// Path or command name for IBM Aspera `ascp`.
    pub ascp_path: PathBuf,
    /// Private key used for public ENA Aspera authentication.
    pub key_path: Option<PathBuf>,
    /// Aspera target transfer rate, passed to `ascp -l`.
    pub target_rate: String,
}

impl Default for NgsAsperaConfig {
    fn default() -> Self {
        Self {
            ascp_path: PathBuf::from("ascp"),
            key_path: default_aspera_key_path(),
            target_rate: DEFAULT_ASPERA_TARGET_RATE.to_owned(),
        }
    }
}

/// Transport selection and command configuration for direct NGS downloads.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NgsDownloadTransportConfig {
    /// Requested transport mode.
    pub mode: NgsDownloadTransport,
    /// Aspera command configuration used by `auto` and `aspera`.
    pub aspera: NgsAsperaConfig,
}

impl Default for NgsDownloadTransportConfig {
    fn default() -> Self {
        Self {
            mode: NgsDownloadTransport::Https,
            aspera: NgsAsperaConfig::default(),
        }
    }
}

/// Request passed to an SRA FASTQ conversion runner.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SraFastqConversionRequest {
    /// Run accession being converted.
    pub run_accession: String,
    /// Root of the NGS materialization tree.
    pub output_root: PathBuf,
    /// Expected SRA archive path within the materialization tree.
    pub sra_archive_path: PathBuf,
    /// Directory where FASTQ outputs should be written.
    pub fastq_dir: PathBuf,
    /// Stable human-readable command line for provenance.
    pub command_line: String,
    /// Container image used for the conversion.
    pub container_image: String,
    /// SRA Toolkit version associated with the container image.
    pub tool_version: String,
}

/// Result returned by an SRA FASTQ conversion runner.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SraFastqConversionResult {
    /// External command exit status.
    pub exit_status: i32,
}

/// Executes a planned SRA FASTQ conversion.
pub trait SraFastqRunner {
    /// Runs the conversion request and returns the external command status.
    fn run(
        &self,
        request: &SraFastqConversionRequest,
    ) -> Result<SraFastqConversionResult, PlatformError>;
}

#[derive(Clone, Copy, Debug, Default)]
struct DockerSraFastqRunner;

impl SraFastqRunner for DockerSraFastqRunner {
    fn run(
        &self,
        request: &SraFastqConversionRequest,
    ) -> Result<SraFastqConversionResult, PlatformError> {
        fs::create_dir_all(&request.fastq_dir)
            .map_err(|error| ngs_io_error("create SRA FASTQ output directory", error))?;
        let script = sra_fastq_container_script(&request.run_accession);
        let status = Command::new("docker")
            .arg("run")
            .arg("--rm")
            .arg("-v")
            .arg(format!("{}:/work", request.output_root.display()))
            .arg(&request.container_image)
            .arg("sh")
            .arg("-lc")
            .arg(script)
            .status()
            .map_err(|error| {
                PlatformError::new(
                    ErrorCategory::Invocation,
                    "failed to execute SRA Toolkit container",
                )
                .with_code("service.ngs_retrieval.sra_container_failed")
                .with_detail(error.to_string())
            })?;

        Ok(SraFastqConversionResult {
            exit_status: status.code().unwrap_or(-1),
        })
    }
}

/// Service-backed NGS dataset retrieval gateway.
#[derive(Clone)]
pub struct ServiceNgsRetrieval<'a, C> {
    config: &'a PlatformConfig,
    providers: &'a ProviderRegistry,
    client: C,
    progress_callback: Option<&'a NgsDownloadProgressCallback>,
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
            progress_callback: None,
        })
    }

    /// Creates a service-backed NGS gateway with download progress reporting.
    pub fn new_with_progress(
        config: &'a PlatformConfig,
        providers: &'a ProviderRegistry,
        progress_callback: Option<&'a NgsDownloadProgressCallback>,
    ) -> Result<Self, PlatformError> {
        Ok(Self {
            config,
            providers,
            client: ReqwestHttpClient::new()?,
            progress_callback,
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
        Self::with_client_and_progress(config, providers, client, None)
    }

    /// Creates a service-backed NGS gateway with an injected HTTP client and progress reporting.
    #[must_use]
    pub fn with_client_and_progress(
        config: &'a PlatformConfig,
        providers: &'a ProviderRegistry,
        client: C,
        progress_callback: Option<&'a NgsDownloadProgressCallback>,
    ) -> Self {
        Self {
            config,
            providers,
            client,
            progress_callback,
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

    /// Materializes directly downloadable assets and plans SRA FASTQ conversion.
    pub fn materialize_download_plan(
        &self,
        plan: &NgsDownloadPlan,
    ) -> Result<Vec<NgsDownloadRecord>, PlatformError> {
        self.materialize_download_plan_with_sra_runner_and_existing_downloads_sequential(
            plan,
            &DockerSraFastqRunner,
            &[],
            &NgsDownloadTransportConfig::default(),
        )
    }

    /// Materializes selected assets using the default SRA runner and thread cap.
    pub fn materialize_download_plan_with_threads(
        &self,
        plan: &NgsDownloadPlan,
        download_threads: usize,
    ) -> Result<Vec<NgsDownloadRecord>, PlatformError>
    where
        C: Sync,
    {
        self.materialize_download_plan_with_sra_runner_and_threads(
            plan,
            &DockerSraFastqRunner,
            download_threads,
            &NgsDownloadTransportConfig::default(),
        )
    }

    /// Materializes selected assets, checking existing download roots first.
    pub fn materialize_download_plan_with_existing_downloads(
        &self,
        plan: &NgsDownloadPlan,
        existing_download_roots: &[PathBuf],
    ) -> Result<Vec<NgsDownloadRecord>, PlatformError> {
        self.materialize_download_plan_with_sra_runner_and_existing_downloads_sequential(
            plan,
            &DockerSraFastqRunner,
            existing_download_roots,
            &NgsDownloadTransportConfig::default(),
        )
    }

    /// Materializes selected assets with existing download roots and thread cap.
    pub fn materialize_download_plan_with_existing_downloads_and_threads(
        &self,
        plan: &NgsDownloadPlan,
        existing_download_roots: &[PathBuf],
        download_threads: usize,
    ) -> Result<Vec<NgsDownloadRecord>, PlatformError>
    where
        C: Sync,
    {
        self.materialize_download_plan_with_sra_runner_existing_downloads_and_threads(
            plan,
            &DockerSraFastqRunner,
            existing_download_roots,
            download_threads,
            &NgsDownloadTransportConfig::default(),
        )
    }

    /// Materializes selected assets with existing roots, thread cap, and transport selection.
    pub fn materialize_download_plan_with_existing_downloads_transport_and_threads(
        &self,
        plan: &NgsDownloadPlan,
        existing_download_roots: &[PathBuf],
        download_threads: usize,
        transport_config: &NgsDownloadTransportConfig,
    ) -> Result<Vec<NgsDownloadRecord>, PlatformError>
    where
        C: Sync,
    {
        self.materialize_download_plan_with_sra_runner_existing_downloads_and_threads(
            plan,
            &DockerSraFastqRunner,
            existing_download_roots,
            download_threads,
            transport_config,
        )
    }

    /// Materializes selected assets with an explicit SRA FASTQ conversion runner.
    pub fn materialize_download_plan_with_sra_runner<R: SraFastqRunner>(
        &self,
        plan: &NgsDownloadPlan,
        runner: &R,
    ) -> Result<Vec<NgsDownloadRecord>, PlatformError> {
        self.materialize_download_plan_with_sra_runner_and_existing_downloads_sequential(
            plan,
            runner,
            &[],
            &NgsDownloadTransportConfig::default(),
        )
    }

    /// Materializes selected assets with an explicit SRA FASTQ conversion runner and thread cap.
    pub fn materialize_download_plan_with_sra_runner_and_threads<R: SraFastqRunner>(
        &self,
        plan: &NgsDownloadPlan,
        runner: &R,
        download_threads: usize,
        transport_config: &NgsDownloadTransportConfig,
    ) -> Result<Vec<NgsDownloadRecord>, PlatformError>
    where
        C: Sync,
    {
        self.materialize_download_plan_with_sra_runner_existing_downloads_and_threads(
            plan,
            runner,
            &[],
            download_threads,
            transport_config,
        )
    }

    /// Materializes selected assets with an explicit runner and existing download roots.
    pub fn materialize_download_plan_with_sra_runner_and_existing_downloads<R: SraFastqRunner>(
        &self,
        plan: &NgsDownloadPlan,
        runner: &R,
        existing_download_roots: &[PathBuf],
    ) -> Result<Vec<NgsDownloadRecord>, PlatformError> {
        self.materialize_download_plan_with_sra_runner_and_existing_downloads_sequential(
            plan,
            runner,
            existing_download_roots,
            &NgsDownloadTransportConfig::default(),
        )
    }

    fn materialize_download_plan_with_sra_runner_and_existing_downloads_sequential<
        R: SraFastqRunner,
    >(
        &self,
        plan: &NgsDownloadPlan,
        runner: &R,
        existing_download_roots: &[PathBuf],
        transport_config: &NgsDownloadTransportConfig,
    ) -> Result<Vec<NgsDownloadRecord>, PlatformError> {
        match plan.manifest.provider.as_str() {
            "ena" => plan
                .selected_assets
                .iter()
                .map(|asset| {
                    materialize_direct_ngs_asset(
                        &self.client,
                        plan,
                        asset,
                        existing_download_roots,
                        self.progress_callback,
                        transport_config,
                    )
                })
                .collect(),
            "sra" => plan
                .selected_assets
                .iter()
                .map(|asset| {
                    materialize_sra_ngs_asset(
                        &self.client,
                        plan,
                        asset,
                        runner,
                        existing_download_roots,
                        self.progress_callback,
                        transport_config,
                    )
                })
                .collect(),
            _ => Err(not_implemented(
                "NGS materialization is not implemented for the requested provider",
                "service.ngs_retrieval.materialization_provider_not_implemented",
            )),
        }
    }

    /// Materializes selected assets with an explicit runner, existing download roots, and thread cap.
    pub fn materialize_download_plan_with_sra_runner_existing_downloads_and_threads<
        R: SraFastqRunner,
    >(
        &self,
        plan: &NgsDownloadPlan,
        runner: &R,
        existing_download_roots: &[PathBuf],
        download_threads: usize,
        transport_config: &NgsDownloadTransportConfig,
    ) -> Result<Vec<NgsDownloadRecord>, PlatformError>
    where
        C: Sync,
    {
        let download_threads = validate_ngs_download_thread_count(download_threads)?;
        match plan.manifest.provider.as_str() {
            "ena" => materialize_ena_assets_with_threads(
                &self.client,
                plan,
                existing_download_roots,
                self.progress_callback,
                download_threads,
                transport_config,
            ),
            "sra" => plan
                .selected_assets
                .iter()
                .map(|asset| {
                    materialize_sra_ngs_asset(
                        &self.client,
                        plan,
                        asset,
                        runner,
                        existing_download_roots,
                        self.progress_callback,
                        transport_config,
                    )
                })
                .collect(),
            _ => Err(not_implemented(
                "NGS materialization is not implemented for the requested provider",
                "service.ngs_retrieval.materialization_provider_not_implemented",
            )),
        }
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
        provenance: &NgsProvenance,
        path: &Path,
    ) -> Result<(), PlatformError> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .map_err(|error| ngs_io_error("create NGS provenance directory", error))?;
        }
        let body =
            serde_json::to_vec_pretty(&ngs_provenance_json(provenance)).map_err(|error| {
                PlatformError::new(
                    ErrorCategory::Invocation,
                    "failed to serialize NGS provenance JSON",
                )
                .with_code("service.ngs_retrieval.provenance_serialization_failed")
                .with_detail(error.to_string())
            })?;
        fs::write(path, body).map_err(|error| ngs_io_error("write NGS provenance JSON", error))
    }

    /// Writes a stable TSV handoff manifest for a materialized NGS acquisition.
    pub fn write_manifest(
        &self,
        provenance: &NgsProvenance,
        path: &Path,
    ) -> Result<(), PlatformError> {
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .map_err(|error| ngs_io_error("create NGS manifest directory", error))?;
        }
        fs::write(path, ngs_handoff_manifest_tsv(provenance))
            .map_err(|error| ngs_io_error("write NGS manifest TSV", error))
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

fn validate_ngs_download_thread_count(download_threads: usize) -> Result<usize, PlatformError> {
    if (1..=MAX_NGS_DOWNLOAD_THREADS).contains(&download_threads) {
        Ok(download_threads)
    } else {
        Err(PlatformError::new(
            ErrorCategory::Validation,
            format!("ngsget --threads must be between 1 and {MAX_NGS_DOWNLOAD_THREADS}"),
        )
        .with_code("service.ngs_retrieval.invalid_thread_count")
        .with_detail(download_threads.to_string()))
    }
}

fn materialize_ena_assets_with_threads<C: ProviderHttpClient + Sync>(
    client: &C,
    plan: &NgsDownloadPlan,
    existing_download_roots: &[PathBuf],
    progress_callback: Option<&NgsDownloadProgressCallback>,
    download_threads: usize,
    transport_config: &NgsDownloadTransportConfig,
) -> Result<Vec<NgsDownloadRecord>, PlatformError> {
    if download_threads == 1 || plan.selected_assets.len() <= 1 {
        return plan
            .selected_assets
            .iter()
            .map(|asset| {
                materialize_direct_ngs_asset(
                    client,
                    plan,
                    asset,
                    existing_download_roots,
                    progress_callback,
                    transport_config,
                )
            })
            .collect();
    }

    let worker_count = download_threads.min(plan.selected_assets.len());
    let next_index = AtomicUsize::new(0);
    let records = (0..plan.selected_assets.len())
        .map(|_| Mutex::new(None))
        .collect::<Vec<Mutex<Option<NgsDownloadRecord>>>>();
    let first_error = Mutex::new(None);

    thread::scope(|scope| {
        let mut handles = Vec::with_capacity(worker_count);
        for _ in 0..worker_count {
            handles.push(scope.spawn(|| {
                loop {
                    if first_error
                        .lock()
                        .expect("NGS download error state should be lockable")
                        .is_some()
                    {
                        break;
                    }
                    let index = next_index.fetch_add(1, Ordering::SeqCst);
                    let Some(asset) = plan.selected_assets.get(index) else {
                        break;
                    };
                    match materialize_direct_ngs_asset(
                        client,
                        plan,
                        asset,
                        existing_download_roots,
                        progress_callback,
                        transport_config,
                    ) {
                        Ok(record) => {
                            *records[index]
                                .lock()
                                .expect("NGS download record slot should be lockable") =
                                Some(record);
                        }
                        Err(error) => {
                            let mut first_error = first_error
                                .lock()
                                .expect("NGS download error state should be lockable");
                            if first_error.is_none() {
                                *first_error = Some(error);
                            }
                            break;
                        }
                    }
                }
            }));
        }
        for handle in handles {
            if handle.join().is_err() {
                let mut first_error = first_error
                    .lock()
                    .expect("NGS download error state should be lockable");
                if first_error.is_none() {
                    *first_error = Some(
                        PlatformError::new(
                            ErrorCategory::Invocation,
                            "NGS download worker thread panicked",
                        )
                        .with_code("service.ngs_retrieval.worker_panicked"),
                    );
                }
            }
        }
    });

    if let Some(error) = first_error
        .into_inner()
        .expect("NGS download error state should be recoverable")
    {
        return Err(error);
    }

    records
        .into_iter()
        .enumerate()
        .map(|(index, record)| {
            record
                .into_inner()
                .expect("NGS download record slot should be recoverable")
                .ok_or_else(|| {
                    PlatformError::new(
                        ErrorCategory::Invocation,
                        "NGS download worker did not produce a record",
                    )
                    .with_code("service.ngs_retrieval.worker_missing_record")
                    .with_detail(index.to_string())
                })
        })
        .collect()
}

fn materialize_direct_ngs_asset<C: ProviderHttpClient>(
    client: &C,
    plan: &NgsDownloadPlan,
    asset: &NgsAsset,
    existing_download_roots: &[PathBuf],
    progress_callback: Option<&NgsDownloadProgressCallback>,
    transport_config: &NgsDownloadTransportConfig,
) -> Result<NgsDownloadRecord, PlatformError> {
    let local_path = local_ngs_asset_path(&plan.output_root, asset);
    if let Some((observed_size, observed_checksum)) =
        verified_existing_asset_evidence(&local_path, asset)?
    {
        return Ok(NgsDownloadRecord::new(asset.clone(), local_path)
            .with_observed_evidence(Some(observed_size), Some(observed_checksum))
            .with_verification_status(NgsVerificationStatus::SkippedVerified)
            .with_materialization_method("direct_download"));
    }

    if let Some(record) =
        materialize_existing_download_candidate(asset, &local_path, existing_download_roots)?
    {
        return Ok(record);
    }

    if should_use_aspera_transport(&asset.source_url, transport_config) {
        return materialize_aspera_ngs_asset(plan, asset, progress_callback, transport_config);
    }

    let Some(download_url) = direct_download_url(&asset.source_url) else {
        return Ok(NgsDownloadRecord::new(asset.clone(), local_path)
            .with_verification_status(NgsVerificationStatus::Failed)
            .with_failure_reason("asset source is not a direct ENA download URL"));
    };

    let partial_path = partial_ngs_asset_path(&local_path);
    if let Some((observed_size, observed_checksum)) =
        verified_existing_asset_evidence(&partial_path, asset)?
    {
        if local_path.exists() {
            fs::remove_file(&local_path)
                .map_err(|error| ngs_io_error("replace existing NGS download", error))?;
        }
        fs::rename(&partial_path, &local_path)
            .map_err(|error| ngs_io_error("promote verified partial NGS download", error))?;
        return Ok(NgsDownloadRecord::new(asset.clone(), local_path)
            .with_observed_evidence(Some(observed_size), Some(observed_checksum))
            .with_verification_status(NgsVerificationStatus::Verified)
            .with_materialization_method("direct_download_resume"));
    }

    let resume_offset = resumable_partial_size(&partial_path, asset)?;
    let request = HttpRequest::new(download_url).with_accept("application/octet-stream, */*");
    let request = if let Some(resume_offset) = resume_offset {
        request.with_range_start(resume_offset)
    } else {
        request
    };
    let response = client.download_to_path(
        &request,
        &partial_path,
        progress_callback.map(|callback| callback as &dyn Fn(HttpDownloadProgress)),
    )?;
    if !(200..300).contains(&response.status) {
        return Ok(NgsDownloadRecord::new(asset.clone(), local_path)
            .with_verification_status(NgsVerificationStatus::Failed)
            .with_failure_reason(format!("download returned HTTP status {}", response.status)));
    }

    let (observed_size, observed_checksum) = ngs_file_evidence(&partial_path)?;
    let observed_size = Some(observed_size);
    let observed_checksum = Some(observed_checksum);
    let verification_failure = ngs_verification_failure(asset, observed_size, &observed_checksum);
    let mut record = NgsDownloadRecord::new(asset.clone(), local_path.clone())
        .with_observed_evidence(observed_size, observed_checksum.clone())
        .with_materialization_method(if response.resumed_from.is_some() {
            "direct_download_resume"
        } else {
            "direct_download"
        });

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

#[derive(Clone, Debug, Eq, PartialEq)]
struct AsperaDownloadSource {
    user: &'static str,
    host: &'static str,
    remote_path: String,
}

fn materialize_aspera_ngs_asset(
    plan: &NgsDownloadPlan,
    asset: &NgsAsset,
    progress_callback: Option<&NgsDownloadProgressCallback>,
    transport_config: &NgsDownloadTransportConfig,
) -> Result<NgsDownloadRecord, PlatformError> {
    let local_path = local_ngs_asset_path(&plan.output_root, asset);
    let partial_path = partial_ngs_asset_path(&local_path);
    let Some(source) = aspera_download_source(&asset.source_url) else {
        return Ok(NgsDownloadRecord::new(asset.clone(), local_path)
            .with_verification_status(NgsVerificationStatus::Failed)
            .with_materialization_method("aspera_download")
            .with_failure_reason("asset source is not an ENA Aspera-compatible URL"));
    };
    let Some(key_path) = transport_config.aspera.key_path.as_ref() else {
        return Ok(NgsDownloadRecord::new(asset.clone(), local_path)
            .with_verification_status(NgsVerificationStatus::Failed)
            .with_materialization_method("aspera_download")
            .with_failure_reason(
                "Aspera transfer requires an auth key; pass --aspera-key or set EPITHEMA_ASPERA_KEY",
            ));
    };
    if !key_path.exists() {
        return Ok(NgsDownloadRecord::new(asset.clone(), local_path)
            .with_verification_status(NgsVerificationStatus::Failed)
            .with_materialization_method("aspera_download")
            .with_failure_reason(format!(
                "Aspera auth key does not exist: {}",
                key_path.display()
            )));
    }

    if let Some((observed_size, observed_checksum)) =
        verified_existing_asset_evidence(&partial_path, asset)?
    {
        if local_path.exists() {
            fs::remove_file(&local_path)
                .map_err(|error| ngs_io_error("replace existing NGS download", error))?;
        }
        fs::rename(&partial_path, &local_path)
            .map_err(|error| ngs_io_error("promote verified partial NGS download", error))?;
        return Ok(NgsDownloadRecord::new(asset.clone(), local_path)
            .with_observed_evidence(Some(observed_size), Some(observed_checksum))
            .with_verification_status(NgsVerificationStatus::Verified)
            .with_materialization_method("aspera_download_resume"));
    }

    if let Some(parent) = partial_path.parent() {
        fs::create_dir_all(parent)
            .map_err(|error| ngs_io_error("create Aspera NGS download directory", error))?;
    }
    let starting_size = fs::metadata(&partial_path)
        .map(|metadata| metadata.len())
        .unwrap_or(0);
    emit_ngs_download_progress(
        progress_callback,
        HttpDownloadProgressState::Started,
        &asset.source_url,
        &partial_path,
        starting_size,
        asset.size_bytes,
    );

    let remote = format!("{}@{}:{}", source.user, source.host, source.remote_path);
    let command_line =
        aspera_command_line(&transport_config.aspera, key_path, &remote, &partial_path);
    let ascp_log_path = aspera_command_log_path(&partial_path);
    let ascp_log = fs::OpenOptions::new()
        .create(true)
        .write(true)
        .truncate(true)
        .open(&ascp_log_path)
        .map_err(|error| ngs_io_error("open Aspera command log", error))?;
    let ascp_error_log = ascp_log
        .try_clone()
        .map_err(|error| ngs_io_error("prepare Aspera command log", error))?;
    let mut child = match Command::new(&transport_config.aspera.ascp_path)
        .arg("-T")
        .arg("-k1")
        .arg("-l")
        .arg(&transport_config.aspera.target_rate)
        .arg("-P")
        .arg("33001")
        .arg("-i")
        .arg(key_path)
        .arg(&remote)
        .arg(&partial_path)
        .stdout(Stdio::from(ascp_log))
        .stderr(Stdio::from(ascp_error_log))
        .spawn()
    {
        Ok(child) => child,
        Err(error) => {
            return Ok(NgsDownloadRecord::new(asset.clone(), local_path)
                .with_verification_status(NgsVerificationStatus::Failed)
                .with_materialization_method("aspera_download")
                .with_command_line(command_line)
                .with_failure_reason(format!("failed to execute ascp: {error}")));
        }
    };

    let mut last_progress_bytes = starting_size;
    loop {
        match child.try_wait() {
            Ok(Some(_status)) => break,
            Ok(None) => {
                thread::sleep(Duration::from_secs(2));
                if let Ok(metadata) = fs::metadata(&partial_path) {
                    let current_size = metadata.len();
                    if current_size != last_progress_bytes {
                        emit_ngs_download_progress(
                            progress_callback,
                            HttpDownloadProgressState::Advanced,
                            &asset.source_url,
                            &partial_path,
                            current_size,
                            asset.size_bytes,
                        );
                        last_progress_bytes = current_size;
                    }
                }
            }
            Err(error) => {
                return Ok(NgsDownloadRecord::new(asset.clone(), local_path)
                    .with_verification_status(NgsVerificationStatus::Failed)
                    .with_materialization_method("aspera_download")
                    .with_command_line(command_line)
                    .with_failure_reason(format!("failed to inspect ascp status: {error}")));
            }
        }
    }

    let status = match child.wait() {
        Ok(status) => status,
        Err(error) => {
            return Ok(NgsDownloadRecord::new(asset.clone(), local_path)
                .with_verification_status(NgsVerificationStatus::Failed)
                .with_materialization_method("aspera_download")
                .with_command_line(command_line)
                .with_failure_reason(format!("failed to collect ascp status: {error}")));
        }
    };

    if !status.success() {
        return Ok(NgsDownloadRecord::new(asset.clone(), local_path)
            .with_verification_status(NgsVerificationStatus::Failed)
            .with_materialization_method("aspera_download")
            .with_command_line(command_line)
            .with_exit_status(status.code().unwrap_or(-1))
            .with_failure_reason(format!(
                "ascp exited with status {}; stderr: {}",
                status.code().unwrap_or(-1),
                aspera_command_log_excerpt(&ascp_log_path)
            )));
    }

    let (observed_size, observed_checksum) = ngs_file_evidence(&partial_path)?;
    emit_ngs_download_progress(
        progress_callback,
        HttpDownloadProgressState::Finished,
        &asset.source_url,
        &partial_path,
        observed_size,
        asset.size_bytes.or(Some(observed_size)),
    );
    let observed_size = Some(observed_size);
    let observed_checksum = Some(observed_checksum);
    let verification_failure = ngs_verification_failure(asset, observed_size, &observed_checksum);
    let method = if starting_size > 0 {
        "aspera_download_resume"
    } else {
        "aspera_download"
    };
    let mut record = NgsDownloadRecord::new(asset.clone(), local_path.clone())
        .with_observed_evidence(observed_size, observed_checksum.clone())
        .with_materialization_method(method)
        .with_command_line(command_line)
        .with_exit_status(status.code().unwrap_or(0));

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
        .map_err(|error| ngs_io_error("promote verified Aspera NGS download", error))?;

    let status = if asset.size_bytes.is_some() || asset.checksum_md5.is_some() {
        NgsVerificationStatus::Verified
    } else {
        NgsVerificationStatus::Unverified
    };
    record = record.with_verification_status(status);
    Ok(record)
}

fn materialize_sra_ngs_asset<C: ProviderHttpClient>(
    client: &C,
    plan: &NgsDownloadPlan,
    asset: &NgsAsset,
    runner: &impl SraFastqRunner,
    existing_download_roots: &[PathBuf],
    progress_callback: Option<&NgsDownloadProgressCallback>,
    transport_config: &NgsDownloadTransportConfig,
) -> Result<NgsDownloadRecord, PlatformError> {
    match asset.role {
        NgsAssetRole::SraArchive => materialize_direct_ngs_asset(
            client,
            plan,
            asset,
            existing_download_roots,
            progress_callback,
            transport_config,
        ),
        NgsAssetRole::GeneratedFastq if asset.source_url.starts_with("sra-convert://") => {
            execute_sra_fastq_conversion_with_runner(plan, asset, runner)
        }
        _ => Ok(NgsDownloadRecord::new(
            asset.clone(),
            local_ngs_asset_path(&plan.output_root, asset),
        )
        .with_verification_status(NgsVerificationStatus::Failed)
        .with_failure_reason(
            "SRA materialization only supports archives and generated FASTQ conversion plans",
        )),
    }
}

fn execute_sra_fastq_conversion_with_runner(
    plan: &NgsDownloadPlan,
    asset: &NgsAsset,
    runner: &impl SraFastqRunner,
) -> Result<NgsDownloadRecord, PlatformError> {
    let request = sra_fastq_conversion_request(plan, asset);
    let result = runner.run(&request)?;
    let mut record = NgsDownloadRecord::new(asset.clone(), request.fastq_dir.clone())
        .with_verification_status(NgsVerificationStatus::Planned)
        .with_materialization_method("sra_toolkit_conversion")
        .with_command_line(request.command_line.clone())
        .with_container_image(request.container_image.clone())
        .with_tool_version(request.tool_version.clone())
        .with_exit_status(result.exit_status);

    if result.exit_status != 0 {
        return Ok(record
            .with_verification_status(NgsVerificationStatus::Failed)
            .with_failure_reason(format!(
                "SRA FASTQ conversion command exited with status {}",
                result.exit_status
            )));
    }

    let generated_paths = discover_sra_fastq_outputs(&request.fastq_dir)?;
    if generated_paths.is_empty() {
        return Ok(record
            .with_verification_status(NgsVerificationStatus::Failed)
            .with_failure_reason("SRA FASTQ conversion did not produce FASTQ outputs"));
    }

    let observed_size = sum_file_sizes(&generated_paths)?;
    record = record
        .with_observed_evidence(Some(observed_size), None)
        .with_generated_paths(generated_paths)
        .with_verification_status(NgsVerificationStatus::Unverified);
    Ok(record)
}

fn sra_fastq_conversion_request(
    plan: &NgsDownloadPlan,
    asset: &NgsAsset,
) -> SraFastqConversionRequest {
    let fastq_dir = plan
        .output_root
        .join("runs")
        .join(sanitize_path_component(&asset.run_accession))
        .join("fastq");
    let run_accession = sanitize_path_component(&asset.run_accession);
    let sra_archive_path = plan
        .output_root
        .join("runs")
        .join(&run_accession)
        .join("sra")
        .join(format!("{run_accession}.sra"));
    let command_line = format!(
        "docker run --rm -v {} {} sh -lc '{}'",
        shell_quote_volume(&plan.output_root),
        DEFAULT_SRA_TOOLKIT_CONTAINER,
        sra_fastq_container_script(&run_accession)
    );

    SraFastqConversionRequest {
        run_accession,
        output_root: plan.output_root.clone(),
        sra_archive_path,
        fastq_dir,
        command_line,
        container_image: DEFAULT_SRA_TOOLKIT_CONTAINER.to_owned(),
        tool_version: DEFAULT_SRA_TOOLKIT_VERSION.to_owned(),
    }
}

fn sra_fastq_container_script(run_accession: &str) -> String {
    format!(
        "mkdir -p /work/runs/{run_accession}/sra /work/runs/{run_accession}/fastq && archive=\"$(find /work/runs/{run_accession}/sra -name \"{run_accession}.sra\" -type f | head -n 1)\" && if [ -z \"$archive\" ]; then prefetch {run_accession} --output-directory /work/runs/{run_accession}/sra; archive=\"$(find /work/runs/{run_accession}/sra -name \"{run_accession}.sra\" -type f | head -n 1)\"; fi && test -n \"$archive\" && fasterq-dump \"$archive\" --outdir /work/runs/{run_accession}/fastq"
    )
}

fn discover_sra_fastq_outputs(fastq_dir: &Path) -> Result<Vec<PathBuf>, PlatformError> {
    if !fastq_dir.exists() {
        return Ok(Vec::new());
    }

    let mut paths = fs::read_dir(fastq_dir)
        .map_err(|error| ngs_io_error("read SRA FASTQ output directory", error))?
        .filter_map(|entry| entry.ok().map(|entry| entry.path()))
        .filter(|path| {
            path.is_file()
                && path
                    .file_name()
                    .and_then(|value| value.to_str())
                    .is_some_and(is_fastq_filename)
        })
        .collect::<Vec<_>>();
    paths.sort();
    Ok(paths)
}

fn is_fastq_filename(file_name: &str) -> bool {
    let lower = file_name.to_ascii_lowercase();
    lower.ends_with(".fastq")
        || lower.ends_with(".fq")
        || lower.ends_with(".fastq.gz")
        || lower.ends_with(".fq.gz")
}

fn sum_file_sizes(paths: &[PathBuf]) -> Result<u64, PlatformError> {
    let mut total = 0u64;
    for path in paths {
        let size = fs::metadata(path)
            .map_err(|error| ngs_io_error("read generated FASTQ metadata", error))?
            .len();
        total = total.checked_add(size).ok_or_else(|| {
            PlatformError::new(
                ErrorCategory::Invocation,
                "generated FASTQ file sizes overflowed u64",
            )
            .with_code("service.ngs_retrieval.size_overflow")
        })?;
    }
    Ok(total)
}

fn ngs_provenance_json(provenance: &NgsProvenance) -> Value {
    let selected_assets = &provenance.download_plan.selected_assets;
    json!({
        "schema": provenance.schema.as_str(),
        "epithema_version": env!("CARGO_PKG_VERSION"),
        "acquisition_timestamp_unix_seconds": provenance.acquisition_timestamp_unix_seconds,
        "query": {
            "accession": provenance.manifest.query.accession.as_str(),
            "provider": provenance.manifest.query.provider.as_ref().map(ProviderId::as_str),
            "object_class": provenance.manifest.query.object_class.map(|object_class| object_class.as_str()),
        },
        "provider": provenance.manifest.provider.as_str(),
        "route": {
            "provider": provenance.manifest.route.provider.as_str(),
            "endpoint": provenance.manifest.route.endpoint.as_str(),
            "format": provenance.manifest.route.format.as_str(),
        },
        "manifest_lookup": artifact_provenance_json(&provenance.manifest.provenance),
        "selection": {
            "output_root": provenance.download_plan.output_root.display().to_string(),
            "include_raw": provenance.download_plan.include_raw,
            "selected_asset_count": provenance.download_plan.selected_assets.len(),
            "considered_asset_count": provenance.manifest.assets().len(),
        },
        "runs": provenance
            .manifest
            .runs
            .iter()
            .map(|run| manifest_run_json(run, selected_assets))
            .collect::<Vec<_>>(),
        "download_records": provenance
            .download_records
            .iter()
            .map(download_record_json)
            .collect::<Vec<_>>(),
        "local_files": provenance
            .download_records
            .iter()
            .flat_map(local_file_records_json)
            .collect::<Vec<_>>(),
    })
}

fn manifest_run_json(run: &NgsManifestRun, selected_assets: &[NgsAsset]) -> Value {
    json!({
        "metadata": run_metadata_json(&run.metadata),
        "assets": run.assets
            .iter()
            .map(|asset| {
                let selection_status = if selected_assets.contains(asset) {
                    "selected"
                } else {
                    "skipped"
                };
                asset_json(asset, selection_status)
            })
            .collect::<Vec<_>>(),
    })
}

fn run_metadata_json(metadata: &NgsRunMetadata) -> Value {
    json!({
        "run_accession": metadata.run_accession.as_str(),
        "experiment_accession": metadata.experiment_accession.as_deref(),
        "sample_accession": metadata.sample_accession.as_deref(),
        "study_accession": metadata.study_accession.as_deref(),
        "study_title": metadata.study_title.as_deref(),
        "sample_title": metadata.sample_title.as_deref(),
        "experiment_title": metadata.experiment_title.as_deref(),
        "scientific_name": metadata.scientific_name.as_deref(),
        "instrument_platform": metadata.instrument_platform.as_deref(),
        "instrument_model": metadata.instrument_model.as_deref(),
        "library_strategy": metadata.library_strategy.as_deref(),
        "library_source": metadata.library_source.as_deref(),
        "library_selection": metadata.library_selection.as_deref(),
        "library_layout": metadata.library_layout.as_deref(),
    })
}

fn asset_json(asset: &NgsAsset, selection_status: &str) -> Value {
    json!({
        "run_accession": asset.run_accession.as_str(),
        "asset_role": asset.role.as_str(),
        "asset_format": asset.format.as_str(),
        "source_url": asset.source_url.as_str(),
        "expected_size_bytes": asset.size_bytes,
        "expected_checksum_md5": asset.checksum_md5.as_deref(),
        "selection_status": selection_status,
    })
}

fn download_record_json(record: &NgsDownloadRecord) -> Value {
    json!({
        "asset": asset_json(&record.asset, "selected"),
        "local_path": record.local_path.display().to_string(),
        "generated_paths": record
            .generated_paths
            .iter()
            .map(|path| path.display().to_string())
            .collect::<Vec<_>>(),
        "observed_size_bytes": record.observed_size_bytes,
        "observed_checksum_md5": record.observed_checksum_md5.as_deref(),
        "verification_status": record.verification_status.as_str(),
        "failure_reason": record.failure_reason.as_deref(),
        "materialization_method": record.materialization_method.as_deref(),
        "command_line": record.command_line.as_deref(),
        "exit_status": record.exit_status,
        "container_image": record.container_image.as_deref(),
        "tool_version": record.tool_version.as_deref(),
    })
}

fn local_file_records_json(record: &NgsDownloadRecord) -> Vec<Value> {
    let mut files = vec![json!({
        "path": record.local_path.display().to_string(),
        "kind": "primary",
        "run_accession": record.asset.run_accession.as_str(),
        "asset_role": record.asset.role.as_str(),
        "asset_format": record.asset.format.as_str(),
        "verification_status": record.verification_status.as_str(),
        "observed_size_bytes": record.observed_size_bytes,
        "observed_checksum_md5": record.observed_checksum_md5.as_deref(),
    })];
    files.extend(record.generated_paths.iter().map(|path| {
        json!({
            "path": path.display().to_string(),
            "kind": "generated",
            "run_accession": record.asset.run_accession.as_str(),
            "asset_role": record.asset.role.as_str(),
            "asset_format": record.asset.format.as_str(),
            "verification_status": record.verification_status.as_str(),
        })
    }));
    files
}

const NGS_HANDOFF_MANIFEST_COLUMNS: &[&str] = &[
    "provider",
    "query_accession",
    "query_object_class",
    "study_accession",
    "study_title",
    "sample_accession",
    "sample_title",
    "experiment_accession",
    "experiment_title",
    "run_accession",
    "instrument_platform",
    "instrument_model",
    "library_strategy",
    "library_source",
    "library_selection",
    "library_layout",
    "asset_role",
    "asset_format",
    "source_url",
    "expected_size_bytes",
    "expected_checksum_md5",
    "selection_status",
    "local_path",
    "generated_paths",
    "observed_size_bytes",
    "observed_checksum_md5",
    "verification_status",
    "materialization_method",
    "failure_reason",
];

fn ngs_handoff_manifest_tsv(provenance: &NgsProvenance) -> String {
    let mut lines = vec![NGS_HANDOFF_MANIFEST_COLUMNS.join("\t")];
    for run in &provenance.manifest.runs {
        for asset in &run.assets {
            let selected = provenance
                .download_plan
                .selected_assets
                .iter()
                .any(|selected_asset| selected_asset == asset);
            let record = provenance
                .download_records
                .iter()
                .find(|record| record.asset == *asset);
            let row = ngs_handoff_manifest_row(provenance, &run.metadata, asset, selected, record);
            lines.push(row.join("\t"));
        }
    }
    lines.push(String::new());
    lines.join("\n")
}

fn ngs_handoff_manifest_row(
    provenance: &NgsProvenance,
    metadata: &NgsRunMetadata,
    asset: &NgsAsset,
    selected: bool,
    record: Option<&NgsDownloadRecord>,
) -> Vec<String> {
    let generated_paths = serde_json::to_string(
        &record
            .map(|record| {
                record
                    .generated_paths
                    .iter()
                    .map(|path| path.display().to_string())
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default(),
    )
    .unwrap_or_else(|_| "[]".to_owned());
    vec![
        tsv_cell(provenance.manifest.provider.as_str()),
        tsv_cell(&provenance.manifest.query.accession),
        tsv_cell(
            provenance
                .manifest
                .query
                .object_class
                .map(|object_class| object_class.as_str())
                .unwrap_or_default(),
        ),
        tsv_optional(metadata.study_accession.as_deref()),
        tsv_optional(metadata.study_title.as_deref()),
        tsv_optional(metadata.sample_accession.as_deref()),
        tsv_optional(metadata.sample_title.as_deref()),
        tsv_optional(metadata.experiment_accession.as_deref()),
        tsv_optional(metadata.experiment_title.as_deref()),
        tsv_cell(&metadata.run_accession),
        tsv_optional(metadata.instrument_platform.as_deref()),
        tsv_optional(metadata.instrument_model.as_deref()),
        tsv_optional(metadata.library_strategy.as_deref()),
        tsv_optional(metadata.library_source.as_deref()),
        tsv_optional(metadata.library_selection.as_deref()),
        tsv_optional(metadata.library_layout.as_deref()),
        tsv_cell(asset.role.as_str()),
        tsv_cell(&asset.format),
        tsv_cell(&asset.source_url),
        tsv_optional_u64(asset.size_bytes),
        tsv_optional(asset.checksum_md5.as_deref()),
        tsv_cell(if selected { "selected" } else { "skipped" }),
        tsv_cell(
            record
                .map(|record| record.local_path.display().to_string())
                .as_deref()
                .unwrap_or_default(),
        ),
        tsv_cell(&generated_paths),
        tsv_optional_u64(record.and_then(|record| record.observed_size_bytes)),
        tsv_optional(record.and_then(|record| record.observed_checksum_md5.as_deref())),
        tsv_cell(
            record
                .map(|record| record.verification_status.as_str())
                .unwrap_or("not_materialized"),
        ),
        tsv_optional(record.and_then(|record| record.materialization_method.as_deref())),
        tsv_optional(record.and_then(|record| record.failure_reason.as_deref())),
    ]
}

fn tsv_optional(value: Option<&str>) -> String {
    value.map(tsv_cell).unwrap_or_default()
}

fn tsv_optional_u64(value: Option<u64>) -> String {
    value.map(|value| value.to_string()).unwrap_or_default()
}

fn tsv_cell(value: &str) -> String {
    value
        .replace('\t', " ")
        .replace(['\r', '\n'], " ")
        .trim()
        .to_owned()
}

fn artifact_provenance_json(provenance: &ArtifactProvenance) -> Value {
    json!({
        "origin_kind": artifact_origin_kind_label(provenance.origin_kind),
        "locator": provenance.locator(),
        "provider": provenance.provider(),
        "description": provenance.description(),
    })
}

fn artifact_origin_kind_label(origin_kind: ArtifactOriginKind) -> &'static str {
    match origin_kind {
        ArtifactOriginKind::LocalFile => "local_file",
        ArtifactOriginKind::LegacyEmbossAsset => "legacy_emboss_asset",
        ArtifactOriginKind::Accession => "accession",
        ArtifactOriginKind::ProviderAsset => "provider_asset",
        ArtifactOriginKind::GeneratedFixture => "generated_fixture",
        ArtifactOriginKind::GeneratedOutput => "generated_output",
        ArtifactOriginKind::Unknown => "unknown",
    }
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

fn should_use_aspera_transport(
    source_url: &str,
    transport_config: &NgsDownloadTransportConfig,
) -> bool {
    if aspera_download_source(source_url).is_none() {
        return false;
    }
    match transport_config.mode {
        NgsDownloadTransport::Https => false,
        NgsDownloadTransport::Aspera => true,
        NgsDownloadTransport::Auto => {
            transport_config
                .aspera
                .key_path
                .as_ref()
                .is_some_and(|path| path.exists())
                && command_path_is_available(&transport_config.aspera.ascp_path)
        }
    }
}

fn aspera_download_source(source_url: &str) -> Option<AsperaDownloadSource> {
    let path = source_url
        .strip_prefix("ftp://ftp.sra.ebi.ac.uk/")
        .or_else(|| source_url.strip_prefix("https://ftp.sra.ebi.ac.uk/"))
        .or_else(|| source_url.strip_prefix("http://ftp.sra.ebi.ac.uk/"));
    if let Some(path) = path {
        return Some(AsperaDownloadSource {
            user: "era-fasp",
            host: "fasp.sra.ebi.ac.uk",
            remote_path: path.trim_start_matches('/').to_owned(),
        });
    }

    let ena_path = source_url
        .strip_prefix("ftp://ftp.ebi.ac.uk/pub/")
        .or_else(|| source_url.strip_prefix("https://ftp.ebi.ac.uk/pub/"))
        .or_else(|| source_url.strip_prefix("http://ftp.ebi.ac.uk/pub/"));
    ena_path.map(|path| AsperaDownloadSource {
        user: "fasp-ebi",
        host: "fasp.ebi.ac.uk",
        remote_path: path.trim_start_matches('/').to_owned(),
    })
}

fn emit_ngs_download_progress(
    progress_callback: Option<&NgsDownloadProgressCallback>,
    state: HttpDownloadProgressState,
    url: &str,
    path: &Path,
    bytes_downloaded: u64,
    total_bytes: Option<u64>,
) {
    if let Some(progress_callback) = progress_callback {
        progress_callback(HttpDownloadProgress {
            state,
            url: url.to_owned(),
            path: path.to_path_buf(),
            bytes_downloaded,
            total_bytes,
        });
    }
}

fn aspera_command_log_path(partial_path: &Path) -> PathBuf {
    partial_path.with_extension("ascp.log")
}

fn aspera_command_log_excerpt(path: &Path) -> String {
    let Ok(contents) = fs::read_to_string(path) else {
        return "unavailable".to_owned();
    };
    let trimmed = contents.trim();
    if trimmed.is_empty() {
        return "empty".to_owned();
    }
    let mut excerpt = trimmed
        .chars()
        .rev()
        .take(2_000)
        .collect::<String>()
        .chars()
        .rev()
        .collect::<String>();
    if excerpt.len() < trimmed.len() {
        excerpt.insert_str(0, "...");
    }
    excerpt
}

fn default_aspera_key_path() -> Option<PathBuf> {
    env::var_os("EPITHEMA_ASPERA_KEY")
        .map(PathBuf::from)
        .filter(|path| path.exists())
        .or_else(|| {
            env::var_os("CONDA_PREFIX")
                .map(PathBuf::from)
                .and_then(|prefix| first_existing_aspera_key(&prefix))
        })
        .or_else(|| {
            env::var_os("HOME").map(PathBuf::from).and_then(|home| {
                first_existing_path([
                    home.join(".aspera/connect/etc/asperaweb_id_dsa.openssh"),
                    home.join(".aspera/connect/etc/aspera_tokenauth_id_dsa"),
                    home.join(".aspera/connect/etc/aspera_tokenauth_id_rsa"),
                ])
            })
        })
}

fn first_existing_aspera_key(prefix: &Path) -> Option<PathBuf> {
    first_existing_path([
        prefix.join("etc/asperaweb_id_dsa.openssh"),
        prefix.join("etc/aspera_tokenauth_id_dsa"),
        prefix.join("etc/aspera_tokenauth_id_rsa"),
    ])
}

fn first_existing_path(paths: impl IntoIterator<Item = PathBuf>) -> Option<PathBuf> {
    paths.into_iter().find(|path| path.exists())
}

fn command_path_is_available(command: &Path) -> bool {
    if command.components().count() > 1 {
        return command.exists();
    }
    let Some(path_var) = env::var_os("PATH") else {
        return false;
    };
    env::split_paths(&path_var).any(|path| path.join(command).exists())
}

fn aspera_command_line(
    config: &NgsAsperaConfig,
    key_path: &Path,
    remote: &str,
    partial_path: &Path,
) -> String {
    [
        display_command_arg(&config.ascp_path),
        "-T".to_owned(),
        "-k1".to_owned(),
        "-l".to_owned(),
        display_shell_arg(&config.target_rate),
        "-P".to_owned(),
        "33001".to_owned(),
        "-i".to_owned(),
        display_command_arg(key_path),
        remote.to_owned(),
        display_command_arg(partial_path),
    ]
    .join(" ")
}

fn display_command_arg(path: &Path) -> String {
    display_shell_arg(&path.display().to_string())
}

fn display_shell_arg(raw: &str) -> String {
    if raw.chars().any(char::is_whitespace) {
        format!("'{}'", raw.replace('\'', "'\\''"))
    } else {
        raw.to_owned()
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
    if asset.source_url.starts_with("sra-convert://") && asset.role == NgsAssetRole::GeneratedFastq
    {
        return format!("{}.fastq", sanitize_path_component(&asset.run_accession));
    }

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

fn shell_quote_volume(output_root: &Path) -> String {
    let raw = format!("{}:/work", output_root.display());
    format!("'{}'", raw.replace('\'', "'\\''"))
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

fn materialize_existing_download_candidate(
    asset: &NgsAsset,
    local_path: &Path,
    existing_download_roots: &[PathBuf],
) -> Result<Option<NgsDownloadRecord>, PlatformError> {
    if existing_download_roots.is_empty() {
        return Ok(None);
    }

    let Some(file_name) = local_path.file_name() else {
        return Ok(None);
    };

    for root in existing_download_roots {
        if !root.exists() {
            return Err(PlatformError::new(
                ErrorCategory::Validation,
                "existing NGS download search root does not exist",
            )
            .with_code("service.ngs_retrieval.existing_download_root_missing")
            .with_detail(root.display().to_string()));
        }

        for candidate in existing_download_candidates(root, file_name)? {
            let (observed_size, observed_checksum) = ngs_file_evidence(&candidate)?;
            let observed_checksum_option = Some(observed_checksum.clone());
            if let Some(reason) =
                ngs_verification_failure(asset, Some(observed_size), &observed_checksum_option)
            {
                return Ok(Some(
                    NgsDownloadRecord::new(asset.clone(), local_path)
                        .with_observed_evidence(Some(observed_size), Some(observed_checksum))
                        .with_verification_status(NgsVerificationStatus::Failed)
                        .with_materialization_method("existing_download_copy")
                        .with_failure_reason(format!(
                            "existing download candidate {} failed verification: {reason}",
                            candidate.display()
                        )),
                ));
            }

            return copy_existing_download_candidate(
                asset,
                &candidate,
                local_path,
                observed_size,
                observed_checksum,
            )
            .map(Some);
        }
    }

    Ok(None)
}

fn existing_download_candidates(
    root: &Path,
    file_name: &std::ffi::OsStr,
) -> Result<Vec<PathBuf>, PlatformError> {
    let mut candidates = Vec::new();
    let mut stack = vec![root.to_path_buf()];

    while let Some(directory) = stack.pop() {
        for entry in fs::read_dir(&directory)
            .map_err(|error| ngs_io_error("read existing NGS download search directory", error))?
        {
            let entry = entry.map_err(|error| {
                ngs_io_error("read existing NGS download directory entry", error)
            })?;
            let file_type = entry
                .file_type()
                .map_err(|error| ngs_io_error("read existing NGS download file type", error))?;
            if file_type.is_dir() {
                stack.push(entry.path());
            } else if file_type.is_file() && entry.file_name() == file_name {
                candidates.push(entry.path());
            }
        }
    }

    candidates.sort();
    Ok(candidates)
}

fn copy_existing_download_candidate(
    asset: &NgsAsset,
    candidate: &Path,
    local_path: &Path,
    observed_size: u64,
    observed_checksum: String,
) -> Result<NgsDownloadRecord, PlatformError> {
    let partial_path = partial_ngs_asset_path(local_path);
    if let Some(parent) = partial_path.parent() {
        fs::create_dir_all(parent)
            .map_err(|error| ngs_io_error("create output directory", error))?;
    }

    fs::copy(candidate, &partial_path)
        .map_err(|error| ngs_io_error("copy existing NGS download candidate", error))?;
    let (copied_size, copied_checksum) = ngs_file_evidence(&partial_path)?;
    let copied_checksum_option = Some(copied_checksum.clone());
    let mut record = NgsDownloadRecord::new(asset.clone(), local_path)
        .with_observed_evidence(Some(copied_size), Some(copied_checksum.clone()))
        .with_materialization_method("existing_download_copy");

    if let Some(reason) =
        ngs_verification_failure(asset, Some(copied_size), &copied_checksum_option)
    {
        return Ok(record
            .with_verification_status(NgsVerificationStatus::Failed)
            .with_failure_reason(format!(
                "copied existing download candidate {} failed verification: {reason}",
                candidate.display()
            )));
    }

    if observed_size != copied_size || !observed_checksum.eq_ignore_ascii_case(&copied_checksum) {
        return Ok(record
            .with_verification_status(NgsVerificationStatus::Failed)
            .with_failure_reason(format!(
                "copied existing download candidate {} changed during copy",
                candidate.display()
            )));
    }

    if local_path.exists() {
        fs::remove_file(local_path)
            .map_err(|error| ngs_io_error("replace existing NGS download", error))?;
    }
    fs::rename(&partial_path, local_path)
        .map_err(|error| ngs_io_error("promote copied existing NGS download", error))?;

    let status = if asset.size_bytes.is_some() || asset.checksum_md5.is_some() {
        NgsVerificationStatus::Verified
    } else {
        NgsVerificationStatus::Unverified
    };
    record = record.with_verification_status(status);
    Ok(record)
}

fn verified_existing_asset_evidence(
    local_path: &Path,
    asset: &NgsAsset,
) -> Result<Option<(u64, String)>, PlatformError> {
    if !local_path.exists() || (asset.size_bytes.is_none() && asset.checksum_md5.is_none()) {
        return Ok(None);
    }

    let (observed_size, observed_checksum) = ngs_file_evidence(local_path)?;
    let observed_checksum_option = Some(observed_checksum.clone());
    if ngs_verification_failure(asset, Some(observed_size), &observed_checksum_option).is_none() {
        Ok(Some((observed_size, observed_checksum)))
    } else {
        Ok(None)
    }
}

fn resumable_partial_size(
    partial_path: &Path,
    asset: &NgsAsset,
) -> Result<Option<u64>, PlatformError> {
    if !partial_path.exists() {
        return Ok(None);
    }
    let size = fs::metadata(partial_path)
        .map_err(|error| ngs_io_error("inspect partial NGS download", error))?
        .len();
    if size == 0 {
        return Ok(None);
    }
    if let Some(expected_size) = asset.size_bytes {
        if size >= expected_size {
            fs::remove_file(partial_path).map_err(|error| {
                ngs_io_error("discard invalid complete partial NGS download", error)
            })?;
            return Ok(None);
        }
    }
    Ok(Some(size))
}

fn ngs_file_evidence(path: &Path) -> Result<(u64, String), PlatformError> {
    let mut file = fs::File::open(path)
        .map_err(|error| ngs_io_error("open NGS file for verification", error))?;
    let mut context = md5::Context::new();
    let mut total_size = 0u64;
    let mut buffer = [0u8; 64 * 1024];

    loop {
        let bytes_read = file
            .read(&mut buffer)
            .map_err(|error| ngs_io_error("read NGS file for verification", error))?;
        if bytes_read == 0 {
            break;
        }
        total_size = total_size
            .checked_add(u64::try_from(bytes_read).map_err(|error| {
                PlatformError::new(
                    ErrorCategory::Invocation,
                    "NGS file size could not be represented as u64",
                )
                .with_code("service.ngs_retrieval.size_overflow")
                .with_detail(error.to_string())
            })?)
            .ok_or_else(|| {
                PlatformError::new(ErrorCategory::Invocation, "NGS file sizes overflowed u64")
                    .with_code("service.ngs_retrieval.size_overflow")
            })?;
        context.consume(&buffer[..bytes_read]);
    }

    Ok((total_size, format!("{:x}", context.compute())))
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
    use std::path::{Path, PathBuf};
    use std::sync::{Arc, Mutex};
    use std::time::{SystemTime, UNIX_EPOCH};

    use epithema_config::{PlatformConfig, ProviderSettings};
    use epithema_diagnostics::PlatformError;
    use epithema_providers::{
        ArchiveRoute, EnaNgsAdapter, HttpBytesResponse, HttpDownloadProgressState, HttpRequest,
        HttpResponse, NgsAsset, NgsAssetRole, NgsDownloadPlan, NgsManifest, NgsManifestRun,
        NgsProvenance, NgsQuery, NgsRunMetadata, NgsVerificationStatus, ProviderCapability,
        ProviderDescriptor, ProviderHttpClient, ProviderId, ProviderRegistry, SraNgsAdapter,
    };

    use super::{
        DEFAULT_SRA_TOOLKIT_CONTAINER, NgsAsperaConfig, NgsDownloadProgressCallback,
        ServiceNgsRetrieval, SraFastqConversionRequest, SraFastqConversionResult, SraFastqRunner,
        aspera_command_line, aspera_download_source, partial_ngs_asset_path,
    };

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

    #[derive(Clone, Debug)]
    struct FakeSraFastqRunner {
        exit_status: i32,
        outputs: Vec<(&'static str, Vec<u8>)>,
    }

    impl FakeSraFastqRunner {
        fn successful_paired() -> Self {
            Self {
                exit_status: 0,
                outputs: vec![
                    ("SRR123456_1.fastq", b"@r1\nACGT\n+\n!!!!\n".to_vec()),
                    ("SRR123456_2.fastq", b"@r2\nTGCA\n+\n!!!!\n".to_vec()),
                ],
            }
        }

        fn failing() -> Self {
            Self {
                exit_status: 2,
                outputs: Vec::new(),
            }
        }

        fn interrupted() -> Self {
            Self {
                exit_status: -1,
                outputs: Vec::new(),
            }
        }
    }

    impl SraFastqRunner for FakeSraFastqRunner {
        fn run(
            &self,
            request: &SraFastqConversionRequest,
        ) -> Result<SraFastqConversionResult, PlatformError> {
            fs::create_dir_all(&request.fastq_dir)
                .expect("fake SRA runner should create FASTQ directory");
            for (file_name, body) in &self.outputs {
                fs::write(request.fastq_dir.join(file_name), body)
                    .expect("fake SRA runner should write FASTQ output");
            }
            Ok(SraFastqConversionResult {
                exit_status: self.exit_status,
            })
        }
    }

    #[test]
    fn derives_ena_aspera_source_from_public_read_ftp_url() {
        let source = aspera_download_source(
            "ftp://ftp.sra.ebi.ac.uk/vol1/fastq/ERR123/ERR123456/ERR123456_1.fastq.gz",
        )
        .expect("ENA read FTP URL should map to an Aspera source");

        assert_eq!(source.user, "era-fasp");
        assert_eq!(source.host, "fasp.sra.ebi.ac.uk");
        assert_eq!(
            source.remote_path,
            "vol1/fastq/ERR123/ERR123456/ERR123456_1.fastq.gz"
        );
    }

    #[test]
    fn derives_ena_aspera_source_from_public_ena_ftp_url() {
        let source = aspera_download_source(
            "ftp://ftp.ebi.ac.uk/pub/databases/ena/wgs/public/wya/WYAA01.dat.gz",
        )
        .expect("ENA public FTP URL should map to an Aspera source");

        assert_eq!(source.user, "fasp-ebi");
        assert_eq!(source.host, "fasp.ebi.ac.uk");
        assert_eq!(
            source.remote_path,
            "databases/ena/wgs/public/wya/WYAA01.dat.gz"
        );
    }

    #[test]
    fn builds_aspera_command_line_with_rate_key_and_destination() {
        let config = NgsAsperaConfig {
            ascp_path: PathBuf::from("/opt/aspera/bin/ascp"),
            key_path: Some(PathBuf::from("/opt/aspera/etc/asperaweb_id_dsa.openssh")),
            target_rate: "1g".to_owned(),
        };

        let command_line = aspera_command_line(
            &config,
            config.key_path.as_deref().expect("key path should exist"),
            "era-fasp@fasp.sra.ebi.ac.uk:vol1/fastq/ERR123/file.fastq.gz",
            Path::new("/tmp/file.fastq.gz.partial"),
        );

        assert!(command_line.contains("/opt/aspera/bin/ascp -T -k1 -l 1g -P 33001 -i"));
        assert!(command_line.contains("/opt/aspera/etc/asperaweb_id_dsa.openssh"));
        assert!(
            command_line.contains("era-fasp@fasp.sra.ebi.ac.uk:vol1/fastq/ERR123/file.fastq.gz")
        );
        assert!(command_line.contains("/tmp/file.fastq.gz.partial"));
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
        let assets = vec![
            NgsAsset::new(
                "SRR123456",
                NgsAssetRole::SraArchive,
                "sra",
                "https://example.invalid/SRR123456.sra",
            ),
            NgsAsset::new(
                "SRR123456",
                NgsAssetRole::GeneratedFastq,
                "fastq",
                "sra-convert://SRR123456/fastq",
            ),
        ];
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
    fn rejects_ngs_manifest_for_unclassified_query() {
        let config = PlatformConfig::default();
        let registry = ProviderRegistry::builtin_defaults();
        let gateway =
            ServiceNgsRetrieval::with_client(&config, &registry, MockHttpClient::default());
        let query = NgsQuery::new("ERR123456");

        let error = gateway
            .retrieve_manifest(&query)
            .expect_err("unclassified query should fail before provider routing");

        assert_eq!(
            error.code(),
            Some("service.ngs_retrieval.unclassified_query")
        );
    }

    #[test]
    fn rejects_ngs_manifest_for_registered_but_unsupported_provider_route() {
        let provider = ProviderId::new("custom").expect("valid provider");
        let mut registry = ProviderRegistry::builtin_defaults();
        registry
            .register(ProviderDescriptor::new(
                provider.clone(),
                "custom archive provider",
                [ProviderCapability::ArchiveAcquisition],
            ))
            .expect("custom provider should register");
        let config =
            PlatformConfig::default().with_provider(ProviderSettings::enabled(provider.clone()));
        let gateway =
            ServiceNgsRetrieval::with_client(&config, &registry, MockHttpClient::default());
        let query = NgsQuery::classify("ena:ERR123456")
            .expect("query should classify")
            .with_provider(provider);

        let error = gateway
            .retrieve_manifest(&query)
            .expect_err("unsupported provider route should fail");

        assert_eq!(
            error.code(),
            Some("service.ngs_retrieval.unsupported_provider")
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
    fn reports_progress_for_direct_ena_downloads() {
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
        let output_root = temp_ngs_output_root("download-progress");
        let plan = NgsDownloadPlan::new(manifest, output_root.clone(), false, vec![asset]);
        let client = MockHttpClient::default().with_byte_response(
            "https://example.invalid/ERR123456.fastq.gz",
            HttpBytesResponse::new(200, body),
        );
        let events = Arc::new(Mutex::new(Vec::new()));
        let progress_callback: Box<NgsDownloadProgressCallback> = {
            let events = Arc::clone(&events);
            Box::new(move |event| {
                events
                    .lock()
                    .expect("progress events should be lockable")
                    .push(event);
            })
        };
        let gateway = ServiceNgsRetrieval::with_client_and_progress(
            &config,
            &registry,
            client,
            Some(progress_callback.as_ref()),
        );

        let records = gateway
            .materialize_download_plan(&plan)
            .expect("direct ENA download should materialize");

        assert_eq!(
            records[0].verification_status,
            NgsVerificationStatus::Verified
        );
        let events = events.lock().expect("progress events should be lockable");
        assert_eq!(
            events.first().map(|event| event.state),
            Some(HttpDownloadProgressState::Started)
        );
        assert_eq!(
            events.last().map(|event| event.state),
            Some(HttpDownloadProgressState::Finished)
        );
        assert_eq!(events.last().map(|event| event.bytes_downloaded), Some(5));
        fs::remove_dir_all(output_root).ok();
    }

    #[test]
    fn copies_verified_existing_download_candidate_before_network_download() {
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
        let output_root = temp_ngs_output_root("download-cache-copy");
        let cache_root = temp_ngs_output_root("download-cache-source");
        let candidate_path = cache_root.join("nested/ERR123456.fastq.gz");
        fs::create_dir_all(
            candidate_path
                .parent()
                .expect("candidate should have parent"),
        )
        .expect("candidate parent should be created");
        fs::write(&candidate_path, &body).expect("candidate file should be written");
        let plan = NgsDownloadPlan::new(manifest, output_root.clone(), false, vec![asset]);
        let gateway =
            ServiceNgsRetrieval::with_client(&config, &registry, MockHttpClient::default());

        let records = gateway
            .materialize_download_plan_with_existing_downloads(&plan, &[cache_root.clone()])
            .expect("verified existing download should be copied");

        assert_eq!(records.len(), 1);
        assert_eq!(
            records[0].verification_status,
            NgsVerificationStatus::Verified
        );
        assert_eq!(
            records[0].materialization_method.as_deref(),
            Some("existing_download_copy")
        );
        assert_eq!(
            fs::read(&records[0].local_path).expect("copied file should be readable"),
            body
        );
        assert_eq!(
            fs::read(&candidate_path).expect("original candidate should remain intact"),
            b"ACGT\n".to_vec()
        );
        assert_eq!(
            records[0].observed_checksum_md5.as_deref(),
            Some(checksum.as_str())
        );
        fs::remove_dir_all(output_root).ok();
        fs::remove_dir_all(cache_root).ok();
    }

    #[test]
    fn fails_when_existing_download_candidate_has_unexpected_checksum() {
        let config = PlatformConfig::default();
        let registry = ProviderRegistry::builtin_defaults();
        let expected_body = b"ACGT\n".to_vec();
        let wrong_body = b"TGCA\n".to_vec();
        let expected_checksum = format!("{:x}", md5::compute(&expected_body));
        let wrong_checksum = format!("{:x}", md5::compute(&wrong_body));
        let asset = NgsAsset::new(
            "ERR123456",
            NgsAssetRole::GeneratedFastq,
            "fastq.gz",
            "ftp://example.invalid/ERR123456.fastq.gz",
        )
        .with_size_bytes(Some(5))
        .with_checksum_md5(Some(expected_checksum));
        let mut manifest = planned_manifest();
        manifest.runs[0].assets = vec![asset.clone()];
        let output_root = temp_ngs_output_root("download-cache-fail");
        let cache_root = temp_ngs_output_root("download-cache-mismatch");
        let candidate_path = cache_root.join("ERR123456.fastq.gz");
        fs::create_dir_all(&cache_root).expect("cache root should be created");
        fs::write(&candidate_path, &wrong_body).expect("candidate file should be written");
        let plan = NgsDownloadPlan::new(manifest, output_root.clone(), false, vec![asset]);
        let gateway =
            ServiceNgsRetrieval::with_client(&config, &registry, MockHttpClient::default());

        let records = gateway
            .materialize_download_plan_with_existing_downloads(&plan, &[cache_root.clone()])
            .expect("checksum mismatch should be captured as a failed record");

        assert_eq!(records.len(), 1);
        assert_eq!(
            records[0].verification_status,
            NgsVerificationStatus::Failed
        );
        assert_eq!(
            records[0].materialization_method.as_deref(),
            Some("existing_download_copy")
        );
        assert_eq!(
            records[0].observed_checksum_md5.as_deref(),
            Some(wrong_checksum.as_str())
        );
        assert!(
            records[0]
                .failure_reason
                .as_deref()
                .is_some_and(|reason| reason.contains("MD5 checksum mismatch"))
        );
        assert!(!records[0].local_path.exists());
        assert_eq!(
            fs::read(&candidate_path).expect("original candidate should remain intact"),
            wrong_body
        );
        fs::remove_dir_all(output_root).ok();
        fs::remove_dir_all(cache_root).ok();
    }

    #[test]
    fn records_missing_checksum_and_size_semantics_for_direct_downloads() {
        let config = PlatformConfig::default();
        let registry = ProviderRegistry::builtin_defaults();
        let body = b"ACGT\n".to_vec();
        let checksum = format!("{:x}", md5::compute(&body));
        let size_only = NgsAsset::new(
            "ERRSIZE",
            NgsAssetRole::GeneratedFastq,
            "fastq.gz",
            "ftp://example.invalid/ERRSIZE.fastq.gz",
        )
        .with_size_bytes(Some(5));
        let checksum_only = NgsAsset::new(
            "ERRCHECKSUM",
            NgsAssetRole::GeneratedFastq,
            "fastq.gz",
            "ftp://example.invalid/ERRCHECKSUM.fastq.gz",
        )
        .with_checksum_md5(Some(checksum.clone()));
        let no_evidence = NgsAsset::new(
            "ERRNOEVIDENCE",
            NgsAssetRole::GeneratedFastq,
            "fastq.gz",
            "ftp://example.invalid/ERRNOEVIDENCE.fastq.gz",
        );
        let provider = ProviderId::new("ena").expect("static provider id should be valid");
        let query = NgsQuery::classify("ena:ERRSIZE").expect("query should classify");
        let runs = vec![
            NgsManifestRun::new(NgsRunMetadata::new("ERRSIZE"), vec![size_only.clone()]),
            NgsManifestRun::new(
                NgsRunMetadata::new("ERRCHECKSUM"),
                vec![checksum_only.clone()],
            ),
            NgsManifestRun::new(
                NgsRunMetadata::new("ERRNOEVIDENCE"),
                vec![no_evidence.clone()],
            ),
        ];
        let manifest = NgsManifest::new(
            query,
            provider.clone(),
            ArchiveRoute::new(provider, "ena.portal.filereport.read_run", "tsv"),
            runs,
        );
        let output_root = temp_ngs_output_root("missing-evidence");
        let plan = NgsDownloadPlan::new(
            manifest,
            output_root.clone(),
            false,
            vec![size_only, checksum_only, no_evidence],
        );
        let client = MockHttpClient::default()
            .with_byte_response(
                "https://example.invalid/ERRSIZE.fastq.gz",
                HttpBytesResponse::new(200, body.clone()),
            )
            .with_byte_response(
                "https://example.invalid/ERRCHECKSUM.fastq.gz",
                HttpBytesResponse::new(200, body.clone()),
            )
            .with_byte_response(
                "https://example.invalid/ERRNOEVIDENCE.fastq.gz",
                HttpBytesResponse::new(200, body),
            );
        let gateway = ServiceNgsRetrieval::with_client(&config, &registry, client);

        let records = gateway
            .materialize_download_plan(&plan)
            .expect("direct downloads should materialize");

        assert_eq!(records.len(), 3);
        assert_eq!(
            records[0].verification_status,
            NgsVerificationStatus::Verified
        );
        assert_eq!(
            records[1].verification_status,
            NgsVerificationStatus::Verified
        );
        assert_eq!(
            records[2].verification_status,
            NgsVerificationStatus::Unverified
        );
        assert_eq!(records[2].observed_size_bytes, Some(5));
        assert!(records[2].observed_checksum_md5.is_some());
        assert!(records.iter().all(|record| record.local_path.exists()));
        fs::remove_dir_all(output_root).ok();
    }

    #[test]
    fn records_provider_404_without_creating_partial_download() {
        let config = PlatformConfig::default();
        let registry = ProviderRegistry::builtin_defaults();
        let asset = NgsAsset::new(
            "ERR404",
            NgsAssetRole::GeneratedFastq,
            "fastq.gz",
            "ftp://example.invalid/ERR404.fastq.gz",
        );
        let provider = ProviderId::new("ena").expect("static provider id should be valid");
        let query = NgsQuery::classify("ena:ERR404").expect("query should classify");
        let manifest = NgsManifest::new(
            query,
            provider.clone(),
            ArchiveRoute::new(provider, "ena.portal.filereport.read_run", "tsv"),
            vec![NgsManifestRun::new(
                NgsRunMetadata::new("ERR404"),
                vec![asset.clone()],
            )],
        );
        let output_root = temp_ngs_output_root("404");
        let plan = NgsDownloadPlan::new(manifest, output_root.clone(), false, vec![asset]);
        let client = MockHttpClient::default().with_byte_response(
            "https://example.invalid/ERR404.fastq.gz",
            HttpBytesResponse::new(404, Vec::new()),
        );
        let gateway = ServiceNgsRetrieval::with_client(&config, &registry, client);

        let records = gateway
            .materialize_download_plan(&plan)
            .expect("provider 404 should be captured as a failed record");

        assert_eq!(records.len(), 1);
        assert_eq!(
            records[0].verification_status,
            NgsVerificationStatus::Failed
        );
        assert_eq!(
            records[0].failure_reason.as_deref(),
            Some("download returned HTTP status 404")
        );
        assert!(!records[0].local_path.exists());
        assert!(!partial_ngs_asset_path(&records[0].local_path).exists());
        fs::remove_dir_all(output_root).ok();
    }

    #[test]
    fn writes_ngs_provenance_json_with_selected_skipped_and_records() {
        let config = PlatformConfig::default();
        let registry = ProviderRegistry::builtin_defaults();
        let body = b"ACGT\n".to_vec();
        let checksum = format!("{:x}", md5::compute(&body));
        let mut manifest = planned_manifest();
        manifest.runs[0].metadata.study_title = Some("Example study".to_owned());
        manifest.runs[0].assets[0] = manifest.runs[0].assets[0]
            .clone()
            .with_size_bytes(Some(5))
            .with_checksum_md5(Some(checksum.clone()));
        let output_root = temp_ngs_output_root("provenance");
        let client = MockHttpClient::default().with_byte_response(
            "https://example.invalid/ERR123456.fastq.gz",
            HttpBytesResponse::new(200, body),
        );
        let gateway = ServiceNgsRetrieval::with_client(&config, &registry, client);
        let plan = gateway
            .plan_downloads(&manifest, &output_root, false)
            .expect("NGS download plan should build");
        let records = gateway
            .materialize_download_plan(&plan)
            .expect("direct ENA asset should materialize");
        let provenance =
            NgsProvenance::new_at_unix_seconds(manifest.clone(), plan, records, 123_456);
        let path = output_root.join("provenance.json");

        gateway
            .write_provenance(&provenance, &path)
            .expect("provenance JSON should be written");

        let value: serde_json::Value =
            serde_json::from_slice(&fs::read(&path).expect("provenance JSON should be readable"))
                .expect("provenance JSON should parse");
        assert_eq!(value["schema"], "epithema.ngs-provenance/v1");
        assert_eq!(value["acquisition_timestamp_unix_seconds"], 123_456);
        assert_eq!(value["query"]["accession"], "ERR123456");
        assert_eq!(value["query"]["object_class"], "run");
        assert_eq!(value["provider"], "ena");
        assert_eq!(value["route"]["endpoint"], "ena.portal.filereport.read_run");
        assert_eq!(value["runs"][0]["metadata"]["study_title"], "Example study");
        assert_eq!(value["selection"]["selected_asset_count"], 1);
        assert_eq!(value["selection"]["considered_asset_count"], 6);
        assert_eq!(
            value["runs"][0]["assets"][0]["selection_status"],
            "selected"
        );
        assert_eq!(value["runs"][0]["assets"][1]["selection_status"], "skipped");
        assert_eq!(
            value["download_records"][0]["materialization_method"],
            "direct_download"
        );
        assert_eq!(
            value["download_records"][0]["verification_status"],
            "verified"
        );
        assert_eq!(
            value["download_records"][0]["observed_checksum_md5"],
            checksum
        );
        assert_eq!(value["local_files"][0]["kind"], "primary");
        assert_eq!(value["local_files"][0]["verification_status"], "verified");
        fs::remove_dir_all(output_root).ok();
    }

    #[test]
    fn writes_ngs_handoff_manifest_for_object_store_importers() {
        let config = PlatformConfig::default();
        let registry = ProviderRegistry::builtin_defaults();
        let body = b"ACGT\n".to_vec();
        let checksum = format!("{:x}", md5::compute(&body));
        let mut manifest = planned_manifest();
        manifest.runs[0].metadata = NgsRunMetadata::new("ERR123456").with_accessions(
            Some("PRJNA1011899".to_owned()),
            Some("SAMN1011899".to_owned()),
            Some("ERX123456".to_owned()),
        );
        manifest.runs[0].metadata.study_title = Some("Example study".to_owned());
        manifest.runs[0].metadata.sample_title = Some("Example sample".to_owned());
        manifest.runs[0].metadata.instrument_platform = Some("ILLUMINA".to_owned());
        manifest.runs[0].metadata.library_strategy = Some("WGS".to_owned());
        manifest.runs[0].assets[0] = manifest.runs[0].assets[0]
            .clone()
            .with_size_bytes(Some(5))
            .with_checksum_md5(Some(checksum.clone()));
        let output_root = temp_ngs_output_root("handoff-manifest");
        let client = MockHttpClient::default().with_byte_response(
            "https://example.invalid/ERR123456.fastq.gz",
            HttpBytesResponse::new(200, body),
        );
        let gateway = ServiceNgsRetrieval::with_client(&config, &registry, client);
        let plan = gateway
            .plan_downloads(&manifest, &output_root, false)
            .expect("NGS download plan should build");
        let records = gateway
            .materialize_download_plan(&plan)
            .expect("direct ENA asset should materialize");
        let provenance =
            NgsProvenance::new_at_unix_seconds(manifest.clone(), plan, records, 123_456);
        let path = output_root.join("manifest.tsv");

        gateway
            .write_manifest(&provenance, &path)
            .expect("handoff manifest TSV should be written");

        let body = fs::read_to_string(&path).expect("manifest TSV should be readable");
        let lines = body.lines().collect::<Vec<_>>();
        assert_eq!(lines.len(), 7);
        assert!(lines[0].starts_with(
            "provider\tquery_accession\tquery_object_class\tstudy_accession\tstudy_title"
        ));
        let selected = lines[1].split('\t').collect::<Vec<_>>();
        assert_eq!(selected[0], "ena");
        assert_eq!(selected[1], "ERR123456");
        assert_eq!(selected[2], "run");
        assert_eq!(selected[3], "PRJNA1011899");
        assert_eq!(selected[4], "Example study");
        assert_eq!(selected[5], "SAMN1011899");
        assert_eq!(selected[9], "ERR123456");
        assert_eq!(selected[16], "generated_fastq");
        assert_eq!(selected[21], "selected");
        assert!(selected[22].ends_with("runs/ERR123456/fastq/ERR123456.fastq.gz"));
        assert_eq!(selected[23], "[]");
        assert_eq!(selected[24], "5");
        assert_eq!(selected[25], checksum);
        assert_eq!(selected[26], "verified");
        assert_eq!(selected[27], "direct_download");
        let skipped = lines[2].split('\t').collect::<Vec<_>>();
        assert_eq!(skipped[16], "submitted_raw");
        assert_eq!(skipped[21], "skipped");
        assert_eq!(skipped[22], "");
        assert_eq!(skipped[26], "not_materialized");
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
        let body = include_bytes!("../tests/fixtures/ngs/checksum_mismatch.fastq").to_vec();
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
    fn resumes_partial_file_on_retry_before_promotion() {
        let config = PlatformConfig::default();
        let registry = ProviderRegistry::builtin_defaults();
        let body = b"ACGT\n".to_vec();
        let checksum = format!("{:x}", md5::compute(&body));
        let asset = NgsAsset::new(
            "ERRRETRY",
            NgsAssetRole::GeneratedFastq,
            "fastq.gz",
            "ftp://example.invalid/ERRRETRY.fastq.gz",
        )
        .with_size_bytes(Some(5))
        .with_checksum_md5(Some(checksum));
        let provider = ProviderId::new("ena").expect("static provider id should be valid");
        let query = NgsQuery::classify("ena:ERRRETRY").expect("query should classify");
        let manifest = NgsManifest::new(
            query,
            provider.clone(),
            ArchiveRoute::new(provider, "ena.portal.filereport.read_run", "tsv"),
            vec![NgsManifestRun::new(
                NgsRunMetadata::new("ERRRETRY"),
                vec![asset.clone()],
            )],
        );
        let output_root = temp_ngs_output_root("retry");
        let local_path = output_root.join("runs/ERRRETRY/fastq/ERRRETRY.fastq.gz");
        let partial_path = partial_ngs_asset_path(&local_path);
        fs::create_dir_all(
            partial_path
                .parent()
                .expect("partial path should have parent"),
        )
        .expect("partial directory should be created");
        fs::write(&partial_path, b"AC").expect("partial should be written");
        let plan = NgsDownloadPlan::new(manifest, output_root.clone(), false, vec![asset]);
        let client = MockHttpClient::default().with_byte_response(
            "https://example.invalid/ERRRETRY.fastq.gz",
            HttpBytesResponse::new(200, b"GT\n".to_vec()),
        );
        let gateway = ServiceNgsRetrieval::with_client(&config, &registry, client);

        let records = gateway
            .materialize_download_plan(&plan)
            .expect("retry should resume partial and promote verified file");

        assert_eq!(records.len(), 1);
        assert_eq!(
            records[0].verification_status,
            NgsVerificationStatus::Verified
        );
        assert_eq!(
            fs::read(&records[0].local_path).expect("promoted file should be readable"),
            body
        );
        assert_eq!(
            records[0].materialization_method.as_deref(),
            Some("direct_download_resume")
        );
        assert!(!partial_path.exists());
        fs::remove_dir_all(output_root).ok();
    }

    #[test]
    fn materializes_sra_archive_and_records_fastq_conversion_plan() {
        let config = PlatformConfig::default();
        let registry = ProviderRegistry::builtin_defaults();
        let manifest = sra_planned_manifest();
        let selected_assets = manifest.assets().into_iter().cloned().collect();
        let output_root = temp_ngs_output_root("sra");
        let plan = NgsDownloadPlan::new(manifest, output_root.clone(), false, selected_assets);
        let client = MockHttpClient::default().with_byte_response(
            "https://example.invalid/SRR123456.sra",
            HttpBytesResponse::new(200, b"SRA_ARCHIVE".to_vec()),
        );
        let gateway = ServiceNgsRetrieval::with_client(&config, &registry, client);

        let runner = FakeSraFastqRunner::successful_paired();
        let records = gateway
            .materialize_download_plan_with_sra_runner(&plan, &runner)
            .expect("SRA materialization should download archive and plan FASTQ conversion");

        assert_eq!(records.len(), 2);
        assert_eq!(records[0].asset.role, NgsAssetRole::SraArchive);
        assert_eq!(
            records[0].verification_status,
            NgsVerificationStatus::Unverified
        );
        assert_eq!(
            records[0].materialization_method.as_deref(),
            Some("direct_download")
        );
        assert_eq!(
            records[0].local_path,
            output_root.join("runs/SRR123456/sra/SRR123456.sra")
        );
        assert_eq!(
            fs::read(&records[0].local_path).expect("SRA archive should be readable"),
            b"SRA_ARCHIVE".to_vec()
        );
        assert_eq!(records[1].asset.role, NgsAssetRole::GeneratedFastq);
        assert_eq!(
            records[1].verification_status,
            NgsVerificationStatus::Unverified
        );
        assert_eq!(records[1].exit_status, Some(0));
        assert_eq!(
            records[1].materialization_method.as_deref(),
            Some("sra_toolkit_conversion")
        );
        assert_eq!(
            records[1].container_image.as_deref(),
            Some(DEFAULT_SRA_TOOLKIT_CONTAINER)
        );
        assert!(
            records[1]
                .command_line
                .as_deref()
                .is_some_and(|command| command.contains("fasterq-dump"))
        );
        assert_eq!(
            records[1].local_path,
            output_root.join("runs/SRR123456/fastq")
        );
        assert_eq!(records[1].generated_paths.len(), 2);
        assert_eq!(
            records[1].generated_paths,
            vec![
                output_root.join("runs/SRR123456/fastq/SRR123456_1.fastq"),
                output_root.join("runs/SRR123456/fastq/SRR123456_2.fastq"),
            ]
        );
        assert_eq!(records[1].observed_size_bytes, Some(32));
        fs::remove_dir_all(output_root).ok();
    }

    #[test]
    fn records_failed_sra_fastq_conversion_exit_status() {
        let config = PlatformConfig::default();
        let registry = ProviderRegistry::builtin_defaults();
        let manifest = sra_planned_manifest();
        let selected_assets = manifest
            .assets()
            .into_iter()
            .filter(|asset| asset.role == NgsAssetRole::GeneratedFastq)
            .cloned()
            .collect();
        let output_root = temp_ngs_output_root("sra-fail");
        let plan = NgsDownloadPlan::new(manifest, output_root.clone(), false, selected_assets);
        let gateway =
            ServiceNgsRetrieval::with_client(&config, &registry, MockHttpClient::default());

        let records = gateway
            .materialize_download_plan_with_sra_runner(&plan, &FakeSraFastqRunner::failing())
            .expect("failed SRA conversion should be recorded");

        assert_eq!(records.len(), 1);
        assert_eq!(
            records[0].verification_status,
            NgsVerificationStatus::Failed
        );
        assert_eq!(records[0].exit_status, Some(2));
        assert!(
            records[0]
                .failure_reason
                .as_deref()
                .is_some_and(|reason| reason.contains("exited with status 2"))
        );
        fs::remove_dir_all(output_root).ok();
    }

    #[test]
    fn records_interrupted_sra_fastq_conversion_as_failed() {
        let config = PlatformConfig::default();
        let registry = ProviderRegistry::builtin_defaults();
        let manifest = sra_planned_manifest();
        let selected_assets = manifest
            .assets()
            .into_iter()
            .filter(|asset| asset.role == NgsAssetRole::GeneratedFastq)
            .cloned()
            .collect();
        let output_root = temp_ngs_output_root("sra-interrupted");
        let plan = NgsDownloadPlan::new(manifest, output_root.clone(), false, selected_assets);
        let gateway =
            ServiceNgsRetrieval::with_client(&config, &registry, MockHttpClient::default());

        let records = gateway
            .materialize_download_plan_with_sra_runner(&plan, &FakeSraFastqRunner::interrupted())
            .expect("interrupted SRA conversion should be recorded");

        assert_eq!(records.len(), 1);
        assert_eq!(
            records[0].verification_status,
            NgsVerificationStatus::Failed
        );
        assert_eq!(records[0].exit_status, Some(-1));
        assert!(
            records[0]
                .failure_reason
                .as_deref()
                .is_some_and(|reason| reason.contains("exited with status -1"))
        );
        fs::remove_dir_all(output_root).ok();
    }
}

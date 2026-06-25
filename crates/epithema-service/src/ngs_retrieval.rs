//! Service-owned NGS dataset acquisition facade.
//!
//! This module is the formal acquisition seam for provider-backed NGS dataset
//! manifest discovery. It centralizes provider routing, configuration checks,
//! and the future download/provenance method surface outside the CLI and tool
//! implementations.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use epithema_config::PlatformConfig;
use epithema_diagnostics::{ArtifactOriginKind, ArtifactProvenance, ErrorCategory, PlatformError};
use epithema_providers::{
    EnaNgsAdapter, HttpRequest, NgsAsset, NgsAssetRole, NgsDownloadPlan, NgsDownloadRecord,
    NgsManifest, NgsManifestRun, NgsProvenance, NgsQuery, NgsRunMetadata, NgsVerificationStatus,
    ProviderCapability, ProviderHttpClient, ProviderId, ProviderRegistry, ReqwestHttpClient,
    SraNgsAdapter,
};
use serde_json::{Value, json};

const DEFAULT_SRA_TOOLKIT_CONTAINER: &str = "docker.io/ncbi/sra-tools:3.1.1";
const DEFAULT_SRA_TOOLKIT_VERSION: &str = "3.1.1";

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

    /// Materializes directly downloadable assets and plans SRA FASTQ conversion.
    pub fn materialize_download_plan(
        &self,
        plan: &NgsDownloadPlan,
    ) -> Result<Vec<NgsDownloadRecord>, PlatformError> {
        self.materialize_download_plan_with_sra_runner(plan, &DockerSraFastqRunner)
    }

    /// Materializes selected assets with an explicit SRA FASTQ conversion runner.
    pub fn materialize_download_plan_with_sra_runner<R: SraFastqRunner>(
        &self,
        plan: &NgsDownloadPlan,
        runner: &R,
    ) -> Result<Vec<NgsDownloadRecord>, PlatformError> {
        match plan.manifest.provider.as_str() {
            "ena" => plan
                .selected_assets
                .iter()
                .map(|asset| materialize_direct_ngs_asset(&self.client, plan, asset))
                .collect(),
            "sra" => plan
                .selected_assets
                .iter()
                .map(|asset| materialize_sra_ngs_asset(&self.client, plan, asset, runner))
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
            .with_verification_status(NgsVerificationStatus::SkippedVerified)
            .with_materialization_method("direct_download"));
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
        .with_observed_evidence(observed_size, observed_checksum.clone())
        .with_materialization_method("direct_download");

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

fn materialize_sra_ngs_asset<C: ProviderHttpClient>(
    client: &C,
    plan: &NgsDownloadPlan,
    asset: &NgsAsset,
    runner: &impl SraFastqRunner,
) -> Result<NgsDownloadRecord, PlatformError> {
    match asset.role {
        NgsAssetRole::SraArchive => materialize_direct_ngs_asset(client, plan, asset),
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
        NgsAssetRole, NgsDownloadPlan, NgsManifest, NgsManifestRun, NgsProvenance, NgsQuery,
        NgsRunMetadata, NgsVerificationStatus, ProviderCapability, ProviderDescriptor,
        ProviderHttpClient, ProviderId, ProviderRegistry, SraNgsAdapter,
    };

    use super::{
        DEFAULT_SRA_TOOLKIT_CONTAINER, ServiceNgsRetrieval, SraFastqConversionRequest,
        SraFastqConversionResult, SraFastqRunner, partial_ngs_asset_path,
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
    fn overwrites_stale_partial_file_on_retry_before_promotion() {
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
        fs::write(&partial_path, b"stale partial").expect("stale partial should be written");
        let plan = NgsDownloadPlan::new(manifest, output_root.clone(), false, vec![asset]);
        let client = MockHttpClient::default().with_byte_response(
            "https://example.invalid/ERRRETRY.fastq.gz",
            HttpBytesResponse::new(200, body.clone()),
        );
        let gateway = ServiceNgsRetrieval::with_client(&config, &registry, client);

        let records = gateway
            .materialize_download_plan(&plan)
            .expect("retry should overwrite stale partial and promote verified file");

        assert_eq!(records.len(), 1);
        assert_eq!(
            records[0].verification_status,
            NgsVerificationStatus::Verified
        );
        assert_eq!(
            fs::read(&records[0].local_path).expect("promoted file should be readable"),
            body
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

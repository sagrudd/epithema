//! Provider-neutral models for governed NGS dataset discovery and acquisition.
//!
//! These types describe NGS dataset manifests, download plans, materialized
//! files, and provenance without committing to an ENA or SRA wire format. The
//! provider-specific adapters are responsible for filling these models from
//! external API responses.

use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use epithema_diagnostics::{ArtifactProvenance, ErrorCategory, PlatformError};

use crate::{ArchiveRoute, ProviderId};

/// Stable schema label for NGS acquisition provenance records.
pub const NGS_PROVENANCE_SCHEMA: &str = "epithema.ngs-provenance/v1";

/// Supported NGS query object classes.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum NgsObjectClass {
    /// Study- or project-level query.
    Study,
    /// Sample-level query.
    Sample,
    /// Experiment-level query.
    Experiment,
    /// Run-level query.
    Run,
}

impl NgsObjectClass {
    /// Returns the stable lowercase label for the object class.
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Study => "study",
            Self::Sample => "sample",
            Self::Experiment => "experiment",
            Self::Run => "run",
        }
    }
}

/// Provider-neutral query for NGS dataset discovery.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NgsQuery {
    /// Original accession or provider-local query string.
    pub accession: String,
    /// Optional provider selected by the user or resolver.
    pub provider: Option<ProviderId>,
    /// Optional object class once the accession has been classified.
    pub object_class: Option<NgsObjectClass>,
}

impl NgsQuery {
    /// Creates a new unresolved NGS query.
    #[must_use]
    pub fn new(accession: impl Into<String>) -> Self {
        Self {
            accession: accession.into(),
            provider: None,
            object_class: None,
        }
    }

    /// Classifies a raw NGS query token into a provider-neutral query.
    ///
    /// This parser accepts bare accessions plus `auto:`, `ena:`, and `sra:`
    /// provider-qualified forms. Bare accessions are classified by object
    /// class only; later service/provider routing decides whether ENA or SRA is
    /// appropriate for materialization.
    pub fn classify(raw: impl Into<String>) -> Result<Self, PlatformError> {
        let raw = raw.into();
        let token = raw.trim();
        if token.is_empty() {
            return Err(PlatformError::new(
                ErrorCategory::Validation,
                "NGS query accession must not be empty",
            )
            .with_code("providers.ngs.query.empty"));
        }

        let (provider, accession) = parse_optional_ngs_provider(token)?;
        let object_class = infer_ngs_object_class(accession)?;
        let mut query = Self::new(accession).with_object_class(object_class);
        if let Some(provider) = provider {
            query = query.with_provider(provider);
        }
        Ok(query)
    }

    /// Attaches a provider identity to the query.
    #[must_use]
    pub fn with_provider(mut self, provider: ProviderId) -> Self {
        self.provider = Some(provider);
        self
    }

    /// Attaches a resolved object class to the query.
    #[must_use]
    pub fn with_object_class(mut self, object_class: NgsObjectClass) -> Self {
        self.object_class = Some(object_class);
        self
    }
}

fn parse_optional_ngs_provider(token: &str) -> Result<(Option<ProviderId>, &str), PlatformError> {
    let Some((provider_raw, accession_raw)) = token.split_once(':') else {
        return Ok((None, token));
    };

    let provider = provider_raw.trim().to_ascii_lowercase();
    let accession = accession_raw.trim();
    if accession.is_empty() {
        return Err(PlatformError::new(
            ErrorCategory::Validation,
            "NGS provider-qualified query is missing an accession",
        )
        .with_code("providers.ngs.query.missing_accession")
        .with_detail(token.to_owned()));
    }

    match provider.as_str() {
        "auto" => Ok((None, accession)),
        "ena" | "sra" => Ok((Some(ProviderId::new(provider)?), accession)),
        _ => Err(PlatformError::new(
            ErrorCategory::Validation,
            "NGS provider-qualified query must use 'auto', 'ena', or 'sra'",
        )
        .with_code("providers.ngs.query.unsupported_provider")
        .with_detail(token.to_owned())),
    }
}

fn infer_ngs_object_class(accession: &str) -> Result<NgsObjectClass, PlatformError> {
    let uppercase = accession.trim().to_ascii_uppercase();
    if uppercase.starts_with("ERR") || uppercase.starts_with("SRR") {
        return Ok(NgsObjectClass::Run);
    }
    if uppercase.starts_with("ERX") || uppercase.starts_with("SRX") {
        return Ok(NgsObjectClass::Experiment);
    }
    if uppercase.starts_with("ERS")
        || uppercase.starts_with("SRS")
        || uppercase.starts_with("SAMN")
        || uppercase.starts_with("SAMEA")
    {
        return Ok(NgsObjectClass::Sample);
    }
    if uppercase.starts_with("ERP")
        || uppercase.starts_with("SRP")
        || uppercase.starts_with("PRJNA")
        || uppercase.starts_with("PRJEB")
    {
        return Ok(NgsObjectClass::Study);
    }

    Err(PlatformError::new(
        ErrorCategory::Validation,
        "NGS accession could not be classified conservatively",
    )
    .with_code("providers.ngs.query.unsupported_accession")
    .with_detail(accession.to_owned()))
}

/// Normalized run-level metadata for an NGS dataset row.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NgsRunMetadata {
    /// Canonical run accession.
    pub run_accession: String,
    /// Experiment accession when available.
    pub experiment_accession: Option<String>,
    /// Sample accession when available.
    pub sample_accession: Option<String>,
    /// Study or project accession when available.
    pub study_accession: Option<String>,
    /// Study title when available.
    pub study_title: Option<String>,
    /// Sample title when available.
    pub sample_title: Option<String>,
    /// Experiment title when available.
    pub experiment_title: Option<String>,
    /// Scientific name when available.
    pub scientific_name: Option<String>,
    /// Sequencing platform when available.
    pub instrument_platform: Option<String>,
    /// Instrument model when available.
    pub instrument_model: Option<String>,
    /// Library strategy when available.
    pub library_strategy: Option<String>,
    /// Library source when available.
    pub library_source: Option<String>,
    /// Library selection when available.
    pub library_selection: Option<String>,
    /// Library layout when available.
    pub library_layout: Option<String>,
}

impl NgsRunMetadata {
    /// Creates run metadata with the required canonical run accession.
    #[must_use]
    pub fn new(run_accession: impl Into<String>) -> Self {
        Self {
            run_accession: run_accession.into(),
            experiment_accession: None,
            sample_accession: None,
            study_accession: None,
            study_title: None,
            sample_title: None,
            experiment_title: None,
            scientific_name: None,
            instrument_platform: None,
            instrument_model: None,
            library_strategy: None,
            library_source: None,
            library_selection: None,
            library_layout: None,
        }
    }

    /// Attaches normalized parent accessions.
    #[must_use]
    pub fn with_accessions(
        mut self,
        study_accession: Option<String>,
        sample_accession: Option<String>,
        experiment_accession: Option<String>,
    ) -> Self {
        self.study_accession = study_accession;
        self.sample_accession = sample_accession;
        self.experiment_accession = experiment_accession;
        self
    }
}

/// Stable role for an NGS dataset asset.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum NgsAssetRole {
    /// Provider-generated FASTQ, usually the default download target.
    GeneratedFastq,
    /// Provider-submitted raw data such as FAST5 or POD5.
    SubmittedRaw,
    /// Provider-submitted alignment data such as BAM or CRAM.
    SubmittedAlignment,
    /// Provider-native SRA archive.
    SraArchive,
    /// Index sidecar such as BAI or CRAI.
    Index,
    /// Submitted asset with an unclassified format.
    UnknownSubmitted,
}

impl NgsAssetRole {
    /// Returns the stable lowercase label for the asset role.
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::GeneratedFastq => "generated_fastq",
            Self::SubmittedRaw => "submitted_raw",
            Self::SubmittedAlignment => "submitted_alignment",
            Self::SraArchive => "sra_archive",
            Self::Index => "index",
            Self::UnknownSubmitted => "unknown_submitted",
        }
    }
}

/// One provider-reported NGS dataset asset.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NgsAsset {
    /// Run accession this asset belongs to.
    pub run_accession: String,
    /// Stable asset role.
    pub role: NgsAssetRole,
    /// File-format label such as `fastq.gz`, `bam`, `fast5`, `pod5`, or `sra`.
    pub format: String,
    /// Source URL or provider-native locator.
    pub source_url: String,
    /// Optional MD5 checksum reported by the provider.
    pub checksum_md5: Option<String>,
    /// Optional byte count reported by the provider.
    pub size_bytes: Option<u64>,
}

impl NgsAsset {
    /// Creates a normalized NGS asset.
    #[must_use]
    pub fn new(
        run_accession: impl Into<String>,
        role: NgsAssetRole,
        format: impl Into<String>,
        source_url: impl Into<String>,
    ) -> Self {
        Self {
            run_accession: run_accession.into(),
            role,
            format: format.into(),
            source_url: source_url.into(),
            checksum_md5: None,
            size_bytes: None,
        }
    }

    /// Attaches an expected MD5 checksum.
    #[must_use]
    pub fn with_checksum_md5(mut self, checksum_md5: Option<String>) -> Self {
        self.checksum_md5 = checksum_md5;
        self
    }

    /// Attaches an expected byte count.
    #[must_use]
    pub fn with_size_bytes(mut self, size_bytes: Option<u64>) -> Self {
        self.size_bytes = size_bytes;
        self
    }
}

/// One run and its associated provider-reported assets.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NgsManifestRun {
    /// Normalized run metadata.
    pub metadata: NgsRunMetadata,
    /// Provider-reported assets associated with the run.
    pub assets: Vec<NgsAsset>,
}

impl NgsManifestRun {
    /// Creates a manifest run entry.
    #[must_use]
    pub fn new(metadata: NgsRunMetadata, assets: Vec<NgsAsset>) -> Self {
        Self { metadata, assets }
    }
}

/// Provider-neutral manifest for an NGS dataset query.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NgsManifest {
    /// Original query that produced this manifest.
    pub query: NgsQuery,
    /// Provider used for manifest expansion.
    pub provider: ProviderId,
    /// Route metadata for the provider lookup.
    pub route: ArchiveRoute,
    /// Run-level manifest entries.
    pub runs: Vec<NgsManifestRun>,
    /// Structured provenance for the manifest lookup.
    pub provenance: ArtifactProvenance,
}

impl NgsManifest {
    /// Creates a provider-neutral NGS manifest.
    #[must_use]
    pub fn new(
        query: NgsQuery,
        provider: ProviderId,
        route: ArchiveRoute,
        runs: Vec<NgsManifestRun>,
    ) -> Self {
        let provenance = ArtifactProvenance::provider_asset(query.accession.clone())
            .with_provider(provider.as_str())
            .with_description(format!(
                "retrieved NGS dataset manifest via {} {}",
                route.endpoint, route.format
            ));
        Self {
            query,
            provider,
            route,
            runs,
            provenance,
        }
    }

    /// Returns all assets in run order.
    #[must_use]
    pub fn assets(&self) -> Vec<&NgsAsset> {
        self.runs.iter().flat_map(|run| run.assets.iter()).collect()
    }

    /// Returns the sum of known file sizes across all assets.
    #[must_use]
    pub fn total_size_bytes(&self) -> Option<u64> {
        let sizes: Vec<u64> = self
            .runs
            .iter()
            .flat_map(|run| run.assets.iter())
            .filter_map(|asset| asset.size_bytes)
            .collect();
        if sizes.is_empty() {
            None
        } else {
            Some(sizes.into_iter().sum())
        }
    }
}

/// Deterministic materialization plan for selected NGS assets.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NgsDownloadPlan {
    /// Manifest that the plan was derived from.
    pub manifest: NgsManifest,
    /// Output root for materialized files and provenance.
    pub output_root: PathBuf,
    /// Whether raw/submitted assets were requested.
    pub include_raw: bool,
    /// Assets selected for materialization.
    pub selected_assets: Vec<NgsAsset>,
}

impl NgsDownloadPlan {
    /// Creates an NGS download plan.
    #[must_use]
    pub fn new(
        manifest: NgsManifest,
        output_root: impl Into<PathBuf>,
        include_raw: bool,
        selected_assets: Vec<NgsAsset>,
    ) -> Self {
        Self {
            manifest,
            output_root: output_root.into(),
            include_raw,
            selected_assets,
        }
    }
}

/// Verification outcome for a materialized NGS asset.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum NgsVerificationStatus {
    /// Asset is planned but has not yet been downloaded or generated.
    Planned,
    /// Asset was materialized and verified against available evidence.
    Verified,
    /// Asset was materialized but no provider checksum or byte count was available.
    Unverified,
    /// Asset verification failed.
    Failed,
    /// Asset was skipped because an already verified local copy exists.
    SkippedVerified,
}

impl NgsVerificationStatus {
    /// Returns the stable lowercase label for the verification status.
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Planned => "planned",
            Self::Verified => "verified",
            Self::Unverified => "unverified",
            Self::Failed => "failed",
            Self::SkippedVerified => "skipped_verified",
        }
    }
}

/// Provenance record for one downloaded or generated NGS asset.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NgsDownloadRecord {
    /// Asset that was selected for materialization.
    pub asset: NgsAsset,
    /// Local materialized path.
    pub local_path: PathBuf,
    /// Observed byte count after materialization.
    pub observed_size_bytes: Option<u64>,
    /// Observed MD5 checksum after materialization.
    pub observed_checksum_md5: Option<String>,
    /// Verification status after materialization.
    pub verification_status: NgsVerificationStatus,
    /// Failure detail when materialization or verification failed.
    pub failure_reason: Option<String>,
    /// Stable materialization method label such as `direct_download` or `sra_toolkit_conversion`.
    pub materialization_method: Option<String>,
    /// Command line used or planned for generated/conversion outputs.
    pub command_line: Option<String>,
    /// Process exit status when an external command was executed.
    pub exit_status: Option<i32>,
    /// Container image used or planned for external tool execution.
    pub container_image: Option<String>,
    /// Tool version string when available.
    pub tool_version: Option<String>,
    /// Local files produced by a conversion step.
    pub generated_paths: Vec<PathBuf>,
}

impl NgsDownloadRecord {
    /// Creates a download record for a selected asset and local path.
    #[must_use]
    pub fn new(asset: NgsAsset, local_path: impl Into<PathBuf>) -> Self {
        Self {
            asset,
            local_path: local_path.into(),
            observed_size_bytes: None,
            observed_checksum_md5: None,
            verification_status: NgsVerificationStatus::Planned,
            failure_reason: None,
            materialization_method: None,
            command_line: None,
            exit_status: None,
            container_image: None,
            tool_version: None,
            generated_paths: Vec::new(),
        }
    }

    /// Attaches observed verification evidence.
    #[must_use]
    pub fn with_observed_evidence(
        mut self,
        observed_size_bytes: Option<u64>,
        observed_checksum_md5: Option<String>,
    ) -> Self {
        self.observed_size_bytes = observed_size_bytes;
        self.observed_checksum_md5 = observed_checksum_md5;
        self
    }

    /// Marks the verification status.
    #[must_use]
    pub fn with_verification_status(mut self, verification_status: NgsVerificationStatus) -> Self {
        self.verification_status = verification_status;
        self
    }

    /// Attaches a failure reason.
    #[must_use]
    pub fn with_failure_reason(mut self, failure_reason: impl Into<String>) -> Self {
        self.failure_reason = Some(failure_reason.into());
        self
    }

    /// Attaches a stable materialization method label.
    #[must_use]
    pub fn with_materialization_method(
        mut self,
        materialization_method: impl Into<String>,
    ) -> Self {
        self.materialization_method = Some(materialization_method.into());
        self
    }

    /// Attaches the command line used or planned for generated outputs.
    #[must_use]
    pub fn with_command_line(mut self, command_line: impl Into<String>) -> Self {
        self.command_line = Some(command_line.into());
        self
    }

    /// Attaches an external process exit status.
    #[must_use]
    pub fn with_exit_status(mut self, exit_status: i32) -> Self {
        self.exit_status = Some(exit_status);
        self
    }

    /// Attaches a container image reference.
    #[must_use]
    pub fn with_container_image(mut self, container_image: impl Into<String>) -> Self {
        self.container_image = Some(container_image.into());
        self
    }

    /// Attaches a tool version string.
    #[must_use]
    pub fn with_tool_version(mut self, tool_version: impl Into<String>) -> Self {
        self.tool_version = Some(tool_version.into());
        self
    }

    /// Attaches local files produced by a conversion step.
    #[must_use]
    pub fn with_generated_paths(mut self, generated_paths: Vec<PathBuf>) -> Self {
        self.generated_paths = generated_paths;
        self
    }
}

/// Provider-neutral provenance bundle for an NGS acquisition run.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NgsProvenance {
    /// Stable schema label.
    pub schema: String,
    /// Acquisition-bundle creation timestamp in Unix seconds.
    pub acquisition_timestamp_unix_seconds: u64,
    /// Manifest used for acquisition.
    pub manifest: NgsManifest,
    /// Download plan selected from the manifest.
    pub download_plan: NgsDownloadPlan,
    /// Materialization and verification records.
    pub download_records: Vec<NgsDownloadRecord>,
}

impl NgsProvenance {
    /// Creates an NGS provenance bundle.
    #[must_use]
    pub fn new(
        manifest: NgsManifest,
        download_plan: NgsDownloadPlan,
        download_records: Vec<NgsDownloadRecord>,
    ) -> Self {
        Self::new_at_unix_seconds(
            manifest,
            download_plan,
            download_records,
            current_unix_seconds(),
        )
    }

    /// Creates an NGS provenance bundle with an explicit timestamp.
    #[must_use]
    pub fn new_at_unix_seconds(
        manifest: NgsManifest,
        download_plan: NgsDownloadPlan,
        download_records: Vec<NgsDownloadRecord>,
        acquisition_timestamp_unix_seconds: u64,
    ) -> Self {
        Self {
            schema: NGS_PROVENANCE_SCHEMA.to_owned(),
            acquisition_timestamp_unix_seconds,
            manifest,
            download_plan,
            download_records,
        }
    }
}

fn current_unix_seconds() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_secs())
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::{
        ArchiveRoute, NGS_PROVENANCE_SCHEMA, NgsAsset, NgsAssetRole, NgsDownloadPlan,
        NgsDownloadRecord, NgsManifest, NgsManifestRun, NgsObjectClass, NgsProvenance, NgsQuery,
        NgsRunMetadata, NgsVerificationStatus, ProviderId,
    };

    fn provider() -> ProviderId {
        ProviderId::new("ena").expect("static provider id should be valid")
    }

    fn manifest() -> NgsManifest {
        let provider = provider();
        let query = NgsQuery::new("PRJNA1011899")
            .with_provider(provider.clone())
            .with_object_class(NgsObjectClass::Study);
        let route = ArchiveRoute::new(provider.clone(), "ena.portal.filereport", "tsv");
        let metadata = NgsRunMetadata::new("SRR1").with_accessions(
            Some("PRJNA1011899".to_owned()),
            Some("SAMN1".to_owned()),
            Some("SRX1".to_owned()),
        );
        let assets = vec![
            NgsAsset::new(
                "SRR1",
                NgsAssetRole::GeneratedFastq,
                "fastq.gz",
                "ftp://example.invalid/SRR1.fastq.gz",
            )
            .with_size_bytes(Some(10)),
            NgsAsset::new(
                "SRR1",
                NgsAssetRole::SubmittedRaw,
                "pod5",
                "ftp://example.invalid/SRR1.pod5",
            )
            .with_size_bytes(Some(20)),
        ];
        NgsManifest::new(
            query,
            provider,
            route,
            vec![NgsManifestRun::new(metadata, assets)],
        )
    }

    #[test]
    fn stable_labels_match_documented_contract() {
        assert_eq!(NgsObjectClass::Study.as_str(), "study");
        assert_eq!(NgsObjectClass::Sample.as_str(), "sample");
        assert_eq!(NgsObjectClass::Experiment.as_str(), "experiment");
        assert_eq!(NgsObjectClass::Run.as_str(), "run");
        assert_eq!(NgsAssetRole::GeneratedFastq.as_str(), "generated_fastq");
        assert_eq!(NgsAssetRole::SubmittedRaw.as_str(), "submitted_raw");
        assert_eq!(
            NgsAssetRole::SubmittedAlignment.as_str(),
            "submitted_alignment"
        );
        assert_eq!(NgsAssetRole::SraArchive.as_str(), "sra_archive");
        assert_eq!(NgsAssetRole::Index.as_str(), "index");
        assert_eq!(NgsAssetRole::UnknownSubmitted.as_str(), "unknown_submitted");
        assert_eq!(
            NgsVerificationStatus::SkippedVerified.as_str(),
            "skipped_verified"
        );
    }

    #[test]
    fn classifies_bare_project_accession_without_provider() {
        let query =
            NgsQuery::classify(" PRJNA1011899 ").expect("project accession should classify");

        assert_eq!(query.accession, "PRJNA1011899");
        assert_eq!(query.provider, None);
        assert_eq!(query.object_class, Some(NgsObjectClass::Study));
    }

    #[test]
    fn classifies_provider_qualified_run_accession() {
        let query = NgsQuery::classify("ena:ERR123456").expect("ENA run accession should classify");

        assert_eq!(query.accession, "ERR123456");
        assert_eq!(query.provider.as_ref().map(ProviderId::as_str), Some("ena"));
        assert_eq!(query.object_class, Some(NgsObjectClass::Run));
    }

    #[test]
    fn classifies_auto_qualified_sample_without_provider_lock() {
        let query =
            NgsQuery::classify("auto:SAMN123456").expect("auto sample accession should classify");

        assert_eq!(query.provider, None);
        assert_eq!(query.object_class, Some(NgsObjectClass::Sample));
    }

    #[test]
    fn rejects_unsupported_ngs_provider_prefix() {
        let error =
            NgsQuery::classify("ncbi:SRR123456").expect_err("unsupported provider should fail");

        assert_eq!(
            error.code(),
            Some("providers.ngs.query.unsupported_provider")
        );
    }

    #[test]
    fn rejects_unclassified_ngs_accession() {
        let error = NgsQuery::classify("ABC123").expect_err("unclassified accession should fail");

        assert_eq!(
            error.code(),
            Some("providers.ngs.query.unsupported_accession")
        );
    }

    #[test]
    fn manifest_totals_known_asset_sizes() {
        let manifest = manifest();

        assert_eq!(manifest.assets().len(), 2);
        assert_eq!(manifest.total_size_bytes(), Some(30));
        assert_eq!(
            manifest.provenance.description(),
            Some("retrieved NGS dataset manifest via ena.portal.filereport tsv")
        );
    }

    #[test]
    fn download_record_tracks_observed_verification_evidence() {
        let asset = NgsAsset::new(
            "SRR1",
            NgsAssetRole::GeneratedFastq,
            "fastq.gz",
            "ftp://example.invalid/SRR1.fastq.gz",
        )
        .with_checksum_md5(Some("expected".to_owned()));
        let record = NgsDownloadRecord::new(asset, "runs/SRR1/fastq/SRR1.fastq.gz")
            .with_observed_evidence(Some(100), Some("observed".to_owned()))
            .with_verification_status(NgsVerificationStatus::Failed)
            .with_failure_reason("checksum mismatch");

        assert_eq!(record.observed_size_bytes, Some(100));
        assert_eq!(record.observed_checksum_md5.as_deref(), Some("observed"));
        assert_eq!(record.verification_status, NgsVerificationStatus::Failed);
        assert_eq!(record.failure_reason.as_deref(), Some("checksum mismatch"));
        assert_eq!(record.materialization_method, None);
    }

    #[test]
    fn provenance_uses_stable_schema_label() {
        let manifest = manifest();
        let selected_assets = manifest.assets().into_iter().cloned().collect::<Vec<_>>();
        let plan = NgsDownloadPlan::new(
            manifest.clone(),
            PathBuf::from("ngs-out"),
            false,
            selected_assets,
        );
        let provenance = NgsProvenance::new(manifest.clone(), plan.clone(), Vec::new());

        assert_eq!(provenance.schema, NGS_PROVENANCE_SCHEMA);
        assert!(provenance.acquisition_timestamp_unix_seconds > 0);
        assert_eq!(provenance.manifest, manifest);
        assert_eq!(provenance.download_plan, plan);
    }
}

//! Provider-neutral models for governed NGS dataset discovery and acquisition.
//!
//! These types describe NGS dataset manifests, download plans, materialized
//! files, and provenance without committing to an ENA or SRA wire format. The
//! provider-specific adapters are responsible for filling these models from
//! external API responses.

use std::path::PathBuf;

use epithema_diagnostics::ArtifactProvenance;

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
}

/// Provider-neutral provenance bundle for an NGS acquisition run.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NgsProvenance {
    /// Stable schema label.
    pub schema: String,
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
        Self {
            schema: NGS_PROVENANCE_SCHEMA.to_owned(),
            manifest,
            download_plan,
            download_records,
        }
    }
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
        assert_eq!(provenance.manifest, manifest);
        assert_eq!(provenance.download_plan, plan);
    }
}

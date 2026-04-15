//! Typed archive metadata and public-run manifest retrieval models.

use csv::ReaderBuilder;
use emboss_diagnostics::{ArtifactProvenance, ErrorCategory, PlatformError};

use crate::{AcquisitionRequest, HttpResponse, InputReference, ProviderHttpClient, ProviderId};

/// Supported archive object classes for the first EMBOSS-RS archive cohort.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ArchiveObjectClass {
    /// Run-level archive object.
    Run,
    /// Experiment-level archive object.
    Experiment,
    /// Sample-level archive object.
    Sample,
    /// Study- or project-level archive object.
    Study,
}

impl ArchiveObjectClass {
    /// Returns the stable lowercase label for the object class.
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Run => "run",
            Self::Experiment => "experiment",
            Self::Sample => "sample",
            Self::Study => "study",
        }
    }
}

/// Provenance-rich route metadata for archive metadata and manifest lookup.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ArchiveRoute {
    /// Provider identity used for the request.
    pub provider: ProviderId,
    /// Stable provider-local route label.
    pub endpoint: String,
    /// Stable payload-format label used for normalization.
    pub format: String,
}

impl ArchiveRoute {
    /// Creates a new archive route descriptor.
    #[must_use]
    pub fn new(
        provider: ProviderId,
        endpoint: impl Into<String>,
        format: impl Into<String>,
    ) -> Self {
        Self {
            provider,
            endpoint: endpoint.into(),
            format: format.into(),
        }
    }
}

/// One normalized public file entry associated with an archive object.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ArchiveFile {
    /// Stable file role label such as `fastq`, `submitted`, or `sra`.
    pub role: String,
    /// URL or provider-backed locator for the file.
    pub url: String,
    /// File-format label such as `fastq.gz` or `sra`.
    pub format: String,
    /// Optional MD5 checksum when the provider exposes one.
    pub checksum_md5: Option<String>,
    /// Optional file length in bytes.
    pub size_bytes: Option<u64>,
}

impl ArchiveFile {
    /// Creates a normalized archive file entry.
    #[must_use]
    pub fn new(role: impl Into<String>, url: impl Into<String>, format: impl Into<String>) -> Self {
        Self {
            role: role.into(),
            url: url.into(),
            format: format.into(),
            checksum_md5: None,
            size_bytes: None,
        }
    }

    /// Attaches an MD5 checksum when available.
    #[must_use]
    pub fn with_checksum_md5(mut self, checksum_md5: Option<String>) -> Self {
        self.checksum_md5 = checksum_md5;
        self
    }

    /// Attaches a byte length when available.
    #[must_use]
    pub fn with_size_bytes(mut self, size_bytes: Option<u64>) -> Self {
        self.size_bytes = size_bytes;
        self
    }
}

/// Provider-neutral normalized archive metadata record.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RetrievedArchiveMetadata {
    /// Provider identity used for lookup.
    pub provider: ProviderId,
    /// Requested accession or provider-local locator.
    pub requested_accession: String,
    /// Archive object class.
    pub object_class: ArchiveObjectClass,
    /// Canonical run accession when present.
    pub run_accession: Option<String>,
    /// Canonical experiment accession when present.
    pub experiment_accession: Option<String>,
    /// Canonical sample accession when present.
    pub sample_accession: Option<String>,
    /// Canonical study accession when present.
    pub study_accession: Option<String>,
    /// Sequencing platform when available.
    pub platform: Option<String>,
    /// Instrument model when available.
    pub instrument_model: Option<String>,
    /// Library layout when available.
    pub library_layout: Option<String>,
    /// Library strategy when available.
    pub library_strategy: Option<String>,
    /// Library source when available.
    pub library_source: Option<String>,
    /// Normalized public files when exposed by the provider route.
    pub files: Vec<ArchiveFile>,
    /// Route metadata for the lookup.
    pub route: ArchiveRoute,
    /// Structured provenance for the looked-up metadata.
    pub provenance: ArtifactProvenance,
}

impl RetrievedArchiveMetadata {
    /// Creates a provider-neutral normalized archive metadata record.
    #[must_use]
    pub fn new(
        provider: ProviderId,
        requested_accession: impl Into<String>,
        object_class: ArchiveObjectClass,
        route: ArchiveRoute,
    ) -> Self {
        let requested_accession = requested_accession.into();
        let provenance = ArtifactProvenance::provider_asset(requested_accession.clone())
            .with_provider(provider.as_str())
            .with_description(format!(
                "retrieved archive metadata via {} {}",
                route.endpoint, route.format
            ));
        Self {
            provider,
            requested_accession,
            object_class,
            run_accession: None,
            experiment_accession: None,
            sample_accession: None,
            study_accession: None,
            platform: None,
            instrument_model: None,
            library_layout: None,
            library_strategy: None,
            library_source: None,
            files: Vec::new(),
            route,
            provenance,
        }
    }

    /// Attaches normalized public files to the metadata record.
    #[must_use]
    pub fn with_files(mut self, files: Vec<ArchiveFile>) -> Self {
        self.files = files;
        self
    }

    /// Converts the metadata record into a provider-neutral public-run manifest.
    #[must_use]
    pub fn to_manifest(&self) -> RetrievedArchiveManifest {
        RetrievedArchiveManifest::new(
            self.provider.clone(),
            self.requested_accession.clone(),
            self.object_class,
            self.route.clone(),
            self.files.clone(),
        )
    }

    /// Returns the sum of known file sizes.
    #[must_use]
    pub fn total_size_bytes(&self) -> Option<u64> {
        let sizes: Vec<u64> = self
            .files
            .iter()
            .filter_map(|file| file.size_bytes)
            .collect();
        if sizes.is_empty() {
            None
        } else {
            Some(sizes.into_iter().sum())
        }
    }
}

/// Provider-neutral public-run manifest.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RetrievedArchiveManifest {
    /// Provider identity used for lookup.
    pub provider: ProviderId,
    /// Requested accession or provider-local locator.
    pub requested_accession: String,
    /// Archive object class for the manifest source.
    pub object_class: ArchiveObjectClass,
    /// Normalized downloadable public files.
    pub files: Vec<ArchiveFile>,
    /// Route metadata for the manifest lookup.
    pub route: ArchiveRoute,
    /// Structured provenance for the manifest.
    pub provenance: ArtifactProvenance,
}

impl RetrievedArchiveManifest {
    /// Creates a provider-neutral public-run manifest.
    #[must_use]
    pub fn new(
        provider: ProviderId,
        requested_accession: impl Into<String>,
        object_class: ArchiveObjectClass,
        route: ArchiveRoute,
        files: Vec<ArchiveFile>,
    ) -> Self {
        let requested_accession = requested_accession.into();
        let provenance = ArtifactProvenance::provider_asset(requested_accession.clone())
            .with_provider(provider.as_str())
            .with_description(format!(
                "retrieved archive manifest via {} {}",
                route.endpoint, route.format
            ));
        Self {
            provider,
            requested_accession,
            object_class,
            files,
            route,
            provenance,
        }
    }

    /// Returns the sum of known file sizes.
    #[must_use]
    pub fn total_size_bytes(&self) -> Option<u64> {
        let sizes: Vec<u64> = self
            .files
            .iter()
            .filter_map(|file| file.size_bytes)
            .collect();
        if sizes.is_empty() {
            None
        } else {
            Some(sizes.into_iter().sum())
        }
    }
}

/// Concrete provider and object resolution for archive requests.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ArchiveProviderResolution {
    /// Chosen provider identity.
    pub provider: ProviderId,
    /// Normalized archive object class.
    pub object_class: ArchiveObjectClass,
    /// Provider-local accession or locator payload.
    pub accession: String,
}

impl ArchiveProviderResolution {
    /// Resolves a provider-backed archive request conservatively.
    pub fn from_request(request: &AcquisitionRequest) -> Result<Self, PlatformError> {
        let provider = request
            .preferred_provider
            .clone()
            .or_else(|| match &request.input {
                InputReference::ProviderAsset { provider, .. } => provider.clone(),
                _ => None,
            });

        let locator = match &request.input {
            InputReference::ProviderAsset { locator, .. } => locator.clone(),
            InputReference::Accession(accession) => accession.clone(),
            other => {
                return Err(PlatformError::new(
                    ErrorCategory::Validation,
                    "archive retrieval only supports accession-style provider inputs",
                )
                .with_code("providers.archive.unsupported_input_kind")
                .with_detail(format!("{other:?}")));
            }
        };

        let (provider, object_class, accession) = match provider {
            Some(provider) => {
                let (object_class, accession) = parse_archive_locator(&locator)?;
                (provider, object_class, accession)
            }
            None => infer_archive_provider_and_class(&locator)?,
        };

        Ok(Self {
            provider,
            object_class,
            accession,
        })
    }
}

/// Provider router for built-in archive metadata and manifest adapters.
#[derive(Clone, Debug, Default)]
pub struct ProviderArchiveRouter;

impl ProviderArchiveRouter {
    /// Creates a provider router.
    #[must_use]
    pub fn new() -> Self {
        Self
    }

    /// Routes a request to a built-in provider archive metadata adapter.
    pub fn lookup_metadata_with<C: ProviderHttpClient>(
        &self,
        request: &AcquisitionRequest,
        client: &C,
    ) -> Result<RetrievedArchiveMetadata, PlatformError> {
        let resolution = ArchiveProviderResolution::from_request(request)?;
        match resolution.provider.as_str() {
            "ena" => crate::ena_archive::EnaArchiveAdapter::new().lookup_metadata(
                request,
                &resolution,
                client,
            ),
            "sra" => crate::sra_archive::SraArchiveAdapter::new().lookup_metadata(
                request,
                &resolution,
                client,
            ),
            other => Err(PlatformError::new(
                ErrorCategory::Registry,
                "archive metadata retrieval is not implemented for the requested provider",
            )
            .with_code("providers.archive.unsupported_provider")
            .with_detail(other.to_owned())),
        }
    }

    /// Routes a request to a built-in provider public-run manifest adapter.
    pub fn manifest_with<C: ProviderHttpClient>(
        &self,
        request: &AcquisitionRequest,
        client: &C,
    ) -> Result<RetrievedArchiveManifest, PlatformError> {
        let resolution = ArchiveProviderResolution::from_request(request)?;
        match resolution.provider.as_str() {
            "ena" => crate::ena_archive::EnaArchiveAdapter::new().run_manifest(
                request,
                &resolution,
                client,
            ),
            "sra" => crate::sra_archive::SraArchiveAdapter::new().run_manifest(
                request,
                &resolution,
                client,
            ),
            other => Err(PlatformError::new(
                ErrorCategory::Registry,
                "archive manifest retrieval is not implemented for the requested provider",
            )
            .with_code("providers.archive.unsupported_provider")
            .with_detail(other.to_owned())),
        }
    }
}

pub(crate) fn validate_archive_response(
    response: &HttpResponse,
    provider: &str,
    requested_accession: &str,
) -> Result<(), PlatformError> {
    if response.status == 404 {
        return Err(PlatformError::new(
            ErrorCategory::Invocation,
            "provider did not find archive metadata for the requested accession",
        )
        .with_code("providers.archive.http.not_found")
        .with_detail(format!("{provider}:{requested_accession}")));
    }

    if !(200..300).contains(&response.status) {
        return Err(PlatformError::new(
            ErrorCategory::Invocation,
            "provider returned an unsuccessful archive HTTP status",
        )
        .with_code("providers.archive.http.failure")
        .with_detail(format!(
            "{provider}:{requested_accession} status={}",
            response.status
        )));
    }

    if response.body.trim().is_empty() {
        return Err(PlatformError::new(
            ErrorCategory::Invocation,
            "provider returned an empty archive response body",
        )
        .with_code("providers.archive.response.empty")
        .with_detail(format!("{provider}:{requested_accession}")));
    }

    Ok(())
}

pub(crate) fn read_delimited_rows(
    body: &str,
    delimiter: u8,
) -> Result<Vec<csv::StringRecord>, PlatformError> {
    let mut reader = ReaderBuilder::new()
        .delimiter(delimiter)
        .flexible(true)
        .has_headers(true)
        .from_reader(body.as_bytes());
    let mut rows = Vec::new();
    for row in reader.records() {
        rows.push(row.map_err(|error| {
            PlatformError::new(
                ErrorCategory::Invocation,
                "provider returned malformed archive metadata content",
            )
            .with_code("providers.archive.parse_failed")
            .with_detail(error.to_string())
        })?);
    }
    Ok(rows)
}

fn parse_archive_locator(locator: &str) -> Result<(ArchiveObjectClass, String), PlatformError> {
    if let Some((class, accession)) = locator.split_once(':') {
        let object_class = match class.trim().to_ascii_lowercase().as_str() {
            "run" => ArchiveObjectClass::Run,
            "experiment" => ArchiveObjectClass::Experiment,
            "sample" => ArchiveObjectClass::Sample,
            "study" | "project" => ArchiveObjectClass::Study,
            _ => {
                return Err(PlatformError::new(
                    ErrorCategory::Validation,
                    "archive locator uses an unsupported object-class selector",
                )
                .with_code("providers.archive.unsupported_locator")
                .with_detail(locator.to_owned()));
            }
        };
        let accession = accession.trim();
        if accession.is_empty() {
            return Err(PlatformError::new(
                ErrorCategory::Validation,
                "archive locator is missing an accession after the object-class selector",
            )
            .with_code("providers.archive.unsupported_locator")
            .with_detail(locator.to_owned()));
        }
        return Ok((object_class, accession.to_owned()));
    }

    infer_object_class_from_accession(locator)
        .map(|object_class| (object_class, locator.to_owned()))
}

fn infer_archive_provider_and_class(
    accession: &str,
) -> Result<(ProviderId, ArchiveObjectClass, String), PlatformError> {
    let uppercase = accession.trim().to_ascii_uppercase();
    let object_class = infer_object_class_from_accession(&uppercase)?;

    let provider = if uppercase.starts_with('E') {
        ProviderId::new("ena").expect("static provider id should be valid")
    } else if uppercase.starts_with('S') {
        ProviderId::new("sra").expect("static provider id should be valid")
    } else {
        return Err(PlatformError::new(
            ErrorCategory::Validation,
            "bare archive accession could not be resolved to a supported provider",
        )
        .with_code("providers.archive.ambiguous_bare_accession")
        .with_detail(accession.to_owned()));
    };

    Ok((provider, object_class, accession.trim().to_owned()))
}

fn infer_object_class_from_accession(accession: &str) -> Result<ArchiveObjectClass, PlatformError> {
    let uppercase = accession.trim().to_ascii_uppercase();
    if uppercase.starts_with("ERR") || uppercase.starts_with("SRR") {
        return Ok(ArchiveObjectClass::Run);
    }
    if uppercase.starts_with("ERX") || uppercase.starts_with("SRX") {
        return Ok(ArchiveObjectClass::Experiment);
    }
    if uppercase.starts_with("ERS") || uppercase.starts_with("SRS") {
        return Ok(ArchiveObjectClass::Sample);
    }
    if uppercase.starts_with("ERP") || uppercase.starts_with("SRP") {
        return Ok(ArchiveObjectClass::Study);
    }

    Err(PlatformError::new(
        ErrorCategory::Validation,
        "archive accession could not be classified conservatively",
    )
    .with_code("providers.archive.unsupported_locator")
    .with_detail(accession.to_owned()))
}

pub(crate) fn parse_u64_field(value: Option<&str>) -> Option<u64> {
    value
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .and_then(|value| value.parse::<u64>().ok())
}

pub(crate) fn split_semicolon_field(value: Option<&str>) -> Vec<String> {
    value
        .unwrap_or_default()
        .split(';')
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(ToOwned::to_owned)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{ArchiveObjectClass, ArchiveProviderResolution};
    use crate::{AcquisitionRequest, InputReference, ProviderId, ResolutionIntent};

    #[test]
    fn infers_ena_run_from_bare_err_accession() {
        let request = AcquisitionRequest::new(
            ResolutionIntent::ArchiveAsset,
            InputReference::accession("ERR123456"),
        );

        let resolution = ArchiveProviderResolution::from_request(&request)
            .expect("ERR run should infer provider and class");
        assert_eq!(resolution.provider.as_str(), "ena");
        assert_eq!(resolution.object_class, ArchiveObjectClass::Run);
    }

    #[test]
    fn infers_sra_run_from_bare_srr_accession() {
        let request = AcquisitionRequest::new(
            ResolutionIntent::ArchiveAsset,
            InputReference::accession("SRR123456"),
        );

        let resolution = ArchiveProviderResolution::from_request(&request)
            .expect("SRR run should infer provider and class");
        assert_eq!(resolution.provider.as_str(), "sra");
        assert_eq!(resolution.object_class, ArchiveObjectClass::Run);
    }

    #[test]
    fn respects_explicit_provider_with_object_prefix() {
        let request = AcquisitionRequest::new(
            ResolutionIntent::ArchiveAsset,
            InputReference::provider_asset(
                Some(ProviderId::new("ena").expect("valid provider")),
                "study:ERP000001",
            ),
        )
        .with_preferred_provider(ProviderId::new("ena").expect("valid provider"));

        let resolution = ArchiveProviderResolution::from_request(&request)
            .expect("explicit provider and class should resolve");
        assert_eq!(resolution.object_class, ArchiveObjectClass::Study);
        assert_eq!(resolution.accession, "ERP000001");
    }
}

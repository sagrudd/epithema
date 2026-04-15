//! SRA archive metadata adapter.

use emboss_diagnostics::{ErrorCategory, PlatformError};
use reqwest::Url;

use crate::{
    AcquisitionRequest, ArchiveObjectClass, ArchiveProviderResolution, ArchiveRoute, HttpRequest,
    ProviderHttpClient, ProviderId, RetrievedArchiveManifest, RetrievedArchiveMetadata,
    archive::{read_delimited_rows, validate_archive_response},
};

/// SRA adapter for archive metadata normalization.
#[derive(Clone, Copy, Debug, Default)]
pub struct SraArchiveAdapter;

impl SraArchiveAdapter {
    /// Creates an SRA archive adapter.
    #[must_use]
    pub fn new() -> Self {
        Self
    }

    /// Builds the SRA runinfo metadata request.
    pub fn build_metadata_request(
        &self,
        resolution: &ArchiveProviderResolution,
    ) -> Result<HttpRequest, PlatformError> {
        if resolution.object_class != ArchiveObjectClass::Run {
            return Err(PlatformError::new(
                ErrorCategory::Validation,
                "SRA archive metadata support is limited to run-level accessions in v1",
            )
            .with_code("providers.archive.sra.unsupported_object_class")
            .with_detail(resolution.object_class.as_str().to_owned()));
        }

        let url = Url::parse_with_params(
            "https://trace.ncbi.nlm.nih.gov/Traces/sra-db-be/runinfo",
            &[("acc", resolution.accession.as_str())],
        )
        .map_err(|error| {
            PlatformError::new(
                ErrorCategory::Configuration,
                "failed to construct SRA runinfo URL",
            )
            .with_code("providers.archive.sra.url_build_failed")
            .with_detail(error.to_string())
        })?;
        Ok(HttpRequest::new(url.to_string()).with_accept("text/csv, text/plain;q=0.9"))
    }

    /// Retrieves normalized archive metadata from SRA runinfo.
    pub fn lookup_metadata<C: ProviderHttpClient>(
        &self,
        _request: &AcquisitionRequest,
        resolution: &ArchiveProviderResolution,
        client: &C,
    ) -> Result<RetrievedArchiveMetadata, PlatformError> {
        let provider = ProviderId::new("sra").expect("static provider id should be valid");
        let http_request = self.build_metadata_request(resolution)?;
        let response = client.get_text(&http_request)?;
        validate_archive_response(&response, "sra", &resolution.accession)?;
        let mut rows = read_delimited_rows(&response.body, b',')?;
        if rows.len() != 1 {
            return Err(PlatformError::new(
                ErrorCategory::Invocation,
                "SRA runinfo returned multiple rows where one was expected",
            )
            .with_code("providers.archive.response.multiple_records")
            .with_detail(resolution.accession.clone()));
        }

        let headers = response
            .body
            .lines()
            .next()
            .ok_or_else(|| {
                PlatformError::new(
                    ErrorCategory::Invocation,
                    "SRA archive metadata response did not include a header row",
                )
                .with_code("providers.archive.parse_failed")
                .with_detail(resolution.accession.clone())
            })?
            .split(',')
            .map(ToOwned::to_owned)
            .collect::<Vec<_>>();
        let row = rows.pop().expect("checked single row");
        let value = |column: &str| -> Option<&str> {
            headers
                .iter()
                .position(|header| header == column)
                .and_then(|index| row.get(index))
                .map(str::trim)
                .filter(|value| !value.is_empty())
        };

        let mut metadata = RetrievedArchiveMetadata::new(
            provider.clone(),
            resolution.accession.clone(),
            ArchiveObjectClass::Run,
            ArchiveRoute::new(provider.clone(), "sra.runinfo", "csv"),
        );
        metadata.run_accession = value("Run").map(str::to_owned);
        metadata.study_accession = value("SRAStudy").map(str::to_owned);
        metadata.experiment_accession = value("Experiment").map(str::to_owned);
        metadata.sample_accession = value("Sample").map(str::to_owned);
        metadata.platform = value("Platform").map(str::to_owned);
        metadata.instrument_model = value("Model").map(str::to_owned);
        metadata.library_layout = value("LibraryLayout").map(str::to_owned);
        metadata.library_strategy = value("LibraryStrategy").map(str::to_owned);
        metadata.library_source = value("LibrarySource").map(str::to_owned);
        Ok(metadata)
    }

    /// Returns a clear v1 failure for direct SRA manifest acquisition.
    pub fn run_manifest<C: ProviderHttpClient>(
        &self,
        _request: &AcquisitionRequest,
        resolution: &ArchiveProviderResolution,
        _client: &C,
    ) -> Result<RetrievedArchiveManifest, PlatformError> {
        Err(PlatformError::new(
            ErrorCategory::Invocation,
            "public run-file acquisition is not yet implemented for SRA-backed routes in EMBOSS-RS",
        )
        .with_code("providers.archive.sra.manifest_not_supported")
        .with_detail(resolution.accession.clone()))
    }
}

#[cfg(test)]
mod tests {
    use super::SraArchiveAdapter;
    use crate::{ArchiveObjectClass, ArchiveProviderResolution, ProviderId};

    #[test]
    fn builds_expected_runinfo_url() {
        let adapter = SraArchiveAdapter::new();
        let request = adapter
            .build_metadata_request(&ArchiveProviderResolution {
                provider: ProviderId::new("sra").expect("valid provider"),
                object_class: ArchiveObjectClass::Run,
                accession: "SRR123456".to_owned(),
            })
            .expect("request should build");

        assert!(request.url.contains("runinfo?acc=SRR123456"));
    }
}

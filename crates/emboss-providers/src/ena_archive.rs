//! ENA archive metadata and public-run manifest adapter.

use std::collections::HashMap;

use emboss_diagnostics::{ErrorCategory, PlatformError};
use reqwest::Url;

use crate::{
    AcquisitionRequest, ArchiveFile, ArchiveObjectClass, ArchiveProviderResolution, ArchiveRoute,
    HttpRequest, ProviderHttpClient, ProviderId, RetrievedArchiveManifest,
    RetrievedArchiveMetadata,
    archive::{
        parse_u64_field, read_delimited_rows, split_semicolon_field, validate_archive_response,
    },
};

/// ENA adapter for archive metadata and public-run file discovery.
#[derive(Clone, Copy, Debug, Default)]
pub struct EnaArchiveAdapter;

impl EnaArchiveAdapter {
    /// Creates an ENA archive adapter.
    #[must_use]
    pub fn new() -> Self {
        Self
    }

    /// Builds the ENA run metadata request.
    pub fn build_metadata_request(
        &self,
        resolution: &ArchiveProviderResolution,
    ) -> Result<HttpRequest, PlatformError> {
        if resolution.object_class != ArchiveObjectClass::Run {
            return Err(PlatformError::new(
                ErrorCategory::Validation,
                "ENA archive metadata support is limited to run-level accessions in v1",
            )
            .with_code("providers.archive.ena.unsupported_object_class")
            .with_detail(resolution.object_class.as_str().to_owned()));
        }

        let fields = [
            "run_accession",
            "study_accession",
            "experiment_accession",
            "sample_accession",
            "instrument_platform",
            "instrument_model",
            "library_layout",
            "library_strategy",
            "library_source",
            "fastq_ftp",
            "fastq_md5",
            "fastq_bytes",
            "submitted_ftp",
            "submitted_md5",
            "submitted_bytes",
            "sra_ftp",
            "sra_md5",
            "sra_bytes",
        ]
        .join(",");
        let url = Url::parse_with_params(
            "https://www.ebi.ac.uk/ena/portal/api/filereport",
            &[
                ("accession", resolution.accession.as_str()),
                ("result", "read_run"),
                ("fields", fields.as_str()),
                ("format", "tsv"),
                ("download", "false"),
            ],
        )
        .map_err(|error| {
            PlatformError::new(
                ErrorCategory::Configuration,
                "failed to construct ENA archive metadata URL",
            )
            .with_code("providers.archive.ena.url_build_failed")
            .with_detail(error.to_string())
        })?;

        Ok(HttpRequest::new(url.to_string())
            .with_accept("text/tab-separated-values, text/plain;q=0.9"))
    }

    /// Retrieves normalized archive metadata from ENA.
    pub fn lookup_metadata<C: ProviderHttpClient>(
        &self,
        _request: &AcquisitionRequest,
        resolution: &ArchiveProviderResolution,
        client: &C,
    ) -> Result<RetrievedArchiveMetadata, PlatformError> {
        let provider = ProviderId::new("ena").expect("static provider id should be valid");
        let http_request = self.build_metadata_request(resolution)?;
        let response = client.get_text(&http_request)?;
        validate_archive_response(&response, "ena", &resolution.accession)?;
        let mut rows = read_delimited_rows(&response.body, b'\t')?;
        if rows.len() != 1 {
            return Err(PlatformError::new(
                ErrorCategory::Invocation,
                "ENA archive metadata route returned multiple rows where one was expected",
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
                    "ENA archive metadata response did not include a header row",
                )
                .with_code("providers.archive.parse_failed")
                .with_detail(resolution.accession.clone())
            })?
            .split('\t')
            .map(ToOwned::to_owned)
            .collect::<Vec<_>>();
        let row = rows.pop().expect("checked single row");

        let values = headers
            .iter()
            .enumerate()
            .filter_map(|(index, header)| {
                row.get(index)
                    .map(|value| (header.clone(), value.to_owned()))
            })
            .collect::<HashMap<_, _>>();

        let mut metadata = RetrievedArchiveMetadata::new(
            provider.clone(),
            resolution.accession.clone(),
            resolution.object_class,
            ArchiveRoute::new(provider.clone(), "ena.portal.filereport", "tsv"),
        );
        metadata.run_accession = values
            .get("run_accession")
            .cloned()
            .filter(|value| !value.is_empty());
        metadata.study_accession = values
            .get("study_accession")
            .cloned()
            .filter(|value| !value.is_empty());
        metadata.experiment_accession = values
            .get("experiment_accession")
            .cloned()
            .filter(|value| !value.is_empty());
        metadata.sample_accession = values
            .get("sample_accession")
            .cloned()
            .filter(|value| !value.is_empty());
        metadata.platform = values
            .get("instrument_platform")
            .cloned()
            .filter(|value| !value.is_empty());
        metadata.instrument_model = values
            .get("instrument_model")
            .cloned()
            .filter(|value| !value.is_empty());
        metadata.library_layout = values
            .get("library_layout")
            .cloned()
            .filter(|value| !value.is_empty());
        metadata.library_strategy = values
            .get("library_strategy")
            .cloned()
            .filter(|value| !value.is_empty());
        metadata.library_source = values
            .get("library_source")
            .cloned()
            .filter(|value| !value.is_empty());
        metadata.files = normalized_ena_files(&values);
        Ok(metadata)
    }

    /// Retrieves a normalized ENA public-run manifest.
    pub fn run_manifest<C: ProviderHttpClient>(
        &self,
        request: &AcquisitionRequest,
        resolution: &ArchiveProviderResolution,
        client: &C,
    ) -> Result<RetrievedArchiveManifest, PlatformError> {
        if resolution.object_class != ArchiveObjectClass::Run {
            return Err(PlatformError::new(
                ErrorCategory::Validation,
                "runget currently supports run-level ENA accessions only",
            )
            .with_code("providers.archive.ena.unsupported_object_class")
            .with_detail(resolution.object_class.as_str().to_owned()));
        }

        let metadata = self.lookup_metadata(request, resolution, client)?;
        if metadata.files.is_empty() {
            return Err(PlatformError::new(
                ErrorCategory::Invocation,
                "ENA did not report any downloadable public files for the requested run",
            )
            .with_code("providers.archive.manifest.no_files")
            .with_detail(resolution.accession.clone()));
        }
        Ok(metadata.to_manifest())
    }
}

fn normalized_ena_files(values: &HashMap<String, String>) -> Vec<ArchiveFile> {
    let mut files = Vec::new();
    append_file_group(
        &mut files,
        "fastq",
        "fastq.gz",
        split_semicolon_field(values.get("fastq_ftp").map(String::as_str)),
        split_semicolon_field(values.get("fastq_md5").map(String::as_str)),
        split_semicolon_field(values.get("fastq_bytes").map(String::as_str)),
    );
    append_file_group(
        &mut files,
        "submitted",
        "submitted",
        split_semicolon_field(values.get("submitted_ftp").map(String::as_str)),
        split_semicolon_field(values.get("submitted_md5").map(String::as_str)),
        split_semicolon_field(values.get("submitted_bytes").map(String::as_str)),
    );
    append_file_group(
        &mut files,
        "sra",
        "sra",
        split_semicolon_field(values.get("sra_ftp").map(String::as_str)),
        split_semicolon_field(values.get("sra_md5").map(String::as_str)),
        split_semicolon_field(values.get("sra_bytes").map(String::as_str)),
    );
    files
}

fn append_file_group(
    files: &mut Vec<ArchiveFile>,
    role: &str,
    format: &str,
    urls: Vec<String>,
    checksums: Vec<String>,
    sizes: Vec<String>,
) {
    for (index, url) in urls.into_iter().enumerate() {
        let checksum = checksums
            .get(index)
            .cloned()
            .filter(|value| !value.is_empty());
        let size_bytes = parse_u64_field(sizes.get(index).map(String::as_str));
        files.push(
            ArchiveFile::new(role, normalize_ena_url(&url), format)
                .with_checksum_md5(checksum)
                .with_size_bytes(size_bytes),
        );
    }
}

fn normalize_ena_url(url: &str) -> String {
    if url.starts_with("ftp://") || url.starts_with("http://") || url.starts_with("https://") {
        url.to_owned()
    } else {
        format!("ftp://{url}")
    }
}

#[cfg(test)]
mod tests {
    use super::EnaArchiveAdapter;
    use crate::{ArchiveObjectClass, ArchiveProviderResolution, ProviderId};

    #[test]
    fn builds_expected_metadata_url() {
        let adapter = EnaArchiveAdapter::new();
        let request = adapter
            .build_metadata_request(&ArchiveProviderResolution {
                provider: ProviderId::new("ena").expect("valid provider"),
                object_class: ArchiveObjectClass::Run,
                accession: "ERR123456".to_owned(),
            })
            .expect("request should build");

        assert!(request.url.contains("accession=ERR123456"));
        assert!(request.url.contains("result=read_run"));
        assert!(request.url.contains("format=tsv"));
    }
}

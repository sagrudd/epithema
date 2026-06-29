//! SRA NGS dataset manifest adapter.

use std::collections::HashMap;

use epithema_diagnostics::{ErrorCategory, PlatformError};
use reqwest::Url;

use crate::{
    ArchiveRoute, HttpRequest, NgsAsset, NgsAssetRole, NgsManifest, NgsManifestRun, NgsQuery,
    NgsRunMetadata, ProviderHttpClient, ProviderId,
    archive::{read_delimited_rows, validate_archive_response},
};

/// SRA adapter for NGS run-level manifest expansion.
#[derive(Clone, Copy, Debug, Default)]
pub struct SraNgsAdapter;

impl SraNgsAdapter {
    /// Creates an SRA NGS adapter.
    #[must_use]
    pub fn new() -> Self {
        Self
    }

    /// Builds the SRA RunInfo request for an NGS query.
    pub fn build_manifest_request(&self, query: &NgsQuery) -> Result<HttpRequest, PlatformError> {
        validate_sra_query(query)?;

        let url = Url::parse_with_params(
            "https://trace.ncbi.nlm.nih.gov/Traces/sra-db-be/runinfo",
            &[("acc", query.accession.as_str())],
        )
        .map_err(|error| {
            PlatformError::new(
                ErrorCategory::Configuration,
                "failed to construct SRA NGS manifest URL",
            )
            .with_code("providers.ngs.sra.url_build_failed")
            .with_detail(error.to_string())
        })?;

        Ok(HttpRequest::new(url.to_string()).with_accept("text/csv, text/plain;q=0.9"))
    }

    /// Retrieves and normalizes an SRA NGS manifest.
    pub fn manifest<C: ProviderHttpClient>(
        &self,
        query: &NgsQuery,
        client: &C,
    ) -> Result<NgsManifest, PlatformError> {
        validate_sra_query(query)?;
        let http_request = self.build_manifest_request(query)?;
        let response = client.get_text(&http_request)?;
        validate_archive_response(&response, "sra", &query.accession)?;

        let rows = read_delimited_rows(&response.body, b',')?;
        if rows.is_empty() {
            return Err(PlatformError::new(
                ErrorCategory::Invocation,
                "SRA NGS manifest query did not return any run rows",
            )
            .with_code("providers.ngs.sra.manifest.no_runs")
            .with_detail(query.accession.clone()));
        }

        let headers = response
            .body
            .lines()
            .next()
            .ok_or_else(|| {
                PlatformError::new(
                    ErrorCategory::Invocation,
                    "SRA NGS manifest response did not include a header row",
                )
                .with_code("providers.ngs.sra.parse_failed")
                .with_detail(query.accession.clone())
            })?
            .split(',')
            .map(ToOwned::to_owned)
            .collect::<Vec<_>>();

        let mut manifest_runs = Vec::new();
        for row in rows {
            let values = headers
                .iter()
                .enumerate()
                .filter_map(|(index, header)| {
                    row.get(index)
                        .map(|value| (header.clone(), value.to_owned()))
                })
                .collect::<HashMap<_, _>>();
            manifest_runs.push(normalize_sra_run(&values, &query.accession)?);
        }

        let provider = ProviderId::new("sra").expect("static provider id should be valid");
        Ok(NgsManifest::new(
            query.clone().with_provider(provider.clone()),
            provider.clone(),
            ArchiveRoute::new(provider, "sra.runinfo", "csv"),
            manifest_runs,
        ))
    }
}

fn validate_sra_query(query: &NgsQuery) -> Result<(), PlatformError> {
    if let Some(provider) = &query.provider {
        if provider.as_str() != "sra" {
            return Err(PlatformError::new(
                ErrorCategory::Validation,
                "SRA NGS manifest expansion requires an SRA or provider-neutral query",
            )
            .with_code("providers.ngs.sra.unsupported_provider")
            .with_detail(provider.as_str().to_owned()));
        }
    }

    if query.object_class.is_none() {
        return Err(PlatformError::new(
            ErrorCategory::Validation,
            "SRA NGS manifest expansion requires a classified NGS query",
        )
        .with_code("providers.ngs.sra.unclassified_query")
        .with_detail(query.accession.clone()));
    }

    Ok(())
}

fn normalize_sra_run(
    values: &HashMap<String, String>,
    requested_accession: &str,
) -> Result<NgsManifestRun, PlatformError> {
    let run_accession = required_value(values, "Run", requested_accession)?;
    let mut metadata = NgsRunMetadata::new(run_accession.clone()).with_accessions(
        first_present(values, &["BioProject", "SRAStudy"]),
        first_present(values, &["BioSample", "Sample"]),
        optional_value(values, "Experiment"),
    );
    metadata.study_title = first_present(values, &["StudyTitle", "Study_Title", "Study"]);
    metadata.sample_title = optional_value(values, "SampleName");
    metadata.experiment_title = first_present(values, &["ExperimentTitle", "Experiment_Title"]);
    metadata.scientific_name = optional_value(values, "ScientificName");
    metadata.instrument_platform = optional_value(values, "Platform");
    metadata.instrument_model = optional_value(values, "Model");
    metadata.library_strategy = optional_value(values, "LibraryStrategy");
    metadata.library_source = optional_value(values, "LibrarySource");
    metadata.library_selection = optional_value(values, "LibrarySelection");
    metadata.library_layout = optional_value(values, "LibraryLayout");

    Ok(NgsManifestRun::new(
        metadata,
        normalized_sra_ngs_assets(&run_accession, values),
    ))
}

fn required_value(
    values: &HashMap<String, String>,
    field: &str,
    requested_accession: &str,
) -> Result<String, PlatformError> {
    optional_value(values, field).ok_or_else(|| {
        PlatformError::new(
            ErrorCategory::Invocation,
            "SRA NGS manifest row is missing a required field",
        )
        .with_code("providers.ngs.sra.parse_failed")
        .with_detail(format!("{requested_accession}:{field}"))
    })
}

fn optional_value(values: &HashMap<String, String>, field: &str) -> Option<String> {
    values
        .get(field)
        .map(String::as_str)
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(ToOwned::to_owned)
}

fn first_present(values: &HashMap<String, String>, fields: &[&str]) -> Option<String> {
    fields
        .iter()
        .find_map(|field| optional_value(values, field))
}

fn normalized_sra_ngs_assets(
    run_accession: &str,
    values: &HashMap<String, String>,
) -> Vec<NgsAsset> {
    let sra_source =
        optional_value(values, "download_path").unwrap_or_else(|| format!("sra://{run_accession}"));

    vec![
        NgsAsset::new(run_accession, NgsAssetRole::SraArchive, "sra", sra_source),
        NgsAsset::new(
            run_accession,
            NgsAssetRole::GeneratedFastq,
            "fastq",
            format!("sra-convert://{run_accession}/fastq"),
        ),
    ]
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use epithema_diagnostics::PlatformError;

    use super::SraNgsAdapter;
    use crate::{
        HttpRequest, HttpResponse, NgsAssetRole, NgsObjectClass, NgsQuery, ProviderHttpClient,
    };

    const SRA_STUDY_FIXTURE: &str = include_str!("../tests/fixtures/ngs/sra_runinfo_study.csv");
    const SRA_SAMPLE_FIXTURE: &str = include_str!("../tests/fixtures/ngs/sra_runinfo_sample.csv");
    const SRA_EXPERIMENT_FIXTURE: &str =
        include_str!("../tests/fixtures/ngs/sra_runinfo_experiment.csv");
    const SRA_RUN_FIXTURE: &str = include_str!("../tests/fixtures/ngs/sra_runinfo_run.csv");

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
                .with_code("providers.ngs.sra.test.missing_response")
                .with_detail(request.url.clone())
            })
        }
    }

    #[test]
    fn builds_expected_runinfo_url() {
        let adapter = SraNgsAdapter::new();
        let query = NgsQuery::classify("sra:PRJNA1011899").expect("query should classify");
        let request = adapter
            .build_manifest_request(&query)
            .expect("request should build");

        assert_eq!(
            request.url,
            "https://trace.ncbi.nlm.nih.gov/Traces/sra-db-be/runinfo?acc=PRJNA1011899"
        );
        assert_eq!(
            request.accept.as_deref(),
            Some("text/csv, text/plain;q=0.9")
        );
    }

    #[test]
    fn expands_multi_run_sra_response_into_ngs_manifest() {
        let adapter = SraNgsAdapter::new();
        let query = NgsQuery::classify("sra:PRJNA1011899").expect("query should classify");
        let request = adapter
            .build_manifest_request(&query)
            .expect("request should build");
        let client = MockHttpClient::default()
            .with_response(request.url, HttpResponse::new(200, SRA_STUDY_FIXTURE));

        let manifest = adapter
            .manifest(&query, &client)
            .expect("manifest should parse");

        assert_eq!(manifest.provider.as_str(), "sra");
        assert_eq!(manifest.query.object_class, Some(NgsObjectClass::Study));
        assert_eq!(manifest.runs.len(), 2);
        assert_eq!(
            manifest.runs[0].metadata.study_accession.as_deref(),
            Some("PRJNA1011899")
        );
        assert_eq!(
            manifest.runs[0].metadata.study_title.as_deref(),
            Some("SRA fixture study")
        );
        assert_eq!(
            manifest.runs[1].metadata.instrument_platform.as_deref(),
            Some("OXFORD_NANOPORE")
        );
        assert_eq!(manifest.assets().len(), 4);
        assert_eq!(manifest.assets()[0].role, NgsAssetRole::SraArchive);
        assert_eq!(
            manifest.assets()[0].source_url,
            "https://example.invalid/SRR100001.sra"
        );
        assert_eq!(manifest.assets()[1].role, NgsAssetRole::GeneratedFastq);
        assert_eq!(
            manifest.assets()[1].source_url,
            "sra-convert://SRR100001/fastq"
        );
        assert_eq!(manifest.assets()[2].source_url, "sra://SRR100002");
    }

    #[test]
    fn expands_sra_sample_experiment_and_run_fixtures() {
        let cases = [
            (
                "sra:SAMN200001",
                SRA_SAMPLE_FIXTURE,
                NgsObjectClass::Sample,
                "sra-convert://SRR200001/fastq",
            ),
            (
                "sra:SRX300001",
                SRA_EXPERIMENT_FIXTURE,
                NgsObjectClass::Experiment,
                "sra-convert://SRR300001/fastq",
            ),
            (
                "sra:SRR400001",
                SRA_RUN_FIXTURE,
                NgsObjectClass::Run,
                "sra-convert://SRR400001/fastq",
            ),
        ];

        for (raw_query, fixture, object_class, conversion_locator) in cases {
            let adapter = SraNgsAdapter::new();
            let query = NgsQuery::classify(raw_query).expect("query should classify");
            let request = adapter
                .build_manifest_request(&query)
                .expect("request should build");
            let client = MockHttpClient::default()
                .with_response(request.url, HttpResponse::new(200, fixture));

            let manifest = adapter
                .manifest(&query, &client)
                .expect("manifest should parse");

            assert_eq!(manifest.query.object_class, Some(object_class));
            assert_eq!(manifest.runs.len(), 1);
            assert_eq!(manifest.assets().len(), 2);
            assert_eq!(manifest.assets()[0].role, NgsAssetRole::SraArchive);
            assert_eq!(manifest.assets()[1].role, NgsAssetRole::GeneratedFastq);
            assert_eq!(manifest.assets()[1].source_url, conversion_locator);
        }
    }

    #[test]
    fn rejects_ena_locked_query_for_sra_adapter() {
        let adapter = SraNgsAdapter::new();
        let query = NgsQuery::classify("ena:ERR123456").expect("query should classify");
        let error = adapter
            .build_manifest_request(&query)
            .expect_err("ENA-locked query should fail");

        assert_eq!(error.code(), Some("providers.ngs.sra.unsupported_provider"));
    }
}

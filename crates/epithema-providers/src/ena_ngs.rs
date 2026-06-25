//! ENA NGS dataset manifest adapter.

use std::collections::HashMap;

use epithema_diagnostics::{ErrorCategory, PlatformError};
use reqwest::Url;

use crate::{
    ArchiveRoute, HttpRequest, NgsAsset, NgsAssetRole, NgsManifest, NgsManifestRun, NgsQuery,
    NgsRunMetadata, ProviderHttpClient, ProviderId,
    archive::{
        parse_u64_field, read_delimited_rows, split_semicolon_field, validate_archive_response,
    },
};

/// ENA adapter for NGS run-level manifest expansion.
#[derive(Clone, Copy, Debug, Default)]
pub struct EnaNgsAdapter;

impl EnaNgsAdapter {
    /// Creates an ENA NGS adapter.
    #[must_use]
    pub fn new() -> Self {
        Self
    }

    /// Builds the ENA read-run file report request for an NGS query.
    pub fn build_manifest_request(&self, query: &NgsQuery) -> Result<HttpRequest, PlatformError> {
        validate_ena_query(query)?;

        let fields = ena_ngs_fields().join(",");
        let url = Url::parse_with_params(
            "https://www.ebi.ac.uk/ena/portal/api/filereport",
            &[
                ("accession", query.accession.as_str()),
                ("result", "read_run"),
                ("fields", fields.as_str()),
                ("format", "tsv"),
                ("download", "false"),
            ],
        )
        .map_err(|error| {
            PlatformError::new(
                ErrorCategory::Configuration,
                "failed to construct ENA NGS manifest URL",
            )
            .with_code("providers.ngs.ena.url_build_failed")
            .with_detail(error.to_string())
        })?;

        Ok(HttpRequest::new(url.to_string())
            .with_accept("text/tab-separated-values, text/plain;q=0.9"))
    }

    /// Retrieves and normalizes an ENA NGS manifest.
    pub fn manifest<C: ProviderHttpClient>(
        &self,
        query: &NgsQuery,
        client: &C,
    ) -> Result<NgsManifest, PlatformError> {
        validate_ena_query(query)?;
        let http_request = self.build_manifest_request(query)?;
        let response = client.get_text(&http_request)?;
        validate_archive_response(&response, "ena", &query.accession)?;

        let rows = read_delimited_rows(&response.body, b'\t')?;
        if rows.is_empty() {
            return Err(PlatformError::new(
                ErrorCategory::Invocation,
                "ENA NGS manifest query did not return any run rows",
            )
            .with_code("providers.ngs.ena.manifest.no_runs")
            .with_detail(query.accession.clone()));
        }

        let headers = response
            .body
            .lines()
            .next()
            .ok_or_else(|| {
                PlatformError::new(
                    ErrorCategory::Invocation,
                    "ENA NGS manifest response did not include a header row",
                )
                .with_code("providers.ngs.ena.parse_failed")
                .with_detail(query.accession.clone())
            })?
            .split('\t')
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
            manifest_runs.push(normalize_ena_run(&values, &query.accession)?);
        }

        let provider = ProviderId::new("ena").expect("static provider id should be valid");
        Ok(NgsManifest::new(
            query.clone().with_provider(provider.clone()),
            provider.clone(),
            ArchiveRoute::new(provider, "ena.portal.filereport.read_run", "tsv"),
            manifest_runs,
        ))
    }
}

fn ena_ngs_fields() -> &'static [&'static str] {
    &[
        "run_accession",
        "study_accession",
        "secondary_study_accession",
        "experiment_accession",
        "sample_accession",
        "secondary_sample_accession",
        "study_title",
        "sample_title",
        "experiment_title",
        "scientific_name",
        "instrument_platform",
        "instrument_model",
        "library_strategy",
        "library_source",
        "library_selection",
        "library_layout",
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
}

fn validate_ena_query(query: &NgsQuery) -> Result<(), PlatformError> {
    if let Some(provider) = &query.provider {
        if provider.as_str() != "ena" {
            return Err(PlatformError::new(
                ErrorCategory::Validation,
                "ENA NGS manifest expansion requires an ENA or provider-neutral query",
            )
            .with_code("providers.ngs.ena.unsupported_provider")
            .with_detail(provider.as_str().to_owned()));
        }
    }

    if query.object_class.is_none() {
        return Err(PlatformError::new(
            ErrorCategory::Validation,
            "ENA NGS manifest expansion requires a classified NGS query",
        )
        .with_code("providers.ngs.ena.unclassified_query")
        .with_detail(query.accession.clone()));
    }

    Ok(())
}

fn normalize_ena_run(
    values: &HashMap<String, String>,
    requested_accession: &str,
) -> Result<NgsManifestRun, PlatformError> {
    let run_accession = required_value(values, "run_accession", requested_accession)?;
    let mut metadata = NgsRunMetadata::new(run_accession.clone()).with_accessions(
        first_present(values, &["study_accession", "secondary_study_accession"]),
        first_present(values, &["sample_accession", "secondary_sample_accession"]),
        optional_value(values, "experiment_accession"),
    );
    metadata.study_title = optional_value(values, "study_title");
    metadata.sample_title = optional_value(values, "sample_title");
    metadata.experiment_title = optional_value(values, "experiment_title");
    metadata.scientific_name = optional_value(values, "scientific_name");
    metadata.instrument_platform = optional_value(values, "instrument_platform");
    metadata.instrument_model = optional_value(values, "instrument_model");
    metadata.library_strategy = optional_value(values, "library_strategy");
    metadata.library_source = optional_value(values, "library_source");
    metadata.library_selection = optional_value(values, "library_selection");
    metadata.library_layout = optional_value(values, "library_layout");

    Ok(NgsManifestRun::new(
        metadata,
        normalized_ena_ngs_assets(&run_accession, values),
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
            "ENA NGS manifest row is missing a required field",
        )
        .with_code("providers.ngs.ena.parse_failed")
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

fn normalized_ena_ngs_assets(
    run_accession: &str,
    values: &HashMap<String, String>,
) -> Vec<NgsAsset> {
    let mut assets = Vec::new();
    append_assets(
        &mut assets,
        run_accession,
        NgsAssetRole::GeneratedFastq,
        Some("fastq.gz"),
        split_semicolon_field(values.get("fastq_ftp").map(String::as_str)),
        split_semicolon_field(values.get("fastq_md5").map(String::as_str)),
        split_semicolon_field(values.get("fastq_bytes").map(String::as_str)),
    );
    append_assets(
        &mut assets,
        run_accession,
        NgsAssetRole::SraArchive,
        Some("sra"),
        split_semicolon_field(values.get("sra_ftp").map(String::as_str)),
        split_semicolon_field(values.get("sra_md5").map(String::as_str)),
        split_semicolon_field(values.get("sra_bytes").map(String::as_str)),
    );
    append_assets(
        &mut assets,
        run_accession,
        NgsAssetRole::UnknownSubmitted,
        None,
        split_semicolon_field(values.get("submitted_ftp").map(String::as_str)),
        split_semicolon_field(values.get("submitted_md5").map(String::as_str)),
        split_semicolon_field(values.get("submitted_bytes").map(String::as_str)),
    );
    assets
}

fn append_assets(
    assets: &mut Vec<NgsAsset>,
    run_accession: &str,
    default_role: NgsAssetRole,
    default_format: Option<&str>,
    urls: Vec<String>,
    checksums: Vec<String>,
    sizes: Vec<String>,
) {
    for (index, url) in urls.into_iter().enumerate() {
        let source_url = normalize_ena_url(&url);
        let (role, format) = if let Some(format) = default_format {
            (default_role, format.to_owned())
        } else {
            classify_submitted_asset(&source_url)
        };
        let checksum = checksums
            .get(index)
            .cloned()
            .filter(|value| !value.is_empty());
        let size_bytes = parse_u64_field(sizes.get(index).map(String::as_str));
        assets.push(
            NgsAsset::new(run_accession, role, format, source_url)
                .with_checksum_md5(checksum)
                .with_size_bytes(size_bytes),
        );
    }
}

fn classify_submitted_asset(source_url: &str) -> (NgsAssetRole, String) {
    let lower = source_url.to_ascii_lowercase();
    if lower.ends_with(".pod5") {
        return (NgsAssetRole::SubmittedRaw, "pod5".to_owned());
    }
    if lower.ends_with(".fast5") {
        return (NgsAssetRole::SubmittedRaw, "fast5".to_owned());
    }
    if lower.ends_with(".bam") {
        return (NgsAssetRole::SubmittedAlignment, "bam".to_owned());
    }
    if lower.ends_with(".cram") {
        return (NgsAssetRole::SubmittedAlignment, "cram".to_owned());
    }
    if lower.ends_with(".bai") {
        return (NgsAssetRole::Index, "bai".to_owned());
    }
    if lower.ends_with(".crai") {
        return (NgsAssetRole::Index, "crai".to_owned());
    }
    if lower.ends_with(".fastq.gz") {
        return (NgsAssetRole::UnknownSubmitted, "fastq.gz".to_owned());
    }
    if lower.ends_with(".fq.gz") {
        return (NgsAssetRole::UnknownSubmitted, "fq.gz".to_owned());
    }

    (NgsAssetRole::UnknownSubmitted, "submitted".to_owned())
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
    use std::collections::HashMap;

    use epithema_diagnostics::PlatformError;

    use super::EnaNgsAdapter;
    use crate::{
        HttpRequest, HttpResponse, NgsAssetRole, NgsObjectClass, NgsQuery, ProviderHttpClient,
    };

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
                .with_code("providers.ngs.ena.test.missing_response")
                .with_detail(request.url.clone())
            })
        }
    }

    #[test]
    fn builds_expected_read_run_file_report_url() {
        let adapter = EnaNgsAdapter::new();
        let query = NgsQuery::classify("ena:PRJNA1011899").expect("query should classify");
        let request = adapter
            .build_manifest_request(&query)
            .expect("request should build");

        assert!(request.url.contains("accession=PRJNA1011899"));
        assert!(request.url.contains("result=read_run"));
        assert!(request.url.contains("fields=run_accession"));
        assert!(request.url.contains("format=tsv"));
    }

    #[test]
    fn expands_multi_run_ena_response_into_ngs_manifest() {
        let adapter = EnaNgsAdapter::new();
        let query = NgsQuery::classify("ena:PRJNA1011899").expect("query should classify");
        let request = adapter
            .build_manifest_request(&query)
            .expect("request should build");
        let body = concat!(
            "run_accession\tstudy_accession\tsecondary_study_accession\texperiment_accession\tsample_accession\tsecondary_sample_accession\tstudy_title\tsample_title\texperiment_title\tscientific_name\tinstrument_platform\tinstrument_model\tlibrary_strategy\tlibrary_source\tlibrary_selection\tlibrary_layout\tfastq_ftp\tfastq_md5\tfastq_bytes\tsubmitted_ftp\tsubmitted_md5\tsubmitted_bytes\tsra_ftp\tsra_md5\tsra_bytes\n",
            "ERR1\tERP1\tPRJNA1011899\tERX1\tERS1\tSAMN1\tStudy title\tSample one\tExperiment one\tHomo sapiens\tILLUMINA\tNovaSeq 6000\tWGS\tGENOMIC\tRANDOM\tPAIRED\tftp.sra.ebi.ac.uk/vol1/fastq/ERR1/ERR1_1.fastq.gz;ftp.sra.ebi.ac.uk/vol1/fastq/ERR1/ERR1_2.fastq.gz\tmd51;md52\t10;12\tftp.sra.ebi.ac.uk/vol1/submitted/ERR1/reads.pod5\tmd5raw\t20\tftp.sra.ebi.ac.uk/vol1/sra/ERR1.sra\tmd5sra\t30\n",
            "ERR2\tERP1\tPRJNA1011899\tERX2\tERS2\tSAMN2\tStudy title\tSample two\tExperiment two\tHomo sapiens\tOXFORD_NANOPORE\tPromethION\tRNA-Seq\tTRANSCRIPTOMIC\tcDNA\tSINGLE\tftp.sra.ebi.ac.uk/vol1/fastq/ERR2/ERR2.fastq.gz\tmd53\t14\tftp.sra.ebi.ac.uk/vol1/submitted/ERR2/alignment.bam\tmd5bam\t40\t\t\t\n"
        );
        let client =
            MockHttpClient::default().with_response(request.url, HttpResponse::new(200, body));

        let manifest = adapter
            .manifest(&query, &client)
            .expect("manifest should parse");

        assert_eq!(manifest.provider.as_str(), "ena");
        assert_eq!(manifest.query.object_class, Some(NgsObjectClass::Study));
        assert_eq!(manifest.runs.len(), 2);
        assert_eq!(
            manifest.runs[0].metadata.study_accession.as_deref(),
            Some("ERP1")
        );
        assert_eq!(
            manifest.runs[0].metadata.study_title.as_deref(),
            Some("Study title")
        );
        assert_eq!(manifest.assets().len(), 6);
        assert_eq!(manifest.total_size_bytes(), Some(126));
        assert_eq!(manifest.assets()[0].role, NgsAssetRole::GeneratedFastq);
        assert_eq!(manifest.assets()[2].role, NgsAssetRole::SraArchive);
        assert_eq!(manifest.assets()[3].role, NgsAssetRole::SubmittedRaw);
        assert_eq!(manifest.assets()[5].role, NgsAssetRole::SubmittedAlignment);
    }

    #[test]
    fn rejects_sra_locked_query_for_ena_adapter() {
        let adapter = EnaNgsAdapter::new();
        let query = NgsQuery::classify("sra:SRR123456").expect("query should classify");
        let error = adapter
            .build_manifest_request(&query)
            .expect_err("SRA-locked query should fail");

        assert_eq!(error.code(), Some("providers.ngs.ena.unsupported_provider"));
    }
}

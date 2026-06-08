//! `whichdb` bounded provider-discovery analytical core.

use emboss_diagnostics::{ErrorCategory, PlatformError};

/// Shared execution error for `whichdb`.
pub type ToolExecutionError = PlatformError;

/// Stable table columns for `whichdb` provider-discovery reports.
pub const WHICHDB_REPORT_COLUMNS: [&str; 5] = [
    "provider",
    "normalized_query",
    "route_label",
    "discovery_status",
    "next_methods",
];

/// Typed parameters for `whichdb` provider discovery.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WhichdbParams {
    /// Provider-qualified accession or identifier query.
    pub query: String,
}

/// Bounded discovery status for one considered provider route.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WhichdbDiscoveryStatus {
    /// The provider has a bounded governed route in the current implementation.
    SupportedProvider,
    /// The provider-qualified query is syntactically valid but no bounded route is shipped.
    UnsupportedProvider,
}

impl WhichdbDiscoveryStatus {
    /// Stable report label for the discovery status.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::SupportedProvider => "supported_provider",
            Self::UnsupportedProvider => "unsupported_provider",
        }
    }
}

/// One deterministic provider-discovery row.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WhichdbDiscoveryRow {
    /// Normalized provider identity considered by the bounded discovery core.
    pub provider: String,
    /// Normalized provider-local query.
    pub normalized_query: String,
    /// Stable provider route label considered for the query.
    pub route_label: String,
    /// Discovery status for the considered route.
    pub status: WhichdbDiscoveryStatus,
    /// Governed retrieval or metadata methods the user should use next.
    pub next_methods: Vec<String>,
}

impl WhichdbDiscoveryRow {
    /// Projects the row into stable report fields.
    #[must_use]
    pub fn report_fields(&self) -> Vec<String> {
        vec![
            self.provider.clone(),
            self.normalized_query.clone(),
            self.route_label.clone(),
            self.status.as_str().to_owned(),
            self.next_methods.join(","),
        ]
    }
}

/// Structured `whichdb` analytical outcome.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WhichdbOutcome {
    /// Normalized provider identity from the query prefix.
    pub provider: String,
    /// Normalized provider-local query from the query suffix.
    pub normalized_query: String,
    /// Deterministic provider-discovery rows.
    pub rows: Vec<WhichdbDiscoveryRow>,
}

impl WhichdbOutcome {
    /// Returns stable report columns for table-first `whichdb` rendering.
    #[must_use]
    pub fn report_columns(&self) -> Vec<String> {
        WHICHDB_REPORT_COLUMNS
            .iter()
            .map(|column| (*column).to_owned())
            .collect()
    }

    /// Projects discovery rows into stable table-first report rows.
    #[must_use]
    pub fn report_rows(&self) -> Vec<Vec<String>> {
        self.rows
            .iter()
            .map(WhichdbDiscoveryRow::report_fields)
            .collect()
    }

    /// Renders a stable tab-separated report suitable for CLI and fixture use.
    #[must_use]
    pub fn render_tsv_report(&self) -> String {
        let mut rendered = self.report_columns().join("\t");
        for row in self.report_rows() {
            rendered.push('\n');
            rendered.push_str(&row.join("\t"));
        }
        rendered
    }
}

/// Returns the bounded `whichdb` help text.
#[must_use]
pub fn whichdb_help() -> &'static str {
    "Usage: emboss-rs whichdb <provider-qualified-query>\n\nReport deterministic bounded provider-discovery rows for one provider-qualified accession or identifier. The v1 seam normalizes the provider prefix and provider-local query, reports the governed retrieval or metadata methods to use next, and does not perform live provider search, payload retrieval, archive download, local file indexing, or database-universe discovery."
}

/// Executes the bounded `whichdb` provider-discovery analytical core.
pub fn run_whichdb(params: WhichdbParams) -> Result<WhichdbOutcome, ToolExecutionError> {
    let (provider, normalized_query) = parse_provider_qualified_query(&params.query)?;
    let route = route_for_provider(&provider);

    Ok(WhichdbOutcome {
        provider: provider.to_owned(),
        normalized_query: normalized_query.to_owned(),
        rows: vec![WhichdbDiscoveryRow {
            provider: provider.to_owned(),
            normalized_query: normalized_query.to_owned(),
            route_label: route.route_label.to_owned(),
            status: route.status,
            next_methods: route
                .next_methods
                .iter()
                .map(|method| (*method).to_owned())
                .collect(),
        }],
    })
}

struct WhichdbRoute {
    route_label: &'static str,
    status: WhichdbDiscoveryStatus,
    next_methods: &'static [&'static str],
}

fn parse_provider_qualified_query(query: &str) -> Result<(String, String), ToolExecutionError> {
    let trimmed = query.trim();
    if trimmed.is_empty() {
        return Err(validation_error(
            "whichdb requires a provider-qualified accession or identifier query",
            "tools.whichdb.query.empty",
        ));
    }

    if trimmed.contains('\n') || trimmed.contains('\r') || trimmed.starts_with('>') {
        return Err(validation_error(
            "whichdb does not accept inline sequence or payload literals",
            "tools.whichdb.query.inline_payload",
        ));
    }

    if looks_like_local_file_reference(trimmed) {
        return Err(validation_error(
            "whichdb does not accept local file references",
            "tools.whichdb.query.local_file",
        ));
    }

    let Some((provider, local_query)) = trimmed.split_once(':') else {
        return Err(validation_error(
            "whichdb requires a provider-qualified query such as ena:AB000263",
            "tools.whichdb.query.unqualified",
        ));
    };

    let provider = provider.trim().to_ascii_lowercase();
    let local_query = local_query.trim().to_owned();

    if provider.is_empty() {
        return Err(validation_error(
            "whichdb requires a non-empty provider prefix",
            "tools.whichdb.provider.empty",
        ));
    }

    if !provider
        .chars()
        .all(|ch| ch.is_ascii_lowercase() || ch.is_ascii_digit() || ch == '-' || ch == '_')
    {
        return Err(validation_error(
            "whichdb provider prefixes must use lower-case ASCII letters, digits, '-' or '_'",
            "tools.whichdb.provider.invalid_format",
        )
        .with_detail(format!("received '{provider}'")));
    }

    if local_query.is_empty() {
        return Err(validation_error(
            "whichdb requires a non-empty provider-local query",
            "tools.whichdb.query.local_empty",
        ));
    }

    if provider == "file" || looks_like_local_file_reference(&local_query) {
        return Err(validation_error(
            "whichdb does not accept local file references",
            "tools.whichdb.query.local_file",
        ));
    }

    Ok((provider, local_query))
}

fn route_for_provider(provider: &str) -> WhichdbRoute {
    match provider {
        "ena" => WhichdbRoute {
            route_label: "ena.sequence-or-archive-discovery",
            status: WhichdbDiscoveryStatus::SupportedProvider,
            next_methods: &["seqret", "runinfo", "runget", "infoassembly"],
        },
        "ncbi" => WhichdbRoute {
            route_label: "ncbi.reference-sequence-discovery",
            status: WhichdbDiscoveryStatus::SupportedProvider,
            next_methods: &["refseqget"],
        },
        "sra" => WhichdbRoute {
            route_label: "sra.archive-discovery",
            status: WhichdbDiscoveryStatus::SupportedProvider,
            next_methods: &["runinfo", "runget", "infoassembly"],
        },
        _ => WhichdbRoute {
            route_label: "unsupported-provider",
            status: WhichdbDiscoveryStatus::UnsupportedProvider,
            next_methods: &[],
        },
    }
}

fn validation_error(message: &'static str, code: &'static str) -> PlatformError {
    PlatformError::new(ErrorCategory::Validation, message).with_code(code)
}

fn looks_like_local_file_reference(query: &str) -> bool {
    query.starts_with('/')
        || query.starts_with("./")
        || query.starts_with("../")
        || query.starts_with("~/")
        || query.starts_with("file:")
}

#[cfg(test)]
mod tests {
    use super::{WHICHDB_REPORT_COLUMNS, WhichdbDiscoveryStatus, WhichdbParams, run_whichdb};

    fn fixture_text(name: &str) -> String {
        std::fs::read_to_string(
            std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
                .join("tests/fixtures")
                .join(name),
        )
        .unwrap_or_else(|error| panic!("fixture {name} should load: {error}"))
    }

    fn fixture_text_without_line_ending(name: &str) -> String {
        let mut text = fixture_text(name);
        if text.ends_with("\r\n") {
            text.truncate(text.len() - 2);
        } else if text.ends_with('\n') {
            text.truncate(text.len() - 1);
        } else {
            panic!("fixture {name} should end with one newline");
        }
        text
    }

    #[test]
    fn normalizes_provider_qualified_ena_query() {
        let outcome = run_whichdb(WhichdbParams {
            query: " ENA:AB000263 ".to_owned(),
        })
        .expect("provider-qualified ENA query should succeed");

        assert_eq!(outcome.provider, "ena");
        assert_eq!(outcome.normalized_query, "AB000263");
        assert_eq!(outcome.rows.len(), 1);
        assert_eq!(outcome.rows[0].provider, "ena");
        assert_eq!(outcome.rows[0].normalized_query, "AB000263");
        assert_eq!(
            outcome.rows[0].route_label,
            "ena.sequence-or-archive-discovery"
        );
        assert_eq!(
            outcome.rows[0].status,
            WhichdbDiscoveryStatus::SupportedProvider
        );
        assert_eq!(
            outcome.rows[0].next_methods,
            vec!["seqret", "runinfo", "runget", "infoassembly"]
        );
        assert_eq!(
            outcome.report_columns(),
            WHICHDB_REPORT_COLUMNS
                .iter()
                .map(|column| (*column).to_owned())
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn keeps_nested_ncbi_database_locator_as_provider_local_query() {
        let outcome = run_whichdb(WhichdbParams {
            query: "ncbi:protein:NP_000537.3".to_owned(),
        })
        .expect("provider-qualified NCBI query should succeed");

        assert_eq!(outcome.provider, "ncbi");
        assert_eq!(outcome.normalized_query, "protein:NP_000537.3");
        assert_eq!(
            outcome.rows[0].route_label,
            "ncbi.reference-sequence-discovery"
        );
        assert_eq!(outcome.rows[0].next_methods, vec!["refseqget"]);
        assert_eq!(
            outcome.render_tsv_report(),
            fixture_text_without_line_ending("whichdb_ncbi_nested_reference_route.tsv")
        );
    }

    #[test]
    fn reports_unsupported_provider_without_fallback_chain() {
        let outcome = run_whichdb(WhichdbParams {
            query: "uniprot:P12345".to_owned(),
        })
        .expect("syntactically valid provider-qualified query should report unsupported provider");

        assert_eq!(outcome.provider, "uniprot");
        assert_eq!(outcome.rows[0].route_label, "unsupported-provider");
        assert_eq!(
            outcome.rows[0].status,
            WhichdbDiscoveryStatus::UnsupportedProvider
        );
        assert!(outcome.rows[0].next_methods.is_empty());
        assert_eq!(
            outcome.rows[0].report_fields(),
            vec![
                "uniprot",
                "P12345",
                "unsupported-provider",
                "unsupported_provider",
                "",
            ]
        );
        assert_eq!(
            outcome.render_tsv_report(),
            fixture_text_without_line_ending("whichdb_unsupported_provider_route.tsv")
        );
    }

    #[test]
    fn renders_stable_table_first_report() {
        let outcome = run_whichdb(WhichdbParams {
            query: "sra:SRR000001".to_owned(),
        })
        .expect("provider-qualified SRA query should succeed");

        assert_eq!(
            outcome.report_rows(),
            vec![vec![
                "sra".to_owned(),
                "SRR000001".to_owned(),
                "sra.archive-discovery".to_owned(),
                "supported_provider".to_owned(),
                "runinfo,runget,infoassembly".to_owned(),
            ]]
        );
        assert_eq!(
            outcome.render_tsv_report(),
            "provider\tnormalized_query\troute_label\tdiscovery_status\tnext_methods\nsra\tSRR000001\tsra.archive-discovery\tsupported_provider\truninfo,runget,infoassembly"
        );
    }

    #[test]
    fn rejects_unqualified_database_universe_query() {
        let error = run_whichdb(WhichdbParams {
            query: "AB000263".to_owned(),
        })
        .expect_err("unqualified queries should fail");

        assert_eq!(error.code(), Some("tools.whichdb.query.unqualified"));
    }

    #[test]
    fn rejects_inline_payload_literals() {
        let error = run_whichdb(WhichdbParams {
            query: ">seq\nACGT".to_owned(),
        })
        .expect_err("inline payloads should fail");

        assert_eq!(error.code(), Some("tools.whichdb.query.inline_payload"));
    }

    #[test]
    fn rejects_local_file_references() {
        let error = run_whichdb(WhichdbParams {
            query: "file:/tmp/query.fasta".to_owned(),
        })
        .expect_err("file scheme references should fail");

        assert_eq!(error.code(), Some("tools.whichdb.query.local_file"));
    }

    #[test]
    fn rejects_provider_local_file_references() {
        let error = run_whichdb(WhichdbParams {
            query: "ena:/tmp/query.fasta".to_owned(),
        })
        .expect_err("provider-local file references should fail");

        assert_eq!(error.code(), Some("tools.whichdb.query.local_file"));
    }

    #[test]
    fn rejects_invalid_provider_prefix() {
        let error = run_whichdb(WhichdbParams {
            query: "ena/http:AB000263".to_owned(),
        })
        .expect_err("invalid provider prefixes should fail");

        assert_eq!(error.code(), Some("tools.whichdb.provider.invalid_format"));
    }
}

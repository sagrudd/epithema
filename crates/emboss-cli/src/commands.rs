//! Command handlers for the `emboss-rs` CLI.

use std::path::Path;

use emboss_docgen::{
    AutodocProcessingSummary, DEFAULT_GENERATED_DOCS_ROOT, GeneratedDocsReport,
    emit_generated_docs_from_path_with_gateway, load_summary_from_path_with_gateway,
};
use emboss_service::EmbossService;

use crate::output;

/// Runs the `list` command against the shared service registry.
pub fn run_list(service: &EmbossService) {
    output::print_tool_list(service);
}

/// Loads, validates, and reports an autodoc contract from a JSON file path.
pub fn run_autodoc(
    service: &EmbossService,
    path: &Path,
    emit_docs: bool,
    docs_output_dir: Option<&Path>,
) -> Result<AutodocProcessingSummary, emboss_docgen::AutodocContractError> {
    let gateway = service.documentation_acquisition();
    let summary =
        load_summary_from_path_with_gateway(path, &service.config().autodoc, Some(&gateway))?;
    output::print_autodoc_summary(&summary, path);
    if emit_docs {
        let report = run_autodoc_emit_docs(service, path, docs_output_dir)?;
        output::print_generated_docs_report(&report);
    }
    Ok(summary)
}

/// Loads an autodoc document and emits generated Markdown pages.
pub fn run_autodoc_emit_docs(
    service: &EmbossService,
    path: &Path,
    docs_output_dir: Option<&Path>,
) -> Result<GeneratedDocsReport, emboss_docgen::AutodocContractError> {
    let output_root = docs_output_dir.unwrap_or_else(|| Path::new(DEFAULT_GENERATED_DOCS_ROOT));
    let gateway = service.documentation_acquisition();
    emit_generated_docs_from_path_with_gateway(
        path,
        output_root,
        &service.config().autodoc,
        Some(&gateway),
    )
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use emboss_service::{EmbossService, ServiceRegistry};

    use super::{run_autodoc, run_autodoc_emit_docs};

    fn fixture(path: &str) -> std::path::PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR")).join(path)
    }

    fn service() -> EmbossService {
        EmbossService::new(ServiceRegistry::new())
    }

    #[test]
    fn autodoc_command_loads_valid_fixture() {
        let summary = run_autodoc(
            &service(),
            &fixture("../emboss-docgen/tests/fixtures/minimal_autodoc.json"),
            false,
            None,
        )
        .expect("fixture should load");
        assert_eq!(summary.tool_name, "needle");
        assert!(summary.valid);
    }

    #[test]
    fn autodoc_command_rejects_missing_fixture() {
        assert!(
            run_autodoc(
                &service(),
                &fixture("tests/fixtures/missing.json"),
                false,
                None
            )
            .is_err()
        );
    }

    #[test]
    fn autodoc_command_can_emit_generated_docs() {
        let output_root = std::env::temp_dir().join(format!(
            "emboss-cli-autodoc-generated-{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("time should advance")
                .as_nanos()
        ));

        let report = run_autodoc_emit_docs(
            &service(),
            &fixture("../emboss-docgen/tests/fixtures/minimal_autodoc.json"),
            Some(&output_root),
        )
        .expect("generated docs emission should succeed");

        assert!(report.tool_page.exists());
        let _ = std::fs::remove_dir_all(&output_root);
    }

    #[test]
    fn autodoc_command_rejects_provider_artifacts_without_real_provider_implementation() {
        let error = run_autodoc(
            &service(),
            &fixture("../emboss-docgen/tests/fixtures/rich_autodoc.json"),
            false,
            None,
        )
        .expect_err("provider-backed fixture should fail through governed gateway");

        assert_eq!(
            error.to_string(),
            "requested documentation provider is not registered: ena"
        );
    }
}

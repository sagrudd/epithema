//! Loading and summary reporting for autodoc contracts.

use std::fs::File;
use std::path::Path;

use emboss_diagnostics::Diagnostic;

use crate::contract::AutodocDocument;
use crate::error::AutodocContractError;

/// Normalized outcome of loading and validating an autodoc contract.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AutodocProcessingSummary {
    /// Schema version declared by the contract.
    pub schema_version: String,
    /// Stable document identifier.
    pub document_id: String,
    /// Documented tool or method name.
    pub tool_name: String,
    /// Number of narrative sections.
    pub section_count: usize,
    /// Number of declared artifacts.
    pub artifact_count: usize,
    /// Number of declared examples.
    pub example_count: usize,
    /// High-level source mode for the contract.
    pub source_mode: String,
    /// Whether semantic validation succeeded.
    pub valid: bool,
    /// Non-fatal diagnostics accumulated during processing.
    pub diagnostics: Vec<Diagnostic>,
}

impl AutodocProcessingSummary {
    /// Builds a summary from a validated autodoc document.
    #[must_use]
    pub fn from_document(document: &AutodocDocument) -> Self {
        Self {
            schema_version: document.schema_version.clone(),
            document_id: document.document_id.clone(),
            tool_name: document.tool.name.clone(),
            section_count: document.sections.len(),
            artifact_count: document.artifacts.len(),
            example_count: document.examples.len(),
            source_mode: format!("{:?}", document.provenance.source_mode).to_ascii_lowercase(),
            valid: true,
            diagnostics: Vec::new(),
        }
    }
}

/// Loads and validates an autodoc document from a JSON file path.
pub fn load_document_from_path(
    path: impl AsRef<Path>,
) -> Result<AutodocDocument, AutodocContractError> {
    let path = path.as_ref();
    let file = File::open(path)
        .map_err(AutodocContractError::from_io)
        .map_err(|error| error.with_path(path))?;
    AutodocDocument::from_json_reader(file)
}

/// Loads, validates, and summarizes an autodoc document from a JSON file path.
pub fn load_summary_from_path(
    path: impl AsRef<Path>,
) -> Result<AutodocProcessingSummary, AutodocContractError> {
    let document = load_document_from_path(path)?;
    Ok(AutodocProcessingSummary::from_document(&document))
}

#[cfg(test)]
mod tests {
    use std::io::Write;
    use std::path::Path;

    use super::{AutodocProcessingSummary, load_document_from_path, load_summary_from_path};

    fn fixture(path: &str) -> std::path::PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR")).join(path)
    }

    #[test]
    fn loads_summary_from_valid_fixture() {
        let summary = load_summary_from_path(fixture("tests/fixtures/minimal_autodoc.json"))
            .expect("fixture should load");

        assert_eq!(summary.tool_name, "needle");
        assert_eq!(summary.artifact_count, 1);
        assert!(summary.valid);
    }

    #[test]
    fn loads_document_from_path() {
        let document = load_document_from_path(fixture("tests/fixtures/rich_autodoc.json"))
            .expect("fixture should load");

        let summary = AutodocProcessingSummary::from_document(&document);
        assert_eq!(summary.example_count, 2);
    }

    #[test]
    fn reports_invalid_json_from_file() {
        let unique = format!(
            "emboss-docgen-invalid-json-{}.json",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("time should move forward")
                .as_nanos()
        );
        let path = std::env::temp_dir().join(unique);
        let mut file = std::fs::File::create(&path).expect("file should be created");
        writeln!(file, "{{ invalid json").expect("invalid json should be written");

        let result = load_summary_from_path(&path);
        let _ = std::fs::remove_file(&path);
        assert!(result.is_err());
    }
}

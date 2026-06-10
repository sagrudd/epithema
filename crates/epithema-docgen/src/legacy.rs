//! Legacy EMBOSS artefact discovery and harvesting.
//!
//! This module discovers and lightly normalizes historical EMBOSS documentation
//! assets from a local source tree. It does not yet derive final autodoc JSON,
//! execute examples, or fetch remote assets. Its role is to preserve source
//! provenance and classify relevant legacy artefacts for later conversion.

use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

use epithema_diagnostics::{
    ArtifactProvenance, Diagnostic, ErrorCategory, PlatformError, Severity,
};

/// Root of a local historical EMBOSS source tree.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LegacyEmbossSourceRoot {
    path: PathBuf,
}

impl LegacyEmbossSourceRoot {
    /// Creates and validates a legacy EMBOSS source root.
    pub fn new(path: impl Into<PathBuf>) -> Result<Self, PlatformError> {
        let path = path.into();

        if !path.exists() {
            return Err(PlatformError::new(
                ErrorCategory::Configuration,
                "legacy EMBOSS source root does not exist",
            )
            .with_code("docgen.legacy.root_missing")
            .with_detail(path.display().to_string()));
        }

        if !path.is_dir() {
            return Err(PlatformError::new(
                ErrorCategory::Configuration,
                "legacy EMBOSS source root must be a directory",
            )
            .with_code("docgen.legacy.root_not_directory")
            .with_detail(path.display().to_string()));
        }

        Ok(Self { path })
    }

    /// Returns the validated filesystem path for the legacy source root.
    #[must_use]
    pub fn path(&self) -> &Path {
        &self.path
    }
}

/// Classified category of a discovered historical artefact.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum LegacyArtifactCategory {
    /// Main application documentation page or narrative source.
    ApplicationDocumentation,
    /// Example-oriented documentation page or example narrative source.
    ExampleDocumentation,
    /// Usage or help text source.
    UsageText,
    /// ACD or similar parameter-definition artefact.
    ParameterDefinition,
    /// Example input, output, or associated support file.
    ExampleData,
}

/// Normalized discovered artefact from the historical EMBOSS tree.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LegacyArtifactRecord {
    /// Tool name associated with the artefact.
    pub tool_name: String,
    /// Classified artefact category.
    pub category: LegacyArtifactCategory,
    /// Path relative to the legacy source root.
    pub relative_path: PathBuf,
    /// Preserved provenance for later autodoc conversion.
    pub provenance: ArtifactProvenance,
    /// Extracted title or primary label when available.
    pub title: Option<String>,
    /// Short extracted content snippet when available.
    pub snippet: Option<String>,
    /// Discovered referenced filenames from the artefact content.
    pub referenced_files: Vec<String>,
}

/// Harvest report for a tool-oriented legacy discovery pass.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LegacyHarvestReport {
    /// Validated legacy source root used for discovery.
    pub source_root: PathBuf,
    /// Requested tool identifier.
    pub tool_name: String,
    /// Discovered and classified artefacts.
    pub artifacts: Vec<LegacyArtifactRecord>,
    /// Non-fatal diagnostics encountered during discovery.
    pub diagnostics: Vec<Diagnostic>,
}

impl LegacyHarvestReport {
    /// Returns the artefacts in the supplied category.
    #[must_use]
    pub fn artifacts_in_category(
        &self,
        category: LegacyArtifactCategory,
    ) -> Vec<&LegacyArtifactRecord> {
        self.artifacts
            .iter()
            .filter(|artifact| artifact.category == category)
            .collect()
    }
}

/// Discovers and classifies legacy EMBOSS artefacts for a named tool.
pub fn discover_legacy_tool_artifacts(
    root: &LegacyEmbossSourceRoot,
    tool_name: &str,
) -> Result<LegacyHarvestReport, PlatformError> {
    let normalized_tool = tool_name.trim().to_ascii_lowercase();

    if normalized_tool.is_empty() {
        return Err(PlatformError::new(
            ErrorCategory::Validation,
            "legacy tool name must not be empty",
        )
        .with_code("docgen.legacy.tool_name.empty"));
    }

    let mut report = LegacyHarvestReport {
        source_root: root.path().to_path_buf(),
        tool_name: normalized_tool.clone(),
        artifacts: Vec::new(),
        diagnostics: Vec::new(),
    };

    walk_directory(root.path(), root.path(), &normalized_tool, &mut report)?;

    if report.artifacts.is_empty() {
        return Err(PlatformError::new(
            ErrorCategory::Validation,
            "no legacy EMBOSS artefacts found for tool",
        )
        .with_code("docgen.legacy.tool_not_found")
        .with_detail(normalized_tool));
    }

    let app_docs = report.artifacts_in_category(LegacyArtifactCategory::ApplicationDocumentation);
    if app_docs.len() > 1 {
        report.diagnostics.push(
            Diagnostic::new(
                Severity::Warning,
                "multiple application documentation artefacts were discovered",
            )
            .with_code("docgen.legacy.multiple_application_docs")
            .with_context(format!(
                "tool '{}' produced {} application documentation candidates",
                report.tool_name,
                app_docs.len()
            )),
        );
    }

    Ok(report)
}

fn walk_directory(
    root: &Path,
    current: &Path,
    tool_name: &str,
    report: &mut LegacyHarvestReport,
) -> Result<(), PlatformError> {
    for entry in fs::read_dir(current).map_err(|error| {
        PlatformError::new(
            ErrorCategory::Configuration,
            "failed to read legacy EMBOSS source directory",
        )
        .with_code("docgen.legacy.read_dir_failed")
        .with_detail(format!("{}: {error}", current.display()))
    })? {
        let entry = entry.map_err(|error| {
            PlatformError::new(
                ErrorCategory::Configuration,
                "failed to enumerate legacy EMBOSS source directory entry",
            )
            .with_code("docgen.legacy.read_dir_entry_failed")
            .with_detail(format!("{}: {error}", current.display()))
        })?;
        let path = entry.path();

        if path.is_dir() {
            walk_directory(root, &path, tool_name, report)?;
            continue;
        }

        let Some(category) = classify_artifact(root, &path, tool_name, report) else {
            continue;
        };

        if !is_text_like(&path) && category != LegacyArtifactCategory::ExampleData {
            report.diagnostics.push(
                Diagnostic::new(
                    Severity::Warning,
                    "unsupported file type encountered in likely legacy artefact location",
                )
                .with_code("docgen.legacy.unsupported_file_type")
                .with_context(path.display().to_string()),
            );
            continue;
        }

        if let Some(record) = build_artifact_record(root, &path, tool_name, category, report)? {
            report.artifacts.push(record);
        }
    }

    Ok(())
}

fn classify_artifact(
    root: &Path,
    path: &Path,
    tool_name: &str,
    report: &mut LegacyHarvestReport,
) -> Option<LegacyArtifactCategory> {
    let relative = path.strip_prefix(root).ok()?;
    let path_lower = relative.to_string_lossy().to_ascii_lowercase();
    let file_name = path.file_name()?.to_string_lossy().to_ascii_lowercase();
    let extension = path
        .extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| ext.to_ascii_lowercase());

    let likely_doc_area = path_lower.contains("/doc/")
        || path_lower.starts_with("doc/")
        || path_lower.contains("/docs/")
        || path_lower.contains("/manual")
        || path_lower.contains("/html/");
    let likely_example_area = path_lower.contains("/examples/")
        || path_lower.starts_with("examples/")
        || path_lower.contains("/example/")
        || path_lower.starts_with("example/")
        || path_lower.contains("/test/")
        || path_lower.starts_with("test/")
        || path_lower.contains("/tests/")
        || path_lower.starts_with("tests/")
        || path_lower.contains("/data/");

    if file_name == format!("{tool_name}.acd")
        || path_lower.contains("/acd/") && file_name.starts_with(tool_name)
    {
        return Some(LegacyArtifactCategory::ParameterDefinition);
    }

    if likely_doc_area && file_name.contains(tool_name) {
        if file_name.contains("usage") || file_name.contains("help") {
            return Some(LegacyArtifactCategory::UsageText);
        }

        if file_name.contains("example") {
            return Some(LegacyArtifactCategory::ExampleDocumentation);
        }

        if matches!(
            extension.as_deref(),
            Some("html" | "htm" | "txt" | "md" | "texi")
        ) {
            return Some(LegacyArtifactCategory::ApplicationDocumentation);
        }
    }

    if likely_example_area
        && (file_name.contains(tool_name)
            || relative.components().any(|component| {
                component
                    .as_os_str()
                    .to_string_lossy()
                    .eq_ignore_ascii_case(tool_name)
            }))
    {
        if !matches!(extension.as_deref(), Some("html" | "htm" | "txt" | "md")) {
            return Some(LegacyArtifactCategory::ExampleData);
        }

        return Some(LegacyArtifactCategory::ExampleDocumentation);
    }

    if likely_doc_area
        && matches!(extension.as_deref(), Some("bin" | "pdf"))
        && file_name.contains(tool_name)
    {
        report.diagnostics.push(
            Diagnostic::new(
                Severity::Warning,
                "binary or unsupported candidate artefact encountered during legacy discovery",
            )
            .with_code("docgen.legacy.binary_candidate")
            .with_context(relative.display().to_string()),
        );
    }

    None
}

fn build_artifact_record(
    root: &Path,
    path: &Path,
    tool_name: &str,
    category: LegacyArtifactCategory,
    report: &mut LegacyHarvestReport,
) -> Result<Option<LegacyArtifactRecord>, PlatformError> {
    let relative = path.strip_prefix(root).map_err(|error| {
        PlatformError::new(
            ErrorCategory::Internal,
            "failed to compute relative legacy artefact path",
        )
        .with_code("docgen.legacy.relative_path_failed")
        .with_detail(error.to_string())
    })?;

    let text = if is_text_like(path) {
        match fs::read_to_string(path) {
            Ok(text) => Some(text),
            Err(error) => {
                report.diagnostics.push(
                    Diagnostic::new(
                        Severity::Warning,
                        "failed to read candidate legacy artefact as text",
                    )
                    .with_code("docgen.legacy.read_text_failed")
                    .with_context(format!("{}: {error}", relative.display())),
                );
                None
            }
        }
    } else {
        None
    };

    let title = text.as_deref().and_then(extract_title);
    let snippet = text.as_deref().and_then(extract_snippet);
    let referenced_files = text
        .as_deref()
        .map(extract_referenced_files)
        .unwrap_or_default()
        .into_iter()
        .collect::<Vec<_>>();

    Ok(Some(LegacyArtifactRecord {
        tool_name: tool_name.to_owned(),
        category,
        relative_path: relative.to_path_buf(),
        provenance: ArtifactProvenance::local_file(relative.display().to_string())
            .with_description("legacy EMBOSS artefact from local source tree"),
        title,
        snippet,
        referenced_files,
    }))
}

fn is_text_like(path: &Path) -> bool {
    matches!(
        path.extension()
            .and_then(|ext| ext.to_str())
            .map(|ext| ext.to_ascii_lowercase())
            .as_deref(),
        Some(
            "html" | "htm" | "txt" | "md" | "texi" | "acd" | "fasta" | "fa" | "dat" | "out" | "in"
        )
    )
}

fn extract_title(text: &str) -> Option<String> {
    let lower = text.to_ascii_lowercase();

    if let (Some(start), Some(end)) = (lower.find("<title>"), lower.find("</title>")) {
        if end > start + 7 {
            return Some(text[start + 7..end].trim().to_owned());
        }
    }

    if let (Some(start), Some(end)) = (lower.find("<h1>"), lower.find("</h1>")) {
        if end > start + 4 {
            return Some(text[start + 4..end].trim().to_owned());
        }
    }

    text.lines()
        .map(str::trim)
        .find(|line| !line.is_empty())
        .map(ToOwned::to_owned)
}

fn extract_snippet(text: &str) -> Option<String> {
    let snippet = text
        .lines()
        .map(str::trim)
        .find(|line| !line.is_empty() && !line.starts_with('<'))?;
    Some(snippet.chars().take(160).collect())
}

fn extract_referenced_files(text: &str) -> BTreeSet<String> {
    text.split(|ch: char| {
        ch.is_whitespace() || matches!(ch, '"' | '\'' | '(' | ')' | '[' | ']' | ',' | ';' | ':')
    })
    .filter_map(|token| {
        let trimmed = token.trim_matches(|ch: char| ch == '.' || ch == '<' || ch == '>');
        let lower = trimmed.to_ascii_lowercase();
        let is_reference = [".fa", ".fasta", ".txt", ".out", ".dat", ".acd", ".html"]
            .iter()
            .any(|suffix| lower.ends_with(suffix));
        if is_reference && !trimmed.is_empty() {
            Some(trimmed.to_owned())
        } else {
            None
        }
    })
    .collect()
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use epithema_diagnostics::Severity;

    use super::{LegacyArtifactCategory, LegacyEmbossSourceRoot, discover_legacy_tool_artifacts};

    fn fixture(path: &str) -> std::path::PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR")).join(path)
    }

    #[test]
    fn discovers_and_classifies_legacy_tool_artifacts() {
        let root = LegacyEmbossSourceRoot::new(fixture("tests/fixtures/legacy_emboss_tree"))
            .expect("valid root");
        let report =
            discover_legacy_tool_artifacts(&root, "needle").expect("harvest should succeed");

        assert!(!report.artifacts.is_empty());
        assert!(
            report
                .artifacts
                .iter()
                .any(|artifact| artifact.category
                    == LegacyArtifactCategory::ApplicationDocumentation)
        );
        assert!(
            report
                .artifacts
                .iter()
                .any(|artifact| artifact.category == LegacyArtifactCategory::ParameterDefinition)
        );
        assert!(
            report
                .artifacts
                .iter()
                .any(|artifact| artifact.category == LegacyArtifactCategory::ExampleData)
        );
    }

    #[test]
    fn reports_missing_tool() {
        let root = LegacyEmbossSourceRoot::new(fixture("tests/fixtures/legacy_emboss_tree"))
            .expect("valid root");
        let error = discover_legacy_tool_artifacts(&root, "nonexistent")
            .expect_err("missing tool should fail");
        assert!(
            error
                .to_string()
                .contains("no legacy EMBOSS artefacts found")
        );
    }

    #[test]
    fn emits_warning_for_multiple_application_docs() {
        let root = LegacyEmbossSourceRoot::new(fixture("tests/fixtures/legacy_emboss_tree"))
            .expect("valid root");
        let report =
            discover_legacy_tool_artifacts(&root, "needle").expect("harvest should succeed");

        assert!(report.diagnostics.iter().any(|diagnostic| {
            diagnostic.severity == Severity::Warning
                && diagnostic
                    .code()
                    .is_some_and(|code| code == "docgen.legacy.multiple_application_docs")
        }));
    }
}

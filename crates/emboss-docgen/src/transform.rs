//! Legacy-to-autodoc transformation.
//!
//! This module converts harvested historical EMBOSS artefacts into the formal
//! versioned autodoc document model. The mapping is intentionally conservative:
//! strongly mappable narrative and artefact content is preserved directly,
//! partially recoverable example structure becomes partial autodoc examples,
//! and ambiguous material remains visible through diagnostics rather than being
//! silently discarded.

use std::collections::{BTreeMap, BTreeSet};

use emboss_diagnostics::{Diagnostic, DiagnosticLocation, ErrorCategory, PlatformError, Severity};

use crate::contract::{
    AcquisitionMethod, ArtifactOrigin, ArtifactReference, ArtifactSpec, AutodocDocument,
    AutodocExample, AutodocNarrativeSection, AutodocProvenance, AutodocSourceMode, LegacyReference,
    NarrativeSectionKind, ToolIdentity, ValidationExpectation,
};
use crate::legacy::{
    LegacyArtifactCategory, LegacyArtifactRecord, LegacyEmbossSourceRoot, LegacyHarvestReport,
    discover_legacy_tool_artifacts,
};

/// Result of converting harvested legacy artefacts into an autodoc document.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct LegacyAutodocTransformReport {
    /// Tool identifier used for the transform.
    pub source_tool_name: String,
    /// Emitted autodoc document identifier.
    pub document_id: String,
    /// Provenance-rich autodoc document.
    pub document: AutodocDocument,
    /// Number of harvested artefacts considered by the transform.
    pub consumed_artifact_count: usize,
    /// Number of narrative sections emitted.
    pub emitted_section_count: usize,
    /// Number of autodoc artefacts emitted.
    pub emitted_artifact_count: usize,
    /// Number of autodoc examples emitted.
    pub emitted_example_count: usize,
    /// Non-fatal diagnostics from harvesting and mapping.
    pub diagnostics: Vec<Diagnostic>,
}

impl LegacyAutodocTransformReport {
    /// Serializes the emitted autodoc document as pretty-printed JSON.
    pub fn document_json_pretty(&self) -> Result<String, PlatformError> {
        serde_json::to_string_pretty(&self.document).map_err(|error| {
            PlatformError::new(
                ErrorCategory::Internal,
                "failed to serialize transformed autodoc document",
            )
            .with_code("docgen.transform.serialize_failed")
            .with_source(error)
        })
    }
}

/// Discovers legacy artefacts for a tool and transforms them into autodoc.
pub fn derive_autodoc_from_legacy_root(
    root: &LegacyEmbossSourceRoot,
    tool_name: &str,
) -> Result<LegacyAutodocTransformReport, PlatformError> {
    let harvest = discover_legacy_tool_artifacts(root, tool_name)?;
    transform_legacy_report(&harvest)
}

/// Converts a harvested legacy report into a validated autodoc document.
pub fn transform_legacy_report(
    report: &LegacyHarvestReport,
) -> Result<LegacyAutodocTransformReport, PlatformError> {
    if report.artifacts.is_empty() {
        return Err(PlatformError::new(
            ErrorCategory::Validation,
            "cannot derive autodoc from an empty legacy harvest report",
        )
        .with_code("docgen.transform.empty_harvest_report")
        .with_detail(report.tool_name.clone()));
    }

    let mut diagnostics = report.diagnostics.clone();
    let mut artifacts = Vec::new();
    let mut artifact_ids_by_file = BTreeMap::new();

    for artifact in &report.artifacts {
        let artifact_id = artifact_id(artifact, &artifact_ids_by_file);
        artifact_ids_by_file.insert(
            artifact
                .relative_path
                .to_string_lossy()
                .to_ascii_lowercase(),
            artifact_id.clone(),
        );
        artifacts.push(ArtifactSpec {
            id: artifact_id,
            label: artifact_label(artifact),
            origin: ArtifactOrigin::LegacyEmbossReference {
                source_label: artifact.relative_path.display().to_string(),
            },
            acquisition: AcquisitionMethod::LegacyHarvest,
            reference: ArtifactReference::Path {
                path: artifact.relative_path.display().to_string(),
            },
            description: artifact_description(artifact),
        });
    }

    let summary = report
        .artifacts
        .iter()
        .find(|artifact| artifact.category == LegacyArtifactCategory::ApplicationDocumentation)
        .and_then(|artifact| artifact.title.clone().or_else(|| artifact.snippet.clone()));

    let source_references = collect_source_references(&report.artifacts);
    let mut document = AutodocDocument::new(
        format!("{}-legacy-derived", report.tool_name),
        ToolIdentity {
            name: report.tool_name.clone(),
            family: None,
            summary,
            legacy_names: vec![report.tool_name.clone()],
        },
        AutodocProvenance {
            source_mode: AutodocSourceMode::LegacyDerived,
            curated_by: None,
            source_references,
        },
    );

    document.sections = build_sections(report, &mut diagnostics);
    document.artifacts = artifacts;
    document.examples = build_examples(report, &artifact_ids_by_file, &mut diagnostics);
    document.validation = Some(ValidationExpectation {
        required_example_ids: document
            .examples
            .iter()
            .map(|example| example.id.clone())
            .collect(),
        compare_against_legacy: true,
        require_provenance_capture: true,
    });
    document.validate()?;

    Ok(LegacyAutodocTransformReport {
        source_tool_name: report.tool_name.clone(),
        document_id: document.document_id.clone(),
        consumed_artifact_count: report.artifacts.len(),
        emitted_section_count: document.sections.len(),
        emitted_artifact_count: document.artifacts.len(),
        emitted_example_count: document.examples.len(),
        document,
        diagnostics,
    })
}

fn build_sections(
    report: &LegacyHarvestReport,
    diagnostics: &mut Vec<Diagnostic>,
) -> Vec<AutodocNarrativeSection> {
    let mut sections = Vec::new();
    let mut emitted_ids = BTreeSet::new();

    for artifact in &report.artifacts {
        let Some(kind) = section_kind_for_category(artifact.category) else {
            continue;
        };

        let Some(content) = section_content(artifact) else {
            diagnostics.push(
                Diagnostic::new(
                    Severity::Notice,
                    "legacy artefact did not yield narrative content",
                )
                .with_code("docgen.transform.section_content_missing")
                .with_context(artifact.relative_path.display().to_string())
                .with_location(DiagnosticLocation::new("legacy_to_autodoc.sections")),
            );
            continue;
        };

        let id = unique_section_id(kind, artifact, &mut emitted_ids);
        sections.push(AutodocNarrativeSection {
            id,
            kind,
            title: section_title(artifact),
            content,
        });
    }

    if sections.is_empty() {
        diagnostics.push(
            Diagnostic::new(
                Severity::Warning,
                "legacy harvest yielded no narrative sections for autodoc",
            )
            .with_code("docgen.transform.no_sections")
            .with_context(report.tool_name.clone())
            .with_location(DiagnosticLocation::new("legacy_to_autodoc.sections")),
        );
    }

    sections
}

fn build_examples(
    report: &LegacyHarvestReport,
    artifact_ids_by_file: &BTreeMap<String, String>,
    diagnostics: &mut Vec<Diagnostic>,
) -> Vec<AutodocExample> {
    let mut examples = Vec::new();

    for (index, artifact) in report
        .artifacts
        .iter()
        .filter(|artifact| artifact.category == LegacyArtifactCategory::ExampleDocumentation)
        .enumerate()
    {
        let mut artifact_ids = Vec::new();
        let mut unresolved = Vec::new();

        for referenced in &artifact.referenced_files {
            let Some(id) = resolve_referenced_artifact_id(report, artifact_ids_by_file, referenced)
            else {
                unresolved.push(referenced.clone());
                continue;
            };

            artifact_ids.push(id);
        }

        if !unresolved.is_empty() {
            diagnostics.push(
                Diagnostic::new(
                    Severity::Warning,
                    "legacy example documentation referenced artefacts that could not be mapped",
                )
                .with_code("docgen.transform.example_reference_unresolved")
                .with_context(format!(
                    "{} -> {}",
                    artifact.relative_path.display(),
                    unresolved.join(", ")
                ))
                .with_location(DiagnosticLocation::new("legacy_to_autodoc.examples")),
            );
        }

        if artifact_ids.is_empty() {
            diagnostics.push(
                Diagnostic::new(
                    Severity::Notice,
                    "legacy example documentation yielded a partial example without resolved artefacts",
                )
                .with_code("docgen.transform.partial_example")
                .with_context(artifact.relative_path.display().to_string())
                .with_location(DiagnosticLocation::new("legacy_to_autodoc.examples")),
            );
        }

        examples.push(AutodocExample {
            id: format!("legacy-example-{}", index + 1),
            title: artifact
                .title
                .clone()
                .unwrap_or_else(|| format!("Legacy {} example", report.tool_name)),
            description: artifact.snippet.clone(),
            artifact_ids,
            parameters: Vec::new(),
            expected_outputs: Vec::new(),
            legacy_reference: Some(artifact_legacy_reference(artifact)),
        });
    }

    examples
}

fn section_kind_for_category(category: LegacyArtifactCategory) -> Option<NarrativeSectionKind> {
    match category {
        LegacyArtifactCategory::ApplicationDocumentation => Some(NarrativeSectionKind::Overview),
        LegacyArtifactCategory::ExampleDocumentation => Some(NarrativeSectionKind::Examples),
        LegacyArtifactCategory::UsageText => Some(NarrativeSectionKind::Notes),
        LegacyArtifactCategory::ParameterDefinition | LegacyArtifactCategory::ExampleData => None,
    }
}

fn section_content(artifact: &LegacyArtifactRecord) -> Option<String> {
    artifact.snippet.clone().or_else(|| artifact.title.clone())
}

fn section_title(artifact: &LegacyArtifactRecord) -> String {
    artifact
        .title
        .clone()
        .unwrap_or_else(|| match artifact.category {
            LegacyArtifactCategory::ApplicationDocumentation => "Legacy overview".to_owned(),
            LegacyArtifactCategory::ExampleDocumentation => "Legacy examples".to_owned(),
            LegacyArtifactCategory::UsageText => "Legacy notes".to_owned(),
            LegacyArtifactCategory::ParameterDefinition => "Legacy parameters".to_owned(),
            LegacyArtifactCategory::ExampleData => "Legacy example artefact".to_owned(),
        })
}

fn artifact_id(
    artifact: &LegacyArtifactRecord,
    existing_by_path: &BTreeMap<String, String>,
) -> String {
    let candidate = format!(
        "legacy-{}",
        slugify(&artifact.relative_path.to_string_lossy())
    );

    if !existing_by_path
        .values()
        .any(|existing| existing == &candidate)
    {
        return candidate;
    }

    format!("{}-{}", candidate, slugify(&artifact.category.sort_key()))
}

fn artifact_label(artifact: &LegacyArtifactRecord) -> String {
    artifact
        .title
        .clone()
        .or_else(|| artifact.provenance.description().map(ToOwned::to_owned))
        .unwrap_or_else(|| artifact.relative_path.display().to_string())
}

fn artifact_description(artifact: &LegacyArtifactRecord) -> Option<String> {
    artifact
        .snippet
        .clone()
        .or_else(|| artifact.title.clone())
        .or_else(|| artifact.provenance.description().map(ToOwned::to_owned))
}

fn artifact_legacy_reference(artifact: &LegacyArtifactRecord) -> LegacyReference {
    LegacyReference {
        source: artifact_label(artifact),
        locator: Some(artifact.relative_path.display().to_string()),
        invocation: None,
    }
}

fn unique_section_id(
    kind: NarrativeSectionKind,
    artifact: &LegacyArtifactRecord,
    emitted_ids: &mut BTreeSet<String>,
) -> String {
    let prefix = match kind {
        NarrativeSectionKind::Overview => "overview",
        NarrativeSectionKind::Inputs => "inputs",
        NarrativeSectionKind::Outputs => "outputs",
        NarrativeSectionKind::Examples => "examples",
        NarrativeSectionKind::LegacyContext => "legacy-context",
        NarrativeSectionKind::Caveats => "caveats",
        NarrativeSectionKind::Notes => "notes",
    };

    let base = format!(
        "{prefix}-{}",
        slugify(&artifact.relative_path.to_string_lossy())
    );
    if emitted_ids.insert(base.clone()) {
        return base;
    }

    let mut ordinal = 2usize;
    loop {
        let candidate = format!("{base}-{ordinal}");
        if emitted_ids.insert(candidate.clone()) {
            return candidate;
        }
        ordinal += 1;
    }
}

fn slugify(value: &str) -> String {
    let slug: String = value
        .chars()
        .map(|character| {
            if character.is_ascii_alphanumeric() {
                character.to_ascii_lowercase()
            } else {
                '-'
            }
        })
        .collect();

    slug.trim_matches('-')
        .split('-')
        .filter(|segment| !segment.is_empty())
        .collect::<Vec<_>>()
        .join("-")
}

fn collect_source_references(artifacts: &[LegacyArtifactRecord]) -> Vec<LegacyReference> {
    let mut seen = BTreeSet::new();
    let mut references = Vec::new();

    for artifact in artifacts {
        let locator = artifact.relative_path.display().to_string();
        if !seen.insert(locator.clone()) {
            continue;
        }

        references.push(LegacyReference {
            source: artifact_label(artifact),
            locator: Some(locator),
            invocation: None,
        });
    }

    references
}

fn resolve_referenced_artifact_id(
    report: &LegacyHarvestReport,
    artifact_ids_by_file: &BTreeMap<String, String>,
    referenced: &str,
) -> Option<String> {
    let referenced_lower = referenced.to_ascii_lowercase();

    if let Some(id) = artifact_ids_by_file.get(&referenced_lower) {
        return Some(id.clone());
    }

    report
        .artifacts
        .iter()
        .find(|artifact| {
            artifact
                .relative_path
                .file_name()
                .and_then(|file_name| file_name.to_str())
                .map(|file_name| file_name.eq_ignore_ascii_case(referenced))
                .unwrap_or(false)
        })
        .and_then(|artifact| {
            artifact_ids_by_file
                .get(
                    &artifact
                        .relative_path
                        .to_string_lossy()
                        .to_ascii_lowercase(),
                )
                .cloned()
        })
}

trait LegacyArtifactCategoryExt {
    fn sort_key(self) -> &'static str;
}

impl LegacyArtifactCategoryExt for LegacyArtifactCategory {
    fn sort_key(self) -> &'static str {
        match self {
            LegacyArtifactCategory::ApplicationDocumentation => "application_documentation",
            LegacyArtifactCategory::ExampleDocumentation => "example_documentation",
            LegacyArtifactCategory::UsageText => "usage_text",
            LegacyArtifactCategory::ParameterDefinition => "parameter_definition",
            LegacyArtifactCategory::ExampleData => "example_data",
        }
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use crate::{
        AutodocDocument, LegacyEmbossSourceRoot, discover_legacy_tool_artifacts,
        transform_legacy_report,
    };

    use super::derive_autodoc_from_legacy_root;

    fn legacy_fixture_root() -> LegacyEmbossSourceRoot {
        let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/legacy_emboss_tree");
        LegacyEmbossSourceRoot::new(path).expect("fixture root should be valid")
    }

    #[test]
    fn transforms_harvested_legacy_report_into_valid_autodoc() {
        let root = legacy_fixture_root();
        let harvest = discover_legacy_tool_artifacts(&root, "needle").expect("harvest should work");
        let report = transform_legacy_report(&harvest).expect("transform should work");

        assert_eq!(report.source_tool_name, "needle");
        assert_eq!(report.document.tool.name, "needle");
        assert_eq!(
            report.document.provenance.source_mode,
            crate::AutodocSourceMode::LegacyDerived
        );
        assert!(!report.document.sections.is_empty());
        assert!(!report.document.artifacts.is_empty());
        assert_eq!(report.document.examples.len(), 1);
        assert!(
            report
                .document
                .provenance
                .source_references
                .iter()
                .any(|reference| reference.locator.as_deref()
                    == Some("doc/programs/html/needle.html"))
        );
        assert!(
            report
                .diagnostics
                .iter()
                .any(|diagnostic| diagnostic.code()
                    == Some("docgen.legacy.multiple_application_docs"))
        );
    }

    #[test]
    fn derived_autodoc_json_round_trips_through_contract_validation() {
        let root = legacy_fixture_root();
        let report =
            derive_autodoc_from_legacy_root(&root, "needle").expect("derivation should work");
        let json = report
            .document_json_pretty()
            .expect("document should serialize");
        let reparsed = AutodocDocument::from_json_str(&json).expect("json should validate");

        assert_eq!(reparsed.document_id, "needle-legacy-derived");
        assert_eq!(reparsed.examples.len(), 1);
    }

    #[test]
    fn empty_harvest_report_is_rejected() {
        let report = crate::LegacyHarvestReport {
            source_root: Path::new("/tmp/legacy").to_path_buf(),
            tool_name: "needle".to_owned(),
            artifacts: Vec::new(),
            diagnostics: Vec::new(),
        };

        let error = transform_legacy_report(&report).expect_err("empty report should fail");
        assert_eq!(error.code(), Some("docgen.transform.empty_harvest_report"));
    }
}

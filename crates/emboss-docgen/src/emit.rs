//! Markdown emission for validated autodoc documents.
//!
//! This module renders deterministic Markdown pages under a generated docs
//! subtree so Sphinx can consume validated autodoc content alongside the
//! hand-authored governance and development documentation.

use std::fs;
use std::path::{Path, PathBuf};

use emboss_diagnostics::{Diagnostic, ErrorCategory, PlatformError};

use crate::contract::{AcquisitionMethod, ArtifactOrigin, ArtifactReference, AutodocDocument};

/// Stable output root for generated documentation pages within `docs/`.
pub const DEFAULT_GENERATED_DOCS_ROOT: &str = "docs/generated";

/// Outcome of emitting generated documentation pages for a single autodoc document.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct GeneratedDocsReport {
    /// Root directory where generated documentation was written.
    pub output_root: PathBuf,
    /// Page path for the documented tool relative to the repository root.
    pub tool_page: PathBuf,
    /// Generated landing page relative to the repository root.
    pub index_page: PathBuf,
    /// Stable tool slug used in the generated path.
    pub tool_slug: String,
    /// Count of narrative sections rendered.
    pub section_count: usize,
    /// Count of declared artifacts rendered.
    pub artifact_count: usize,
    /// Count of declared examples rendered.
    pub example_count: usize,
    /// Non-fatal transform or generation diagnostics included in the rendered output.
    pub diagnostic_count: usize,
}

/// Emits a validated autodoc document as generated Markdown under the target docs root.
pub fn emit_generated_docs(
    document: &AutodocDocument,
    diagnostics: &[Diagnostic],
    output_root: impl AsRef<Path>,
) -> Result<GeneratedDocsReport, PlatformError> {
    document.validate()?;

    let output_root = output_root.as_ref();
    let tool_slug = slugify(&document.tool.name);
    let tools_dir = output_root.join("tools");
    let tool_page = tools_dir.join(format!("{tool_slug}.md"));
    let index_page = output_root.join("index.md");

    fs::create_dir_all(&tools_dir).map_err(|error| {
        PlatformError::new(
            ErrorCategory::Configuration,
            "failed to create generated docs output directory",
        )
        .with_code("docgen.emit.create_dir_failed")
        .with_detail(format!("{}: {error}", tools_dir.display()))
    })?;

    fs::write(&tool_page, render_tool_page(document, diagnostics)).map_err(|error| {
        PlatformError::new(
            ErrorCategory::Configuration,
            "failed to write generated tool documentation page",
        )
        .with_code("docgen.emit.write_tool_page_failed")
        .with_detail(format!("{}: {error}", tool_page.display()))
    })?;

    regenerate_index(output_root).map_err(|error| {
        PlatformError::new(
            ErrorCategory::Configuration,
            "failed to regenerate generated docs index",
        )
        .with_code("docgen.emit.write_index_failed")
        .with_detail(format!("{}: {error}", index_page.display()))
    })?;

    Ok(GeneratedDocsReport {
        output_root: output_root.to_path_buf(),
        tool_page,
        index_page,
        tool_slug,
        section_count: document.sections.len(),
        artifact_count: document.artifacts.len(),
        example_count: document.examples.len(),
        diagnostic_count: diagnostics.len(),
    })
}

fn regenerate_index(output_root: &Path) -> Result<(), std::io::Error> {
    let tools_dir = output_root.join("tools");
    let mut pages = Vec::new();

    if tools_dir.exists() {
        for entry in fs::read_dir(&tools_dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.extension().and_then(|ext| ext.to_str()) != Some("md") {
                continue;
            }
            let stem = path
                .file_stem()
                .and_then(|stem| stem.to_str())
                .unwrap_or_default()
                .to_owned();
            pages.push(stem);
        }
    }

    pages.sort();

    let mut content = String::new();
    content.push_str("# Generated Tool Documentation\n\n");
    content.push_str(
        "This section contains Markdown pages generated from validated `emboss-rs autodoc` inputs. ",
    );
    content.push_str(
        "These files are deterministic documentation source artefacts intended for Sphinx ingestion.\n\n",
    );
    content.push_str("## Contents\n\n");
    content.push_str("```{toctree}\n:maxdepth: 1\n:caption: Generated Tools\n\n");

    for page in &pages {
        content.push_str(&format!("tools/{page}\n"));
    }

    content.push_str("```\n");
    fs::write(output_root.join("index.md"), content)
}

fn render_tool_page(document: &AutodocDocument, diagnostics: &[Diagnostic]) -> String {
    let mut content = String::new();

    content.push_str(&format!("# {}\n\n", document.tool.name));
    content.push_str(render_generation_notice(document.provenance.source_mode));
    content.push_str("\n\n");

    if let Some(summary) = &document.tool.summary {
        content.push_str("## Summary\n\n");
        content.push_str(summary);
        content.push_str("\n\n");
    }

    content.push_str("## Document Metadata\n\n");
    content.push_str(&format!("- Document ID: `{}`\n", document.document_id));
    content.push_str(&format!(
        "- Schema version: `{}`\n",
        document.schema_version
    ));
    content.push_str(&format!(
        "- Source mode: `{}`\n",
        source_mode_label(document)
    ));
    if let Some(family) = &document.tool.family {
        content.push_str(&format!("- Tool family: `{family}`\n"));
    }
    if !document.tool.legacy_names.is_empty() {
        content.push_str(&format!(
            "- Legacy names: `{}`\n",
            document.tool.legacy_names.join("`, `")
        ));
    }
    content.push('\n');

    for section in &document.sections {
        content.push_str(&format!("## {}\n\n", section.title));
        content.push_str(&section.content);
        content.push_str("\n\n");
    }

    content.push_str("## Declared Artifacts\n\n");
    if document.artifacts.is_empty() {
        content.push_str("No artifacts are declared for this autodoc document.\n\n");
    } else {
        for artifact in &document.artifacts {
            content.push_str(&format!("### {}\n\n", artifact.label));
            content.push_str(&format!("- Artifact ID: `{}`\n", artifact.id));
            content.push_str(&format!(
                "- Origin: {}\n",
                format_artifact_origin(&artifact.origin)
            ));
            content.push_str(&format!(
                "- Acquisition: {}\n",
                format_acquisition_method(&artifact.acquisition)
            ));
            content.push_str(&format!(
                "- Reference: {}\n",
                format_artifact_reference(&artifact.reference)
            ));
            if let Some(description) = &artifact.description {
                content.push_str(&format!("- Notes: {}\n", description.trim()));
            }
            content.push('\n');
        }
    }

    content.push_str("## Declared Examples\n\n");
    if document.examples.is_empty() {
        content.push_str("No examples are declared for this autodoc document.\n\n");
    } else {
        for example in &document.examples {
            content.push_str(&format!("### {}\n\n", example.title));
            content.push_str(&format!("- Example ID: `{}`\n", example.id));
            if let Some(description) = &example.description {
                content.push_str(&format!("- Description: {}\n", description.trim()));
            }
            if example.artifact_ids.is_empty() {
                content.push_str("- Referenced artifacts: none declared\n");
            } else {
                content.push_str(&format!(
                    "- Referenced artifacts: `{}`\n",
                    example.artifact_ids.join("`, `")
                ));
            }
            if !example.parameters.is_empty() {
                content.push_str("- Parameters:\n");
                for parameter in &example.parameters {
                    content.push_str(&format!(
                        "  - `{}` = `{}`\n",
                        parameter.name, parameter.value
                    ));
                }
            }
            if !example.expected_outputs.is_empty() {
                content.push_str("- Expected outputs:\n");
                for output in &example.expected_outputs {
                    content.push_str(&format!("  - `{}`: {}", output.id, output.label));
                    if let Some(description) = &output.description {
                        content.push_str(&format!(" ({})", description.trim()));
                    }
                    content.push('\n');
                }
            }
            if let Some(reference) = &example.legacy_reference {
                content.push_str(&format!("- Legacy reference: {}\n", reference.source));
                if let Some(locator) = &reference.locator {
                    content.push_str(&format!("  - Locator: `{locator}`\n"));
                }
                if let Some(invocation) = &reference.invocation {
                    content.push_str(&format!("  - Invocation: `{invocation}`\n"));
                }
            }
            content.push('\n');
        }
    }

    content.push_str("## Provenance\n\n");
    if let Some(curated_by) = &document.provenance.curated_by {
        content.push_str(&format!(
            "- {}: {}\n",
            provenance_label(document.provenance.source_mode),
            curated_by.trim()
        ));
    }
    if document.provenance.source_references.is_empty() {
        content.push_str("- Source references: none declared\n");
    } else {
        content.push_str("- Source references:\n");
        for reference in &document.provenance.source_references {
            content.push_str(&format!("  - {}", reference.source));
            if let Some(locator) = &reference.locator {
                content.push_str(&format!(" (`{locator}`)"));
            }
            content.push('\n');
        }
    }
    content.push('\n');

    if !diagnostics.is_empty() {
        content.push_str("## Transformation Notes\n\n");
        for diagnostic in diagnostics {
            content.push_str(&format!(
                "- `{}`: {}",
                severity_label(diagnostic),
                diagnostic.message()
            ));
            if let Some(code) = diagnostic.code() {
                content.push_str(&format!(" (`{code}`)"));
            }
            if let Some(context) = diagnostic.context() {
                content.push_str(&format!(" - {}", context.trim()));
            }
            content.push('\n');
        }
        content.push('\n');
    }

    if let Some(validation) = &document.validation {
        content.push_str("## Validation Intent\n\n");
        if validation.required_example_ids.is_empty() {
            content.push_str("- Required examples: none declared\n");
        } else {
            content.push_str(&format!(
                "- Required examples: `{}`\n",
                validation.required_example_ids.join("`, `")
            ));
        }
        content.push_str(&format!(
            "- Compare against legacy: {}\n",
            yes_no(validation.compare_against_legacy)
        ));
        content.push_str(&format!(
            "- Require provenance capture: {}\n\n",
            yes_no(validation.require_provenance_capture)
        ));
    }

    content
}

fn render_generation_notice(source_mode: crate::AutodocSourceMode) -> &'static str {
    match source_mode {
        crate::AutodocSourceMode::RegistryStub => {
            "> Generated from a registry-backed autodoc stub. Edit or replace the source autodoc document rather than this page."
        }
        crate::AutodocSourceMode::Curated
        | crate::AutodocSourceMode::LegacyDerived
        | crate::AutodocSourceMode::Mixed => {
            "> Generated from validated autodoc input. Edit the source autodoc document rather than this page."
        }
    }
}

fn provenance_label(source_mode: crate::AutodocSourceMode) -> &'static str {
    match source_mode {
        crate::AutodocSourceMode::RegistryStub => "Stub generated by",
        crate::AutodocSourceMode::Curated => "Curated by",
        crate::AutodocSourceMode::LegacyDerived => "Derived by",
        crate::AutodocSourceMode::Mixed => "Maintained by",
    }
}

fn source_mode_label(document: &AutodocDocument) -> &'static str {
    match document.provenance.source_mode {
        crate::AutodocSourceMode::RegistryStub => "registry-stub",
        crate::AutodocSourceMode::Curated => "curated",
        crate::AutodocSourceMode::LegacyDerived => "legacy-derived",
        crate::AutodocSourceMode::Mixed => "mixed",
    }
}

fn format_artifact_origin(origin: &ArtifactOrigin) -> String {
    match origin {
        ArtifactOrigin::LocalFile => "local file".to_owned(),
        ArtifactOrigin::AccessionedResource => "accessioned resource".to_owned(),
        ArtifactOrigin::FixtureAsset => "fixture asset".to_owned(),
        ArtifactOrigin::GeneratedArtifact => "generated artifact".to_owned(),
        ArtifactOrigin::LegacyEmbossReference { source_label } => {
            format!("legacy EMBOSS reference ({source_label})")
        }
        ArtifactOrigin::Other { label } => format!("other ({label})"),
    }
}

fn format_acquisition_method(method: &AcquisitionMethod) -> String {
    match method {
        AcquisitionMethod::LocalPath => "local path".to_owned(),
        AcquisitionMethod::Provider {
            intent,
            preferred_provider,
        } => match preferred_provider {
            Some(provider) => format!(
                "provider ({}; preferred {})",
                format_resolution_intent(*intent),
                provider
            ),
            None => format!("provider ({})", format_resolution_intent(*intent)),
        },
        AcquisitionMethod::Fixture => "fixture".to_owned(),
        AcquisitionMethod::LegacyHarvest => "legacy harvest".to_owned(),
        AcquisitionMethod::Generated => "generated".to_owned(),
        AcquisitionMethod::Manual => "manual".to_owned(),
    }
}

fn format_artifact_reference(reference: &ArtifactReference) -> String {
    match reference {
        ArtifactReference::Path { path } => format!("path `{path}`"),
        ArtifactReference::Accession { accession } => format!("accession `{accession}`"),
        ArtifactReference::ProviderLocator { provider, locator } => match provider {
            Some(provider) => format!("provider `{provider}` locator `{locator}`"),
            None => format!("provider locator `{locator}`"),
        },
        ArtifactReference::ManagedAsset { asset_id } => format!("managed asset `{asset_id}`"),
        ArtifactReference::Generated { locator } => format!("generated locator `{locator}`"),
    }
}

fn severity_label(diagnostic: &Diagnostic) -> &'static str {
    match diagnostic.severity {
        emboss_diagnostics::Severity::Error => "error",
        emboss_diagnostics::Severity::Warning => "warning",
        emboss_diagnostics::Severity::Notice => "notice",
    }
}

fn format_resolution_intent(intent: crate::ResolutionIntentModel) -> &'static str {
    match intent {
        crate::ResolutionIntentModel::SequenceInput => "sequence input",
        crate::ResolutionIntentModel::MetadataLookup => "metadata lookup",
        crate::ResolutionIntentModel::ArchiveAsset => "archive asset",
        crate::ResolutionIntentModel::DocumentationAsset => "documentation asset",
    }
}

fn yes_no(value: bool) -> &'static str {
    if value { "yes" } else { "no" }
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

#[cfg(test)]
mod tests {
    use std::path::Path;

    use crate::{
        LegacyEmbossSourceRoot, build_stub_document, derive_autodoc_from_legacy_root,
        emit_generated_docs, load_document_from_path,
    };
    use emboss_tools::governed_tool_descriptors;

    use super::{DEFAULT_GENERATED_DOCS_ROOT, slugify};

    fn fixture(path: &str) -> std::path::PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR")).join(path)
    }

    #[test]
    fn emits_page_for_minimal_curated_document() {
        let document = load_document_from_path(fixture("tests/fixtures/minimal_autodoc.json"))
            .expect("fixture should load");
        let output_root = std::env::temp_dir().join(format!(
            "emboss-docgen-generated-{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("time should advance")
                .as_nanos()
        ));

        let report =
            emit_generated_docs(&document, &[], &output_root).expect("generation should succeed");
        let tool_page = std::fs::read_to_string(&report.tool_page).expect("tool page should exist");
        let index_page =
            std::fs::read_to_string(&report.index_page).expect("index page should exist");

        assert!(tool_page.contains("# needle"));
        assert!(tool_page.contains("## Declared Artifacts"));
        assert!(tool_page.contains("## Declared Examples"));
        assert!(index_page.contains("tools/needle"));

        let _ = std::fs::remove_dir_all(&output_root);
    }

    #[test]
    fn emits_page_for_legacy_derived_document_with_notes() {
        let root = LegacyEmbossSourceRoot::new(fixture("tests/fixtures/legacy_emboss_tree"))
            .expect("fixture root should be valid");
        let transform =
            derive_autodoc_from_legacy_root(&root, "needle").expect("transform should succeed");
        let output_root = std::env::temp_dir().join(format!(
            "emboss-docgen-generated-legacy-{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("time should advance")
                .as_nanos()
        ));

        let report = emit_generated_docs(&transform.document, &transform.diagnostics, &output_root)
            .expect("generation should succeed");
        let tool_page = std::fs::read_to_string(&report.tool_page).expect("tool page should exist");

        assert!(tool_page.contains("legacy-derived"));
        assert!(tool_page.contains("## Transformation Notes"));
        assert!(tool_page.contains("docgen.legacy.multiple_application_docs"));

        let _ = std::fs::remove_dir_all(&output_root);
    }

    #[test]
    fn emits_stub_page_with_explicit_registry_stub_provenance() {
        let descriptor = governed_tool_descriptors()
            .iter()
            .copied()
            .find(|descriptor| descriptor.name == "aligncopy")
            .expect("aligncopy descriptor should exist");
        let document = build_stub_document(descriptor);
        let output_root = std::env::temp_dir().join(format!(
            "emboss-docgen-generated-stub-{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("time should advance")
                .as_nanos()
        ));

        let report =
            emit_generated_docs(&document, &[], &output_root).expect("generation should succeed");
        let tool_page = std::fs::read_to_string(&report.tool_page).expect("tool page should exist");

        assert!(tool_page.contains("Generated from a registry-backed autodoc stub"));
        assert!(tool_page.contains("- Source mode: `registry-stub`"));
        assert!(tool_page.contains("- Stub generated by: emboss-rs autodoc stub generator"));

        let _ = std::fs::remove_dir_all(&output_root);
    }

    #[test]
    fn slugifies_tool_names_for_stable_paths() {
        assert_eq!(slugify("needle"), "needle");
        assert_eq!(slugify("needle.legacy"), "needle-legacy");
    }

    #[test]
    fn default_generated_root_matches_docs_subtree() {
        assert_eq!(DEFAULT_GENERATED_DOCS_ROOT, "docs/generated");
    }
}

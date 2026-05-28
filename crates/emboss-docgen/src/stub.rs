//! Deterministic autodoc stub generation for the exposed tool registry.
//!
//! These helpers exist to keep the generated documentation surface aligned with
//! the actual governed tool registry even when a tool does not yet have rich
//! harvested historical material or executable examples.

use std::fs;
use std::path::{Path, PathBuf};

use emboss_diagnostics::{ErrorCategory, PlatformError};
use emboss_tools::{ToolDescriptor, governed_tool_descriptors};

use crate::contract::{
    AcquisitionMethod, ArtifactOrigin, ArtifactReference, ArtifactSpec, AutodocDocument,
    AutodocExample, AutodocExampleOutput, AutodocNarrativeSection, AutodocProvenance,
    AutodocSourceMode, ExampleParameter, NarrativeSectionKind, ToolIdentity, ValidationExpectation,
};

/// Stable repository path for committed autodoc inputs.
pub const DEFAULT_AUTODOC_STUBS_ROOT: &str = "docs/autodoc/tools";

/// Builds one deterministic autodoc document for an exposed tool descriptor.
#[must_use]
pub fn build_stub_document(descriptor: ToolDescriptor) -> AutodocDocument {
    if descriptor.name == "needle" {
        return build_needle_document(descriptor);
    }

    let mut document = AutodocDocument::new(
        format!("{}-stub-v1", descriptor.name),
        ToolIdentity {
            name: descriptor.name.to_owned(),
            family: Some(descriptor.family.to_owned()),
            summary: Some(capitalized_summary(descriptor.summary)),
            legacy_names: vec![descriptor.name.to_owned()],
        },
        AutodocProvenance {
            source_mode: AutodocSourceMode::RegistryStub,
            curated_by: Some("emboss-rs autodoc stub generator".to_owned()),
            source_references: Vec::new(),
        },
    );

    document.sections = vec![
        section(
            "overview",
            NarrativeSectionKind::Overview,
            "Overview",
            format!(
                "`{}` is part of the exposed EMBOSS-RS `{}` cohort. This page is a generated baseline documentation stub produced through the governed autodoc path so the shipped tool surface remains fully documented even where richer harvested narrative or executable examples are still pending.",
                descriptor.name, descriptor.family
            ),
        ),
        section(
            "inputs",
            NarrativeSectionKind::Inputs,
            "Inputs",
            input_summary(descriptor),
        ),
        section(
            "outputs",
            NarrativeSectionKind::Outputs,
            "Outputs",
            output_summary(descriptor),
        ),
        section(
            "status",
            NarrativeSectionKind::Notes,
            "Current Status",
            format!(
                "This method is implemented and exposed through `emboss-rs {}`. The generated tool page and the machine-readable validation stub at [`../validation/{}.validation.json`](../validation/{}.validation.json) are current. No richer autodoc examples are declared in this contract yet; future prompts should replace or extend this stub with harvested or executable evidence rather than hand-maintaining the generated page directly.",
                descriptor.name, descriptor.name, descriptor.name
            ),
        ),
        section(
            "caveats",
            NarrativeSectionKind::Caveats,
            "Caveats",
            "Baseline stub coverage documents the exposed command surface and links to available validation evidence, but it does not imply that all historical EMBOSS examples, rendered screenshots, or legacy comparisons have been captured yet.".to_owned(),
        ),
    ];

    document
}

/// Builds deterministic stub documents for the full governed tool registry.
#[must_use]
pub fn build_stub_catalog() -> Vec<AutodocDocument> {
    governed_tool_descriptors()
        .iter()
        .copied()
        .map(build_stub_document)
        .collect()
}

/// Writes one JSON autodoc contract per exposed tool into the target directory.
pub fn write_stub_catalog(output_root: impl AsRef<Path>) -> Result<Vec<PathBuf>, PlatformError> {
    let output_root = output_root.as_ref();
    fs::create_dir_all(output_root).map_err(|error| {
        PlatformError::new(
            ErrorCategory::Configuration,
            "failed to create autodoc stub output directory",
        )
        .with_code("docgen.stub.create_dir_failed")
        .with_detail(format!("{}: {error}", output_root.display()))
    })?;

    let mut paths = Vec::new();
    for document in build_stub_catalog() {
        let path = output_root.join(format!("{}.json", document.tool.name));
        let json = serde_json::to_string_pretty(&document).map_err(|error| {
            PlatformError::new(
                ErrorCategory::Internal,
                "failed to serialize autodoc stub document",
            )
            .with_code("docgen.stub.serialize_failed")
            .with_detail(format!("{}: {error}", document.tool.name))
        })?;
        fs::write(&path, format!("{json}\n")).map_err(|error| {
            PlatformError::new(
                ErrorCategory::Configuration,
                "failed to write autodoc stub document",
            )
            .with_code("docgen.stub.write_failed")
            .with_detail(format!("{}: {error}", path.display()))
        })?;
        paths.push(path);
    }

    paths.sort();
    Ok(paths)
}

fn build_needle_document(descriptor: ToolDescriptor) -> AutodocDocument {
    let mut document = AutodocDocument::new(
        "needle-minimal",
        ToolIdentity {
            name: descriptor.name.to_owned(),
            family: Some(descriptor.family.to_owned()),
            summary: Some("Global pairwise sequence alignment".to_owned()),
            legacy_names: vec!["needle".to_owned()],
        },
        AutodocProvenance {
            source_mode: AutodocSourceMode::Curated,
            curated_by: Some("emboss-rs maintainers".to_owned()),
            source_references: Vec::new(),
        },
    );

    document.sections = vec![section(
        "overview",
        NarrativeSectionKind::Overview,
        "Overview",
        "Needle computes a global alignment between two sequences.".to_owned(),
    )];
    document.artifacts = vec![ArtifactSpec {
        id: "example_fasta".to_owned(),
        label: "Example FASTA input".to_owned(),
        origin: ArtifactOrigin::FixtureAsset,
        acquisition: AcquisitionMethod::Fixture,
        reference: ArtifactReference::ManagedAsset {
            asset_id: "fixtures/needle/example_fasta".to_owned(),
        },
        description: Some("Repository-managed fixture for a minimal example.".to_owned()),
    }];
    document.examples = vec![AutodocExample {
        id: "basic_alignment".to_owned(),
        title: "Basic alignment example".to_owned(),
        description: Some("Demonstrates a single alignment run.".to_owned()),
        artifact_ids: vec!["example_fasta".to_owned()],
        parameters: vec![ExampleParameter {
            name: "gapopen".to_owned(),
            value: "10".to_owned(),
        }],
        expected_outputs: vec![AutodocExampleOutput {
            id: "report".to_owned(),
            label: "Alignment report".to_owned(),
            description: Some("Placeholder for the future rendered report.".to_owned()),
        }],
        legacy_reference: None,
    }];
    document.validation = Some(ValidationExpectation {
        required_example_ids: vec!["basic_alignment".to_owned()],
        compare_against_legacy: false,
        require_provenance_capture: true,
    });
    document
}

fn section(
    id: &str,
    kind: NarrativeSectionKind,
    title: &str,
    content: String,
) -> AutodocNarrativeSection {
    AutodocNarrativeSection {
        id: id.to_owned(),
        kind,
        title: title.to_owned(),
        content,
    }
}

fn input_summary(descriptor: ToolDescriptor) -> String {
    match descriptor.family {
        "archive_tools" => "This archive-facing tool accepts one governed ENA/SRA-style archive accession through the shared input-resolution seam. Provider qualification and bare-accession behaviour follow the documented archive acquisition policy for the implemented v1 subset.".to_owned(),
        "retrieval_tools" => "This retrieval tool accepts one governed input reference. Depending on the method, that may be a local sequence file, a provider-qualified accession, or a conservatively inferable accession accepted by the shared acquisition seam.".to_owned(),
        "alignment_tools" | "alignment_analysis" | "pairwise_alignment" => "This tool accepts alignment or sequence inputs through the shared alignment and sequence IO layers. Exact parameter shape remains governed by the implemented CLI/service definition for the method.".to_owned(),
        "feature_tools" => "This tool accepts annotated sequence records through the shared IO layer and operates on simple feature spans preserved in EMBOSS-RS feature-aware payloads.".to_owned(),
        "pattern_tools" => "This tool accepts nucleotide or protein sequence inputs plus a deterministic pattern specification defined by the implemented method parameters.".to_owned(),
        "protein_plots" => "This tool accepts exactly one protein sequence record for the current v1 plotted analytical path and computes plot-ready analytical data in Rust.".to_owned(),
        "sequence_stream" | "sequence_edit" | "sequence_transform" => "This tool accepts local sequence records or in-memory sequence payloads through the shared sequence IO abstraction. Record ordering is deterministic and preserved unless the method explicitly transforms it.".to_owned(),
        "sequence_stats" => "This tool accepts nucleotide or protein sequence records through the shared sequence IO abstraction and emits structured analytical summaries rather than bespoke CLI-only text.".to_owned(),
        "translation_tools" | "codon_tools" => "This tool accepts coding-sequence or protein-oriented inputs through the shared sequence IO and typed parameter layers used by the translation and codon-analysis cohort.".to_owned(),
        _ => "This tool accepts governed EMBOSS-RS inputs through the shared typed parameter and IO layers. Refer to the implemented CLI help and service definitions for exact parameter requirements.".to_owned(),
    }
}

fn output_summary(descriptor: ToolDescriptor) -> String {
    match descriptor.family {
        "archive_tools" => "The current implementation emits structured archive metadata or normalized public-run manifest rows. Direct acquisition support is method-specific and explicitly documented where available.".to_owned(),
        "retrieval_tools" => "The current implementation emits normalized sequence content together with structured retrieval provenance routed through the shared acquisition layer.".to_owned(),
        "alignment_tools" => "The current implementation emits normalized alignments or structured alignment summaries through the shared result layer.".to_owned(),
        "alignment_analysis" => "The current implementation emits structured analytical summaries such as matrices, consensus sequences, or deterministic comparison tables.".to_owned(),
        "pairwise_alignment" => "The current implementation emits governed alignment reports and structured comparison summaries rather than legacy free-form output alone.".to_owned(),
        "feature_tools" => "The current implementation emits transformed sequence records plus structured feature or masking summaries where appropriate.".to_owned(),
        "pattern_tools" => "The current implementation emits deterministic pattern-hit tables with explicit coordinates, frames, or matched motifs as appropriate to the tool.".to_owned(),
        "protein_plots" => "The current implementation emits both a structured analytical report and a formal plot-contract payload consumed by the R rendering surface.".to_owned(),
        "sequence_stream" => "The current implementation emits normalized sequence records or simple structured counts for stream-oriented sequence selection methods.".to_owned(),
        "sequence_edit" | "sequence_transform" => "The current implementation emits normalized sequence records after applying the documented record-level transformation or extraction.".to_owned(),
        "sequence_stats" => "The current implementation emits structured per-record and aggregate statistics tables suitable for CLI rendering, testing, and later R projection.".to_owned(),
        "translation_tools" | "codon_tools" => "The current implementation emits structured comparison rows, codon profiles, or transformed sequence records through the shared result layer.".to_owned(),
        _ => "The current implementation emits governed structured results through the shared result/report objects used across EMBOSS-RS.".to_owned(),
    }
}

fn capitalized_summary(summary: &str) -> String {
    let mut chars = summary.chars();
    match chars.next() {
        Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
        None => String::new(),
    }
}

#[cfg(test)]
mod tests {
    use super::{build_stub_catalog, build_stub_document};
    use crate::AutodocSourceMode;
    use emboss_tools::governed_tool_descriptors;

    #[test]
    fn builds_one_document_per_exposed_tool() {
        let documents = build_stub_catalog();
        assert_eq!(documents.len(), governed_tool_descriptors().len());
    }

    #[test]
    fn baseline_stub_includes_tool_family_and_summary() {
        let document = build_stub_document(governed_tool_descriptors()[0]);
        assert_eq!(document.tool.name, "aligncopy");
        assert_eq!(document.tool.family.as_deref(), Some("alignment_tools"));
        assert!(document.tool.summary.is_some());
        assert_eq!(
            document.provenance.source_mode,
            AutodocSourceMode::RegistryStub
        );
    }

    #[test]
    fn needle_preserves_richer_curated_content() {
        let needle = governed_tool_descriptors()
            .iter()
            .copied()
            .find(|descriptor| descriptor.name == "needle")
            .expect("needle should exist");
        let document = build_stub_document(needle);
        assert_eq!(document.examples.len(), 1);
        assert!(document.validation.is_some());
    }
}

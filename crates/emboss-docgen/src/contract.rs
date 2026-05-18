//! Versioned autodoc JSON contract types.

use std::io::Read;

use emboss_diagnostics::PlatformError;
use emboss_providers::ResolutionIntent;
use serde::{Deserialize, Serialize};

use crate::error::AutodocContractError;
use crate::validate::{AUTODOC_SCHEMA_VERSION, validate_document};

/// Root machine-readable autodoc contract consumed by `emboss-rs autodoc`.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AutodocDocument {
    /// Version of the autodoc contract schema.
    pub schema_version: String,
    /// Stable identifier for the documentation unit.
    pub document_id: String,
    /// Tool or method being documented.
    pub tool: ToolIdentity,
    /// Human-authored narrative sections.
    #[serde(default)]
    pub sections: Vec<AutodocNarrativeSection>,
    /// Declared input and output artifacts referenced by the contract.
    #[serde(default)]
    pub artifacts: Vec<ArtifactSpec>,
    /// Declared examples or future runs associated with the tool.
    #[serde(default)]
    pub examples: Vec<AutodocExample>,
    /// Provenance and source-mode metadata for the contract.
    pub provenance: AutodocProvenance,
    /// Optional validation expectations for later execution.
    pub validation: Option<ValidationExpectation>,
}

impl AutodocDocument {
    /// Creates a new autodoc document using the current canonical schema version.
    #[must_use]
    pub fn new(
        document_id: impl Into<String>,
        tool: ToolIdentity,
        provenance: AutodocProvenance,
    ) -> Self {
        Self {
            schema_version: AUTODOC_SCHEMA_VERSION.to_owned(),
            document_id: document_id.into(),
            tool,
            sections: Vec::new(),
            artifacts: Vec::new(),
            examples: Vec::new(),
            provenance,
            validation: None,
        }
    }

    /// Parses and validates an autodoc document from JSON text.
    pub fn from_json_str(json: &str) -> Result<Self, AutodocContractError> {
        let document: Self = serde_json::from_str(json)?;
        document.validate()?;
        Ok(document)
    }

    /// Parses and validates an autodoc document from a reader.
    pub fn from_json_reader(reader: impl Read) -> Result<Self, AutodocContractError> {
        let document: Self = serde_json::from_reader(reader)?;
        document.validate()?;
        Ok(document)
    }

    /// Validates the semantic invariants of the autodoc document.
    pub fn validate(&self) -> Result<(), PlatformError> {
        validate_document(self)
    }
}

/// Identity of the tool or method being documented.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ToolIdentity {
    /// Stable EMBOSS-RS tool name.
    pub name: String,
    /// Optional governed family name.
    pub family: Option<String>,
    /// Optional one-line summary.
    pub summary: Option<String>,
    /// Optional legacy EMBOSS names associated with this tool.
    #[serde(default)]
    pub legacy_names: Vec<String>,
}

/// Human-authored narrative content block.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AutodocNarrativeSection {
    /// Stable identifier for the section.
    pub id: String,
    /// Kind of narrative block.
    pub kind: NarrativeSectionKind,
    /// Display title for the section.
    pub title: String,
    /// Markdown or plain-text content for the section.
    pub content: String,
}

/// Supported narrative block categories.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum NarrativeSectionKind {
    /// High-level overview of the method.
    Overview,
    /// Input requirements and semantics.
    Inputs,
    /// Output semantics.
    Outputs,
    /// Example or worked example commentary.
    Examples,
    /// Historical or legacy-context notes.
    LegacyContext,
    /// Warnings, caveats, or ambiguity notes.
    Caveats,
    /// Free-form additional notes.
    Notes,
}

/// Declared artifact referenced by the documentation contract.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ArtifactSpec {
    /// Stable artifact identifier referenced by examples.
    pub id: String,
    /// Human-readable label for the artifact.
    pub label: String,
    /// Origin classification for the artifact.
    pub origin: ArtifactOrigin,
    /// Intended acquisition mechanism for the artifact.
    pub acquisition: AcquisitionMethod,
    /// Locator or identifier for the artifact.
    pub reference: ArtifactReference,
    /// Optional descriptive notes.
    pub description: Option<String>,
}

/// Origin classification for an autodoc artifact.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum ArtifactOrigin {
    /// Local source file or directory.
    LocalFile,
    /// Accessions or provider-backed biological resources.
    AccessionedResource,
    /// Fixture asset maintained in the repository or test material.
    FixtureAsset,
    /// Generated or intermediate artifact.
    GeneratedArtifact,
    /// Legacy EMBOSS-derived source artifact.
    LegacyEmbossReference {
        /// Legacy source label or path.
        source_label: String,
    },
    /// Explicitly labelled fallback for unusual or not-yet-modelled origins.
    Other {
        /// Human-supplied explicit label.
        label: String,
    },
}

/// How an artifact is expected to be resolved or acquired.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "mode", rename_all = "snake_case")]
pub enum AcquisitionMethod {
    /// Resolved directly from a declared local path.
    LocalPath,
    /// Resolved through a governed provider path.
    Provider {
        /// Resolution intent associated with the provider access.
        intent: ResolutionIntentModel,
        /// Optional preferred provider identity.
        preferred_provider: Option<String>,
    },
    /// Resolved from governed fixture inventory.
    Fixture,
    /// Derived from legacy EMBOSS artefacts.
    LegacyHarvest,
    /// Generated within the documentation flow.
    Generated,
    /// Manually curated artifact with explicit notes.
    Manual,
}

/// Reference payload for a declared artifact.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum ArtifactReference {
    /// Filesystem path reference.
    Path {
        /// Relative or absolute path.
        path: String,
    },
    /// Accession-like identifier reference.
    Accession {
        /// Accession or provider-local identifier.
        accession: String,
    },
    /// Provider-backed locator reference.
    ProviderLocator {
        /// Optional explicit provider identity.
        provider: Option<String>,
        /// Provider-local locator.
        locator: String,
    },
    /// Managed fixture or documentation asset reference.
    ManagedAsset {
        /// Stable managed asset identifier.
        asset_id: String,
    },
    /// Generated artifact label or path placeholder.
    Generated {
        /// Generated artifact locator or identifier.
        locator: String,
    },
}

/// Resolution intent used in the contract without exposing provider crate internals directly.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResolutionIntentModel {
    /// Resolve sequence input.
    SequenceInput,
    /// Resolve metadata only.
    MetadataLookup,
    /// Resolve archive or run-level assets.
    ArchiveAsset,
    /// Resolve documentation or historical artifacts.
    DocumentationAsset,
}

impl From<ResolutionIntentModel> for ResolutionIntent {
    fn from(value: ResolutionIntentModel) -> Self {
        match value {
            ResolutionIntentModel::SequenceInput => Self::SequenceInput,
            ResolutionIntentModel::MetadataLookup => Self::MetadataLookup,
            ResolutionIntentModel::ArchiveAsset => Self::ArchiveAsset,
            ResolutionIntentModel::DocumentationAsset => Self::DocumentationAsset,
        }
    }
}

/// Declared example or future run for a documented tool.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AutodocExample {
    /// Stable example identifier.
    pub id: String,
    /// Human-readable title.
    pub title: String,
    /// Optional example description or narrative note.
    pub description: Option<String>,
    /// Artifact identifiers required by this example.
    #[serde(default)]
    pub artifact_ids: Vec<String>,
    /// Example parameters or arguments.
    #[serde(default)]
    pub parameters: Vec<ExampleParameter>,
    /// Expected output declarations or placeholders.
    #[serde(default)]
    pub expected_outputs: Vec<AutodocExampleOutput>,
    /// Optional mapping to a legacy EMBOSS invocation or artefact set.
    pub legacy_reference: Option<LegacyReference>,
}

/// Parameter entry for a declared example.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ExampleParameter {
    /// Stable parameter or argument label.
    pub name: String,
    /// Parameter value as authored in the contract.
    pub value: String,
}

/// Expected output declaration for a future documentation run.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AutodocExampleOutput {
    /// Output identifier within the example.
    pub id: String,
    /// Human-readable label.
    pub label: String,
    /// Optional descriptive notes.
    pub description: Option<String>,
}

/// Legacy EMBOSS reference metadata for a section or example.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LegacyReference {
    /// Human-readable legacy source label.
    pub source: String,
    /// Optional source path or URL.
    pub locator: Option<String>,
    /// Optional historical invocation form.
    pub invocation: Option<String>,
}

/// Contract-level provenance metadata.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AutodocProvenance {
    /// Source mode for the document.
    pub source_mode: AutodocSourceMode,
    /// Optional author or curator label.
    pub curated_by: Option<String>,
    /// Optional source references associated with the document.
    #[serde(default)]
    pub source_references: Vec<LegacyReference>,
}

/// High-level source mode for an autodoc contract.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AutodocSourceMode {
    /// Deterministic registry-generated baseline stub.
    RegistryStub,
    /// Entirely hand-curated contract.
    Curated,
    /// Entirely derived from legacy material.
    LegacyDerived,
    /// Mixed curated and legacy-derived inputs.
    Mixed,
}

impl AutodocSourceMode {
    /// Stable machine-readable label used across summaries and generated docs.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::RegistryStub => "registry_stub",
            Self::Curated => "curated",
            Self::LegacyDerived => "legacy_derived",
            Self::Mixed => "mixed",
        }
    }
}

/// Optional validation expectations for future autodoc execution.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ValidationExpectation {
    /// Example identifiers expected to be executable or at least preserved.
    #[serde(default)]
    pub required_example_ids: Vec<String>,
    /// Whether legacy comparison is expected later.
    pub compare_against_legacy: bool,
    /// Whether provenance capture is mandatory for the run.
    pub require_provenance_capture: bool,
}

#[cfg(test)]
mod tests {
    use emboss_providers::{ProviderId, ResolutionIntent};

    use super::{AutodocDocument, AutodocProvenance, AutodocSourceMode, ToolIdentity};
    use crate::validate::AUTODOC_SCHEMA_VERSION;

    #[test]
    fn new_document_uses_current_schema_version() {
        let document = AutodocDocument::new(
            "needle-basic",
            ToolIdentity {
                name: "needle".to_owned(),
                family: Some("alignment".to_owned()),
                summary: None,
                legacy_names: Vec::new(),
            },
            AutodocProvenance {
                source_mode: AutodocSourceMode::Curated,
                curated_by: Some("test".to_owned()),
                source_references: Vec::new(),
            },
        );

        assert_eq!(document.schema_version, AUTODOC_SCHEMA_VERSION);
    }

    #[test]
    fn resolution_intent_maps_to_provider_intent() {
        let intent: ResolutionIntent = super::ResolutionIntentModel::DocumentationAsset.into();
        assert_eq!(intent, ResolutionIntent::DocumentationAsset);
    }

    #[test]
    fn provider_reference_accepts_valid_provider_id() {
        let provider = ProviderId::new("ena").expect("provider id should be valid");
        assert_eq!(provider.as_str(), "ena");
    }
}

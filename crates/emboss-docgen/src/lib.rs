//! Documentation generation and autodoc scaffolding for EMBOSS-RS.
//!
//! This crate owns the canonical machine-readable contract consumed by the
//! future `emboss-rs autodoc` command. It defines the versioned JSON model,
//! parsing helpers, validation logic, and generated-page emission for curated
//! and legacy-derived documentation inputs.

pub mod contract;
pub mod emit;
pub mod error;
pub mod legacy;
pub mod process;
pub mod transform;
pub mod validate;

pub use contract::{
    AcquisitionMethod, ArtifactOrigin, ArtifactReference, ArtifactSpec, AutodocDocument,
    AutodocExample, AutodocExampleOutput, AutodocNarrativeSection, AutodocProvenance,
    AutodocSourceMode, ExampleParameter, LegacyReference, NarrativeSectionKind,
    ResolutionIntentModel, ToolIdentity, ValidationExpectation,
};
pub use emit::{DEFAULT_GENERATED_DOCS_ROOT, GeneratedDocsReport, emit_generated_docs};
pub use error::AutodocContractError;
pub use legacy::{
    LegacyArtifactCategory, LegacyArtifactRecord, LegacyEmbossSourceRoot, LegacyHarvestReport,
    discover_legacy_tool_artifacts,
};
pub use process::{
    AutodocProcessingSummary, emit_generated_docs_from_path, load_document_from_path,
    load_summary_from_path,
};
pub use transform::{
    LegacyAutodocTransformReport, derive_autodoc_from_legacy_root, transform_legacy_report,
};
pub use validate::AUTODOC_SCHEMA_VERSION;

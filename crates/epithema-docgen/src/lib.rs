//! Documentation generation and autodoc scaffolding for Epithema.
//!
//! This crate owns the canonical machine-readable contract consumed by the
//! future `epithema autodoc` command. It defines the versioned JSON model,
//! parsing helpers, validation logic, governed acquisition enforcement, and
//! generated-page emission for curated and legacy-derived documentation inputs.

pub mod acquisition;
pub mod contract;
pub mod emit;
pub mod error;
pub mod legacy;
pub mod process;
pub mod stub;
pub mod transform;
pub mod validate;

pub use acquisition::{DocumentationAcquisitionReport, enforce_documentation_acquisition_policy};
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
    AutodocProcessingSummary, emit_generated_docs_from_path,
    emit_generated_docs_from_path_with_gateway, load_document_from_path, load_summary_from_path,
    load_summary_from_path_with_gateway,
};
pub use stub::{
    DEFAULT_AUTODOC_STUBS_ROOT, build_stub_catalog, build_stub_document, write_stub_catalog,
};
pub use transform::{
    LegacyAutodocTransformReport, derive_autodoc_from_legacy_root, transform_legacy_report,
};
pub use validate::AUTODOC_SCHEMA_VERSION;

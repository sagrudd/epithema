//! Documentation generation and autodoc scaffolding for EMBOSS-RS.
//!
//! This crate owns the canonical machine-readable contract consumed by the
//! future `emboss-rs autodoc` command. It defines the versioned JSON model,
//! parsing helpers, and validation logic for curated and legacy-derived
//! documentation inputs without yet implementing harvesting or Sphinx rendering.

pub mod contract;
pub mod error;
pub mod legacy;
pub mod process;
pub mod transform;
pub mod validate;

pub use contract::{
    AcquisitionMethod, ArtifactOrigin, ArtifactReference, ArtifactSpec, AutodocDocument,
    AutodocExample, AutodocExampleOutput, AutodocNarrativeSection, AutodocProvenance,
    AutodocSourceMode, ExampleParameter, LegacyReference, NarrativeSectionKind,
    ValidationExpectation,
};
pub use error::AutodocContractError;
pub use legacy::{
    LegacyArtifactCategory, LegacyArtifactRecord, LegacyEmbossSourceRoot, LegacyHarvestReport,
    discover_legacy_tool_artifacts,
};
pub use process::{AutodocProcessingSummary, load_document_from_path, load_summary_from_path};
pub use transform::{
    LegacyAutodocTransformReport, derive_autodoc_from_legacy_root, transform_legacy_report,
};
pub use validate::AUTODOC_SCHEMA_VERSION;

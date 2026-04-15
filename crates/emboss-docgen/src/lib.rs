//! Documentation generation and autodoc scaffolding for EMBOSS-RS.
//!
//! This crate owns the canonical machine-readable contract consumed by the
//! future `emboss-rs autodoc` command. It defines the versioned JSON model,
//! parsing helpers, and validation logic for curated and legacy-derived
//! documentation inputs without yet implementing harvesting or Sphinx rendering.

pub mod contract;
pub mod error;
pub mod validate;

pub use contract::{
    AcquisitionMethod, ArtifactOrigin, ArtifactReference, ArtifactSpec, AutodocDocument,
    AutodocExample, AutodocExampleOutput, AutodocNarrativeSection, AutodocProvenance,
    AutodocSourceMode, ExampleParameter, LegacyReference, NarrativeSectionKind,
    ValidationExpectation,
};
pub use error::AutodocContractError;
pub use validate::AUTODOC_SCHEMA_VERSION;

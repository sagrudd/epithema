//! Shared error, diagnostics, provenance, and execution reporting models for Epithema.
//!
//! This crate defines the cross-cutting language used across CLI, services,
//! future tools, validation harnesses, autodoc, and other front ends to
//! describe failures, non-fatal diagnostics, provenance, and run metadata.

pub mod diagnostic;
pub mod error;
pub mod execution;
pub mod provenance;
pub mod report;
pub mod severity;

pub use diagnostic::{Diagnostic, DiagnosticLocation};
pub use error::{ErrorCategory, PlatformError};
pub use execution::{ExecutionContext, ExecutionMetadata, InvocationOrigin, RunId};
pub use provenance::{ArtifactOriginKind, ArtifactProvenance};
pub use report::{ExecutionOutcome, ExecutionReport, OutcomeStatus};
pub use severity::Severity;

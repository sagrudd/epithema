//! Shared execution and runtime abstractions for EMBOSS-RS.
//!
//! This crate defines the front-end-neutral execution seam used by the CLI, the
//! future R bridge, autodoc workflows, and a future API surface. It differs from
//! `emboss-core`, which owns biological primitives, and from `emboss-tools`,
//! which owns governed tool descriptors.

pub mod context;
pub mod error;
pub mod registry;
pub mod request;
pub mod response;
pub mod service;
pub mod tool;

pub use context::{ExecutionContext, InvocationOrigin};
pub use emboss_config::{AcquisitionPolicy, AutodocPolicy, ConfigEnvironment, PlatformConfig};
pub use emboss_diagnostics::{
    Diagnostic, DiagnosticLocation, ErrorCategory, ExecutionMetadata, ExecutionOutcome,
    ExecutionReport, OutcomeStatus, PlatformError, RunId, Severity,
};
pub use emboss_providers::{
    AcquisitionRequest, DocumentationAssetRequest, InputReference, InputReferenceKind,
    MetadataLookupRequest, ProviderCapability, ProviderDescriptor, ProviderId, ProviderRegistry,
    ResolutionIntent, SequenceRequest,
};
pub use error::ServiceError;
pub use registry::{ServiceRegistry, ToolCatalog};
pub use request::InvocationRequest;
pub use response::{InvocationResponse, InvocationStatus};
pub use service::EmbossService;
pub use tool::ToolName;

/// Backwards-compatible alias for the initial service façade name.
pub use service::EmbossService as ServiceRuntime;

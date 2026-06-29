//! Shared execution and runtime abstractions for Epithema.
//!
//! This crate defines the front-end-neutral execution seam used by the CLI, the
//! future R bridge, autodoc workflows, and a future API surface. It differs from
//! `epithema-core`, which owns biological primitives, and from `epithema-tools`,
//! which owns governed tool descriptors.

pub mod acquisition;
pub mod archive_retrieval;
pub mod context;
pub mod error;
pub mod input;
pub mod input_resolution;
pub mod ngs_retrieval;
pub mod registry;
pub mod request;
pub mod response;
pub mod result;
pub mod sequence_retrieval;
pub mod service;
pub mod tool;

pub use acquisition::ServiceDocumentationAcquisition;
pub use archive_retrieval::ServiceArchiveRetrieval;
pub use context::{ExecutionContext, InvocationOrigin};
pub use epithema_config::{AcquisitionPolicy, AutodocPolicy, ConfigEnvironment, PlatformConfig};
pub use epithema_diagnostics::{
    Diagnostic, DiagnosticLocation, ErrorCategory, ExecutionMetadata, ExecutionOutcome,
    ExecutionReport, OutcomeStatus, PlatformError, RunId, Severity,
};
pub use epithema_providers::{
    AcquisitionRequest, ArchiveFile, ArchiveObjectClass, ArchiveProviderResolution, ArchiveRoute,
    DocumentationAcquisitionGateway, DocumentationAcquisitionRecord,
    DocumentationAcquisitionRequest, DocumentationAcquisitionRoute, DocumentationAssetRequest,
    HttpDownloadProgress, HttpDownloadProgressState, InputReference as ProviderInputReference,
    InputReferenceKind, MetadataLookupRequest, NgsAsset, NgsAssetRole, NgsDownloadPlan,
    NgsDownloadRecord, NgsManifest, NgsManifestRun, NgsObjectClass, NgsProvenance, NgsQuery,
    NgsRunMetadata, NgsVerificationStatus, ProviderCapability, ProviderDescriptor, ProviderId,
    ProviderRegistry, ResolutionIntent, RetrievalFormat, RetrievalRoute, RetrievedArchiveManifest,
    RetrievedArchiveMetadata, RetrievedSequence, SequenceProviderResolution, SequenceRequest,
};
pub use error::ServiceError;
pub use input::{ToolInputKind, ToolInputReference, ToolInputResolution, ToolInputResolver};
pub use ngs_retrieval::{NgsDownloadProgressCallback, ServiceNgsRetrieval};
pub use registry::{ServiceRegistry, ToolCatalog};
pub use request::InvocationRequest;
pub use response::{InvocationResponse, InvocationStatus};
pub use result::{
    ArtifactKind, ArtifactReference, MethodResult, ResultPayload, ResultSummary, TableReport,
    TextReport,
};
pub use sequence_retrieval::ServiceSequenceRetrieval;
pub use service::EpithemaService;
pub use tool::ToolName;

/// Backwards-compatible alias for the initial service façade name.
pub use service::EpithemaService as ServiceRuntime;

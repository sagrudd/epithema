//! Modern provider abstractions and input-resolution contracts for EMBOSS-RS.
//!
//! This crate defines typed seams for accession-driven acquisition, local versus
//! remote resolution, documentation asset retrieval, and future archive support
//! without embedding network logic into the CLI or individual tools. It now
//! includes the first real provider-backed single-sequence retrieval adapters
//! for ENA and NCBI. The design is intentionally provider-explicit and does not
//! recreate legacy EMBOSS server discovery or generic remote registry plumbing.

pub mod acquisition;
pub mod capability;
pub mod client;
pub mod descriptor;
pub mod ena;
pub mod identity;
pub mod input;
pub mod ncbi;
pub mod registry;
pub mod request;
pub mod sequence_retrieval;
pub mod traits;

pub use acquisition::{
    DocumentationAcquisitionGateway, DocumentationAcquisitionRecord,
    DocumentationAcquisitionRequest, DocumentationAcquisitionRoute,
};
pub use capability::ProviderCapability;
pub use client::{HttpRequest, HttpResponse, ProviderHttpClient, ReqwestHttpClient};
pub use descriptor::ProviderDescriptor;
pub use ena::EnaSequenceAdapter;
pub use identity::ProviderId;
pub use input::{InputReference, InputReferenceKind, ResolutionIntent};
pub use ncbi::{NcbiDatabase, NcbiSequenceAdapter};
pub use registry::ProviderRegistry;
pub use request::{
    AcquisitionRequest, DocumentationAssetRequest, MetadataLookupRequest, SequenceRequest,
};
pub use sequence_retrieval::{
    ProviderSequenceRouter, RetrievalFormat, RetrievalRoute, RetrievedSequence,
    SequenceProviderResolution,
};
pub use traits::{
    ArchiveProvider, CapabilityProvider, DocumentationAssetProvider, MetadataProvider,
    SequenceProvider,
};

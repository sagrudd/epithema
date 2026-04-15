//! Modern provider abstractions and input-resolution contracts for EMBOSS-RS.
//!
//! This crate defines typed seams for accession-driven acquisition, local versus
//! remote resolution, documentation asset retrieval, and modern archive access
//! without embedding network logic into the CLI or individual tools. It now
//! includes provider-backed single-sequence retrieval for ENA and NCBI plus the
//! first public archive metadata and run-manifest adapters for ENA and SRA. The
//! design is intentionally provider-explicit and does not recreate legacy
//! EMBOSS server discovery, generic remote registries, or SRS-era abstractions.

pub mod acquisition;
pub mod archive;
pub mod capability;
pub mod client;
pub mod descriptor;
pub mod ena;
pub mod ena_archive;
pub mod identity;
pub mod input;
pub mod ncbi;
pub mod registry;
pub mod request;
pub mod sequence_retrieval;
pub mod sra_archive;
pub mod traits;

pub use acquisition::{
    DocumentationAcquisitionGateway, DocumentationAcquisitionRecord,
    DocumentationAcquisitionRequest, DocumentationAcquisitionRoute,
};
pub use archive::{
    ArchiveFile, ArchiveObjectClass, ArchiveProviderResolution, ArchiveRoute,
    ProviderArchiveRouter, RetrievedArchiveManifest, RetrievedArchiveMetadata,
};
pub use capability::ProviderCapability;
pub use client::{HttpRequest, HttpResponse, ProviderHttpClient, ReqwestHttpClient};
pub use descriptor::ProviderDescriptor;
pub use ena::EnaSequenceAdapter;
pub use ena_archive::EnaArchiveAdapter;
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
pub use sra_archive::SraArchiveAdapter;
pub use traits::{
    ArchiveProvider, CapabilityProvider, DocumentationAssetProvider, MetadataProvider,
    SequenceProvider,
};

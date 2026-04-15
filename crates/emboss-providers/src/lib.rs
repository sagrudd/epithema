//! Modern provider abstractions and input-resolution contracts for EMBOSS-RS.
//!
//! This crate defines typed seams for accession-driven acquisition, local versus
//! remote resolution, documentation asset retrieval, and future archive support
//! without embedding network logic into the CLI or individual tools.

pub mod capability;
pub mod descriptor;
pub mod identity;
pub mod input;
pub mod registry;
pub mod request;
pub mod traits;

pub use capability::ProviderCapability;
pub use descriptor::ProviderDescriptor;
pub use identity::ProviderId;
pub use input::{InputReference, InputReferenceKind, ResolutionIntent};
pub use registry::ProviderRegistry;
pub use request::{
    AcquisitionRequest, DocumentationAssetRequest, MetadataLookupRequest, SequenceRequest,
};
pub use traits::{
    ArchiveProvider, CapabilityProvider, DocumentationAssetProvider, MetadataProvider,
    SequenceProvider,
};

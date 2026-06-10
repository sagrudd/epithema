//! Provider trait boundaries.

use epithema_diagnostics::PlatformError;

use crate::{
    AcquisitionRequest, DocumentationAssetRequest, MetadataLookupRequest, ProviderDescriptor,
    RetrievedSequence, SequenceRequest,
};

/// Trait for exposing provider identity and capability metadata.
pub trait CapabilityProvider {
    /// Returns the static descriptor for the provider implementation.
    fn descriptor(&self) -> &ProviderDescriptor;
}

/// Trait for providers that can resolve metadata.
pub trait MetadataProvider: CapabilityProvider {
    /// Attempts to resolve provider-backed metadata.
    fn lookup_metadata(&self, request: &MetadataLookupRequest) -> Result<(), PlatformError>;
}

/// Trait for providers that can resolve sequences.
pub trait SequenceProvider: CapabilityProvider {
    /// Attempts to resolve sequence input through the provider.
    fn retrieve_sequence(
        &self,
        request: &SequenceRequest,
    ) -> Result<RetrievedSequence, PlatformError>;
}

/// Trait for providers that can resolve archive or run-level assets.
pub trait ArchiveProvider: CapabilityProvider {
    /// Attempts to resolve an archive-oriented acquisition request.
    fn acquire_archive_asset(&self, request: &AcquisitionRequest) -> Result<(), PlatformError>;
}

/// Trait for providers that can resolve documentation or historical assets.
pub trait DocumentationAssetProvider: CapabilityProvider {
    /// Attempts to resolve a documentation asset request.
    fn retrieve_documentation_asset(
        &self,
        request: &DocumentationAssetRequest,
    ) -> Result<(), PlatformError>;
}

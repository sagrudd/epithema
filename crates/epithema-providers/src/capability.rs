//! Provider capability declarations.

/// Supported capability classes for modern Epithema providers.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum ProviderCapability {
    /// Resolve metadata for an accession or provider asset.
    MetadataLookup,
    /// Retrieve sequence data.
    SequenceRetrieval,
    /// Retrieve run, archive, or assay-associated assets.
    ArchiveAcquisition,
    /// Retrieve documentation or historical artefacts through governed paths.
    DocumentationAssetAcquisition,
}

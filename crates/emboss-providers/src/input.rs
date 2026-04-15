//! Input-reference and resolution-intent models.

use std::path::{Path, PathBuf};

use crate::ProviderId;

/// High-level classification of an incoming input reference.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum InputReferenceKind {
    /// Local filesystem path.
    LocalPath,
    /// Accession-like biological identifier.
    Accession,
    /// Provider-backed locator.
    ProviderAsset,
    /// Fixture or documentation asset.
    ManagedAsset,
    /// Literal content supplied inline.
    LiteralContent,
    /// Input could not yet be classified.
    Unresolved,
}

/// Front-end-neutral description of an input reference.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum InputReference {
    /// Local path input.
    LocalPath(PathBuf),
    /// Accession-like identifier.
    Accession(String),
    /// Provider-qualified asset locator.
    ProviderAsset {
        /// Optional preferred provider identity.
        provider: Option<ProviderId>,
        /// Provider-local locator or accession.
        locator: String,
    },
    /// Fixture or documentation asset reference.
    ManagedAsset(String),
    /// Literal inline content.
    LiteralContent(String),
    /// Caller supplied raw unresolved input.
    Unresolved(String),
}

impl InputReference {
    /// Creates a local path reference.
    #[must_use]
    pub fn local_path(path: impl Into<PathBuf>) -> Self {
        Self::LocalPath(path.into())
    }

    /// Creates an accession reference.
    #[must_use]
    pub fn accession(accession: impl Into<String>) -> Self {
        Self::Accession(accession.into())
    }

    /// Creates a provider-backed asset reference.
    #[must_use]
    pub fn provider_asset(provider: Option<ProviderId>, locator: impl Into<String>) -> Self {
        Self::ProviderAsset {
            provider,
            locator: locator.into(),
        }
    }

    /// Creates a managed asset reference.
    #[must_use]
    pub fn managed_asset(locator: impl Into<String>) -> Self {
        Self::ManagedAsset(locator.into())
    }

    /// Creates a literal content reference.
    #[must_use]
    pub fn literal_content(content: impl Into<String>) -> Self {
        Self::LiteralContent(content.into())
    }

    /// Returns the classified kind for the input reference.
    #[must_use]
    pub fn kind(&self) -> InputReferenceKind {
        match self {
            Self::LocalPath(_) => InputReferenceKind::LocalPath,
            Self::Accession(_) => InputReferenceKind::Accession,
            Self::ProviderAsset { .. } => InputReferenceKind::ProviderAsset,
            Self::ManagedAsset(_) => InputReferenceKind::ManagedAsset,
            Self::LiteralContent(_) => InputReferenceKind::LiteralContent,
            Self::Unresolved(_) => InputReferenceKind::Unresolved,
        }
    }

    /// Returns the local path when the reference is file-backed.
    #[must_use]
    pub fn as_local_path(&self) -> Option<&Path> {
        match self {
            Self::LocalPath(path) => Some(path.as_path()),
            _ => None,
        }
    }
}

/// Declares why a caller wants an input resolved.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ResolutionIntent {
    /// Resolve biological sequence input for a tool or service request.
    SequenceInput,
    /// Resolve metadata associated with an input.
    MetadataLookup,
    /// Resolve archive or run-level data.
    ArchiveAsset,
    /// Resolve documentation or historical assets.
    DocumentationAsset,
}

#[cfg(test)]
mod tests {
    use super::{InputReference, InputReferenceKind, ResolutionIntent};

    #[test]
    fn classifies_local_path_inputs() {
        let input = InputReference::local_path("data/example.fa");
        assert_eq!(input.kind(), InputReferenceKind::LocalPath);
        assert!(input.as_local_path().is_some());
    }

    #[test]
    fn resolution_intent_is_distinct() {
        assert_ne!(
            ResolutionIntent::SequenceInput,
            ResolutionIntent::DocumentationAsset
        );
    }
}

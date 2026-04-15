//! Provider request models.

use crate::{InputReference, ProviderId, ResolutionIntent};

/// Shared acquisition request envelope for provider-mediated resolution.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AcquisitionRequest {
    /// Why the caller needs the input resolved.
    pub intent: ResolutionIntent,
    /// Input reference supplied by the caller.
    pub input: InputReference,
    /// Optional preferred provider identity.
    pub preferred_provider: Option<ProviderId>,
}

impl AcquisitionRequest {
    /// Creates an acquisition request.
    #[must_use]
    pub fn new(intent: ResolutionIntent, input: InputReference) -> Self {
        Self {
            intent,
            input,
            preferred_provider: None,
        }
    }

    /// Applies a preferred provider constraint.
    #[must_use]
    pub fn with_preferred_provider(mut self, provider: ProviderId) -> Self {
        self.preferred_provider = Some(provider);
        self
    }
}

/// Request to resolve metadata for an input reference.
pub type MetadataLookupRequest = AcquisitionRequest;
/// Request to resolve a sequence-bearing input reference.
pub type SequenceRequest = AcquisitionRequest;
/// Request to resolve documentation or historical artefacts.
pub type DocumentationAssetRequest = AcquisitionRequest;

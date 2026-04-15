//! Typed single-sequence retrieval models and provider router.

use emboss_core::{MoleculeKind, SequenceRecord};
use emboss_diagnostics::{ArtifactProvenance, ErrorCategory, PlatformError};
use emboss_io::{IoError, parse_fasta_str, parse_fasta_str_with_molecule};

use crate::{
    AcquisitionRequest, EnaSequenceAdapter, HttpResponse, InputReference, NcbiSequenceAdapter,
    ProviderHttpClient, ProviderId,
};

/// Supported payload formats for provider-backed single-sequence retrieval.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RetrievalFormat {
    /// FASTA text payload.
    Fasta,
}

impl RetrievalFormat {
    /// Returns the stable lowercase label.
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Fasta => "fasta",
        }
    }
}

/// Provenance-rich route metadata for a retrieval attempt.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RetrievalRoute {
    /// Provider identity used for the request.
    pub provider: ProviderId,
    /// Provider-local route label.
    pub endpoint: String,
    /// Payload format requested from the provider.
    pub format: RetrievalFormat,
}

impl RetrievalRoute {
    /// Creates a retrieval route description.
    #[must_use]
    pub fn new(provider: ProviderId, endpoint: impl Into<String>, format: RetrievalFormat) -> Self {
        Self {
            provider,
            endpoint: endpoint.into(),
            format,
        }
    }
}

/// Provider-neutral retrieval result carrying both parsed and raw content.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RetrievedSequence {
    /// Provider identity.
    pub provider: ProviderId,
    /// Requested accession or provider-local locator.
    pub requested_accession: String,
    /// Retrieval route metadata.
    pub route: RetrievalRoute,
    /// Raw text payload returned by the provider.
    pub raw_content: String,
    /// Parsed sequence record.
    pub record: SequenceRecord,
    /// Structured provenance for the retrieved artefact.
    pub provenance: ArtifactProvenance,
}

impl RetrievedSequence {
    /// Creates a provider-neutral retrieved sequence record.
    #[must_use]
    pub fn new(
        provider: ProviderId,
        requested_accession: impl Into<String>,
        route: RetrievalRoute,
        raw_content: impl Into<String>,
        record: SequenceRecord,
    ) -> Self {
        let requested_accession = requested_accession.into();
        let provenance = ArtifactProvenance::provider_asset(requested_accession.clone())
            .with_provider(provider.as_str())
            .with_description(format!(
                "retrieved via {} {}",
                route.endpoint,
                route.format.as_str()
            ));

        Self {
            provider,
            requested_accession,
            route,
            raw_content: raw_content.into(),
            record,
            provenance,
        }
    }
}

/// Concrete provider selection for a sequence request.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SequenceProviderResolution {
    /// Chosen provider identity.
    pub provider: ProviderId,
    /// Provider-local locator or accession payload.
    pub locator: String,
}

impl SequenceProviderResolution {
    /// Resolves a provider-backed sequence request conservatively.
    pub fn from_request(request: &AcquisitionRequest) -> Result<Self, PlatformError> {
        let provider = request
            .preferred_provider
            .clone()
            .or_else(|| match &request.input {
                InputReference::ProviderAsset { provider, .. } => provider.clone(),
                _ => None,
            });

        let locator = match &request.input {
            InputReference::ProviderAsset { locator, .. } => locator.clone(),
            InputReference::Accession(accession) => accession.clone(),
            other => {
                return Err(PlatformError::new(
                    ErrorCategory::Validation,
                    "single-sequence retrieval only supports accession-style provider inputs",
                )
                .with_code("providers.sequence.unsupported_input_kind")
                .with_detail(format!("{other:?}")));
            }
        };

        let Some(provider) = provider else {
            return Err(PlatformError::new(
                ErrorCategory::Validation,
                "bare accession could not be resolved to a retrieval provider",
            )
            .with_code("providers.sequence.ambiguous_bare_accession")
            .with_detail(locator));
        };

        Ok(Self { provider, locator })
    }
}

/// Provider router for built-in single-sequence adapters.
#[derive(Clone, Debug, Default)]
pub struct ProviderSequenceRouter;

impl ProviderSequenceRouter {
    /// Creates a provider router.
    #[must_use]
    pub fn new() -> Self {
        Self
    }

    /// Routes a request to a built-in provider adapter.
    pub fn retrieve_with<C: ProviderHttpClient>(
        &self,
        request: &AcquisitionRequest,
        client: &C,
    ) -> Result<RetrievedSequence, PlatformError> {
        let resolution = SequenceProviderResolution::from_request(request)?;

        match resolution.provider.as_str() {
            "ena" => EnaSequenceAdapter::new().retrieve(request, &resolution, client),
            "ncbi" => NcbiSequenceAdapter::new().retrieve(request, &resolution, client),
            other => Err(PlatformError::new(
                ErrorCategory::Registry,
                "single-sequence retrieval is not implemented for the requested provider",
            )
            .with_code("providers.sequence.unsupported_provider")
            .with_detail(other.to_owned())),
        }
    }
}

pub(crate) fn parse_single_fasta_record(
    fasta: &str,
    explicit_molecule: Option<MoleculeKind>,
    provider: &ProviderId,
    requested_accession: &str,
    route: RetrievalRoute,
) -> Result<RetrievedSequence, PlatformError> {
    let parsed = parse_fasta_payload(fasta, explicit_molecule).map_err(|error| {
        PlatformError::new(
            ErrorCategory::Invocation,
            "provider returned malformed FASTA content",
        )
        .with_code("providers.sequence.parse_failed")
        .with_detail(error.to_string())
    })?;

    if parsed.is_empty() {
        return Err(PlatformError::new(
            ErrorCategory::Invocation,
            "provider returned an empty FASTA payload",
        )
        .with_code("providers.sequence.response.empty")
        .with_detail(requested_accession.to_owned()));
    }

    if parsed.len() != 1 {
        return Err(PlatformError::new(
            ErrorCategory::Invocation,
            "provider returned multiple FASTA records where one was expected",
        )
        .with_code("providers.sequence.response.multiple_records")
        .with_detail(requested_accession.to_owned()));
    }

    let mut record = parsed.into_iter().next().expect("checked single record");
    let metadata = record.metadata().clone().with_source(provider.as_str());
    record = record.with_metadata(metadata);

    Ok(RetrievedSequence::new(
        provider.clone(),
        requested_accession.to_owned(),
        route,
        fasta.to_owned(),
        record,
    ))
}

fn parse_fasta_payload(
    fasta: &str,
    explicit_molecule: Option<MoleculeKind>,
) -> Result<Vec<SequenceRecord>, IoError> {
    if let Some(molecule) = explicit_molecule {
        parse_fasta_str_with_molecule(fasta, molecule)
    } else {
        parse_fasta_str(fasta)
    }
}

pub(crate) fn validate_success_response(
    response: &HttpResponse,
    provider: &str,
    requested_accession: &str,
) -> Result<(), PlatformError> {
    if response.status == 404 {
        return Err(PlatformError::new(
            ErrorCategory::Invocation,
            "provider did not find a sequence for the requested accession",
        )
        .with_code("providers.sequence.http.not_found")
        .with_detail(format!("{provider}:{requested_accession}")));
    }

    if !(200..300).contains(&response.status) {
        return Err(PlatformError::new(
            ErrorCategory::Invocation,
            "provider returned an unsuccessful HTTP status",
        )
        .with_code("providers.sequence.http.failure")
        .with_detail(format!(
            "{provider}:{requested_accession} status={}",
            response.status
        )));
    }

    if response.body.trim().is_empty() {
        return Err(PlatformError::new(
            ErrorCategory::Invocation,
            "provider returned an empty response body",
        )
        .with_code("providers.sequence.response.empty")
        .with_detail(format!("{provider}:{requested_accession}")));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use emboss_core::MoleculeKind;

    use super::{
        RetrievalFormat, RetrievalRoute, SequenceProviderResolution, parse_single_fasta_record,
    };
    use crate::{AcquisitionRequest, InputReference, ProviderId, ResolutionIntent};

    #[test]
    fn rejects_bare_accessions_without_provider_inference() {
        let request = AcquisitionRequest::new(
            ResolutionIntent::SequenceInput,
            InputReference::accession("AB000263"),
        );

        let error = SequenceProviderResolution::from_request(&request)
            .expect_err("bare accession should remain unresolved");
        assert_eq!(
            error.code(),
            Some("providers.sequence.ambiguous_bare_accession")
        );
    }

    #[test]
    fn parses_single_fasta_record_with_provider_source_metadata() {
        let provider = ProviderId::new("ena").expect("valid provider");
        let route = RetrievalRoute::new(
            provider.clone(),
            "ena.browser.api.fasta",
            RetrievalFormat::Fasta,
        );

        let result = parse_single_fasta_record(
            ">AB000263 example\nACGT\n",
            Some(MoleculeKind::Dna),
            &provider,
            "AB000263",
            route,
        )
        .expect("single FASTA record should parse");

        assert_eq!(result.record.identifier().accession(), "AB000263");
        assert_eq!(result.record.metadata().source.as_deref(), Some("ena"));
    }
}

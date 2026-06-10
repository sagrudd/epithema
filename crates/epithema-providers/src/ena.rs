//! ENA single-sequence retrieval adapter.

use epithema_core::MoleculeKind;
use epithema_diagnostics::{ErrorCategory, PlatformError};

use crate::{
    AcquisitionRequest, HttpRequest, ProviderHttpClient, ProviderId, RetrievalFormat,
    RetrievalRoute, RetrievedSequence, SequenceProviderResolution,
    sequence_retrieval::{parse_single_fasta_record, validate_success_response},
};

/// ENA adapter for single assembled/reference sequence retrieval.
#[derive(Clone, Copy, Debug, Default)]
pub struct EnaSequenceAdapter;

impl EnaSequenceAdapter {
    /// Creates an ENA adapter.
    #[must_use]
    pub fn new() -> Self {
        Self
    }

    /// Builds the ENA FASTA retrieval request.
    pub fn build_request(
        &self,
        resolution: &SequenceProviderResolution,
    ) -> Result<HttpRequest, PlatformError> {
        if resolution.locator.contains(':') {
            return Err(PlatformError::new(
                ErrorCategory::Validation,
                "ENA single-sequence retrieval expects a simple accession without sub-route prefixes",
            )
            .with_code("providers.sequence.ena.unsupported_locator")
            .with_detail(resolution.locator.clone()));
        }

        Ok(HttpRequest::new(format!(
            "https://www.ebi.ac.uk/ena/browser/api/fasta/{}",
            resolution.locator
        ))
        .with_accept("text/x-fasta, text/plain;q=0.9"))
    }

    /// Retrieves one FASTA record from ENA.
    pub fn retrieve<C: ProviderHttpClient>(
        &self,
        _request: &AcquisitionRequest,
        resolution: &SequenceProviderResolution,
        client: &C,
    ) -> Result<RetrievedSequence, PlatformError> {
        let provider = ProviderId::new("ena").expect("static provider id is valid");
        let http_request = self.build_request(resolution)?;
        let response = client.get_text(&http_request)?;
        validate_success_response(&response, "ena", &resolution.locator)?;

        parse_single_fasta_record(
            &response.body,
            Some(MoleculeKind::Dna),
            &provider,
            &resolution.locator,
            RetrievalRoute::new(
                provider.clone(),
                "ena.browser.api.fasta",
                RetrievalFormat::Fasta,
            ),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::EnaSequenceAdapter;
    use crate::{ProviderId, SequenceProviderResolution};

    #[test]
    fn builds_expected_browser_api_url() {
        let adapter = EnaSequenceAdapter::new();
        let request = adapter
            .build_request(&SequenceProviderResolution {
                provider: ProviderId::new("ena").expect("valid provider"),
                locator: String::from("AB000263"),
            })
            .expect("request should build");

        assert_eq!(
            request.url,
            "https://www.ebi.ac.uk/ena/browser/api/fasta/AB000263"
        );
    }
}

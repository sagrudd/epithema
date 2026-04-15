//! NCBI E-utilities single-sequence retrieval adapter.

use emboss_core::MoleculeKind;
use emboss_diagnostics::{ErrorCategory, PlatformError};
use reqwest::Url;

use crate::{
    AcquisitionRequest, HttpRequest, ProviderHttpClient, ProviderId, RetrievalFormat,
    RetrievalRoute, RetrievedSequence, SequenceProviderResolution,
    sequence_retrieval::{parse_single_fasta_record, validate_success_response},
};

/// Supported NCBI databases for v1 single-sequence retrieval.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum NcbiDatabase {
    /// Nucleotide sequence database.
    Nuccore,
    /// Protein sequence database.
    Protein,
}

impl NcbiDatabase {
    /// Returns the E-utilities `db` parameter value.
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Nuccore => "nuccore",
            Self::Protein => "protein",
        }
    }

    /// Returns the explicit molecule hint for FASTA parsing.
    #[must_use]
    pub fn molecule_hint(self) -> MoleculeKind {
        match self {
            Self::Nuccore => MoleculeKind::Dna,
            Self::Protein => MoleculeKind::Protein,
        }
    }
}

/// NCBI adapter for E-utilities single-sequence retrieval.
#[derive(Clone, Copy, Debug, Default)]
pub struct NcbiSequenceAdapter;

impl NcbiSequenceAdapter {
    /// Creates an NCBI adapter.
    #[must_use]
    pub fn new() -> Self {
        Self
    }

    fn parse_locator(&self, locator: &str) -> Result<(NcbiDatabase, String), PlatformError> {
        if let Some((database, accession)) = locator.split_once(':') {
            let database = match database.trim().to_ascii_lowercase().as_str() {
                "nuccore" | "nucleotide" => NcbiDatabase::Nuccore,
                "protein" => NcbiDatabase::Protein,
                _ => {
                    return Err(PlatformError::new(
                        ErrorCategory::Validation,
                        "NCBI retrieval requires a supported database selector",
                    )
                    .with_code("providers.sequence.ncbi.unsupported_locator")
                    .with_detail(locator.to_owned()));
                }
            };
            let accession = accession.trim();
            if accession.is_empty() {
                return Err(PlatformError::new(
                    ErrorCategory::Validation,
                    "NCBI retrieval locator is missing an accession after the database selector",
                )
                .with_code("providers.sequence.ncbi.unsupported_locator")
                .with_detail(locator.to_owned()));
            }

            return Ok((database, accession.to_owned()));
        }

        let uppercase = locator.trim().to_ascii_uppercase();
        if uppercase.starts_with("NP_")
            || uppercase.starts_with("XP_")
            || uppercase.starts_with("YP_")
            || uppercase.starts_with("WP_")
        {
            return Ok((NcbiDatabase::Protein, locator.trim().to_owned()));
        }

        if uppercase.starts_with("NM_")
            || uppercase.starts_with("NR_")
            || uppercase.starts_with("NC_")
            || uppercase.starts_with("NG_")
            || uppercase.starts_with("NT_")
            || uppercase.starts_with("NW_")
            || uppercase.starts_with("NZ_")
            || uppercase.starts_with("XM_")
            || uppercase.starts_with("XR_")
        {
            return Ok((NcbiDatabase::Nuccore, locator.trim().to_owned()));
        }

        Err(PlatformError::new(
            ErrorCategory::Validation,
            "NCBI retrieval could not infer a safe database from the locator; use 'nuccore:<accession>' or 'protein:<accession>'",
        )
        .with_code("providers.sequence.ncbi.ambiguous_locator")
        .with_detail(locator.to_owned()))
    }

    /// Builds the NCBI efetch request URL.
    pub fn build_request(
        &self,
        resolution: &SequenceProviderResolution,
    ) -> Result<(NcbiDatabase, String, HttpRequest), PlatformError> {
        let (database, accession) = self.parse_locator(&resolution.locator)?;
        let url = Url::parse_with_params(
            "https://eutils.ncbi.nlm.nih.gov/entrez/eutils/efetch.fcgi",
            &[
                ("db", database.as_str()),
                ("id", accession.as_str()),
                ("rettype", "fasta"),
                ("retmode", "text"),
            ],
        )
        .map_err(|error| {
            PlatformError::new(
                ErrorCategory::Configuration,
                "failed to construct NCBI retrieval URL",
            )
            .with_code("providers.sequence.ncbi.url_build_failed")
            .with_detail(error.to_string())
        })?;

        Ok((
            database,
            accession,
            HttpRequest::new(url.to_string()).with_accept("text/plain, text/x-fasta;q=0.9"),
        ))
    }

    /// Retrieves one FASTA record from NCBI.
    pub fn retrieve<C: ProviderHttpClient>(
        &self,
        _request: &AcquisitionRequest,
        resolution: &SequenceProviderResolution,
        client: &C,
    ) -> Result<RetrievedSequence, PlatformError> {
        let provider = ProviderId::new("ncbi").expect("static provider id is valid");
        let (database, accession, http_request) = self.build_request(resolution)?;
        let response = client.get_text(&http_request)?;
        validate_success_response(&response, "ncbi", &accession)?;

        parse_single_fasta_record(
            &response.body,
            Some(database.molecule_hint()),
            &provider,
            &accession,
            RetrievalRoute::new(
                provider.clone(),
                format!("ncbi.eutils.efetch.{}", database.as_str()),
                RetrievalFormat::Fasta,
            ),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::{NcbiDatabase, NcbiSequenceAdapter};
    use crate::{ProviderId, SequenceProviderResolution};

    #[test]
    fn builds_nuccore_request_for_explicit_database_locator() {
        let adapter = NcbiSequenceAdapter::new();
        let (database, accession, request) = adapter
            .build_request(&SequenceProviderResolution {
                provider: ProviderId::new("ncbi").expect("valid provider"),
                locator: String::from("nuccore:NM_000546.6"),
            })
            .expect("request should build");

        assert_eq!(database, NcbiDatabase::Nuccore);
        assert_eq!(accession, "NM_000546.6");
        assert!(request.url.contains("db=nuccore"));
        assert!(request.url.contains("id=NM_000546.6"));
    }

    #[test]
    fn infers_protein_database_for_safe_refseq_prefixes() {
        let adapter = NcbiSequenceAdapter::new();
        let (database, accession, request) = adapter
            .build_request(&SequenceProviderResolution {
                provider: ProviderId::new("ncbi").expect("valid provider"),
                locator: String::from("NP_000537.3"),
            })
            .expect("request should build");

        assert_eq!(database, NcbiDatabase::Protein);
        assert_eq!(accession, "NP_000537.3");
        assert!(request.url.contains("db=protein"));
    }
}

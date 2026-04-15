//! Documentation-acquisition policy enforcement for autodoc workflows.
//!
//! This module ensures that autodoc and generated-doc workflows do not embed
//! hidden downloader behavior. Local, fixture, legacy, and generated artefacts
//! are accepted directly when declared consistently. Provider-backed artefacts
//! must flow through the formal acquisition seam exposed by the broader
//! EMBOSS-RS platform.

use std::path::PathBuf;

use emboss_config::AutodocPolicy;
use emboss_diagnostics::{ArtifactProvenance, Diagnostic, ErrorCategory, PlatformError};
use emboss_providers::{
    AcquisitionRequest, DocumentationAcquisitionGateway, DocumentationAcquisitionRecord,
    DocumentationAcquisitionRequest, DocumentationAcquisitionRoute, InputReference, ProviderId,
    ResolutionIntent,
};

use crate::contract::{
    AcquisitionMethod, ArtifactOrigin, ArtifactReference, ArtifactSpec, AutodocDocument,
};

/// Provenance-rich enforcement report for autodoc artefact acquisition.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DocumentationAcquisitionReport {
    /// Successfully accepted or resolved artefacts.
    pub records: Vec<DocumentationAcquisitionRecord>,
    /// Non-fatal diagnostics emitted during policy evaluation.
    pub diagnostics: Vec<Diagnostic>,
}

impl DocumentationAcquisitionReport {
    /// Returns the number of artefacts accepted or resolved.
    #[must_use]
    pub fn resolved_count(&self) -> usize {
        self.records.len()
    }
}

/// Enforces documentation-acquisition policy for a validated autodoc document.
pub fn enforce_documentation_acquisition_policy(
    document: &AutodocDocument,
    policy: &AutodocPolicy,
    gateway: Option<&dyn DocumentationAcquisitionGateway>,
) -> Result<DocumentationAcquisitionReport, PlatformError> {
    document.validate()?;

    let mut report = DocumentationAcquisitionReport {
        records: Vec::new(),
        diagnostics: Vec::new(),
    };

    for artifact in &document.artifacts {
        report
            .records
            .push(resolve_artifact(artifact, policy, gateway)?);
    }

    Ok(report)
}

fn resolve_artifact(
    artifact: &ArtifactSpec,
    policy: &AutodocPolicy,
    gateway: Option<&dyn DocumentationAcquisitionGateway>,
) -> Result<DocumentationAcquisitionRecord, PlatformError> {
    reject_direct_download_like_reference(artifact)?;

    match (&artifact.origin, &artifact.acquisition, &artifact.reference) {
        (
            ArtifactOrigin::LocalFile,
            AcquisitionMethod::LocalPath,
            ArtifactReference::Path { path },
        ) => {
            if !policy.allow_local_declared_artifacts {
                return Err(policy_rejection("local declared artefacts", artifact));
            }
            Ok(DocumentationAcquisitionRecord::new(
                artifact.id.clone(),
                DocumentationAcquisitionRoute::LocalDeclared,
                ArtifactProvenance::local_file(path.clone()),
            )
            .with_local_path(PathBuf::from(path)))
        }
        (
            ArtifactOrigin::FixtureAsset,
            AcquisitionMethod::Fixture,
            ArtifactReference::ManagedAsset { asset_id },
        ) => {
            if !policy.allow_fixture_assets {
                return Err(policy_rejection("fixture artefacts", artifact));
            }
            Ok(DocumentationAcquisitionRecord::new(
                artifact.id.clone(),
                DocumentationAcquisitionRoute::FixtureAsset,
                ArtifactProvenance::generated_fixture(asset_id.clone()),
            ))
        }
        (
            ArtifactOrigin::LegacyEmbossReference { source_label },
            AcquisitionMethod::LegacyHarvest,
            ArtifactReference::Path { path },
        ) => {
            if !policy.allow_legacy_harvest_artifacts {
                return Err(policy_rejection("legacy-harvest artefacts", artifact));
            }
            Ok(DocumentationAcquisitionRecord::new(
                artifact.id.clone(),
                DocumentationAcquisitionRoute::LegacyHarvest,
                ArtifactProvenance::legacy_emboss_asset(path.clone())
                    .with_description(source_label.clone()),
            ))
        }
        (
            ArtifactOrigin::GeneratedArtifact,
            AcquisitionMethod::Generated,
            ArtifactReference::Generated { locator },
        ) => {
            if !policy.allow_generated_artifacts {
                return Err(policy_rejection("generated artefacts", artifact));
            }
            Ok(DocumentationAcquisitionRecord::new(
                artifact.id.clone(),
                DocumentationAcquisitionRoute::GeneratedArtifact,
                ArtifactProvenance::generated_output(locator.clone()),
            ))
        }
        (
            _,
            AcquisitionMethod::Provider {
                intent,
                preferred_provider,
            },
            reference,
        ) => {
            if !policy.acquire_through_providers {
                return Err(PlatformError::new(
                    ErrorCategory::Configuration,
                    "provider-backed documentation acquisition is disabled by autodoc policy",
                )
                .with_code("docgen.acquisition.provider_disabled")
                .with_detail(artifact.id.clone()));
            }

            let provider_id = preferred_provider
                .as_deref()
                .map(parse_provider_id)
                .transpose()?;
            let request = DocumentationAcquisitionRequest::new(
                artifact.id.clone(),
                AcquisitionRequest::new(
                    ResolutionIntent::DocumentationAsset,
                    input_reference_from_artifact(reference, provider_id.clone())?,
                )
                .with_preferred_provider_if(provider_id.clone()),
            );

            let gateway = gateway.ok_or_else(|| {
                PlatformError::new(
                    ErrorCategory::NotImplemented,
                    "provider-backed documentation artefact requires a formal EMBOSS-RS acquisition gateway",
                )
                .with_code("docgen.acquisition.gateway_required")
                .with_detail(format!(
                    "artifact '{}' declared provider-backed acquisition for {:?}",
                    artifact.id, intent
                ))
            })?;

            gateway.acquire_documentation_artifact(&request)
        }
        _ => Err(PlatformError::new(
            ErrorCategory::Validation,
            "autodoc artefact declaration does not satisfy documentation acquisition policy",
        )
        .with_code("docgen.acquisition.invalid_declaration")
        .with_detail(format!("artifact '{}'", artifact.id))),
    }
}

fn policy_rejection(label: &str, artifact: &ArtifactSpec) -> PlatformError {
    PlatformError::new(
        ErrorCategory::Configuration,
        "autodoc policy rejected a declared documentation artefact",
    )
    .with_code("docgen.acquisition.policy_rejected")
    .with_detail(format!(
        "{label} are disabled for artifact '{}'",
        artifact.id
    ))
}

fn reject_direct_download_like_reference(artifact: &ArtifactSpec) -> Result<(), PlatformError> {
    let candidate = match &artifact.reference {
        ArtifactReference::Path { path } => Some(path.as_str()),
        ArtifactReference::ProviderLocator { locator, .. } => Some(locator.as_str()),
        ArtifactReference::Generated { locator } => Some(locator.as_str()),
        ArtifactReference::Accession { accession } => Some(accession.as_str()),
        ArtifactReference::ManagedAsset { asset_id } => Some(asset_id.as_str()),
    };

    if let Some(value) = candidate {
        if value.starts_with("http://") || value.starts_with("https://") {
            return Err(PlatformError::new(
                ErrorCategory::Validation,
                "direct downloader-style documentation references are not permitted",
            )
            .with_code("docgen.acquisition.direct_download_disallowed")
            .with_detail(format!(
                "artifact '{}' declared remote locator '{}'",
                artifact.id, value
            )));
        }
    }

    Ok(())
}

fn input_reference_from_artifact(
    reference: &ArtifactReference,
    provider_id: Option<ProviderId>,
) -> Result<InputReference, PlatformError> {
    match reference {
        ArtifactReference::Accession { accession } => {
            Ok(InputReference::accession(accession.clone()))
        }
        ArtifactReference::ProviderLocator { provider, locator } => {
            let provider = provider
                .as_deref()
                .map(parse_provider_id)
                .transpose()?
                .or(provider_id);
            Ok(InputReference::provider_asset(provider, locator.clone()))
        }
        ArtifactReference::ManagedAsset { asset_id } => {
            Ok(InputReference::managed_asset(asset_id.clone()))
        }
        ArtifactReference::Path { path } => Ok(InputReference::local_path(path.clone())),
        ArtifactReference::Generated { locator } => {
            Ok(InputReference::managed_asset(locator.clone()))
        }
    }
}

fn parse_provider_id(provider: &str) -> Result<ProviderId, PlatformError> {
    ProviderId::new(provider).map_err(|error| {
        PlatformError::new(
            ErrorCategory::Validation,
            "invalid provider identifier in autodoc acquisition declaration",
        )
        .with_code("docgen.acquisition.invalid_provider_id")
        .with_detail(error.to_string())
    })
}

trait AcquisitionRequestExt {
    fn with_preferred_provider_if(self, provider: Option<ProviderId>) -> Self;
}

impl AcquisitionRequestExt for AcquisitionRequest {
    fn with_preferred_provider_if(mut self, provider: Option<ProviderId>) -> Self {
        if let Some(provider) = provider {
            self = self.with_preferred_provider(provider);
        }
        self
    }
}

#[cfg(test)]
mod tests {
    use emboss_config::AutodocPolicy;
    use emboss_diagnostics::ArtifactProvenance;
    use emboss_providers::{
        DocumentationAcquisitionGateway, DocumentationAcquisitionRecord,
        DocumentationAcquisitionRequest, DocumentationAcquisitionRoute, ProviderId,
    };

    use crate::{AutodocDocument, load_document_from_path};

    use super::enforce_documentation_acquisition_policy;

    struct FakeGateway;

    impl DocumentationAcquisitionGateway for FakeGateway {
        fn acquire_documentation_artifact(
            &self,
            request: &DocumentationAcquisitionRequest,
        ) -> Result<DocumentationAcquisitionRecord, emboss_diagnostics::PlatformError> {
            Ok(DocumentationAcquisitionRecord::new(
                request.artifact_id.clone(),
                DocumentationAcquisitionRoute::GovernedProvider {
                    provider: Some(ProviderId::new("ena").expect("valid provider id")),
                },
                ArtifactProvenance::provider_asset("AB000263").with_provider("ena"),
            ))
        }
    }

    fn fixture(path: &str) -> std::path::PathBuf {
        std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join(path)
    }

    #[test]
    fn allows_fixture_backed_curated_document() {
        let document = load_document_from_path(fixture("tests/fixtures/minimal_autodoc.json"))
            .expect("fixture should load");
        let report =
            enforce_documentation_acquisition_policy(&document, &AutodocPolicy::default(), None)
                .expect("fixture-backed document should pass policy");

        assert_eq!(report.records.len(), 1);
        assert!(matches!(
            report.records[0].route,
            DocumentationAcquisitionRoute::FixtureAsset
        ));
    }

    #[test]
    fn rejects_provider_backed_document_without_formal_gateway() {
        let document = load_document_from_path(fixture("tests/fixtures/rich_autodoc.json"))
            .expect("fixture should load");
        let error =
            enforce_documentation_acquisition_policy(&document, &AutodocPolicy::default(), None)
                .expect_err("provider-backed document should fail without gateway");

        assert_eq!(error.code(), Some("docgen.acquisition.gateway_required"));
    }

    #[test]
    fn allows_provider_backed_document_through_formal_gateway() {
        let document = load_document_from_path(fixture("tests/fixtures/rich_autodoc.json"))
            .expect("fixture should load");
        let report = enforce_documentation_acquisition_policy(
            &document,
            &AutodocPolicy::default(),
            Some(&FakeGateway),
        )
        .expect("provider-backed document should pass with gateway");

        assert!(report.records.iter().any(|record| matches!(
            record.route,
            DocumentationAcquisitionRoute::GovernedProvider { .. }
        )));
    }

    #[test]
    fn rejects_direct_download_like_reference() {
        let json = r#"
        {
          "schema_version": "emboss-rs.autodoc/v1",
          "document_id": "bad-download",
          "tool": { "name": "needle", "family": null, "summary": null, "legacy_names": [] },
          "sections": [],
          "artifacts": [
            {
              "id": "remote_html",
              "label": "Remote html",
              "origin": { "kind": "local_file" },
              "acquisition": { "mode": "local_path" },
              "reference": { "kind": "path", "path": "https://example.com/doc.html" },
              "description": null
            }
          ],
          "examples": [],
          "provenance": { "source_mode": "curated", "curated_by": null, "source_references": [] },
          "validation": null
        }"#;

        let document = AutodocDocument::from_json_str(json).expect("contract shape should parse");
        let error =
            enforce_documentation_acquisition_policy(&document, &AutodocPolicy::default(), None)
                .expect_err("direct remote reference should fail");

        assert_eq!(
            error.code(),
            Some("docgen.acquisition.direct_download_disallowed")
        );
    }
}

//! Semantic validation for the autodoc JSON contract.

use std::collections::HashSet;

use epithema_diagnostics::{ErrorCategory, PlatformError};

use crate::contract::AutodocDocument;

/// Canonical schema version string for the first formal autodoc JSON contract.
pub const AUTODOC_SCHEMA_VERSION: &str = "epithema.autodoc/v1";

pub(crate) fn validate_document(document: &AutodocDocument) -> Result<(), PlatformError> {
    if document.schema_version != AUTODOC_SCHEMA_VERSION {
        return Err(PlatformError::new(
            ErrorCategory::Validation,
            "unsupported autodoc schema version",
        )
        .with_code("docgen.schema.unsupported_version")
        .with_detail(format!("received '{}'", document.schema_version)));
    }

    ensure_unique(
        document
            .artifacts
            .iter()
            .map(|artifact| artifact.id.as_str()),
        "artifact",
        "docgen.artifact.duplicate_id",
    )?;
    ensure_unique(
        document.sections.iter().map(|section| section.id.as_str()),
        "section",
        "docgen.section.duplicate_id",
    )?;
    ensure_unique(
        document.examples.iter().map(|example| example.id.as_str()),
        "example",
        "docgen.example.duplicate_id",
    )?;

    let artifact_ids: HashSet<&str> = document
        .artifacts
        .iter()
        .map(|artifact| artifact.id.as_str())
        .collect();

    for example in &document.examples {
        for artifact_id in &example.artifact_ids {
            if !artifact_ids.contains(artifact_id.as_str()) {
                return Err(PlatformError::new(
                    ErrorCategory::Validation,
                    "example references an undeclared artifact",
                )
                .with_code("docgen.example.unknown_artifact")
                .with_detail(format!(
                    "example '{}' references missing artifact '{}'",
                    example.id, artifact_id
                )));
            }
        }
    }

    if let Some(validation) = &document.validation {
        for required in &validation.required_example_ids {
            if !document
                .examples
                .iter()
                .any(|example| &example.id == required)
            {
                return Err(PlatformError::new(
                    ErrorCategory::Validation,
                    "validation block references an undeclared example",
                )
                .with_code("docgen.validation.unknown_example")
                .with_detail(format!("required example '{}' is not declared", required)));
            }
        }
    }

    Ok(())
}

fn ensure_unique<'a>(
    ids: impl IntoIterator<Item = &'a str>,
    label: &str,
    code: &str,
) -> Result<(), PlatformError> {
    let mut seen = HashSet::new();

    for id in ids {
        if !seen.insert(id) {
            return Err(PlatformError::new(
                ErrorCategory::Validation,
                format!("duplicate {label} identifier in autodoc contract"),
            )
            .with_code(code)
            .with_detail(format!("duplicate identifier '{id}'")));
        }
    }

    Ok(())
}

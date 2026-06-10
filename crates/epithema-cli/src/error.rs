//! Error types for the `epithema` command surface.

use std::process::ExitCode;

use epithema_diagnostics::{ErrorCategory, PlatformError};
use epithema_docgen::AutodocContractError;

/// CLI-local failures with explicit process exit behavior.
#[derive(Debug)]
pub(crate) struct CliError(PlatformError);

impl CliError {
    pub(crate) fn missing_tool_name() -> Self {
        Self(
            PlatformError::new(
                ErrorCategory::Validation,
                "a tool name is required after `epithema`",
            )
            .with_code("cli.tool.missing_name"),
        )
    }

    pub(crate) fn exit_code(&self) -> ExitCode {
        match self.0.category() {
            ErrorCategory::Internal => ExitCode::from(1),
            _ => ExitCode::from(2),
        }
    }
}

impl std::fmt::Display for CliError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0, f)
    }
}

impl From<PlatformError> for CliError {
    fn from(value: PlatformError) -> Self {
        Self(value)
    }
}

impl From<AutodocContractError> for CliError {
    fn from(value: AutodocContractError) -> Self {
        match value {
            AutodocContractError::Json(error) => Self(
                PlatformError::new(ErrorCategory::Validation, "invalid autodoc JSON document")
                    .with_code("cli.autodoc.invalid_json")
                    .with_detail(error.to_string()),
            ),
            AutodocContractError::Io { path, source } => {
                let error = PlatformError::new(
                    ErrorCategory::Configuration,
                    "failed to read autodoc input",
                )
                .with_code("cli.autodoc.input_read_failed")
                .with_detail(match path {
                    Some(path) => format!("{}: {source}", path.display()),
                    None => source.to_string(),
                });
                Self(error)
            }
            AutodocContractError::Validation(error) => Self(error),
        }
    }
}

//! Error types for the `emboss-rs` command surface.

use std::process::ExitCode;

use emboss_diagnostics::{ErrorCategory, PlatformError};

/// CLI-local failures with explicit process exit behavior.
#[derive(Debug)]
pub(crate) struct CliError(PlatformError);

impl CliError {
    pub(crate) fn missing_tool_name() -> Self {
        Self(
            PlatformError::new(
                ErrorCategory::Validation,
                "a tool name is required after `emboss-rs`",
            )
            .with_code("cli.tool.missing_name"),
        )
    }

    pub(crate) fn tool_arguments_not_implemented(tool: String) -> Self {
        Self(
            PlatformError::new(
                ErrorCategory::NotImplemented,
                format!("tool argument forwarding for '{tool}' is not implemented yet"),
            )
            .with_code("cli.tool.arguments_not_implemented"),
        )
    }

    pub(crate) fn autodoc_not_implemented() -> Self {
        Self(
            PlatformError::new(
                ErrorCategory::NotImplemented,
                "`emboss-rs autodoc` is reserved but not implemented yet",
            )
            .with_code("cli.autodoc.not_implemented"),
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

//! Error types for the `emboss-rs` command surface.

use std::fmt::{Display, Formatter};
use std::process::ExitCode;

use emboss_service::ServiceError;

/// CLI-local failures with explicit process exit behavior.
#[derive(Debug)]
pub(crate) enum CliError {
    MissingToolName,
    ToolArgumentsNotImplemented { tool: String },
    AutodocNotImplemented,
    Service(ServiceError),
}

impl CliError {
    pub(crate) fn missing_tool_name() -> Self {
        Self::MissingToolName
    }

    pub(crate) fn tool_arguments_not_implemented(tool: String) -> Self {
        Self::ToolArgumentsNotImplemented { tool }
    }

    pub(crate) fn autodoc_not_implemented() -> Self {
        Self::AutodocNotImplemented
    }

    pub(crate) fn exit_code(&self) -> ExitCode {
        ExitCode::from(2)
    }
}

impl Display for CliError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MissingToolName => write!(f, "a tool name is required after `emboss-rs`"),
            Self::ToolArgumentsNotImplemented { tool } => write!(
                f,
                "tool argument forwarding for '{tool}' is not implemented yet"
            ),
            Self::AutodocNotImplemented => {
                write!(f, "`emboss-rs autodoc` is reserved but not implemented yet")
            }
            Self::Service(error) => Display::fmt(error, f),
        }
    }
}

impl From<ServiceError> for CliError {
    fn from(value: ServiceError) -> Self {
        Self::Service(value)
    }
}

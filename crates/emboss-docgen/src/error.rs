//! Error types for autodoc contract parsing and validation.

use std::error::Error;
use std::fmt::{Display, Formatter};

use emboss_diagnostics::{ErrorCategory, PlatformError};

/// Autodoc contract failures detected during parsing or validation.
#[derive(Debug)]
pub enum AutodocContractError {
    /// JSON deserialization failed.
    Json(serde_json::Error),
    /// The contract failed semantic validation.
    Validation(PlatformError),
}

impl Display for AutodocContractError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Json(error) => Display::fmt(error, f),
            Self::Validation(error) => Display::fmt(error, f),
        }
    }
}

impl Error for AutodocContractError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Json(error) => Some(error),
            Self::Validation(error) => Some(error),
        }
    }
}

impl From<serde_json::Error> for AutodocContractError {
    fn from(value: serde_json::Error) -> Self {
        Self::Json(value)
    }
}

impl From<PlatformError> for AutodocContractError {
    fn from(value: PlatformError) -> Self {
        Self::Validation(value)
    }
}

impl AutodocContractError {
    /// Returns the shared platform category for semantic validation failures.
    #[must_use]
    pub fn category(&self) -> ErrorCategory {
        match self {
            Self::Json(_) => ErrorCategory::Validation,
            Self::Validation(error) => error.category(),
        }
    }
}

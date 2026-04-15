//! Error types for autodoc contract parsing and validation.

use std::error::Error;
use std::fmt::{Display, Formatter};
use std::path::PathBuf;

use emboss_diagnostics::{ErrorCategory, PlatformError};

/// Autodoc contract failures detected during parsing or validation.
#[derive(Debug)]
pub enum AutodocContractError {
    /// JSON deserialization failed.
    Json(serde_json::Error),
    /// Input file could not be read.
    Io {
        /// Path that failed to load.
        path: Option<PathBuf>,
        /// Underlying I/O error.
        source: std::io::Error,
    },
    /// The contract failed semantic validation.
    Validation(PlatformError),
}

impl Display for AutodocContractError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Json(error) => Display::fmt(error, f),
            Self::Io { path, source } => {
                if let Some(path) = path {
                    write!(
                        f,
                        "failed to read autodoc input '{}': {source}",
                        path.display()
                    )
                } else {
                    write!(f, "failed to read autodoc input: {source}")
                }
            }
            Self::Validation(error) => Display::fmt(error, f),
        }
    }
}

impl Error for AutodocContractError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Json(error) => Some(error),
            Self::Io { source, .. } => Some(source),
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
    /// Builds an I/O loading error without an associated path.
    #[must_use]
    pub fn from_io(source: std::io::Error) -> Self {
        Self::Io { path: None, source }
    }

    /// Attaches the source path to an existing I/O loading error.
    #[must_use]
    pub fn with_path(self, path: impl Into<PathBuf>) -> Self {
        match self {
            Self::Io { source, .. } => Self::Io {
                path: Some(path.into()),
                source,
            },
            other => other,
        }
    }

    /// Returns the shared platform category for semantic validation failures.
    #[must_use]
    pub fn category(&self) -> ErrorCategory {
        match self {
            Self::Json(_) => ErrorCategory::Validation,
            Self::Io { .. } => ErrorCategory::Configuration,
            Self::Validation(error) => error.category(),
        }
    }
}

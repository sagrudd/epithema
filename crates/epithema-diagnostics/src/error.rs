//! Shared platform error types.

use std::error::Error;
use std::fmt::{Display, Formatter};

/// Stable category for a platform error.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ErrorCategory {
    /// Validation or user-input problem.
    Validation,
    /// Registry or governed surface resolution problem.
    Registry,
    /// Invocation or execution boundary problem.
    Invocation,
    /// Configuration or environment problem.
    Configuration,
    /// Feature exists as governed surface but is not implemented yet.
    NotImplemented,
    /// Internal platform invariant or unexpected failure.
    Internal,
}

impl Display for ErrorCategory {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Validation => f.write_str("validation"),
            Self::Registry => f.write_str("registry"),
            Self::Invocation => f.write_str("invocation"),
            Self::Configuration => f.write_str("configuration"),
            Self::NotImplemented => f.write_str("not-implemented"),
            Self::Internal => f.write_str("internal"),
        }
    }
}

/// Shared error type for platform-facing failures.
#[derive(Debug)]
pub struct PlatformError {
    category: ErrorCategory,
    code: Option<String>,
    message: String,
    detail: Option<String>,
    source: Option<Box<dyn Error + Send + Sync + 'static>>,
}

impl PlatformError {
    /// Creates a platform error from a category and human-readable message.
    #[must_use]
    pub fn new(category: ErrorCategory, message: impl Into<String>) -> Self {
        Self {
            category,
            code: None,
            message: message.into(),
            detail: None,
            source: None,
        }
    }

    /// Adds a stable machine-oriented code to the error.
    #[must_use]
    pub fn with_code(mut self, code: impl Into<String>) -> Self {
        self.code = Some(code.into());
        self
    }

    /// Adds optional human-readable detail for callers.
    #[must_use]
    pub fn with_detail(mut self, detail: impl Into<String>) -> Self {
        self.detail = Some(detail.into());
        self
    }

    /// Attaches an error source for chained reporting.
    #[must_use]
    pub fn with_source<E>(mut self, source: E) -> Self
    where
        E: Error + Send + Sync + 'static,
    {
        self.source = Some(Box::new(source));
        self
    }

    /// Returns the stable category for the error.
    #[must_use]
    pub fn category(&self) -> ErrorCategory {
        self.category
    }

    /// Returns the error code when present.
    #[must_use]
    pub fn code(&self) -> Option<&str> {
        self.code.as_deref()
    }

    /// Returns the primary error message.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }

    /// Returns optional detail associated with the error.
    #[must_use]
    pub fn detail(&self) -> Option<&str> {
        self.detail.as_deref()
    }
}

impl Display for PlatformError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.message)?;

        if let Some(detail) = self.detail() {
            write!(f, ": {detail}")?;
        }

        Ok(())
    }
}

impl Error for PlatformError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        self.source.as_deref().map(|source| source as _)
    }
}

#[cfg(test)]
mod tests {
    use super::{ErrorCategory, PlatformError};

    #[test]
    fn formats_error_message_and_detail() {
        let error = PlatformError::new(ErrorCategory::Validation, "tool name must not be empty")
            .with_code("service.tool_name.empty")
            .with_detail("received only whitespace");

        assert_eq!(error.code(), Some("service.tool_name.empty"));
        assert_eq!(
            error.to_string(),
            "tool name must not be empty: received only whitespace"
        );
    }
}

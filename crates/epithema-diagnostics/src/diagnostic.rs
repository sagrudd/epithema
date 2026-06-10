//! Structured non-fatal diagnostics.

use std::fmt::{Display, Formatter};

use crate::Severity;

/// Optional location or scope associated with a diagnostic.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DiagnosticLocation {
    scope: String,
}

impl DiagnosticLocation {
    /// Creates a generic location or scope label.
    #[must_use]
    pub fn new(scope: impl Into<String>) -> Self {
        Self {
            scope: scope.into(),
        }
    }

    /// Returns the scoped location label.
    #[must_use]
    pub fn scope(&self) -> &str {
        &self.scope
    }
}

/// Non-fatal structured information emitted during execution or validation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Diagnostic {
    /// Severity associated with the diagnostic.
    pub severity: Severity,
    code: Option<String>,
    message: String,
    context: Option<String>,
    location: Option<DiagnosticLocation>,
}

impl Diagnostic {
    /// Creates a diagnostic with the supplied severity and message.
    #[must_use]
    pub fn new(severity: Severity, message: impl Into<String>) -> Self {
        Self {
            severity,
            code: None,
            message: message.into(),
            context: None,
            location: None,
        }
    }

    /// Adds a stable machine-oriented code to the diagnostic.
    #[must_use]
    pub fn with_code(mut self, code: impl Into<String>) -> Self {
        self.code = Some(code.into());
        self
    }

    /// Adds contextual detail for the diagnostic.
    #[must_use]
    pub fn with_context(mut self, context: impl Into<String>) -> Self {
        self.context = Some(context.into());
        self
    }

    /// Adds an optional location or scope to the diagnostic.
    #[must_use]
    pub fn with_location(mut self, location: DiagnosticLocation) -> Self {
        self.location = Some(location);
        self
    }

    /// Returns the diagnostic code when present.
    #[must_use]
    pub fn code(&self) -> Option<&str> {
        self.code.as_deref()
    }

    /// Returns the main human-readable diagnostic message.
    #[must_use]
    pub fn message(&self) -> &str {
        &self.message
    }

    /// Returns optional contextual detail.
    #[must_use]
    pub fn context(&self) -> Option<&str> {
        self.context.as_deref()
    }

    /// Returns the optional diagnostic location.
    #[must_use]
    pub fn location(&self) -> Option<&DiagnosticLocation> {
        self.location.as_ref()
    }
}

impl Display for Diagnostic {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.severity, self.message)?;

        if let Some(code) = self.code() {
            write!(f, " [{code}]")?;
        }

        if let Some(location) = self.location() {
            write!(f, " @ {}", location.scope())?;
        }

        Ok(())
    }
}

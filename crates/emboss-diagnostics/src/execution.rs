//! Execution-context and run-metadata primitives.

use std::fmt::{Display, Formatter};
use std::process;
use std::time::{SystemTime, UNIX_EPOCH};

/// The calling surface that initiated platform work.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Hash)]
pub enum InvocationOrigin {
    /// Invocation from the `emboss-rs` CLI.
    Cli,
    /// Invocation from the sister `emboss-r` interface.
    R,
    /// Invocation from documentation generation or autodoc.
    Autodoc,
    /// Invocation from validation or harness tooling.
    Validation,
    /// Invocation from a future network or API surface.
    Api,
    /// Invocation origin is not yet specified.
    #[default]
    Unknown,
}

impl InvocationOrigin {
    /// Returns a stable lower-case label for the invocation origin.
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Cli => "cli",
            Self::R => "r",
            Self::Autodoc => "autodoc",
            Self::Validation => "validation",
            Self::Api => "api",
            Self::Unknown => "unknown",
        }
    }
}

impl Display for InvocationOrigin {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Shared execution context used across front ends and service boundaries.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ExecutionContext {
    /// Front-end surface that initiated the request.
    pub origin: InvocationOrigin,
    /// Optional caller-assigned request label.
    pub request_label: Option<String>,
    /// Whether the request should avoid durable side effects.
    pub dry_run: bool,
}

impl ExecutionContext {
    /// Creates a context for the supplied origin.
    #[must_use]
    pub fn for_origin(origin: InvocationOrigin) -> Self {
        Self {
            origin,
            request_label: None,
            dry_run: false,
        }
    }

    /// Creates a context for CLI-originated work.
    #[must_use]
    pub fn cli() -> Self {
        Self::for_origin(InvocationOrigin::Cli)
    }

    /// Attaches a request label for audit or tracing purposes.
    #[must_use]
    pub fn with_request_label(mut self, request_label: impl Into<String>) -> Self {
        self.request_label = Some(request_label.into());
        self
    }

    /// Marks the request as dry-run.
    #[must_use]
    pub fn with_dry_run(mut self, dry_run: bool) -> Self {
        self.dry_run = dry_run;
        self
    }
}

/// Stable identifier for a single platform run or invocation attempt.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct RunId(String);

impl RunId {
    /// Generates a run identifier using origin, time, and process identity.
    #[must_use]
    pub fn generate(origin: InvocationOrigin) -> Self {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_nanos();

        Self(format!(
            "{}-{timestamp:x}-{:x}",
            origin.as_str(),
            process::id()
        ))
    }

    /// Returns the run identifier as a string slice.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Display for RunId {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Runtime metadata attached to a single execution attempt.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExecutionMetadata {
    /// Unique run identifier for the execution attempt.
    pub run_id: RunId,
    /// Timestamp recorded when the metadata was created.
    pub started_at: SystemTime,
    /// Calling surface that initiated the run.
    pub origin: InvocationOrigin,
    /// Binary or package identity associated with the run.
    pub binary_name: String,
    /// Package version associated with the run.
    pub package_version: String,
    /// Optional request label copied from execution context.
    pub request_label: Option<String>,
    /// Whether the run was requested as dry-run.
    pub dry_run: bool,
}

impl ExecutionMetadata {
    /// Builds runtime metadata from an execution context.
    #[must_use]
    pub fn from_context(
        context: &ExecutionContext,
        binary_name: impl Into<String>,
        package_version: impl Into<String>,
    ) -> Self {
        Self {
            run_id: RunId::generate(context.origin),
            started_at: SystemTime::now(),
            origin: context.origin,
            binary_name: binary_name.into(),
            package_version: package_version.into(),
            request_label: context.request_label.clone(),
            dry_run: context.dry_run,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{ExecutionContext, ExecutionMetadata, InvocationOrigin};

    #[test]
    fn defaults_to_unknown_origin() {
        let context = ExecutionContext::default();
        assert_eq!(context.origin, InvocationOrigin::Unknown);
        assert!(!context.dry_run);
    }

    #[test]
    fn metadata_captures_context_fields() {
        let context = ExecutionContext::for_origin(InvocationOrigin::Validation)
            .with_request_label("smoke-check")
            .with_dry_run(true);
        let metadata = ExecutionMetadata::from_context(&context, "emboss-rs", "0.1.0");

        assert_eq!(metadata.origin, InvocationOrigin::Validation);
        assert_eq!(metadata.request_label.as_deref(), Some("smoke-check"));
        assert!(metadata.dry_run);
        assert!(metadata.run_id.as_str().starts_with("validation-"));
    }
}

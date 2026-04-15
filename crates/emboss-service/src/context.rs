//! Execution context shared by all EMBOSS-RS front ends.

/// The calling surface that initiated a service request.
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

/// Shared execution context for service requests.
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

#[cfg(test)]
mod tests {
    use super::{ExecutionContext, InvocationOrigin};

    #[test]
    fn defaults_to_unknown_origin() {
        let context = ExecutionContext::default();
        assert_eq!(context.origin, InvocationOrigin::Unknown);
        assert!(!context.dry_run);
    }
}

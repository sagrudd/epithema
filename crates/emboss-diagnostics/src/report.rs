//! Aggregated execution reports.

use crate::{ArtifactProvenance, Diagnostic, ExecutionContext, ExecutionMetadata, Severity};

/// Overall status recorded for an execution attempt.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OutcomeStatus {
    /// Execution completed successfully.
    Succeeded,
    /// Execution failed.
    Failed,
    /// Execution reached a governed but not-yet-implemented path.
    NotImplemented,
}

/// High-level outcome summary for an execution attempt.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExecutionOutcome {
    /// Status recorded for the execution attempt.
    pub status: OutcomeStatus,
    summary: Option<String>,
}

impl ExecutionOutcome {
    /// Creates an execution outcome from a status.
    #[must_use]
    pub fn new(status: OutcomeStatus) -> Self {
        Self {
            status,
            summary: None,
        }
    }

    /// Adds an optional summary message.
    #[must_use]
    pub fn with_summary(mut self, summary: impl Into<String>) -> Self {
        self.summary = Some(summary.into());
        self
    }

    /// Returns the outcome summary when present.
    #[must_use]
    pub fn summary(&self) -> Option<&str> {
        self.summary.as_deref()
    }
}

/// Structured report carrying metadata, diagnostics, provenance, and outcome.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExecutionReport {
    /// Run metadata for the execution attempt.
    pub metadata: ExecutionMetadata,
    /// Overall execution outcome.
    pub outcome: ExecutionOutcome,
    diagnostics: Vec<Diagnostic>,
    provenance: Vec<ArtifactProvenance>,
}

impl ExecutionReport {
    /// Creates an execution report from metadata and outcome.
    #[must_use]
    pub fn new(metadata: ExecutionMetadata, outcome: ExecutionOutcome) -> Self {
        Self {
            metadata,
            outcome,
            diagnostics: Vec::new(),
            provenance: Vec::new(),
        }
    }

    /// Creates an execution report directly from execution context.
    #[must_use]
    pub fn from_context(
        context: &ExecutionContext,
        binary_name: impl Into<String>,
        package_version: impl Into<String>,
        outcome: ExecutionOutcome,
    ) -> Self {
        Self::new(
            ExecutionMetadata::from_context(context, binary_name, package_version),
            outcome,
        )
    }

    /// Appends a diagnostic to the report.
    pub fn push_diagnostic(&mut self, diagnostic: Diagnostic) {
        self.diagnostics.push(diagnostic);
    }

    /// Appends provenance information to the report.
    pub fn push_provenance(&mut self, provenance: ArtifactProvenance) {
        self.provenance.push(provenance);
    }

    /// Returns all recorded diagnostics.
    #[must_use]
    pub fn diagnostics(&self) -> &[Diagnostic] {
        &self.diagnostics
    }

    /// Returns all recorded provenance entries.
    #[must_use]
    pub fn provenance(&self) -> &[ArtifactProvenance] {
        &self.provenance
    }

    /// Returns true when the report contains an error-level diagnostic.
    #[must_use]
    pub fn has_errors(&self) -> bool {
        self.diagnostics
            .iter()
            .any(|diagnostic| diagnostic.severity == Severity::Error)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        ArtifactProvenance, Diagnostic, ExecutionContext, ExecutionOutcome, ExecutionReport,
        InvocationOrigin, OutcomeStatus, Severity,
    };

    #[test]
    fn report_aggregates_diagnostics_and_provenance() {
        let context = ExecutionContext::for_origin(InvocationOrigin::Autodoc);
        let outcome = ExecutionOutcome::new(OutcomeStatus::Failed).with_summary("input mismatch");
        let mut report = ExecutionReport::from_context(
            &context,
            "emboss-rs",
            env!("CARGO_PKG_VERSION"),
            outcome,
        );

        report.push_diagnostic(
            Diagnostic::new(Severity::Warning, "historical metadata was incomplete")
                .with_code("autodoc.metadata.incomplete"),
        );
        report.push_diagnostic(Diagnostic::new(
            Severity::Error,
            "documentation build aborted",
        ));
        report.push_provenance(
            ArtifactProvenance::local_file("docs/input.json")
                .with_description("autodoc input contract"),
        );

        assert_eq!(report.diagnostics().len(), 2);
        assert_eq!(report.provenance().len(), 1);
        assert!(report.has_errors());
        assert_eq!(report.outcome.summary(), Some("input mismatch"));
    }
}

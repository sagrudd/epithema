//! Shared method-result and reporting objects.
//!
//! These types form the front-end-neutral output seam for Epithema methods.
//! Tools should return structured result envelopes here rather than formatting
//! ad hoc CLI text or bridge-specific projections directly.

use std::path::PathBuf;

use epithema_core::{Alignment, Feature, SequenceRecord};
use epithema_diagnostics::{ArtifactProvenance, ExecutionReport};
use epithema_plot_contract::PlotPayload;

use crate::tool::ToolName;

/// High-level category of a produced output artefact.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ArtifactKind {
    /// Sequence file or sequence-like output.
    Sequence,
    /// Alignment file or alignment-like output.
    Alignment,
    /// Annotation or feature-table output.
    FeatureTable,
    /// Tabular report or statistics output.
    Table,
    /// Human-readable text report.
    Text,
    /// Plot-oriented or other auxiliary output.
    Auxiliary,
}

/// Reference to an output artefact produced by a method.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ArtifactReference {
    /// Stable logical artefact identifier.
    pub id: String,
    /// Artefact category.
    pub kind: ArtifactKind,
    /// Optional short human-readable label.
    pub label: Option<String>,
    /// Optional materialized local path.
    pub local_path: Option<PathBuf>,
    /// Optional structured provenance.
    pub provenance: Option<ArtifactProvenance>,
}

impl ArtifactReference {
    /// Creates a new output artefact reference.
    #[must_use]
    pub fn new(id: impl Into<String>, kind: ArtifactKind) -> Self {
        Self {
            id: id.into(),
            kind,
            label: None,
            local_path: None,
            provenance: None,
        }
    }

    /// Adds a short human-readable label.
    #[must_use]
    pub fn with_label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    /// Attaches a local path.
    #[must_use]
    pub fn with_local_path(mut self, path: impl Into<PathBuf>) -> Self {
        self.local_path = Some(path.into());
        self
    }

    /// Attaches artefact provenance.
    #[must_use]
    pub fn with_provenance(mut self, provenance: ArtifactProvenance) -> Self {
        self.provenance = Some(provenance);
        self
    }
}

/// Compact summary intended for CLI, docs, and bridge-friendly projections.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResultSummary {
    /// Short title for the result.
    pub title: String,
    /// Stable summary lines in display order.
    pub lines: Vec<String>,
}

impl ResultSummary {
    /// Creates a new summary with no lines.
    #[must_use]
    pub fn new(title: impl Into<String>) -> Self {
        Self {
            title: title.into(),
            lines: Vec::new(),
        }
    }

    /// Appends a summary line.
    #[must_use]
    pub fn with_line(mut self, line: impl Into<String>) -> Self {
        self.lines.push(line.into());
        self
    }
}

/// Compact human-readable text report.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TextReport {
    /// Optional report title.
    pub title: Option<String>,
    /// Report body text.
    pub body: String,
}

impl TextReport {
    /// Creates a text report from body text.
    #[must_use]
    pub fn new(body: impl Into<String>) -> Self {
        Self {
            title: None,
            body: body.into(),
        }
    }

    /// Sets a report title.
    #[must_use]
    pub fn with_title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());
        self
    }
}

/// Compact tabular report.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TableReport {
    /// Ordered column headings.
    pub columns: Vec<String>,
    /// Ordered row values.
    pub rows: Vec<Vec<String>>,
}

impl TableReport {
    /// Creates a table report.
    #[must_use]
    pub fn new(columns: Vec<String>, rows: Vec<Vec<String>>) -> Self {
        Self { columns, rows }
    }

    /// Returns the row count.
    #[must_use]
    pub fn row_count(&self) -> usize {
        self.rows.len()
    }
}

/// Shared primary scientific payload families.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ResultPayload {
    /// No primary payload was produced.
    Empty,
    /// Sequence payload.
    Sequence(SequenceRecord),
    /// Multi-record sequence payload.
    SequenceCollection(Vec<SequenceRecord>),
    /// Partitioned multi-record sequence payload.
    SequencePartitions(Vec<Vec<SequenceRecord>>),
    /// Alignment payload.
    Alignment(Alignment),
    /// Feature payload.
    Features(Vec<Feature>),
    /// Compact human-readable text report.
    TextReport(TextReport),
    /// Compact tabular report.
    TableReport(TableReport),
}

impl ResultPayload {
    /// Returns a stable payload family label.
    #[must_use]
    pub fn kind_label(&self) -> &'static str {
        match self {
            Self::Empty => "empty",
            Self::Sequence(_) => "sequence",
            Self::SequenceCollection(_) => "sequence-collection",
            Self::SequencePartitions(_) => "sequence-partitions",
            Self::Alignment(_) => "alignment",
            Self::Features(_) => "features",
            Self::TextReport(_) => "text-report",
            Self::TableReport(_) => "table-report",
        }
    }
}

/// Front-end-neutral method result envelope.
#[derive(Clone, Debug, PartialEq)]
pub struct MethodResult {
    /// Tool or method identifier that produced the result.
    pub tool: ToolName,
    /// Primary scientific or report payload.
    pub payload: ResultPayload,
    /// Compact result summary.
    pub summary: ResultSummary,
    /// Optional plot-ready payload for the R backend.
    pub plot: Option<PlotPayload>,
    /// Auxiliary output artefacts.
    pub artifacts: Vec<ArtifactReference>,
    /// Structured execution report carrying outcome, diagnostics, and provenance.
    pub report: ExecutionReport,
}

impl MethodResult {
    /// Creates a new method result from tool, payload, summary, and report.
    #[must_use]
    pub fn new(
        tool: ToolName,
        payload: ResultPayload,
        summary: ResultSummary,
        report: ExecutionReport,
    ) -> Self {
        Self {
            tool,
            payload,
            summary,
            plot: None,
            artifacts: Vec::new(),
            report,
        }
    }

    /// Attaches a plot-ready payload.
    #[must_use]
    pub fn with_plot(mut self, plot: PlotPayload) -> Self {
        self.plot = Some(plot);
        self
    }

    /// Attaches an auxiliary artefact reference.
    #[must_use]
    pub fn with_artifact(mut self, artifact: ArtifactReference) -> Self {
        self.artifacts.push(artifact);
        self
    }
}

#[cfg(test)]
mod tests {
    use epithema_core::{MoleculeKind, SequenceIdentifier, SequenceRecord};
    use epithema_diagnostics::{
        ArtifactProvenance, Diagnostic, ExecutionContext, ExecutionOutcome, ExecutionReport,
        InvocationOrigin, OutcomeStatus, Severity,
    };

    use super::{ArtifactKind, ArtifactReference, MethodResult, ResultPayload, ResultSummary};
    use crate::ToolName;

    #[test]
    fn builds_result_envelope_with_artifact_and_diagnostics() {
        let context = ExecutionContext::for_origin(InvocationOrigin::Cli);
        let mut report = ExecutionReport::from_context(
            &context,
            "epithema",
            env!("CARGO_PKG_VERSION"),
            ExecutionOutcome::new(OutcomeStatus::Succeeded).with_summary("completed"),
        );
        report.push_diagnostic(
            Diagnostic::new(Severity::Warning, "minor caveat").with_code("result.warning"),
        );
        report.push_provenance(
            ArtifactProvenance::generated_output("result.fa").with_description("sequence output"),
        );

        let payload = ResultPayload::Sequence(
            SequenceRecord::new(
                SequenceIdentifier::new("seq1").expect("valid identifier"),
                MoleculeKind::Dna,
                "ACGT",
            )
            .expect("sequence should build"),
        );
        let result = MethodResult::new(
            ToolName::new("seqret").expect("tool name should build"),
            payload,
            ResultSummary::new("Sequence result").with_line("Length: 4"),
            report,
        )
        .with_artifact(
            ArtifactReference::new("primary-sequence", ArtifactKind::Sequence)
                .with_label("Primary sequence")
                .with_provenance(ArtifactProvenance::generated_output("result.fa")),
        );

        assert_eq!(result.summary.title, "Sequence result");
        assert_eq!(result.artifacts.len(), 1);
        assert!(result.plot.is_none());
        assert_eq!(result.report.diagnostics().len(), 1);
        assert_eq!(result.report.provenance().len(), 1);
        assert_eq!(result.payload.kind_label(), "sequence");
    }
}

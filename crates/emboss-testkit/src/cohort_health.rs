//! Standing cohort-health report for roadmap reprioritization.

use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

use emboss_diagnostics::{ErrorCategory, PlatformError};
use serde::{Deserialize, Serialize};

use crate::governance::{
    GovernanceAlignmentMethodRecord, GovernanceAlignmentReport, derive_governance_alignment_report,
};
use crate::report::{
    CohortEvidenceLevel, CohortValidationReport, derive_shipped_cohort_validation_report,
};

/// Severity of a roadmap reprioritization signal.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CohortHealthSignalSeverity {
    /// Informational signal that still deserves review.
    Notice,
    /// Stronger signal that should drive the next roadmap reorder.
    Warning,
}

/// Stable signal code for roadmap reprioritization.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CohortHealthSignalCode {
    /// Current family with the largest retained unshipped backlog.
    DominantRetainedBacklog,
    /// Current shipped family with the largest weak-evidence burden.
    WeakEvidenceBurden,
    /// Release-truth documentation has fallen behind the shipped/generated state.
    ReleaseTruthLag,
}

/// One surfaced reprioritization signal.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct CohortHealthSignal {
    /// Stable signal code.
    pub code: CohortHealthSignalCode,
    /// Signal severity.
    pub severity: CohortHealthSignalSeverity,
    /// Short summary line.
    pub summary: String,
    /// Supporting detail for maintainers.
    pub detail: String,
}

/// One ordered recommendation for the next roadmap action.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct CohortHealthRecommendation {
    /// Stable ordering priority.
    pub priority: usize,
    /// Short target label.
    pub target: String,
    /// Human-readable rationale.
    pub rationale: String,
    /// Triggering signal.
    pub source_signal: CohortHealthSignalCode,
}

/// Aggregate cohort-health summary.
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct CohortHealthSummary {
    /// Total shipped method count.
    pub total_method_count: usize,
    /// Shipped methods with compared evidence.
    pub compared_evidence_count: usize,
    /// Shipped methods with harvested legacy provenance recorded.
    pub harvested_legacy_presence_count: usize,
    /// Retained governance backlog still unshipped.
    pub retained_backlog_count: usize,
    /// Current family with the largest retained backlog.
    pub largest_retained_backlog_family: Option<String>,
    /// Size of the largest retained backlog.
    pub largest_retained_backlog_size: usize,
    /// Current shipped family with the largest weak-evidence burden.
    pub weakest_evidence_family: Option<String>,
    /// Number of weak-evidence shipped methods in that family.
    pub weak_evidence_method_count: usize,
    /// Whether the RC readiness report reflects the current generated counts.
    pub release_truth_current: bool,
}

/// Generated cohort-health report.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct CohortHealthReport {
    /// Stable schema version.
    pub schema_version: u32,
    /// Stable report identifier.
    pub report_id: String,
    /// Current generated summary.
    pub summary: CohortHealthSummary,
    /// Signals that should drive roadmap reorder decisions.
    pub signals: Vec<CohortHealthSignal>,
    /// Ordered next recommendations.
    pub recommendations: Vec<CohortHealthRecommendation>,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
struct ReadinessAlignment {
    release_truth_current: bool,
    missing_markers: Vec<String>,
}

/// Derives the standing cohort-health report for roadmap reprioritization.
pub fn derive_cohort_health_report(
    repo_root: impl AsRef<Path>,
) -> Result<CohortHealthReport, PlatformError> {
    let repo_root = repo_root.as_ref();
    let cohort = derive_shipped_cohort_validation_report(repo_root)?;
    let governance = derive_governance_alignment_report(repo_root)?;
    let readiness = assess_readiness_alignment(repo_root, &cohort, &governance)?;

    let largest_backlog_family = governance
        .families
        .iter()
        .max_by_key(|family| family.retained_backlog);
    let weak_evidence_family = dominant_weak_evidence_family(&governance.shipped_methods);

    let summary = CohortHealthSummary {
        total_method_count: cohort.summary.total_method_count,
        compared_evidence_count: cohort.summary.compared_evidence_count,
        harvested_legacy_presence_count: cohort.summary.harvested_legacy_presence_count,
        retained_backlog_count: governance.summary.retained_backlog_count,
        largest_retained_backlog_family: largest_backlog_family.and_then(|family| {
            (family.retained_backlog > 0).then(|| family.governance_family.clone())
        }),
        largest_retained_backlog_size: largest_backlog_family
            .map(|family| family.retained_backlog)
            .unwrap_or_default(),
        weakest_evidence_family: weak_evidence_family
            .as_ref()
            .map(|(family, _, _)| family.clone()),
        weak_evidence_method_count: weak_evidence_family
            .as_ref()
            .map(|(_, weak_count, _)| *weak_count)
            .unwrap_or_default(),
        release_truth_current: readiness.release_truth_current,
    };

    let mut signals = Vec::new();
    if let Some(family) = &summary.largest_retained_backlog_family {
        signals.push(CohortHealthSignal {
            code: CohortHealthSignalCode::DominantRetainedBacklog,
            severity: CohortHealthSignalSeverity::Warning,
            summary: format!("largest retained backlog remains in '{}'", family),
            detail: format!(
                "The governance report shows {} retained unshipped methods in '{}'.",
                summary.largest_retained_backlog_size, family
            ),
        });
    }
    if let Some((family, weak_count, compared_count)) = &weak_evidence_family {
        signals.push(CohortHealthSignal {
            code: CohortHealthSignalCode::WeakEvidenceBurden,
            severity: if *weak_count >= 5 {
                CohortHealthSignalSeverity::Warning
            } else {
                CohortHealthSignalSeverity::Notice
            },
            summary: format!("'{}' carries the largest weak-evidence burden", family),
            detail: format!(
                "'{}' has {} shipped methods below compared evidence and {} already compared.",
                family, weak_count, compared_count
            ),
        });
    }
    if !readiness.release_truth_current {
        signals.push(CohortHealthSignal {
            code: CohortHealthSignalCode::ReleaseTruthLag,
            severity: CohortHealthSignalSeverity::Warning,
            summary: "release-truth documentation is behind the current generated state".to_owned(),
            detail: format!(
                "The RC readiness document is missing current markers for: {}.",
                readiness.missing_markers.join(", ")
            ),
        });
    }

    let mut recommendations = Vec::new();
    let mut priority = 1usize;
    if !readiness.release_truth_current {
        recommendations.push(CohortHealthRecommendation {
            priority,
            target: "release readiness truth".to_owned(),
            rationale: "Refresh the RC readiness material before adding more shipped scope so release-facing documentation does not lag the generated cohort state.".to_owned(),
            source_signal: CohortHealthSignalCode::ReleaseTruthLag,
        });
        priority += 1;
    }
    if let Some(family) = &summary.largest_retained_backlog_family {
        recommendations.push(CohortHealthRecommendation {
            priority,
            target: family.clone(),
            rationale: format!(
                "This remains the largest retained backlog family with {} unshipped methods, so it should stay ahead of smaller backlog sweeps.",
                summary.largest_retained_backlog_size
            ),
            source_signal: CohortHealthSignalCode::DominantRetainedBacklog,
        });
        priority += 1;
    }
    if let Some((family, weak_count, _)) = &weak_evidence_family {
        recommendations.push(CohortHealthRecommendation {
            priority,
            target: family.clone(),
            rationale: format!(
                "This family has {} shipped methods still below compared evidence, so it is the strongest candidate for the next acceptance/harvest deepening sweep.",
                weak_count
            ),
            source_signal: CohortHealthSignalCode::WeakEvidenceBurden,
        });
    }

    Ok(CohortHealthReport {
        schema_version: 1,
        report_id: "cohort-health-reprioritization-gate".to_owned(),
        summary,
        signals,
        recommendations,
    })
}

/// Renders the cohort-health report as Markdown.
#[must_use]
pub fn render_cohort_health_markdown(report: &CohortHealthReport) -> String {
    let mut rendered = String::new();
    rendered.push_str("# Cohort Health Gate\n\n");
    rendered.push_str(
        "This page is generated from the shipped cohort validation report, the governance alignment report, and the current release-candidate readiness document. Review it before reordering future roadmap sweeps.\n\n",
    );
    rendered.push_str("## Summary\n\n");
    rendered.push_str(&format!(
        "- Shipped methods: `{}`\n- Compared-evidence methods: `{}`\n- Methods with harvested legacy provenance recorded: `{}`\n- Retained backlog still unshipped: `{}`\n- Largest retained backlog family: `{}` (`{}` remaining)\n- Weakest evidence family: `{}` (`{}` methods below compared evidence)\n- Release-truth document current: `{}`\n\n",
        report.summary.total_method_count,
        report.summary.compared_evidence_count,
        report.summary.harvested_legacy_presence_count,
        report.summary.retained_backlog_count,
        report
            .summary
            .largest_retained_backlog_family
            .as_deref()
            .unwrap_or("none"),
        report.summary.largest_retained_backlog_size,
        report
            .summary
            .weakest_evidence_family
            .as_deref()
            .unwrap_or("none"),
        report.summary.weak_evidence_method_count,
        if report.summary.release_truth_current { "yes" } else { "no" }
    ));
    rendered.push_str("## Reprioritization Signals\n\n");
    if report.signals.is_empty() {
        rendered.push_str("No reprioritization signals were generated.\n\n");
    } else {
        for signal in &report.signals {
            rendered.push_str(&format!(
                "- `{}` / `{}`: {} {}\n",
                signal_code_label(signal.code),
                signal_severity_label(signal.severity),
                signal.summary,
                signal.detail
            ));
        }
        rendered.push('\n');
    }
    rendered.push_str("## Ordered Recommendations\n\n");
    if report.recommendations.is_empty() {
        rendered.push_str("No recommendation changes are required.\n");
    } else {
        for recommendation in &report.recommendations {
            rendered.push_str(&format!(
                "{}. `{}`: {} (`{}`)\n",
                recommendation.priority,
                recommendation.target,
                recommendation.rationale,
                signal_code_label(recommendation.source_signal)
            ));
        }
    }
    rendered
}

/// Writes the cohort-health report as JSON.
pub fn write_cohort_health_report_json(
    report: &CohortHealthReport,
    path: impl AsRef<Path>,
) -> Result<(), PlatformError> {
    let path = path.as_ref();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|error| {
            PlatformError::new(
                ErrorCategory::Configuration,
                "failed to create cohort-health JSON output directory",
            )
            .with_code("testkit.cohort_health.json_dir_failed")
            .with_detail(format!("{}: {error}", parent.display()))
        })?;
    }

    let json = serde_json::to_string_pretty(report).map_err(|error| {
        PlatformError::new(
            ErrorCategory::Internal,
            "failed to serialize cohort-health report",
        )
        .with_code("testkit.cohort_health.serialize_failed")
        .with_source(error)
    })?;

    fs::write(path, json).map_err(|error| {
        PlatformError::new(
            ErrorCategory::Configuration,
            "failed to write cohort-health JSON report",
        )
        .with_code("testkit.cohort_health.json_write_failed")
        .with_detail(format!("{}: {error}", path.display()))
    })
}

/// Writes the cohort-health report as Markdown.
pub fn write_cohort_health_markdown(
    report: &CohortHealthReport,
    path: impl AsRef<Path>,
) -> Result<(), PlatformError> {
    let path = path.as_ref();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|error| {
            PlatformError::new(
                ErrorCategory::Configuration,
                "failed to create cohort-health Markdown output directory",
            )
            .with_code("testkit.cohort_health.markdown_dir_failed")
            .with_detail(format!("{}: {error}", parent.display()))
        })?;
    }

    fs::write(path, render_cohort_health_markdown(report)).map_err(|error| {
        PlatformError::new(
            ErrorCategory::Configuration,
            "failed to write cohort-health Markdown report",
        )
        .with_code("testkit.cohort_health.markdown_write_failed")
        .with_detail(format!("{}: {error}", path.display()))
    })
}

fn dominant_weak_evidence_family(
    shipped_methods: &[GovernanceAlignmentMethodRecord],
) -> Option<(String, usize, usize)> {
    let mut grouped: BTreeMap<String, (usize, usize)> = BTreeMap::new();
    for method in shipped_methods {
        let family = method
            .governance_family
            .clone()
            .unwrap_or_else(|| method.shipped_family.clone());
        let entry = grouped.entry(family).or_insert((0, 0));
        if method.evidence_level == CohortEvidenceLevel::ComparedEvidence {
            entry.1 += 1;
        } else {
            entry.0 += 1;
        }
    }

    let dominant = grouped
        .into_iter()
        .max_by(|left, right| {
            left.1
                .0
                .cmp(&right.1.0)
                .then_with(|| left.0.cmp(&right.0).reverse())
        })
        .map(|(family, (weak_count, compared_count))| (family, weak_count, compared_count));

    dominant.and_then(|entry| (entry.1 > 0).then_some(entry))
}

fn assess_readiness_alignment(
    repo_root: &Path,
    cohort: &CohortValidationReport,
    governance: &GovernanceAlignmentReport,
) -> Result<ReadinessAlignment, PlatformError> {
    let path = repo_root.join("docs/release/v1_0_0_rc_readiness.md");
    let text = fs::read_to_string(&path).map_err(|error| {
        PlatformError::new(
            ErrorCategory::Configuration,
            "failed to read RC readiness report for cohort-health alignment",
        )
        .with_code("testkit.cohort_health.readiness.read_failed")
        .with_detail(format!("{}: {error}", path.display()))
    })?;

    let expected_markers = [
        format!(
            "- Shipped methods audited: `{}`",
            cohort.summary.total_method_count
        ),
        format!(
            "- Compared-evidence methods: `{}`",
            cohort.summary.compared_evidence_count
        ),
        format!(
            "- Executable-evidence methods: `{}`",
            cohort.summary.executable_evidence_count
        ),
        format!(
            "- Methods with harvested legacy provenance recorded: `{}`",
            cohort.summary.harvested_legacy_presence_count
        ),
        format!(
            "- Retained backlog still unshipped: `{}`",
            governance.summary.retained_backlog_count
        ),
    ];

    let missing_markers = expected_markers
        .iter()
        .filter(|marker| !text.contains(marker.as_str()))
        .cloned()
        .collect::<Vec<_>>();

    Ok(ReadinessAlignment {
        release_truth_current: missing_markers.is_empty(),
        missing_markers,
    })
}

fn signal_code_label(code: CohortHealthSignalCode) -> &'static str {
    match code {
        CohortHealthSignalCode::DominantRetainedBacklog => "dominant_retained_backlog",
        CohortHealthSignalCode::WeakEvidenceBurden => "weak_evidence_burden",
        CohortHealthSignalCode::ReleaseTruthLag => "release_truth_lag",
    }
}

fn signal_severity_label(severity: CohortHealthSignalSeverity) -> &'static str {
    match severity {
        CohortHealthSignalSeverity::Notice => "notice",
        CohortHealthSignalSeverity::Warning => "warning",
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::{derive_cohort_health_report, render_cohort_health_markdown};

    fn repo_root() -> std::path::PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .canonicalize()
            .expect("repo root should resolve")
    }

    #[test]
    fn derives_report_for_current_repository_state() {
        let report =
            derive_cohort_health_report(repo_root()).expect("cohort health report should derive");
        assert!(report.summary.total_method_count > 0);
        assert_eq!(report.summary.largest_retained_backlog_size, 0);
        assert_eq!(
            report.summary.weakest_evidence_family.as_deref(),
            Some("Modernize — Rework — Protein property and structural-summary utilities")
        );
        assert_eq!(report.summary.weak_evidence_method_count, 1);
        assert!(report.summary.release_truth_current);
        assert_eq!(report.signals.len(), 1);
        assert_eq!(report.recommendations.len(), 1);
    }

    #[test]
    fn renders_stable_markdown() {
        let report =
            derive_cohort_health_report(repo_root()).expect("cohort health report should derive");
        let markdown = render_cohort_health_markdown(&report);
        assert!(markdown.contains("# Cohort Health Gate"));
        assert!(markdown.contains(
            "Weakest evidence family: `Modernize — Rework — Protein property and structural-summary utilities` (`1` methods below compared evidence)"
        ));
        assert!(markdown.contains("`weak_evidence_burden` / `notice`"));
        assert!(markdown.contains("Release-truth document current: `yes`"));
    }
}

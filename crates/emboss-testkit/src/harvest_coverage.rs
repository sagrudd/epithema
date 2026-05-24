//! Harvest-coverage exceptions reporting.

use std::fs;
use std::path::Path;

use emboss_diagnostics::{ErrorCategory, PlatformError};
use serde::{Deserialize, Serialize};

use crate::report::{
    CohortMethodGap, CohortValidationReport, derive_shipped_cohort_validation_report,
};

/// One shipped method still lacking harvested legacy provenance.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct HarvestCoverageExceptionRecord {
    /// Tool identifier from the shipped cohort.
    pub tool_name: String,
    /// Shipped Rust family label.
    pub family: String,
    /// Current evidence level label.
    pub evidence_level: String,
    /// Whether executable validation is already present.
    pub executable_validation_present: bool,
    /// Whether compared validation is already present.
    pub compared_validation_present: bool,
    /// Stable gap codes still visible for this method.
    pub unresolved_gap_codes: Vec<String>,
    /// Human-readable reason when one is directly inferable from the current cohort state.
    pub reason: Option<String>,
}

/// Aggregate harvest-coverage summary.
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct HarvestCoverageSummary {
    /// Total shipped methods represented in the cohort.
    pub total_method_count: usize,
    /// Shipped methods with harvested legacy provenance recorded.
    pub harvested_legacy_presence_count: usize,
    /// Shipped methods currently missing harvested legacy provenance.
    pub harvest_exception_count: usize,
    /// Whether harvested legacy provenance is complete across the shipped cohort.
    pub harvest_coverage_complete: bool,
}

/// Typed harvest-coverage exceptions report.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct HarvestCoverageReport {
    /// Stable schema version.
    pub schema_version: u32,
    /// Stable report identifier.
    pub report_id: String,
    /// Registry source used for shipped-cohort derivation.
    pub registry_source: String,
    /// Aggregate harvest-coverage summary.
    pub summary: HarvestCoverageSummary,
    /// Stable ordered list of harvest-coverage exceptions.
    pub exceptions: Vec<HarvestCoverageExceptionRecord>,
}

/// Derives the harvest-coverage exceptions report from the shipped cohort report.
pub fn derive_harvest_coverage_report(
    repo_root: impl AsRef<Path>,
) -> Result<HarvestCoverageReport, PlatformError> {
    let cohort = derive_shipped_cohort_validation_report(repo_root)?;
    let mut exceptions = collect_harvest_exceptions(&cohort);
    exceptions.sort_by(|left, right| {
        left.family
            .cmp(&right.family)
            .then_with(|| left.tool_name.cmp(&right.tool_name))
    });

    let summary = HarvestCoverageSummary {
        total_method_count: cohort.summary.total_method_count,
        harvested_legacy_presence_count: cohort.summary.harvested_legacy_presence_count,
        harvest_exception_count: exceptions.len(),
        harvest_coverage_complete: exceptions.is_empty()
            && cohort.summary.harvested_legacy_presence_count == cohort.summary.total_method_count,
    };

    Ok(HarvestCoverageReport {
        schema_version: 1,
        report_id: "harvest-coverage-exceptions".to_owned(),
        registry_source: cohort.registry_source,
        summary,
        exceptions,
    })
}

fn collect_harvest_exceptions(
    cohort: &CohortValidationReport,
) -> Vec<HarvestCoverageExceptionRecord> {
    cohort
        .methods
        .iter()
        .filter(|method| !method.harvested_legacy_evidence_present)
        .map(|method| HarvestCoverageExceptionRecord {
            tool_name: method.tool_name.clone(),
            family: method.family.clone(),
            evidence_level: evidence_level_label(method.evidence_level).to_owned(),
            executable_validation_present: method.executable_validation_present,
            compared_validation_present: method.compared_validation_present,
            unresolved_gap_codes: method
                .unresolved_gaps
                .iter()
                .map(gap_code_label)
                .collect(),
            reason: infer_reason(method.unresolved_gaps.as_slice()),
        })
        .collect()
}

fn infer_reason(gaps: &[CohortMethodGap]) -> Option<String> {
    if gaps.is_empty() {
        return None;
    }
    if gaps
        .iter()
        .any(|gap| matches!(gap.code, crate::report::CohortGapCode::MissingHarvestedLegacyEvidence))
    {
        return Some(
            "harvested legacy provenance is not yet recorded for the governed validation surface"
                .to_owned(),
        );
    }
    Some("method still carries unresolved validation-report gaps".to_owned())
}

fn gap_code_label(gap: &CohortMethodGap) -> String {
    format!("{:?}", gap.code).to_lowercase()
}

/// Writes the harvest-coverage report as pretty JSON.
pub fn write_harvest_coverage_report_json(
    report: &HarvestCoverageReport,
    path: impl AsRef<Path>,
) -> Result<(), PlatformError> {
    let path = path.as_ref();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|error| {
            PlatformError::new(
                ErrorCategory::Configuration,
                "failed to create harvest-coverage output directory",
            )
            .with_code("testkit.harvest_coverage.write_dir_failed")
            .with_detail(format!("{}: {error}", parent.display()))
        })?;
    }

    let json = serde_json::to_string_pretty(report).map_err(|error| {
        PlatformError::new(
            ErrorCategory::Internal,
            "failed to serialize harvest-coverage report",
        )
        .with_code("testkit.harvest_coverage.serialize_failed")
        .with_source(error)
    })?;

    fs::write(path, json).map_err(|error| {
        PlatformError::new(
            ErrorCategory::Configuration,
            "failed to write harvest-coverage report",
        )
        .with_code("testkit.harvest_coverage.write_failed")
        .with_detail(format!("{}: {error}", path.display()))
    })
}

/// Renders the harvest-coverage report as stable Markdown.
#[must_use]
pub fn render_harvest_coverage_markdown(report: &HarvestCoverageReport) -> String {
    let mut rendered = String::new();
    rendered.push_str("# Harvest Coverage Exceptions\n\n");
    rendered.push_str(
        "This page is generated from the shipped cohort validation report. It exists to surface only the shipped methods that still lack harvested legacy provenance, or to state explicitly that harvested coverage is complete across the shipped cohort.\n\n",
    );
    rendered.push_str("## Summary\n\n");
    rendered.push_str(&format!(
        "- Registry source: `{}`\n- Shipped methods: `{}`\n- Methods with harvested legacy provenance recorded: `{}`\n- Harvest exceptions: `{}`\n- Harvest coverage complete: `{}`\n\n",
        report.registry_source,
        report.summary.total_method_count,
        report.summary.harvested_legacy_presence_count,
        report.summary.harvest_exception_count,
        if report.summary.harvest_coverage_complete { "yes" } else { "no" },
    ));

    rendered.push_str("## Exceptions\n\n");
    if report.exceptions.is_empty() {
        rendered.push_str(
            "No shipped methods remain without harvested legacy provenance. Harvest coverage is currently complete across the shipped cohort.\n",
        );
    } else {
        rendered.push_str("| Tool | Family | Evidence level | Executable validation | Compared validation | Unresolved gap codes | Reason |\n");
        rendered.push_str("|---|---|---|---|---|---|---|\n");
        for method in &report.exceptions {
            rendered.push_str(&format!(
                "| `{}` | `{}` | `{}` | `{}` | `{}` | `{}` | {} |\n",
                method.tool_name,
                method.family,
                method.evidence_level,
                yes_no(method.executable_validation_present),
                yes_no(method.compared_validation_present),
                if method.unresolved_gap_codes.is_empty() {
                    "none".to_owned()
                } else {
                    method.unresolved_gap_codes.join(", ")
                },
                method.reason.as_deref().unwrap_or("none"),
            ));
        }
    }

    rendered
}

/// Writes the harvest-coverage Markdown page.
pub fn write_harvest_coverage_markdown(
    report: &HarvestCoverageReport,
    path: impl AsRef<Path>,
) -> Result<(), PlatformError> {
    let path = path.as_ref();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|error| {
            PlatformError::new(
                ErrorCategory::Configuration,
                "failed to create harvest-coverage markdown output directory",
            )
            .with_code("testkit.harvest_coverage.markdown_dir_failed")
            .with_detail(format!("{}: {error}", parent.display()))
        })?;
    }

    fs::write(path, render_harvest_coverage_markdown(report)).map_err(|error| {
        PlatformError::new(
            ErrorCategory::Configuration,
            "failed to write harvest-coverage markdown report",
        )
        .with_code("testkit.harvest_coverage.markdown_write_failed")
        .with_detail(format!("{}: {error}", path.display()))
    })
}

fn yes_no(value: bool) -> &'static str {
    if value { "yes" } else { "no" }
}

fn evidence_level_label(level: crate::report::CohortEvidenceLevel) -> &'static str {
    match level {
        crate::report::CohortEvidenceLevel::DocumentedOnly => "documented_only",
        crate::report::CohortEvidenceLevel::DeclaredEvidence => "declared_evidence",
        crate::report::CohortEvidenceLevel::HarvestedEvidence => "harvested_evidence",
        crate::report::CohortEvidenceLevel::ExecutableEvidence => "executable_evidence",
        crate::report::CohortEvidenceLevel::ComparedEvidence => "compared_evidence",
    }
}

#[cfg(test)]
mod tests {
    use super::{derive_harvest_coverage_report, render_harvest_coverage_markdown};

    fn repo_root() -> std::path::PathBuf {
        std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .canonicalize()
            .expect("repo root should resolve")
    }

    #[test]
    fn derives_report_for_current_repository_state() {
        let report = derive_harvest_coverage_report(repo_root())
            .expect("harvest-coverage report should derive");

        assert_eq!(report.summary.total_method_count, 100);
        assert_eq!(report.summary.harvested_legacy_presence_count, 100);
        assert_eq!(report.summary.harvest_exception_count, 0);
        assert!(report.summary.harvest_coverage_complete);
        assert!(report.exceptions.is_empty());
    }

    #[test]
    fn renders_stable_markdown() {
        let report = derive_harvest_coverage_report(repo_root())
            .expect("harvest-coverage report should derive");
        let markdown = render_harvest_coverage_markdown(&report);

        assert!(markdown.contains("# Harvest Coverage Exceptions"));
        assert!(markdown.contains("## Summary"));
        assert!(markdown.contains("Harvest coverage complete: `yes`"));
        assert!(markdown.contains("## Exceptions"));
    }
}

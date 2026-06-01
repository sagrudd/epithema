//! Family-level comparison coverage reporting for the shipped cohort.

use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

use emboss_diagnostics::{ErrorCategory, PlatformError};
use serde::{Deserialize, Serialize};

use crate::governance::derive_governance_alignment_report;
use crate::report::{CohortEvidenceLevel, derive_shipped_cohort_validation_report};

/// Aggregate comparison-coverage summary.
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct ComparisonCoverageSummary {
    /// Total shipped methods represented in the report.
    pub total_method_count: usize,
    /// Number of family rows in the report.
    pub family_count: usize,
    /// Shipped methods with compared evidence.
    pub compared_count: usize,
    /// Shipped methods still at executable-only evidence.
    pub executable_only_count: usize,
    /// Shipped methods with harvested legacy provenance but no compared evidence.
    pub harvested_but_not_compared_count: usize,
}

/// One family-level comparison-coverage row.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ComparisonCoverageFamilyRecord {
    /// Governance family when present, otherwise shipped Rust family.
    pub family: String,
    /// Total shipped methods counted in this family row.
    pub shipped_method_count: usize,
    /// Number of methods with compared evidence in this family.
    pub compared_count: usize,
    /// Number of methods still at executable-only evidence in this family.
    pub executable_only_count: usize,
    /// Number of methods with harvested legacy provenance but not compared evidence.
    pub harvested_but_not_compared_count: usize,
}

/// Typed comparison-coverage report for family-by-family evidence scanning.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ComparisonCoverageReport {
    /// Stable schema version.
    pub schema_version: u32,
    /// Stable report identifier.
    pub report_id: String,
    /// Registry source used to derive the shipped cohort.
    pub registry_source: String,
    /// Governance source used to derive family labels.
    pub governance_source: String,
    /// Aggregate summary counts.
    pub summary: ComparisonCoverageSummary,
    /// Family-by-family comparison coverage rows.
    pub families: Vec<ComparisonCoverageFamilyRecord>,
}

#[derive(Default)]
struct MutableCoverageFamilyRecord {
    shipped_method_count: usize,
    compared_count: usize,
    executable_only_count: usize,
    harvested_but_not_compared_count: usize,
}

/// Derives the family-level comparison coverage report.
pub fn derive_comparison_coverage_report(
    repo_root: impl AsRef<Path>,
) -> Result<ComparisonCoverageReport, PlatformError> {
    let repo_root = repo_root.as_ref();
    let cohort = derive_shipped_cohort_validation_report(repo_root)?;
    let governance = derive_governance_alignment_report(repo_root)?;
    let harvested_by_tool = cohort
        .methods
        .iter()
        .map(|method| {
            (
                method.tool_name.as_str(),
                method.harvested_legacy_evidence_present,
            )
        })
        .collect::<BTreeMap<_, _>>();

    let mut families = BTreeMap::<String, MutableCoverageFamilyRecord>::new();
    let mut summary = ComparisonCoverageSummary {
        total_method_count: governance.shipped_methods.len(),
        family_count: 0,
        ..ComparisonCoverageSummary::default()
    };

    for method in &governance.shipped_methods {
        let family = method
            .governance_family
            .clone()
            .unwrap_or_else(|| method.shipped_family.clone());
        let harvested = harvested_by_tool
            .get(method.tool_name.as_str())
            .copied()
            .unwrap_or(false);
        let record = families.entry(family).or_default();
        record.shipped_method_count += 1;

        match method.evidence_level {
            CohortEvidenceLevel::ComparedEvidence => {
                record.compared_count += 1;
                summary.compared_count += 1;
            }
            CohortEvidenceLevel::ExecutableEvidence => {
                record.executable_only_count += 1;
                summary.executable_only_count += 1;
                if harvested {
                    record.harvested_but_not_compared_count += 1;
                    summary.harvested_but_not_compared_count += 1;
                }
            }
            _ => {
                if harvested {
                    record.harvested_but_not_compared_count += 1;
                    summary.harvested_but_not_compared_count += 1;
                }
            }
        }
    }

    let mut family_rows = families
        .into_iter()
        .map(|(family, record)| ComparisonCoverageFamilyRecord {
            family,
            shipped_method_count: record.shipped_method_count,
            compared_count: record.compared_count,
            executable_only_count: record.executable_only_count,
            harvested_but_not_compared_count: record.harvested_but_not_compared_count,
        })
        .collect::<Vec<_>>();

    family_rows.sort_by(|left, right| {
        right
            .executable_only_count
            .cmp(&left.executable_only_count)
            .then_with(|| {
                right
                    .harvested_but_not_compared_count
                    .cmp(&left.harvested_but_not_compared_count)
            })
            .then_with(|| left.family.cmp(&right.family))
    });

    summary.family_count = family_rows.len();

    Ok(ComparisonCoverageReport {
        schema_version: 1,
        report_id: "family-comparison-coverage".to_owned(),
        registry_source: cohort.registry_source,
        governance_source: governance.governance_source,
        summary,
        families: family_rows,
    })
}

/// Writes the comparison coverage report as pretty JSON.
pub fn write_comparison_coverage_report_json(
    report: &ComparisonCoverageReport,
    path: impl AsRef<Path>,
) -> Result<(), PlatformError> {
    let path = path.as_ref();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|error| {
            PlatformError::new(
                ErrorCategory::Configuration,
                "failed to create comparison coverage output directory",
            )
            .with_code("testkit.comparison_coverage.write_dir_failed")
            .with_detail(format!("{}: {error}", parent.display()))
        })?;
    }

    let json = serde_json::to_string_pretty(report).map_err(|error| {
        PlatformError::new(
            ErrorCategory::Internal,
            "failed to serialize comparison coverage report",
        )
        .with_code("testkit.comparison_coverage.serialize_failed")
        .with_source(error)
    })?;

    fs::write(path, json).map_err(|error| {
        PlatformError::new(
            ErrorCategory::Configuration,
            "failed to write comparison coverage report",
        )
        .with_code("testkit.comparison_coverage.write_failed")
        .with_detail(format!("{}: {error}", path.display()))
    })
}

/// Renders the comparison coverage report as stable Markdown.
#[must_use]
pub fn render_comparison_coverage_markdown(report: &ComparisonCoverageReport) -> String {
    let mut rendered = String::new();
    rendered.push_str("# Family Comparison Coverage Report\n\n");
    rendered.push_str(
        "This page is generated from the shipped cohort validation report and the governance-alignment report. It exists to make family-by-family compared coverage easy to scan before reprioritizing the next evidence sweep.\n\n",
    );
    rendered.push_str("## Summary\n\n");
    rendered.push_str(&format!(
        "- Registry source: `{}`\n- Governance source: `{}`\n- Shipped methods: `{}`\n- Family rows: `{}`\n- Compared-evidence methods: `{}`\n- Executable-only methods: `{}`\n- Harvested-but-not-compared methods: `{}`\n\n",
        report.registry_source,
        report.governance_source,
        report.summary.total_method_count,
        report.summary.family_count,
        report.summary.compared_count,
        report.summary.executable_only_count,
        report.summary.harvested_but_not_compared_count,
    ));

    rendered.push_str("## Family Coverage Table\n\n");
    rendered.push_str(
        "| Family | Shipped methods | Compared | Executable-only | Harvested but not compared |\n",
    );
    rendered.push_str("|---|---:|---:|---:|---:|\n");
    for family in &report.families {
        rendered.push_str(&format!(
            "| {} | {} | {} | {} | {} |\n",
            family.family,
            family.shipped_method_count,
            family.compared_count,
            family.executable_only_count,
            family.harvested_but_not_compared_count,
        ));
    }

    rendered
}

/// Writes the comparison coverage Markdown page.
pub fn write_comparison_coverage_markdown(
    report: &ComparisonCoverageReport,
    path: impl AsRef<Path>,
) -> Result<(), PlatformError> {
    let path = path.as_ref();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|error| {
            PlatformError::new(
                ErrorCategory::Configuration,
                "failed to create comparison coverage markdown output directory",
            )
            .with_code("testkit.comparison_coverage.markdown_dir_failed")
            .with_detail(format!("{}: {error}", parent.display()))
        })?;
    }

    fs::write(path, render_comparison_coverage_markdown(report)).map_err(|error| {
        PlatformError::new(
            ErrorCategory::Configuration,
            "failed to write comparison coverage markdown report",
        )
        .with_code("testkit.comparison_coverage.markdown_write_failed")
        .with_detail(format!("{}: {error}", path.display()))
    })
}

#[cfg(test)]
mod tests {
    use super::{derive_comparison_coverage_report, render_comparison_coverage_markdown};

    fn repo_root() -> std::path::PathBuf {
        std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .canonicalize()
            .expect("repo root should resolve")
    }

    #[test]
    fn derives_report_for_current_repository_state() {
        let report = derive_comparison_coverage_report(repo_root())
            .expect("comparison coverage report should derive");

        assert_eq!(report.summary.total_method_count, 106);
        assert_eq!(report.summary.compared_count, 106);
        assert_eq!(report.summary.executable_only_count, 0);
        assert_eq!(report.summary.harvested_but_not_compared_count, 0);

        let family = report
            .families
            .iter()
            .find(|family| family.family == "Modernize — Rework — Plotting and visualization tools")
            .expect("plotting rework family should be present");
        assert_eq!(family.compared_count, 10);
        assert_eq!(family.executable_only_count, 0);
        assert_eq!(family.harvested_but_not_compared_count, 0);
    }

    #[test]
    fn renders_stable_markdown() {
        let report = derive_comparison_coverage_report(repo_root())
            .expect("comparison coverage report should derive");
        let markdown = render_comparison_coverage_markdown(&report);

        assert!(markdown.contains("# Family Comparison Coverage Report"));
        assert!(markdown.contains("## Family Coverage Table"));
        assert!(markdown.contains("Harvested but not compared"));
        assert!(markdown.contains("Core Retain — Sequence editing and manipulation"));
    }
}

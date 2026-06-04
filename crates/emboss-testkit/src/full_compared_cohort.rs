//! Full-compared-cohort release gate reporting.

use std::fs;
use std::path::Path;

use emboss_diagnostics::{ErrorCategory, PlatformError};
use serde::{Deserialize, Serialize};

use crate::report::{
    CohortEvidenceLevel, CohortValidationReport, derive_shipped_cohort_validation_report,
};

/// One shipped method still below compared evidence.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct BelowComparedMethodRecord {
    /// Tool identifier from the shipped cohort.
    pub tool_name: String,
    /// Shipped Rust family label.
    pub family: String,
    /// Current evidence level.
    pub evidence_level: CohortEvidenceLevel,
    /// Whether harvested legacy provenance is already present.
    pub harvested_legacy_evidence_present: bool,
    /// Whether executable validation is already present.
    pub executable_validation_present: bool,
    /// Whether compared validation is already present.
    pub compared_validation_present: bool,
}

/// Aggregate full-compared-cohort gate summary.
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct FullComparedCohortSummary {
    /// Total shipped methods represented in the cohort.
    pub total_method_count: usize,
    /// Methods currently at compared evidence.
    pub compared_evidence_count: usize,
    /// Methods still at executable evidence.
    pub executable_evidence_count: usize,
    /// Methods still below compared evidence, regardless of subtype.
    pub below_compared_method_count: usize,
    /// Whether the full shipped cohort currently reaches compared evidence.
    pub full_compared_cohort: bool,
}

/// Typed full-compared-cohort release gate report.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct FullComparedCohortReport {
    /// Stable schema version.
    pub schema_version: u32,
    /// Stable report identifier.
    pub report_id: String,
    /// Registry source used for shipped-cohort derivation.
    pub registry_source: String,
    /// Aggregate full-compared-cohort summary.
    pub summary: FullComparedCohortSummary,
    /// Stable ordered list of below-compared shipped methods.
    pub below_compared_methods: Vec<BelowComparedMethodRecord>,
}

/// Derives the full-compared-cohort release gate from the shipped cohort report.
pub fn derive_full_compared_cohort_report(
    repo_root: impl AsRef<Path>,
) -> Result<FullComparedCohortReport, PlatformError> {
    let cohort = derive_shipped_cohort_validation_report(repo_root)?;
    let mut below_compared_methods = collect_below_compared_methods(&cohort);
    below_compared_methods.sort_by(|left, right| {
        left.family
            .cmp(&right.family)
            .then_with(|| left.tool_name.cmp(&right.tool_name))
    });

    let summary = FullComparedCohortSummary {
        total_method_count: cohort.summary.total_method_count,
        compared_evidence_count: cohort.summary.compared_evidence_count,
        executable_evidence_count: cohort.summary.executable_evidence_count,
        below_compared_method_count: below_compared_methods.len(),
        full_compared_cohort: below_compared_methods.is_empty()
            && cohort.summary.compared_evidence_count == cohort.summary.total_method_count
            && cohort.summary.executable_evidence_count == 0,
    };

    Ok(FullComparedCohortReport {
        schema_version: 1,
        report_id: "full-compared-cohort-release-gate".to_owned(),
        registry_source: cohort.registry_source,
        summary,
        below_compared_methods,
    })
}

fn collect_below_compared_methods(
    cohort: &CohortValidationReport,
) -> Vec<BelowComparedMethodRecord> {
    cohort
        .methods
        .iter()
        .filter(|method| method.evidence_level != CohortEvidenceLevel::ComparedEvidence)
        .map(|method| BelowComparedMethodRecord {
            tool_name: method.tool_name.clone(),
            family: method.family.clone(),
            evidence_level: method.evidence_level,
            harvested_legacy_evidence_present: method.harvested_legacy_evidence_present,
            executable_validation_present: method.executable_validation_present,
            compared_validation_present: method.compared_validation_present,
        })
        .collect()
}

/// Writes the full-compared-cohort report as pretty JSON.
pub fn write_full_compared_cohort_report_json(
    report: &FullComparedCohortReport,
    path: impl AsRef<Path>,
) -> Result<(), PlatformError> {
    let path = path.as_ref();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|error| {
            PlatformError::new(
                ErrorCategory::Configuration,
                "failed to create full-compared-cohort output directory",
            )
            .with_code("testkit.full_compared.write_dir_failed")
            .with_detail(format!("{}: {error}", parent.display()))
        })?;
    }

    let json = serde_json::to_string_pretty(report).map_err(|error| {
        PlatformError::new(
            ErrorCategory::Internal,
            "failed to serialize full-compared-cohort report",
        )
        .with_code("testkit.full_compared.serialize_failed")
        .with_source(error)
    })?;

    fs::write(path, json).map_err(|error| {
        PlatformError::new(
            ErrorCategory::Configuration,
            "failed to write full-compared-cohort report",
        )
        .with_code("testkit.full_compared.write_failed")
        .with_detail(format!("{}: {error}", path.display()))
    })
}

/// Renders the full-compared-cohort report as stable Markdown.
#[must_use]
pub fn render_full_compared_cohort_markdown(report: &FullComparedCohortReport) -> String {
    let mut rendered = String::new();
    rendered.push_str("# Full Compared Cohort Gate\n\n");
    rendered.push_str(
        "This page is generated from the shipped cohort validation report. It exists to make the full-compared-cohort release milestone explicit as checked repository truth rather than a prose-only claim.\n\n",
    );
    rendered.push_str("## Summary\n\n");
    rendered.push_str(&format!(
        "- Registry source: `{}`\n- Shipped methods: `{}`\n- Compared-evidence methods: `{}`\n- Executable-evidence methods: `{}`\n- Methods below compared evidence: `{}`\n- Full compared cohort: `{}`\n\n",
        report.registry_source,
        report.summary.total_method_count,
        report.summary.compared_evidence_count,
        report.summary.executable_evidence_count,
        report.summary.below_compared_method_count,
        if report.summary.full_compared_cohort { "yes" } else { "no" },
    ));

    rendered.push_str("## Below-Compared Exceptions\n\n");
    if report.below_compared_methods.is_empty() {
        rendered.push_str(
            "No shipped methods remain below compared evidence. The shipped cohort currently satisfies the full-compared release gate.\n",
        );
    } else {
        rendered.push_str("| Tool | Family | Evidence level | Harvested legacy | Executable validation | Compared validation |\n");
        rendered.push_str("|---|---|---|---|---|---|\n");
        for method in &report.below_compared_methods {
            rendered.push_str(&format!(
                "| `{}` | `{}` | `{}` | `{}` | `{}` | `{}` |\n",
                method.tool_name,
                method.family,
                evidence_level_label(method.evidence_level),
                yes_no(method.harvested_legacy_evidence_present),
                yes_no(method.executable_validation_present),
                yes_no(method.compared_validation_present),
            ));
        }
    }

    rendered
}

/// Writes the full-compared-cohort Markdown page.
pub fn write_full_compared_cohort_markdown(
    report: &FullComparedCohortReport,
    path: impl AsRef<Path>,
) -> Result<(), PlatformError> {
    let path = path.as_ref();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|error| {
            PlatformError::new(
                ErrorCategory::Configuration,
                "failed to create full-compared-cohort markdown output directory",
            )
            .with_code("testkit.full_compared.markdown_dir_failed")
            .with_detail(format!("{}: {error}", parent.display()))
        })?;
    }

    fs::write(path, render_full_compared_cohort_markdown(report)).map_err(|error| {
        PlatformError::new(
            ErrorCategory::Configuration,
            "failed to write full-compared-cohort markdown report",
        )
        .with_code("testkit.full_compared.markdown_write_failed")
        .with_detail(format!("{}: {error}", path.display()))
    })
}

fn evidence_level_label(level: CohortEvidenceLevel) -> &'static str {
    match level {
        CohortEvidenceLevel::DocumentedOnly => "documented_only",
        CohortEvidenceLevel::DeclaredEvidence => "declared_evidence",
        CohortEvidenceLevel::HarvestedEvidence => "harvested_evidence",
        CohortEvidenceLevel::ExecutableEvidence => "executable_evidence",
        CohortEvidenceLevel::ComparedEvidence => "compared_evidence",
    }
}

fn yes_no(value: bool) -> &'static str {
    if value { "yes" } else { "no" }
}

#[cfg(test)]
mod tests {
    use super::{derive_full_compared_cohort_report, render_full_compared_cohort_markdown};

    fn repo_root() -> std::path::PathBuf {
        std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .canonicalize()
            .expect("repo root should resolve")
    }

    #[test]
    fn derives_report_for_current_repository_state() {
        let report = derive_full_compared_cohort_report(repo_root())
            .expect("full-compared-cohort report should derive");

        assert_eq!(report.summary.total_method_count, 110);
        assert_eq!(report.summary.compared_evidence_count, 110);
        assert_eq!(report.summary.executable_evidence_count, 0);
        assert_eq!(report.summary.below_compared_method_count, 0);
        assert!(report.summary.full_compared_cohort);
        assert!(report.below_compared_methods.is_empty());
    }

    #[test]
    fn renders_stable_markdown() {
        let report = derive_full_compared_cohort_report(repo_root())
            .expect("full-compared-cohort report should derive");
        let markdown = render_full_compared_cohort_markdown(&report);

        assert!(markdown.contains("# Full Compared Cohort Gate"));
        assert!(markdown.contains("## Summary"));
        assert!(markdown.contains("Full compared cohort: `yes`"));
    }
}

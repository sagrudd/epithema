//! Retained-backlog closure reporting.

use std::collections::BTreeMap;
use std::fs;
use std::path::Path;

use emboss_diagnostics::{ErrorCategory, PlatformError};
use serde::{Deserialize, Serialize};

use crate::governance::{
    GovernanceAlignmentReport, GovernanceDecision, derive_governance_alignment_report,
};

/// Stable blocker classification for a retained backlog item.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RetainedBacklogBlocker {
    /// The method is still absent from the shipped Rust registry.
    Implementation,
    /// The method exists but is blocked by missing validation depth.
    Validation,
    /// The method exists but is blocked by missing documentation/governance wiring.
    Documentation,
}

/// One retained backlog closure row.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct RetainedBacklogRecord {
    /// Tool identifier from the governance appendix.
    pub tool_name: String,
    /// Governance family heading.
    pub governance_family: String,
    /// Short governed description from the appendix.
    pub description: String,
    /// Nearest shipped Rust family for the remaining method.
    pub nearest_implemented_rust_family: Option<String>,
    /// Recommended next sweep to close this item.
    pub recommended_next_sweep: String,
    /// Primary blocker classification.
    pub blocker: RetainedBacklogBlocker,
}

/// Aggregate retained backlog closure summary.
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct RetainedBacklogSummary {
    /// Total retained methods governed in the appendix.
    pub retained_tool_count: usize,
    /// Retained methods already shipped.
    pub retained_shipped_count: usize,
    /// Retained methods still unshipped.
    pub retained_backlog_count: usize,
    /// Whether the retained backlog is fully closed.
    pub retained_backlog_closed: bool,
}

/// Typed retained backlog closure report.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct RetainedBacklogReport {
    /// Stable schema version.
    pub schema_version: u32,
    /// Stable report identifier.
    pub report_id: String,
    /// Governance source path used for backlog truth.
    pub governance_source: String,
    /// Aggregate retained-backlog summary.
    pub summary: RetainedBacklogSummary,
    /// Remaining retained backlog rows in stable order.
    pub remaining_methods: Vec<RetainedBacklogRecord>,
}

/// Derives the retained backlog closure report from governance alignment truth.
pub fn derive_retained_backlog_report(
    repo_root: impl AsRef<Path>,
) -> Result<RetainedBacklogReport, PlatformError> {
    let governance = derive_governance_alignment_report(repo_root)?;
    let summary = RetainedBacklogSummary {
        retained_tool_count: governance.summary.retained_tool_count,
        retained_shipped_count: governance.summary.shipped_retain_count,
        retained_backlog_count: governance.summary.retained_backlog_count,
        retained_backlog_closed: governance.summary.retained_backlog_count == 0,
    };

    let nearest_family_by_governance = nearest_family_by_governance(&governance);
    let recommendation_by_governance = governance
        .families
        .iter()
        .map(|family| {
            (
                family.governance_family.as_str(),
                family.recommendation.clone(),
            )
        })
        .collect::<BTreeMap<_, _>>();

    let mut remaining_methods = governance
        .retained_backlog_tools
        .iter()
        .filter(|entry| entry.decision == GovernanceDecision::Retain)
        .map(|entry| RetainedBacklogRecord {
            tool_name: entry.tool_name.clone(),
            governance_family: entry.governance_family.clone(),
            description: entry.description.clone(),
            nearest_implemented_rust_family: nearest_family_by_governance
                .get(entry.governance_family.as_str())
                .cloned(),
            recommended_next_sweep: recommendation_by_governance
                .get(entry.governance_family.as_str())
                .cloned()
                .unwrap_or_else(|| {
                    "keep this family in the next governed implementation sweep".to_owned()
                }),
            blocker: RetainedBacklogBlocker::Implementation,
        })
        .collect::<Vec<_>>();

    remaining_methods.sort_by(|left, right| {
        left.governance_family
            .cmp(&right.governance_family)
            .then_with(|| left.tool_name.cmp(&right.tool_name))
    });

    Ok(RetainedBacklogReport {
        schema_version: 1,
        report_id: "retained-backlog-closure".to_owned(),
        governance_source: governance.governance_source,
        summary,
        remaining_methods,
    })
}

/// Writes the retained backlog report as pretty JSON.
pub fn write_retained_backlog_report_json(
    report: &RetainedBacklogReport,
    path: impl AsRef<Path>,
) -> Result<(), PlatformError> {
    let path = path.as_ref();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|error| {
            PlatformError::new(
                ErrorCategory::Configuration,
                "failed to create retained backlog output directory",
            )
            .with_code("testkit.retained_backlog.write_dir_failed")
            .with_detail(format!("{}: {error}", parent.display()))
        })?;
    }

    let json = serde_json::to_string_pretty(report).map_err(|error| {
        PlatformError::new(
            ErrorCategory::Internal,
            "failed to serialize retained backlog report",
        )
        .with_code("testkit.retained_backlog.serialize_failed")
        .with_source(error)
    })?;

    fs::write(path, json).map_err(|error| {
        PlatformError::new(
            ErrorCategory::Configuration,
            "failed to write retained backlog report",
        )
        .with_code("testkit.retained_backlog.write_failed")
        .with_detail(format!("{}: {error}", path.display()))
    })
}

/// Renders the retained backlog closure report as stable Markdown.
#[must_use]
pub fn render_retained_backlog_markdown(report: &RetainedBacklogReport) -> String {
    let mut rendered = String::new();
    rendered.push_str("# Retained Backlog Closure Report\n\n");
    rendered.push_str(
        "This page is generated from the governance-alignment report. It exists to make the remaining retained backlog explicit when nonzero, and to document closure truth when the governed retained backlog reaches zero.\n\n",
    );
    rendered.push_str("## Summary\n\n");
    rendered.push_str(&format!(
        "- Governance source: `{}`\n- Governed retained methods: `{}`\n- Retained methods already shipped: `{}`\n- Retained backlog still unshipped: `{}`\n- Retained backlog closed: `{}`\n\n",
        report.governance_source,
        report.summary.retained_tool_count,
        report.summary.retained_shipped_count,
        report.summary.retained_backlog_count,
        if report.summary.retained_backlog_closed { "yes" } else { "no" },
    ));

    rendered.push_str("## Remaining Retained Backlog\n\n");
    if report.remaining_methods.is_empty() {
        rendered.push_str(
            "No retained governance backlog remains outside the shipped registry.\n",
        );
    } else {
        rendered.push_str("| Tool | Governance family | Nearest implemented Rust family | Recommended next sweep | Blocker |\n");
        rendered.push_str("|---|---|---|---|---|\n");
        for method in &report.remaining_methods {
            rendered.push_str(&format!(
                "| `{}` | {} | {} | {} | `{}` |\n",
                method.tool_name,
                method.governance_family,
                method
                    .nearest_implemented_rust_family
                    .as_deref()
                    .unwrap_or("none"),
                method.recommended_next_sweep,
                blocker_label(method.blocker),
            ));
        }
    }

    rendered
}

/// Writes the retained backlog Markdown page.
pub fn write_retained_backlog_markdown(
    report: &RetainedBacklogReport,
    path: impl AsRef<Path>,
) -> Result<(), PlatformError> {
    let path = path.as_ref();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|error| {
            PlatformError::new(
                ErrorCategory::Configuration,
                "failed to create retained backlog markdown output directory",
            )
            .with_code("testkit.retained_backlog.markdown_dir_failed")
            .with_detail(format!("{}: {error}", parent.display()))
        })?;
    }

    fs::write(path, render_retained_backlog_markdown(report)).map_err(|error| {
        PlatformError::new(
            ErrorCategory::Configuration,
            "failed to write retained backlog markdown report",
        )
        .with_code("testkit.retained_backlog.markdown_write_failed")
        .with_detail(format!("{}: {error}", path.display()))
    })
}

fn nearest_family_by_governance(
    governance: &GovernanceAlignmentReport,
) -> BTreeMap<&str, String> {
    let mut grouped = BTreeMap::<&str, BTreeMap<&str, usize>>::new();
    for method in &governance.shipped_methods {
        let Some(governance_family) = method.governance_family.as_deref() else {
            continue;
        };
        let entry = grouped.entry(governance_family).or_default();
        *entry.entry(method.shipped_family.as_str()).or_default() += 1;
    }

    grouped
        .into_iter()
        .filter_map(|(governance_family, families)| {
            families
                .into_iter()
                .max_by(|left, right| left.1.cmp(&right.1).then_with(|| left.0.cmp(right.0).reverse()))
                .map(|(family, _)| (governance_family, family.to_owned()))
        })
        .collect()
}

fn blocker_label(blocker: RetainedBacklogBlocker) -> &'static str {
    match blocker {
        RetainedBacklogBlocker::Implementation => "implementation",
        RetainedBacklogBlocker::Validation => "validation",
        RetainedBacklogBlocker::Documentation => "documentation",
    }
}

#[cfg(test)]
mod tests {
    use super::{derive_retained_backlog_report, render_retained_backlog_markdown};

    fn repo_root() -> std::path::PathBuf {
        std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .canonicalize()
            .expect("repo root should resolve")
    }

    #[test]
    fn derives_report_for_current_repository_state() {
        let report = derive_retained_backlog_report(repo_root())
            .expect("retained backlog report should derive");

        assert_eq!(report.summary.retained_tool_count, 90);
        assert_eq!(report.summary.retained_shipped_count, 90);
        assert_eq!(report.summary.retained_backlog_count, 0);
        assert!(report.summary.retained_backlog_closed);
        assert!(report.remaining_methods.is_empty());
    }

    #[test]
    fn renders_stable_markdown() {
        let report = derive_retained_backlog_report(repo_root())
            .expect("retained backlog report should derive");
        let markdown = render_retained_backlog_markdown(&report);

        assert!(markdown.contains("# Retained Backlog Closure Report"));
        assert!(markdown.contains("Retained backlog closed: `yes`"));
        assert!(markdown.contains("No retained governance backlog remains"));
    }
}

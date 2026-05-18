//! Governance backlog and shipped-registry reconciliation reporting.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::Path;

use emboss_diagnostics::{ErrorCategory, PlatformError};
use emboss_docgen::{AutodocSourceMode, load_document_from_path};
use serde::{Deserialize, Serialize};

use crate::report::{
    CohortEvidenceLevel, CohortMethodValidationRecord, CohortValidationReport,
    derive_shipped_cohort_validation_report,
};

/// Governance decision from the maintained family-to-tool mapping reference.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum GovernanceDecision {
    /// Historical tool is explicitly retained in the governance scope.
    Retain,
    /// Historical tool remains in scope but is explicitly marked for rework.
    Rework,
    /// Historical tool is intentionally omitted.
    Omit,
    /// New strategic capability added beyond the historical EMBOSS catalogue.
    Add,
}

/// One per-tool mapping entry parsed from the governance appendix.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct GovernanceMappingEntry {
    /// Tool name from the governance appendix.
    pub tool_name: String,
    /// Family heading under which the tool appears.
    pub governance_family: String,
    /// Governed per-tool decision.
    pub decision: GovernanceDecision,
    /// Short governance description from the appendix.
    pub description: String,
}

/// One shipped-tool reconciliation row.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct GovernanceAlignmentMethodRecord {
    /// Stable tool identifier.
    pub tool_name: String,
    /// Shipped Rust family.
    pub shipped_family: String,
    /// Shipped registry summary.
    pub shipped_summary: String,
    /// Governance family if the tool appears in the appendix.
    pub governance_family: Option<String>,
    /// Governance decision if the tool appears in the appendix.
    pub governance_decision: Option<GovernanceDecision>,
    /// Whether the current autodoc contract is curated rather than a registry stub.
    pub curated_autodoc_present: bool,
    /// Highest visible evidence level from the shipped cohort report.
    pub evidence_level: CohortEvidenceLevel,
}

/// One family-level reconciliation summary row.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct GovernanceAlignmentFamilyRecord {
    /// Governance family heading.
    pub governance_family: String,
    /// Total tools in this family marked retain.
    pub retained_total: usize,
    /// Retained tools already shipped in the Rust registry.
    pub retained_shipped: usize,
    /// Retained tools still absent from the shipped Rust registry.
    pub retained_backlog: usize,
    /// Shipped tools in this family with curated autodoc coverage.
    pub shipped_curated_autodoc: usize,
    /// Shipped tools in this family with executable or compared evidence.
    pub shipped_executable_or_better: usize,
    /// Shipped tools in this family with compared evidence.
    pub shipped_compared: usize,
    /// Stable next-sweep recommendation for this family.
    pub recommendation: String,
    /// Retained backlog tool names in stable order.
    pub retained_backlog_tools: Vec<String>,
}

/// Aggregate governance alignment counts.
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct GovernanceAlignmentSummary {
    /// Total historical/governance mapping entries parsed from the appendix.
    pub governed_tool_count: usize,
    /// Total tools explicitly marked retain in the appendix.
    pub retained_tool_count: usize,
    /// Total currently shipped Rust tools.
    pub shipped_tool_count: usize,
    /// Shipped tools with a governance appendix mapping.
    pub shipped_with_governance_mapping_count: usize,
    /// Shipped tools marked retain in the appendix.
    pub shipped_retain_count: usize,
    /// Shipped tools marked rework in the appendix.
    pub shipped_rework_count: usize,
    /// Shipped tools marked omit in the appendix.
    pub shipped_omit_count: usize,
    /// Shipped tools marked add in the appendix.
    pub shipped_add_count: usize,
    /// Retained governance tools not yet shipped.
    pub retained_backlog_count: usize,
    /// Shipped tools with curated autodoc coverage.
    pub shipped_curated_autodoc_count: usize,
    /// Shipped tools with executable or compared evidence.
    pub shipped_executable_or_better_count: usize,
    /// Shipped tools with compared evidence.
    pub shipped_compared_count: usize,
    /// Shipped tools still at documented-only evidence level.
    pub shipped_documented_only_count: usize,
}

/// Cohort-level governance alignment report.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct GovernanceAlignmentReport {
    /// Stable schema version.
    pub schema_version: u32,
    /// Stable report identifier.
    pub report_id: String,
    /// Governance appendix used as the source of truth for the backlog.
    pub governance_source: String,
    /// Shipped cohort registry source.
    pub registry_source: String,
    /// Aggregate summary counts.
    pub summary: GovernanceAlignmentSummary,
    /// One shipped-tool reconciliation row.
    pub shipped_methods: Vec<GovernanceAlignmentMethodRecord>,
    /// One family-level reconciliation row.
    pub families: Vec<GovernanceAlignmentFamilyRecord>,
    /// Retained governance tools not yet shipped.
    pub retained_backlog_tools: Vec<GovernanceMappingEntry>,
    /// Shipped tools not mapped in the governance appendix.
    pub shipped_without_governance_mapping: Vec<String>,
}

/// Parses the maintained governance family-to-tool appendix.
pub fn parse_governance_mapping_reference(
    repo_root: impl AsRef<Path>,
) -> Result<Vec<GovernanceMappingEntry>, PlatformError> {
    let path = repo_root
        .as_ref()
        .join("docs/governance/appendices/family_to_tool_mapping_reference.md");
    let text = fs::read_to_string(&path).map_err(|error| {
        PlatformError::new(
            ErrorCategory::Configuration,
            "failed to read governance family-to-tool mapping reference",
        )
        .with_code("testkit.governance.reference.read_failed")
        .with_detail(format!("{}: {error}", path.display()))
    })?;

    let mut current_family: Option<String> = None;
    let mut entries = Vec::new();
    let mut seen = BTreeSet::new();

    for line in text.lines() {
        if let Some(rest) = line.strip_prefix("## ") {
            current_family = Some(rest.trim().to_owned());
            continue;
        }

        let Some(stripped) = line.strip_prefix("- `") else {
            continue;
        };
        let Some((tool_name, remainder)) = stripped.split_once("` — **") else {
            continue;
        };
        let Some((decision, description)) = remainder.split_once("** — ") else {
            continue;
        };
        let Some(family) = current_family.as_ref() else {
            continue;
        };

        let decision = parse_governance_decision(decision)?;
        let tool_name = tool_name.trim().to_owned();
        // The appendix may repeat historical tools under later "adjacent
        // precursor" sections. First occurrence wins because it reflects the
        // primary governed family mapping rather than a secondary cross-link.
        if !seen.insert(tool_name.clone()) {
            continue;
        }

        entries.push(GovernanceMappingEntry {
            tool_name,
            governance_family: family.clone(),
            decision,
            description: description.trim().to_owned(),
        });
    }

    if entries.is_empty() {
        return Err(PlatformError::new(
            ErrorCategory::Configuration,
            "governance family-to-tool mapping reference did not yield any tool entries",
        )
        .with_code("testkit.governance.reference.empty"));
    }

    Ok(entries)
}

/// Derives a typed governance-alignment report joining the appendix, shipped
/// registry, curated autodoc surface, and cohort evidence report.
pub fn derive_governance_alignment_report(
    repo_root: impl AsRef<Path>,
) -> Result<GovernanceAlignmentReport, PlatformError> {
    let repo_root = repo_root.as_ref();
    let cohort = derive_shipped_cohort_validation_report(repo_root)?;
    let governance_entries = parse_governance_mapping_reference(repo_root)?;
    let governance_by_tool = governance_entries
        .iter()
        .map(|entry| (entry.tool_name.as_str(), entry))
        .collect::<BTreeMap<_, _>>();

    let mut shipped_methods = Vec::new();
    let mut shipped_without_governance_mapping = Vec::new();

    for method in &cohort.methods {
        let governance_entry = governance_by_tool.get(method.tool_name.as_str()).copied();
        if governance_entry.is_none() {
            shipped_without_governance_mapping.push(method.tool_name.clone());
        }

        shipped_methods.push(GovernanceAlignmentMethodRecord {
            tool_name: method.tool_name.clone(),
            shipped_family: method.family.clone(),
            shipped_summary: method.summary.clone(),
            governance_family: governance_entry.map(|entry| entry.governance_family.clone()),
            governance_decision: governance_entry.map(|entry| entry.decision),
            curated_autodoc_present: shipped_method_is_curated(repo_root, method)?,
            evidence_level: method.evidence_level,
        });
    }

    let shipped_names = shipped_methods
        .iter()
        .map(|method| method.tool_name.as_str())
        .collect::<BTreeSet<_>>();
    let retained_backlog_tools = governance_entries
        .iter()
        .filter(|entry| entry.decision == GovernanceDecision::Retain)
        .filter(|entry| !shipped_names.contains(entry.tool_name.as_str()))
        .cloned()
        .collect::<Vec<_>>();

    let families = derive_family_alignment(&governance_entries, &shipped_methods);
    let summary = GovernanceAlignmentSummary::from_alignment(
        &governance_entries,
        &cohort,
        &shipped_methods,
        retained_backlog_tools.len(),
        shipped_without_governance_mapping.len(),
    );

    Ok(GovernanceAlignmentReport {
        schema_version: 1,
        report_id: "governance-registry-alignment".to_owned(),
        governance_source:
            "docs/governance/appendices/family_to_tool_mapping_reference.md".to_owned(),
        registry_source: cohort.registry_source,
        summary,
        shipped_methods,
        families,
        retained_backlog_tools,
        shipped_without_governance_mapping,
    })
}

/// Writes the governance alignment report as pretty JSON.
pub fn write_governance_alignment_report_json(
    report: &GovernanceAlignmentReport,
    path: impl AsRef<Path>,
) -> Result<(), PlatformError> {
    let path = path.as_ref();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|error| {
            PlatformError::new(
                ErrorCategory::Configuration,
                "failed to create governance alignment output directory",
            )
            .with_code("testkit.governance.write_dir_failed")
            .with_detail(format!("{}: {error}", parent.display()))
        })?;
    }

    let json = serde_json::to_string_pretty(report).map_err(|error| {
        PlatformError::new(
            ErrorCategory::Internal,
            "failed to serialize governance alignment report",
        )
        .with_code("testkit.governance.serialize_failed")
        .with_source(error)
    })?;

    fs::write(path, json).map_err(|error| {
        PlatformError::new(
            ErrorCategory::Configuration,
            "failed to write governance alignment report",
        )
        .with_code("testkit.governance.write_failed")
        .with_detail(format!("{}: {error}", path.display()))
    })
}

/// Renders the governance alignment report as stable Markdown for docs.
#[must_use]
pub fn render_governance_alignment_markdown(report: &GovernanceAlignmentReport) -> String {
    let mut rendered = String::new();
    rendered.push_str("# Governance and Registry Alignment Report\n\n");
    rendered.push_str(
        "This page is generated from the maintained governance family-to-tool appendix, the shipped EMBOSS-RS Rust registry, and the cohort validation report. It exists to keep backlog truth, shipped scope, curated autodoc coverage, and evidence depth aligned.\n\n",
    );

    rendered.push_str("## Summary\n\n");
    rendered.push_str(&format!(
        "- Governance source: `{}`\n- Registry source: `{}`\n- Governed mapped tools: `{}`\n- Governed retained tools: `{}`\n- Shipped tools: `{}`\n- Shipped tools with governance mapping: `{}`\n- Retained backlog still unshipped: `{}`\n- Shipped tools with curated autodoc: `{}`\n- Shipped tools with executable or compared evidence: `{}`\n- Shipped tools with compared evidence: `{}`\n- Shipped tools still documented-only: `{}`\n\n",
        report.governance_source,
        report.registry_source,
        report.summary.governed_tool_count,
        report.summary.retained_tool_count,
        report.summary.shipped_tool_count,
        report.summary.shipped_with_governance_mapping_count,
        report.summary.retained_backlog_count,
        report.summary.shipped_curated_autodoc_count,
        report.summary.shipped_executable_or_better_count,
        report.summary.shipped_compared_count,
        report.summary.shipped_documented_only_count,
    ));

    rendered.push_str("## Shipped Decision Split\n\n");
    rendered.push_str(&format!(
        "- Shipped retain methods: `{}`\n- Shipped rework methods: `{}`\n- Shipped omit methods: `{}`\n- Shipped add methods: `{}`\n\n",
        report.summary.shipped_retain_count,
        report.summary.shipped_rework_count,
        report.summary.shipped_omit_count,
        report.summary.shipped_add_count,
    ));

    rendered.push_str("## Family Reconciliation\n\n");
    rendered.push_str("| Governance family | Retained total | Retained shipped | Retained backlog | Shipped curated | Shipped executable+ | Shipped compared | Recommendation |\n");
    rendered.push_str("|---|---:|---:|---:|---:|---:|---:|---|\n");
    for family in &report.families {
        rendered.push_str(&format!(
            "| {} | {} | {} | {} | {} | {} | {} | {} |\n",
            family.governance_family,
            family.retained_total,
            family.retained_shipped,
            family.retained_backlog,
            family.shipped_curated_autodoc,
            family.shipped_executable_or_better,
            family.shipped_compared,
            family.recommendation
        ));
    }

    rendered.push_str("\n## Recommended Next Governed Sweeps\n\n");
    let mut ranked = report.families.clone();
    ranked.sort_by(|left, right| {
        right
            .retained_backlog
            .cmp(&left.retained_backlog)
            .then_with(|| {
                let left_gap = left.shipped_curated_autodoc.saturating_sub(left.shipped_compared);
                let right_gap =
                    right.shipped_curated_autodoc.saturating_sub(right.shipped_compared);
                right_gap.cmp(&left_gap)
            })
            .then_with(|| left.governance_family.cmp(&right.governance_family))
    });
    for family in ranked.into_iter().filter(|family| family.retained_backlog > 0).take(5) {
        let backlog = if family.retained_backlog_tools.is_empty() {
            "none".to_owned()
        } else {
            family
                .retained_backlog_tools
                .iter()
                .map(|tool| format!("`{tool}`"))
                .collect::<Vec<_>>()
                .join(", ")
        };
        rendered.push_str(&format!(
            "- **{}**: {}. Retained backlog: {}\n",
            family.governance_family, family.recommendation, backlog
        ));
    }

    rendered.push_str("\n## Retained Backlog\n\n");
    if report.retained_backlog_tools.is_empty() {
        rendered.push_str("No retained governance backlog remains outside the shipped registry.\n");
    } else {
        for entry in &report.retained_backlog_tools {
            rendered.push_str(&format!(
                "- `{}` — {} — {}\n",
                entry.tool_name,
                governance_decision_label(entry.decision),
                entry.governance_family
            ));
        }
    }

    rendered.push_str("\n## Shipped Methods Without Governance Mapping\n\n");
    if report.shipped_without_governance_mapping.is_empty() {
        rendered.push_str("All shipped methods are mapped in the governance appendix.\n");
    } else {
        for tool in &report.shipped_without_governance_mapping {
            rendered.push_str(&format!("- `{tool}`\n"));
        }
    }

    rendered.push_str("\n## Shipped Registry Surface\n\n");
    rendered.push_str("| Tool | Shipped family | Governance family | Governance decision | Curated autodoc | Evidence level |\n");
    rendered.push_str("|---|---|---|---|---:|---|\n");
    for method in &report.shipped_methods {
        rendered.push_str(&format!(
            "| `{}` | `{}` | {} | {} | {} | `{}` |\n",
            method.tool_name,
            method.shipped_family,
            method
                .governance_family
                .as_deref()
                .unwrap_or("unmapped"),
            method
                .governance_decision
                .map(governance_decision_label)
                .unwrap_or("unmapped"),
            yes_no(method.curated_autodoc_present),
            evidence_level_label(method.evidence_level),
        ));
    }

    rendered
}

/// Writes the governance alignment Markdown page.
pub fn write_governance_alignment_markdown(
    report: &GovernanceAlignmentReport,
    path: impl AsRef<Path>,
) -> Result<(), PlatformError> {
    let path = path.as_ref();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|error| {
            PlatformError::new(
                ErrorCategory::Configuration,
                "failed to create governance alignment markdown output directory",
            )
            .with_code("testkit.governance.markdown_dir_failed")
            .with_detail(format!("{}: {error}", parent.display()))
        })?;
    }

    fs::write(path, render_governance_alignment_markdown(report)).map_err(|error| {
        PlatformError::new(
            ErrorCategory::Configuration,
            "failed to write governance alignment markdown report",
        )
        .with_code("testkit.governance.markdown_write_failed")
        .with_detail(format!("{}: {error}", path.display()))
    })
}

impl GovernanceAlignmentSummary {
    fn from_alignment(
        governance_entries: &[GovernanceMappingEntry],
        cohort: &CohortValidationReport,
        shipped_methods: &[GovernanceAlignmentMethodRecord],
        retained_backlog_count: usize,
        shipped_without_governance_mapping_count: usize,
    ) -> Self {
        let mut summary = Self {
            governed_tool_count: governance_entries.len(),
            retained_tool_count: governance_entries
                .iter()
                .filter(|entry| entry.decision == GovernanceDecision::Retain)
                .count(),
            shipped_tool_count: cohort.summary.total_method_count,
            retained_backlog_count,
            ..Self::default()
        };

        for method in shipped_methods {
            if method.governance_decision.is_some() {
                summary.shipped_with_governance_mapping_count += 1;
            }
            match method.governance_decision {
                Some(GovernanceDecision::Retain) => summary.shipped_retain_count += 1,
                Some(GovernanceDecision::Rework) => summary.shipped_rework_count += 1,
                Some(GovernanceDecision::Omit) => summary.shipped_omit_count += 1,
                Some(GovernanceDecision::Add) => summary.shipped_add_count += 1,
                None => {}
            }
            if method.curated_autodoc_present {
                summary.shipped_curated_autodoc_count += 1;
            }
            if matches!(
                method.evidence_level,
                CohortEvidenceLevel::ExecutableEvidence | CohortEvidenceLevel::ComparedEvidence
            ) {
                summary.shipped_executable_or_better_count += 1;
            }
            if method.evidence_level == CohortEvidenceLevel::ComparedEvidence {
                summary.shipped_compared_count += 1;
            }
            if method.evidence_level == CohortEvidenceLevel::DocumentedOnly {
                summary.shipped_documented_only_count += 1;
            }
        }

        summary.shipped_with_governance_mapping_count = summary
            .shipped_tool_count
            .saturating_sub(shipped_without_governance_mapping_count);

        summary
    }
}

fn derive_family_alignment(
    governance_entries: &[GovernanceMappingEntry],
    shipped_methods: &[GovernanceAlignmentMethodRecord],
) -> Vec<GovernanceAlignmentFamilyRecord> {
    let mut families = governance_entries
        .iter()
        .map(|entry| entry.governance_family.clone())
        .collect::<BTreeSet<_>>()
        .into_iter()
        .collect::<Vec<_>>();
    families.sort();

    families
        .into_iter()
        .map(|family| {
            let family_entries = governance_entries
                .iter()
                .filter(|entry| entry.governance_family == family)
                .collect::<Vec<_>>();
            let retained_entries = family_entries
                .iter()
                .filter(|entry| entry.decision == GovernanceDecision::Retain)
                .collect::<Vec<_>>();
            let shipped_family_methods = shipped_methods
                .iter()
                .filter(|method| method.governance_family.as_deref() == Some(family.as_str()))
                .collect::<Vec<_>>();
            let retained_backlog_tools = retained_entries
                .iter()
                .filter(|entry| {
                    !shipped_methods
                        .iter()
                        .any(|method| method.tool_name == entry.tool_name)
                })
                .map(|entry| entry.tool_name.clone())
                .collect::<Vec<_>>();

            let retained_shipped = retained_entries
                .iter()
                .filter(|entry| {
                    shipped_methods.iter().any(|method| {
                        method.tool_name == entry.tool_name
                            && method.governance_decision == Some(GovernanceDecision::Retain)
                    })
                })
                .count();

            let shipped_curated_autodoc = shipped_family_methods
                .iter()
                .filter(|method| method.curated_autodoc_present)
                .count();
            let shipped_executable_or_better = shipped_family_methods
                .iter()
                .filter(|method| {
                    matches!(
                        method.evidence_level,
                        CohortEvidenceLevel::ExecutableEvidence
                            | CohortEvidenceLevel::ComparedEvidence
                    )
                })
                .count();
            let shipped_compared = shipped_family_methods
                .iter()
                .filter(|method| method.evidence_level == CohortEvidenceLevel::ComparedEvidence)
                .count();

            GovernanceAlignmentFamilyRecord {
                governance_family: family.clone(),
                retained_total: retained_entries.len(),
                retained_shipped,
                retained_backlog: retained_backlog_tools.len(),
                shipped_curated_autodoc,
                shipped_executable_or_better,
                shipped_compared,
                recommendation: recommend_family_sweep(
                    retained_entries.len(),
                    retained_backlog_tools.len(),
                    shipped_curated_autodoc,
                    shipped_executable_or_better,
                    shipped_compared,
                ),
                retained_backlog_tools,
            }
        })
        .collect()
}

fn shipped_method_is_curated(
    repo_root: &Path,
    method: &CohortMethodValidationRecord,
) -> Result<bool, PlatformError> {
    let path = repo_root
        .join("docs/autodoc/tools")
        .join(format!("{}.json", method.tool_name));
    let document = load_document_from_path(&path).map_err(|error| {
        PlatformError::new(
            ErrorCategory::Configuration,
            "failed to load autodoc contract while deriving governance alignment report",
        )
        .with_code("testkit.governance.autodoc.load_failed")
        .with_source(error)
        .with_detail(path.display().to_string())
    })?;
    Ok(document.provenance.source_mode != AutodocSourceMode::RegistryStub)
}

fn parse_governance_decision(token: &str) -> Result<GovernanceDecision, PlatformError> {
    match token.trim() {
        "Retain" => Ok(GovernanceDecision::Retain),
        "Rework" => Ok(GovernanceDecision::Rework),
        "Omit" => Ok(GovernanceDecision::Omit),
        "Add" => Ok(GovernanceDecision::Add),
        other => Err(PlatformError::new(
            ErrorCategory::Configuration,
            "unknown governance decision in family-to-tool mapping reference",
        )
        .with_code("testkit.governance.reference.decision_unknown")
        .with_detail(other.to_owned())),
    }
}

fn recommend_family_sweep(
    retained_total: usize,
    retained_backlog: usize,
    shipped_curated_autodoc: usize,
    shipped_executable_or_better: usize,
    shipped_compared: usize,
) -> String {
    if retained_backlog > 0 {
        return format!("prioritise retained backlog closure ({retained_backlog} remaining)");
    }
    if retained_total > 0 && shipped_curated_autodoc < retained_total {
        return "finish curated autodoc coverage for shipped retained methods".to_owned();
    }
    if retained_total > 0 && shipped_executable_or_better < retained_total {
        return "upgrade shipped retained methods to executable evidence".to_owned();
    }
    if retained_total > 0 && shipped_compared < retained_total {
        return "upgrade shipped retained methods to compared evidence".to_owned();
    }
    "family is aligned at the current governance and evidence threshold".to_owned()
}

fn governance_decision_label(decision: GovernanceDecision) -> &'static str {
    match decision {
        GovernanceDecision::Retain => "retain",
        GovernanceDecision::Rework => "rework",
        GovernanceDecision::Omit => "omit",
        GovernanceDecision::Add => "add",
    }
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
    use std::path::Path;

    use super::{
        GovernanceDecision, derive_governance_alignment_report,
        parse_governance_mapping_reference, render_governance_alignment_markdown,
    };

    fn repo_root() -> std::path::PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .canonicalize()
            .expect("repo root should resolve")
    }

    #[test]
    fn parses_known_governance_entries() {
        let entries =
            parse_governance_mapping_reference(repo_root()).expect("governance appendix should parse");
        let getorf = entries
            .iter()
            .find(|entry| entry.tool_name == "getorf")
            .expect("getorf should be mapped");
        assert_eq!(getorf.decision, GovernanceDecision::Retain);
        assert_eq!(
            getorf.governance_family,
            "Core Retain — ORF and translation-adjacent utilities"
        );

        let charge = entries
            .iter()
            .find(|entry| entry.tool_name == "charge")
            .expect("charge should be mapped");
        assert_eq!(charge.decision, GovernanceDecision::Rework);
    }

    #[test]
    fn derives_alignment_report_for_current_registry() {
        let report =
            derive_governance_alignment_report(repo_root()).expect("alignment report should derive");

        assert_eq!(report.summary.shipped_tool_count, report.shipped_methods.len());
        assert!(report.shipped_without_governance_mapping.is_empty());
        assert!(report.summary.retained_backlog_count > 0);

        let transeq = report
            .shipped_methods
            .iter()
            .find(|method| method.tool_name == "transeq")
            .expect("transeq should be present");
        assert_eq!(transeq.governance_decision, Some(GovernanceDecision::Retain));
        assert!(transeq.curated_autodoc_present);

        let charge = report
            .shipped_methods
            .iter()
            .find(|method| method.tool_name == "charge")
            .expect("charge should be present");
        assert_eq!(charge.governance_decision, Some(GovernanceDecision::Rework));
    }

    #[test]
    fn renders_stable_alignment_markdown() {
        let report =
            derive_governance_alignment_report(repo_root()).expect("alignment report should derive");
        let markdown = render_governance_alignment_markdown(&report);

        assert!(markdown.contains("# Governance and Registry Alignment Report"));
        assert!(markdown.contains("## Family Reconciliation"));
        assert!(markdown.contains("## Retained Backlog"));
        assert!(markdown.contains("## Shipped Registry Surface"));
        assert!(markdown.contains("`transeq`"));
        assert!(markdown.contains("Core Retain — ORF and translation-adjacent utilities"));
    }
}

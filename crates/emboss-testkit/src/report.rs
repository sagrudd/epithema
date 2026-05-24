//! Aggregate validation reports.

use std::fs;
use std::path::Path;

use emboss_diagnostics::{ErrorCategory, PlatformError};
use emboss_docgen::{AutodocSourceMode, LegacyReference, load_document_from_path};
use emboss_fixtures::FixtureCatalog;
use emboss_tools::governed_tool_descriptors;
use serde::{Deserialize, Serialize};

use crate::evidence::{
    ComparisonStatus, EvidenceNote, EvidenceSourceKind, ExecutionStatus, ToolValidationCase,
};

/// Minimal validation context for future harness expansion.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ValidationContext {
    /// Fixture source used by the current validation run.
    pub fixtures: FixtureCatalog,
}

impl ValidationContext {
    /// Creates a validation context for the workspace fixture catalogue.
    #[must_use]
    pub fn new() -> Self {
        Self {
            fixtures: FixtureCatalog::workspace(),
        }
    }
}

impl Default for ValidationContext {
    fn default() -> Self {
        Self::new()
    }
}

/// Aggregate counts that summarize a tool validation record.
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ValidationEvidenceSummary {
    /// Total validation case count.
    pub total_case_count: usize,
    /// Cases declared directly from curated inputs.
    pub declared_case_count: usize,
    /// Cases backed wholly or partly by harvested legacy material.
    pub harvested_case_count: usize,
    /// Cases that appear runnable later.
    pub runnable_case_count: usize,
    /// Cases already marked as executed.
    pub executed_case_count: usize,
    /// Cases with a comparison already completed.
    pub compared_case_count: usize,
    /// Cases with passing comparison status.
    pub passed_case_count: usize,
    /// Cases with failing comparison status.
    pub failed_case_count: usize,
    /// Cases with partial comparison status.
    pub partial_case_count: usize,
    /// Cases marked unsupported.
    pub unsupported_case_count: usize,
}

/// Tool-level validation evidence report.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ToolValidationReport {
    /// Stable tool name.
    pub tool_name: String,
    /// Source autodoc document identifier.
    pub document_id: String,
    /// High-level source mode inherited from autodoc.
    #[serde(default = "default_autodoc_source_mode")]
    pub source_mode: AutodocSourceMode,
    /// Primary evidence source classification.
    #[serde(default = "default_evidence_source_kind")]
    pub evidence_source: EvidenceSourceKind,
    /// Aggregate case summary.
    pub summary: ValidationEvidenceSummary,
    /// Case-level validation records.
    pub cases: Vec<ToolValidationCase>,
    /// Unresolved gaps that future execution or comparison work must address.
    pub unresolved_gaps: Vec<String>,
    /// Report-level diagnostics.
    pub diagnostics: Vec<EvidenceNote>,
    /// Report-level provenance references.
    #[serde(default)]
    pub provenance: Vec<LegacyReference>,
}

/// High-level documentation completeness for one shipped tool.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CohortDocumentationStatus {
    /// Required documentation artefacts are complete and aligned.
    Complete,
    /// One or more required documentation artefacts are missing or invalid.
    Incomplete,
}

/// Highest evidence maturity currently visible for one shipped tool.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CohortEvidenceLevel {
    /// The tool is documented but has no declared validation cases.
    DocumentedOnly,
    /// The tool has declared validation cases only.
    DeclaredEvidence,
    /// The tool has harvested legacy-backed evidence.
    HarvestedEvidence,
    /// The tool has runnable or executed validation cases.
    ExecutableEvidence,
    /// The tool has compared evidence recorded.
    ComparedEvidence,
}

/// Typed unresolved gap code surfaced in the cohort report.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum CohortGapCode {
    MissingAutodocContract,
    InvalidAutodocContract,
    MissingGeneratedPage,
    MissingGeneratedIndexEntry,
    MissingValidationStub,
    InvalidValidationStub,
    MissingValidationCases,
    MissingHarvestedLegacyEvidence,
    MissingExecutableEvidence,
    MissingComparedEvidence,
    MissingExplicitLegacyReference,
    ValidationReportGap,
}

/// One unresolved cohort-level evidence gap for a shipped tool.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct CohortMethodGap {
    /// Stable typed gap code.
    pub code: CohortGapCode,
    /// Human-readable explanation.
    pub message: String,
}

impl CohortMethodGap {
    #[must_use]
    pub fn new(code: CohortGapCode, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
        }
    }
}

/// Documentation state recorded for one shipped tool.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct CohortDocumentationRecord {
    /// Whether the autodoc contract exists.
    pub autodoc_contract_present: bool,
    /// Whether the autodoc contract parsed and validated.
    pub autodoc_contract_valid: bool,
    /// Whether the generated Markdown page exists.
    pub generated_page_present: bool,
    /// Whether the generated docs index contains the tool.
    pub indexed_in_generated_docs: bool,
    /// Aggregate documentation state.
    pub status: CohortDocumentationStatus,
}

/// One shipped-tool entry in the cohort validation report.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct CohortMethodValidationRecord {
    /// Stable tool identifier.
    pub tool_name: String,
    /// Tool family from the governed registry.
    pub family: String,
    /// Registry summary text.
    pub summary: String,
    /// Per-tool documentation record.
    pub documentation: CohortDocumentationRecord,
    /// Whether the validation stub JSON exists.
    pub validation_stub_present: bool,
    /// Highest visible evidence maturity.
    pub evidence_level: CohortEvidenceLevel,
    /// Whether any harvested legacy evidence is present.
    pub harvested_legacy_evidence_present: bool,
    /// Whether any executable validation case is present.
    pub executable_validation_present: bool,
    /// Whether any compared evidence is present.
    pub compared_validation_present: bool,
    /// Tool-level unresolved gaps from the cohort perspective.
    pub unresolved_gaps: Vec<CohortMethodGap>,
}

/// Aggregate counts over the shipped method cohort.
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct CohortValidationSummary {
    /// Total shipped method count.
    pub total_method_count: usize,
    /// Methods with complete documentation artefacts.
    pub documentation_complete_count: usize,
    /// Methods with validation stubs present.
    pub validation_stub_count: usize,
    /// Methods at documented-only level.
    pub documented_only_count: usize,
    /// Methods at declared-evidence level.
    pub declared_evidence_count: usize,
    /// Methods with harvested evidence.
    pub harvested_evidence_count: usize,
    /// Methods with any harvested legacy provenance, regardless of highest evidence tier.
    pub harvested_legacy_presence_count: usize,
    /// Methods with executable validation.
    pub executable_evidence_count: usize,
    /// Methods with compared validation.
    pub compared_evidence_count: usize,
    /// Methods with one or more blocking unresolved cohort gaps.
    pub gapped_method_count: usize,
}

/// Cohort-level structured validation report for the shipped method surface.
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct CohortValidationReport {
    /// Stable schema version for this cohort report.
    pub schema_version: u32,
    /// Stable cohort identifier.
    pub cohort_id: String,
    /// Source of truth used to discover the cohort.
    pub registry_source: String,
    /// Aggregate cohort summary.
    pub summary: CohortValidationSummary,
    /// One entry per shipped tool.
    pub methods: Vec<CohortMethodValidationRecord>,
}

impl ToolValidationReport {
    /// Builds a report and derives its summary counts.
    #[must_use]
    pub fn new(
        tool_name: impl Into<String>,
        document_id: impl Into<String>,
        source_mode: AutodocSourceMode,
        evidence_source: EvidenceSourceKind,
        cases: Vec<ToolValidationCase>,
        unresolved_gaps: Vec<String>,
        diagnostics: Vec<EvidenceNote>,
        provenance: Vec<LegacyReference>,
    ) -> Self {
        let summary = ValidationEvidenceSummary::from_cases(&cases);
        Self {
            tool_name: tool_name.into(),
            document_id: document_id.into(),
            source_mode,
            evidence_source,
            summary,
            cases,
            unresolved_gaps,
            diagnostics,
            provenance,
        }
    }
}

fn default_autodoc_source_mode() -> AutodocSourceMode {
    AutodocSourceMode::Curated
}

fn default_evidence_source_kind() -> EvidenceSourceKind {
    EvidenceSourceKind::CuratedAutodoc
}

impl ValidationEvidenceSummary {
    /// Builds aggregate counts from case records.
    #[must_use]
    pub fn from_cases(cases: &[ToolValidationCase]) -> Self {
        let mut summary = Self {
            total_case_count: cases.len(),
            ..Self::default()
        };

        for case in cases {
            match case.declaration_status {
                crate::EvidenceDeclarationStatus::Declared => summary.declared_case_count += 1,
                crate::EvidenceDeclarationStatus::Harvested => summary.harvested_case_count += 1,
            }

            match case.execution_status {
                ExecutionStatus::Runnable => summary.runnable_case_count += 1,
                ExecutionStatus::Executed => summary.executed_case_count += 1,
                ExecutionStatus::Unsupported => summary.unsupported_case_count += 1,
                ExecutionStatus::Pending => {}
            }

            match case.comparison_status {
                ComparisonStatus::Compared => summary.compared_case_count += 1,
                ComparisonStatus::Passed => {
                    summary.compared_case_count += 1;
                    summary.passed_case_count += 1;
                }
                ComparisonStatus::Failed => {
                    summary.compared_case_count += 1;
                    summary.failed_case_count += 1;
                }
                ComparisonStatus::Partial => {
                    summary.compared_case_count += 1;
                    summary.partial_case_count += 1;
                }
                ComparisonStatus::Unsupported => summary.unsupported_case_count += 1,
                ComparisonStatus::NotRequested | ComparisonStatus::Pending => {}
            }
        }

        summary
    }
}

impl CohortValidationSummary {
    #[must_use]
    pub fn from_methods(methods: &[CohortMethodValidationRecord]) -> Self {
        let mut summary = Self {
            total_method_count: methods.len(),
            ..Self::default()
        };

        for method in methods {
            if method.documentation.status == CohortDocumentationStatus::Complete {
                summary.documentation_complete_count += 1;
            }
            if method.validation_stub_present {
                summary.validation_stub_count += 1;
            }
            if method.harvested_legacy_evidence_present {
                summary.harvested_legacy_presence_count += 1;
            }
            match method.evidence_level {
                CohortEvidenceLevel::DocumentedOnly => summary.documented_only_count += 1,
                CohortEvidenceLevel::DeclaredEvidence => summary.declared_evidence_count += 1,
                CohortEvidenceLevel::HarvestedEvidence => summary.harvested_evidence_count += 1,
                CohortEvidenceLevel::ExecutableEvidence => summary.executable_evidence_count += 1,
                CohortEvidenceLevel::ComparedEvidence => summary.compared_evidence_count += 1,
            }
            if method
                .unresolved_gaps
                .iter()
                .any(|gap| is_blocking_cohort_gap(gap.code))
            {
                summary.gapped_method_count += 1;
            }
        }

        summary
    }
}

fn is_blocking_cohort_gap(code: CohortGapCode) -> bool {
    !matches!(
        code,
        CohortGapCode::ValidationReportGap | CohortGapCode::MissingExplicitLegacyReference
    )
}

/// Derives the shipped cohort validation report from the governed registry and
/// checked-in documentation and validation artefacts.
pub fn derive_shipped_cohort_validation_report(
    repo_root: impl AsRef<Path>,
) -> Result<CohortValidationReport, PlatformError> {
    let repo_root = repo_root.as_ref();
    let generated_index = fs::read_to_string(repo_root.join("docs/generated/index.md")).map_err(
        |error| {
            PlatformError::new(
                ErrorCategory::Configuration,
                "failed to read generated docs index for cohort validation",
            )
            .with_code("testkit.cohort.index.read_failed")
            .with_detail(format!(
                "{}: {error}",
                repo_root.join("docs/generated/index.md").display()
            ))
        },
    )?;

    let methods = governed_tool_descriptors()
        .into_iter()
        .map(|descriptor| derive_method_record(repo_root, &generated_index, *descriptor))
        .collect::<Result<Vec<_>, _>>()?;

    Ok(CohortValidationReport {
        schema_version: 1,
        cohort_id: "shipped-method-cohort".to_owned(),
        registry_source: "emboss_tools::governed_tool_descriptors".to_owned(),
        summary: CohortValidationSummary::from_methods(&methods),
        methods,
    })
}

/// Writes a cohort report as pretty-printed JSON.
pub fn write_cohort_validation_report_json(
    report: &CohortValidationReport,
    path: impl AsRef<Path>,
) -> Result<(), PlatformError> {
    let path = path.as_ref();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|error| {
            PlatformError::new(
                ErrorCategory::Configuration,
                "failed to create cohort report output directory",
            )
            .with_code("testkit.cohort.create_dir_failed")
            .with_detail(format!("{}: {error}", parent.display()))
        })?;
    }

    let json = serde_json::to_string_pretty(report).map_err(|error| {
        PlatformError::new(
            ErrorCategory::Internal,
            "failed to serialize cohort validation report",
        )
        .with_code("testkit.cohort.serialize_failed")
        .with_source(error)
    })?;

    fs::write(path, json).map_err(|error| {
        PlatformError::new(
            ErrorCategory::Configuration,
            "failed to write cohort validation report",
        )
        .with_code("testkit.cohort.write_failed")
        .with_detail(format!("{}: {error}", path.display()))
    })
}

/// Renders the shipped cohort report as stable Markdown for Sphinx ingestion.
#[must_use]
pub fn render_cohort_validation_markdown(report: &CohortValidationReport) -> String {
    let mut rendered = String::new();
    rendered.push_str("# Shipped Cohort Validation Report\n\n");
    rendered.push_str(
        "This page is generated from the governed EMBOSS-RS tool registry plus checked-in autodoc and validation artefacts. It reports evidence maturity and visible gaps across the shipped method cohort.\n\n",
    );
    rendered.push_str("## Summary\n\n");
    rendered.push_str(&format!(
        "- Registry source: `{}`\n- Methods in cohort: `{}`\n- Documentation-complete methods: `{}`\n- Methods with validation stubs: `{}`\n- Documented-only methods: `{}`\n- Methods with declared evidence only: `{}`\n- Methods at harvested-evidence maturity: `{}`\n- Methods with harvested legacy provenance recorded: `{}`\n- Methods with executable validation: `{}`\n- Methods with compared evidence: `{}`\n- Methods with blocking cohort gaps: `{}`\n\n",
        report.registry_source,
        report.summary.total_method_count,
        report.summary.documentation_complete_count,
        report.summary.validation_stub_count,
        report.summary.documented_only_count,
        report.summary.declared_evidence_count,
        report.summary.harvested_evidence_count,
        report.summary.harvested_legacy_presence_count,
        report.summary.executable_evidence_count,
        report.summary.compared_evidence_count,
        report.summary.gapped_method_count
    ));
    rendered.push_str("## Evidence Level Definitions\n\n");
    rendered.push_str("- `documented_only`: the tool has documentation artefacts but no declared validation cases yet.\n");
    rendered.push_str("- `declared_evidence`: the tool has declared validation cases, but no runnable or executed evidence yet.\n");
    rendered.push_str("- `harvested_evidence`: the tool has legacy-derived or legacy-backed declared evidence.\n");
    rendered.push_str("- `executable_evidence`: the tool has at least one runnable or executed validation case.\n");
    rendered.push_str("- `compared_evidence`: the tool has at least one completed comparison result.\n\n");
    rendered.push_str("## Cohort Table\n\n");
    rendered.push_str("| Tool | Family | Evidence level | Docs | Stub | Harvested | Executable | Compared | Gap count |\n");
    rendered.push_str("| --- | --- | --- | --- | --- | --- | --- | --- | --- |\n");
    for method in &report.methods {
        rendered.push_str(&format!(
            "| `{}` | `{}` | `{}` | `{}` | `{}` | `{}` | `{}` | `{}` | `{}` |\n",
            method.tool_name,
            method.family,
            evidence_level_label(method.evidence_level),
            yes_no(method.documentation.status == CohortDocumentationStatus::Complete),
            yes_no(method.validation_stub_present),
            yes_no(method.harvested_legacy_evidence_present),
            yes_no(method.executable_validation_present),
            yes_no(method.compared_validation_present),
            method.unresolved_gaps.len()
        ));
    }
    rendered.push_str("\n## Visible Gaps\n\n");
    rendered.push_str(
        "Visible gaps may include non-blocking notes that do not lower the tool's current evidence maturity or contribute to the blocking cohort-gap count above. In the current zero-burden state, the remaining visible plotting notes reflect missing explicit legacy-reference artefacts rather than missing compared evidence.\n\n",
    );

    let grouped = report
        .methods
        .iter()
        .filter(|method| !method.unresolved_gaps.is_empty())
        .map(|method| {
            let codes = method
                .unresolved_gaps
                .iter()
                .map(|gap| format!("`{}`", gap_code_label(gap.code)))
                .collect::<Vec<_>>()
                .join(", ");
            format!("- `{}`: {}", method.tool_name, codes)
        })
        .collect::<Vec<_>>();

    if grouped.is_empty() {
        rendered.push_str("No unresolved gaps recorded.\n");
    } else {
        rendered.push_str(&grouped.join("\n"));
        rendered.push('\n');
    }

    rendered
}

/// Writes the rendered cohort Markdown page.
pub fn write_cohort_validation_markdown(
    report: &CohortValidationReport,
    path: impl AsRef<Path>,
) -> Result<(), PlatformError> {
    let path = path.as_ref();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|error| {
            PlatformError::new(
                ErrorCategory::Configuration,
                "failed to create cohort markdown output directory",
            )
            .with_code("testkit.cohort.markdown_dir_failed")
            .with_detail(format!("{}: {error}", parent.display()))
        })?;
    }

    fs::write(path, render_cohort_validation_markdown(report)).map_err(|error| {
        PlatformError::new(
            ErrorCategory::Configuration,
            "failed to write cohort markdown report",
        )
        .with_code("testkit.cohort.markdown_write_failed")
        .with_detail(format!("{}: {error}", path.display()))
    })
}

fn derive_method_record(
    repo_root: &Path,
    generated_index: &str,
    descriptor: emboss_tools::ToolDescriptor,
) -> Result<CohortMethodValidationRecord, PlatformError> {
    let slug = descriptor.name;
    let contract_path = repo_root.join("docs/autodoc/tools").join(format!("{slug}.json"));
    let page_path = repo_root.join("docs/generated/tools").join(format!("{slug}.md"));
    let validation_path = repo_root
        .join("docs/generated/validation")
        .join(format!("{slug}.validation.json"));

    let autodoc_contract_present = contract_path.exists();
    let generated_page_present = page_path.exists();
    let indexed_in_generated_docs = generated_index.contains(&format!("tools/{slug}"));

    let mut unresolved_gaps = Vec::new();
    let autodoc_contract_valid = if autodoc_contract_present {
        match load_document_from_path(&contract_path) {
            Ok(document) => {
                if document.tool.name != descriptor.name || document.tool.family.as_deref() != Some(descriptor.family) {
                    unresolved_gaps.push(CohortMethodGap::new(
                        CohortGapCode::InvalidAutodocContract,
                        format!(
                            "autodoc contract for '{}' does not align with the governed registry descriptor",
                            slug
                        ),
                    ));
                    false
                } else {
                    true
                }
            }
            Err(error) => {
                unresolved_gaps.push(CohortMethodGap::new(
                    CohortGapCode::InvalidAutodocContract,
                    format!("autodoc contract for '{}' is invalid: {error}", slug),
                ));
                false
            }
        }
    } else {
        unresolved_gaps.push(CohortMethodGap::new(
            CohortGapCode::MissingAutodocContract,
            format!("missing autodoc contract for shipped tool '{}'", slug),
        ));
        false
    };

    if !generated_page_present {
        unresolved_gaps.push(CohortMethodGap::new(
            CohortGapCode::MissingGeneratedPage,
            format!("missing generated documentation page for shipped tool '{}'", slug),
        ));
    }
    if !indexed_in_generated_docs {
        unresolved_gaps.push(CohortMethodGap::new(
            CohortGapCode::MissingGeneratedIndexEntry,
            format!("generated docs index does not contain shipped tool '{}'", slug),
        ));
    }

    let validation_stub_present = validation_path.exists();
    let parsed_validation = if validation_stub_present {
        match fs::read_to_string(&validation_path)
            .ok()
            .and_then(|json| serde_json::from_str::<ToolValidationReport>(&json).ok())
        {
            Some(report) => Some(report),
            None => {
                unresolved_gaps.push(CohortMethodGap::new(
                    CohortGapCode::InvalidValidationStub,
                    format!("validation stub for '{}' could not be parsed", slug),
                ));
                None
            }
        }
    } else {
        unresolved_gaps.push(CohortMethodGap::new(
            CohortGapCode::MissingValidationStub,
            format!("missing validation stub for shipped tool '{}'", slug),
        ));
        None
    };

    let harvested_legacy_evidence_present = parsed_validation
        .as_ref()
        .map(|report| {
            report.summary.harvested_case_count > 0
                || !report.provenance.is_empty()
                || report
                    .cases
                    .iter()
                    .any(|case| !case.provenance.is_empty())
        })
        .unwrap_or(false);
    let executable_validation_present = parsed_validation
        .as_ref()
        .map(|report| report.summary.runnable_case_count > 0 || report.summary.executed_case_count > 0)
        .unwrap_or(false);
    let compared_validation_present = parsed_validation
        .as_ref()
        .map(|report| report.summary.compared_case_count > 0)
        .unwrap_or(false);

    if let Some(report) = &parsed_validation {
        if report.summary.total_case_count == 0 {
            unresolved_gaps.push(CohortMethodGap::new(
                CohortGapCode::MissingValidationCases,
                format!("tool '{}' has no declared validation cases", slug),
            ));
        }
        if !harvested_legacy_evidence_present {
            unresolved_gaps.push(CohortMethodGap::new(
                CohortGapCode::MissingHarvestedLegacyEvidence,
                format!("tool '{}' has no harvested legacy evidence recorded yet", slug),
            ));
        }
        if !executable_validation_present {
            unresolved_gaps.push(CohortMethodGap::new(
                CohortGapCode::MissingExecutableEvidence,
                format!("tool '{}' has no runnable or executed validation case yet", slug),
            ));
        }
        if !compared_validation_present {
            unresolved_gaps.push(CohortMethodGap::new(
                CohortGapCode::MissingComparedEvidence,
                format!("tool '{}' has no compared validation evidence yet", slug),
            ));
        }
        unresolved_gaps.extend(report.unresolved_gaps.iter().map(|gap| {
            CohortMethodGap::new(classify_validation_report_gap(report, gap), gap.clone())
        }));
    }

    let evidence_level = match parsed_validation.as_ref().map(|report| &report.summary) {
        Some(summary) if summary.compared_case_count > 0 => CohortEvidenceLevel::ComparedEvidence,
        Some(summary) if summary.executed_case_count > 0 || summary.runnable_case_count > 0 => {
            CohortEvidenceLevel::ExecutableEvidence
        }
        Some(summary) if summary.harvested_case_count > 0 => CohortEvidenceLevel::HarvestedEvidence,
        Some(summary) if summary.total_case_count > 0 => CohortEvidenceLevel::DeclaredEvidence,
        _ => CohortEvidenceLevel::DocumentedOnly,
    };

    let documentation_status = if autodoc_contract_present
        && autodoc_contract_valid
        && generated_page_present
        && indexed_in_generated_docs
    {
        CohortDocumentationStatus::Complete
    } else {
        CohortDocumentationStatus::Incomplete
    };

    Ok(CohortMethodValidationRecord {
        tool_name: slug.to_owned(),
        family: descriptor.family.to_owned(),
        summary: descriptor.summary.to_owned(),
        documentation: CohortDocumentationRecord {
            autodoc_contract_present,
            autodoc_contract_valid,
            generated_page_present,
            indexed_in_generated_docs,
            status: documentation_status,
        },
        validation_stub_present,
        evidence_level,
        harvested_legacy_evidence_present,
        executable_validation_present,
        compared_validation_present,
        unresolved_gaps,
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

fn gap_code_label(code: CohortGapCode) -> &'static str {
    match code {
        CohortGapCode::MissingAutodocContract => "missing_autodoc_contract",
        CohortGapCode::InvalidAutodocContract => "invalid_autodoc_contract",
        CohortGapCode::MissingGeneratedPage => "missing_generated_page",
        CohortGapCode::MissingGeneratedIndexEntry => "missing_generated_index_entry",
        CohortGapCode::MissingValidationStub => "missing_validation_stub",
        CohortGapCode::InvalidValidationStub => "invalid_validation_stub",
        CohortGapCode::MissingValidationCases => "missing_validation_cases",
        CohortGapCode::MissingHarvestedLegacyEvidence => "missing_harvested_legacy_evidence",
        CohortGapCode::MissingExecutableEvidence => "missing_executable_evidence",
        CohortGapCode::MissingComparedEvidence => "missing_compared_evidence",
        CohortGapCode::MissingExplicitLegacyReference => "missing_explicit_legacy_reference",
        CohortGapCode::ValidationReportGap => "validation_report_gap",
    }
}

fn classify_validation_report_gap(report: &ToolValidationReport, gap: &str) -> CohortGapCode {
    if gap.contains("needs a legacy reference before comparison can run")
        && report_has_diagnostic_code(report, "testkit.case.missing_legacy_reference")
    {
        return CohortGapCode::MissingExplicitLegacyReference;
    }

    CohortGapCode::ValidationReportGap
}

fn report_has_diagnostic_code(report: &ToolValidationReport, code: &str) -> bool {
    report
        .diagnostics
        .iter()
        .any(|note| note.code.as_deref() == Some(code))
        || report.cases.iter().any(|case| {
            case.diagnostics
                .iter()
                .any(|note| note.code.as_deref() == Some(code))
        })
}

fn yes_no(value: bool) -> &'static str {
    if value { "yes" } else { "no" }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use emboss_tools::governed_tool_descriptors;

    use crate::{
        ComparisonStatus, CohortDocumentationStatus, CohortEvidenceLevel,
        EvidenceDeclarationStatus, EvidenceSourceKind, ExecutionStatus, ToolValidationCase,
        ToolValidationReport, ValidationContext, ValidationEvidenceSummary,
        derive_shipped_cohort_validation_report, render_cohort_validation_markdown,
    };

    fn repo_root() -> std::path::PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .canonicalize()
            .expect("repo root should resolve")
    }

    #[test]
    fn binds_fixture_catalogue() {
        assert_eq!(
            ValidationContext::new().fixtures.source,
            "workspace fixture catalogue"
        );
    }

    #[test]
    fn aggregates_case_counts_by_status() {
        let cases = vec![
            ToolValidationCase {
                id: "declared".to_owned(),
                title: "Declared".to_owned(),
                evidence_source: EvidenceSourceKind::CuratedAutodoc,
                declaration_status: EvidenceDeclarationStatus::Declared,
                execution_status: ExecutionStatus::Runnable,
                comparison_status: ComparisonStatus::Pending,
                required: true,
                artifact_ids: vec!["input".to_owned()],
                expected_output_ids: vec!["output".to_owned()],
                provenance: Vec::new(),
                diagnostics: Vec::new(),
            },
            ToolValidationCase {
                id: "harvested".to_owned(),
                title: "Harvested".to_owned(),
                evidence_source: EvidenceSourceKind::LegacyHarvestedAutodoc,
                declaration_status: EvidenceDeclarationStatus::Harvested,
                execution_status: ExecutionStatus::Executed,
                comparison_status: ComparisonStatus::Passed,
                required: false,
                artifact_ids: Vec::new(),
                expected_output_ids: Vec::new(),
                provenance: Vec::new(),
                diagnostics: Vec::new(),
            },
        ];

        let summary = ValidationEvidenceSummary::from_cases(&cases);
        assert_eq!(summary.total_case_count, 2);
        assert_eq!(summary.declared_case_count, 1);
        assert_eq!(summary.harvested_case_count, 1);
        assert_eq!(summary.runnable_case_count, 1);
        assert_eq!(summary.executed_case_count, 1);
        assert_eq!(summary.passed_case_count, 1);
        assert_eq!(summary.compared_case_count, 1);
    }

    #[test]
    fn deserializes_legacy_minimal_validation_reports() {
        let report = serde_json::from_str::<ToolValidationReport>(
            r#"{
  "document_id": "aligncopy-minimal",
  "tool_name": "aligncopy",
  "summary": {
    "declared_case_count": 1,
    "harvested_case_count": 0,
    "runnable_case_count": 1,
    "executed_case_count": 0,
    "compared_case_count": 0,
    "total_case_count": 1
  },
  "cases": [
    {
      "id": "aligncopy-stockholm",
      "declaration_status": "declared",
      "source_kind": "curated",
      "execution_status": "pending",
      "comparison_status": "not_compared",
      "notes": [
        "Copy a single Stockholm alignment unchanged and emit Stockholm by default."
      ]
    }
  ],
  "unresolved_gaps": [],
  "diagnostics": []
}"#,
        )
        .expect("legacy minimal report should deserialize");

        assert_eq!(report.tool_name, "aligncopy");
        assert_eq!(report.summary.total_case_count, 1);
        assert_eq!(report.summary.passed_case_count, 0);
        assert_eq!(report.source_mode, emboss_docgen::AutodocSourceMode::Curated);
        assert_eq!(
            report.evidence_source,
            EvidenceSourceKind::CuratedAutodoc
        );
        assert!(report.provenance.is_empty());
    }

    #[test]
    fn shipped_cohort_report_covers_the_governed_registry() {
        let report = derive_shipped_cohort_validation_report(repo_root())
            .expect("cohort report should derive");
        let expected = governed_tool_descriptors()
            .into_iter()
            .map(|descriptor| descriptor.name.to_owned())
            .collect::<Vec<_>>();
        let actual = report
            .methods
            .iter()
            .map(|method| method.tool_name.clone())
            .collect::<Vec<_>>();

        assert_eq!(actual, expected);
        assert!(report.summary.total_method_count > 0);
        assert_eq!(report.summary.total_method_count, report.methods.len());
        assert!(
            report
                .methods
                .iter()
                .all(|method| method.documentation.status == CohortDocumentationStatus::Complete)
        );
        assert!(report.methods.iter().all(|method| method.validation_stub_present));
        assert!(
            report
                .methods
                .iter()
                .all(|method| method.unresolved_gaps.iter().all(
                    |gap| gap.code != crate::report::CohortGapCode::InvalidValidationStub
                ))
        );
    }

    #[test]
    fn shipped_cohort_report_renders_stable_markdown() {
        let report = derive_shipped_cohort_validation_report(repo_root())
            .expect("cohort report should derive");
        let markdown = render_cohort_validation_markdown(&report);

        assert!(markdown.contains("# Shipped Cohort Validation Report"));
        assert!(markdown.contains("## Evidence Level Definitions"));
        assert!(markdown.contains("`documented_only`"));
        assert!(markdown.contains("| Tool | Family | Evidence level |"));
        assert!(markdown.contains("`needle`"));
        assert!(markdown.contains("Visible Gaps"));
    }

    #[test]
    fn shipped_cohort_report_surfaces_visible_gaps() {
        let report = derive_shipped_cohort_validation_report(repo_root())
            .expect("cohort report should derive");
        assert_eq!(report.summary.gapped_method_count, 1);
        let gap_map = report
            .methods
            .iter()
            .map(|method| (method.tool_name.as_str(), method))
            .collect::<std::collections::BTreeMap<_, _>>();

        let charge = gap_map.get("charge").expect("charge should be present");
        assert_eq!(charge.evidence_level, CohortEvidenceLevel::ComparedEvidence);
        assert!(
            charge
                .unresolved_gaps
                .iter()
                .all(|gap| gap.code != crate::report::CohortGapCode::MissingComparedEvidence)
        );
        assert!(charge.unresolved_gaps.iter().any(|gap| {
            gap.code == crate::report::CohortGapCode::MissingExplicitLegacyReference
        }));

        let pepwindow = gap_map.get("pepwindow").expect("pepwindow should be present");
        assert_eq!(pepwindow.evidence_level, CohortEvidenceLevel::ComparedEvidence);
        assert!(pepwindow.unresolved_gaps.iter().any(|gap| {
            gap.code == crate::report::CohortGapCode::MissingExplicitLegacyReference
        }));

        let descseq = gap_map.get("descseq").expect("descseq should be present");
        assert!(descseq.executable_validation_present);
        assert_eq!(descseq.evidence_level, CohortEvidenceLevel::ComparedEvidence);

        let hmoment = gap_map.get("hmoment").expect("hmoment should be present");
        assert_eq!(hmoment.evidence_level, CohortEvidenceLevel::ComparedEvidence);
        assert!(hmoment.unresolved_gaps.iter().any(|gap| {
            gap.code == crate::report::CohortGapCode::MissingExplicitLegacyReference
        }));

        let octanol = gap_map.get("octanol").expect("octanol should be present");
        assert_eq!(octanol.evidence_level, CohortEvidenceLevel::ComparedEvidence);
        assert!(octanol.unresolved_gaps.iter().any(|gap| {
            gap.code == crate::report::CohortGapCode::MissingExplicitLegacyReference
        }));

        let pepinfo = gap_map.get("pepinfo").expect("pepinfo should be present");
        assert_eq!(pepinfo.evidence_level, CohortEvidenceLevel::ExecutableEvidence);
        assert!(pepinfo.unresolved_gaps.iter().any(|gap| {
            gap.code == crate::report::CohortGapCode::MissingComparedEvidence
        }));
    }
}

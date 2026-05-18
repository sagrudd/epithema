//! Shared service façade for front-end-neutral tool discovery and invocation.

use std::path::PathBuf;
use std::str::FromStr;

use emboss_config::PlatformConfig;
use emboss_core::{
    FeatureKind, FeatureSelector, Interval, MoleculeKind, NucleotidePattern, PLATFORM_IDENTITY,
    PatternError, ProteinPattern, RevseqMode, Strand,
};
use emboss_diagnostics::{
    ArtifactProvenance, Diagnostic, ErrorCategory, ExecutionOutcome, ExecutionReport,
    OutcomeStatus, PlatformError,
};
use emboss_providers::{
    AcquisitionRequest, ArchiveObjectClass, ProviderHttpClient, ProviderRegistry,
    RetrievedArchiveManifest, RetrievedArchiveMetadata, RetrievedSequence,
};
use emboss_tools::ToolDescriptor;
use emboss_tools::alignment_analysis::{
    ConsParams, ConsambigParams, DistmatParams, MatcherParams, cons_help, consambig_help,
    distmat_help, matcher_help, run_cons, run_consambig, run_distmat, run_matcher,
};
use emboss_tools::alignment_tools::{
    AligncopyParams, AligncopypairParams, AlignmentInput, ExtractalignParams, InfoalignParams,
    aligncopy_help, aligncopypair_help, extractalign_help, infoalign_help, run_aligncopy,
    run_aligncopypair, run_extractalign, run_infoalign,
};
use emboss_tools::archive_tools::{
    RungetParams, RuninfoParams, run_runget, run_runinfo, runget_help, runinfo_help,
};
use emboss_tools::codon_tools::{
    CaiParams, ChipsParams, CodcmpParams, CodcopyParams, cai_help, chips_help, codcmp_help,
    codcopy_help, render_profile_rows, run_cai, run_chips, run_codcmp, run_codcopy,
};
use emboss_tools::feature_tools::{
    CoderetParams, ExtractfeatParams, FeatcopyParams, FeatmergeParams, FeatreportParams,
    FeattextParams, MaskfeatParams, MaskseqParams, coderet_help, extractfeat_help,
    featcopy_help, featmerge_help, featreport_help, feattext_help, maskfeat_help, maskseq_help,
    run_coderet, run_extractfeat, run_featcopy, run_featmerge, run_featreport, run_feattext,
    run_maskfeat, run_maskseq,
};
use emboss_tools::pairwise_alignment::{
    NeedleParams, NeedleallParams, WaterParams, needle_help, needleall_help, run_needle,
    run_needleall, run_water, water_help,
};
use emboss_tools::pattern_tools::{
    FuzznucParams, FuzzproParams, FuzztranParams, fuzznuc_help, fuzzpro_help, fuzztran_help,
    run_fuzznuc, run_fuzzpro, run_fuzztran,
};
use emboss_tools::protein_plots::{ChargeParams, charge_help, run_charge};
use emboss_tools::retrieval_tools::{
    RefseqgetParams, SeqretParams, SeqretSource, refseqget_help, run_refseqget, run_seqret,
    seqret_help,
};
use emboss_tools::sequence_edit::{
    DegapseqParams, DescseqParams, RevseqParams, TrimseqParams, degapseq_help, descseq_help,
    revseq_help, run_degapseq, run_descseq, run_revseq, run_trimseq, trimseq_help,
};
use emboss_tools::sequence_stats::{
    ComplexParams, CompseqParams, GeeceeParams, PepstatsParams, complex_help, compseq_help,
    geecee_help, pepstats_help, run_complex, run_compseq, run_geecee, run_pepstats,
};
use emboss_tools::sequence_stream::{
    NewseqParams, NotseqParams, NthseqParams, SeqcountParams, SequenceInput, SkipseqParams,
    load_sequence_records, newseq_help, notseq_help, nthseq_help, run_newseq, run_notseq,
    run_nthseq, run_seqcount, run_skipseq, seqcount_help, skipseq_help,
};
use emboss_tools::sequence_transform::{
    CutseqParams, ExtractseqParams, SplitterParams, UnionParams, cutseq_help, extractseq_help,
    run_cutseq, run_extractseq, run_splitter, run_union, splitter_help, union_help,
};
use emboss_tools::translation_tools::{
    BacktranambigParams, BacktranseqParams, ChecktransParams, GetorfParams, PrettyseqParams,
    TranalignParams, TranslationFrameSelection, TranseqParams, backtranambig_help,
    backtranseq_help, checktrans_help, getorf_help, prettyseq_help, run_backtranambig,
    run_backtranseq, run_checktrans, run_getorf, run_prettyseq, run_tranalign, run_transeq,
    tranalign_help, transeq_help,
};

use crate::ServiceDocumentationAcquisition;
use crate::archive_retrieval::ServiceArchiveRetrieval;
use crate::context::ExecutionContext;
use crate::error::{ServiceError, unknown_tool};
use crate::input::{ToolInputReference, ToolInputResolution, ToolInputResolver};
use crate::registry::{ServiceRegistry, ToolCatalog};
use crate::request::InvocationRequest;
use crate::response::InvocationResponse;
use crate::result::{
    ArtifactKind, ArtifactReference, MethodResult, ResultPayload, ResultSummary, TableReport,
    TextReport,
};
use crate::sequence_retrieval::ServiceSequenceRetrieval;

/// Front-end-neutral EMBOSS-RS service façade.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct EmbossService {
    registry: ServiceRegistry,
    config: PlatformConfig,
    providers: ProviderRegistry,
}

impl EmbossService {
    /// Creates a service façade for the supplied registry.
    #[must_use]
    pub fn new(registry: ServiceRegistry) -> Self {
        Self::with_platform(
            registry,
            PlatformConfig::default(),
            ProviderRegistry::builtin_defaults(),
        )
    }

    /// Creates a service façade with explicit platform configuration and providers.
    #[must_use]
    pub fn with_platform(
        registry: ServiceRegistry,
        config: PlatformConfig,
        providers: ProviderRegistry,
    ) -> Self {
        Self {
            registry,
            config,
            providers,
        }
    }

    /// Creates an empty service façade.
    #[must_use]
    pub fn empty() -> Self {
        Self::new(ServiceRegistry::new())
    }

    /// Returns the active tool registry.
    #[must_use]
    pub fn registry(&self) -> &ServiceRegistry {
        &self.registry
    }

    /// Returns the active platform configuration.
    #[must_use]
    pub fn config(&self) -> &PlatformConfig {
        &self.config
    }

    /// Returns the active provider registry.
    #[must_use]
    pub fn providers(&self) -> &ProviderRegistry {
        &self.providers
    }

    /// Returns the formal documentation acquisition gateway for docgen paths.
    #[must_use]
    pub fn documentation_acquisition(&self) -> ServiceDocumentationAcquisition<'_> {
        ServiceDocumentationAcquisition::new(&self.config, &self.providers)
    }

    /// Returns the formal single-sequence retrieval gateway for provider-backed accessions.
    pub fn sequence_retrieval(
        &self,
    ) -> Result<ServiceSequenceRetrieval<'_, emboss_providers::ReqwestHttpClient>, ServiceError>
    {
        ServiceSequenceRetrieval::new(&self.config, &self.providers)
    }

    /// Returns the formal archive metadata and manifest retrieval gateway.
    pub fn archive_retrieval(
        &self,
    ) -> Result<ServiceArchiveRetrieval<'_, emboss_providers::ReqwestHttpClient>, ServiceError>
    {
        ServiceArchiveRetrieval::new(&self.config, &self.providers)
    }

    /// Resolves an accession-style input into a provider-backed single sequence record.
    pub fn retrieve_single_sequence(
        &self,
        raw: impl Into<String>,
    ) -> Result<RetrievedSequence, ServiceError> {
        let reference = self.classify_input(raw.into())?;
        match self.resolve_input(reference, emboss_providers::ResolutionIntent::SequenceInput)? {
            ToolInputResolution::ProviderRouted { request, .. } => {
                self.sequence_retrieval()?.retrieve_single_sequence(&request)
            }
            ToolInputResolution::LocalFile {
                provenance, ..
            } => Err(PlatformError::new(
                ErrorCategory::Validation,
                "single-sequence provider retrieval expects an accession-style input, not a local file",
            )
            .with_code("service.sequence_retrieval.local_input_not_supported")
            .with_detail(provenance.locator().to_owned())),
            ToolInputResolution::InlineSequence { .. } => Err(PlatformError::new(
                ErrorCategory::Validation,
                "single-sequence provider retrieval expects an accession-style input, not an inline literal",
            )
            .with_code("service.sequence_retrieval.inline_input_not_supported")),
            ToolInputResolution::Unresolved {
                reference,
                diagnostics,
            } => Err(PlatformError::new(
                ErrorCategory::Validation,
                format!("could not resolve provider-backed input '{}'", reference.raw()),
            )
            .with_code("service.sequence_retrieval.unresolved_input")
            .with_detail(
                diagnostics
                    .iter()
                    .map(|diagnostic| diagnostic.message().to_owned())
                    .collect::<Vec<_>>()
                    .join("; "),
            )),
        }
    }

    /// Returns a human-readable status line for front ends.
    #[must_use]
    pub fn status_line(&self) -> String {
        format!(
            "{} service ready; {} tools registered; {} providers configured",
            PLATFORM_IDENTITY.invocation_pattern(),
            self.registry.len(),
            self.providers.len()
        )
    }

    /// Returns the known tool descriptors.
    #[must_use]
    pub fn descriptors(&self) -> &[ToolDescriptor] {
        self.registry.descriptors()
    }

    /// Resolves a request to a known tool and returns the placeholder invocation
    /// response or real execution result for implemented tools.
    pub fn invoke(&self, request: InvocationRequest) -> Result<InvocationResponse, ServiceError> {
        let descriptor = self
            .registry
            .find(request.tool())
            .copied()
            .ok_or_else(|| unknown_tool(request.tool()))?;

        match descriptor.name {
            "aligncopy" => self.invoke_aligncopy(request, descriptor),
            "aligncopypair" => self.invoke_aligncopypair(request, descriptor),
            "infoalign" => self.invoke_infoalign(request, descriptor),
            "extractalign" => self.invoke_extractalign(request, descriptor),
            "runinfo" => self.invoke_runinfo(request, descriptor),
            "runget" => self.invoke_runget(request, descriptor),
            "matcher" => self.invoke_matcher(request, descriptor),
            "distmat" => self.invoke_distmat(request, descriptor),
            "cons" => self.invoke_cons(request, descriptor),
            "consambig" => self.invoke_consambig(request, descriptor),
            "needle" => self.invoke_needle(request, descriptor),
            "needleall" => self.invoke_needleall(request, descriptor),
            "water" => self.invoke_water(request, descriptor),
            "seqret" => self.invoke_seqret(request, descriptor),
            "refseqget" => self.invoke_refseqget(request, descriptor),
            "seqcount" => self.invoke_seqcount(request, descriptor),
            "nthseq" => self.invoke_nthseq(request, descriptor),
            "skipseq" => self.invoke_skipseq(request, descriptor),
            "notseq" => self.invoke_notseq(request, descriptor),
            "newseq" => self.invoke_newseq(request, descriptor),
            "degapseq" => self.invoke_degapseq(request, descriptor),
            "revseq" => self.invoke_revseq(request, descriptor),
            "trimseq" => self.invoke_trimseq(request, descriptor),
            "descseq" => self.invoke_descseq(request, descriptor),
            "maskseq" => self.invoke_maskseq(request, descriptor),
            "maskfeat" => self.invoke_maskfeat(request, descriptor),
            "extractfeat" => self.invoke_extractfeat(request, descriptor),
            "featcopy" => self.invoke_featcopy(request, descriptor),
            "coderet" => self.invoke_coderet(request, descriptor),
            "featmerge" => self.invoke_featmerge(request, descriptor),
            "featreport" => self.invoke_featreport(request, descriptor),
            "feattext" => self.invoke_feattext(request, descriptor),
            "cai" => self.invoke_cai(request, descriptor),
            "chips" => self.invoke_chips(request, descriptor),
            "codcmp" => self.invoke_codcmp(request, descriptor),
            "codcopy" => self.invoke_codcopy(request, descriptor),
            "fuzznuc" => self.invoke_fuzznuc(request, descriptor),
            "fuzzpro" => self.invoke_fuzzpro(request, descriptor),
            "fuzztran" => self.invoke_fuzztran(request, descriptor),
            "charge" => self.invoke_charge(request, descriptor),
            "complex" => self.invoke_complex(request, descriptor),
            "compseq" => self.invoke_compseq(request, descriptor),
            "geecee" => self.invoke_geecee(request, descriptor),
            "pepstats" => self.invoke_pepstats(request, descriptor),
            "backtranseq" => self.invoke_backtranseq(request, descriptor),
            "backtranambig" => self.invoke_backtranambig(request, descriptor),
            "checktrans" => self.invoke_checktrans(request, descriptor),
            "transeq" => self.invoke_transeq(request, descriptor),
            "getorf" => self.invoke_getorf(request, descriptor),
            "prettyseq" => self.invoke_prettyseq(request, descriptor),
            "tranalign" => self.invoke_tranalign(request, descriptor),
            "extractseq" => self.invoke_extractseq(request, descriptor),
            "cutseq" => self.invoke_cutseq(request, descriptor),
            "union" => self.invoke_union(request, descriptor),
            "splitter" => self.invoke_splitter(request, descriptor),
            _ => {
                let report = ExecutionReport::from_context(
                    &request.context,
                    PLATFORM_IDENTITY.binary_name,
                    env!("CARGO_PKG_VERSION"),
                    ExecutionOutcome::new(OutcomeStatus::NotImplemented).with_summary(format!(
                        "tool '{}' is governed but not implemented yet",
                        descriptor.name
                    )),
                );

                Ok(InvocationResponse::not_implemented(
                    request.context,
                    request.tool,
                    descriptor,
                    report,
                ))
            }
        }
    }

    /// Builds the default CLI-oriented context for callers that do not supply one.
    #[must_use]
    pub fn default_context(&self) -> ExecutionContext {
        ExecutionContext::cli()
    }

    /// Classifies a raw tool input token using the shared service input model.
    pub fn classify_input(
        &self,
        raw: impl Into<String>,
    ) -> Result<ToolInputReference, ServiceError> {
        ToolInputResolver::new().classify(raw)
    }

    /// Resolves a typed tool input reference for a given provider-resolution intent.
    pub fn resolve_input(
        &self,
        reference: ToolInputReference,
        intent: emboss_providers::ResolutionIntent,
    ) -> Result<ToolInputResolution, ServiceError> {
        ToolInputResolver::new().resolve(reference, intent)
    }

    fn invoke_aligncopy(
        &self,
        request: InvocationRequest,
        descriptor: ToolDescriptor,
    ) -> Result<InvocationResponse, ServiceError> {
        if help_requested(request.arguments()) {
            return Ok(self.help_response(request, descriptor, aligncopy_help()));
        }

        let [input]: [String; 1] = request
            .arguments
            .clone()
            .try_into()
            .map_err(|_| tool_usage_error("aligncopy", aligncopy_help()))?;
        let (input, input_provenance, input_diagnostics) =
            self.resolve_local_alignment_input(&input)?;
        let outcome = run_aligncopy(AligncopyParams { input })?;

        let report = self.success_report(
            &request.context,
            format!(
                "copied alignment with {} rows and {} columns",
                outcome.alignment.row_count(),
                outcome.alignment.column_count()
            ),
            input_diagnostics,
            vec![input_provenance],
        );
        let result = MethodResult::new(
            request.tool.clone(),
            ResultPayload::Alignment(outcome.alignment),
            ResultSummary::new("Alignment copied")
                .with_line(format!("Input: {}", outcome.input.path.display()))
                .with_line("Output format: Stockholm")
                .with_line("Copy policy: preserve row order and aligned content unchanged"),
            report.clone(),
        );

        Ok(InvocationResponse::completed(
            request.context,
            request.tool,
            descriptor,
            report,
            result,
        ))
    }

    fn invoke_aligncopypair(
        &self,
        request: InvocationRequest,
        descriptor: ToolDescriptor,
    ) -> Result<InvocationResponse, ServiceError> {
        if help_requested(request.arguments()) {
            return Ok(self.help_response(request, descriptor, aligncopypair_help()));
        }

        let [input]: [String; 1] = request
            .arguments
            .clone()
            .try_into()
            .map_err(|_| tool_usage_error("aligncopypair", aligncopypair_help()))?;
        let (input, input_provenance, input_diagnostics) =
            self.resolve_local_alignment_input(&input)?;
        let outcome = run_aligncopypair(AligncopypairParams { input })?;

        let report = self.success_report(
            &request.context,
            "copied pairwise alignment".to_owned(),
            input_diagnostics,
            vec![input_provenance],
        );
        let result = MethodResult::new(
            request.tool.clone(),
            ResultPayload::Alignment(outcome.alignment),
            ResultSummary::new("Pairwise alignment copied")
                .with_line(format!("Input: {}", outcome.input.path.display()))
                .with_line("Pairwise policy: exactly two rows required")
                .with_line("Output format: Stockholm"),
            report.clone(),
        );

        Ok(InvocationResponse::completed(
            request.context,
            request.tool,
            descriptor,
            report,
            result,
        ))
    }

    fn invoke_infoalign(
        &self,
        request: InvocationRequest,
        descriptor: ToolDescriptor,
    ) -> Result<InvocationResponse, ServiceError> {
        if help_requested(request.arguments()) {
            return Ok(self.help_response(request, descriptor, infoalign_help()));
        }

        let [input]: [String; 1] = request
            .arguments
            .clone()
            .try_into()
            .map_err(|_| tool_usage_error("infoalign", infoalign_help()))?;
        let (input, input_provenance, input_diagnostics) =
            self.resolve_local_alignment_input(&input)?;
        let outcome = run_infoalign(InfoalignParams { input })?;

        let rows = outcome
            .rows
            .iter()
            .map(|row| {
                vec![
                    outcome
                        .alignment_identifier
                        .clone()
                        .unwrap_or_else(|| "-".to_owned()),
                    outcome.classification.clone(),
                    outcome.row_count.to_string(),
                    outcome.column_count.to_string(),
                    row.ordinal.to_string(),
                    row.identifier.clone(),
                    row.ungapped_length.to_string(),
                    row.gap_count.to_string(),
                ]
            })
            .collect();

        let report = self.success_report(
            &request.context,
            format!("reported alignment summary for {} rows", outcome.row_count),
            input_diagnostics,
            vec![input_provenance],
        );
        let result = MethodResult::new(
            request.tool.clone(),
            ResultPayload::TableReport(TableReport::new(
                vec![
                    "alignment".to_owned(),
                    "classification".to_owned(),
                    "row_count".to_owned(),
                    "column_count".to_owned(),
                    "row_ordinal".to_owned(),
                    "row_identifier".to_owned(),
                    "ungapped_length".to_owned(),
                    "gap_count".to_owned(),
                ],
                rows,
            )),
            ResultSummary::new("Alignment summary reported")
                .with_line(format!("Input: {}", outcome.input.path.display()))
                .with_line(format!(
                    "Alignment identifier: {}",
                    outcome
                        .alignment_identifier
                        .clone()
                        .unwrap_or_else(|| "-".to_owned())
                ))
                .with_line(format!("Classification: {}", outcome.classification))
                .with_line(format!("Rows: {}", outcome.row_count))
                .with_line(format!("Columns: {}", outcome.column_count)),
            report.clone(),
        );

        Ok(InvocationResponse::completed(
            request.context,
            request.tool,
            descriptor,
            report,
            result,
        ))
    }

    fn invoke_extractalign(
        &self,
        request: InvocationRequest,
        descriptor: ToolDescriptor,
    ) -> Result<InvocationResponse, ServiceError> {
        if help_requested(request.arguments()) {
            return Ok(self.help_response(request, descriptor, extractalign_help()));
        }

        let params = parse_extractalign_params(request.arguments())?;
        let (input, input_provenance, input_diagnostics) =
            self.resolve_local_alignment_input(&params.input.path.display().to_string())?;
        let outcome = run_extractalign(ExtractalignParams {
            input,
            row_ordinals: params.row_ordinals,
            row_identifiers: params.row_identifiers,
            start: params.start,
            end: params.end,
        })?;

        let report = self.success_report(
            &request.context,
            format!(
                "extracted sub-alignment with {} rows and {} columns",
                outcome.alignment.row_count(),
                outcome.alignment.column_count()
            ),
            input_diagnostics,
            vec![input_provenance],
        );
        let result = MethodResult::new(
            request.tool.clone(),
            ResultPayload::Alignment(outcome.alignment),
            ResultSummary::new("Alignment extracted")
                .with_line(format!("Input: {}", outcome.input.path.display()))
                .with_line(format!(
                    "Selected rows: {}",
                    if outcome.row_ordinals.is_empty() && outcome.row_identifiers.is_empty() {
                        "all".to_owned()
                    } else {
                        let mut parts = Vec::new();
                        if !outcome.row_ordinals.is_empty() {
                            parts.push(format!(
                                "ordinals {}",
                                outcome
                                    .row_ordinals
                                    .iter()
                                    .map(ToString::to_string)
                                    .collect::<Vec<_>>()
                                    .join(",")
                            ));
                        }
                        if !outcome.row_identifiers.is_empty() {
                            parts
                                .push(format!("identifiers {}", outcome.row_identifiers.join(",")));
                        }
                        parts.join("; ")
                    }
                ))
                .with_line(format!(
                    "Selected columns: {}",
                    match (outcome.start, outcome.end) {
                        (Some(start), Some(end)) => format!("{start}..{end}"),
                        _ => "all".to_owned(),
                    }
                ))
                .with_line("Output format: Stockholm"),
            report.clone(),
        );

        Ok(InvocationResponse::completed(
            request.context,
            request.tool,
            descriptor,
            report,
            result,
        ))
    }

    fn invoke_needle(
        &self,
        request: InvocationRequest,
        descriptor: ToolDescriptor,
    ) -> Result<InvocationResponse, ServiceError> {
        if help_requested(request.arguments()) {
            return Ok(self.help_response(request, descriptor, needle_help()));
        }

        let params = parse_needle_params("needle", request.arguments(), needle_help())?;
        let (query, query_provenance, query_diagnostics) =
            self.resolve_local_sequence_input(&params.query.path.display().to_string())?;
        let (target, target_provenance, target_diagnostics) =
            self.resolve_local_sequence_input(&params.target.path.display().to_string())?;
        let outcome = run_needle(NeedleParams {
            query,
            target,
            gap_open: params.gap_open,
            gap_extend: params.gap_extend,
        })?;

        let mut diagnostics = query_diagnostics;
        diagnostics.extend(target_diagnostics);
        let report = self.success_report(
            &request.context,
            "computed global pairwise alignment".to_owned(),
            diagnostics,
            vec![query_provenance, target_provenance],
        );
        let summary = &outcome.result.summary;
        let result = MethodResult::new(
            request.tool.clone(),
            ResultPayload::Alignment(outcome.result.alignment),
            ResultSummary::new("Global alignment computed")
                .with_line(format!("Query: {}", outcome.query.path.display()))
                .with_line(format!("Target: {}", outcome.target.path.display()))
                .with_line(format!(
                    "Mode: {}",
                    match summary.mode {
                        emboss_core::AlignmentMode::Nucleotide => "nucleotide",
                        emboss_core::AlignmentMode::Protein => "protein",
                    }
                ))
                .with_line(format!("Score: {}", summary.score))
                .with_line(format!("Aligned length: {}", summary.aligned_length))
                .with_line(format!(
                    "Identity: {} ({}%)",
                    summary.identity_count, summary.identity_percent
                ))
                .with_line(format!(
                    "Gap penalties: open={} extend={}",
                    outcome.gap_open, outcome.gap_extend
                ))
                .with_line("Output format: Stockholm"),
            report.clone(),
        );

        Ok(InvocationResponse::completed(
            request.context,
            request.tool,
            descriptor,
            report,
            result,
        ))
    }

    fn invoke_water(
        &self,
        request: InvocationRequest,
        descriptor: ToolDescriptor,
    ) -> Result<InvocationResponse, ServiceError> {
        if help_requested(request.arguments()) {
            return Ok(self.help_response(request, descriptor, water_help()));
        }

        let params = parse_needle_params("water", request.arguments(), water_help())?;
        let (query, query_provenance, query_diagnostics) =
            self.resolve_local_sequence_input(&params.query.path.display().to_string())?;
        let (target, target_provenance, target_diagnostics) =
            self.resolve_local_sequence_input(&params.target.path.display().to_string())?;
        let outcome = run_water(WaterParams {
            query,
            target,
            gap_open: params.gap_open,
            gap_extend: params.gap_extend,
        })?;

        let mut diagnostics = query_diagnostics;
        diagnostics.extend(target_diagnostics);
        let report = self.success_report(
            &request.context,
            "computed local pairwise alignment".to_owned(),
            diagnostics,
            vec![query_provenance, target_provenance],
        );
        let summary = &outcome.result.summary;
        let result = MethodResult::new(
            request.tool.clone(),
            ResultPayload::Alignment(outcome.result.alignment),
            ResultSummary::new("Local alignment computed")
                .with_line(format!("Query: {}", outcome.query.path.display()))
                .with_line(format!("Target: {}", outcome.target.path.display()))
                .with_line(format!(
                    "Mode: {}",
                    match summary.mode {
                        emboss_core::AlignmentMode::Nucleotide => "nucleotide",
                        emboss_core::AlignmentMode::Protein => "protein",
                    }
                ))
                .with_line(format!("Score: {}", summary.score))
                .with_line(format!("Aligned length: {}", summary.aligned_length))
                .with_line(format!(
                    "Identity: {} ({}%)",
                    summary.identity_count, summary.identity_percent
                ))
                .with_line(format!(
                    "Query span: {}-{}",
                    summary.query_start + 1,
                    summary.query_end
                ))
                .with_line(format!(
                    "Target span: {}-{}",
                    summary.target_start + 1,
                    summary.target_end
                ))
                .with_line(format!(
                    "Gap penalties: open={} extend={}",
                    outcome.gap_open, outcome.gap_extend
                ))
                .with_line("Output format: Stockholm"),
            report.clone(),
        );

        Ok(InvocationResponse::completed(
            request.context,
            request.tool,
            descriptor,
            report,
            result,
        ))
    }

    fn invoke_runinfo(
        &self,
        request: InvocationRequest,
        descriptor: ToolDescriptor,
    ) -> Result<InvocationResponse, ServiceError> {
        self.invoke_runinfo_inner::<emboss_providers::ReqwestHttpClient>(request, descriptor, None)
    }

    fn invoke_runinfo_inner<C: ProviderHttpClient>(
        &self,
        request: InvocationRequest,
        descriptor: ToolDescriptor,
        client: Option<&C>,
    ) -> Result<InvocationResponse, ServiceError> {
        if help_requested(request.arguments()) {
            return Ok(self.help_response(request, descriptor, runinfo_help()));
        }

        let [input]: [String; 1] = request
            .arguments
            .clone()
            .try_into()
            .map_err(|_| tool_usage_error("runinfo", runinfo_help()))?;
        let (route_provenance, metadata, diagnostics) =
            self.resolve_archive_metadata_with_client(&input, client)?;
        let outcome = run_runinfo(RuninfoParams {
            provider: metadata.provider.as_str().to_owned(),
            accession: metadata.requested_accession.clone(),
            object_class: metadata.object_class.as_str().to_owned(),
            run_accession: metadata.run_accession.clone(),
            experiment_accession: metadata.experiment_accession.clone(),
            sample_accession: metadata.sample_accession.clone(),
            study_accession: metadata.study_accession.clone(),
            platform: metadata.platform.clone(),
            instrument_model: metadata.instrument_model.clone(),
            library_layout: metadata.library_layout.clone(),
            library_strategy: metadata.library_strategy.clone(),
            library_source: metadata.library_source.clone(),
        })?;

        let report = self.success_report(
            &request.context,
            format!(
                "retrieved archive metadata for {}:{}",
                outcome.provider, outcome.accession
            ),
            diagnostics,
            vec![route_provenance, metadata.provenance.clone()],
        );
        let result = MethodResult::new(
            request.tool.clone(),
            ResultPayload::TableReport(TableReport::new(
                vec![
                    "role".to_owned(),
                    "format".to_owned(),
                    "url".to_owned(),
                    "bytes".to_owned(),
                    "md5".to_owned(),
                ],
                archive_file_rows(&metadata.files),
            )),
            ResultSummary::new("Archive metadata normalized")
                .with_line(format!("Provider: {}", outcome.provider))
                .with_line(format!("Accession: {}", outcome.accession))
                .with_line(format!("Object class: {}", outcome.object_class))
                .with_line(format!(
                    "Run: {}",
                    outcome.run_accession.as_deref().unwrap_or("-")
                ))
                .with_line(format!(
                    "Experiment: {}",
                    outcome.experiment_accession.as_deref().unwrap_or("-")
                ))
                .with_line(format!(
                    "Sample: {}",
                    outcome.sample_accession.as_deref().unwrap_or("-")
                ))
                .with_line(format!(
                    "Study: {}",
                    outcome.study_accession.as_deref().unwrap_or("-")
                ))
                .with_line(format!(
                    "Platform: {}",
                    outcome.platform.as_deref().unwrap_or("-")
                ))
                .with_line(format!(
                    "Instrument: {}",
                    outcome.instrument_model.as_deref().unwrap_or("-")
                ))
                .with_line(format!("Files: {}", metadata.files.len()))
                .with_line(format!("Route: {}", metadata.route.endpoint)),
            report.clone(),
        );

        Ok(InvocationResponse::completed(
            request.context,
            request.tool,
            descriptor,
            report,
            result,
        ))
    }

    fn invoke_runget(
        &self,
        request: InvocationRequest,
        descriptor: ToolDescriptor,
    ) -> Result<InvocationResponse, ServiceError> {
        self.invoke_runget_inner::<emboss_providers::ReqwestHttpClient>(request, descriptor, None)
    }

    fn invoke_runget_inner<C: ProviderHttpClient>(
        &self,
        request: InvocationRequest,
        descriptor: ToolDescriptor,
        client: Option<&C>,
    ) -> Result<InvocationResponse, ServiceError> {
        if help_requested(request.arguments()) {
            return Ok(self.help_response(request, descriptor, runget_help()));
        }

        let (input, download) = parse_runget_arguments(request.arguments())
            .map_err(|_| tool_usage_error("runget", runget_help()))?;
        if download {
            return Err(PlatformError::new(
                ErrorCategory::NotImplemented,
                "runget --download is not implemented in v1; use the default manifest output",
            )
            .with_code("service.runget.download_not_supported"));
        }

        let (route_provenance, manifest, diagnostics) =
            self.resolve_run_manifest_with_client(&input, client)?;
        let outcome = run_runget(RungetParams {
            provider: manifest.provider.as_str().to_owned(),
            accession: manifest.requested_accession.clone(),
            object_class: manifest.object_class.as_str().to_owned(),
            download,
        })?;

        let report = self.success_report(
            &request.context,
            format!(
                "generated public-run manifest for {}:{}",
                outcome.provider, outcome.accession
            ),
            diagnostics,
            vec![route_provenance, manifest.provenance.clone()],
        );
        let result = MethodResult::new(
            request.tool.clone(),
            ResultPayload::TableReport(TableReport::new(
                vec![
                    "role".to_owned(),
                    "format".to_owned(),
                    "url".to_owned(),
                    "bytes".to_owned(),
                    "md5".to_owned(),
                ],
                archive_file_rows(&manifest.files),
            )),
            ResultSummary::new("Public run manifest normalized")
                .with_line(format!("Provider: {}", outcome.provider))
                .with_line(format!("Accession: {}", outcome.accession))
                .with_line(format!("Object class: {}", outcome.object_class))
                .with_line("Mode: manifest only")
                .with_line(format!("Files: {}", manifest.files.len()))
                .with_line(format!(
                    "Total bytes: {}",
                    manifest
                        .total_size_bytes()
                        .map(|value| value.to_string())
                        .unwrap_or_else(|| "-".to_owned())
                ))
                .with_line(format!("Route: {}", manifest.route.endpoint)),
            report.clone(),
        );

        Ok(InvocationResponse::completed(
            request.context,
            request.tool,
            descriptor,
            report,
            result,
        ))
    }

    fn invoke_matcher(
        &self,
        request: InvocationRequest,
        descriptor: ToolDescriptor,
    ) -> Result<InvocationResponse, ServiceError> {
        if help_requested(request.arguments()) {
            return Ok(self.help_response(request, descriptor, matcher_help()));
        }

        let params = parse_sequence_pair_params("matcher", request.arguments(), matcher_help())?;
        let (query, query_provenance, query_diagnostics) =
            self.resolve_local_sequence_input(&params.query.path.display().to_string())?;
        let (target, target_provenance, target_diagnostics) =
            self.resolve_local_sequence_input(&params.target.path.display().to_string())?;
        let outcome = run_matcher(MatcherParams { query, target })?;

        let mut diagnostics = query_diagnostics;
        diagnostics.extend(target_diagnostics);
        let report = self.success_report(
            &request.context,
            "computed direct pairwise match summary".to_owned(),
            diagnostics,
            vec![query_provenance, target_provenance],
        );
        let summary = &outcome.summary;
        let result = MethodResult::new(
            request.tool.clone(),
            ResultPayload::TableReport(TableReport::new(
                vec![
                    "query".to_owned(),
                    "target".to_owned(),
                    "mode".to_owned(),
                    "query_length".to_owned(),
                    "target_length".to_owned(),
                    "compared_length".to_owned(),
                    "identity_count".to_owned(),
                    "mismatch_count".to_owned(),
                    "identity_percent".to_owned(),
                    "length_difference".to_owned(),
                ],
                vec![vec![
                    outcome.query.path.display().to_string(),
                    outcome.target.path.display().to_string(),
                    match summary.mode {
                        emboss_core::AlignmentMode::Nucleotide => "nucleotide".to_owned(),
                        emboss_core::AlignmentMode::Protein => "protein".to_owned(),
                    },
                    summary.query_length.to_string(),
                    summary.target_length.to_string(),
                    summary.compared_length.to_string(),
                    summary.identity_count.to_string(),
                    summary.mismatch_count.to_string(),
                    summary.identity_percent.to_string(),
                    summary.length_difference.to_string(),
                ]],
            )),
            ResultSummary::new("Direct match summary computed")
                .with_line(format!("Query: {}", outcome.query.path.display()))
                .with_line(format!("Target: {}", outcome.target.path.display()))
                .with_line("Comparison mode: ungapped positional overlap")
                .with_line("Identity denominator: compared overlap length")
                .with_line(format!("Compared length: {}", summary.compared_length))
                .with_line(format!(
                    "Identity: {} ({}%)",
                    summary.identity_count, summary.identity_percent
                )),
            report.clone(),
        );

        Ok(InvocationResponse::completed(
            request.context,
            request.tool,
            descriptor,
            report,
            result,
        ))
    }

    fn invoke_distmat(
        &self,
        request: InvocationRequest,
        descriptor: ToolDescriptor,
    ) -> Result<InvocationResponse, ServiceError> {
        if help_requested(request.arguments()) {
            return Ok(self.help_response(request, descriptor, distmat_help()));
        }

        let input = request
            .arguments()
            .first()
            .cloned()
            .ok_or_else(|| tool_usage_error("distmat", distmat_help()))?;
        let (input, provenance, diagnostics) = self.resolve_local_sequence_input(&input)?;
        let outcome = run_distmat(DistmatParams { input })?;
        let report = self.success_report(
            &request.context,
            "computed pairwise distance matrix".to_owned(),
            diagnostics,
            vec![provenance],
        );

        let mut rows = Vec::with_capacity(outcome.matrix.identifiers.len());
        for (index, identifier) in outcome.matrix.identifiers.iter().enumerate() {
            let mut row = Vec::with_capacity(outcome.matrix.identifiers.len() + 1);
            row.push(identifier.clone());
            row.extend(
                outcome.matrix.values[index]
                    .iter()
                    .map(|value| format!("{value:.6}")),
            );
            rows.push(row);
        }

        let mut columns = vec!["record".to_owned()];
        columns.extend(outcome.matrix.identifiers.iter().cloned());

        let result = MethodResult::new(
            request.tool.clone(),
            ResultPayload::TableReport(TableReport::new(columns, rows)),
            ResultSummary::new("Distance matrix computed")
                .with_line(format!("Input: {}", outcome.input.path.display()))
                .with_line(format!("Mode: {}", outcome.mode_label()))
                .with_line("Distance measure: p-distance (mismatches / sequence length)")
                .with_line(format!(
                    "Equal-length requirement: {} residues",
                    outcome.matrix.sequence_length
                ))
                .with_line(format!("Records: {}", outcome.matrix.identifiers.len())),
            report.clone(),
        );

        Ok(InvocationResponse::completed(
            request.context,
            request.tool,
            descriptor,
            report,
            result,
        ))
    }

    fn invoke_cons(
        &self,
        request: InvocationRequest,
        descriptor: ToolDescriptor,
    ) -> Result<InvocationResponse, ServiceError> {
        if help_requested(request.arguments()) {
            return Ok(self.help_response(request, descriptor, cons_help()));
        }

        let input = request
            .arguments()
            .first()
            .cloned()
            .ok_or_else(|| tool_usage_error("cons", cons_help()))?;
        let (input, provenance, diagnostics) = self.resolve_local_alignment_input(&input)?;
        let outcome = run_cons(ConsParams { input })?;
        let report = self.success_report(
            &request.context,
            "derived simple consensus sequence".to_owned(),
            diagnostics,
            vec![provenance],
        );
        let result = MethodResult::new(
            request.tool.clone(),
            ResultPayload::Sequence(outcome.consensus),
            ResultSummary::new("Consensus sequence derived")
                .with_line(format!("Input: {}", outcome.input.path.display()))
                .with_line("Consensus rule: majority non-gap residue")
                .with_line("Tie policy: N for nucleotide, X for protein")
                .with_line("Output format: FASTA"),
            report.clone(),
        );

        Ok(InvocationResponse::completed(
            request.context,
            request.tool,
            descriptor,
            report,
            result,
        ))
    }

    fn invoke_consambig(
        &self,
        request: InvocationRequest,
        descriptor: ToolDescriptor,
    ) -> Result<InvocationResponse, ServiceError> {
        if help_requested(request.arguments()) {
            return Ok(self.help_response(request, descriptor, consambig_help()));
        }

        let input = request
            .arguments()
            .first()
            .cloned()
            .ok_or_else(|| tool_usage_error("consambig", consambig_help()))?;
        let (input, provenance, diagnostics) = self.resolve_local_alignment_input(&input)?;
        let outcome = run_consambig(ConsambigParams { input })?;
        let report = self.success_report(
            &request.context,
            "derived ambiguity-aware consensus sequence".to_owned(),
            diagnostics,
            vec![provenance],
        );
        let result = MethodResult::new(
            request.tool.clone(),
            ResultPayload::Sequence(outcome.consensus),
            ResultSummary::new("Ambiguity-aware consensus derived")
                .with_line(format!("Input: {}", outcome.input.path.display()))
                .with_line("Consensus rule: gap-ignoring nucleotide/protein ambiguity handling")
                .with_line("Nucleotide ambiguity: IUPAC exact-base sets")
                .with_line("Protein ambiguity: X for non-singleton columns")
                .with_line("Output format: FASTA"),
            report.clone(),
        );

        Ok(InvocationResponse::completed(
            request.context,
            request.tool,
            descriptor,
            report,
            result,
        ))
    }

    fn invoke_needleall(
        &self,
        request: InvocationRequest,
        descriptor: ToolDescriptor,
    ) -> Result<InvocationResponse, ServiceError> {
        if help_requested(request.arguments()) {
            return Ok(self.help_response(request, descriptor, needleall_help()));
        }

        let params = parse_needle_params("needleall", request.arguments(), needleall_help())?;
        let (query, query_provenance, query_diagnostics) =
            self.resolve_local_sequence_input(&params.query.path.display().to_string())?;
        let (target, target_provenance, target_diagnostics) =
            self.resolve_local_sequence_input(&params.target.path.display().to_string())?;
        let outcome = run_needleall(NeedleallParams {
            query,
            target,
            gap_open: params.gap_open,
            gap_extend: params.gap_extend,
        })?;

        let mut diagnostics = query_diagnostics;
        diagnostics.extend(target_diagnostics);
        let report = self.success_report(
            &request.context,
            format!(
                "computed {} global pairwise alignments",
                outcome.cases.len()
            ),
            diagnostics,
            vec![query_provenance, target_provenance],
        );
        let rows = outcome
            .cases
            .iter()
            .map(|case| {
                vec![
                    case.query_id.clone(),
                    case.target_id.clone(),
                    match case.mode {
                        emboss_core::AlignmentMode::Nucleotide => "nucleotide".to_owned(),
                        emboss_core::AlignmentMode::Protein => "protein".to_owned(),
                    },
                    case.score.to_string(),
                    case.aligned_length.to_string(),
                    case.identity_count.to_string(),
                    case.identity_percent.to_string(),
                    case.query_gap_count.to_string(),
                    case.target_gap_count.to_string(),
                ]
            })
            .collect();
        let result = MethodResult::new(
            request.tool.clone(),
            ResultPayload::TableReport(TableReport::new(
                vec![
                    "query".to_owned(),
                    "target".to_owned(),
                    "mode".to_owned(),
                    "score".to_owned(),
                    "aligned_length".to_owned(),
                    "identity_count".to_owned(),
                    "identity_percent".to_owned(),
                    "query_gap_count".to_owned(),
                    "target_gap_count".to_owned(),
                ],
                rows,
            )),
            ResultSummary::new("Global alignment comparisons reported")
                .with_line(format!("Query input: {}", outcome.query.path.display()))
                .with_line(format!("Target input: {}", outcome.target.path.display()))
                .with_line("Comparison order: query-major then target-major")
                .with_line(format!("Comparisons: {}", outcome.cases.len()))
                .with_line(format!(
                    "Gap overrides: open={} extend={}",
                    outcome
                        .gap_open
                        .map(|value| value.to_string())
                        .unwrap_or_else(|| "default".to_owned()),
                    outcome
                        .gap_extend
                        .map(|value| value.to_string())
                        .unwrap_or_else(|| "default".to_owned())
                ))
                .with_line("Alignment outputs: summary table only in v1"),
            report.clone(),
        );

        Ok(InvocationResponse::completed(
            request.context,
            request.tool,
            descriptor,
            report,
            result,
        ))
    }

    fn invoke_seqret(
        &self,
        request: InvocationRequest,
        descriptor: ToolDescriptor,
    ) -> Result<InvocationResponse, ServiceError> {
        self.invoke_seqret_inner::<emboss_providers::ReqwestHttpClient>(request, descriptor, None)
    }

    fn invoke_seqret_inner<C: ProviderHttpClient>(
        &self,
        request: InvocationRequest,
        descriptor: ToolDescriptor,
        client: Option<&C>,
    ) -> Result<InvocationResponse, ServiceError> {
        if help_requested(request.arguments()) {
            return Ok(self.help_response(request, descriptor, seqret_help()));
        }

        let [input]: [String; 1] = request
            .arguments
            .clone()
            .try_into()
            .map_err(|_| tool_usage_error("seqret", seqret_help()))?;
        let (source, records, provenance, diagnostics) =
            self.resolve_seqret_records_with_client(&input, client)?;
        let outcome = run_seqret(SeqretParams { source, records })?;

        let output_provenance = ArtifactProvenance::generated_output("stdout")
            .with_description("normalized FASTA output");
        let mut report_provenance = provenance;
        report_provenance.push(output_provenance.clone());
        let report = self.success_report(
            &request.context,
            format!("normalized {} sequence record(s)", outcome.records.len()),
            diagnostics,
            report_provenance,
        );
        let input_label = match &outcome.source {
            SeqretSource::LocalPath(path) => path.display().to_string(),
            SeqretSource::Retrieved {
                provider,
                accession,
            } => format!("{provider}:{accession}"),
        };
        let retrieval_mode = match &outcome.source {
            SeqretSource::LocalPath(_) => "local normalization".to_owned(),
            SeqretSource::Retrieved {
                provider,
                accession,
            } => {
                format!("provider-backed retrieval via {provider} for {accession}")
            }
        };
        let result = MethodResult::new(
            request.tool.clone(),
            ResultPayload::SequenceCollection(outcome.records),
            ResultSummary::new("Sequence retrieval and normalization completed")
                .with_line(format!("Input: {input_label}"))
                .with_line(format!("Mode: {retrieval_mode}"))
                .with_line("Output format: FASTA")
                .with_line(
                    "Record policy: local inputs may emit multiple records; retrieved inputs emit one record",
                ),
            report.clone(),
        )
        .with_artifact(
            ArtifactReference::new("stdout", ArtifactKind::Sequence)
                .with_label("normalized FASTA")
                .with_provenance(output_provenance),
        );

        Ok(InvocationResponse::completed(
            request.context,
            request.tool,
            descriptor,
            report,
            result,
        ))
    }

    fn invoke_refseqget(
        &self,
        request: InvocationRequest,
        descriptor: ToolDescriptor,
    ) -> Result<InvocationResponse, ServiceError> {
        self.invoke_refseqget_inner::<emboss_providers::ReqwestHttpClient>(
            request, descriptor, None,
        )
    }

    fn invoke_refseqget_inner<C: ProviderHttpClient>(
        &self,
        request: InvocationRequest,
        descriptor: ToolDescriptor,
        client: Option<&C>,
    ) -> Result<InvocationResponse, ServiceError> {
        if help_requested(request.arguments()) {
            return Ok(self.help_response(request, descriptor, refseqget_help()));
        }

        let [input]: [String; 1] = request
            .arguments
            .clone()
            .try_into()
            .map_err(|_| tool_usage_error("refseqget", refseqget_help()))?;
        let (route_provenance, retrieved, diagnostics) =
            self.resolve_refseqget_record_with_client(&input, client)?;
        let outcome = run_refseqget(RefseqgetParams {
            provider: retrieved.provider.as_str().to_owned(),
            accession: retrieved.requested_accession.clone(),
            record: retrieved.record.clone(),
        })?;

        let output_provenance = ArtifactProvenance::generated_output("stdout")
            .with_description("normalized FASTA output");
        let report = self.success_report(
            &request.context,
            format!("retrieved one sequence record from {}", outcome.provider),
            diagnostics,
            vec![
                route_provenance,
                retrieved.provenance.clone(),
                output_provenance.clone(),
            ],
        );
        let result = MethodResult::new(
            request.tool.clone(),
            ResultPayload::Sequence(outcome.record),
            ResultSummary::new("Reference sequence retrieved")
                .with_line(format!("Provider: {}", outcome.provider))
                .with_line(format!("Accession: {}", outcome.accession))
                .with_line(format!("Route: {}", retrieved.route.endpoint))
                .with_line("Output format: FASTA")
                .with_line("Input policy: provider-qualified accession retrieval only"),
            report.clone(),
        )
        .with_artifact(
            ArtifactReference::new("stdout", ArtifactKind::Sequence)
                .with_label("normalized FASTA")
                .with_provenance(output_provenance),
        );

        Ok(InvocationResponse::completed(
            request.context,
            request.tool,
            descriptor,
            report,
            result,
        ))
    }

    fn invoke_seqcount(
        &self,
        request: InvocationRequest,
        descriptor: ToolDescriptor,
    ) -> Result<InvocationResponse, ServiceError> {
        if help_requested(request.arguments()) {
            return Ok(self.help_response(request, descriptor, seqcount_help()));
        }

        let [input]: [String; 1] = request
            .arguments
            .clone()
            .try_into()
            .map_err(|_| tool_usage_error("seqcount", seqcount_help()))?;
        let (input, input_provenance, input_diagnostics) =
            self.resolve_local_sequence_input(&input)?;
        let outcome = run_seqcount(SeqcountParams { input })?;
        let output_provenance = ArtifactProvenance::generated_output("stdout")
            .with_description("sequence count report");

        let report = self.success_report(
            &request.context,
            format!("counted {} sequence records", outcome.count),
            input_diagnostics,
            vec![input_provenance, output_provenance.clone()],
        );
        let result = MethodResult::new(
            request.tool.clone(),
            ResultPayload::TableReport(TableReport::new(
                vec!["input".to_owned(), "count".to_owned()],
                vec![vec![
                    outcome.input.path.display().to_string(),
                    outcome.count.to_string(),
                ]],
            )),
            ResultSummary::new("Sequence count completed")
                .with_line(format!("Input: {}", outcome.input.path.display()))
                .with_line(format!("Records: {}", outcome.count))
                .with_line("Output format: tabular report"),
            report.clone(),
        )
        .with_artifact(
            ArtifactReference::new("sequence-count-report", ArtifactKind::Table)
                .with_label("Sequence count report")
                .with_provenance(output_provenance),
        );

        Ok(InvocationResponse::completed(
            request.context,
            request.tool,
            descriptor,
            report,
            result,
        ))
    }

    fn invoke_nthseq(
        &self,
        request: InvocationRequest,
        descriptor: ToolDescriptor,
    ) -> Result<InvocationResponse, ServiceError> {
        if help_requested(request.arguments()) {
            return Ok(self.help_response(request, descriptor, nthseq_help()));
        }

        let arguments: [String; 2] = request
            .arguments
            .clone()
            .try_into()
            .map_err(|_| tool_usage_error("nthseq", nthseq_help()))?;
        let (input, input_provenance, input_diagnostics) =
            self.resolve_local_sequence_input(&arguments[0])?;
        let index = parse_positive_index("nthseq", &arguments[1])?;
        let outcome = run_nthseq(NthseqParams { input, index })?;

        let output_provenance = ArtifactProvenance::generated_output("stdout")
            .with_description("selected FASTA output");
        let report = self.success_report(
            &request.context,
            format!("selected sequence {}", outcome.index),
            input_diagnostics,
            vec![input_provenance, output_provenance.clone()],
        );
        let result = MethodResult::new(
            request.tool.clone(),
            ResultPayload::Sequence(outcome.record),
            ResultSummary::new("Nth sequence selected")
                .with_line(format!("Input: {}", outcome.input.path.display()))
                .with_line(format!("Selected index: {}", outcome.index))
                .with_line(format!("Total records: {}", outcome.total_count))
                .with_line("Output format: fasta"),
            report.clone(),
        )
        .with_artifact(
            ArtifactReference::new("selected-sequence", ArtifactKind::Sequence)
                .with_label("Selected sequence")
                .with_provenance(output_provenance),
        );

        Ok(InvocationResponse::completed(
            request.context,
            request.tool,
            descriptor,
            report,
            result,
        ))
    }

    fn invoke_skipseq(
        &self,
        request: InvocationRequest,
        descriptor: ToolDescriptor,
    ) -> Result<InvocationResponse, ServiceError> {
        if help_requested(request.arguments()) {
            return Ok(self.help_response(request, descriptor, skipseq_help()));
        }

        let arguments: [String; 2] = request
            .arguments
            .clone()
            .try_into()
            .map_err(|_| tool_usage_error("skipseq", skipseq_help()))?;
        let (input, input_provenance, input_diagnostics) =
            self.resolve_local_sequence_input(&arguments[0])?;
        let count = parse_non_negative_count("skipseq", &arguments[1])?;
        let outcome = run_skipseq(SkipseqParams { input, count })?;

        let output_provenance = ArtifactProvenance::generated_output("stdout")
            .with_description("remaining FASTA output");
        let report = self.success_report(
            &request.context,
            format!(
                "returned {} records after skipping {}",
                outcome.records.len(),
                outcome.skipped_count
            ),
            input_diagnostics,
            vec![input_provenance, output_provenance.clone()],
        );
        let result = MethodResult::new(
            request.tool.clone(),
            ResultPayload::SequenceCollection(outcome.records),
            ResultSummary::new("Sequence stream filtered")
                .with_line(format!("Input: {}", outcome.input.path.display()))
                .with_line(format!("Skipped: {}", outcome.skipped_count))
                .with_line(format!("Input records: {}", outcome.total_count))
                .with_line(format!(
                    "Returned: {}",
                    outcome.total_count.saturating_sub(outcome.skipped_count)
                ))
                .with_line("Output format: fasta"),
            report.clone(),
        )
        .with_artifact(
            ArtifactReference::new("remaining-sequences", ArtifactKind::Sequence)
                .with_label("Remaining sequences")
                .with_provenance(output_provenance),
        );

        Ok(InvocationResponse::completed(
            request.context,
            request.tool,
            descriptor,
            report,
            result,
        ))
    }

    fn invoke_notseq(
        &self,
        request: InvocationRequest,
        descriptor: ToolDescriptor,
    ) -> Result<InvocationResponse, ServiceError> {
        if help_requested(request.arguments()) {
            return Ok(self.help_response(request, descriptor, notseq_help()));
        }

        let arguments: [String; 2] = request
            .arguments
            .clone()
            .try_into()
            .map_err(|_| tool_usage_error("notseq", notseq_help()))?;
        let (input, input_provenance, input_diagnostics) =
            self.resolve_local_sequence_input(&arguments[0])?;
        let excluded_index = parse_positive_index("notseq", &arguments[1])?;
        let outcome = run_notseq(NotseqParams {
            input,
            excluded_index,
        })?;

        let output_provenance = ArtifactProvenance::generated_output("stdout")
            .with_description("filtered FASTA output");
        let report = self.success_report(
            &request.context,
            format!("excluded sequence {}", outcome.excluded_index),
            input_diagnostics,
            vec![input_provenance, output_provenance.clone()],
        );
        let result = MethodResult::new(
            request.tool.clone(),
            ResultPayload::SequenceCollection(outcome.records),
            ResultSummary::new("Sequence excluded from stream")
                .with_line(format!("Input: {}", outcome.input.path.display()))
                .with_line(format!("Excluded index: {}", outcome.excluded_index))
                .with_line(format!("Input records: {}", outcome.total_count))
                .with_line(format!(
                    "Returned: {}",
                    outcome.total_count.saturating_sub(1)
                ))
                .with_line("Output format: fasta"),
            report.clone(),
        )
        .with_artifact(
            ArtifactReference::new("filtered-sequences", ArtifactKind::Sequence)
                .with_label("Filtered sequences")
                .with_provenance(output_provenance),
        );

        Ok(InvocationResponse::completed(
            request.context,
            request.tool,
            descriptor,
            report,
            result,
        ))
    }

    fn invoke_newseq(
        &self,
        request: InvocationRequest,
        descriptor: ToolDescriptor,
    ) -> Result<InvocationResponse, ServiceError> {
        if help_requested(request.arguments()) {
            return Ok(self.help_response(request, descriptor, newseq_help()));
        }

        let params = parse_newseq_params(request.arguments())?;
        let outcome = run_newseq(params)?;
        let output_provenance =
            ArtifactProvenance::generated_output("stdout").with_description("created FASTA output");
        let report = self.success_report(
            &request.context,
            format!(
                "created sequence {}",
                outcome.record.identifier().accession()
            ),
            Vec::new(),
            vec![output_provenance.clone()],
        );
        let result = MethodResult::new(
            request.tool.clone(),
            ResultPayload::Sequence(outcome.record.clone()),
            ResultSummary::new("Sequence record created")
                .with_line(format!(
                    "Identifier: {}",
                    outcome.record.identifier().accession()
                ))
                .with_line(format!("Length: {}", outcome.record.len()))
                .with_line(format!("Molecule: {}", outcome.record.molecule()))
                .with_line(format!("Alphabet: {}", outcome.record.alphabet()))
                .with_line(format!(
                    "Description: {}",
                    outcome
                        .record
                        .metadata()
                        .description
                        .clone()
                        .unwrap_or_else(|| "-".to_owned())
                ))
                .with_line("Output format: fasta"),
            report.clone(),
        )
        .with_artifact(
            ArtifactReference::new("created-sequence", ArtifactKind::Sequence)
                .with_label("Created sequence")
                .with_provenance(output_provenance),
        );

        Ok(InvocationResponse::completed(
            request.context,
            request.tool,
            descriptor,
            report,
            result,
        ))
    }

    fn invoke_extractseq(
        &self,
        request: InvocationRequest,
        descriptor: ToolDescriptor,
    ) -> Result<InvocationResponse, ServiceError> {
        if help_requested(request.arguments()) {
            return Ok(self.help_response(request, descriptor, extractseq_help()));
        }

        let arguments: [String; 3] = request
            .arguments
            .clone()
            .try_into()
            .map_err(|_| tool_usage_error("extractseq", extractseq_help()))?;
        let (input, input_provenance, input_diagnostics) =
            self.resolve_local_sequence_input(&arguments[0])?;
        let start = parse_positive_index("extractseq", &arguments[1])?;
        let end = parse_positive_index("extractseq", &arguments[2])?;
        let outcome = run_extractseq(ExtractseqParams { input, start, end })?;

        let output_provenance = ArtifactProvenance::generated_output("stdout")
            .with_description("extracted FASTA output");
        let report = self.success_report(
            &request.context,
            format!("extracted region {}..{}", outcome.start, outcome.end),
            input_diagnostics,
            vec![input_provenance, output_provenance.clone()],
        );
        let result = MethodResult::new(
            request.tool.clone(),
            ResultPayload::SequenceCollection(outcome.records),
            ResultSummary::new("Sequence region extracted")
                .with_line(format!("Input: {}", outcome.input.path.display()))
                .with_line(format!("Coordinates: {}..{}", outcome.start, outcome.end))
                .with_line("Coordinate convention: 1-based inclusive")
                .with_line("Output format: fasta"),
            report.clone(),
        )
        .with_artifact(
            ArtifactReference::new("extracted-sequences", ArtifactKind::Sequence)
                .with_label("Extracted sequences")
                .with_provenance(output_provenance),
        );

        Ok(InvocationResponse::completed(
            request.context,
            request.tool,
            descriptor,
            report,
            result,
        ))
    }

    fn invoke_degapseq(
        &self,
        request: InvocationRequest,
        descriptor: ToolDescriptor,
    ) -> Result<InvocationResponse, ServiceError> {
        if help_requested(request.arguments()) {
            return Ok(self.help_response(request, descriptor, degapseq_help()));
        }

        let [input]: [String; 1] = request
            .arguments
            .clone()
            .try_into()
            .map_err(|_| tool_usage_error("degapseq", degapseq_help()))?;
        let (input, input_provenance, input_diagnostics) =
            self.resolve_local_sequence_input(&input)?;
        let outcome = run_degapseq(DegapseqParams { input })?;

        let output_provenance = ArtifactProvenance::generated_output("stdout")
            .with_description("degapped FASTA output");
        let report = self.success_report(
            &request.context,
            format!(
                "removed gap characters from {} records",
                outcome.records.len()
            ),
            input_diagnostics,
            vec![input_provenance, output_provenance.clone()],
        );
        let result = MethodResult::new(
            request.tool.clone(),
            ResultPayload::SequenceCollection(outcome.records),
            ResultSummary::new("Sequence gaps removed")
                .with_line(format!("Input: {}", outcome.input.path.display()))
                .with_line("Removed characters: '-' and '.'")
                .with_line("Output format: fasta"),
            report.clone(),
        )
        .with_artifact(
            ArtifactReference::new("degapped-sequences", ArtifactKind::Sequence)
                .with_label("Degapped sequences")
                .with_provenance(output_provenance),
        );

        Ok(InvocationResponse::completed(
            request.context,
            request.tool,
            descriptor,
            report,
            result,
        ))
    }

    fn invoke_revseq(
        &self,
        request: InvocationRequest,
        descriptor: ToolDescriptor,
    ) -> Result<InvocationResponse, ServiceError> {
        if help_requested(request.arguments()) {
            return Ok(self.help_response(request, descriptor, revseq_help()));
        }

        let params = parse_revseq_params(request.arguments())?;
        let (input, input_provenance, input_diagnostics) =
            self.resolve_local_sequence_input(&params.input.path.display().to_string())?;
        let outcome = run_revseq(RevseqParams {
            input,
            mode: params.mode,
        })?;

        let output_provenance =
            ArtifactProvenance::generated_output("stdout").with_description("revseq FASTA output");
        let report = self.success_report(
            &request.context,
            format!(
                "transformed {} records with {}",
                outcome.records.len(),
                outcome.mode
            ),
            input_diagnostics,
            vec![input_provenance, output_provenance.clone()],
        );
        let result = MethodResult::new(
            request.tool.clone(),
            ResultPayload::SequenceCollection(outcome.records),
            ResultSummary::new("Sequence reversal completed")
                .with_line(format!("Input: {}", outcome.input.path.display()))
                .with_line(format!("Mode: {}", outcome.mode))
                .with_line(
                    "Default auto behavior: reverse-complement DNA/RNA, reverse protein/unknown",
                )
                .with_line("Output format: fasta"),
            report.clone(),
        )
        .with_artifact(
            ArtifactReference::new("reversed-sequences", ArtifactKind::Sequence)
                .with_label("Revseq output sequences")
                .with_provenance(output_provenance),
        );

        Ok(InvocationResponse::completed(
            request.context,
            request.tool,
            descriptor,
            report,
            result,
        ))
    }

    fn invoke_trimseq(
        &self,
        request: InvocationRequest,
        descriptor: ToolDescriptor,
    ) -> Result<InvocationResponse, ServiceError> {
        if help_requested(request.arguments()) {
            return Ok(self.help_response(request, descriptor, trimseq_help()));
        }

        let params = parse_trimseq_params(request.arguments())?;
        let input_path = params.input.path.display().to_string();
        let (input, input_provenance, input_diagnostics) =
            self.resolve_local_sequence_input(&input_path)?;
        let outcome = run_trimseq(TrimseqParams {
            input,
            left_trim: params.left_trim,
            right_trim: params.right_trim,
        })?;

        let output_provenance =
            ArtifactProvenance::generated_output("stdout").with_description("trimmed FASTA output");
        let report = self.success_report(
            &request.context,
            format!("trimmed {} records", outcome.records.len()),
            input_diagnostics,
            vec![input_provenance, output_provenance.clone()],
        );
        let result = MethodResult::new(
            request.tool.clone(),
            ResultPayload::SequenceCollection(outcome.records),
            ResultSummary::new("Sequence trimming completed")
                .with_line(format!("Input: {}", outcome.input.path.display()))
                .with_line(format!("Left trim: {}", outcome.left_trim))
                .with_line(format!("Right trim: {}", outcome.right_trim))
                .with_line("Output format: fasta"),
            report.clone(),
        )
        .with_artifact(
            ArtifactReference::new("trimmed-sequences", ArtifactKind::Sequence)
                .with_label("Trimmed sequences")
                .with_provenance(output_provenance),
        );

        Ok(InvocationResponse::completed(
            request.context,
            request.tool,
            descriptor,
            report,
            result,
        ))
    }

    fn invoke_descseq(
        &self,
        request: InvocationRequest,
        descriptor: ToolDescriptor,
    ) -> Result<InvocationResponse, ServiceError> {
        if help_requested(request.arguments()) {
            return Ok(self.help_response(request, descriptor, descseq_help()));
        }

        let params = parse_descseq_params(request.arguments())?;
        let input_path = params.input.path.display().to_string();
        let (input, input_provenance, input_diagnostics) =
            self.resolve_local_sequence_input(&input_path)?;
        let outcome = run_descseq(DescseqParams { input })?;

        let rows = outcome
            .rows
            .iter()
            .map(|row| {
                vec![
                    row.ordinal.to_string(),
                    row.identifier.clone(),
                    row.display_name.clone().unwrap_or_else(|| "-".to_owned()),
                    row.description.clone().unwrap_or_else(|| "-".to_owned()),
                    row.length.to_string(),
                    row.molecule.clone(),
                    row.alphabet.clone(),
                    row.feature_count.to_string(),
                    row.source.clone().unwrap_or_else(|| "-".to_owned()),
                    row.organism.clone().unwrap_or_else(|| "-".to_owned()),
                    row.topology.clone().unwrap_or_else(|| "-".to_owned()),
                ]
            })
            .collect::<Vec<_>>();

        let output_provenance = ArtifactProvenance::generated_output("stdout")
            .with_description("sequence description table");
        let report = self.success_report(
            &request.context,
            format!("reported descriptions for {} records", outcome.rows.len()),
            input_diagnostics,
            vec![input_provenance, output_provenance.clone()],
        );
        let result = MethodResult::new(
            request.tool.clone(),
            ResultPayload::TableReport(TableReport::new(
                vec![
                    "ordinal".to_owned(),
                    "identifier".to_owned(),
                    "display_name".to_owned(),
                    "description".to_owned(),
                    "length".to_owned(),
                    "molecule".to_owned(),
                    "alphabet".to_owned(),
                    "feature_count".to_owned(),
                    "source".to_owned(),
                    "organism".to_owned(),
                    "topology".to_owned(),
                ],
                rows,
            )),
            ResultSummary::new("Sequence descriptions reported")
                .with_line(format!("Input: {}", outcome.input.path.display()))
                .with_line(format!("Records: {}", outcome.rows.len()))
                .with_line("Output format: tabular report")
                .with_line("Ordering: source order, one row per record"),
            report.clone(),
        )
        .with_artifact(
            ArtifactReference::new("sequence-description-report", ArtifactKind::Table)
                .with_label("Sequence description report")
                .with_provenance(output_provenance),
        );

        Ok(InvocationResponse::completed(
            request.context,
            request.tool,
            descriptor,
            report,
            result,
        ))
    }

    fn invoke_maskseq(
        &self,
        request: InvocationRequest,
        descriptor: ToolDescriptor,
    ) -> Result<InvocationResponse, ServiceError> {
        if help_requested(request.arguments()) {
            return Ok(self.help_response(request, descriptor, maskseq_help()));
        }

        let params = parse_maskseq_params(request.arguments())?;
        let input_path = params.input.path.display().to_string();
        let (input, input_provenance, input_diagnostics) =
            self.resolve_local_sequence_input(&input_path)?;
        let outcome = run_maskseq(MaskseqParams {
            input,
            intervals: params.intervals.clone(),
            mask_char: params.mask_char,
        })?;

        let output_provenance =
            ArtifactProvenance::generated_output("stdout").with_description("masked FASTA output");
        let report = self.success_report(
            &request.context,
            format!(
                "masked {} records across {} intervals",
                outcome.records.len(),
                outcome.intervals.len()
            ),
            input_diagnostics,
            vec![input_provenance, output_provenance.clone()],
        );
        let result = MethodResult::new(
            request.tool.clone(),
            ResultPayload::SequenceCollection(outcome.records),
            ResultSummary::new("Explicit sequence masking completed")
                .with_line(format!("Input: {}", outcome.input.path.display()))
                .with_line(format!(
                    "Intervals: {}",
                    format_interval_list(&outcome.intervals)
                ))
                .with_line("Coordinate convention: 1-based inclusive")
                .with_line(format!(
                    "Mask symbol: {}",
                    mask_char_summary(outcome.mask_char)
                ))
                .with_line("Output format: fasta"),
            report.clone(),
        )
        .with_artifact(
            ArtifactReference::new("masked-sequences", ArtifactKind::Sequence)
                .with_label("Masked sequences")
                .with_provenance(output_provenance),
        );

        Ok(InvocationResponse::completed(
            request.context,
            request.tool,
            descriptor,
            report,
            result,
        ))
    }

    fn invoke_maskfeat(
        &self,
        request: InvocationRequest,
        descriptor: ToolDescriptor,
    ) -> Result<InvocationResponse, ServiceError> {
        if help_requested(request.arguments()) {
            return Ok(self.help_response(request, descriptor, maskfeat_help()));
        }

        let params = parse_maskfeat_params(request.arguments())?;
        let input_path = params.input.path.display().to_string();
        let selector_summary = describe_selector(&params.selector);
        let (input, input_provenance, input_diagnostics) =
            self.resolve_local_sequence_input(&input_path)?;
        let outcome = run_maskfeat(MaskfeatParams {
            input,
            selector: params.selector.clone(),
            mask_char: params.mask_char,
        })?;

        let output_provenance = ArtifactProvenance::generated_output("stdout")
            .with_description("feature-masked FASTA output");
        let report = self.success_report(
            &request.context,
            format!(
                "masked {} selected features across {} records",
                outcome.selected_feature_count,
                outcome.records.len()
            ),
            input_diagnostics,
            vec![input_provenance, output_provenance.clone()],
        );
        let result = MethodResult::new(
            request.tool.clone(),
            ResultPayload::SequenceCollection(outcome.records),
            ResultSummary::new("Feature masking completed")
                .with_line(format!("Input: {}", outcome.input.path.display()))
                .with_line(format!("Selector: {selector_summary}"))
                .with_line(format!(
                    "Selected features: {}",
                    outcome.selected_feature_count
                ))
                .with_line(format!(
                    "Mask symbol: {}",
                    mask_char_summary(outcome.mask_char)
                ))
                .with_line("Output format: fasta (annotations retained in payload)"),
            report.clone(),
        )
        .with_artifact(
            ArtifactReference::new("feature-masked-sequences", ArtifactKind::Sequence)
                .with_label("Feature-masked sequences")
                .with_provenance(output_provenance),
        );

        Ok(InvocationResponse::completed(
            request.context,
            request.tool,
            descriptor,
            report,
            result,
        ))
    }

    fn invoke_extractfeat(
        &self,
        request: InvocationRequest,
        descriptor: ToolDescriptor,
    ) -> Result<InvocationResponse, ServiceError> {
        if help_requested(request.arguments()) {
            return Ok(self.help_response(request, descriptor, extractfeat_help()));
        }

        let params = parse_extractfeat_params(request.arguments())?;
        let input_path = params.input.path.display().to_string();
        let selector_summary = describe_selector(&params.selector);
        let (input, input_provenance, input_diagnostics) =
            self.resolve_local_sequence_input(&input_path)?;
        let outcome = run_extractfeat(ExtractfeatParams {
            input,
            selector: params.selector.clone(),
        })?;
        let extracted_records = outcome
            .records
            .into_iter()
            .map(|record| record.record)
            .collect::<Vec<_>>();

        let output_provenance = ArtifactProvenance::generated_output("stdout")
            .with_description("feature-extracted FASTA output");
        let report = self.success_report(
            &request.context,
            format!(
                "extracted {} feature-defined regions",
                extracted_records.len()
            ),
            input_diagnostics,
            vec![input_provenance, output_provenance.clone()],
        );
        let result = MethodResult::new(
            request.tool.clone(),
            ResultPayload::SequenceCollection(extracted_records),
            ResultSummary::new("Feature extraction completed")
                .with_line(format!("Input: {}", outcome.input.path.display()))
                .with_line(format!("Selector: {selector_summary}"))
                .with_line(format!(
                    "Extracted records: {}",
                    outcome.extracted_feature_count
                ))
                .with_line("Derived identifiers: <source>:<start>-<end>:<feature-or-kind>")
                .with_line("Output format: fasta (rebased feature retained in payload)"),
            report.clone(),
        )
        .with_artifact(
            ArtifactReference::new("extracted-feature-sequences", ArtifactKind::Sequence)
                .with_label("Feature-defined extracted sequences")
                .with_provenance(output_provenance),
        );

        Ok(InvocationResponse::completed(
            request.context,
            request.tool,
            descriptor,
            report,
            result,
        ))
    }

    fn invoke_featcopy(
        &self,
        request: InvocationRequest,
        descriptor: ToolDescriptor,
    ) -> Result<InvocationResponse, ServiceError> {
        if help_requested(request.arguments()) {
            return Ok(self.help_response(request, descriptor, featcopy_help()));
        }

        let params = parse_featcopy_params(request.arguments())?;
        let source_path = params.source.path.display().to_string();
        let target_path = params.target.path.display().to_string();
        let selector_summary = describe_selector(&params.selector);
        let (source, source_provenance, mut source_diagnostics) =
            self.resolve_local_sequence_input(&source_path)?;
        let (target, target_provenance, target_diagnostics) =
            self.resolve_local_sequence_input(&target_path)?;
        source_diagnostics.extend(target_diagnostics);
        let outcome = run_featcopy(FeatcopyParams {
            source,
            target,
            selector: params.selector.clone(),
        })?;

        let output_provenance = ArtifactProvenance::generated_output("stdout")
            .with_description("feature-copied FASTA output");
        let report = self.success_report(
            &request.context,
            format!(
                "copied {} features onto {} target records",
                outcome.copied_feature_count,
                outcome.records.len()
            ),
            source_diagnostics,
            vec![
                source_provenance,
                target_provenance,
                output_provenance.clone(),
            ],
        );
        let result = MethodResult::new(
            request.tool.clone(),
            ResultPayload::SequenceCollection(outcome.records),
            ResultSummary::new("Feature copy completed")
                .with_line(format!("Source: {}", outcome.source.path.display()))
                .with_line(format!("Target: {}", outcome.target.path.display()))
                .with_line(format!("Selector: {selector_summary}"))
                .with_line(format!("Copied features: {}", outcome.copied_feature_count))
                .with_line("Compatibility: pair by identifier and require equal lengths")
                .with_line("Output format: fasta (copied annotations retained in payload)"),
            report.clone(),
        )
        .with_artifact(
            ArtifactReference::new("feature-copied-sequences", ArtifactKind::Sequence)
                .with_label("Feature-copied sequences")
                .with_provenance(output_provenance),
        );

        Ok(InvocationResponse::completed(
            request.context,
            request.tool,
            descriptor,
            report,
            result,
        ))
    }

    fn invoke_coderet(
        &self,
        request: InvocationRequest,
        descriptor: ToolDescriptor,
    ) -> Result<InvocationResponse, ServiceError> {
        if help_requested(request.arguments()) {
            return Ok(self.help_response(request, descriptor, coderet_help()));
        }

        let params = parse_coderet_params(request.arguments())?;
        let input_path = params.input.path.display().to_string();
        let selector_summary = describe_selector(&params.selector);
        let (input, input_provenance, input_diagnostics) =
            self.resolve_local_sequence_input(&input_path)?;
        let outcome = run_coderet(CoderetParams {
            input,
            selector: params.selector.clone(),
            translate: params.translate,
        })?;

        let output_provenance = ArtifactProvenance::generated_output("stdout").with_description(
            if outcome.translate {
                "coding-feature translated FASTA output"
            } else {
                "coding-feature extracted FASTA output"
            },
        );
        let report = self.success_report(
            &request.context,
            format!(
                "returned {} coding-feature-derived records",
                outcome.records.len()
            ),
            input_diagnostics,
            vec![input_provenance, output_provenance.clone()],
        );
        let result = MethodResult::new(
            request.tool.clone(),
            ResultPayload::SequenceCollection(outcome.records),
            ResultSummary::new("Coding-feature retrieval completed")
                .with_line(format!("Input: {}", outcome.input.path.display()))
                .with_line(format!("Selector: {selector_summary}"))
                .with_line(format!("Translate: {}", outcome.translate))
                .with_line(format!(
                    "Returned records: {}",
                    outcome.extracted_record_count
                ))
                .with_line(if outcome.translate {
                    "Output format: fasta protein records"
                } else {
                    "Output format: fasta nucleotide records"
                }),
            report.clone(),
        )
        .with_artifact(
            ArtifactReference::new("coderet-sequences", ArtifactKind::Sequence)
                .with_label("Coding-feature-derived sequences")
                .with_provenance(output_provenance),
        );

        Ok(InvocationResponse::completed(
            request.context,
            request.tool,
            descriptor,
            report,
            result,
        ))
    }

    fn invoke_featmerge(
        &self,
        request: InvocationRequest,
        descriptor: ToolDescriptor,
    ) -> Result<InvocationResponse, ServiceError> {
        if help_requested(request.arguments()) {
            return Ok(self.help_response(request, descriptor, featmerge_help()));
        }

        let params = parse_featmerge_params(request.arguments())?;
        let left_path = params.left.path.display().to_string();
        let right_path = params.right.path.display().to_string();
        let selector_summary = describe_selector(&params.selector);
        let (left, left_provenance, mut left_diagnostics) =
            self.resolve_local_sequence_input(&left_path)?;
        let (right, right_provenance, right_diagnostics) =
            self.resolve_local_sequence_input(&right_path)?;
        left_diagnostics.extend(right_diagnostics);
        let outcome = run_featmerge(FeatmergeParams {
            left,
            right,
            selector: params.selector.clone(),
        })?;

        let output_provenance = ArtifactProvenance::generated_output("stdout")
            .with_description("merged-feature FASTA output");
        let report = self.success_report(
            &request.context,
            format!(
                "merged {} selected right-hand features across {} records",
                outcome.merged_feature_count,
                outcome.records.len()
            ),
            left_diagnostics,
            vec![left_provenance, right_provenance, output_provenance.clone()],
        );
        let result = MethodResult::new(
            request.tool.clone(),
            ResultPayload::SequenceCollection(outcome.records),
            ResultSummary::new("Feature merge completed")
                .with_line(format!("Left: {}", outcome.left.path.display()))
                .with_line(format!("Right: {}", outcome.right.path.display()))
                .with_line(format!("Selector: {selector_summary}"))
                .with_line(format!(
                    "Merged right-hand features: {}",
                    outcome.merged_feature_count
                ))
                .with_line(
                    "Compatibility: pair by identifier, require equal lengths, skip exact duplicate features",
                )
                .with_line("Output format: fasta (merged annotations retained in payload)"),
            report.clone(),
        )
        .with_artifact(
            ArtifactReference::new("merged-feature-sequences", ArtifactKind::Sequence)
                .with_label("Feature-merged sequences")
                .with_provenance(output_provenance),
        );

        Ok(InvocationResponse::completed(
            request.context,
            request.tool,
            descriptor,
            report,
            result,
        ))
    }

    fn invoke_featreport(
        &self,
        request: InvocationRequest,
        descriptor: ToolDescriptor,
    ) -> Result<InvocationResponse, ServiceError> {
        if help_requested(request.arguments()) {
            return Ok(self.help_response(request, descriptor, featreport_help()));
        }

        let params = parse_featreport_params(request.arguments())?;
        let input_path = params.input.path.display().to_string();
        let selector_summary = describe_selector(&params.selector);
        let (input, input_provenance, input_diagnostics) =
            self.resolve_local_sequence_input(&input_path)?;
        let outcome = run_featreport(FeatreportParams {
            input,
            selector: params.selector.clone(),
        })?;

        let output_provenance = ArtifactProvenance::generated_output("stdout")
            .with_description("feature summary table");
        let report = self.success_report(
            &request.context,
            format!("reported {} selected features", outcome.rows.len()),
            input_diagnostics,
            vec![input_provenance, output_provenance.clone()],
        );
        let result = MethodResult::new(
            request.tool.clone(),
            ResultPayload::TableReport(TableReport::new(
                vec![
                    "record".to_owned(),
                    "kind".to_owned(),
                    "location".to_owned(),
                    "start".to_owned(),
                    "end".to_owned(),
                    "strand".to_owned(),
                    "name".to_owned(),
                    "qualifier_count".to_owned(),
                ],
                outcome.rows,
            )),
            ResultSummary::new("Feature report completed")
                .with_line(format!("Input: {}", outcome.input.path.display()))
                .with_line(format!("Selector: {selector_summary}"))
                .with_line("Ordering: stable source-record order then feature order")
                .with_line("Output format: governed table report"),
            report.clone(),
        )
        .with_artifact(
            ArtifactReference::new("feature-report", ArtifactKind::Table)
                .with_label("Feature report")
                .with_provenance(output_provenance),
        );

        Ok(InvocationResponse::completed(
            request.context,
            request.tool,
            descriptor,
            report,
            result,
        ))
    }

    fn invoke_feattext(
        &self,
        request: InvocationRequest,
        descriptor: ToolDescriptor,
    ) -> Result<InvocationResponse, ServiceError> {
        if help_requested(request.arguments()) {
            return Ok(self.help_response(request, descriptor, feattext_help()));
        }

        let params = parse_feattext_params(request.arguments())?;
        let input_path = params.input.path.display().to_string();
        let selector_summary = describe_selector(&params.selector);
        let (input, input_provenance, input_diagnostics) =
            self.resolve_local_sequence_input(&input_path)?;
        let outcome = run_feattext(FeattextParams {
            input,
            selector: params.selector.clone(),
        })?;

        let output_provenance = ArtifactProvenance::generated_output("stdout")
            .with_description("normalized feature-table text");
        let report = self.success_report(
            &request.context,
            "rendered normalized feature-table text",
            input_diagnostics,
            vec![input_provenance, output_provenance.clone()],
        );
        let result = MethodResult::new(
            request.tool.clone(),
            ResultPayload::TextReport(TextReport::new(outcome.body).with_title("feattext")),
            ResultSummary::new("Feature text rendering completed")
                .with_line(format!("Input: {}", outcome.input.path.display()))
                .with_line(format!("Selector: {selector_summary}"))
                .with_line("Rendering: normalized feature-table text, not source-exact recovery"),
            report.clone(),
        )
        .with_artifact(
            ArtifactReference::new("feature-text", ArtifactKind::FeatureTable)
                .with_label("Normalized feature table")
                .with_provenance(output_provenance),
        );

        Ok(InvocationResponse::completed(
            request.context,
            request.tool,
            descriptor,
            report,
            result,
        ))
    }

    fn invoke_backtranseq(
        &self,
        request: InvocationRequest,
        descriptor: ToolDescriptor,
    ) -> Result<InvocationResponse, ServiceError> {
        if help_requested(request.arguments()) {
            return Ok(self.help_response(request, descriptor, backtranseq_help()));
        }

        let [input]: [String; 1] = request
            .arguments
            .clone()
            .try_into()
            .map_err(|_| tool_usage_error("backtranseq", backtranseq_help()))?;
        let (input, input_provenance, input_diagnostics) =
            self.resolve_local_sequence_input(&input)?;
        let outcome = run_backtranseq(BacktranseqParams { input })?;

        let output_provenance = ArtifactProvenance::generated_output("stdout")
            .with_description("representative back-translated FASTA output");
        let report = self.success_report(
            &request.context,
            format!("back-translated {} protein records", outcome.records.len()),
            input_diagnostics,
            vec![input_provenance, output_provenance.clone()],
        );
        let result = MethodResult::new(
            request.tool.clone(),
            ResultPayload::SequenceCollection(outcome.records),
            ResultSummary::new("Representative back-translation completed")
                .with_line(format!("Input: {}", outcome.input.path.display()))
                .with_line("Genetic code: standard")
                .with_line("Codon policy: one deterministic representative DNA codon per residue")
                .with_line("Stop symbol handling: '*' -> TAA")
                .with_line("Output format: fasta"),
            report.clone(),
        )
        .with_artifact(
            ArtifactReference::new("backtranslated-sequences", ArtifactKind::Sequence)
                .with_label("Representative back-translated sequences")
                .with_provenance(output_provenance),
        );

        Ok(InvocationResponse::completed(
            request.context,
            request.tool,
            descriptor,
            report,
            result,
        ))
    }

    fn invoke_transeq(
        &self,
        request: InvocationRequest,
        descriptor: ToolDescriptor,
    ) -> Result<InvocationResponse, ServiceError> {
        if help_requested(request.arguments()) {
            return Ok(self.help_response(request, descriptor, transeq_help()));
        }

        let params = parse_transeq_params(request.arguments())?;
        let (input, input_provenance, input_diagnostics) =
            self.resolve_local_sequence_input(&params.input.path.display().to_string())?;
        let outcome = run_transeq(TranseqParams {
            input,
            frame: params.frame,
        })?;

        let output_provenance = ArtifactProvenance::generated_output("stdout")
            .with_description("translated protein FASTA output");
        let report = self.success_report(
            &request.context,
            format!("translated {} protein records", outcome.records.len()),
            input_diagnostics,
            vec![input_provenance, output_provenance.clone()],
        );
        let result = MethodResult::new(
            request.tool.clone(),
            ResultPayload::SequenceCollection(outcome.records),
            ResultSummary::new("Forward translation completed")
                .with_line(format!("Input: {}", outcome.input.path.display()))
                .with_line(format!(
                    "Frame selection: {}",
                    match outcome.frame {
                        TranslationFrameSelection::Frame1 => "frame 1",
                        TranslationFrameSelection::Frame2 => "frame 2",
                        TranslationFrameSelection::Frame3 => "frame 3",
                        TranslationFrameSelection::AllForward => "all forward frames",
                    }
                ))
                .with_line("Genetic code: standard")
                .with_line("Trailing partial codons: ignored")
                .with_line("Output format: fasta"),
            report.clone(),
        )
        .with_artifact(
            ArtifactReference::new("translated-sequences", ArtifactKind::Sequence)
                .with_label("Translated protein sequences")
                .with_provenance(output_provenance),
        );

        Ok(InvocationResponse::completed(
            request.context,
            request.tool,
            descriptor,
            report,
            result,
        ))
    }

    fn invoke_getorf(
        &self,
        request: InvocationRequest,
        descriptor: ToolDescriptor,
    ) -> Result<InvocationResponse, ServiceError> {
        if help_requested(request.arguments()) {
            return Ok(self.help_response(request, descriptor, getorf_help()));
        }

        let [input]: [String; 1] = request
            .arguments
            .clone()
            .try_into()
            .map_err(|_| tool_usage_error("getorf", getorf_help()))?;
        let (input, input_provenance, input_diagnostics) =
            self.resolve_local_sequence_input(&input)?;
        let outcome = run_getorf(GetorfParams { input })?;

        let output_provenance = ArtifactProvenance::generated_output("stdout")
            .with_description("ORF FASTA output");
        let report = self.success_report(
            &request.context,
            format!("extracted {} ORFs", outcome.records.len()),
            input_diagnostics,
            vec![input_provenance, output_provenance.clone()],
        );
        let result = MethodResult::new(
            request.tool.clone(),
            ResultPayload::SequenceCollection(outcome.records),
            ResultSummary::new("Forward ORF extraction completed")
                .with_line(format!("Input: {}", outcome.input.path.display()))
                .with_line("Frame policy: forward frames 1-3")
                .with_line("ORF policy: ATG start to first in-frame stop, stop codon included")
                .with_line(format!("ORFs: {}", outcome.cases.len()))
                .with_line("Output format: fasta"),
            report.clone(),
        )
        .with_artifact(
            ArtifactReference::new("orf-sequences", ArtifactKind::Sequence)
                .with_label("Extracted ORF sequences")
                .with_provenance(output_provenance),
        );

        Ok(InvocationResponse::completed(
            request.context,
            request.tool,
            descriptor,
            report,
            result,
        ))
    }

    fn invoke_prettyseq(
        &self,
        request: InvocationRequest,
        descriptor: ToolDescriptor,
    ) -> Result<InvocationResponse, ServiceError> {
        if help_requested(request.arguments()) {
            return Ok(self.help_response(request, descriptor, prettyseq_help()));
        }

        let params = parse_prettyseq_params(request.arguments())?;
        let (input, input_provenance, input_diagnostics) =
            self.resolve_local_sequence_input(&params.input.path.display().to_string())?;
        let outcome = run_prettyseq(PrettyseqParams {
            input,
            frame: params.frame,
            width: params.width,
        })?;

        let output_provenance = ArtifactProvenance::generated_output("stdout")
            .with_description("pretty-formatted nucleotide and translation report");
        let report = self.success_report(
            &request.context,
            format!("rendered pretty sequence view for {} records", outcome.record_count),
            input_diagnostics,
            vec![input_provenance, output_provenance.clone()],
        );
        let result = MethodResult::new(
            request.tool.clone(),
            ResultPayload::TextReport(TextReport::new(outcome.body).with_title("prettyseq")),
            ResultSummary::new("Pretty sequence report rendered")
                .with_line(format!("Input: {}", outcome.input.path.display()))
                .with_line(format!(
                    "Frame: {}",
                    match outcome.frame {
                        TranslationFrameSelection::Frame1 => "1",
                        TranslationFrameSelection::Frame2 => "2",
                        TranslationFrameSelection::Frame3 => "3",
                        TranslationFrameSelection::AllForward => "all",
                    }
                ))
                .with_line(format!("Width: {}", outcome.width))
                .with_line("Translation policy: forward frame only, trailing partial codons ignored")
                .with_line(format!("Records: {}", outcome.record_count)),
            report.clone(),
        )
        .with_artifact(
            ArtifactReference::new("prettyseq-report", ArtifactKind::Text)
                .with_label("Pretty sequence report")
                .with_provenance(output_provenance),
        );

        Ok(InvocationResponse::completed(
            request.context,
            request.tool,
            descriptor,
            report,
            result,
        ))
    }

    fn invoke_tranalign(
        &self,
        request: InvocationRequest,
        descriptor: ToolDescriptor,
    ) -> Result<InvocationResponse, ServiceError> {
        if help_requested(request.arguments()) {
            return Ok(self.help_response(request, descriptor, tranalign_help()));
        }

        let arguments: [String; 2] = request
            .arguments
            .clone()
            .try_into()
            .map_err(|_| tool_usage_error("tranalign", tranalign_help()))?;
        let (protein_alignment, alignment_provenance, mut diagnostics) =
            self.resolve_local_alignment_input(&arguments[0])?;
        let (nucleotide_input, nucleotide_provenance, nucleotide_diagnostics) =
            self.resolve_local_sequence_input(&arguments[1])?;
        diagnostics.extend(nucleotide_diagnostics);
        let outcome = run_tranalign(TranalignParams {
            protein_alignment,
            nucleotide_input,
        })?;

        let output_provenance = ArtifactProvenance::generated_output("stdout")
            .with_description("codon-projected Stockholm alignment");
        let report = self.success_report(
            &request.context,
            format!("projected {} aligned rows into codon space", outcome.alignment.row_count()),
            diagnostics,
            vec![
                alignment_provenance,
                nucleotide_provenance,
                output_provenance.clone(),
            ],
        );
        let result = MethodResult::new(
            request.tool.clone(),
            ResultPayload::Alignment(outcome.alignment),
            ResultSummary::new("Codon alignment projected")
                .with_line(format!(
                    "Protein alignment: {}",
                    outcome.protein_alignment.path.display()
                ))
                .with_line(format!(
                    "Nucleotide input: {}",
                    outcome.nucleotide_input.path.display()
                ))
                .with_line("Compatibility: exact identifier pairing and strict frame-1 translation")
                .with_line("Gap policy: protein gaps become triple-nucleotide gaps")
                .with_line("Output format: Stockholm"),
            report.clone(),
        )
        .with_artifact(
            ArtifactReference::new("codon-alignment", ArtifactKind::Alignment)
                .with_label("Projected codon alignment")
                .with_provenance(output_provenance),
        );

        Ok(InvocationResponse::completed(
            request.context,
            request.tool,
            descriptor,
            report,
            result,
        ))
    }

    fn invoke_fuzznuc(
        &self,
        request: InvocationRequest,
        descriptor: ToolDescriptor,
    ) -> Result<InvocationResponse, ServiceError> {
        if help_requested(request.arguments()) {
            return Ok(self.help_response(request, descriptor, fuzznuc_help()));
        }

        let arguments: [String; 2] = request
            .arguments
            .clone()
            .try_into()
            .map_err(|_| tool_usage_error("fuzznuc", fuzznuc_help()))?;
        let (input, input_provenance, input_diagnostics) =
            self.resolve_local_sequence_input(&arguments[0])?;
        let pattern = parse_nucleotide_pattern("fuzznuc", &arguments[1])?;
        let outcome = run_fuzznuc(FuzznucParams { input, pattern })?;

        let report = self.success_report(
            &request.context,
            format!("reported {} nucleotide pattern hits", outcome.hits.len()),
            input_diagnostics,
            vec![input_provenance],
        );
        let rows = outcome
            .hits
            .iter()
            .map(|hit| {
                vec![
                    hit.record_id.clone(),
                    hit.pattern.clone(),
                    hit.strand.clone(),
                    (hit.start + 1).to_string(),
                    hit.end.to_string(),
                    hit.matched.clone(),
                ]
            })
            .collect();
        let result = MethodResult::new(
            request.tool.clone(),
            ResultPayload::TableReport(TableReport::new(
                vec![
                    "record".to_owned(),
                    "pattern".to_owned(),
                    "strand".to_owned(),
                    "start".to_owned(),
                    "end".to_owned(),
                    "matched".to_owned(),
                ],
                rows,
            )),
            ResultSummary::new("Nucleotide pattern search completed")
                .with_line(format!("Input: {}", outcome.input.path.display()))
                .with_line(format!("Pattern: {}", outcome.pattern))
                .with_line("Coordinate convention: 1-based inclusive")
                .with_line("Strand policy: forward only")
                .with_line(format!("Hits: {}", outcome.hits.len())),
            report.clone(),
        );

        Ok(InvocationResponse::completed(
            request.context,
            request.tool,
            descriptor,
            report,
            result,
        ))
    }

    fn invoke_fuzzpro(
        &self,
        request: InvocationRequest,
        descriptor: ToolDescriptor,
    ) -> Result<InvocationResponse, ServiceError> {
        if help_requested(request.arguments()) {
            return Ok(self.help_response(request, descriptor, fuzzpro_help()));
        }

        let arguments: [String; 2] = request
            .arguments
            .clone()
            .try_into()
            .map_err(|_| tool_usage_error("fuzzpro", fuzzpro_help()))?;
        let (input, input_provenance, input_diagnostics) =
            self.resolve_local_sequence_input(&arguments[0])?;
        let pattern = parse_protein_pattern("fuzzpro", &arguments[1])?;
        let outcome = run_fuzzpro(FuzzproParams { input, pattern })?;

        let report = self.success_report(
            &request.context,
            format!("reported {} protein pattern hits", outcome.hits.len()),
            input_diagnostics,
            vec![input_provenance],
        );
        let rows = outcome
            .hits
            .iter()
            .map(|hit| {
                vec![
                    hit.record_id.clone(),
                    hit.pattern.clone(),
                    (hit.start + 1).to_string(),
                    hit.end.to_string(),
                    hit.matched.clone(),
                ]
            })
            .collect();
        let result = MethodResult::new(
            request.tool.clone(),
            ResultPayload::TableReport(TableReport::new(
                vec![
                    "record".to_owned(),
                    "pattern".to_owned(),
                    "start".to_owned(),
                    "end".to_owned(),
                    "matched".to_owned(),
                ],
                rows,
            )),
            ResultSummary::new("Protein pattern search completed")
                .with_line(format!("Input: {}", outcome.input.path.display()))
                .with_line(format!("Pattern: {}", outcome.pattern))
                .with_line("Coordinate convention: 1-based inclusive")
                .with_line("Pattern syntax: exact residues with X wildcard")
                .with_line(format!("Hits: {}", outcome.hits.len())),
            report.clone(),
        );

        Ok(InvocationResponse::completed(
            request.context,
            request.tool,
            descriptor,
            report,
            result,
        ))
    }

    fn invoke_fuzztran(
        &self,
        request: InvocationRequest,
        descriptor: ToolDescriptor,
    ) -> Result<InvocationResponse, ServiceError> {
        if help_requested(request.arguments()) {
            return Ok(self.help_response(request, descriptor, fuzztran_help()));
        }

        let arguments: [String; 2] = request
            .arguments
            .clone()
            .try_into()
            .map_err(|_| tool_usage_error("fuzztran", fuzztran_help()))?;
        let (input, input_provenance, input_diagnostics) =
            self.resolve_local_sequence_input(&arguments[0])?;
        let pattern = parse_protein_pattern("fuzztran", &arguments[1])?;
        let outcome = run_fuzztran(FuzztranParams { input, pattern })?;

        let report = self.success_report(
            &request.context,
            format!("reported {} translated pattern hits", outcome.hits.len()),
            input_diagnostics,
            vec![input_provenance],
        );
        let rows = outcome
            .hits
            .iter()
            .map(|hit| {
                vec![
                    hit.record_id.clone(),
                    hit.pattern.clone(),
                    hit.frame.to_string(),
                    (hit.amino_start + 1).to_string(),
                    hit.amino_end.to_string(),
                    (hit.nucleotide_start + 1).to_string(),
                    hit.nucleotide_end.to_string(),
                    hit.matched.clone(),
                ]
            })
            .collect();
        let result = MethodResult::new(
            request.tool.clone(),
            ResultPayload::TableReport(TableReport::new(
                vec![
                    "record".to_owned(),
                    "pattern".to_owned(),
                    "frame".to_owned(),
                    "aa_start".to_owned(),
                    "aa_end".to_owned(),
                    "nt_start".to_owned(),
                    "nt_end".to_owned(),
                    "matched".to_owned(),
                ],
                rows,
            )),
            ResultSummary::new("Translated pattern search completed")
                .with_line(format!("Input: {}", outcome.input.path.display()))
                .with_line(format!("Pattern: {}", outcome.pattern))
                .with_line("Frame policy: forward frames 1-3")
                .with_line("Coordinate convention: 1-based inclusive")
                .with_line(format!("Hits: {}", outcome.hits.len())),
            report.clone(),
        );

        Ok(InvocationResponse::completed(
            request.context,
            request.tool,
            descriptor,
            report,
            result,
        ))
    }

    fn invoke_charge(
        &self,
        request: InvocationRequest,
        descriptor: ToolDescriptor,
    ) -> Result<InvocationResponse, ServiceError> {
        if help_requested(request.arguments()) {
            return Ok(self.help_response(request, descriptor, charge_help()));
        }

        let (params, plot_contract_out) = parse_charge_params(request.arguments())?;
        let (input, input_provenance, input_diagnostics) =
            self.resolve_local_sequence_input(&params.input.path.display().to_string())?;
        let outcome = run_charge(ChargeParams {
            input,
            window: params.window,
            step: params.step,
        })?;
        let status_message = format!(
            "computed charge profile across {} windows",
            outcome.profile.windows.len()
        );

        let mut provenance = vec![input_provenance];
        let mut result = MethodResult::new(
            request.tool.clone(),
            ResultPayload::TableReport(TableReport::new(
                vec![
                    "sequence_id".to_owned(),
                    "window_start".to_owned(),
                    "window_end".to_owned(),
                    "window_length".to_owned(),
                    "mean_charge".to_owned(),
                ],
                outcome
                    .profile
                    .windows
                    .iter()
                    .map(|window| {
                        vec![
                            outcome.profile.identifier.clone(),
                            window.window_start.to_string(),
                            window.window_end.to_string(),
                            window.window_length.to_string(),
                            format!("{:.6}", window.mean_charge),
                        ]
                    })
                    .collect(),
            )),
            ResultSummary::new("Protein charge profile computed")
                .with_line(format!("Input: {}", outcome.input.path.display()))
                .with_line(format!("Sequence: {}", outcome.profile.identifier))
                .with_line(format!("Window: {}", outcome.profile.window))
                .with_line(format!("Step: {}", outcome.profile.step))
                .with_line("X axis: 1-based window start")
                .with_line("Charge model: D/E=-1.0, K/R=+1.0, H=+0.5, others=0.0")
                .with_line(format!(
                    "Plot contract: {}",
                    plot_contract_out
                        .as_ref()
                        .map(|path| path.display().to_string())
                        .unwrap_or_else(|| "attached in method result only".to_owned())
                )),
            self.success_report(
                &request.context,
                status_message.clone(),
                input_diagnostics.clone(),
                Vec::new(),
            ),
        )
        .with_plot(outcome.plot.clone());

        if let Some(path) = &plot_contract_out {
            let json = outcome.plot.to_json_pretty().map_err(|error| {
                PlatformError::new(
                    ErrorCategory::Validation,
                    format!("failed to serialize charge plot contract: {error}"),
                )
                .with_code("service.charge.plot.serialize_failed")
            })?;
            std::fs::write(path, json).map_err(|error| {
                PlatformError::new(
                    ErrorCategory::Configuration,
                    format!("failed to write charge plot contract to {}", path.display()),
                )
                .with_code("service.charge.plot.write_failed")
                .with_detail(error.to_string())
            })?;

            let plot_provenance = ArtifactProvenance::generated_output(path.display().to_string())
                .with_description("charge plot contract");
            provenance.push(plot_provenance.clone());
            result = result.with_artifact(
                ArtifactReference::new("charge-plot-contract", ArtifactKind::Auxiliary)
                    .with_label("Charge plot contract")
                    .with_local_path(path)
                    .with_provenance(plot_provenance),
            );
        }

        let report = self.success_report(
            &request.context,
            status_message,
            input_diagnostics,
            provenance,
        );
        result.report = report.clone();

        Ok(InvocationResponse::completed(
            request.context,
            request.tool,
            descriptor,
            report,
            result,
        ))
    }

    fn invoke_compseq(
        &self,
        request: InvocationRequest,
        descriptor: ToolDescriptor,
    ) -> Result<InvocationResponse, ServiceError> {
        if help_requested(request.arguments()) {
            return Ok(self.help_response(request, descriptor, compseq_help()));
        }

        let [input]: [String; 1] = request
            .arguments
            .clone()
            .try_into()
            .map_err(|_| tool_usage_error("compseq", compseq_help()))?;
        let (input, input_provenance, input_diagnostics) =
            self.resolve_local_sequence_input(&input)?;
        let outcome = run_compseq(CompseqParams { input })?;

        let mut rows = Vec::new();
        for record in &outcome.records {
            for (residue, count) in record.composition.counts() {
                rows.push(vec![
                    "record".to_owned(),
                    record.record_id.clone(),
                    record.molecule.as_str().to_owned(),
                    record.sequence_length.to_string(),
                    residue.to_string(),
                    count.to_string(),
                    format!("{:.4}", record.composition.frequency_for(*residue)),
                ]);
            }
        }
        for (residue, count) in outcome.aggregate.counts() {
            rows.push(vec![
                "aggregate".to_owned(),
                "ALL".to_owned(),
                "mixed".to_owned(),
                outcome.aggregate.counted_symbols().to_string(),
                residue.to_string(),
                count.to_string(),
                format!("{:.4}", outcome.aggregate.frequency_for(*residue)),
            ]);
        }

        let report = self.success_report(
            &request.context,
            format!("reported composition for {} records", outcome.records.len()),
            input_diagnostics,
            vec![input_provenance],
        );
        let result = MethodResult::new(
            request.tool.clone(),
            ResultPayload::TableReport(TableReport::new(
                vec![
                    "scope".to_owned(),
                    "record".to_owned(),
                    "molecule".to_owned(),
                    "length".to_owned(),
                    "residue".to_owned(),
                    "count".to_owned(),
                    "frequency".to_owned(),
                ],
                rows,
            )),
            ResultSummary::new("Sequence composition reported")
                .with_line(format!("Input: {}", outcome.input.path.display()))
                .with_line("Scope: per-record plus aggregate summary")
                .with_line("Gap policy: '-' is ignored")
                .with_line("Frequency denominator: all non-gap normalized residue symbols")
                .with_line(format!("Records: {}", outcome.records.len())),
            report.clone(),
        );

        Ok(InvocationResponse::completed(
            request.context,
            request.tool,
            descriptor,
            report,
            result,
        ))
    }

    fn invoke_chips(
        &self,
        request: InvocationRequest,
        descriptor: ToolDescriptor,
    ) -> Result<InvocationResponse, ServiceError> {
        if help_requested(request.arguments()) {
            return Ok(self.help_response(request, descriptor, chips_help()));
        }

        let [input]: [String; 1] = request
            .arguments
            .clone()
            .try_into()
            .map_err(|_| tool_usage_error("chips", chips_help()))?;
        let (input, input_provenance, input_diagnostics) =
            self.resolve_local_sequence_input(&input)?;
        let outcome = run_chips(ChipsParams { input })?;

        let mut rows = Vec::new();
        for record in &outcome.records {
            for codon in emboss_core::sense_codons() {
                let count = record.profile.count_for(codon);
                if count == 0 {
                    continue;
                }
                rows.push(vec![
                    "record".to_owned(),
                    record.record_id.clone(),
                    codon.to_owned(),
                    emboss_core::amino_acid_for_sense_codon(codon)
                        .expect("sense codon should have amino acid")
                        .to_string(),
                    count.to_string(),
                    format!("{:.6}", record.profile.frequency_for(codon)),
                    record
                        .terminal_stop
                        .clone()
                        .unwrap_or_else(|| "-".to_owned()),
                ]);
            }
        }
        for codon in emboss_core::sense_codons() {
            let count = outcome.aggregate.count_for(codon);
            if count == 0 {
                continue;
            }
            rows.push(vec![
                "aggregate".to_owned(),
                "ALL".to_owned(),
                codon.to_owned(),
                emboss_core::amino_acid_for_sense_codon(codon)
                    .expect("sense codon should have amino acid")
                    .to_string(),
                count.to_string(),
                format!("{:.6}", outcome.aggregate.frequency_for(codon)),
                "-".to_owned(),
            ]);
        }

        let report = self.success_report(
            &request.context,
            format!("reported codon usage for {} records", outcome.records.len()),
            input_diagnostics,
            vec![input_provenance],
        );
        let result = MethodResult::new(
            request.tool.clone(),
            ResultPayload::TableReport(TableReport::new(
                vec![
                    "scope".to_owned(),
                    "record".to_owned(),
                    "codon".to_owned(),
                    "amino_acid".to_owned(),
                    "count".to_owned(),
                    "frequency".to_owned(),
                    "terminal_stop".to_owned(),
                ],
                rows,
            )),
            ResultSummary::new("Codon usage reported")
                .with_line(format!("Input: {}", outcome.input.path.display()))
                .with_line("Scope: per-record plus aggregate summary")
                .with_line("Coding policy: strict in-frame coding sequences only")
                .with_line("Stop policy: one terminal stop allowed and excluded from profile")
                .with_line(format!("Records: {}", outcome.records.len())),
            report.clone(),
        );

        Ok(InvocationResponse::completed(
            request.context,
            request.tool,
            descriptor,
            report,
            result,
        ))
    }

    fn invoke_codcopy(
        &self,
        request: InvocationRequest,
        descriptor: ToolDescriptor,
    ) -> Result<InvocationResponse, ServiceError> {
        if help_requested(request.arguments()) {
            return Ok(self.help_response(request, descriptor, codcopy_help()));
        }

        let params = parse_codcopy_params(request.arguments())?;
        let (source, source_provenance, source_diagnostics) =
            self.resolve_local_file_input(&params.source.display().to_string())?;
        let outcome = run_codcopy(CodcopyParams {
            source,
            profile_out: params.profile_out.clone(),
        })?;

        let mut provenance = vec![source_provenance];
        let mut result = MethodResult::new(
            request.tool.clone(),
            ResultPayload::TableReport(TableReport::new(
                vec![
                    "codon".to_owned(),
                    "amino_acid".to_owned(),
                    "count".to_owned(),
                    "frequency".to_owned(),
                    "weight".to_owned(),
                ],
                render_profile_rows(&outcome.profile),
            )),
            ResultSummary::new("Codon profile normalized")
                .with_line(format!("Source: {}", outcome.source.display()))
                .with_line("Profile format: tab-separated codon/amino_acid/count/frequency/weight")
                .with_line("Weight convention: count / max synonymous count in reference")
                .with_line(format!(
                    "Profile output: {}",
                    outcome
                        .profile_out
                        .as_ref()
                        .map(|path| path.display().to_string())
                        .unwrap_or_else(|| "stdout only".to_owned())
                )),
            self.success_report(
                &request.context,
                "normalized codon profile".to_owned(),
                source_diagnostics,
                Vec::new(),
            ),
        );

        if let Some(path) = &outcome.profile_out {
            let profile_provenance =
                ArtifactProvenance::generated_output(path.display().to_string())
                    .with_description("normalized codon profile");
            provenance.push(profile_provenance.clone());
            result = result.with_artifact(
                ArtifactReference::new("codon-profile", ArtifactKind::Table)
                    .with_label("Codon profile")
                    .with_local_path(path)
                    .with_provenance(profile_provenance),
            );
        }

        let report = self.success_report(
            &request.context,
            "normalized codon profile".to_owned(),
            Vec::new(),
            provenance,
        );
        result.report = report.clone();

        Ok(InvocationResponse::completed(
            request.context,
            request.tool,
            descriptor,
            report,
            result,
        ))
    }

    fn invoke_cai(
        &self,
        request: InvocationRequest,
        descriptor: ToolDescriptor,
    ) -> Result<InvocationResponse, ServiceError> {
        if help_requested(request.arguments()) {
            return Ok(self.help_response(request, descriptor, cai_help()));
        }

        let arguments: [String; 2] = request
            .arguments
            .clone()
            .try_into()
            .map_err(|_| tool_usage_error("cai", cai_help()))?;
        let (input, input_provenance, input_diagnostics) =
            self.resolve_local_sequence_input(&arguments[0])?;
        let (reference, reference_provenance, reference_diagnostics) =
            self.resolve_local_file_input(&arguments[1])?;
        let outcome = run_cai(CaiParams { input, reference })?;

        let mut diagnostics = input_diagnostics;
        diagnostics.extend(reference_diagnostics);
        let report = self.success_report(
            &request.context,
            format!("reported CAI for {} records", outcome.cases.len()),
            diagnostics,
            vec![input_provenance, reference_provenance],
        );
        let rows = outcome
            .cases
            .iter()
            .map(|case| {
                vec![
                    case.record_id.clone(),
                    case.sense_codon_count.to_string(),
                    case.terminal_stop.clone().unwrap_or_else(|| "-".to_owned()),
                    format!("{:.6}", case.cai),
                ]
            })
            .collect();
        let result = MethodResult::new(
            request.tool.clone(),
            ResultPayload::TableReport(TableReport::new(
                vec![
                    "record".to_owned(),
                    "sense_codons".to_owned(),
                    "terminal_stop".to_owned(),
                    "cai".to_owned(),
                ],
                rows,
            )),
            ResultSummary::new("CAI reported")
                .with_line(format!("Input: {}", outcome.input.path.display()))
                .with_line(format!("Reference: {}", outcome.reference.display()))
                .with_line("CAI convention: geometric mean of codon weights")
                .with_line("Weight convention: codon_count / max_synonymous_count")
                .with_line("Zero-weight codons yield CAI 0.0"),
            report.clone(),
        );

        Ok(InvocationResponse::completed(
            request.context,
            request.tool,
            descriptor,
            report,
            result,
        ))
    }

    fn invoke_codcmp(
        &self,
        request: InvocationRequest,
        descriptor: ToolDescriptor,
    ) -> Result<InvocationResponse, ServiceError> {
        if help_requested(request.arguments()) {
            return Ok(self.help_response(request, descriptor, codcmp_help()));
        }

        let arguments: [String; 2] = request
            .arguments
            .clone()
            .try_into()
            .map_err(|_| tool_usage_error("codcmp", codcmp_help()))?;
        let (left, left_provenance, left_diagnostics) =
            self.resolve_local_file_input(&arguments[0])?;
        let (right, right_provenance, right_diagnostics) =
            self.resolve_local_file_input(&arguments[1])?;
        let outcome = run_codcmp(CodcmpParams { left, right })?;

        let mut diagnostics = left_diagnostics;
        diagnostics.extend(right_diagnostics);
        let report = self.success_report(
            &request.context,
            "reported codon-usage comparison".to_owned(),
            diagnostics,
            vec![left_provenance, right_provenance],
        );
        let rows = outcome
            .rows
            .iter()
            .map(|row| {
                vec![
                    row.codon.clone(),
                    row.amino_acid.to_string(),
                    row.left_count.to_string(),
                    format!("{:.6}", row.left_frequency),
                    row.right_count.to_string(),
                    format!("{:.6}", row.right_frequency),
                    format!("{:.6}", row.delta_frequency),
                ]
            })
            .collect();
        let result = MethodResult::new(
            request.tool.clone(),
            ResultPayload::TableReport(TableReport::new(
                vec![
                    "codon".to_owned(),
                    "amino_acid".to_owned(),
                    "left_count".to_owned(),
                    "left_frequency".to_owned(),
                    "right_count".to_owned(),
                    "right_frequency".to_owned(),
                    "delta_frequency".to_owned(),
                ],
                rows,
            )),
            ResultSummary::new("Codon-usage comparison reported")
                .with_line(format!("Left: {}", outcome.left.display()))
                .with_line(format!("Right: {}", outcome.right.display()))
                .with_line(format!(
                    "Total variation distance: {:.6}",
                    outcome.total_variation_distance
                ))
                .with_line("Comparison scope: 61 sense codons"),
            report.clone(),
        );

        Ok(InvocationResponse::completed(
            request.context,
            request.tool,
            descriptor,
            report,
            result,
        ))
    }

    fn invoke_complex(
        &self,
        request: InvocationRequest,
        descriptor: ToolDescriptor,
    ) -> Result<InvocationResponse, ServiceError> {
        if help_requested(request.arguments()) {
            return Ok(self.help_response(request, descriptor, complex_help()));
        }

        let params = parse_complex_params(request.arguments())?;
        let (input, input_provenance, input_diagnostics) =
            self.resolve_local_sequence_input(&params.input.path.display().to_string())?;
        let outcome = run_complex(ComplexParams {
            input,
            k_min: params.k_min,
            k_max: params.k_max,
            window: params.window,
            step: params.step,
        })?;

        let mut rows = Vec::new();
        for record in &outcome.sequences {
            rows.push(vec![
                "record".to_owned(),
                record.record_id.clone(),
                record.sequence_length.to_string(),
                record.k_min.to_string(),
                record.k_max.to_string(),
                String::new(),
                String::new(),
                String::new(),
                format!("{:.6}", record.complexity),
            ]);
        }
        for window in &outcome.windows {
            rows.push(vec![
                "window".to_owned(),
                window.record_id.clone(),
                String::new(),
                window.k_min.to_string(),
                window.k_max.to_string(),
                (window.start + 1).to_string(),
                window.end.to_string(),
                window.window_length.to_string(),
                format!("{:.6}", window.complexity),
            ]);
        }

        let report = self.success_report(
            &request.context,
            format!(
                "reported linguistic complexity for {} records{}",
                outcome.sequences.len(),
                if outcome.windows.is_empty() {
                    String::new()
                } else {
                    format!(" and {} windows", outcome.windows.len())
                }
            ),
            input_diagnostics,
            vec![input_provenance],
        );
        let result = MethodResult::new(
            request.tool.clone(),
            ResultPayload::TableReport(TableReport::new(
                vec![
                    "scope".to_owned(),
                    "record".to_owned(),
                    "sequence_length".to_owned(),
                    "k_min".to_owned(),
                    "k_max".to_owned(),
                    "window_start".to_owned(),
                    "window_end".to_owned(),
                    "window_length".to_owned(),
                    "complexity".to_owned(),
                ],
                rows,
            )),
            ResultSummary::new("Linguistic complexity reported")
                .with_line(format!("Input: {}", outcome.input.path.display()))
                .with_line("Definition: sum(observed distinct k-mers) / sum(min(4^k, L-k+1))")
                .with_line("Alphabet policy: canonical A/C/G/T only")
                .with_line(format!("k-range: {}..={}", outcome.k_min, outcome.k_max))
                .with_line(format!(
                    "Sliding windows: {}",
                    match (outcome.window, outcome.step) {
                        (Some(window), Some(step)) => format!("window={window} step={step}"),
                        _ => "disabled".to_owned(),
                    }
                ))
                .with_line(format!("Record summaries: {}", outcome.sequences.len()))
                .with_line(format!("Window rows: {}", outcome.windows.len())),
            report.clone(),
        );

        Ok(InvocationResponse::completed(
            request.context,
            request.tool,
            descriptor,
            report,
            result,
        ))
    }

    fn invoke_geecee(
        &self,
        request: InvocationRequest,
        descriptor: ToolDescriptor,
    ) -> Result<InvocationResponse, ServiceError> {
        if help_requested(request.arguments()) {
            return Ok(self.help_response(request, descriptor, geecee_help()));
        }

        let [input]: [String; 1] = request
            .arguments
            .clone()
            .try_into()
            .map_err(|_| tool_usage_error("geecee", geecee_help()))?;
        let (input, input_provenance, input_diagnostics) =
            self.resolve_local_sequence_input(&input)?;
        let outcome = run_geecee(GeeceeParams { input })?;

        let mut rows = Vec::new();
        for record in &outcome.records {
            rows.push(vec![
                "record".to_owned(),
                record.record_id.clone(),
                record.sequence_length.to_string(),
                record.gc.gc_symbols.to_string(),
                record.gc.canonical_symbols.to_string(),
                record.gc.ambiguous_symbols.to_string(),
                format!("{:.2}", record.gc.gc_percent()),
            ]);
        }
        rows.push(vec![
            "aggregate".to_owned(),
            "ALL".to_owned(),
            outcome.aggregate.counted_symbols.to_string(),
            outcome.aggregate.gc_symbols.to_string(),
            outcome.aggregate.canonical_symbols.to_string(),
            outcome.aggregate.ambiguous_symbols.to_string(),
            format!("{:.2}", outcome.aggregate.gc_percent()),
        ]);

        let report = self.success_report(
            &request.context,
            format!(
                "reported GC statistics for {} records",
                outcome.records.len()
            ),
            input_diagnostics,
            vec![input_provenance],
        );
        let result = MethodResult::new(
            request.tool.clone(),
            ResultPayload::TableReport(TableReport::new(
                vec![
                    "scope".to_owned(),
                    "record".to_owned(),
                    "length".to_owned(),
                    "gc_count".to_owned(),
                    "gc_denominator".to_owned(),
                    "ambiguous_count".to_owned(),
                    "gc_percent".to_owned(),
                ],
                rows,
            )),
            ResultSummary::new("GC statistics reported")
                .with_line(format!("Input: {}", outcome.input.path.display()))
                .with_line("Scope: per-record plus aggregate summary")
                .with_line("GC denominator: canonical A/C/G/T/U symbols only")
                .with_line("Ambiguous symbols are excluded from GC percentage")
                .with_line(format!("Records: {}", outcome.records.len())),
            report.clone(),
        );

        Ok(InvocationResponse::completed(
            request.context,
            request.tool,
            descriptor,
            report,
            result,
        ))
    }

    fn invoke_pepstats(
        &self,
        request: InvocationRequest,
        descriptor: ToolDescriptor,
    ) -> Result<InvocationResponse, ServiceError> {
        if help_requested(request.arguments()) {
            return Ok(self.help_response(request, descriptor, pepstats_help()));
        }

        let [input]: [String; 1] = request
            .arguments
            .clone()
            .try_into()
            .map_err(|_| tool_usage_error("pepstats", pepstats_help()))?;
        let (input, input_provenance, input_diagnostics) =
            self.resolve_local_sequence_input(&input)?;
        let outcome = run_pepstats(PepstatsParams { input })?;

        let mut rows = Vec::new();
        for record in &outcome.records {
            rows.push(vec![
                "summary".to_owned(),
                record.record_id.clone(),
                "sequence_length".to_owned(),
                record.sequence_length.to_string(),
                String::new(),
                String::new(),
            ]);
            rows.push(vec![
                "summary".to_owned(),
                record.record_id.clone(),
                "residue_length".to_owned(),
                record.residue_length.to_string(),
                String::new(),
                String::new(),
            ]);
            rows.push(vec![
                "summary".to_owned(),
                record.record_id.clone(),
                "stop_count".to_owned(),
                record.stop_count.to_string(),
                String::new(),
                String::new(),
            ]);
            rows.push(vec![
                "summary".to_owned(),
                record.record_id.clone(),
                "molecular_weight".to_owned(),
                format!("{:.3}", record.molecular_weight),
                String::new(),
                String::new(),
            ]);
            for (residue, count) in record.composition.counts() {
                rows.push(vec![
                    "composition".to_owned(),
                    record.record_id.clone(),
                    residue.to_string(),
                    count.to_string(),
                    format!("{:.4}", record.composition.frequency_for(*residue)),
                    String::new(),
                ]);
            }
        }

        let report = self.success_report(
            &request.context,
            format!(
                "reported protein statistics for {} records",
                outcome.records.len()
            ),
            input_diagnostics,
            vec![input_provenance],
        );
        let result = MethodResult::new(
            request.tool.clone(),
            ResultPayload::TableReport(TableReport::new(
                vec![
                    "section".to_owned(),
                    "record".to_owned(),
                    "metric_or_residue".to_owned(),
                    "value_or_count".to_owned(),
                    "frequency".to_owned(),
                    "notes".to_owned(),
                ],
                rows,
            )),
            ResultSummary::new("Protein statistics reported")
                .with_line(format!("Input: {}", outcome.input.path.display()))
                .with_line("Mass convention: average residue masses plus one water molecule")
                .with_line("Stop symbols are excluded from residue_length and mass")
                .with_line("pI estimation: deferred in v1")
                .with_line(format!("Records: {}", outcome.records.len())),
            report.clone(),
        );

        Ok(InvocationResponse::completed(
            request.context,
            request.tool,
            descriptor,
            report,
            result,
        ))
    }

    fn invoke_backtranambig(
        &self,
        request: InvocationRequest,
        descriptor: ToolDescriptor,
    ) -> Result<InvocationResponse, ServiceError> {
        if help_requested(request.arguments()) {
            return Ok(self.help_response(request, descriptor, backtranambig_help()));
        }

        let [input]: [String; 1] = request
            .arguments
            .clone()
            .try_into()
            .map_err(|_| tool_usage_error("backtranambig", backtranambig_help()))?;
        let (input, input_provenance, input_diagnostics) =
            self.resolve_local_sequence_input(&input)?;
        let outcome = run_backtranambig(BacktranambigParams { input })?;

        let output_provenance = ArtifactProvenance::generated_output("stdout")
            .with_description("ambiguous back-translated FASTA output");
        let report = self.success_report(
            &request.context,
            format!(
                "ambiguously back-translated {} protein records",
                outcome.records.len()
            ),
            input_diagnostics,
            vec![input_provenance, output_provenance.clone()],
        );
        let result = MethodResult::new(
            request.tool.clone(),
            ResultPayload::SequenceCollection(outcome.records),
            ResultSummary::new("Ambiguous back-translation completed")
                .with_line(format!("Input: {}", outcome.input.path.display()))
                .with_line("Genetic code: standard")
                .with_line("Codon policy: deterministic IUPAC-ambiguous DNA codons")
                .with_line("Stop symbol handling: '*' -> TAR")
                .with_line("Output format: fasta"),
            report.clone(),
        )
        .with_artifact(
            ArtifactReference::new("ambiguous-backtranslated-sequences", ArtifactKind::Sequence)
                .with_label("Ambiguous back-translated sequences")
                .with_provenance(output_provenance),
        );

        Ok(InvocationResponse::completed(
            request.context,
            request.tool,
            descriptor,
            report,
            result,
        ))
    }

    fn invoke_checktrans(
        &self,
        request: InvocationRequest,
        descriptor: ToolDescriptor,
    ) -> Result<InvocationResponse, ServiceError> {
        if help_requested(request.arguments()) {
            return Ok(self.help_response(request, descriptor, checktrans_help()));
        }

        let arguments: [String; 2] = request
            .arguments
            .clone()
            .try_into()
            .map_err(|_| tool_usage_error("checktrans", checktrans_help()))?;
        let (nucleotide_input, nucleotide_provenance, mut diagnostics) =
            self.resolve_local_sequence_input(&arguments[0])?;
        let (protein_input, protein_provenance, protein_diagnostics) =
            self.resolve_local_sequence_input(&arguments[1])?;
        diagnostics.extend(protein_diagnostics);
        let outcome = run_checktrans(ChecktransParams {
            nucleotide_input,
            protein_input,
        })?;

        let match_count = outcome.cases.iter().filter(|case| case.matches).count();
        let output_provenance = ArtifactProvenance::generated_output("stdout")
            .with_description("translation-check tabular report");
        let report = self.success_report(
            &request.context,
            format!(
                "checked {} nucleotide/protein record pairs",
                outcome.cases.len()
            ),
            diagnostics,
            vec![
                nucleotide_provenance,
                protein_provenance,
                output_provenance.clone(),
            ],
        );
        let rows = outcome
            .cases
            .iter()
            .map(|case| {
                vec![
                    case.nucleotide_id.clone(),
                    case.protein_id.clone(),
                    case.matches.to_string(),
                    case.translated_terminal_stop.to_string(),
                    case.expected_terminal_stop.to_string(),
                    case.detail.clone(),
                ]
            })
            .collect();
        let result = MethodResult::new(
            request.tool.clone(),
            ResultPayload::TableReport(TableReport::new(
                vec![
                    "nucleotide_id".to_owned(),
                    "protein_id".to_owned(),
                    "matches".to_owned(),
                    "translated_terminal_stop".to_owned(),
                    "expected_terminal_stop".to_owned(),
                    "detail".to_owned(),
                ],
                rows,
            )),
            ResultSummary::new("Translation check completed")
                .with_line(format!(
                    "Nucleotide input: {}",
                    outcome.nucleotide_input.path.display()
                ))
                .with_line(format!(
                    "Protein input: {}",
                    outcome.protein_input.path.display()
                ))
                .with_line("Genetic code: standard")
                .with_line("Frame assumption: strict frame 1 only")
                .with_line("Pairing rule: equal record counts paired by input order")
                .with_line(format!(
                    "Matching pairs: {match_count}/{}",
                    outcome.cases.len()
                )),
            report.clone(),
        )
        .with_artifact(
            ArtifactReference::new("translation-check-report", ArtifactKind::Table)
                .with_label("Translation check report")
                .with_provenance(output_provenance),
        );

        Ok(InvocationResponse::completed(
            request.context,
            request.tool,
            descriptor,
            report,
            result,
        ))
    }

    fn invoke_cutseq(
        &self,
        request: InvocationRequest,
        descriptor: ToolDescriptor,
    ) -> Result<InvocationResponse, ServiceError> {
        if help_requested(request.arguments()) {
            return Ok(self.help_response(request, descriptor, cutseq_help()));
        }

        let arguments: [String; 2] = request
            .arguments
            .clone()
            .try_into()
            .map_err(|_| tool_usage_error("cutseq", cutseq_help()))?;
        let (input, input_provenance, input_diagnostics) =
            self.resolve_local_sequence_input(&arguments[0])?;
        let cut_position = parse_positive_index("cutseq", &arguments[1])?;
        let outcome = run_cutseq(CutseqParams {
            input,
            cut_position,
        })?;

        let output_provenance =
            ArtifactProvenance::generated_output("stdout").with_description("cut FASTA output");
        let report = self.success_report(
            &request.context,
            format!("cut sequences at position {}", outcome.cut_position),
            input_diagnostics,
            vec![input_provenance, output_provenance.clone()],
        );
        let result = MethodResult::new(
            request.tool.clone(),
            ResultPayload::SequenceCollection(outcome.records),
            ResultSummary::new("Sequence cut completed")
                .with_line(format!("Input: {}", outcome.input.path.display()))
                .with_line(format!("Cut position: {}", outcome.cut_position))
                .with_line("Coordinate convention: 1-based cut after position")
                .with_line("Output format: fasta"),
            report.clone(),
        )
        .with_artifact(
            ArtifactReference::new("cut-sequences", ArtifactKind::Sequence)
                .with_label("Cut fragments")
                .with_provenance(output_provenance),
        );

        Ok(InvocationResponse::completed(
            request.context,
            request.tool,
            descriptor,
            report,
            result,
        ))
    }

    fn invoke_union(
        &self,
        request: InvocationRequest,
        descriptor: ToolDescriptor,
    ) -> Result<InvocationResponse, ServiceError> {
        if help_requested(request.arguments()) {
            return Ok(self.help_response(request, descriptor, union_help()));
        }

        if request.arguments.len() < 2 {
            return Err(tool_usage_error("union", union_help()));
        }

        let (inputs, input_provenance, input_diagnostics) =
            self.resolve_multiple_local_sequence_inputs(request.arguments())?;
        let input_count = inputs.len();
        let outcome = run_union(UnionParams { inputs })?;
        let output_count = outcome.records.len();
        let output_provenance =
            ArtifactProvenance::generated_output("stdout").with_description("union FASTA output");
        let mut provenance = input_provenance;
        provenance.push(output_provenance.clone());
        let report = self.success_report(
            &request.context,
            format!("concatenated {} sequence inputs", input_count),
            input_diagnostics,
            provenance,
        );
        let result = MethodResult::new(
            request.tool.clone(),
            ResultPayload::SequenceCollection(outcome.records),
            ResultSummary::new("Sequence union completed")
                .with_line(format!("Inputs: {}", input_count))
                .with_line(format!("Output records: {}", output_count))
                .with_line("Ordering: preserve input order and per-input record order")
                .with_line("Duplicate policy: preserve duplicates exactly as read")
                .with_line("Output format: fasta"),
            report.clone(),
        )
        .with_artifact(
            ArtifactReference::new("union-sequences", ArtifactKind::Sequence)
                .with_label("Union sequence stream")
                .with_provenance(output_provenance),
        );

        Ok(InvocationResponse::completed(
            request.context,
            request.tool,
            descriptor,
            report,
            result,
        ))
    }

    fn invoke_splitter(
        &self,
        request: InvocationRequest,
        descriptor: ToolDescriptor,
    ) -> Result<InvocationResponse, ServiceError> {
        if help_requested(request.arguments()) {
            return Ok(self.help_response(request, descriptor, splitter_help()));
        }

        let arguments: [String; 2] = request
            .arguments
            .clone()
            .try_into()
            .map_err(|_| tool_usage_error("splitter", splitter_help()))?;
        let (input, input_provenance, input_diagnostics) =
            self.resolve_local_sequence_input(&arguments[0])?;
        let chunk_size = parse_non_negative_count("splitter", &arguments[1])?;
        let outcome = run_splitter(SplitterParams { input, chunk_size })?;

        let output_provenance = ArtifactProvenance::generated_output("stdout")
            .with_description("partitioned FASTA output");
        let report = self.success_report(
            &request.context,
            format!(
                "partitioned sequence stream into {} chunks",
                outcome.partitions.len()
            ),
            input_diagnostics,
            vec![input_provenance, output_provenance.clone()],
        );
        let result = MethodResult::new(
            request.tool.clone(),
            ResultPayload::SequencePartitions(outcome.partitions),
            ResultSummary::new("Sequence partitions created")
                .with_line(format!("Input: {}", outcome.input.path.display()))
                .with_line(format!("Chunk size: {}", outcome.chunk_size))
                .with_line("Partition rule: fixed-size record chunks")
                .with_line("Output format: fasta partitions"),
            report.clone(),
        )
        .with_artifact(
            ArtifactReference::new("sequence-partitions", ArtifactKind::Sequence)
                .with_label("Partitioned sequence stream")
                .with_provenance(output_provenance),
        );

        Ok(InvocationResponse::completed(
            request.context,
            request.tool,
            descriptor,
            report,
            result,
        ))
    }

    fn help_response(
        &self,
        request: InvocationRequest,
        descriptor: ToolDescriptor,
        help: &str,
    ) -> InvocationResponse {
        let report = ExecutionReport::from_context(
            &request.context,
            PLATFORM_IDENTITY.binary_name,
            env!("CARGO_PKG_VERSION"),
            ExecutionOutcome::new(OutcomeStatus::Succeeded)
                .with_summary(format!("rendered help for '{}'", descriptor.name)),
        );
        let result = MethodResult::new(
            request.tool.clone(),
            ResultPayload::TextReport(TextReport::new(help).with_title(descriptor.name)),
            ResultSummary::new(format!("{} help", descriptor.name)).with_line(descriptor.summary),
            report.clone(),
        );

        InvocationResponse::completed(request.context, request.tool, descriptor, report, result)
    }

    fn success_report(
        &self,
        context: &ExecutionContext,
        summary: impl Into<String>,
        diagnostics: Vec<Diagnostic>,
        provenance: Vec<ArtifactProvenance>,
    ) -> ExecutionReport {
        let mut report = ExecutionReport::from_context(
            context,
            PLATFORM_IDENTITY.binary_name,
            env!("CARGO_PKG_VERSION"),
            ExecutionOutcome::new(OutcomeStatus::Succeeded).with_summary(summary),
        );

        for diagnostic in diagnostics {
            report.push_diagnostic(diagnostic);
        }
        for entry in provenance {
            report.push_provenance(entry);
        }

        report
    }

    fn resolve_seqret_records_with_client<C: ProviderHttpClient>(
        &self,
        raw: &str,
        client: Option<&C>,
    ) -> Result<
        (
            SeqretSource,
            Vec<emboss_core::SequenceRecord>,
            Vec<ArtifactProvenance>,
            Vec<Diagnostic>,
        ),
        ServiceError,
    > {
        let reference = self.classify_input(raw.to_owned())?;
        match self.resolve_input(reference, emboss_providers::ResolutionIntent::SequenceInput)? {
            ToolInputResolution::LocalFile {
                canonical_path,
                provenance,
                diagnostics,
                ..
            } => {
                let input = SequenceInput::new(canonical_path.clone());
                let records = load_sequence_records(&input)?;
                Ok((
                    SeqretSource::LocalPath(canonical_path),
                    records,
                    vec![provenance],
                    diagnostics,
                ))
            }
            ToolInputResolution::ProviderRouted {
                request,
                provenance,
                diagnostics,
                ..
            } => {
                let retrieved =
                    self.retrieve_routed_sequence_request_with_client(&request, client)?;
                Ok((
                    SeqretSource::Retrieved {
                        provider: retrieved.provider.as_str().to_owned(),
                        accession: retrieved.requested_accession.clone(),
                    },
                    vec![retrieved.record.clone()],
                    vec![provenance, retrieved.provenance.clone()],
                    diagnostics,
                ))
            }
            ToolInputResolution::InlineSequence { .. } => Err(PlatformError::new(
                ErrorCategory::Validation,
                "seqret does not accept inline sequence literals in v1",
            )
            .with_code("service.seqret.inline_not_supported")),
            ToolInputResolution::Unresolved {
                reference,
                diagnostics,
            } => Err(PlatformError::new(
                ErrorCategory::Validation,
                format!("could not resolve seqret input '{}'", reference.raw()),
            )
            .with_code("service.seqret.input.unresolved")
            .with_detail(
                diagnostics
                    .iter()
                    .map(|diagnostic| diagnostic.message().to_owned())
                    .collect::<Vec<_>>()
                    .join("; "),
            )),
        }
    }

    fn resolve_refseqget_record_with_client<C: ProviderHttpClient>(
        &self,
        raw: &str,
        client: Option<&C>,
    ) -> Result<(ArtifactProvenance, RetrievedSequence, Vec<Diagnostic>), ServiceError> {
        let reference = self.classify_input(raw.to_owned())?;
        match self.resolve_input(reference, emboss_providers::ResolutionIntent::SequenceInput)? {
            ToolInputResolution::ProviderRouted {
                request,
                provenance,
                diagnostics,
                ..
            } => Ok((
                provenance,
                self.retrieve_routed_sequence_request_with_client(&request, client)?,
                diagnostics,
            )),
            ToolInputResolution::LocalFile { provenance, .. } => Err(PlatformError::new(
                ErrorCategory::Validation,
                "refseqget is restricted to accession-oriented provider retrieval in v1",
            )
            .with_code("service.refseqget.local_input_not_supported")
            .with_detail(provenance.locator().to_owned())),
            ToolInputResolution::InlineSequence { .. } => Err(PlatformError::new(
                ErrorCategory::Validation,
                "refseqget does not accept inline sequence literals",
            )
            .with_code("service.refseqget.inline_not_supported")),
            ToolInputResolution::Unresolved {
                reference,
                diagnostics,
            } => Err(PlatformError::new(
                ErrorCategory::Validation,
                format!("could not resolve refseqget input '{}'", reference.raw()),
            )
            .with_code("service.refseqget.input.unresolved")
            .with_detail(
                diagnostics
                    .iter()
                    .map(|diagnostic| diagnostic.message().to_owned())
                    .collect::<Vec<_>>()
                    .join("; "),
            )),
        }
    }

    fn retrieve_routed_sequence_request_with_client<C: ProviderHttpClient>(
        &self,
        request: &AcquisitionRequest,
        client: Option<&C>,
    ) -> Result<RetrievedSequence, ServiceError> {
        match client {
            Some(client) => {
                ServiceSequenceRetrieval::with_client(&self.config, &self.providers, client)
                    .retrieve_single_sequence(request)
            }
            None => self.sequence_retrieval()?.retrieve_single_sequence(request),
        }
    }

    fn resolve_archive_metadata_with_client<C: ProviderHttpClient>(
        &self,
        raw: &str,
        client: Option<&C>,
    ) -> Result<
        (
            ArtifactProvenance,
            RetrievedArchiveMetadata,
            Vec<Diagnostic>,
        ),
        ServiceError,
    > {
        let reference = self.classify_input(raw.to_owned())?;
        match self.resolve_input(reference, emboss_providers::ResolutionIntent::ArchiveAsset)? {
            ToolInputResolution::ProviderRouted {
                request,
                provenance,
                diagnostics,
                ..
            } => Ok((
                provenance,
                self.retrieve_archive_metadata_request_with_client(&request, client)?,
                diagnostics,
            )),
            ToolInputResolution::LocalFile { provenance, .. } => Err(PlatformError::new(
                ErrorCategory::Validation,
                "runinfo expects archive accession input, not a local file",
            )
            .with_code("service.runinfo.local_input_not_supported")
            .with_detail(provenance.locator().to_owned())),
            ToolInputResolution::InlineSequence { .. } => Err(PlatformError::new(
                ErrorCategory::Validation,
                "runinfo does not accept inline sequence literals",
            )
            .with_code("service.runinfo.inline_not_supported")),
            ToolInputResolution::Unresolved {
                reference,
                diagnostics,
            } => Err(PlatformError::new(
                ErrorCategory::Validation,
                format!("could not resolve runinfo input '{}'", reference.raw()),
            )
            .with_code("service.runinfo.input.unresolved")
            .with_detail(
                diagnostics
                    .iter()
                    .map(|diagnostic| diagnostic.message().to_owned())
                    .collect::<Vec<_>>()
                    .join("; "),
            )),
        }
    }

    fn resolve_run_manifest_with_client<C: ProviderHttpClient>(
        &self,
        raw: &str,
        client: Option<&C>,
    ) -> Result<
        (
            ArtifactProvenance,
            RetrievedArchiveManifest,
            Vec<Diagnostic>,
        ),
        ServiceError,
    > {
        let reference = self.classify_input(raw.to_owned())?;
        match self.resolve_input(reference, emboss_providers::ResolutionIntent::ArchiveAsset)? {
            ToolInputResolution::ProviderRouted {
                request,
                provenance,
                diagnostics,
                ..
            } => {
                let manifest =
                    self.retrieve_archive_manifest_request_with_client(&request, client)?;
                if manifest.object_class != ArchiveObjectClass::Run {
                    return Err(PlatformError::new(
                        ErrorCategory::Validation,
                        "runget currently supports run accessions only",
                    )
                    .with_code("service.runget.unsupported_object_class")
                    .with_detail(manifest.object_class.as_str().to_owned()));
                }
                Ok((provenance, manifest, diagnostics))
            }
            ToolInputResolution::LocalFile { provenance, .. } => Err(PlatformError::new(
                ErrorCategory::Validation,
                "runget expects archive run accessions, not a local file",
            )
            .with_code("service.runget.local_input_not_supported")
            .with_detail(provenance.locator().to_owned())),
            ToolInputResolution::InlineSequence { .. } => Err(PlatformError::new(
                ErrorCategory::Validation,
                "runget does not accept inline sequence literals",
            )
            .with_code("service.runget.inline_not_supported")),
            ToolInputResolution::Unresolved {
                reference,
                diagnostics,
            } => Err(PlatformError::new(
                ErrorCategory::Validation,
                format!("could not resolve runget input '{}'", reference.raw()),
            )
            .with_code("service.runget.input.unresolved")
            .with_detail(
                diagnostics
                    .iter()
                    .map(|diagnostic| diagnostic.message().to_owned())
                    .collect::<Vec<_>>()
                    .join("; "),
            )),
        }
    }

    fn retrieve_archive_metadata_request_with_client<C: ProviderHttpClient>(
        &self,
        request: &AcquisitionRequest,
        client: Option<&C>,
    ) -> Result<RetrievedArchiveMetadata, ServiceError> {
        match client {
            Some(client) => {
                ServiceArchiveRetrieval::with_client(&self.config, &self.providers, client)
                    .lookup_metadata(request)
            }
            None => self.archive_retrieval()?.lookup_metadata(request),
        }
    }

    fn retrieve_archive_manifest_request_with_client<C: ProviderHttpClient>(
        &self,
        request: &AcquisitionRequest,
        client: Option<&C>,
    ) -> Result<RetrievedArchiveManifest, ServiceError> {
        match client {
            Some(client) => {
                ServiceArchiveRetrieval::with_client(&self.config, &self.providers, client)
                    .retrieve_run_manifest(request)
            }
            None => self.archive_retrieval()?.retrieve_run_manifest(request),
        }
    }

    fn resolve_local_sequence_input(
        &self,
        raw: &str,
    ) -> Result<(SequenceInput, ArtifactProvenance, Vec<Diagnostic>), ServiceError> {
        let reference = self.classify_input(raw.to_owned())?;
        match self.resolve_input(reference, emboss_providers::ResolutionIntent::SequenceInput)? {
            ToolInputResolution::LocalFile {
                canonical_path,
                provenance,
                diagnostics,
                ..
            } => Ok((SequenceInput::new(canonical_path), provenance, diagnostics)),
            ToolInputResolution::ProviderRouted { provenance, .. } => Err(PlatformError::new(
                ErrorCategory::NotImplemented,
                "provider-backed sequence acquisition is not implemented for this tool cohort yet",
            )
            .with_code("service.tool.input.provider_not_supported")
            .with_detail(provenance.locator().to_owned())),
            ToolInputResolution::InlineSequence { .. } => Err(PlatformError::new(
                ErrorCategory::NotImplemented,
                "inline sequence literals are not accepted for sequence-stream input files",
            )
            .with_code("service.tool.input.inline_not_supported")),
            ToolInputResolution::Unresolved {
                reference,
                diagnostics,
            } => Err(PlatformError::new(
                ErrorCategory::Validation,
                format!("could not resolve tool input '{}'", reference.raw()),
            )
            .with_code("service.tool.input.unresolved")
            .with_detail(
                diagnostics
                    .iter()
                    .map(|diagnostic| diagnostic.message().to_owned())
                    .collect::<Vec<_>>()
                    .join("; "),
            )),
        }
    }

    fn resolve_local_alignment_input(
        &self,
        raw: &str,
    ) -> Result<(AlignmentInput, ArtifactProvenance, Vec<Diagnostic>), ServiceError> {
        let reference = self.classify_input(raw.to_owned())?;
        match self.resolve_input(reference, emboss_providers::ResolutionIntent::SequenceInput)? {
            ToolInputResolution::LocalFile {
                canonical_path,
                provenance,
                diagnostics,
                ..
            } => Ok((AlignmentInput::new(canonical_path), provenance, diagnostics)),
            ToolInputResolution::ProviderRouted { provenance, .. } => Err(PlatformError::new(
                ErrorCategory::NotImplemented,
                "provider-backed alignment acquisition is not implemented for this tool cohort yet",
            )
            .with_code("service.tool.input.provider_not_supported")
            .with_detail(provenance.locator().to_owned())),
            ToolInputResolution::InlineSequence { .. } => Err(PlatformError::new(
                ErrorCategory::NotImplemented,
                "inline sequence literals are not accepted for alignment input files",
            )
            .with_code("service.tool.input.inline_not_supported")),
            ToolInputResolution::Unresolved {
                reference,
                diagnostics,
            } => Err(PlatformError::new(
                ErrorCategory::Validation,
                format!("could not resolve tool input '{}'", reference.raw()),
            )
            .with_code("service.tool.input.unresolved")
            .with_detail(
                diagnostics
                    .iter()
                    .map(|diagnostic| diagnostic.message().to_owned())
                    .collect::<Vec<_>>()
                    .join("; "),
            )),
        }
    }

    fn resolve_multiple_local_sequence_inputs(
        &self,
        raw_inputs: &[String],
    ) -> Result<(Vec<SequenceInput>, Vec<ArtifactProvenance>, Vec<Diagnostic>), ServiceError> {
        let mut inputs = Vec::new();
        let mut provenance = Vec::new();
        let mut diagnostics = Vec::new();

        for raw in raw_inputs {
            let (input, input_provenance, input_diagnostics) =
                self.resolve_local_sequence_input(raw)?;
            inputs.push(input);
            provenance.push(input_provenance);
            diagnostics.extend(input_diagnostics);
        }

        Ok((inputs, provenance, diagnostics))
    }

    fn resolve_local_file_input(
        &self,
        raw: &str,
    ) -> Result<(PathBuf, ArtifactProvenance, Vec<Diagnostic>), ServiceError> {
        let reference = self.classify_input(raw.to_owned())?;
        match self.resolve_input(reference, emboss_providers::ResolutionIntent::SequenceInput)? {
            ToolInputResolution::LocalFile {
                canonical_path,
                provenance,
                diagnostics,
                ..
            } => Ok((canonical_path, provenance, diagnostics)),
            ToolInputResolution::ProviderRouted { provenance, .. } => Err(PlatformError::new(
                ErrorCategory::NotImplemented,
                "provider-backed file acquisition is not implemented for this tool cohort yet",
            )
            .with_code("service.tool.input.provider_not_supported")
            .with_detail(provenance.locator().to_owned())),
            ToolInputResolution::InlineSequence { .. } => Err(PlatformError::new(
                ErrorCategory::NotImplemented,
                "inline sequence literals are not accepted for file-backed inputs",
            )
            .with_code("service.tool.input.inline_not_supported")),
            ToolInputResolution::Unresolved {
                reference,
                diagnostics,
            } => Err(PlatformError::new(
                ErrorCategory::Validation,
                format!("could not resolve tool input '{}'", reference.raw()),
            )
            .with_code("service.tool.input.unresolved")
            .with_detail(
                diagnostics
                    .iter()
                    .map(|diagnostic| diagnostic.message().to_owned())
                    .collect::<Vec<_>>()
                    .join("; "),
            )),
        }
    }
}

fn help_requested(arguments: &[String]) -> bool {
    arguments
        .iter()
        .any(|argument| argument == "--help" || argument == "-h")
}

fn tool_usage_error(tool: &str, help: &str) -> ServiceError {
    PlatformError::new(
        ErrorCategory::Validation,
        format!("invalid arguments for '{tool}'"),
    )
    .with_code("service.tool.arguments.invalid")
    .with_detail(help)
}

fn parse_positive_index(tool: &str, value: &str) -> Result<usize, ServiceError> {
    let index = value.parse::<usize>().map_err(|_| {
        PlatformError::new(
            ErrorCategory::Validation,
            format!("{tool} expects a positive integer index"),
        )
        .with_code("service.tool.index.parse_failed")
        .with_detail(value.to_owned())
    })?;

    if index == 0 {
        return Err(PlatformError::new(
            ErrorCategory::Validation,
            format!("{tool} uses 1-based indexing and requires index >= 1"),
        )
        .with_code("service.tool.index.zero"));
    }

    Ok(index)
}

fn parse_non_negative_count(tool: &str, value: &str) -> Result<usize, ServiceError> {
    value.parse::<usize>().map_err(|_| {
        PlatformError::new(
            ErrorCategory::Validation,
            format!("{tool} expects a non-negative integer count"),
        )
        .with_code("service.tool.count.parse_failed")
        .with_detail(value.to_owned())
    })
}

fn parse_newseq_params(arguments: &[String]) -> Result<NewseqParams, ServiceError> {
    if arguments.len() < 2 {
        return Err(tool_usage_error("newseq", newseq_help()));
    }

    let identifier = arguments[0].clone();
    let residues = arguments[1].clone();
    let mut description = None;
    let mut molecule = None;

    let mut index = 2;
    while index < arguments.len() {
        let argument = &arguments[index];
        if let Some(value) = argument.strip_prefix("--description=") {
            description = Some(value.to_owned());
            index += 1;
            continue;
        }
        if argument == "--description" {
            let value = arguments.get(index + 1).ok_or_else(|| {
                PlatformError::new(ErrorCategory::Validation, "missing value for --description")
                    .with_code("service.tool.newseq.description_missing")
            })?;
            description = Some(value.clone());
            index += 2;
            continue;
        }
        if let Some(value) = argument.strip_prefix("--molecule=") {
            molecule = Some(parse_molecule(value)?);
            index += 1;
            continue;
        }
        if argument == "--molecule" {
            let value = arguments.get(index + 1).ok_or_else(|| {
                PlatformError::new(ErrorCategory::Validation, "missing value for --molecule")
                    .with_code("service.tool.newseq.molecule_missing")
            })?;
            molecule = Some(parse_molecule(value)?);
            index += 2;
            continue;
        }

        return Err(PlatformError::new(
            ErrorCategory::Validation,
            format!("unknown newseq argument '{argument}'"),
        )
        .with_code("service.tool.newseq.argument_unknown")
        .with_detail(newseq_help()));
    }

    Ok(NewseqParams {
        identifier,
        residues,
        description,
        molecule,
    })
}

fn parse_molecule(value: &str) -> Result<MoleculeKind, ServiceError> {
    MoleculeKind::from_str(value).map_err(|_| {
        PlatformError::new(
            ErrorCategory::Validation,
            "molecule must be one of dna, rna, protein, or unknown",
        )
        .with_code("service.tool.newseq.molecule_invalid")
        .with_detail(value.to_owned())
    })
}

fn parse_nucleotide_pattern(tool: &str, value: &str) -> Result<NucleotidePattern, ServiceError> {
    NucleotidePattern::parse(value).map_err(|error| map_pattern_error(tool, error))
}

fn parse_protein_pattern(tool: &str, value: &str) -> Result<ProteinPattern, ServiceError> {
    ProteinPattern::parse(value).map_err(|error| map_pattern_error(tool, error))
}

fn parse_codcopy_params(arguments: &[String]) -> Result<CodcopyParams, ServiceError> {
    if arguments.is_empty() {
        return Err(tool_usage_error("codcopy", codcopy_help()));
    }

    let source = PathBuf::from(arguments[0].clone());
    let mut profile_out = None;
    let mut index = 1usize;

    while index < arguments.len() {
        let argument = &arguments[index];
        if let Some(value) = argument.strip_prefix("--profile-out=") {
            profile_out = Some(PathBuf::from(value));
            index += 1;
            continue;
        }
        if argument == "--profile-out" {
            let value = arguments.get(index + 1).ok_or_else(|| {
                PlatformError::new(ErrorCategory::Validation, "missing value for --profile-out")
                    .with_code("service.tool.codcopy.profile_out_missing")
            })?;
            profile_out = Some(PathBuf::from(value));
            index += 2;
            continue;
        }

        return Err(PlatformError::new(
            ErrorCategory::Validation,
            format!("unknown codcopy argument '{argument}'"),
        )
        .with_code("service.tool.codcopy.argument_unknown")
        .with_detail(codcopy_help()));
    }

    Ok(CodcopyParams {
        source,
        profile_out,
    })
}

fn parse_complex_params(arguments: &[String]) -> Result<ComplexParams, ServiceError> {
    if arguments.is_empty() {
        return Err(tool_usage_error("complex", complex_help()));
    }

    let input = SequenceInput::new(arguments[0].clone());
    let mut k_min = None;
    let mut k_max = None;
    let mut window = None;
    let mut step = None;
    let mut index = 1usize;

    while index < arguments.len() {
        let argument = &arguments[index];
        if let Some(value) = argument.strip_prefix("--k-min=") {
            k_min = Some(parse_positive_count("complex", value, "--k-min")?);
            index += 1;
            continue;
        }
        if argument == "--k-min" {
            let value = arguments.get(index + 1).ok_or_else(|| {
                PlatformError::new(ErrorCategory::Validation, "missing value for --k-min")
                    .with_code("service.tool.complex.k_min_missing")
            })?;
            k_min = Some(parse_positive_count("complex", value, "--k-min")?);
            index += 2;
            continue;
        }
        if let Some(value) = argument.strip_prefix("--k-max=") {
            k_max = Some(parse_positive_count("complex", value, "--k-max")?);
            index += 1;
            continue;
        }
        if argument == "--k-max" {
            let value = arguments.get(index + 1).ok_or_else(|| {
                PlatformError::new(ErrorCategory::Validation, "missing value for --k-max")
                    .with_code("service.tool.complex.k_max_missing")
            })?;
            k_max = Some(parse_positive_count("complex", value, "--k-max")?);
            index += 2;
            continue;
        }
        if let Some(value) = argument.strip_prefix("--window=") {
            window = Some(parse_positive_count("complex", value, "--window")?);
            index += 1;
            continue;
        }
        if argument == "--window" {
            let value = arguments.get(index + 1).ok_or_else(|| {
                PlatformError::new(ErrorCategory::Validation, "missing value for --window")
                    .with_code("service.tool.complex.window_missing")
            })?;
            window = Some(parse_positive_count("complex", value, "--window")?);
            index += 2;
            continue;
        }
        if let Some(value) = argument.strip_prefix("--step=") {
            step = Some(parse_positive_count("complex", value, "--step")?);
            index += 1;
            continue;
        }
        if argument == "--step" {
            let value = arguments.get(index + 1).ok_or_else(|| {
                PlatformError::new(ErrorCategory::Validation, "missing value for --step")
                    .with_code("service.tool.complex.step_missing")
            })?;
            step = Some(parse_positive_count("complex", value, "--step")?);
            index += 2;
            continue;
        }

        return Err(PlatformError::new(
            ErrorCategory::Validation,
            format!("unknown complex argument '{argument}'"),
        )
        .with_code("service.tool.complex.argument_unknown")
        .with_detail(complex_help()));
    }

    Ok(ComplexParams {
        input,
        k_min: k_min.ok_or_else(|| tool_usage_error("complex", complex_help()))?,
        k_max: k_max.ok_or_else(|| tool_usage_error("complex", complex_help()))?,
        window,
        step,
    })
}

fn parse_charge_params(
    arguments: &[String],
) -> Result<(ChargeParams, Option<PathBuf>), ServiceError> {
    if arguments.is_empty() {
        return Err(tool_usage_error("charge", charge_help()));
    }

    let input = SequenceInput::new(arguments[0].clone());
    let mut window = 5usize;
    let mut step = 1usize;
    let mut plot_contract_out = None;
    let mut index = 1usize;

    while index < arguments.len() {
        let argument = &arguments[index];
        if let Some(value) = argument.strip_prefix("--window=") {
            window = parse_positive_count("charge", value, "--window")?;
            index += 1;
            continue;
        }
        if argument == "--window" {
            let value = arguments.get(index + 1).ok_or_else(|| {
                PlatformError::new(ErrorCategory::Validation, "missing value for --window")
                    .with_code("service.tool.charge.window_missing")
            })?;
            window = parse_positive_count("charge", value, "--window")?;
            index += 2;
            continue;
        }
        if let Some(value) = argument.strip_prefix("--step=") {
            step = parse_positive_count("charge", value, "--step")?;
            index += 1;
            continue;
        }
        if argument == "--step" {
            let value = arguments.get(index + 1).ok_or_else(|| {
                PlatformError::new(ErrorCategory::Validation, "missing value for --step")
                    .with_code("service.tool.charge.step_missing")
            })?;
            step = parse_positive_count("charge", value, "--step")?;
            index += 2;
            continue;
        }
        if let Some(value) = argument.strip_prefix("--plot-contract-out=") {
            plot_contract_out = Some(PathBuf::from(value));
            index += 1;
            continue;
        }
        if argument == "--plot-contract-out" {
            let value = arguments.get(index + 1).ok_or_else(|| {
                PlatformError::new(
                    ErrorCategory::Validation,
                    "missing value for --plot-contract-out",
                )
                .with_code("service.tool.charge.plot_contract_out_missing")
            })?;
            plot_contract_out = Some(PathBuf::from(value));
            index += 2;
            continue;
        }

        return Err(PlatformError::new(
            ErrorCategory::Validation,
            format!("unknown charge argument '{argument}'"),
        )
        .with_code("service.tool.charge.argument_unknown")
        .with_detail(charge_help()));
    }

    Ok((
        ChargeParams {
            input,
            window,
            step,
        },
        plot_contract_out,
    ))
}

fn parse_transeq_params(arguments: &[String]) -> Result<TranseqParams, ServiceError> {
    if arguments.is_empty() {
        return Err(tool_usage_error("transeq", transeq_help()));
    }

    let input = SequenceInput::new(arguments[0].clone());
    let mut frame = TranslationFrameSelection::Frame1;
    let mut index = 1usize;

    while index < arguments.len() {
        let argument = &arguments[index];
        if let Some(value) = argument.strip_prefix("--frame=") {
            frame = parse_translation_frame("transeq", value, true)?;
            index += 1;
            continue;
        }
        if argument == "--frame" {
            let value = arguments.get(index + 1).ok_or_else(|| {
                PlatformError::new(ErrorCategory::Validation, "missing value for --frame")
                    .with_code("service.tool.transeq.frame_missing")
            })?;
            frame = parse_translation_frame("transeq", value, true)?;
            index += 2;
            continue;
        }

        return Err(PlatformError::new(
            ErrorCategory::Validation,
            format!("unknown transeq argument '{argument}'"),
        )
        .with_code("service.tool.transeq.argument_unknown")
        .with_detail(transeq_help()));
    }

    Ok(TranseqParams { input, frame })
}

fn parse_prettyseq_params(arguments: &[String]) -> Result<PrettyseqParams, ServiceError> {
    if arguments.is_empty() {
        return Err(tool_usage_error("prettyseq", prettyseq_help()));
    }

    let input = SequenceInput::new(arguments[0].clone());
    let mut frame = TranslationFrameSelection::Frame1;
    let mut width = 60usize;
    let mut index = 1usize;

    while index < arguments.len() {
        let argument = &arguments[index];
        if let Some(value) = argument.strip_prefix("--frame=") {
            frame = parse_translation_frame("prettyseq", value, false)?;
            index += 1;
            continue;
        }
        if argument == "--frame" {
            let value = arguments.get(index + 1).ok_or_else(|| {
                PlatformError::new(ErrorCategory::Validation, "missing value for --frame")
                    .with_code("service.tool.prettyseq.frame_missing")
            })?;
            frame = parse_translation_frame("prettyseq", value, false)?;
            index += 2;
            continue;
        }
        if let Some(value) = argument.strip_prefix("--width=") {
            width = parse_positive_count("prettyseq", value, "--width")?;
            index += 1;
            continue;
        }
        if argument == "--width" {
            let value = arguments.get(index + 1).ok_or_else(|| {
                PlatformError::new(ErrorCategory::Validation, "missing value for --width")
                    .with_code("service.tool.prettyseq.width_missing")
            })?;
            width = parse_positive_count("prettyseq", value, "--width")?;
            index += 2;
            continue;
        }

        return Err(PlatformError::new(
            ErrorCategory::Validation,
            format!("unknown prettyseq argument '{argument}'"),
        )
        .with_code("service.tool.prettyseq.argument_unknown")
        .with_detail(prettyseq_help()));
    }

    Ok(PrettyseqParams {
        input,
        frame,
        width,
    })
}

fn parse_translation_frame(
    tool: &str,
    value: &str,
    allow_all: bool,
) -> Result<TranslationFrameSelection, ServiceError> {
    match value.trim().to_ascii_lowercase().as_str() {
        "1" | "frame1" | "frame-1" => Ok(TranslationFrameSelection::Frame1),
        "2" | "frame2" | "frame-2" => Ok(TranslationFrameSelection::Frame2),
        "3" | "frame3" | "frame-3" => Ok(TranslationFrameSelection::Frame3),
        "all" if allow_all => Ok(TranslationFrameSelection::AllForward),
        _ => Err(PlatformError::new(
            ErrorCategory::Validation,
            if allow_all {
                "translation frame must be one of 1, 2, 3, or all"
            } else {
                "translation frame must be one of 1, 2, or 3"
            },
        )
        .with_code(format!("service.tool.{tool}.frame_invalid"))
        .with_detail(value.to_owned())),
    }
}

fn parse_extractalign_params(arguments: &[String]) -> Result<ExtractalignParams, ServiceError> {
    if arguments.is_empty() {
        return Err(tool_usage_error("extractalign", extractalign_help()));
    }

    let input = AlignmentInput::new(arguments[0].clone());
    let mut row_ordinals = Vec::new();
    let mut row_identifiers = Vec::new();
    let mut start = None;
    let mut end = None;
    let mut index = 1usize;

    while index < arguments.len() {
        let argument = &arguments[index];
        if let Some(value) = argument.strip_prefix("--row=") {
            row_ordinals.push(parse_positive_count("extractalign", value, "row")?);
            index += 1;
            continue;
        }
        if argument == "--row" {
            let value = arguments.get(index + 1).ok_or_else(|| {
                PlatformError::new(ErrorCategory::Validation, "missing value for --row")
                    .with_code("service.tool.extractalign.row_missing")
            })?;
            row_ordinals.push(parse_positive_count("extractalign", value, "row")?);
            index += 2;
            continue;
        }
        if let Some(value) = argument.strip_prefix("--row-id=") {
            row_identifiers.push(value.to_owned());
            index += 1;
            continue;
        }
        if argument == "--row-id" {
            let value = arguments.get(index + 1).ok_or_else(|| {
                PlatformError::new(ErrorCategory::Validation, "missing value for --row-id")
                    .with_code("service.tool.extractalign.row_id_missing")
            })?;
            row_identifiers.push(value.clone());
            index += 2;
            continue;
        }
        if let Some(value) = argument.strip_prefix("--start=") {
            start = Some(parse_positive_count("extractalign", value, "start")?);
            index += 1;
            continue;
        }
        if argument == "--start" {
            let value = arguments.get(index + 1).ok_or_else(|| {
                PlatformError::new(ErrorCategory::Validation, "missing value for --start")
                    .with_code("service.tool.extractalign.start_missing")
            })?;
            start = Some(parse_positive_count("extractalign", value, "start")?);
            index += 2;
            continue;
        }
        if let Some(value) = argument.strip_prefix("--end=") {
            end = Some(parse_positive_count("extractalign", value, "end")?);
            index += 1;
            continue;
        }
        if argument == "--end" {
            let value = arguments.get(index + 1).ok_or_else(|| {
                PlatformError::new(ErrorCategory::Validation, "missing value for --end")
                    .with_code("service.tool.extractalign.end_missing")
            })?;
            end = Some(parse_positive_count("extractalign", value, "end")?);
            index += 2;
            continue;
        }

        return Err(PlatformError::new(
            ErrorCategory::Validation,
            format!("unknown extractalign argument '{argument}'"),
        )
        .with_code("service.tool.extractalign.argument_unknown")
        .with_detail(extractalign_help()));
    }

    Ok(ExtractalignParams {
        input,
        row_ordinals,
        row_identifiers,
        start,
        end,
    })
}

struct NeedleCliParams {
    query: SequenceInput,
    target: SequenceInput,
    gap_open: Option<i32>,
    gap_extend: Option<i32>,
}

struct SequencePairCliParams {
    query: SequenceInput,
    target: SequenceInput,
}

fn parse_needle_params(
    tool: &str,
    arguments: &[String],
    help: &str,
) -> Result<NeedleCliParams, ServiceError> {
    if arguments.len() < 2 {
        return Err(tool_usage_error(tool, help));
    }

    let query = SequenceInput::new(arguments[0].clone());
    let target = SequenceInput::new(arguments[1].clone());
    let mut gap_open = None;
    let mut gap_extend = None;
    let mut index = 2usize;

    while index < arguments.len() {
        let argument = &arguments[index];
        if let Some(value) = argument.strip_prefix("--gap-open=") {
            gap_open = Some(parse_alignment_penalty(tool, "gap_open", value)?);
            index += 1;
            continue;
        }
        if argument == "--gap-open" {
            let value = arguments.get(index + 1).ok_or_else(|| {
                PlatformError::new(ErrorCategory::Validation, "missing value for --gap-open")
                    .with_code(format!("service.tool.{tool}.gap_open_missing"))
            })?;
            gap_open = Some(parse_alignment_penalty(tool, "gap_open", value)?);
            index += 2;
            continue;
        }
        if let Some(value) = argument.strip_prefix("--gap-extend=") {
            gap_extend = Some(parse_alignment_penalty(tool, "gap_extend", value)?);
            index += 1;
            continue;
        }
        if argument == "--gap-extend" {
            let value = arguments.get(index + 1).ok_or_else(|| {
                PlatformError::new(ErrorCategory::Validation, "missing value for --gap-extend")
                    .with_code(format!("service.tool.{tool}.gap_extend_missing"))
            })?;
            gap_extend = Some(parse_alignment_penalty(tool, "gap_extend", value)?);
            index += 2;
            continue;
        }

        return Err(PlatformError::new(
            ErrorCategory::Validation,
            format!("unknown {tool} argument '{argument}'"),
        )
        .with_code(format!("service.tool.{tool}.argument_unknown"))
        .with_detail(help));
    }

    Ok(NeedleCliParams {
        query,
        target,
        gap_open,
        gap_extend,
    })
}

fn parse_alignment_penalty(tool: &str, field: &str, value: &str) -> Result<i32, ServiceError> {
    let parsed = value.parse::<i32>().map_err(|_| {
        PlatformError::new(
            ErrorCategory::Validation,
            format!("{field} must be a positive integer"),
        )
        .with_code(format!("service.tool.{tool}.{field}_invalid"))
        .with_detail(value.to_owned())
    })?;
    if parsed <= 0 {
        return Err(PlatformError::new(
            ErrorCategory::Validation,
            format!("{field} must be a positive integer"),
        )
        .with_code(format!("service.tool.{tool}.{field}_invalid"))
        .with_detail(value.to_owned()));
    }
    Ok(parsed)
}

fn parse_sequence_pair_params(
    tool: &str,
    arguments: &[String],
    help: &str,
) -> Result<SequencePairCliParams, ServiceError> {
    if arguments.len() != 2 {
        return Err(tool_usage_error(tool, help));
    }

    Ok(SequencePairCliParams {
        query: SequenceInput::new(arguments[0].clone()),
        target: SequenceInput::new(arguments[1].clone()),
    })
}

fn parse_runget_arguments(arguments: &[String]) -> Result<(String, bool), ServiceError> {
    match arguments {
        [input] => Ok((input.clone(), false)),
        [input, flag] if flag == "--download" => Ok((input.clone(), true)),
        _ => Err(tool_usage_error("runget", runget_help())),
    }
}

fn archive_file_rows(files: &[emboss_providers::ArchiveFile]) -> Vec<Vec<String>> {
    files
        .iter()
        .map(|file| {
            vec![
                file.role.clone(),
                file.format.clone(),
                file.url.clone(),
                file.size_bytes
                    .map(|value| value.to_string())
                    .unwrap_or_else(|| "-".to_owned()),
                file.checksum_md5.clone().unwrap_or_else(|| "-".to_owned()),
            ]
        })
        .collect()
}

fn parse_positive_count(tool: &str, value: &str, flag: &str) -> Result<usize, ServiceError> {
    let parsed = value.parse::<usize>().map_err(|_| {
        PlatformError::new(
            ErrorCategory::Validation,
            format!("{flag} must be a positive integer"),
        )
        .with_code(format!("service.tool.{tool}.{flag}_invalid"))
        .with_detail(value.to_owned())
    })?;

    if parsed == 0 {
        return Err(PlatformError::new(
            ErrorCategory::Validation,
            format!("{flag} must be a positive integer"),
        )
        .with_code(format!("service.tool.{tool}.{flag}_invalid"))
        .with_detail(value.to_owned()));
    }

    Ok(parsed)
}

fn map_pattern_error(tool: &str, error: PatternError) -> ServiceError {
    let code = match error {
        PatternError::EmptyPattern => format!("service.tool.{tool}.pattern.empty"),
        PatternError::InvalidNucleotideSymbol(_) => {
            format!("service.tool.{tool}.pattern.nucleotide_invalid")
        }
        PatternError::InvalidProteinSymbol(_) => {
            format!("service.tool.{tool}.pattern.protein_invalid")
        }
    };
    PlatformError::new(ErrorCategory::Validation, error.to_string()).with_code(code)
}

fn parse_trimseq_params(arguments: &[String]) -> Result<TrimseqParams, ServiceError> {
    if arguments.is_empty() {
        return Err(tool_usage_error("trimseq", trimseq_help()));
    }

    let input = SequenceInput::new(arguments[0].clone());
    let mut left_trim = 0usize;
    let mut right_trim = 0usize;
    let mut index = 1usize;

    while index < arguments.len() {
        let argument = &arguments[index];
        if let Some(value) = argument.strip_prefix("--left=") {
            left_trim = parse_non_negative_count("trimseq", value)?;
            index += 1;
            continue;
        }
        if argument == "--left" {
            let value = arguments.get(index + 1).ok_or_else(|| {
                PlatformError::new(ErrorCategory::Validation, "missing value for --left")
                    .with_code("service.tool.trimseq.left_missing")
            })?;
            left_trim = parse_non_negative_count("trimseq", value)?;
            index += 2;
            continue;
        }
        if let Some(value) = argument.strip_prefix("--right=") {
            right_trim = parse_non_negative_count("trimseq", value)?;
            index += 1;
            continue;
        }
        if argument == "--right" {
            let value = arguments.get(index + 1).ok_or_else(|| {
                PlatformError::new(ErrorCategory::Validation, "missing value for --right")
                    .with_code("service.tool.trimseq.right_missing")
            })?;
            right_trim = parse_non_negative_count("trimseq", value)?;
            index += 2;
            continue;
        }

        return Err(PlatformError::new(
            ErrorCategory::Validation,
            format!("unknown trimseq argument '{argument}'"),
        )
        .with_code("service.tool.trimseq.argument_unknown")
        .with_detail(trimseq_help()));
    }

    Ok(TrimseqParams {
        input,
        left_trim,
        right_trim,
    })
}

fn parse_revseq_params(arguments: &[String]) -> Result<RevseqParams, ServiceError> {
    if arguments.is_empty() {
        return Err(tool_usage_error("revseq", revseq_help()));
    }

    let mut input = None;
    let mut mode = RevseqMode::Auto;

    for argument in arguments {
        match argument.as_str() {
            "--reverse-only" => {
                if mode == RevseqMode::ReverseComplement {
                    return Err(PlatformError::new(
                        ErrorCategory::Validation,
                        "revseq cannot combine --reverse-only with --complement",
                    )
                    .with_code("service.tool.revseq.mode_conflict")
                    .with_detail(revseq_help()));
                }
                mode = RevseqMode::ReverseOnly;
            }
            "--complement" => {
                if mode == RevseqMode::ReverseOnly {
                    return Err(PlatformError::new(
                        ErrorCategory::Validation,
                        "revseq cannot combine --complement with --reverse-only",
                    )
                    .with_code("service.tool.revseq.mode_conflict")
                    .with_detail(revseq_help()));
                }
                mode = RevseqMode::ReverseComplement;
            }
            value if value.starts_with("--") => {
                return Err(PlatformError::new(
                    ErrorCategory::Validation,
                    format!("unknown revseq argument '{value}'"),
                )
                .with_code("service.tool.revseq.argument_unknown")
                .with_detail(revseq_help()));
            }
            value => {
                if input.is_some() {
                    return Err(tool_usage_error("revseq", revseq_help()));
                }
                input = Some(SequenceInput::new(value.to_owned()));
            }
        }
    }

    Ok(RevseqParams {
        input: input.ok_or_else(|| tool_usage_error("revseq", revseq_help()))?,
        mode,
    })
}

fn parse_descseq_params(arguments: &[String]) -> Result<DescseqParams, ServiceError> {
    if arguments.len() != 1 {
        return Err(tool_usage_error("descseq", descseq_help()));
    }

    Ok(DescseqParams {
        input: SequenceInput::new(arguments[0].clone()),
    })
}

fn parse_maskseq_params(arguments: &[String]) -> Result<MaskseqParams, ServiceError> {
    if arguments.len() < 2 {
        return Err(tool_usage_error("maskseq", maskseq_help()));
    }

    let input = SequenceInput::new(arguments[0].clone());
    let mut intervals = Vec::new();
    let mut mask_char = None;
    let mut index = 1usize;

    while index < arguments.len() {
        let argument = &arguments[index];
        if let Some(value) = argument.strip_prefix("--mask-char=") {
            mask_char = Some(parse_mask_char("maskseq", value)?);
            index += 1;
            continue;
        }
        if argument == "--mask-char" {
            let value = arguments.get(index + 1).ok_or_else(|| {
                PlatformError::new(ErrorCategory::Validation, "missing value for --mask-char")
                    .with_code("service.tool.maskseq.mask_char_missing")
            })?;
            mask_char = Some(parse_mask_char("maskseq", value)?);
            index += 2;
            continue;
        }
        if argument.starts_with("--") {
            return Err(PlatformError::new(
                ErrorCategory::Validation,
                format!("unknown maskseq argument '{argument}'"),
            )
            .with_code("service.tool.maskseq.argument_unknown")
            .with_detail(maskseq_help()));
        }

        intervals.push(parse_interval_token("maskseq", argument)?);
        index += 1;
    }

    if intervals.is_empty() {
        return Err(tool_usage_error("maskseq", maskseq_help()));
    }

    Ok(MaskseqParams {
        input,
        intervals,
        mask_char,
    })
}

fn parse_maskfeat_params(arguments: &[String]) -> Result<MaskfeatParams, ServiceError> {
    if arguments.is_empty() {
        return Err(tool_usage_error("maskfeat", maskfeat_help()));
    }

    let input = SequenceInput::new(arguments[0].clone());
    let (selector, mask_char) = parse_feature_selector_flags("maskfeat", &arguments[1..], true)?;

    Ok(MaskfeatParams {
        input,
        selector,
        mask_char,
    })
}

fn parse_extractfeat_params(arguments: &[String]) -> Result<ExtractfeatParams, ServiceError> {
    if arguments.is_empty() {
        return Err(tool_usage_error("extractfeat", extractfeat_help()));
    }

    let input = SequenceInput::new(arguments[0].clone());
    let (selector, mask_char) =
        parse_feature_selector_flags("extractfeat", &arguments[1..], false)?;
    if mask_char.is_some() {
        return Err(PlatformError::new(
            ErrorCategory::Validation,
            "extractfeat does not accept --mask-char",
        )
        .with_code("service.tool.extractfeat.mask_char_unsupported")
        .with_detail(extractfeat_help()));
    }

    Ok(ExtractfeatParams { input, selector })
}

fn parse_featcopy_params(arguments: &[String]) -> Result<FeatcopyParams, ServiceError> {
    if arguments.len() < 2 {
        return Err(tool_usage_error("featcopy", featcopy_help()));
    }

    let source = SequenceInput::new(arguments[0].clone());
    let target = SequenceInput::new(arguments[1].clone());
    let (selector, mask_char) = parse_feature_selector_flags("featcopy", &arguments[2..], false)?;
    if mask_char.is_some() {
        return Err(PlatformError::new(
            ErrorCategory::Validation,
            "featcopy does not accept --mask-char",
        )
        .with_code("service.tool.featcopy.mask_char_unsupported")
        .with_detail(featcopy_help()));
    }

    Ok(FeatcopyParams {
        source,
        target,
        selector,
    })
}

fn parse_coderet_params(arguments: &[String]) -> Result<CoderetParams, ServiceError> {
    if arguments.is_empty() {
        return Err(tool_usage_error("coderet", coderet_help()));
    }

    let input = SequenceInput::new(arguments[0].clone());
    let mut translate = false;
    let mut selector_arguments = Vec::new();

    for argument in &arguments[1..] {
        if argument == "--translate" {
            translate = true;
        } else {
            selector_arguments.push(argument.clone());
        }
    }

    let (selector, mask_char) =
        parse_feature_selector_flags("coderet", &selector_arguments, false)?;
    if mask_char.is_some() {
        return Err(PlatformError::new(
            ErrorCategory::Validation,
            "coderet does not accept --mask-char",
        )
        .with_code("service.tool.coderet.mask_char_unsupported")
        .with_detail(coderet_help()));
    }

    let selector = if matches!(selector, FeatureSelector::Any) {
        FeatureSelector::Kind(FeatureKind::CodingSequence)
    } else {
        selector
    };

    Ok(CoderetParams {
        input,
        selector,
        translate,
    })
}

fn parse_featmerge_params(arguments: &[String]) -> Result<FeatmergeParams, ServiceError> {
    if arguments.len() < 2 {
        return Err(tool_usage_error("featmerge", featmerge_help()));
    }

    let left = SequenceInput::new(arguments[0].clone());
    let right = SequenceInput::new(arguments[1].clone());
    let (selector, mask_char) =
        parse_feature_selector_flags("featmerge", &arguments[2..], false)?;
    if mask_char.is_some() {
        return Err(PlatformError::new(
            ErrorCategory::Validation,
            "featmerge does not accept --mask-char",
        )
        .with_code("service.tool.featmerge.mask_char_unsupported")
        .with_detail(featmerge_help()));
    }

    Ok(FeatmergeParams {
        left,
        right,
        selector,
    })
}

fn parse_featreport_params(arguments: &[String]) -> Result<FeatreportParams, ServiceError> {
    if arguments.is_empty() {
        return Err(tool_usage_error("featreport", featreport_help()));
    }

    let input = SequenceInput::new(arguments[0].clone());
    let (selector, mask_char) =
        parse_feature_selector_flags("featreport", &arguments[1..], false)?;
    if mask_char.is_some() {
        return Err(PlatformError::new(
            ErrorCategory::Validation,
            "featreport does not accept --mask-char",
        )
        .with_code("service.tool.featreport.mask_char_unsupported")
        .with_detail(featreport_help()));
    }

    Ok(FeatreportParams { input, selector })
}

fn parse_feattext_params(arguments: &[String]) -> Result<FeattextParams, ServiceError> {
    if arguments.is_empty() {
        return Err(tool_usage_error("feattext", feattext_help()));
    }

    let input = SequenceInput::new(arguments[0].clone());
    let (selector, mask_char) =
        parse_feature_selector_flags("feattext", &arguments[1..], false)?;
    if mask_char.is_some() {
        return Err(PlatformError::new(
            ErrorCategory::Validation,
            "feattext does not accept --mask-char",
        )
        .with_code("service.tool.feattext.mask_char_unsupported")
        .with_detail(feattext_help()));
    }

    Ok(FeattextParams { input, selector })
}

fn parse_feature_selector_flags(
    tool: &str,
    arguments: &[String],
    allow_mask_char: bool,
) -> Result<(FeatureSelector, Option<char>), ServiceError> {
    let mut selectors = Vec::new();
    let mut mask_char = None;
    let mut index = 0usize;

    while index < arguments.len() {
        let argument = &arguments[index];
        if let Some(value) = argument.strip_prefix("--kind=") {
            selectors.push(FeatureSelector::Kind(parse_feature_kind(value)));
            index += 1;
            continue;
        }
        if argument == "--kind" {
            let value = arguments.get(index + 1).ok_or_else(|| {
                PlatformError::new(ErrorCategory::Validation, "missing value for --kind")
                    .with_code(format!("service.tool.{tool}.kind_missing"))
            })?;
            selectors.push(FeatureSelector::Kind(parse_feature_kind(value)));
            index += 2;
            continue;
        }
        if let Some(value) = argument.strip_prefix("--name=") {
            selectors.push(FeatureSelector::Name(value.to_owned()));
            index += 1;
            continue;
        }
        if argument == "--name" {
            let value = arguments.get(index + 1).ok_or_else(|| {
                PlatformError::new(ErrorCategory::Validation, "missing value for --name")
                    .with_code(format!("service.tool.{tool}.name_missing"))
            })?;
            selectors.push(FeatureSelector::Name(value.clone()));
            index += 2;
            continue;
        }
        if let Some(value) = argument.strip_prefix("--qualifier=") {
            selectors.push(parse_feature_qualifier_selector(value)?);
            index += 1;
            continue;
        }
        if argument == "--qualifier" {
            let value = arguments.get(index + 1).ok_or_else(|| {
                PlatformError::new(ErrorCategory::Validation, "missing value for --qualifier")
                    .with_code(format!("service.tool.{tool}.qualifier_missing"))
            })?;
            selectors.push(parse_feature_qualifier_selector(value)?);
            index += 2;
            continue;
        }
        if let Some(value) = argument.strip_prefix("--strand=") {
            selectors.push(FeatureSelector::Strand(parse_feature_strand(value)?));
            index += 1;
            continue;
        }
        if argument == "--strand" {
            let value = arguments.get(index + 1).ok_or_else(|| {
                PlatformError::new(ErrorCategory::Validation, "missing value for --strand")
                    .with_code(format!("service.tool.{tool}.strand_missing"))
            })?;
            selectors.push(FeatureSelector::Strand(parse_feature_strand(value)?));
            index += 2;
            continue;
        }
        if allow_mask_char {
            if let Some(value) = argument.strip_prefix("--mask-char=") {
                mask_char = Some(parse_mask_char(tool, value)?);
                index += 1;
                continue;
            }
            if argument == "--mask-char" {
                let value = arguments.get(index + 1).ok_or_else(|| {
                    PlatformError::new(ErrorCategory::Validation, "missing value for --mask-char")
                        .with_code(format!("service.tool.{tool}.mask_char_missing"))
                })?;
                mask_char = Some(parse_mask_char(tool, value)?);
                index += 2;
                continue;
            }
        }

        return Err(PlatformError::new(
            ErrorCategory::Validation,
            format!("unknown {tool} argument '{argument}'"),
        )
        .with_code(format!("service.tool.{tool}.argument_unknown"))
        .with_detail(feature_tool_help(tool)));
    }

    let selector = match selectors.len() {
        0 => FeatureSelector::Any,
        1 => selectors.remove(0),
        _ => FeatureSelector::All(selectors),
    };

    Ok((selector, mask_char))
}

fn parse_feature_qualifier_selector(value: &str) -> Result<FeatureSelector, ServiceError> {
    let (key, qualifier_value) = match value.split_once('=') {
        Some((key, qualifier_value)) => (key.trim(), Some(qualifier_value.trim().to_owned())),
        None => (value.trim(), None),
    };

    if key.is_empty() {
        return Err(PlatformError::new(
            ErrorCategory::Validation,
            "feature qualifier selectors require a non-empty key",
        )
        .with_code("service.tool.feature.qualifier_invalid"));
    }

    Ok(FeatureSelector::Qualifier {
        key: key.to_owned(),
        value: qualifier_value.filter(|value| !value.is_empty()),
    })
}

fn parse_feature_kind(value: &str) -> FeatureKind {
    match value.trim().to_ascii_lowercase().as_str() {
        "gene" => FeatureKind::Gene,
        "cds" | "codingsequence" | "coding_sequence" => FeatureKind::CodingSequence,
        "exon" => FeatureKind::Exon,
        "intron" => FeatureKind::Intron,
        "region" => FeatureKind::Region,
        "motif" => FeatureKind::Motif,
        "repeat" | "repeatregion" | "repeat_region" => FeatureKind::RepeatRegion,
        "misc" | "miscfeature" | "misc_feature" => FeatureKind::MiscFeature,
        other => FeatureKind::Other(other.to_owned()),
    }
}

fn parse_feature_strand(value: &str) -> Result<Strand, ServiceError> {
    match value.trim().to_ascii_lowercase().as_str() {
        "forward" | "+" | "plus" => Ok(Strand::Forward),
        "reverse" | "-" | "minus" => Ok(Strand::Reverse),
        "unknown" | "." => Ok(Strand::Unknown),
        other => Err(PlatformError::new(
            ErrorCategory::Validation,
            "strand must be one of forward, reverse, or unknown",
        )
        .with_code("service.tool.feature.strand_invalid")
        .with_detail(other.to_owned())),
    }
}

fn parse_mask_char(tool: &str, value: &str) -> Result<char, ServiceError> {
    let mut chars = value.chars();
    match (chars.next(), chars.next()) {
        (Some(symbol), None) => Ok(symbol),
        _ => Err(PlatformError::new(
            ErrorCategory::Validation,
            format!("{tool} expects a single mask character"),
        )
        .with_code("service.tool.mask_char.invalid")
        .with_detail(value.to_owned())),
    }
}

fn parse_interval_token(tool: &str, value: &str) -> Result<Interval, ServiceError> {
    let (start_raw, end_raw) = value.split_once(':').ok_or_else(|| {
        PlatformError::new(
            ErrorCategory::Validation,
            format!("{tool} expects intervals in start:end form"),
        )
        .with_code("service.tool.interval.parse_failed")
        .with_detail(value.to_owned())
    })?;
    let start = parse_positive_index(tool, start_raw.trim())?;
    let end = parse_positive_index(tool, end_raw.trim())?;
    if start > end {
        return Err(PlatformError::new(
            ErrorCategory::Validation,
            format!("{tool} requires interval start <= end"),
        )
        .with_code("service.tool.interval.invalid")
        .with_detail(value.to_owned()));
    }

    Interval::new(start - 1, end).map_err(|error| {
        PlatformError::new(ErrorCategory::Validation, error.to_string())
            .with_code("service.tool.interval.invalid")
            .with_detail(value.to_owned())
    })
}

fn format_interval_list(intervals: &[Interval]) -> String {
    intervals
        .iter()
        .map(|interval| format!("{}:{}", interval.start() + 1, interval.end()))
        .collect::<Vec<_>>()
        .join(", ")
}

fn mask_char_summary(mask_char: Option<char>) -> String {
    match mask_char {
        Some(mask_char) => mask_char.to_string(),
        None => "auto (N for nucleotide, X for protein)".to_owned(),
    }
}

fn describe_selector(selector: &FeatureSelector) -> String {
    match selector {
        FeatureSelector::Any => "all features".to_owned(),
        FeatureSelector::Kind(kind) => format!("kind={}", describe_feature_kind(kind)),
        FeatureSelector::Name(name) => format!("name={name}"),
        FeatureSelector::Qualifier { key, value } => match value {
            Some(value) => format!("qualifier={key}={value}"),
            None => format!("qualifier={key}"),
        },
        FeatureSelector::Strand(strand) => format!("strand={strand}"),
        FeatureSelector::Overlaps(interval) => {
            format!("overlaps={}..{}", interval.start() + 1, interval.end())
        }
        FeatureSelector::ContainedWithin(interval) => {
            format!(
                "contained-within={}..{}",
                interval.start() + 1,
                interval.end()
            )
        }
        FeatureSelector::All(selectors) => selectors
            .iter()
            .map(describe_selector)
            .collect::<Vec<_>>()
            .join(" and "),
        FeatureSelector::AnyOf(selectors) => selectors
            .iter()
            .map(describe_selector)
            .collect::<Vec<_>>()
            .join(" or "),
        FeatureSelector::Not(selector) => format!("not ({})", describe_selector(selector)),
    }
}

fn describe_feature_kind(kind: &FeatureKind) -> String {
    match kind {
        FeatureKind::Gene => "gene".to_owned(),
        FeatureKind::CodingSequence => "cds".to_owned(),
        FeatureKind::Exon => "exon".to_owned(),
        FeatureKind::Intron => "intron".to_owned(),
        FeatureKind::Region => "region".to_owned(),
        FeatureKind::Motif => "motif".to_owned(),
        FeatureKind::RepeatRegion => "repeat_region".to_owned(),
        FeatureKind::MiscFeature => "misc_feature".to_owned(),
        FeatureKind::Other(label) => label.clone(),
    }
}

fn feature_tool_help(tool: &str) -> &'static str {
    match tool {
        "matcher" => matcher_help(),
        "distmat" => distmat_help(),
        "cons" => cons_help(),
        "consambig" => consambig_help(),
        "needle" => needle_help(),
        "needleall" => needleall_help(),
        "water" => water_help(),
        "seqret" => seqret_help(),
        "refseqget" => refseqget_help(),
        "runinfo" => runinfo_help(),
        "runget" => runget_help(),
        "aligncopy" => aligncopy_help(),
        "aligncopypair" => aligncopypair_help(),
        "infoalign" => infoalign_help(),
        "extractalign" => extractalign_help(),
        "maskfeat" => maskfeat_help(),
        "extractfeat" => extractfeat_help(),
        "featcopy" => featcopy_help(),
        "coderet" => coderet_help(),
        "featmerge" => featmerge_help(),
        "featreport" => featreport_help(),
        "feattext" => feattext_help(),
        "fuzznuc" => fuzznuc_help(),
        "fuzzpro" => fuzzpro_help(),
        "fuzztran" => fuzztran_help(),
        "charge" => charge_help(),
        "complex" => complex_help(),
        "compseq" => compseq_help(),
        "geecee" => geecee_help(),
        "pepstats" => pepstats_help(),
        "chips" => chips_help(),
        "codcopy" => codcopy_help(),
        "cai" => cai_help(),
        "codcmp" => codcmp_help(),
        _ => "",
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use emboss_core::MoleculeKind;
    use emboss_diagnostics::PlatformError;
    use emboss_providers::{HttpRequest, HttpResponse, ProviderHttpClient};
    use emboss_tools::{ToolDescriptor, governed_tool_descriptors};

    use super::EmbossService;
    use crate::{
        ExecutionContext, InvocationOrigin, InvocationRequest, OutcomeStatus, ResultPayload,
        ServiceRegistry, ToolCatalog, ToolInputKind, ToolInputResolution, ToolName,
    };

    #[derive(Clone, Debug, Default)]
    struct MockHttpClient {
        responses: HashMap<String, HttpResponse>,
    }

    impl MockHttpClient {
        fn with_response(mut self, url: impl Into<String>, response: HttpResponse) -> Self {
            self.responses.insert(url.into(), response);
            self
        }
    }

    impl ProviderHttpClient for MockHttpClient {
        fn get_text(&self, request: &HttpRequest) -> Result<HttpResponse, PlatformError> {
            self.responses.get(&request.url).cloned().ok_or_else(|| {
                PlatformError::new(
                    emboss_diagnostics::ErrorCategory::Invocation,
                    "mock response was not configured for provider request",
                )
                .with_code("service.seqret.test.missing_response")
                .with_detail(request.url.clone())
            })
        }
    }

    #[test]
    fn resolves_registered_tool_to_placeholder_response() {
        let mut registry = ServiceRegistry::new();
        registry
            .register(ToolDescriptor::new("customtool", "custom placeholder"))
            .expect("registration should succeed");

        let service = EmbossService::new(registry);
        let request = InvocationRequest::new(
            ExecutionContext::for_origin(InvocationOrigin::Cli),
            ToolName::new("customtool").expect("tool name should be valid"),
        );

        let response = service.invoke(request).expect("tool should resolve");
        assert_eq!(response.descriptor.name, "customtool");
        assert_eq!(response.tool.as_str(), "customtool");
        assert_eq!(
            response.report.outcome.status,
            OutcomeStatus::NotImplemented
        );
    }

    #[test]
    fn rejects_unknown_tool_invocation() {
        let service = EmbossService::empty();
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("water").expect("tool name should be valid"),
        );

        assert!(service.invoke(request).is_err());
    }

    #[test]
    fn starts_with_default_platform_configuration_and_builtin_sequence_providers() {
        let service = EmbossService::empty();
        assert!(
            service
                .providers()
                .find(&emboss_providers::ProviderId::new("ena").expect("valid provider"))
                .is_some()
        );
        assert!(
            service
                .providers()
                .find(&emboss_providers::ProviderId::new("ncbi").expect("valid provider"))
                .is_some()
        );
        assert!(
            service
                .providers()
                .find(&emboss_providers::ProviderId::new("sra").expect("valid provider"))
                .is_some()
        );
        assert!(service.config().acquisition.allow_remote_acquisition);
    }

    #[test]
    fn classifies_provider_qualified_inputs_through_service() {
        let service = EmbossService::empty();
        let reference = service
            .classify_input("ena:AB000263")
            .expect("input should classify");
        assert_eq!(reference.kind(), ToolInputKind::ProviderQualified);
    }

    #[test]
    fn resolves_accessions_through_shared_service_seam() {
        let service = EmbossService::empty();
        let reference = service
            .classify_input("AB000263")
            .expect("input should classify");

        let resolution = service
            .resolve_input(reference, emboss_providers::ResolutionIntent::SequenceInput)
            .expect("resolution should succeed");
        assert!(matches!(
            resolution,
            ToolInputResolution::ProviderRouted { .. }
        ));
    }

    fn sequence_fixture() -> std::path::PathBuf {
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../emboss-tools/tests/fixtures/three_records.fasta")
    }

    fn second_sequence_fixture() -> std::path::PathBuf {
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../emboss-tools/tests/fixtures/two_records.fasta")
    }

    fn gapped_sequence_fixture() -> std::path::PathBuf {
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../emboss-tools/tests/fixtures/gapped_records.fasta")
    }

    fn annotated_feature_fixture() -> std::path::PathBuf {
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../emboss-tools/tests/fixtures/annotated_feature.gbk")
    }

    fn annotated_complex_fixture() -> std::path::PathBuf {
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../emboss-tools/tests/fixtures/annotated_complex.gbk")
    }

    fn featcopy_target_fixture() -> std::path::PathBuf {
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../emboss-tools/tests/fixtures/featcopy_target.fasta")
    }

    fn featmerge_right_fixture() -> std::path::PathBuf {
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../emboss-tools/tests/fixtures/annotated_merge_right.gbk")
    }

    fn featcopy_mismatch_fixture() -> std::path::PathBuf {
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../emboss-tools/tests/fixtures/featcopy_mismatch.fasta")
    }

    fn protein_fixture() -> std::path::PathBuf {
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../emboss-tools/tests/fixtures/protein_records.fasta")
    }

    fn nucleotide_pattern_fixture() -> std::path::PathBuf {
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../emboss-tools/tests/fixtures/nucleotide_pattern_records.fasta")
    }

    fn complex_fixture() -> std::path::PathBuf {
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../emboss-tools/tests/fixtures/complex_records.fasta")
    }

    fn complex_invalid_fixture() -> std::path::PathBuf {
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../emboss-tools/tests/fixtures/complex_invalid.fasta")
    }

    fn charge_fixture() -> std::path::PathBuf {
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../emboss-tools/tests/fixtures/charge_protein.fasta")
    }

    fn charge_invalid_fixture() -> std::path::PathBuf {
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../emboss-tools/tests/fixtures/charge_invalid.fasta")
    }

    fn charge_plot_fixture() -> std::path::PathBuf {
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../emboss-tools/tests/fixtures/charge_plot_contract.json")
    }

    fn protein_stats_fixture() -> std::path::PathBuf {
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../emboss-tools/tests/fixtures/protein_stats_records.fasta")
    }

    fn codon_reference_fixture() -> std::path::PathBuf {
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../emboss-tools/tests/fixtures/codon_reference.fasta")
    }

    fn codon_query_fixture() -> std::path::PathBuf {
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../emboss-tools/tests/fixtures/codon_query.fasta")
    }

    fn codon_compare_right_fixture() -> std::path::PathBuf {
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../emboss-tools/tests/fixtures/codon_compare_right.fasta")
    }

    fn pairwise_alignment_fixture() -> std::path::PathBuf {
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../emboss-tools/tests/fixtures/pairwise_alignment.sto")
    }

    fn multiple_alignment_fixture() -> std::path::PathBuf {
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../emboss-tools/tests/fixtures/multiple_alignment.sto")
    }

    fn needle_query_fixture() -> std::path::PathBuf {
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../emboss-tools/tests/fixtures/needle_query.fasta")
    }

    fn needle_target_fixture() -> std::path::PathBuf {
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../emboss-tools/tests/fixtures/needle_target.fasta")
    }

    fn water_query_fixture() -> std::path::PathBuf {
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../emboss-tools/tests/fixtures/water_query.fasta")
    }

    fn water_target_fixture() -> std::path::PathBuf {
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../emboss-tools/tests/fixtures/water_target.fasta")
    }

    fn needleall_queries_fixture() -> std::path::PathBuf {
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../emboss-tools/tests/fixtures/needleall_queries.fasta")
    }

    fn needleall_targets_fixture() -> std::path::PathBuf {
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../emboss-tools/tests/fixtures/needleall_targets.fasta")
    }

    fn checktrans_nucleotide_fixture() -> std::path::PathBuf {
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../emboss-tools/tests/fixtures/checktrans_nucleotide.fasta")
    }

    fn checktrans_protein_fixture() -> std::path::PathBuf {
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../emboss-tools/tests/fixtures/checktrans_protein.fasta")
    }

    fn checktrans_mismatch_fixture() -> std::path::PathBuf {
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../emboss-tools/tests/fixtures/checktrans_mismatch.fasta")
    }

    fn checktrans_invalid_codon_fixture() -> std::path::PathBuf {
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../emboss-tools/tests/fixtures/checktrans_invalid_codon.fasta")
    }

    fn getorf_fixture() -> std::path::PathBuf {
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../emboss-tools/tests/fixtures/getorf_records.fasta")
    }

    fn tranalign_protein_alignment_fixture() -> std::path::PathBuf {
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../emboss-tools/tests/fixtures/tranalign_protein_alignment.sto")
    }

    fn implemented_service() -> EmbossService {
        let mut registry = ServiceRegistry::new();
        for descriptor in governed_tool_descriptors() {
            registry
                .register(*descriptor)
                .expect("tool registration should succeed");
        }
        EmbossService::new(registry)
    }

    #[test]
    fn executes_seqret_against_local_fixture() {
        let service = implemented_service();
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("seqret").expect("tool name should be valid"),
        )
        .with_arguments(vec![sequence_fixture().display().to_string()]);

        let response = service.invoke(request).expect("seqret should execute");
        assert_eq!(response.status, crate::InvocationStatus::Completed);
        match &response.result.payload {
            ResultPayload::SequenceCollection(records) => {
                assert_eq!(records.len(), 3);
                assert_eq!(records[0].identifier().accession(), "alpha");
            }
            payload => panic!("unexpected payload: {payload:?}"),
        }
    }

    #[test]
    fn seqret_rejects_ambiguous_bare_accession() {
        let service = implemented_service();
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("seqret").expect("tool name should be valid"),
        )
        .with_arguments(vec!["AB000263".to_owned()]);

        let error = service
            .invoke(request)
            .expect_err("bare accession should be rejected");
        assert_eq!(
            error.code(),
            Some("providers.sequence.ambiguous_bare_accession")
        );
    }

    #[test]
    fn executes_seqret_against_provider_qualified_accession_with_mocked_retrieval() {
        let service = implemented_service();
        let tool = ToolName::new("seqret").expect("tool name should be valid");
        let descriptor = service
            .registry()
            .find(&tool)
            .copied()
            .expect("seqret should be registered");
        let request = InvocationRequest::new(ExecutionContext::default(), tool)
            .with_arguments(vec!["ena:AB000263".to_owned()]);
        let client = MockHttpClient::default().with_response(
            "https://www.ebi.ac.uk/ena/browser/api/fasta/AB000263",
            HttpResponse::new(200, ">AB000263 example\nACGT\n"),
        );

        let response = service
            .invoke_seqret_inner(request, descriptor, Some(&client))
            .expect("seqret should execute with mocked retrieval");
        match &response.result.payload {
            ResultPayload::SequenceCollection(records) => {
                assert_eq!(records.len(), 1);
                assert_eq!(records[0].identifier().accession(), "AB000263");
                assert_eq!(records[0].metadata().source.as_deref(), Some("ena"));
            }
            payload => panic!("unexpected payload: {payload:?}"),
        }
    }

    #[test]
    fn executes_refseqget_against_provider_qualified_accession_with_mocked_retrieval() {
        let service = implemented_service();
        let tool = ToolName::new("refseqget").expect("tool name should be valid");
        let descriptor = service
            .registry()
            .find(&tool)
            .copied()
            .expect("refseqget should be registered");
        let request = InvocationRequest::new(ExecutionContext::default(), tool)
            .with_arguments(vec!["ncbi:protein:NP_000537.3".to_owned()]);
        let client = MockHttpClient::default().with_response(
            "https://eutils.ncbi.nlm.nih.gov/entrez/eutils/efetch.fcgi?db=protein&id=NP_000537.3&rettype=fasta&retmode=text",
            HttpResponse::new(200, ">NP_000537.3 TP53\nMEEPQSDPSV\n"),
        );

        let response = service
            .invoke_refseqget_inner(request, descriptor, Some(&client))
            .expect("refseqget should execute with mocked retrieval");
        match &response.result.payload {
            ResultPayload::Sequence(record) => {
                assert_eq!(record.identifier().accession(), "NP_000537.3");
                assert_eq!(record.metadata().source.as_deref(), Some("ncbi"));
            }
            payload => panic!("unexpected payload: {payload:?}"),
        }
    }

    #[test]
    fn refseqget_rejects_local_file_input() {
        let service = implemented_service();
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("refseqget").expect("tool name should be valid"),
        )
        .with_arguments(vec![sequence_fixture().display().to_string()]);

        let error = service
            .invoke(request)
            .expect_err("refseqget should reject local file inputs");
        assert_eq!(
            error.code(),
            Some("service.refseqget.local_input_not_supported")
        );
    }

    #[test]
    fn executes_runinfo_against_mocked_ena_run_metadata() {
        let service = implemented_service();
        let tool = ToolName::new("runinfo").expect("tool name should be valid");
        let descriptor = service
            .registry()
            .find(&tool)
            .copied()
            .expect("runinfo should be registered");
        let request = InvocationRequest::new(ExecutionContext::default(), tool)
            .with_arguments(vec!["ena:ERR123456".to_owned()]);
        let client = MockHttpClient::default().with_response(
            "https://www.ebi.ac.uk/ena/portal/api/filereport?accession=ERR123456&result=read_run&fields=run_accession%2Cstudy_accession%2Cexperiment_accession%2Csample_accession%2Cinstrument_platform%2Cinstrument_model%2Clibrary_layout%2Clibrary_strategy%2Clibrary_source%2Cfastq_ftp%2Cfastq_md5%2Cfastq_bytes%2Csubmitted_ftp%2Csubmitted_md5%2Csubmitted_bytes%2Csra_ftp%2Csra_md5%2Csra_bytes&format=tsv&download=false",
            HttpResponse::new(200, "run_accession\tstudy_accession\texperiment_accession\tsample_accession\tinstrument_platform\tinstrument_model\tlibrary_layout\tlibrary_strategy\tlibrary_source\tfastq_ftp\tfastq_md5\tfastq_bytes\tsubmitted_ftp\tsubmitted_md5\tsubmitted_bytes\tsra_ftp\tsra_md5\tsra_bytes\nERR123456\tERP000001\tERX000001\tERS000001\tILLUMINA\tNovaSeq 6000\tPAIRED\tWGS\tGENOMIC\tftp.sra.ebi.ac.uk/vol1/fastq/ERR123/ERR123456/ERR123456_1.fastq.gz;ftp.sra.ebi.ac.uk/vol1/fastq/ERR123/ERR123456/ERR123456_2.fastq.gz\tmd51;md52\t10;12\t\t\t\t\t\t\n"),
        );

        let response = service
            .invoke_runinfo_inner(request, descriptor, Some(&client))
            .expect("runinfo should execute with mocked ENA metadata");
        match &response.result.payload {
            ResultPayload::TableReport(table) => {
                assert_eq!(table.rows.len(), 2);
                assert_eq!(table.rows[0][0], "fastq");
            }
            payload => panic!("unexpected payload: {payload:?}"),
        }
        assert!(
            response
                .result
                .summary
                .lines
                .iter()
                .any(|line| line == "Provider: ena")
        );
    }

    #[test]
    fn executes_runinfo_against_mocked_sra_run_metadata() {
        let service = implemented_service();
        let tool = ToolName::new("runinfo").expect("tool name should be valid");
        let descriptor = service
            .registry()
            .find(&tool)
            .copied()
            .expect("runinfo should be registered");
        let request = InvocationRequest::new(ExecutionContext::default(), tool)
            .with_arguments(vec!["sra:SRR123456".to_owned()]);
        let client = MockHttpClient::default().with_response(
            "https://trace.ncbi.nlm.nih.gov/Traces/sra-db-be/runinfo?acc=SRR123456",
            HttpResponse::new(200, "Run,ReleaseDate,LoadDate,spots,bases,spots_with_mates,avgLength,size_MB,AssemblyName,download_path,Experiment,LibraryName,LibraryStrategy,LibrarySelection,LibrarySource,LibraryLayout,InsertSize,InsertDev,Platform,Model,SRAStudy,BioProject,Study_Pubmed_id,ProjectID,Sample,BioSample,SampleType,TaxID,ScientificName,SampleName,CenterName,Submission,dbgap_study_accession,Consent,RunHash,ReadHash\nSRR123456,2024-01-01,2024-01-02,1,100,1,100,1,,https://example.invalid/SRR123456,SRX123456,,WGS,,GENOMIC,PAIRED,,,,ILLUMINA,NextSeq 2000,SRP000001,PRJNA1,,1,SRS123456,SAMN1,,9606,Homo sapiens,,NCBI,SRA000001,,,runhash,readhash\n"),
        );

        let response = service
            .invoke_runinfo_inner(request, descriptor, Some(&client))
            .expect("runinfo should execute with mocked SRA metadata");
        match &response.result.payload {
            ResultPayload::TableReport(table) => {
                assert!(table.rows.is_empty());
            }
            payload => panic!("unexpected payload: {payload:?}"),
        }
        assert!(
            response
                .result
                .summary
                .lines
                .iter()
                .any(|line| line == "Provider: sra")
        );
    }

    #[test]
    fn executes_runget_against_mocked_ena_manifest() {
        let service = implemented_service();
        let tool = ToolName::new("runget").expect("tool name should be valid");
        let descriptor = service
            .registry()
            .find(&tool)
            .copied()
            .expect("runget should be registered");
        let request = InvocationRequest::new(ExecutionContext::default(), tool)
            .with_arguments(vec!["ena:ERR123456".to_owned()]);
        let client = MockHttpClient::default().with_response(
            "https://www.ebi.ac.uk/ena/portal/api/filereport?accession=ERR123456&result=read_run&fields=run_accession%2Cstudy_accession%2Cexperiment_accession%2Csample_accession%2Cinstrument_platform%2Cinstrument_model%2Clibrary_layout%2Clibrary_strategy%2Clibrary_source%2Cfastq_ftp%2Cfastq_md5%2Cfastq_bytes%2Csubmitted_ftp%2Csubmitted_md5%2Csubmitted_bytes%2Csra_ftp%2Csra_md5%2Csra_bytes&format=tsv&download=false",
            HttpResponse::new(200, "run_accession\tstudy_accession\texperiment_accession\tsample_accession\tinstrument_platform\tinstrument_model\tlibrary_layout\tlibrary_strategy\tlibrary_source\tfastq_ftp\tfastq_md5\tfastq_bytes\tsubmitted_ftp\tsubmitted_md5\tsubmitted_bytes\tsra_ftp\tsra_md5\tsra_bytes\nERR123456\tERP000001\tERX000001\tERS000001\tILLUMINA\tNovaSeq 6000\tPAIRED\tWGS\tGENOMIC\tftp.sra.ebi.ac.uk/vol1/fastq/ERR123/ERR123456/ERR123456_1.fastq.gz;ftp.sra.ebi.ac.uk/vol1/fastq/ERR123/ERR123456/ERR123456_2.fastq.gz\tmd51;md52\t10;12\t\t\t\t\t\t\n"),
        );

        let response = service
            .invoke_runget_inner(request, descriptor, Some(&client))
            .expect("runget should execute with mocked ENA manifest");
        match &response.result.payload {
            ResultPayload::TableReport(table) => {
                assert_eq!(table.rows.len(), 2);
                assert_eq!(table.rows[0][0], "fastq");
            }
            payload => panic!("unexpected payload: {payload:?}"),
        }
    }

    #[test]
    fn runget_rejects_download_mode_in_v1() {
        let service = implemented_service();
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("runget").expect("tool name should be valid"),
        )
        .with_arguments(vec!["ena:ERR123456".to_owned(), "--download".to_owned()]);

        let error = service
            .invoke(request)
            .expect_err("download mode should be rejected");
        assert_eq!(error.code(), Some("service.runget.download_not_supported"));
    }

    #[test]
    fn runget_reports_sra_manifest_as_not_supported() {
        let service = implemented_service();
        let tool = ToolName::new("runget").expect("tool name should be valid");
        let descriptor = service
            .registry()
            .find(&tool)
            .copied()
            .expect("runget should be registered");
        let request = InvocationRequest::new(ExecutionContext::default(), tool)
            .with_arguments(vec!["sra:SRR123456".to_owned()]);
        let client = MockHttpClient::default();

        let error = service
            .invoke_runget_inner(request, descriptor, Some(&client))
            .expect_err("SRA manifest should not yet be supported");
        assert_eq!(
            error.code(),
            Some("providers.archive.sra.manifest_not_supported")
        );
    }

    #[test]
    fn executes_seqcount_against_real_fixture() {
        let service = implemented_service();
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("seqcount").expect("tool name should be valid"),
        )
        .with_arguments(vec![sequence_fixture().display().to_string()]);

        let response = service.invoke(request).expect("seqcount should execute");
        assert_eq!(response.status, crate::InvocationStatus::Completed);
        match &response.result.payload {
            ResultPayload::TableReport(table) => {
                assert_eq!(table.columns, vec!["input", "count"]);
                assert!(table.rows[0][0].ends_with("three_records.fasta"));
                assert_eq!(table.rows[0][1], "3");
            }
            payload => panic!("unexpected payload: {payload:?}"),
        }
    }

    #[test]
    fn seqcount_rejects_malformed_input() {
        let path = std::env::temp_dir().join(format!(
            "emboss-rs-seqcount-malformed-{}.fasta",
            std::process::id()
        ));
        std::fs::write(&path, "ACGT\n").expect("fixture should write");

        let service = implemented_service();
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("seqcount").expect("tool name should be valid"),
        )
        .with_arguments(vec![path.display().to_string()]);

        let error = service
            .invoke(request)
            .expect_err("seqcount should reject malformed input");
        assert!(error.to_string().contains("invalid fasta content"));

        let _ = std::fs::remove_file(path);
    }

    #[test]
    fn executes_aligncopy_against_multiple_alignment_fixture() {
        let service = implemented_service();
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("aligncopy").expect("tool name should be valid"),
        )
        .with_arguments(vec![multiple_alignment_fixture().display().to_string()]);

        let response = service.invoke(request).expect("aligncopy should execute");
        match &response.result.payload {
            ResultPayload::Alignment(alignment) => {
                assert_eq!(alignment.row_count(), 3);
                assert_eq!(alignment.identifier(), Some("multiple-demo"));
            }
            payload => panic!("unexpected payload: {payload:?}"),
        }
    }

    #[test]
    fn executes_needle_against_singleton_fixtures() {
        let service = implemented_service();
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("needle").expect("tool name should be valid"),
        )
        .with_arguments(vec![
            needle_query_fixture().display().to_string(),
            needle_target_fixture().display().to_string(),
        ]);

        let response = service.invoke(request).expect("needle should execute");
        match &response.result.payload {
            ResultPayload::Alignment(alignment) => {
                assert_eq!(alignment.row_count(), 2);
                assert_eq!(alignment.column_count(), 4);
                assert_eq!(alignment.rows()[0].aligned(), "ACGT");
                assert_eq!(alignment.rows()[1].aligned(), "AC-T");
            }
            payload => panic!("unexpected payload: {payload:?}"),
        }
    }

    #[test]
    fn executes_water_against_singleton_fixtures() {
        let service = implemented_service();
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("water").expect("tool name should be valid"),
        )
        .with_arguments(vec![
            water_query_fixture().display().to_string(),
            water_target_fixture().display().to_string(),
        ]);

        let response = service.invoke(request).expect("water should execute");
        match &response.result.payload {
            ResultPayload::Alignment(alignment) => {
                assert_eq!(alignment.row_count(), 2);
                assert_eq!(alignment.rows()[0].aligned(), "ACGTA");
                assert_eq!(alignment.rows()[1].aligned(), "ACGTA");
            }
            payload => panic!("unexpected payload: {payload:?}"),
        }
        assert!(
            response
                .result
                .summary
                .lines
                .iter()
                .any(|line| line == "Query span: 3-7")
        );
        assert!(
            response
                .result
                .summary
                .lines
                .iter()
                .any(|line| line == "Target span: 3-7")
        );
    }

    #[test]
    fn needle_rejects_multi_record_query_input() {
        let service = implemented_service();
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("needle").expect("tool name should be valid"),
        )
        .with_arguments(vec![
            needleall_queries_fixture().display().to_string(),
            needle_target_fixture().display().to_string(),
        ]);

        assert!(service.invoke(request).is_err());
    }

    #[test]
    fn executes_needleall_against_multi_record_fixtures() {
        let service = implemented_service();
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("needleall").expect("tool name should be valid"),
        )
        .with_arguments(vec![
            needleall_queries_fixture().display().to_string(),
            needleall_targets_fixture().display().to_string(),
        ]);

        let response = service.invoke(request).expect("needleall should execute");
        match &response.result.payload {
            ResultPayload::TableReport(table) => {
                assert_eq!(table.rows.len(), 4);
                assert_eq!(table.rows[0][0], "q1");
                assert_eq!(table.rows[0][1], "t1");
                assert_eq!(table.rows[1][0], "q1");
                assert_eq!(table.rows[1][1], "t2");
                assert_eq!(table.rows[2][0], "q2");
                assert_eq!(table.rows[2][1], "t1");
            }
            payload => panic!("unexpected payload: {payload:?}"),
        }
    }

    #[test]
    fn executes_matcher_against_singleton_fixtures() {
        let service = implemented_service();
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("matcher").expect("tool name should be valid"),
        )
        .with_arguments(vec![
            needle_query_fixture().display().to_string(),
            needle_target_fixture().display().to_string(),
        ]);

        let response = service.invoke(request).expect("matcher should execute");
        match &response.result.payload {
            ResultPayload::TableReport(table) => {
                assert_eq!(table.rows.len(), 1);
                assert_eq!(table.rows[0][5], "3");
                assert_eq!(table.rows[0][6], "2");
                assert_eq!(table.rows[0][7], "1");
                assert_eq!(table.rows[0][8], "66");
            }
            payload => panic!("unexpected payload: {payload:?}"),
        }
    }

    #[test]
    fn executes_distmat_against_equal_length_fixture() {
        let service = implemented_service();
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("distmat").expect("tool name should be valid"),
        )
        .with_arguments(vec![sequence_fixture().display().to_string()]);

        let response = service.invoke(request).expect("distmat should execute");
        match &response.result.payload {
            ResultPayload::TableReport(table) => {
                assert_eq!(table.rows.len(), 3);
                assert_eq!(table.rows[0][0], "alpha");
                assert_eq!(table.rows[0][1], "0.000000");
                assert_eq!(table.rows[0][2], "0.750000");
                assert_eq!(table.rows[0][3], "1.000000");
            }
            payload => panic!("unexpected payload: {payload:?}"),
        }
    }

    #[test]
    fn executes_cons_against_multiple_alignment_fixture() {
        let service = implemented_service();
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("cons").expect("tool name should be valid"),
        )
        .with_arguments(vec![multiple_alignment_fixture().display().to_string()]);

        let response = service.invoke(request).expect("cons should execute");
        match &response.result.payload {
            ResultPayload::Sequence(sequence) => {
                assert_eq!(sequence.residues(), "ACNGT");
            }
            payload => panic!("unexpected payload: {payload:?}"),
        }
    }

    #[test]
    fn executes_consambig_against_multiple_alignment_fixture() {
        let service = implemented_service();
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("consambig").expect("tool name should be valid"),
        )
        .with_arguments(vec![multiple_alignment_fixture().display().to_string()]);

        let response = service.invoke(request).expect("consambig should execute");
        match &response.result.payload {
            ResultPayload::Sequence(sequence) => {
                assert_eq!(sequence.residues(), "ACYGT");
            }
            payload => panic!("unexpected payload: {payload:?}"),
        }
    }

    #[test]
    fn executes_aligncopypair_against_pairwise_alignment_fixture() {
        let service = implemented_service();
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("aligncopypair").expect("tool name should be valid"),
        )
        .with_arguments(vec![pairwise_alignment_fixture().display().to_string()]);

        let response = service
            .invoke(request)
            .expect("aligncopypair should execute");
        match &response.result.payload {
            ResultPayload::Alignment(alignment) => {
                assert!(alignment.is_pairwise());
                assert_eq!(alignment.column_count(), 5);
            }
            payload => panic!("unexpected payload: {payload:?}"),
        }
    }

    #[test]
    fn aligncopypair_rejects_multiple_alignment_fixture() {
        let service = implemented_service();
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("aligncopypair").expect("tool name should be valid"),
        )
        .with_arguments(vec![multiple_alignment_fixture().display().to_string()]);

        assert!(service.invoke(request).is_err());
    }

    #[test]
    fn executes_infoalign_against_multiple_alignment_fixture() {
        let service = implemented_service();
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("infoalign").expect("tool name should be valid"),
        )
        .with_arguments(vec![multiple_alignment_fixture().display().to_string()]);

        let response = service.invoke(request).expect("infoalign should execute");
        match &response.result.payload {
            ResultPayload::TableReport(table) => {
                assert_eq!(table.rows.len(), 3);
                assert_eq!(table.rows[0][2], "3");
                assert_eq!(table.rows[0][3], "5");
                assert_eq!(table.rows[0][7], "1");
            }
            payload => panic!("unexpected payload: {payload:?}"),
        }
    }

    #[test]
    fn executes_extractalign_against_multiple_alignment_fixture() {
        let service = implemented_service();
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("extractalign").expect("tool name should be valid"),
        )
        .with_arguments(vec![
            multiple_alignment_fixture().display().to_string(),
            "--row-id".to_owned(),
            "alpha".to_owned(),
            "--row".to_owned(),
            "3".to_owned(),
            "--start".to_owned(),
            "2".to_owned(),
            "--end".to_owned(),
            "4".to_owned(),
        ]);

        let response = service
            .invoke(request)
            .expect("extractalign should execute");
        match &response.result.payload {
            ResultPayload::Alignment(alignment) => {
                assert_eq!(alignment.row_count(), 2);
                assert_eq!(alignment.column_count(), 3);
                assert_eq!(alignment.rows()[0].identifier().accession(), "alpha");
                assert_eq!(alignment.rows()[0].aligned(), "C-G");
                assert_eq!(alignment.rows()[1].identifier().accession(), "gamma");
                assert_eq!(alignment.rows()[1].aligned(), "CCG");
            }
            payload => panic!("unexpected payload: {payload:?}"),
        }
    }

    #[test]
    fn executes_nthseq_against_real_fixture() {
        let service = implemented_service();
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("nthseq").expect("tool name should be valid"),
        )
        .with_arguments(vec![
            sequence_fixture().display().to_string(),
            "2".to_owned(),
        ]);

        let response = service.invoke(request).expect("nthseq should execute");
        match &response.result.payload {
            ResultPayload::Sequence(record) => {
                assert_eq!(record.identifier().accession(), "beta");
            }
            payload => panic!("unexpected payload: {payload:?}"),
        }
        assert_eq!(response.result.summary.lines[1], "Selected index: 2");
        assert_eq!(response.result.summary.lines[2], "Total records: 3");
        assert_eq!(response.result.summary.lines[3], "Output format: fasta");
    }

    #[test]
    fn rejects_nthseq_index_out_of_range() {
        let service = implemented_service();
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("nthseq").expect("tool name should be valid"),
        )
        .with_arguments(vec![
            sequence_fixture().display().to_string(),
            "4".to_owned(),
        ]);

        let error = service
            .invoke(request)
            .expect_err("out of range nthseq index should fail");
        assert!(error.to_string().contains("out of range"));
    }

    #[test]
    fn executes_skipseq_against_real_fixture() {
        let service = implemented_service();
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("skipseq").expect("tool name should be valid"),
        )
        .with_arguments(vec![
            sequence_fixture().display().to_string(),
            "1".to_owned(),
        ]);

        let response = service.invoke(request).expect("skipseq should execute");
        match &response.result.payload {
            ResultPayload::SequenceCollection(records) => {
                assert_eq!(records.len(), 2);
                assert_eq!(records[0].identifier().accession(), "beta");
            }
            payload => panic!("unexpected payload: {payload:?}"),
        }
        assert_eq!(response.result.summary.lines[1], "Skipped: 1");
        assert_eq!(response.result.summary.lines[2], "Input records: 3");
        assert_eq!(response.result.summary.lines[3], "Returned: 2");
    }

    #[test]
    fn executes_skipseq_beyond_end_as_empty_stream() {
        let service = implemented_service();
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("skipseq").expect("tool name should be valid"),
        )
        .with_arguments(vec![
            sequence_fixture().display().to_string(),
            "99".to_owned(),
        ]);

        let response = service.invoke(request).expect("skipseq should execute");
        match &response.result.payload {
            ResultPayload::SequenceCollection(records) => {
                assert!(records.is_empty());
            }
            payload => panic!("unexpected payload: {payload:?}"),
        }
        assert_eq!(response.result.summary.lines[1], "Skipped: 3");
        assert_eq!(response.result.summary.lines[2], "Input records: 3");
        assert_eq!(response.result.summary.lines[3], "Returned: 0");
    }

    #[test]
    fn executes_notseq_against_real_fixture() {
        let service = implemented_service();
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("notseq").expect("tool name should be valid"),
        )
        .with_arguments(vec![
            sequence_fixture().display().to_string(),
            "2".to_owned(),
        ]);

        let response = service.invoke(request).expect("notseq should execute");
        match &response.result.payload {
            ResultPayload::SequenceCollection(records) => {
                assert_eq!(records.len(), 2);
                assert_eq!(records[0].identifier().accession(), "alpha");
                assert_eq!(records[1].identifier().accession(), "gamma");
            }
            payload => panic!("unexpected payload: {payload:?}"),
        }
        assert_eq!(response.result.summary.lines[1], "Excluded index: 2");
        assert_eq!(response.result.summary.lines[2], "Input records: 3");
        assert_eq!(response.result.summary.lines[3], "Returned: 2");
    }

    #[test]
    fn rejects_notseq_index_out_of_range() {
        let service = implemented_service();
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("notseq").expect("tool name should be valid"),
        )
        .with_arguments(vec![
            sequence_fixture().display().to_string(),
            "4".to_owned(),
        ]);

        let error = service
            .invoke(request)
            .expect_err("out of range notseq index should fail");
        assert!(error.to_string().contains("out of range"));
    }

    #[test]
    fn executes_newseq_with_inline_content() {
        let service = implemented_service();
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("newseq").expect("tool name should be valid"),
        )
        .with_arguments(vec![
            "created".to_owned(),
            "ACGTAC".to_owned(),
            "--description".to_owned(),
            "created example".to_owned(),
            "--molecule".to_owned(),
            "dna".to_owned(),
        ]);

        let response = service.invoke(request).expect("newseq should execute");
        match &response.result.payload {
            ResultPayload::Sequence(record) => {
                assert_eq!(record.identifier().accession(), "created");
                assert_eq!(
                    record.metadata().description.as_deref(),
                    Some("created example")
                );
                assert_eq!(record.molecule().to_string(), "dna");
                assert_eq!(record.alphabet().to_string(), "DNA alphabet");
            }
            payload => panic!("unexpected payload: {payload:?}"),
        }
    }

    #[test]
    fn executes_newseq_with_explicit_protein_molecule() {
        let service = implemented_service();
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("newseq").expect("tool name should be valid"),
        )
        .with_arguments(vec![
            "prot-created".to_owned(),
            "m s t n".to_owned(),
            "--molecule".to_owned(),
            "protein".to_owned(),
        ]);

        let response = service.invoke(request).expect("newseq should execute");
        match &response.result.payload {
            ResultPayload::Sequence(record) => {
                assert_eq!(record.identifier().accession(), "prot-created");
                assert_eq!(record.residues(), "MSTN");
                assert_eq!(record.molecule().to_string(), "protein");
                assert_eq!(record.alphabet().to_string(), "protein alphabet");
            }
            payload => panic!("unexpected payload: {payload:?}"),
        }
    }

    #[test]
    fn newseq_rejects_invalid_residue_for_declared_molecule() {
        let service = implemented_service();
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("newseq").expect("tool name should be valid"),
        )
        .with_arguments(vec![
            "bad-dna".to_owned(),
            "ACGTZ".to_owned(),
            "--molecule".to_owned(),
            "dna".to_owned(),
        ]);

        let error = service
            .invoke(request)
            .expect_err("newseq should reject invalid dna residue");
        assert!(error.to_string().contains("invalid residue"));
    }

    #[test]
    fn executes_extractseq_against_real_fixture() {
        let service = implemented_service();
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("extractseq").expect("tool name should be valid"),
        )
        .with_arguments(vec![
            sequence_fixture().display().to_string(),
            "2".to_owned(),
            "3".to_owned(),
        ]);

        let response = service.invoke(request).expect("extractseq should execute");
        match &response.result.payload {
            ResultPayload::SequenceCollection(records) => {
                assert_eq!(records.len(), 3);
                assert_eq!(records[0].residues(), "CG");
                assert_eq!(records[1].residues(), "TT");
                assert_eq!(records[2].residues(), "GC");
            }
            payload => panic!("unexpected payload: {payload:?}"),
        }
    }

    #[test]
    fn extracts_full_length_region_against_real_fixture() {
        let service = implemented_service();
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("extractseq").expect("tool name should be valid"),
        )
        .with_arguments(vec![
            sequence_fixture().display().to_string(),
            "1".to_owned(),
            "4".to_owned(),
        ]);

        let response = service.invoke(request).expect("extractseq should execute");
        match &response.result.payload {
            ResultPayload::SequenceCollection(records) => {
                assert_eq!(records[0].residues(), "ACGT");
                assert_eq!(records[1].residues(), "TTTT");
                assert_eq!(records[2].residues(), "GGCC");
            }
            payload => panic!("unexpected payload: {payload:?}"),
        }
    }

    #[test]
    fn extracts_boundary_region_against_real_fixture() {
        let service = implemented_service();
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("extractseq").expect("tool name should be valid"),
        )
        .with_arguments(vec![
            sequence_fixture().display().to_string(),
            "1".to_owned(),
            "1".to_owned(),
        ]);

        let response = service.invoke(request).expect("extractseq should execute");
        match &response.result.payload {
            ResultPayload::SequenceCollection(records) => {
                assert_eq!(records[0].residues(), "A");
                assert_eq!(records[1].residues(), "T");
                assert_eq!(records[2].residues(), "G");
            }
            payload => panic!("unexpected payload: {payload:?}"),
        }
    }

    #[test]
    fn extractseq_rejects_out_of_range_coordinates() {
        let service = implemented_service();
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("extractseq").expect("tool name should be valid"),
        )
        .with_arguments(vec![
            sequence_fixture().display().to_string(),
            "2".to_owned(),
            "5".to_owned(),
        ]);

        let error = service
            .invoke(request)
            .expect_err("out-of-range coordinates should fail");
        assert_eq!(
            error.to_string(),
            "requested region 2..5 is out of range for sequence 'alpha' of length 4"
        );
    }

    #[test]
    fn extractseq_rejects_start_greater_than_end() {
        let service = implemented_service();
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("extractseq").expect("tool name should be valid"),
        )
        .with_arguments(vec![
            sequence_fixture().display().to_string(),
            "4".to_owned(),
            "2".to_owned(),
        ]);

        let error = service
            .invoke(request)
            .expect_err("inverted coordinates should fail");
        assert_eq!(
            error.to_string(),
            "extractseq requires 1-based inclusive coordinates with start <= end"
        );
    }

    #[test]
    fn executes_cutseq_against_real_fixture() {
        let service = implemented_service();
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("cutseq").expect("tool name should be valid"),
        )
        .with_arguments(vec![
            sequence_fixture().display().to_string(),
            "2".to_owned(),
        ]);

        let response = service.invoke(request).expect("cutseq should execute");
        match &response.result.payload {
            ResultPayload::SequenceCollection(records) => {
                assert_eq!(records.len(), 6);
                assert_eq!(records[0].identifier().accession(), "alpha.left");
                assert_eq!(records[0].residues(), "AC");
                assert_eq!(records[1].identifier().accession(), "alpha.right");
                assert_eq!(records[1].residues(), "GT");
                assert_eq!(records[2].identifier().accession(), "beta.left");
                assert_eq!(records[2].residues(), "TT");
                assert_eq!(records[3].identifier().accession(), "beta.right");
                assert_eq!(records[3].residues(), "TT");
                assert_eq!(records[4].identifier().accession(), "gamma.left");
                assert_eq!(records[4].residues(), "GG");
                assert_eq!(records[5].identifier().accession(), "gamma.right");
                assert_eq!(records[5].residues(), "CC");
            }
            payload => panic!("unexpected payload: {payload:?}"),
        }
    }

    #[test]
    fn executes_cutseq_at_first_boundary_position() {
        let service = implemented_service();
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("cutseq").expect("tool name should be valid"),
        )
        .with_arguments(vec![
            sequence_fixture().display().to_string(),
            "1".to_owned(),
        ]);

        let response = service.invoke(request).expect("cutseq should execute");
        match &response.result.payload {
            ResultPayload::SequenceCollection(records) => {
                assert_eq!(records[0].residues(), "A");
                assert_eq!(records[1].residues(), "CGT");
                assert_eq!(records[2].residues(), "T");
                assert_eq!(records[3].residues(), "TTT");
            }
            payload => panic!("unexpected payload: {payload:?}"),
        }
    }

    #[test]
    fn cutseq_rejects_non_interior_position() {
        let service = implemented_service();
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("cutseq").expect("tool name should be valid"),
        )
        .with_arguments(vec![
            sequence_fixture().display().to_string(),
            "4".to_owned(),
        ]);

        let error = service
            .invoke(request)
            .expect_err("position equal to record length must fail");
        assert_eq!(
            error.to_string(),
            "cut position 4 must be an interior position for sequence 'alpha' of length 4"
        );
    }

    #[test]
    fn executes_union_against_two_fixtures() {
        let service = implemented_service();
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("union").expect("tool name should be valid"),
        )
        .with_arguments(vec![
            sequence_fixture().display().to_string(),
            second_sequence_fixture().display().to_string(),
        ]);

        let response = service.invoke(request).expect("union should execute");
        match &response.result.payload {
            ResultPayload::SequenceCollection(records) => {
                assert_eq!(records.len(), 5);
                assert_eq!(records[0].identifier().accession(), "alpha");
                assert_eq!(records[4].identifier().accession(), "epsilon");
            }
            payload => panic!("unexpected payload: {payload:?}"),
        }
        assert_eq!(response.result.summary.lines[0], "Inputs: 2");
        assert_eq!(response.result.summary.lines[1], "Output records: 5");
        assert_eq!(
            response.result.summary.lines[2],
            "Ordering: preserve input order and per-input record order"
        );
        assert_eq!(
            response.result.summary.lines[3],
            "Duplicate policy: preserve duplicates exactly as read"
        );
    }

    #[test]
    fn rejects_union_with_too_few_inputs() {
        let service = implemented_service();
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("union").expect("tool name should be valid"),
        )
        .with_arguments(vec![sequence_fixture().display().to_string()]);

        let error = service
            .invoke(request)
            .expect_err("union should reject a single input");
        assert!(error.to_string().contains("Usage: emboss-rs union"));
    }

    #[test]
    fn executes_splitter_against_real_fixture() {
        let service = implemented_service();
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("splitter").expect("tool name should be valid"),
        )
        .with_arguments(vec![
            sequence_fixture().display().to_string(),
            "2".to_owned(),
        ]);

        let response = service.invoke(request).expect("splitter should execute");
        match &response.result.payload {
            ResultPayload::SequencePartitions(partitions) => {
                assert_eq!(partitions.len(), 2);
                assert_eq!(partitions[0].len(), 2);
                assert_eq!(partitions[1].len(), 1);
            }
            payload => panic!("unexpected payload: {payload:?}"),
        }
    }

    #[test]
    fn executes_degapseq_against_gapped_fixture() {
        let service = implemented_service();
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("degapseq").expect("tool name should be valid"),
        )
        .with_arguments(vec![gapped_sequence_fixture().display().to_string()]);

        let response = service.invoke(request).expect("degapseq should execute");
        match &response.result.payload {
            ResultPayload::SequenceCollection(records) => {
                assert_eq!(records[0].residues(), "ACGT");
                assert_eq!(records[1].residues(), "TTA");
            }
            payload => panic!("unexpected payload: {payload:?}"),
        }
    }

    #[test]
    fn executes_revseq_against_real_fixture() {
        let service = implemented_service();
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("revseq").expect("tool name should be valid"),
        )
        .with_arguments(vec![sequence_fixture().display().to_string()]);

        let response = service.invoke(request).expect("revseq should execute");
        match &response.result.payload {
            ResultPayload::SequenceCollection(records) => {
                assert_eq!(records[0].residues(), "ACGT");
                assert_eq!(records[1].residues(), "AAAA");
                assert_eq!(records[2].residues(), "CCGG");
            }
            payload => panic!("unexpected payload: {payload:?}"),
        }
    }

    #[test]
    fn executes_revseq_reverse_only_against_real_fixture() {
        let service = implemented_service();
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("revseq").expect("tool name should be valid"),
        )
        .with_arguments(vec![
            sequence_fixture().display().to_string(),
            "--reverse-only".to_owned(),
        ]);

        let response = service
            .invoke(request)
            .expect("reverse-only revseq should execute");
        match &response.result.payload {
            ResultPayload::SequenceCollection(records) => {
                assert_eq!(records[0].residues(), "TGCA");
                assert_eq!(records[1].residues(), "TTTT");
            }
            payload => panic!("unexpected payload: {payload:?}"),
        }
    }

    #[test]
    fn executes_revseq_against_unknown_fasta_fixture_as_reverse_only_in_auto_mode() {
        let service = implemented_service();
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("revseq").expect("tool name should be valid"),
        )
        .with_arguments(vec![protein_fixture().display().to_string()]);

        let response = service
            .invoke(request)
            .expect("unknown-molecule revseq should execute");
        match &response.result.payload {
            ResultPayload::SequenceCollection(records) => {
                assert_eq!(records[0].residues(), "*AM");
                assert_eq!(records[1].residues(), "SL");
            }
            payload => panic!("unexpected payload: {payload:?}"),
        }
    }

    #[test]
    fn revseq_rejects_explicit_complement_for_unknown_fasta_input() {
        let service = implemented_service();
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("revseq").expect("tool name should be valid"),
        )
        .with_arguments(vec![
            protein_fixture().display().to_string(),
            "--complement".to_owned(),
        ]);

        let error = service
            .invoke(request)
            .expect_err("unknown-molecule reverse-complement should fail");
        assert_eq!(
            error.to_string(),
            "reverse-complement is not supported for molecule kind unknown"
        );
    }

    #[test]
    fn executes_trimseq_against_real_fixture() {
        let service = implemented_service();
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("trimseq").expect("tool name should be valid"),
        )
        .with_arguments(vec![
            sequence_fixture().display().to_string(),
            "--left".to_owned(),
            "1".to_owned(),
            "--right".to_owned(),
            "1".to_owned(),
        ]);

        let response = service.invoke(request).expect("trimseq should execute");
        match &response.result.payload {
            ResultPayload::SequenceCollection(records) => {
                assert_eq!(records[0].residues(), "CG");
                assert_eq!(records[1].residues(), "TT");
            }
            payload => panic!("unexpected payload: {payload:?}"),
        }
    }

    #[test]
    fn executes_descseq_against_real_fixture() {
        let service = implemented_service();
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("descseq").expect("tool name should be valid"),
        )
        .with_arguments(vec![sequence_fixture().display().to_string()]);

        let response = service.invoke(request).expect("descseq should execute");
        match &response.result.payload {
            ResultPayload::TableReport(table) => {
                assert_eq!(
                    table.columns,
                    vec![
                        "ordinal",
                        "identifier",
                        "display_name",
                        "description",
                        "length",
                        "molecule",
                        "alphabet",
                        "feature_count",
                        "source",
                        "organism",
                        "topology",
                    ]
                );
                assert_eq!(table.rows[0][1], "alpha");
                assert_eq!(table.rows[0][3], "first example");
                assert_eq!(table.rows[0][4], "4");
                assert_eq!(table.rows[0][5], "dna");
                assert_eq!(table.rows[0][7], "0");
            }
            payload => panic!("unexpected payload: {payload:?}"),
        }
    }

    #[test]
    fn descseq_reports_annotation_aware_metadata() {
        let service = implemented_service();
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("descseq").expect("tool name should be valid"),
        )
        .with_arguments(vec![annotated_feature_fixture().display().to_string()]);

        let response = service.invoke(request).expect("descseq should execute");
        match &response.result.payload {
            ResultPayload::TableReport(table) => {
                assert_eq!(table.rows.len(), 1);
                assert_eq!(table.rows[0][1], "FEAT1");
                assert_eq!(table.rows[0][3], "Example annotated sequence.");
                assert_eq!(table.rows[0][7], "2");
                assert_eq!(table.rows[0][8], "Synthetic construct");
                assert_eq!(table.rows[0][9], "Synthetic construct");
                assert_eq!(table.rows[0][10], "-");
            }
            payload => panic!("unexpected payload: {payload:?}"),
        }
    }

    #[test]
    fn executes_maskseq_against_real_fixture() {
        let service = implemented_service();
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("maskseq").expect("tool name should be valid"),
        )
        .with_arguments(vec![
            sequence_fixture().display().to_string(),
            "2:3".to_owned(),
        ]);

        let response = service.invoke(request).expect("maskseq should execute");
        match &response.result.payload {
            ResultPayload::SequenceCollection(records) => {
                assert_eq!(records[0].residues(), "ANNT");
                assert_eq!(records[1].residues(), "TNNT");
                assert_eq!(records[2].residues(), "GNNC");
            }
            payload => panic!("unexpected payload: {payload:?}"),
        }
    }

    #[test]
    fn executes_maskseq_whole_sequence_against_real_fixture() {
        let service = implemented_service();
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("maskseq").expect("tool name should be valid"),
        )
        .with_arguments(vec![
            sequence_fixture().display().to_string(),
            "1:4".to_owned(),
        ]);

        let response = service.invoke(request).expect("maskseq should execute");
        match &response.result.payload {
            ResultPayload::SequenceCollection(records) => {
                assert_eq!(records[0].residues(), "NNNN");
                assert_eq!(records[1].residues(), "NNNN");
                assert_eq!(records[2].residues(), "NNNN");
            }
            payload => panic!("unexpected payload: {payload:?}"),
        }
    }

    #[test]
    fn executes_maskseq_uses_x_for_records_classified_as_protein() {
        let service = implemented_service();
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("maskseq").expect("tool name should be valid"),
        )
        .with_arguments(vec![
            protein_fixture().display().to_string(),
            "2:2".to_owned(),
        ]);

        let response = service.invoke(request).expect("maskseq should execute");
        match &response.result.payload {
            ResultPayload::SequenceCollection(records) => {
                assert_eq!(records[0].residues(), "MN*");
                assert_eq!(records[1].residues(), "LX");
            }
            payload => panic!("unexpected payload: {payload:?}"),
        }
    }

    #[test]
    fn maskseq_rejects_invalid_custom_mask_character_for_protein() {
        let service = implemented_service();
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("maskseq").expect("tool name should be valid"),
        )
        .with_arguments(vec![
            protein_fixture().display().to_string(),
            "2:2".to_owned(),
            "--mask-char".to_owned(),
            "?".to_owned(),
        ]);

        let error = service
            .invoke(request)
            .expect_err("invalid protein mask should fail");
        assert_eq!(
            error.to_string(),
            "maskseq mask character '?' is not valid for protein sequences"
        );
    }

    #[test]
    fn executes_maskfeat_against_annotated_fixture() {
        let service = implemented_service();
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("maskfeat").expect("tool name should be valid"),
        )
        .with_arguments(vec![
            annotated_feature_fixture().display().to_string(),
            "--kind".to_owned(),
            "gene".to_owned(),
        ]);

        let response = service.invoke(request).expect("maskfeat should execute");
        match &response.result.payload {
            ResultPayload::SequenceCollection(records) => {
                assert_eq!(records[0].residues(), "ANNNNNGTACGT");
                assert_eq!(records[0].features().len(), 2);
            }
            payload => panic!("unexpected payload: {payload:?}"),
        }
    }

    #[test]
    fn maskfeat_masks_multiple_selected_regions_in_stable_order() {
        let service = implemented_service();
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("maskfeat").expect("tool name should be valid"),
        )
        .with_arguments(vec![annotated_feature_fixture().display().to_string()]);

        let response = service.invoke(request).expect("maskfeat should execute");
        match &response.result.payload {
            ResultPayload::SequenceCollection(records) => {
                assert_eq!(records.len(), 1);
                assert_eq!(records[0].residues(), "ANNNNNGNNNGT");
                assert_eq!(records[0].features().len(), 2);
            }
            payload => panic!("unexpected payload: {payload:?}"),
        }
    }

    #[test]
    fn maskfeat_rejects_no_matching_features() {
        let service = implemented_service();
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("maskfeat").expect("tool name should be valid"),
        )
        .with_arguments(vec![
            annotated_feature_fixture().display().to_string(),
            "--name".to_owned(),
            "missing".to_owned(),
        ]);

        let error = service
            .invoke(request)
            .expect_err("missing selector should fail");
        assert_eq!(
            error.to_string(),
            "maskfeat did not find any features matching the requested selector"
        );
    }

    #[test]
    fn maskfeat_rejects_complex_locations() {
        let service = implemented_service();
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("maskfeat").expect("tool name should be valid"),
        )
        .with_arguments(vec![annotated_complex_fixture().display().to_string()]);

        let error = service
            .invoke(request)
            .expect_err("complex locations should be rejected");
        assert_eq!(
            error.to_string(),
            "feature extraction currently supports only simple single-span locations"
        );
    }

    #[test]
    fn executes_extractfeat_against_annotated_fixture() {
        let service = implemented_service();
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("extractfeat").expect("tool name should be valid"),
        )
        .with_arguments(vec![
            annotated_feature_fixture().display().to_string(),
            "--kind".to_owned(),
            "gene".to_owned(),
        ]);

        let response = service.invoke(request).expect("extractfeat should execute");
        match &response.result.payload {
            ResultPayload::SequenceCollection(records) => {
                assert_eq!(records.len(), 1);
                assert_eq!(records[0].identifier().accession(), "FEAT1:2-6:geneA");
                assert_eq!(records[0].residues(), "CGTAC");
                assert_eq!(records[0].features()[0].location.bounds().start(), 0);
            }
            payload => panic!("unexpected payload: {payload:?}"),
        }
    }

    #[test]
    fn extractfeat_supports_qualifier_selection_and_stable_order() {
        let service = implemented_service();
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("extractfeat").expect("tool name should be valid"),
        )
        .with_arguments(vec![
            annotated_feature_fixture().display().to_string(),
            "--qualifier".to_owned(),
            "product=short peptide".to_owned(),
        ]);

        let response = service.invoke(request).expect("extractfeat should execute");
        match &response.result.payload {
            ResultPayload::SequenceCollection(records) => {
                assert_eq!(records.len(), 1);
                assert_eq!(records[0].identifier().accession(), "FEAT1:8-10:cdsA");
                assert_eq!(records[0].residues(), "TAC");
                assert_eq!(records[0].features()[0].location.bounds().start(), 0);
            }
            payload => panic!("unexpected payload: {payload:?}"),
        }
    }

    #[test]
    fn extractfeat_rejects_no_matching_features() {
        let service = implemented_service();
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("extractfeat").expect("tool name should be valid"),
        )
        .with_arguments(vec![
            annotated_feature_fixture().display().to_string(),
            "--name".to_owned(),
            "missing".to_owned(),
        ]);

        let error = service
            .invoke(request)
            .expect_err("missing selector should fail");
        assert_eq!(
            error.to_string(),
            "extractfeat did not find any features matching the requested selector"
        );
    }

    #[test]
    fn extractfeat_rejects_complex_locations() {
        let service = implemented_service();
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("extractfeat").expect("tool name should be valid"),
        )
        .with_arguments(vec![annotated_complex_fixture().display().to_string()]);

        assert!(service.invoke(request).is_err());
    }

    #[test]
    fn executes_featcopy_against_matching_fixtures() {
        let service = implemented_service();
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("featcopy").expect("tool name should be valid"),
        )
        .with_arguments(vec![
            annotated_feature_fixture().display().to_string(),
            featcopy_target_fixture().display().to_string(),
            "--kind".to_owned(),
            "gene".to_owned(),
        ]);

        let response = service.invoke(request).expect("featcopy should execute");
        match &response.result.payload {
            ResultPayload::SequenceCollection(records) => {
                assert_eq!(records.len(), 1);
                assert_eq!(records[0].identifier().accession(), "FEAT1");
                assert_eq!(records[0].features().len(), 1);
                assert_eq!(records[0].features()[0].name.as_deref(), Some("geneA"));
            }
            payload => panic!("unexpected payload: {payload:?}"),
        }
    }

    #[test]
    fn featcopy_preserves_feature_metadata_under_qualifier_selection() {
        let service = implemented_service();
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("featcopy").expect("tool name should be valid"),
        )
        .with_arguments(vec![
            annotated_feature_fixture().display().to_string(),
            featcopy_target_fixture().display().to_string(),
            "--qualifier".to_owned(),
            "product=short peptide".to_owned(),
        ]);

        let response = service.invoke(request).expect("featcopy should execute");
        match &response.result.payload {
            ResultPayload::SequenceCollection(records) => {
                assert_eq!(records.len(), 1);
                assert_eq!(records[0].features().len(), 1);
                assert_eq!(records[0].features()[0].name.as_deref(), Some("cdsA"));
                assert_eq!(
                    records[0].features()[0]
                        .qualifiers
                        .get("product")
                        .map(String::as_str),
                    Some("short peptide")
                );
            }
            payload => panic!("unexpected payload: {payload:?}"),
        }
    }

    #[test]
    fn featcopy_rejects_no_matching_features() {
        let service = implemented_service();
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("featcopy").expect("tool name should be valid"),
        )
        .with_arguments(vec![
            annotated_feature_fixture().display().to_string(),
            featcopy_target_fixture().display().to_string(),
            "--name".to_owned(),
            "missing".to_owned(),
        ]);

        let error = service
            .invoke(request)
            .expect_err("missing selector should fail");
        assert_eq!(
            error.to_string(),
            "featcopy did not find any features matching the requested selector"
        );
    }

    #[test]
    fn featcopy_rejects_identifier_mismatch() {
        let service = implemented_service();
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("featcopy").expect("tool name should be valid"),
        )
        .with_arguments(vec![
            annotated_feature_fixture().display().to_string(),
            featcopy_mismatch_fixture().display().to_string(),
        ]);

        assert!(service.invoke(request).is_err());
    }

    #[test]
    fn executes_coderet_against_annotated_fixture() {
        let service = implemented_service();
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("coderet").expect("tool name should be valid"),
        )
        .with_arguments(vec![annotated_feature_fixture().display().to_string()]);

        let response = service.invoke(request).expect("coderet should execute");
        match &response.result.payload {
            ResultPayload::SequenceCollection(records) => {
                assert_eq!(records.len(), 1);
                assert_eq!(records[0].identifier().accession(), "FEAT1:8-10:cdsA");
                assert_eq!(records[0].residues(), "TAC");
            }
            payload => panic!("unexpected payload: {payload:?}"),
        }
    }

    #[test]
    fn coderet_supports_strict_translation() {
        let service = implemented_service();
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("coderet").expect("tool name should be valid"),
        )
        .with_arguments(vec![
            annotated_feature_fixture().display().to_string(),
            "--translate".to_owned(),
        ]);

        let response = service
            .invoke(request)
            .expect("translated coderet should execute");
        match &response.result.payload {
            ResultPayload::SequenceCollection(records) => {
                assert_eq!(records.len(), 1);
                assert_eq!(records[0].molecule(), MoleculeKind::Protein);
                assert_eq!(records[0].identifier().accession(), "FEAT1:8-10:cdsA.pep");
                assert_eq!(records[0].residues(), "Y");
            }
            payload => panic!("unexpected payload: {payload:?}"),
        }
    }

    #[test]
    fn executes_featmerge_against_matching_fixtures() {
        let service = implemented_service();
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("featmerge").expect("tool name should be valid"),
        )
        .with_arguments(vec![
            annotated_feature_fixture().display().to_string(),
            featmerge_right_fixture().display().to_string(),
        ]);

        let response = service.invoke(request).expect("featmerge should execute");
        match &response.result.payload {
            ResultPayload::SequenceCollection(records) => {
                assert_eq!(records.len(), 1);
                assert_eq!(records[0].features().len(), 3);
                assert_eq!(records[0].features()[2].name.as_deref(), Some("exonA"));
            }
            payload => panic!("unexpected payload: {payload:?}"),
        }
    }

    #[test]
    fn featmerge_rejects_when_no_selected_features_are_admitted() {
        let service = implemented_service();
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("featmerge").expect("tool name should be valid"),
        )
        .with_arguments(vec![
            annotated_feature_fixture().display().to_string(),
            featmerge_right_fixture().display().to_string(),
            "--kind".to_owned(),
            "gene".to_owned(),
        ]);

        let error = service
            .invoke(request)
            .expect_err("fully duplicated right-side selection should fail");
        assert_eq!(
            error.to_string(),
            "featmerge did not admit any selected right-hand features after deterministic deduplication"
        );
    }

    #[test]
    fn executes_featreport_against_annotated_fixture() {
        let service = implemented_service();
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("featreport").expect("tool name should be valid"),
        )
        .with_arguments(vec![annotated_feature_fixture().display().to_string()]);

        let response = service.invoke(request).expect("featreport should execute");
        match &response.result.payload {
            ResultPayload::TableReport(table) => {
                assert_eq!(table.columns[0], "record");
                assert_eq!(table.rows.len(), 2);
                assert_eq!(table.rows[0][0], "FEAT1");
                assert_eq!(table.rows[0][1], "gene");
                assert_eq!(table.rows[1][1], "CDS");
            }
            payload => panic!("unexpected payload: {payload:?}"),
        }
    }

    #[test]
    fn executes_feattext_against_annotated_fixture() {
        let service = implemented_service();
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("feattext").expect("tool name should be valid"),
        )
        .with_arguments(vec![annotated_feature_fixture().display().to_string()]);

        let response = service.invoke(request).expect("feattext should execute");
        match &response.result.payload {
            ResultPayload::TextReport(report) => {
                assert!(report.body.contains("ID   FEAT1"));
                assert!(report.body.contains("FEATURES             Location/Qualifiers"));
                assert!(report.body.contains("/product=\"short peptide\""));
            }
            payload => panic!("unexpected payload: {payload:?}"),
        }
    }

    #[test]
    fn executes_backtranseq_against_protein_fixture() {
        let service = implemented_service();
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("backtranseq").expect("tool name should be valid"),
        )
        .with_arguments(vec![protein_fixture().display().to_string()]);

        let response = service.invoke(request).expect("backtranseq should execute");
        match &response.result.payload {
            ResultPayload::SequenceCollection(records) => {
                assert_eq!(records[0].molecule(), MoleculeKind::Dna);
                assert_eq!(records[0].residues(), "ATGGCTTAA");
                assert_eq!(records[1].residues(), "CTTTCT");
            }
            payload => panic!("unexpected payload: {payload:?}"),
        }
    }

    #[test]
    fn executes_backtranambig_against_protein_fixture() {
        let service = implemented_service();
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("backtranambig").expect("tool name should be valid"),
        )
        .with_arguments(vec![protein_fixture().display().to_string()]);

        let response = service
            .invoke(request)
            .expect("backtranambig should execute");
        match &response.result.payload {
            ResultPayload::SequenceCollection(records) => {
                assert_eq!(records[0].molecule(), MoleculeKind::Dna);
                assert_eq!(records[0].residues(), "ATGGCNTAR");
                assert_eq!(records[1].residues(), "YTNWSN");
            }
            payload => panic!("unexpected payload: {payload:?}"),
        }
    }

    #[test]
    fn executes_transeq_against_matching_coding_fixture() {
        let service = implemented_service();
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("transeq").expect("tool name should be valid"),
        )
        .with_arguments(vec![
            checktrans_nucleotide_fixture().display().to_string(),
            "--frame".to_owned(),
            "1".to_owned(),
        ]);

        let response = service.invoke(request).expect("transeq should execute");
        match &response.result.payload {
            ResultPayload::SequenceCollection(records) => {
                assert_eq!(records.len(), 2);
                assert_eq!(records[0].identifier().accession(), "cdsA.frame1");
                assert_eq!(records[0].residues(), "MA*");
                assert_eq!(records[1].identifier().accession(), "cdsB.frame1");
                assert_eq!(records[1].residues(), "LS");
            }
            payload => panic!("unexpected payload: {payload:?}"),
        }
        assert!(
            response.result.summary.lines[0]
                .ends_with("crates/emboss-tools/tests/fixtures/checktrans_nucleotide.fasta")
        );
        assert_eq!(response.result.summary.lines[1], "Frame selection: frame 1");
        assert_eq!(response.result.summary.lines[2], "Genetic code: standard");
    }

    #[test]
    fn executes_getorf_against_orf_fixture() {
        let service = implemented_service();
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("getorf").expect("tool name should be valid"),
        )
        .with_arguments(vec![getorf_fixture().display().to_string()]);

        let response = service.invoke(request).expect("getorf should execute");
        match &response.result.payload {
            ResultPayload::SequenceCollection(records) => {
                assert_eq!(records.len(), 2);
                assert_eq!(records[0].residues(), "ATGAAATAG");
                assert_eq!(records[1].residues(), "ATGCCCTAA");
            }
            payload => panic!("unexpected payload: {payload:?}"),
        }
        assert_eq!(response.result.summary.lines[1], "Frame policy: forward frames 1-3");
        assert_eq!(
            response.result.summary.lines[2],
            "ORF policy: ATG start to first in-frame stop, stop codon included"
        );
        assert_eq!(response.result.summary.lines[3], "ORFs: 2");
    }

    #[test]
    fn executes_prettyseq_against_matching_coding_fixture() {
        let service = implemented_service();
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("prettyseq").expect("tool name should be valid"),
        )
        .with_arguments(vec![
            checktrans_nucleotide_fixture().display().to_string(),
            "--width".to_owned(),
            "9".to_owned(),
        ]);

        let response = service.invoke(request).expect("prettyseq should execute");
        match &response.result.payload {
            ResultPayload::TextReport(report) => {
                assert!(report.body.contains(">cdsA"));
                assert!(report.body.contains("FRAME 1"));
                assert!(report.body.contains("NT     1 ATGGCTTAA     9"));
                assert!(report.body.contains("AA     1 MA*     3"));
            }
            payload => panic!("unexpected payload: {payload:?}"),
        }
        assert_eq!(response.result.summary.lines[1], "Frame: 1");
        assert_eq!(response.result.summary.lines[2], "Width: 9");
    }

    #[test]
    fn executes_tranalign_against_coding_fixture() {
        let service = implemented_service();
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("tranalign").expect("tool name should be valid"),
        )
        .with_arguments(vec![
            tranalign_protein_alignment_fixture().display().to_string(),
            checktrans_nucleotide_fixture().display().to_string(),
        ]);

        let response = service.invoke(request).expect("tranalign should execute");
        match &response.result.payload {
            ResultPayload::Alignment(alignment) => {
                assert_eq!(alignment.row_count(), 2);
                assert_eq!(alignment.rows()[0].identifier().accession(), "cdsA");
                assert_eq!(alignment.rows()[0].aligned(), "ATGGCT---");
                assert_eq!(alignment.rows()[1].aligned(), "---CTTTCT");
            }
            payload => panic!("unexpected payload: {payload:?}"),
        }
        assert_eq!(
            response.result.summary.lines[2],
            "Compatibility: exact identifier pairing and strict frame-1 translation"
        );
    }

    #[test]
    fn executes_checktrans_against_matching_fixture_pair() {
        let service = implemented_service();
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("checktrans").expect("tool name should be valid"),
        )
        .with_arguments(vec![
            checktrans_nucleotide_fixture().display().to_string(),
            checktrans_protein_fixture().display().to_string(),
        ]);

        let response = service.invoke(request).expect("checktrans should execute");
        match &response.result.payload {
            ResultPayload::TableReport(table) => {
                assert_eq!(table.rows.len(), 2);
                assert_eq!(table.rows[0][2], "true");
                assert_eq!(table.rows[1][2], "true");
            }
            payload => panic!("unexpected payload: {payload:?}"),
        }
    }

    #[test]
    fn checktrans_reports_mismatch_for_inconsistent_translation() {
        let service = implemented_service();
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("checktrans").expect("tool name should be valid"),
        )
        .with_arguments(vec![
            checktrans_nucleotide_fixture().display().to_string(),
            checktrans_mismatch_fixture().display().to_string(),
        ]);

        let response = service.invoke(request).expect("checktrans should execute");
        match &response.result.payload {
            ResultPayload::TableReport(table) => {
                assert_eq!(table.rows[0][2], "false");
                assert!(table.rows[0][5].contains("translation mismatch"));
            }
            payload => panic!("unexpected payload: {payload:?}"),
        }
    }

    #[test]
    fn checktrans_rejects_invalid_codon_input() {
        let service = implemented_service();
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("checktrans").expect("tool name should be valid"),
        )
        .with_arguments(vec![
            checktrans_invalid_codon_fixture().display().to_string(),
            checktrans_protein_fixture().display().to_string(),
        ]);

        assert!(service.invoke(request).is_err());
    }

    #[test]
    fn executes_fuzznuc_against_nucleotide_fixture() {
        let service = implemented_service();
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("fuzznuc").expect("tool name should be valid"),
        )
        .with_arguments(vec![
            nucleotide_pattern_fixture().display().to_string(),
            "ACGN".to_owned(),
        ]);

        let response = service.invoke(request).expect("fuzznuc should execute");
        match &response.result.payload {
            ResultPayload::TableReport(table) => {
                assert_eq!(table.rows.len(), 2);
                assert_eq!(table.rows[0][0], "nucA");
                assert_eq!(table.rows[0][1], "ACGN");
                assert_eq!(table.rows[0][2], "forward");
                assert_eq!(table.rows[0][3], "1");
                assert_eq!(table.rows[0][4], "4");
                assert_eq!(table.rows[1][5], "ACGT");
            }
            payload => panic!("unexpected payload: {payload:?}"),
        }
        assert!(
            response.result.summary.lines[0]
                .ends_with("crates/emboss-tools/tests/fixtures/nucleotide_pattern_records.fasta")
        );
        assert_eq!(response.result.summary.lines[1], "Pattern: ACGN");
        assert_eq!(
            response.result.summary.lines[2],
            "Coordinate convention: 1-based inclusive"
        );
        assert_eq!(
            response.result.summary.lines[3],
            "Strand policy: forward only"
        );
        assert_eq!(response.result.summary.lines[4], "Hits: 2");
    }

    #[test]
    fn executes_fuzzpro_against_protein_fixture() {
        let service = implemented_service();
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("fuzzpro").expect("tool name should be valid"),
        )
        .with_arguments(vec![
            protein_fixture().display().to_string(),
            "MX".to_owned(),
        ]);

        let response = service.invoke(request).expect("fuzzpro should execute");
        match &response.result.payload {
            ResultPayload::TableReport(table) => {
                assert_eq!(table.rows.len(), 1);
                assert_eq!(table.rows[0][0], "protA");
                assert_eq!(table.rows[0][1], "MX");
                assert_eq!(table.rows[0][2], "1");
                assert_eq!(table.rows[0][3], "2");
                assert_eq!(table.rows[0][4], "MA");
            }
            payload => panic!("unexpected payload: {payload:?}"),
        }
        assert!(
            response.result.summary.lines[0]
                .ends_with("crates/emboss-tools/tests/fixtures/protein_records.fasta")
        );
        assert_eq!(response.result.summary.lines[1], "Pattern: MX");
        assert_eq!(
            response.result.summary.lines[2],
            "Coordinate convention: 1-based inclusive"
        );
        assert_eq!(
            response.result.summary.lines[3],
            "Pattern syntax: exact residues with X wildcard"
        );
        assert_eq!(response.result.summary.lines[4], "Hits: 1");
    }

    #[test]
    fn executes_fuzztran_against_matching_coding_fixture() {
        let service = implemented_service();
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("fuzztran").expect("tool name should be valid"),
        )
        .with_arguments(vec![
            checktrans_nucleotide_fixture().display().to_string(),
            "MA".to_owned(),
        ]);

        let response = service.invoke(request).expect("fuzztran should execute");
        match &response.result.payload {
            ResultPayload::TableReport(table) => {
                assert_eq!(table.rows.len(), 1);
                assert_eq!(table.rows[0][0], "cdsA");
                assert_eq!(table.rows[0][1], "MA");
                assert_eq!(table.rows[0][2], "1");
                assert_eq!(table.rows[0][3], "1");
                assert_eq!(table.rows[0][4], "2");
                assert_eq!(table.rows[0][5], "1");
                assert_eq!(table.rows[0][6], "6");
                assert_eq!(table.rows[0][7], "MA");
            }
            payload => panic!("unexpected payload: {payload:?}"),
        }
        assert!(
            response.result.summary.lines[0]
                .ends_with("crates/emboss-tools/tests/fixtures/checktrans_nucleotide.fasta")
        );
        assert_eq!(response.result.summary.lines[1], "Pattern: MA");
        assert_eq!(
            response.result.summary.lines[2],
            "Frame policy: forward frames 1-3"
        );
        assert_eq!(
            response.result.summary.lines[3],
            "Coordinate convention: 1-based inclusive"
        );
        assert_eq!(response.result.summary.lines[4], "Hits: 1");
    }

    #[test]
    fn rejects_invalid_fuzznuc_pattern() {
        let service = implemented_service();
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("fuzznuc").expect("tool name should be valid"),
        )
        .with_arguments(vec![
            nucleotide_pattern_fixture().display().to_string(),
            "AC?".to_owned(),
        ]);

        assert!(service.invoke(request).is_err());
    }

    #[test]
    fn rejects_invalid_fuzzpro_pattern() {
        let service = implemented_service();
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("fuzzpro").expect("tool name should be valid"),
        )
        .with_arguments(vec![
            protein_fixture().display().to_string(),
            "M?".to_owned(),
        ]);

        assert!(service.invoke(request).is_err());
    }

    #[test]
    fn rejects_invalid_fuzztran_pattern() {
        let service = implemented_service();
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("fuzztran").expect("tool name should be valid"),
        )
        .with_arguments(vec![
            checktrans_nucleotide_fixture().display().to_string(),
            "M?".to_owned(),
        ]);

        assert!(service.invoke(request).is_err());
    }

    #[test]
    fn rejects_nucleotide_input_for_fuzzpro() {
        let service = implemented_service();
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("fuzzpro").expect("tool name should be valid"),
        )
        .with_arguments(vec![
            sequence_fixture().display().to_string(),
            "MX".to_owned(),
        ]);

        let error = service
            .invoke(request)
            .expect_err("nucleotide input should fail for fuzzpro");
        assert!(error.to_string().contains("expects protein input"));
    }

    #[test]
    fn rejects_protein_input_for_fuzztran() {
        let service = implemented_service();
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("fuzztran").expect("tool name should be valid"),
        )
        .with_arguments(vec![
            protein_fixture().display().to_string(),
            "MA".to_owned(),
        ]);

        let error = service
            .invoke(request)
            .expect_err("protein input should fail for fuzztran");
        assert!(error.to_string().contains("invalid codon"));
    }

    #[test]
    fn reports_overlapping_fuzzpro_hits() {
        let service = implemented_service();
        let temp = std::env::temp_dir().join(format!(
            "emboss-rs-fuzzpro-overlap-{}.fasta",
            std::process::id()
        ));
        std::fs::write(&temp, ">ovl\nMAMAM\n").expect("overlap fixture should be written");

        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("fuzzpro").expect("tool name should be valid"),
        )
        .with_arguments(vec![temp.display().to_string(), "MAM".to_owned()]);

        let response = service.invoke(request).expect("fuzzpro should execute");
        std::fs::remove_file(temp).ok();

        match &response.result.payload {
            ResultPayload::TableReport(table) => {
                assert_eq!(table.rows.len(), 2);
                assert_eq!(table.rows[0][0], "ovl");
                assert_eq!(table.rows[0][2], "1");
                assert_eq!(table.rows[0][3], "3");
                assert_eq!(table.rows[1][2], "3");
                assert_eq!(table.rows[1][3], "5");
            }
            payload => panic!("unexpected payload: {payload:?}"),
        }
    }

    #[test]
    fn reports_overlapping_fuzztran_hits() {
        let service = implemented_service();
        let temp = std::env::temp_dir().join(format!(
            "emboss-rs-fuzztran-overlap-{}.fasta",
            std::process::id()
        ));
        std::fs::write(&temp, ">ovl\nATGGCTATGGCTATG\n")
            .expect("overlap fixture should be written");

        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("fuzztran").expect("tool name should be valid"),
        )
        .with_arguments(vec![temp.display().to_string(), "MAM".to_owned()]);

        let response = service.invoke(request).expect("fuzztran should execute");
        std::fs::remove_file(temp).ok();

        match &response.result.payload {
            ResultPayload::TableReport(table) => {
                assert_eq!(table.rows.len(), 2);
                assert_eq!(table.rows[0][0], "ovl");
                assert_eq!(table.rows[0][2], "1");
                assert_eq!(table.rows[0][3], "1");
                assert_eq!(table.rows[0][4], "3");
                assert_eq!(table.rows[0][5], "1");
                assert_eq!(table.rows[0][6], "9");
                assert_eq!(table.rows[1][3], "3");
                assert_eq!(table.rows[1][4], "5");
                assert_eq!(table.rows[1][5], "7");
                assert_eq!(table.rows[1][6], "15");
            }
            payload => panic!("unexpected payload: {payload:?}"),
        }
    }

    #[test]
    fn rejects_ambiguous_translation_for_fuzztran() {
        let service = implemented_service();
        let temp = std::env::temp_dir().join(format!(
            "emboss-rs-fuzztran-ambiguous-{}.fasta",
            std::process::id()
        ));
        std::fs::write(&temp, ">amb\nATGNNNTAA\n").expect("ambiguous fixture should be written");

        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("fuzztran").expect("tool name should be valid"),
        )
        .with_arguments(vec![temp.display().to_string(), "MX".to_owned()]);

        let error = service
            .invoke(request)
            .expect_err("ambiguous translation should fail");
        std::fs::remove_file(temp).ok();
        assert!(error.to_string().contains("invalid codon"));
    }

    #[test]
    fn rejects_protein_input_for_fuzznuc() {
        let service = implemented_service();
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("fuzznuc").expect("tool name should be valid"),
        )
        .with_arguments(vec![
            protein_fixture().display().to_string(),
            "ACG".to_owned(),
        ]);

        let error = service
            .invoke(request)
            .expect_err("protein input should fail for fuzznuc");
        assert!(error.to_string().contains("expects nucleotide input"));
    }

    #[test]
    fn reports_overlapping_fuzznuc_hits() {
        let service = implemented_service();
        let temp = std::env::temp_dir().join(format!(
            "emboss-rs-fuzznuc-overlap-{}.fasta",
            std::process::id()
        ));
        std::fs::write(&temp, ">ovl\nATATA\n").expect("overlap fixture should be written");

        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("fuzznuc").expect("tool name should be valid"),
        )
        .with_arguments(vec![temp.display().to_string(), "ATA".to_owned()]);

        let response = service.invoke(request).expect("fuzznuc should execute");
        std::fs::remove_file(temp).ok();

        match &response.result.payload {
            ResultPayload::TableReport(table) => {
                assert_eq!(table.rows.len(), 2);
                assert_eq!(table.rows[0][0], "ovl");
                assert_eq!(table.rows[0][3], "1");
                assert_eq!(table.rows[0][4], "3");
                assert_eq!(table.rows[1][3], "3");
                assert_eq!(table.rows[1][4], "5");
            }
            payload => panic!("unexpected payload: {payload:?}"),
        }
    }

    #[test]
    fn executes_compseq_against_nucleotide_fixture() {
        let service = implemented_service();
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("compseq").expect("tool name should be valid"),
        )
        .with_arguments(vec![nucleotide_pattern_fixture().display().to_string()]);

        let response = service.invoke(request).expect("compseq should execute");
        match &response.result.payload {
            ResultPayload::TableReport(table) => {
                assert_eq!(
                    table.columns,
                    vec![
                        "scope",
                        "record",
                        "molecule",
                        "length",
                        "residue",
                        "count",
                        "frequency"
                    ]
                );
                assert!(table.rows.iter().any(|row| {
                    row[0] == "record" && row[1] == "nucA" && row[4] == "N" && row[5] == "1"
                }));
                assert!(
                    table
                        .rows
                        .iter()
                        .any(|row| { row[0] == "aggregate" && row[4] == "C" && row[5] == "4" })
                );
            }
            payload => panic!("unexpected payload: {payload:?}"),
        }
        assert!(
            response.result.summary.lines[0]
                .ends_with("crates/emboss-tools/tests/fixtures/nucleotide_pattern_records.fasta")
        );
        assert_eq!(
            response.result.summary.lines[1],
            "Scope: per-record plus aggregate summary"
        );
        assert_eq!(
            response.result.summary.lines[2],
            "Gap policy: '-' is ignored"
        );
        assert_eq!(
            response.result.summary.lines[3],
            "Frequency denominator: all non-gap normalized residue symbols"
        );
        assert_eq!(response.result.summary.lines[4], "Records: 2");
    }

    #[test]
    fn executes_compseq_against_protein_fixture() {
        let service = implemented_service();
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("compseq").expect("tool name should be valid"),
        )
        .with_arguments(vec![protein_fixture().display().to_string()]);

        let response = service.invoke(request).expect("compseq should execute");
        match &response.result.payload {
            ResultPayload::TableReport(table) => {
                assert!(table.rows.iter().any(|row| {
                    row[0] == "record" && row[1] == "protA" && row[4] == "*" && row[5] == "1"
                }));
                assert!(
                    table
                        .rows
                        .iter()
                        .any(|row| { row[0] == "aggregate" && row[4] == "L" && row[5] == "1" })
                );
            }
            payload => panic!("unexpected payload: {payload:?}"),
        }
    }

    #[test]
    fn executes_complex_against_whole_sequence_fixture() {
        let service = implemented_service();
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("complex").expect("tool name should be valid"),
        )
        .with_arguments(vec![
            complex_fixture().display().to_string(),
            "--k-min".to_owned(),
            "1".to_owned(),
            "--k-max".to_owned(),
            "2".to_owned(),
        ]);

        let response = service.invoke(request).expect("complex should execute");
        match &response.result.payload {
            ResultPayload::TableReport(table) => {
                assert_eq!(table.rows.len(), 2);
                assert_eq!(table.rows[0][0], "record");
                assert_eq!(table.rows[0][1], "low");
                assert_eq!(table.rows[1][1], "high");
            }
            payload => panic!("unexpected payload: {payload:?}"),
        }
    }

    #[test]
    fn executes_complex_against_windowed_fixture() {
        let service = implemented_service();
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("complex").expect("tool name should be valid"),
        )
        .with_arguments(vec![
            complex_fixture().display().to_string(),
            "--k-min".to_owned(),
            "1".to_owned(),
            "--k-max".to_owned(),
            "2".to_owned(),
            "--window".to_owned(),
            "4".to_owned(),
            "--step".to_owned(),
            "2".to_owned(),
        ]);

        let response = service.invoke(request).expect("complex should execute");
        match &response.result.payload {
            ResultPayload::TableReport(table) => {
                assert!(table.rows.len() > 2);
                assert_eq!(table.rows[2][0], "window");
                assert_eq!(table.rows[2][5], "1");
                assert_eq!(table.rows[2][6], "4");
            }
            payload => panic!("unexpected payload: {payload:?}"),
        }
    }

    #[test]
    fn complex_rejects_unsupported_symbols() {
        let service = implemented_service();
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("complex").expect("tool name should be valid"),
        )
        .with_arguments(vec![
            complex_invalid_fixture().display().to_string(),
            "--k-min".to_owned(),
            "1".to_owned(),
            "--k-max".to_owned(),
            "2".to_owned(),
        ]);

        assert!(service.invoke(request).is_err());
    }

    #[test]
    fn executes_charge_against_protein_fixture() {
        let service = implemented_service();
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("charge").expect("tool name should be valid"),
        )
        .with_arguments(vec![charge_fixture().display().to_string()]);

        let response = service.invoke(request).expect("charge should execute");
        match &response.result.payload {
            ResultPayload::TableReport(table) => {
                assert_eq!(table.rows.len(), 3);
                assert_eq!(table.rows[0][0], "charge_example");
                assert_eq!(table.rows[0][1], "1");
                assert_eq!(table.rows[0][4], "0.300000");
            }
            payload => panic!("unexpected payload: {payload:?}"),
        }
        let plot = response
            .result
            .plot
            .as_ref()
            .expect("charge should attach a plot payload");
        assert_eq!(plot.kind.as_str(), "line");
    }

    #[test]
    fn charge_writes_canonical_plot_contract_fixture() {
        let service = implemented_service();
        let output_path = std::env::temp_dir().join(format!(
            "emboss-charge-plot-{}.json",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("time should advance")
                .as_nanos()
        ));
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("charge").expect("tool name should be valid"),
        )
        .with_arguments(vec![
            charge_fixture().display().to_string(),
            "--plot-contract-out".to_owned(),
            output_path.display().to_string(),
        ]);

        let response = service.invoke(request).expect("charge should execute");
        let emitted =
            std::fs::read_to_string(&output_path).expect("plot contract file should exist");
        let canonical =
            std::fs::read_to_string(charge_plot_fixture()).expect("canonical fixture should exist");
        assert_eq!(emitted.trim(), canonical.trim());
        assert!(response.result.artifacts.iter().any(|artifact| {
            artifact.id == "charge-plot-contract"
                && artifact.local_path.as_ref() == Some(&output_path)
        }));
        let _ = std::fs::remove_file(output_path);
    }

    #[test]
    fn charge_rejects_unsupported_residues() {
        let service = implemented_service();
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("charge").expect("tool name should be valid"),
        )
        .with_arguments(vec![charge_invalid_fixture().display().to_string()]);

        let error = service
            .invoke(request)
            .expect_err("unsupported residues should fail");
        assert_eq!(error.code(), Some("tools.charge.input.unsupported_residue"));
    }

    #[test]
    fn executes_geecee_against_nucleotide_fixture() {
        let service = implemented_service();
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("geecee").expect("tool name should be valid"),
        )
        .with_arguments(vec![nucleotide_pattern_fixture().display().to_string()]);

        let response = service.invoke(request).expect("geecee should execute");
        match &response.result.payload {
            ResultPayload::TableReport(table) => {
                assert_eq!(
                    table.columns,
                    vec![
                        "scope",
                        "record",
                        "length",
                        "gc_count",
                        "gc_denominator",
                        "ambiguous_count",
                        "gc_percent"
                    ]
                );
                assert!(table.rows.iter().any(|row| {
                    row[0] == "record"
                        && row[1] == "nucA"
                        && row[2] == "9"
                        && row[3] == "4"
                        && row[4] == "8"
                        && row[5] == "1"
                        && row[6] == "50.00"
                }));
                assert!(table.rows.iter().any(|row| {
                    row[0] == "aggregate"
                        && row[1] == "ALL"
                        && row[2] == "15"
                        && row[3] == "7"
                        && row[4] == "14"
                        && row[5] == "1"
                        && row[6] == "50.00"
                }));
            }
            payload => panic!("unexpected payload: {payload:?}"),
        }
        assert!(
            response.result.summary.lines[0]
                .ends_with("crates/emboss-tools/tests/fixtures/nucleotide_pattern_records.fasta")
        );
        assert_eq!(
            response.result.summary.lines[1],
            "Scope: per-record plus aggregate summary"
        );
        assert_eq!(
            response.result.summary.lines[2],
            "GC denominator: canonical A/C/G/T/U symbols only"
        );
        assert_eq!(
            response.result.summary.lines[3],
            "Ambiguous symbols are excluded from GC percentage"
        );
        assert_eq!(response.result.summary.lines[4], "Records: 2");
    }

    #[test]
    fn rejects_protein_input_for_geecee() {
        let service = implemented_service();
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("geecee").expect("tool name should be valid"),
        )
        .with_arguments(vec![protein_stats_fixture().display().to_string()]);

        let error = service
            .invoke(request)
            .expect_err("protein input should fail for geecee");
        assert!(error.to_string().contains("expects nucleotide input"));
    }

    #[test]
    fn executes_pepstats_against_protein_fixture() {
        let service = implemented_service();
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("pepstats").expect("tool name should be valid"),
        )
        .with_arguments(vec![protein_stats_fixture().display().to_string()]);

        let response = service.invoke(request).expect("pepstats should execute");
        match &response.result.payload {
            ResultPayload::TableReport(table) => {
                assert_eq!(
                    table.columns,
                    vec![
                        "section",
                        "record",
                        "metric_or_residue",
                        "value_or_count",
                        "frequency",
                        "notes"
                    ]
                );
                assert!(table.rows.iter().any(|row| {
                    row[0] == "summary"
                        && row[1] == "pepA"
                        && row[2] == "sequence_length"
                        && row[3] == "3"
                }));
                assert!(table.rows.iter().any(|row| {
                    row[0] == "summary"
                        && row[1] == "pepA"
                        && row[2] == "residue_length"
                        && row[3] == "2"
                }));
                assert!(table.rows.iter().any(|row| {
                    row[0] == "summary"
                        && row[1] == "pepA"
                        && row[2] == "stop_count"
                        && row[3] == "1"
                }));
                assert!(table.rows.iter().any(|row| {
                    row[0] == "summary"
                        && row[1] == "pepA"
                        && row[2] == "molecular_weight"
                        && row[3] == "220.287"
                }));
                assert!(table.rows.iter().any(|row| {
                    row[0] == "composition"
                        && row[1] == "pepA"
                        && row[2] == "M"
                        && row[3] == "1"
                        && row[4] == "0.3333"
                }));
            }
            payload => panic!("unexpected payload: {payload:?}"),
        }
        assert!(
            response.result.summary.lines[0]
                .ends_with("crates/emboss-tools/tests/fixtures/protein_stats_records.fasta")
        );
        assert_eq!(
            response.result.summary.lines[1],
            "Mass convention: average residue masses plus one water molecule"
        );
        assert_eq!(
            response.result.summary.lines[2],
            "Stop symbols are excluded from residue_length and mass"
        );
        assert_eq!(
            response.result.summary.lines[3],
            "pI estimation: deferred in v1"
        );
        assert_eq!(response.result.summary.lines[4], "Records: 2");
    }

    #[test]
    fn rejects_nucleotide_input_for_pepstats() {
        let service = implemented_service();
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("pepstats").expect("tool name should be valid"),
        )
        .with_arguments(vec![nucleotide_pattern_fixture().display().to_string()]);

        let error = service
            .invoke(request)
            .expect_err("nucleotide input should fail for pepstats");
        assert!(error.to_string().contains("expects protein input"));
    }

    #[test]
    fn executes_chips_against_coding_fixture() {
        let service = implemented_service();
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("chips").expect("tool name should be valid"),
        )
        .with_arguments(vec![codon_reference_fixture().display().to_string()]);

        let response = service.invoke(request).expect("chips should execute");
        match &response.result.payload {
            ResultPayload::TableReport(table) => {
                assert!(
                    table
                        .rows
                        .iter()
                        .any(|row| { row[0] == "aggregate" && row[2] == "CTT" && row[4] == "5" })
                );
            }
            payload => panic!("unexpected payload: {payload:?}"),
        }
    }

    #[test]
    fn executes_cai_against_reference_fixture() {
        let service = implemented_service();
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("cai").expect("tool name should be valid"),
        )
        .with_arguments(vec![
            codon_query_fixture().display().to_string(),
            codon_reference_fixture().display().to_string(),
        ]);

        let response = service.invoke(request).expect("cai should execute");
        match &response.result.payload {
            ResultPayload::TableReport(table) => {
                assert_eq!(table.rows.len(), 2);
                let preferred = table
                    .rows
                    .iter()
                    .find(|row| row[0] == "query_pref")
                    .unwrap();
                let rare = table
                    .rows
                    .iter()
                    .find(|row| row[0] == "query_rare")
                    .unwrap();
                assert!(preferred[3].parse::<f64>().unwrap() > rare[3].parse::<f64>().unwrap());
            }
            payload => panic!("unexpected payload: {payload:?}"),
        }
    }

    #[test]
    fn executes_codcmp_between_two_coding_fixtures() {
        let service = implemented_service();
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("codcmp").expect("tool name should be valid"),
        )
        .with_arguments(vec![
            codon_reference_fixture().display().to_string(),
            codon_compare_right_fixture().display().to_string(),
        ]);

        let response = service.invoke(request).expect("codcmp should execute");
        match &response.result.payload {
            ResultPayload::TableReport(table) => {
                assert!(
                    table
                        .rows
                        .iter()
                        .any(|row| { row[0] == "CTT" && row[2] == "5" && row[4] == "0" })
                );
            }
            payload => panic!("unexpected payload: {payload:?}"),
        }
    }

    #[test]
    fn codcopy_writes_reusable_profile_for_cai() {
        let service = implemented_service();
        let profile_path = std::env::temp_dir().join("emboss-rs-codon-profile.tsv");
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("codcopy").expect("tool name should be valid"),
        )
        .with_arguments(vec![
            codon_reference_fixture().display().to_string(),
            "--profile-out".to_owned(),
            profile_path.display().to_string(),
        ]);
        service.invoke(request).expect("codcopy should execute");

        let cai_request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("cai").expect("tool name should be valid"),
        )
        .with_arguments(vec![
            codon_query_fixture().display().to_string(),
            profile_path.display().to_string(),
        ]);
        let response = service.invoke(cai_request).expect("cai should execute");
        match &response.result.payload {
            ResultPayload::TableReport(table) => assert_eq!(table.rows.len(), 2),
            payload => panic!("unexpected payload: {payload:?}"),
        }

        let _ = std::fs::remove_file(profile_path);
    }
}

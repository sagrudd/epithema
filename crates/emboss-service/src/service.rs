//! Shared service façade for front-end-neutral tool discovery and invocation.

use std::str::FromStr;

use emboss_config::PlatformConfig;
use emboss_core::{MoleculeKind, PLATFORM_IDENTITY};
use emboss_diagnostics::{
    ArtifactProvenance, Diagnostic, ErrorCategory, ExecutionOutcome, ExecutionReport,
    OutcomeStatus, PlatformError,
};
use emboss_providers::ProviderRegistry;
use emboss_tools::ToolDescriptor;
use emboss_tools::sequence_stream::{
    NewseqParams, NotseqParams, NthseqParams, SeqcountParams, SequenceInput, SkipseqParams,
    newseq_help, notseq_help, nthseq_help, run_newseq, run_notseq, run_nthseq, run_seqcount,
    run_skipseq, seqcount_help, skipseq_help,
};
use emboss_tools::sequence_transform::{
    CutseqParams, ExtractseqParams, SplitterParams, UnionParams, cutseq_help, extractseq_help,
    run_cutseq, run_extractseq, run_splitter, run_union, splitter_help, union_help,
};

use crate::ServiceDocumentationAcquisition;
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
        Self::with_platform(registry, PlatformConfig::default(), ProviderRegistry::new())
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
            "seqcount" => self.invoke_seqcount(request, descriptor),
            "nthseq" => self.invoke_nthseq(request, descriptor),
            "skipseq" => self.invoke_skipseq(request, descriptor),
            "notseq" => self.invoke_notseq(request, descriptor),
            "newseq" => self.invoke_newseq(request, descriptor),
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

        let report = self.success_report(
            &request.context,
            format!("counted {} sequence records", outcome.count),
            input_diagnostics,
            vec![input_provenance],
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
                .with_line(format!("Records: {}", outcome.count)),
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
                .with_line("Ordering: preserve input order and per-input record order")
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

#[cfg(test)]
mod tests {
    use emboss_tools::{ToolDescriptor, governed_tool_descriptors};

    use super::EmbossService;
    use crate::{
        ExecutionContext, InvocationOrigin, InvocationRequest, OutcomeStatus, ResultPayload,
        ServiceRegistry, ToolInputKind, ToolInputResolution, ToolName,
    };

    #[test]
    fn resolves_registered_tool_to_placeholder_response() {
        let mut registry = ServiceRegistry::new();
        registry
            .register(ToolDescriptor::new("needle", "global alignment"))
            .expect("registration should succeed");

        let service = EmbossService::new(registry);
        let request = InvocationRequest::new(
            ExecutionContext::for_origin(InvocationOrigin::Cli),
            ToolName::new("needle").expect("tool name should be valid"),
        );

        let response = service.invoke(request).expect("tool should resolve");
        assert_eq!(response.descriptor.name, "needle");
        assert_eq!(response.tool.as_str(), "needle");
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
    fn starts_with_default_platform_configuration_and_no_providers() {
        let service = EmbossService::empty();
        assert!(service.providers().is_empty());
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
                assert_eq!(table.rows[0][1], "3");
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
            }
            payload => panic!("unexpected payload: {payload:?}"),
        }
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
            }
            payload => panic!("unexpected payload: {payload:?}"),
        }
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
            }
            payload => panic!("unexpected payload: {payload:?}"),
        }
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
}

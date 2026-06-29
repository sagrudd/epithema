//! Shared service façade for front-end-neutral tool discovery and invocation.

use std::path::PathBuf;
use std::str::FromStr;
use std::sync::Arc;

use epithema_config::PlatformConfig;
use epithema_core::{
    FeatureKind, FeatureSelector, Interval, MoleculeKind, NucleotidePattern, PLATFORM_IDENTITY,
    PatternError, ProteinPattern, RevseqMode, Strand,
};
use epithema_diagnostics::{
    ArtifactProvenance, Diagnostic, ErrorCategory, ExecutionOutcome, ExecutionReport,
    OutcomeStatus, PlatformError,
};
use epithema_providers::{
    AcquisitionRequest, ArchiveObjectClass, NgsManifest, NgsProvenance, NgsQuery,
    NgsVerificationStatus, ProviderHttpClient, ProviderId, ProviderRegistry,
    RetrievedArchiveManifest, RetrievedArchiveMetadata, RetrievedSequence,
};
use epithema_tools::ToolDescriptor;
use epithema_tools::alignment_analysis::{
    ConsParams, ConsambigParams, DistmatParams, MatcherParams, cons_help, consambig_help,
    distmat_help, matcher_help, run_cons, run_consambig, run_distmat, run_matcher,
};
use epithema_tools::alignment_tools::{
    AligncopyParams, AligncopypairParams, AlignmentInput, DiffseqParams, EdialignParams,
    ExtractalignParams, InfoalignParams, NthseqsetParams, aligncopy_help, aligncopypair_help,
    diffseq_help, edialign_help, extractalign_help, infoalign_help, nthseqset_help, run_aligncopy,
    run_aligncopypair, run_diffseq, run_edialign, run_extractalign, run_infoalign, run_nthseqset,
};
use epithema_tools::archive_tools::{
    AssemblygetParams, InfoassemblyParams, NgsgetParams, NgslistFormat, NgslistParams,
    RungetParams, RuninfoParams, assemblyget_help, infoassembly_help, ngsget_help, ngslist_help,
    run_assemblyget, run_infoassembly, run_ngsget, run_ngslist, run_runget, run_runinfo,
    runget_help, runinfo_help,
};
use epithema_tools::codon_tools::{
    CaiParams, ChipsParams, CodcmpParams, CodcopyParams, CuspParams, cai_help, chips_help,
    codcmp_help, codcopy_help, cusp_help, render_profile_rows, run_cai, run_chips, run_codcmp,
    run_codcopy, run_cusp,
};
use epithema_tools::command_tools::{
    SeealsoParams, WossnameParams, run_seealso, run_wossname, seealso_help, wossname_help,
};
use epithema_tools::feature_tools::{
    CoderetParams, ExtractfeatParams, FeatcopyParams, FeatmergeParams, FeatreportParams,
    FeattextParams, MaskambignucParams, MaskambigprotParams, MaskfeatParams, MaskseqParams,
    SplitsourceParams, TwofeatParams, coderet_help, extractfeat_help, featcopy_help,
    featmerge_help, featreport_help, feattext_help, maskambignuc_help, maskambigprot_help,
    maskfeat_help, maskseq_help, run_coderet, run_extractfeat, run_featcopy, run_featmerge,
    run_featreport, run_feattext, run_maskambignuc, run_maskambigprot, run_maskfeat, run_maskseq,
    run_splitsource, run_twofeat, splitsource_help, twofeat_help,
};
use epithema_tools::nucleotide_plots::{
    BananaParams, DensityParams, IsochoreParams, SycoParams, WobbleParams, banana_help,
    density_help, isochore_help, run_banana, run_density, run_isochore, run_syco, run_wobble,
    syco_help, wobble_help,
};
use epithema_tools::pairwise_alignment::{
    NeedleParams, NeedleallParams, WaterParams, needle_help, needleall_help, run_needle,
    run_needleall, run_water, water_help,
};
use epithema_tools::pattern_tools::{
    DregParams, EinvertedParams, FuzznucParams, FuzzproParams, FuzztranParams, PalindromeParams,
    PatmatdbParams, PregParams, SeqmatchallParams, WordfinderParams, WordmatchParams, dreg_help,
    einverted_help, fuzznuc_help, fuzzpro_help, fuzztran_help, palindrome_help, patmatdb_help,
    preg_help, run_dreg, run_einverted, run_fuzznuc, run_fuzzpro, run_fuzztran, run_palindrome,
    run_patmatdb, run_preg, run_seqmatchall, run_wordfinder, run_wordmatch, seqmatchall_help,
    wordfinder_help, wordmatch_help,
};
use epithema_tools::primer_tools::{
    Eprimer3Params, PrimersearchPairInput, PrimersearchParams, SirnaParams, eprimer3_help,
    primersearch_help, run_eprimer3, run_primersearch, run_sirna, sirna_help,
};
use epithema_tools::protein_coordinates::{PsiphiInput, PsiphiParams, psiphi_help, run_psiphi};
use epithema_tools::protein_plots::{
    ChargeParams, HmomentParams, OctanolParams, PepinfoParams, PepwindowParams, charge_help,
    hmoment_help, octanol_help, pepinfo_help, pepwindow_help, run_charge, run_hmoment, run_octanol,
    run_pepinfo, run_pepwindow,
};
use epithema_tools::restriction_tools::{
    RecoderParams, SilentParams, recoder_help, run_recoder, run_silent, silent_help,
};
use epithema_tools::retrieval_tools::{
    RefseqgetParams, SeqretParams, SeqretSource, SeqretsetallInputSet, SeqretsetallParams,
    SeqretsplitParams, WhichdbParams, refseqget_help, run_refseqget, run_seqret, run_seqretsetall,
    run_seqretsplit, run_whichdb, seqret_help, seqretsetall_help, seqretsplit_help, whichdb_help,
};
use epithema_tools::sequence_edit::{
    BiosedParams, DegapseqParams, DescseqParams, MsbarMutation, MsbarParams, RevseqParams,
    TrimestParams, TrimseqParams, VectorstripParams, biosed_help, degapseq_help, descseq_help,
    msbar_help, revseq_help, run_biosed, run_degapseq, run_descseq, run_msbar, run_revseq,
    run_trimest, run_trimseq, run_vectorstrip, trimest_help, trimseq_help, vectorstrip_help,
};
use epithema_tools::sequence_stats::{
    AaindexextractParams, ComplexParams, CompseqParams, DanParams, GeeceeParams, IepParams,
    InfobaseParams, InforesidueParams, InfoseqParams, OddcompParams, PepdigestParams,
    PepdigestProtease, PepstatsParams, WordcountParams, aaindexextract_help, complex_help,
    compseq_help, dan_help, geecee_help, iep_help, infobase_help, inforesidue_help, infoseq_help,
    oddcomp_help, parse_aaindexextract_index, pepdigest_help, pepstats_help, run_aaindexextract,
    run_complex, run_compseq, run_dan, run_geecee, run_iep, run_infobase, run_inforesidue,
    run_infoseq, run_oddcomp, run_pepdigest, run_pepstats, run_wordcount, word_frequency,
    wordcount_help,
};
use epithema_tools::sequence_stream::{
    ListorParams, MakenucseqParams, MakeprotseqParams, NewseqParams, NotseqParams, NthseqParams,
    SeqcountParams, SequenceInput, SequenceSetOperator, SkipredundantParams, SkipseqParams,
    listor_help, load_sequence_records, makenucseq_help, makeprotseq_help, newseq_help,
    notseq_help, nthseq_help, run_listor, run_makenucseq, run_makeprotseq, run_newseq, run_notseq,
    run_nthseq, run_seqcount, run_skipredundant, run_skipseq, seqcount_help, skipredundant_help,
    skipseq_help,
};
use epithema_tools::sequence_transform::{
    CutseqParams, ExtractseqParams, MegamergerParams, MergerParams, PasteseqParams,
    ShuffleseqParams, SizeseqParams, SplitterParams, UnionParams, cutseq_help, extractseq_help,
    megamerger_help, merger_help, pasteseq_help, run_cutseq, run_extractseq, run_megamerger,
    run_merger, run_pasteseq, run_shuffleseq, run_sizeseq, run_splitter, run_union,
    shuffleseq_help, sizeseq_help, splitter_help, union_help,
};
use epithema_tools::translation_tools::{
    BacktranambigParams, BacktranseqParams, ChecktransParams, GetorfParams, PrettyseqParams,
    TranalignParams, TranseqParams, TranslationFrameSelection, backtranambig_help,
    backtranseq_help, checktrans_help, getorf_help, prettyseq_help, run_backtranambig,
    run_backtranseq, run_checktrans, run_getorf, run_prettyseq, run_tranalign, run_transeq,
    tranalign_help, transeq_help,
};

use crate::ServiceDocumentationAcquisition;
use crate::archive_retrieval::ServiceArchiveRetrieval;
use crate::context::ExecutionContext;
use crate::error::{ServiceError, unknown_tool};
use crate::input::{ToolInputReference, ToolInputResolution, ToolInputResolver};
use crate::ngs_retrieval::{NgsDownloadProgressCallback, ServiceNgsRetrieval};
use crate::registry::{ServiceRegistry, ToolCatalog};
use crate::request::InvocationRequest;
use crate::response::InvocationResponse;
use crate::result::{
    ArtifactKind, ArtifactReference, MethodResult, ResultPayload, ResultSummary, TableReport,
    TextReport,
};
use crate::sequence_retrieval::ServiceSequenceRetrieval;

/// Front-end-neutral Epithema service façade.
#[derive(Clone, Default)]
pub struct EpithemaService {
    registry: ServiceRegistry,
    config: PlatformConfig,
    providers: ProviderRegistry,
    ngs_download_progress: Option<Arc<NgsDownloadProgressCallback>>,
}

impl EpithemaService {
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
            ngs_download_progress: None,
        }
    }

    /// Installs a callback for NGS direct-download progress events.
    #[must_use]
    pub fn with_ngs_download_progress(
        mut self,
        progress_callback: Arc<NgsDownloadProgressCallback>,
    ) -> Self {
        self.ngs_download_progress = Some(progress_callback);
        self
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
    ) -> Result<ServiceSequenceRetrieval<'_, epithema_providers::ReqwestHttpClient>, ServiceError>
    {
        ServiceSequenceRetrieval::new(&self.config, &self.providers)
    }

    /// Returns the formal archive metadata and manifest retrieval gateway.
    pub fn archive_retrieval(
        &self,
    ) -> Result<ServiceArchiveRetrieval<'_, epithema_providers::ReqwestHttpClient>, ServiceError>
    {
        ServiceArchiveRetrieval::new(&self.config, &self.providers)
    }

    /// Returns the formal NGS dataset acquisition gateway.
    pub fn ngs_retrieval(
        &self,
    ) -> Result<ServiceNgsRetrieval<'_, epithema_providers::ReqwestHttpClient>, ServiceError> {
        ServiceNgsRetrieval::new_with_progress(
            &self.config,
            &self.providers,
            self.ngs_download_progress.as_deref(),
        )
    }

    /// Resolves an accession-style input into a provider-backed single sequence record.
    pub fn retrieve_single_sequence(
        &self,
        raw: impl Into<String>,
    ) -> Result<RetrievedSequence, ServiceError> {
        let reference = self.classify_input(raw.into())?;
        match self.resolve_input(reference, epithema_providers::ResolutionIntent::SequenceInput)? {
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
            "diffseq" => self.invoke_diffseq(request, descriptor),
            "edialign" => self.invoke_edialign(request, descriptor),
            "infoalign" => self.invoke_infoalign(request, descriptor),
            "extractalign" => self.invoke_extractalign(request, descriptor),
            "nthseqset" => self.invoke_nthseqset(request, descriptor),
            "assemblyget" => self.invoke_assemblyget(request, descriptor),
            "infoassembly" => self
                .invoke_infoassembly_with_client::<epithema_providers::ReqwestHttpClient>(
                    request, descriptor, None,
                ),
            "runinfo" => self.invoke_runinfo(request, descriptor),
            "runget" => self.invoke_runget(request, descriptor),
            "ngslist" => self.invoke_ngslist(request, descriptor),
            "ngsget" => self.invoke_ngsget(request, descriptor),
            "matcher" => self.invoke_matcher(request, descriptor),
            "distmat" => self.invoke_distmat(request, descriptor),
            "cons" => self.invoke_cons(request, descriptor),
            "consambig" => self.invoke_consambig(request, descriptor),
            "needle" => self.invoke_needle(request, descriptor),
            "needleall" => self.invoke_needleall(request, descriptor),
            "water" => self.invoke_water(request, descriptor),
            "seqret" => self.invoke_seqret(request, descriptor),
            "seqretsetall" => self.invoke_seqretsetall(request, descriptor),
            "seqretsplit" => self.invoke_seqretsplit(request, descriptor),
            "refseqget" => self.invoke_refseqget(request, descriptor),
            "whichdb" => self.invoke_whichdb(request, descriptor),
            "seqcount" => self.invoke_seqcount(request, descriptor),
            "nthseq" => self.invoke_nthseq(request, descriptor),
            "skipseq" => self.invoke_skipseq(request, descriptor),
            "listor" => self.invoke_listor(request, descriptor),
            "skipredundant" => self.invoke_skipredundant(request, descriptor),
            "notseq" => self.invoke_notseq(request, descriptor),
            "newseq" => self.invoke_newseq(request, descriptor),
            "makenucseq" => self.invoke_makenucseq(request, descriptor),
            "makeprotseq" => self.invoke_makeprotseq(request, descriptor),
            "biosed" => self.invoke_biosed(request, descriptor),
            "degapseq" => self.invoke_degapseq(request, descriptor),
            "revseq" => self.invoke_revseq(request, descriptor),
            "msbar" => self.invoke_msbar(request, descriptor),
            "trimest" => self.invoke_trimest(request, descriptor),
            "trimseq" => self.invoke_trimseq(request, descriptor),
            "descseq" => self.invoke_descseq(request, descriptor),
            "vectorstrip" => self.invoke_vectorstrip(request, descriptor),
            "infoseq" => self.invoke_infoseq(request, descriptor),
            "maskseq" => self.invoke_maskseq(request, descriptor),
            "maskambignuc" => self.invoke_maskambignuc(request, descriptor),
            "maskambigprot" => self.invoke_maskambigprot(request, descriptor),
            "maskfeat" => self.invoke_maskfeat(request, descriptor),
            "extractfeat" => self.invoke_extractfeat(request, descriptor),
            "featcopy" => self.invoke_featcopy(request, descriptor),
            "coderet" => self.invoke_coderet(request, descriptor),
            "featmerge" => self.invoke_featmerge(request, descriptor),
            "featreport" => self.invoke_featreport(request, descriptor),
            "feattext" => self.invoke_feattext(request, descriptor),
            "splitsource" => self.invoke_splitsource(request, descriptor),
            "twofeat" => self.invoke_twofeat(request, descriptor),
            "cai" => self.invoke_cai(request, descriptor),
            "chips" => self.invoke_chips(request, descriptor),
            "cusp" => self.invoke_cusp(request, descriptor),
            "codcmp" => self.invoke_codcmp(request, descriptor),
            "codcopy" => self.invoke_codcopy(request, descriptor),
            "dreg" => self.invoke_dreg(request, descriptor),
            "einverted" => self.invoke_einverted(request, descriptor),
            "fuzznuc" => self.invoke_fuzznuc(request, descriptor),
            "fuzzpro" => self.invoke_fuzzpro(request, descriptor),
            "fuzztran" => self.invoke_fuzztran(request, descriptor),
            "palindrome" => self.invoke_palindrome(request, descriptor),
            "preg" => self.invoke_preg(request, descriptor),
            "patmatdb" => self.invoke_patmatdb(request, descriptor),
            "seqmatchall" => self.invoke_seqmatchall(request, descriptor),
            "wordmatch" => self.invoke_wordmatch(request, descriptor),
            "wordfinder" => self.invoke_wordfinder(request, descriptor),
            "seealso" => self.invoke_seealso(request, descriptor),
            "wossname" => self.invoke_wossname(request, descriptor),
            "banana" => self.invoke_banana(request, descriptor),
            "density" => self.invoke_density(request, descriptor),
            "wobble" => self.invoke_wobble(request, descriptor),
            "isochore" => self.invoke_isochore(request, descriptor),
            "syco" => self.invoke_syco(request, descriptor),
            "charge" => self.invoke_charge(request, descriptor),
            "hmoment" => self.invoke_hmoment(request, descriptor),
            "octanol" => self.invoke_octanol(request, descriptor),
            "pepinfo" => self.invoke_pepinfo(request, descriptor),
            "pepwindow" => self.invoke_pepwindow(request, descriptor),
            "eprimer3" => self.invoke_eprimer3(request, descriptor),
            "primersearch" => self.invoke_primersearch(request, descriptor),
            "sirna" => self.invoke_sirna(request, descriptor),
            "psiphi" => self.invoke_psiphi(request, descriptor),
            "recoder" => self.invoke_recoder(request, descriptor),
            "silent" => self.invoke_silent(request, descriptor),
            "aaindexextract" => self.invoke_aaindexextract(request, descriptor),
            "complex" => self.invoke_complex(request, descriptor),
            "compseq" => self.invoke_compseq(request, descriptor),
            "dan" => self.invoke_dan(request, descriptor),
            "geecee" => self.invoke_geecee(request, descriptor),
            "infobase" => self.invoke_infobase(request, descriptor),
            "inforesidue" => self.invoke_inforesidue(request, descriptor),
            "iep" => self.invoke_iep(request, descriptor),
            "oddcomp" => self.invoke_oddcomp(request, descriptor),
            "pepdigest" => self.invoke_pepdigest(request, descriptor),
            "wordcount" => self.invoke_wordcount(request, descriptor),
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
            "pasteseq" => self.invoke_pasteseq(request, descriptor),
            "splitter" => self.invoke_splitter(request, descriptor),
            "merger" => self.invoke_merger(request, descriptor),
            "megamerger" => self.invoke_megamerger(request, descriptor),
            "sizeseq" => self.invoke_sizeseq(request, descriptor),
            "shuffleseq" => self.invoke_shuffleseq(request, descriptor),
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
        intent: epithema_providers::ResolutionIntent,
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

    fn invoke_diffseq(
        &self,
        request: InvocationRequest,
        descriptor: ToolDescriptor,
    ) -> Result<InvocationResponse, ServiceError> {
        if help_requested(request.arguments()) {
            return Ok(self.help_response(request, descriptor, diffseq_help()));
        }

        let params = parse_diffseq_params(request.arguments())?;
        let (asequence, asequence_provenance, asequence_diagnostics) =
            self.resolve_local_sequence_input(&params.asequence.path.display().to_string())?;
        let (bsequence, bsequence_provenance, bsequence_diagnostics) =
            self.resolve_local_sequence_input(&params.bsequence.path.display().to_string())?;
        let outcome = run_diffseq(DiffseqParams {
            asequence,
            bsequence,
            gap_open: params.gap_open,
            gap_extend: params.gap_extend,
        })?;

        let mut diagnostics = asequence_diagnostics;
        diagnostics.extend(bsequence_diagnostics);
        let report = self.success_report(
            &request.context,
            format!(
                "reported {} contiguous difference blocks",
                outcome.blocks.len()
            ),
            diagnostics,
            vec![asequence_provenance, bsequence_provenance],
        );
        let rows = outcome
            .blocks
            .iter()
            .map(|block| {
                vec![
                    block.ordinal.to_string(),
                    block.classification.clone(),
                    block
                        .a_start
                        .map(|value| value.to_string())
                        .unwrap_or_else(|| "-".to_owned()),
                    block
                        .a_end
                        .map(|value| value.to_string())
                        .unwrap_or_else(|| "-".to_owned()),
                    block
                        .b_start
                        .map(|value| value.to_string())
                        .unwrap_or_else(|| "-".to_owned()),
                    block
                        .b_end
                        .map(|value| value.to_string())
                        .unwrap_or_else(|| "-".to_owned()),
                    block.a_segment.clone(),
                    block.b_segment.clone(),
                ]
            })
            .collect();
        let result = MethodResult::new(
            request.tool.clone(),
            ResultPayload::TableReport(TableReport::new(
                vec![
                    "block".to_owned(),
                    "classification".to_owned(),
                    "a_start".to_owned(),
                    "a_end".to_owned(),
                    "b_start".to_owned(),
                    "b_end".to_owned(),
                    "a_segment".to_owned(),
                    "b_segment".to_owned(),
                ],
                rows,
            )),
            ResultSummary::new("Sequence-difference report completed")
                .with_line(format!("A sequence: {}", outcome.asequence.path.display()))
                .with_line(format!("B sequence: {}", outcome.bsequence.path.display()))
                .with_line(format!(
                    "Mode: {}",
                    match outcome.mode {
                        epithema_core::AlignmentMode::Nucleotide => "nucleotide",
                        epithema_core::AlignmentMode::Protein => "protein",
                    }
                ))
                .with_line(format!("Aligned length: {}", outcome.aligned_length))
                .with_line(format!("Difference blocks: {}", outcome.blocks.len()))
                .with_line(format!(
                    "Gap penalties: open={} extend={}",
                    outcome.gap_open, outcome.gap_extend
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

    fn invoke_edialign(
        &self,
        request: InvocationRequest,
        descriptor: ToolDescriptor,
    ) -> Result<InvocationResponse, ServiceError> {
        if help_requested(request.arguments()) {
            return Ok(self.help_response(request, descriptor, edialign_help()));
        }

        let params = parse_edialign_params(request.arguments())?;
        let (input, input_provenance, input_diagnostics) =
            self.resolve_local_sequence_input(&params.input.path.display().to_string())?;
        let outcome = run_edialign(EdialignParams {
            input,
            min_length: params.min_length,
        })?;

        let report = self.success_report(
            &request.context,
            format!(
                "derived exact shared local alignment of length {} across {} rows",
                outcome.alignment.column_count(),
                outcome.alignment.row_count()
            ),
            input_diagnostics,
            vec![input_provenance],
        );
        let result = MethodResult::new(
            request.tool.clone(),
            ResultPayload::Alignment(outcome.alignment),
            ResultSummary::new("Exact shared local alignment derived")
                .with_line(format!("Input: {}", outcome.input.path.display()))
                .with_line(format!(
                    "Minimum shared block length: {}",
                    outcome.min_length
                ))
                .with_line(format!("Shared block: {}", outcome.block))
                .with_line(format!("Rows: {}", outcome.rows.len()))
                .with_line("Alignment model: longest exact shared block")
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

    fn invoke_nthseqset(
        &self,
        request: InvocationRequest,
        descriptor: ToolDescriptor,
    ) -> Result<InvocationResponse, ServiceError> {
        if help_requested(request.arguments()) {
            return Ok(self.help_response(request, descriptor, nthseqset_help()));
        }

        let arguments: [String; 2] = request
            .arguments
            .clone()
            .try_into()
            .map_err(|_| tool_usage_error("nthseqset", nthseqset_help()))?;
        let (input, input_provenance, input_diagnostics) =
            self.resolve_local_alignment_input(&arguments[0])?;
        let number = parse_positive_index("nthseqset", &arguments[1])?;
        let outcome = run_nthseqset(NthseqsetParams { input, number })?;

        let report = self.success_report(
            &request.context,
            format!("selected alignment set {}", outcome.number),
            input_diagnostics,
            vec![input_provenance],
        );
        let result = MethodResult::new(
            request.tool.clone(),
            ResultPayload::Alignment(outcome.alignment),
            ResultSummary::new("Alignment set selected")
                .with_line(format!("Input: {}", outcome.input.path.display()))
                .with_line(format!("Selected set: {}", outcome.number))
                .with_line(format!("Total sets: {}", outcome.total_count))
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
                        epithema_core::AlignmentMode::Nucleotide => "nucleotide",
                        epithema_core::AlignmentMode::Protein => "protein",
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
                        epithema_core::AlignmentMode::Nucleotide => "nucleotide",
                        epithema_core::AlignmentMode::Protein => "protein",
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
        self.invoke_runinfo_with_client::<epithema_providers::ReqwestHttpClient>(
            request, descriptor, None,
        )
    }

    /// Invokes `runinfo` using an explicit provider HTTP client.
    pub fn invoke_runinfo_with_client<C: ProviderHttpClient>(
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

    fn invoke_assemblyget(
        &self,
        request: InvocationRequest,
        descriptor: ToolDescriptor,
    ) -> Result<InvocationResponse, ServiceError> {
        self.invoke_assemblyget_with_client::<epithema_providers::ReqwestHttpClient>(
            request, descriptor, None,
        )
    }

    /// Invokes `assemblyget` using an explicit provider HTTP client.
    pub fn invoke_assemblyget_with_client<C: ProviderHttpClient>(
        &self,
        request: InvocationRequest,
        descriptor: ToolDescriptor,
        client: Option<&C>,
    ) -> Result<InvocationResponse, ServiceError> {
        if help_requested(request.arguments()) {
            return Ok(self.help_response(request, descriptor, assemblyget_help()));
        }

        let [input]: [String; 1] = request
            .arguments
            .clone()
            .try_into()
            .map_err(|_| tool_usage_error("assemblyget", assemblyget_help()))?;
        let (outcome, provenance, diagnostics) =
            self.resolve_assemblyget_with_client(&input, client)?;

        let output_provenance = ArtifactProvenance::generated_output("stdout")
            .with_description("bounded assembly manifest intent table");
        let mut report_provenance = provenance;
        report_provenance.push(output_provenance.clone());
        let report = self.success_report(
            &request.context,
            format!(
                "reported assembly manifest intent for {}:{}",
                outcome.provider, outcome.accession
            ),
            diagnostics,
            report_provenance,
        );
        let result = MethodResult::new(
            request.tool.clone(),
            ResultPayload::TableReport(TableReport::new(
                outcome.report_columns(),
                outcome.report_rows(),
            )),
            ResultSummary::new("Assembly manifest intent reported")
                .with_line(format!("Provider: {}", outcome.provider))
                .with_line(format!("Requested accession: {}", outcome.accession))
                .with_line(format!("Object class: {}", outcome.object_class))
                .with_line(format!("Assembly: {}", outcome.assembly_accession))
                .with_line(format!(
                    "Run: {}",
                    outcome.run_accession.as_deref().unwrap_or("-")
                ))
                .with_line(format!("Manifest mode: {}", outcome.manifest_mode))
                .with_line(format!(
                    "Materialization: {}",
                    outcome.materialization_status.as_str()
                ))
                .with_line("Acquisition policy: manifest intent only; no files are downloaded"),
            report.clone(),
        )
        .with_artifact(
            ArtifactReference::new("stdout", ArtifactKind::Table)
                .with_label("bounded assembly manifest intent table")
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

    /// Invokes `infoassembly` using an explicit provider HTTP client.
    pub fn invoke_infoassembly_with_client<C: ProviderHttpClient>(
        &self,
        request: InvocationRequest,
        descriptor: ToolDescriptor,
        client: Option<&C>,
    ) -> Result<InvocationResponse, ServiceError> {
        if help_requested(request.arguments()) {
            return Ok(self.help_response(request, descriptor, infoassembly_help()));
        }

        let [input]: [String; 1] = request
            .arguments
            .clone()
            .try_into()
            .map_err(|_| tool_usage_error("infoassembly", infoassembly_help()))?;
        let (outcome, provenance, diagnostics) =
            self.resolve_infoassembly_with_client(&input, client)?;

        let report = self.success_report(
            &request.context,
            format!(
                "retrieved assembly-first archive metadata for {}:{}",
                outcome.provider, outcome.accession
            ),
            diagnostics,
            provenance,
        );
        let result = MethodResult::new(
            request.tool.clone(),
            ResultPayload::TableReport(TableReport::new(
                vec!["field".to_owned(), "value".to_owned()],
                infoassembly_rows(&outcome),
            )),
            ResultSummary::new("Assembly metadata normalized")
                .with_line(format!("Provider: {}", outcome.provider))
                .with_line(format!("Accession: {}", outcome.accession))
                .with_line(format!("Object class: {}", outcome.object_class))
                .with_line(format!("Assembly: {}", outcome.assembly_accession))
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
                .with_line(format!("Files: {}", outcome.file_count))
                .with_line(format!(
                    "Total bytes: {}",
                    outcome
                        .total_size_bytes
                        .map_or_else(|| "-".to_owned(), |value| value.to_string())
                ))
                .with_line(format!("Route: {}", outcome.route_endpoint)),
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
        self.invoke_runget_with_client::<epithema_providers::ReqwestHttpClient>(
            request, descriptor, None,
        )
    }

    /// Invokes `runget` using an explicit provider HTTP client.
    pub fn invoke_runget_with_client<C: ProviderHttpClient>(
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

    fn invoke_ngslist(
        &self,
        request: InvocationRequest,
        descriptor: ToolDescriptor,
    ) -> Result<InvocationResponse, ServiceError> {
        self.invoke_ngslist_with_client::<epithema_providers::ReqwestHttpClient>(
            request, descriptor, None,
        )
    }

    /// Invokes `ngslist` using an explicit provider HTTP client.
    pub fn invoke_ngslist_with_client<C: ProviderHttpClient>(
        &self,
        request: InvocationRequest,
        descriptor: ToolDescriptor,
        client: Option<&C>,
    ) -> Result<InvocationResponse, ServiceError> {
        if help_requested(request.arguments()) {
            return Ok(self.help_response(request, descriptor, ngslist_help()));
        }

        let params = parse_ngslist_arguments(request.arguments())?;
        let query = build_ngslist_query(&params.accession, &params.provider)?;
        let manifest = self.retrieve_ngs_manifest_with_client(&query, client)?;
        let rows = ngs_manifest_rows(&manifest);
        let outcome = run_ngslist(NgslistParams {
            accession: manifest.query.accession.clone(),
            provider: manifest.provider.as_str().to_owned(),
            format: params.format,
            run_count: manifest.runs.len(),
            asset_count: rows.len(),
            route_endpoint: manifest.route.endpoint.clone(),
        })?;

        let output_provenance = ArtifactProvenance::generated_output("stdout")
            .with_description(format!("ngslist {} report", outcome.format.as_str()));
        let report = self.success_report(
            &request.context,
            format!(
                "listed NGS assets for {}:{}",
                outcome.provider, outcome.accession
            ),
            Vec::new(),
            vec![manifest.provenance.clone(), output_provenance],
        );
        let summary = ResultSummary::new("NGS asset manifest listed")
            .with_line(format!("Provider: {}", outcome.provider))
            .with_line(format!("Accession: {}", outcome.accession))
            .with_line(format!(
                "Object class: {}",
                manifest
                    .query
                    .object_class
                    .map(|object_class| object_class.as_str())
                    .unwrap_or("-")
            ))
            .with_line(format!("Runs: {}", outcome.run_count))
            .with_line(format!("Assets: {}", outcome.asset_count))
            .with_line(format!("Route: {}", outcome.route_endpoint))
            .with_line(format!("Format: {}", outcome.format.as_str()));
        let payload = match outcome.format {
            NgslistFormat::Table => {
                ResultPayload::TableReport(TableReport::new(ngs_manifest_columns(), rows))
            }
            NgslistFormat::Json => {
                ResultPayload::TextReport(TextReport::new(render_ngs_manifest_json(&manifest)))
            }
        };
        let result = MethodResult::new(request.tool.clone(), payload, summary, report.clone());

        Ok(InvocationResponse::completed(
            request.context,
            request.tool,
            descriptor,
            report,
            result,
        ))
    }

    fn invoke_ngsget(
        &self,
        request: InvocationRequest,
        descriptor: ToolDescriptor,
    ) -> Result<InvocationResponse, ServiceError> {
        self.invoke_ngsget_with_client::<epithema_providers::ReqwestHttpClient>(
            request, descriptor, None,
        )
    }

    /// Invokes `ngsget` using an explicit provider HTTP client.
    pub fn invoke_ngsget_with_client<C: ProviderHttpClient>(
        &self,
        request: InvocationRequest,
        descriptor: ToolDescriptor,
        client: Option<&C>,
    ) -> Result<InvocationResponse, ServiceError> {
        if help_requested(request.arguments()) {
            return Ok(self.help_response(request, descriptor, ngsget_help()));
        }

        let params = parse_ngsget_arguments(request.arguments())?;
        let query = build_ngsget_query(&params.accession, &params.provider)?;
        match client {
            Some(client) => {
                let gateway = ServiceNgsRetrieval::with_client_and_progress(
                    &self.config,
                    &self.providers,
                    client,
                    self.ngs_download_progress.as_deref(),
                );
                self.invoke_ngsget_with_gateway(request, descriptor, params, &query, gateway)
            }
            None => {
                let gateway = self.ngs_retrieval()?;
                self.invoke_ngsget_with_gateway(request, descriptor, params, &query, gateway)
            }
        }
    }

    fn invoke_ngsget_with_gateway<C: ProviderHttpClient>(
        &self,
        request: InvocationRequest,
        descriptor: ToolDescriptor,
        params: NgsgetCliParams,
        query: &NgsQuery,
        gateway: ServiceNgsRetrieval<'_, C>,
    ) -> Result<InvocationResponse, ServiceError> {
        let manifest = gateway.retrieve_manifest(query)?;
        let plan = gateway.plan_downloads(&manifest, &params.output_root, params.include_raw)?;
        let records = if params.existing_download_roots.is_empty() {
            gateway.materialize_download_plan(&plan)?
        } else {
            gateway.materialize_download_plan_with_existing_downloads(
                &plan,
                &params.existing_download_roots,
            )?
        };
        let selected_asset_count = plan.selected_assets.len();
        let failed_record_count = records
            .iter()
            .filter(|record| record.verification_status == NgsVerificationStatus::Failed)
            .count();
        let provenance = NgsProvenance::new(manifest.clone(), plan, records.clone());
        let manifest_path = params.output_root.join("manifest.tsv");
        let provenance_path = params.output_root.join("provenance.json");
        gateway.write_manifest(&provenance, &manifest_path)?;
        gateway.write_provenance(&provenance, &provenance_path)?;

        let outcome = run_ngsget(NgsgetParams {
            accession: manifest.query.accession.clone(),
            provider: manifest.provider.as_str().to_owned(),
            output_root: params.output_root.clone(),
            include_raw: params.include_raw,
            existing_download_roots: params.existing_download_roots.clone(),
            run_count: manifest.runs.len(),
            selected_asset_count,
            failed_record_count,
        })?;

        let manifest_provenance =
            ArtifactProvenance::generated_output(manifest_path.display().to_string())
                .with_description("ngsget handoff manifest TSV");
        let provenance_provenance =
            ArtifactProvenance::generated_output(provenance_path.display().to_string())
                .with_description("ngsget acquisition provenance JSON");
        let report = self.success_report(
            &request.context,
            format!(
                "materialized NGS assets for {}:{}",
                outcome.provider, outcome.accession
            ),
            Vec::new(),
            vec![
                manifest.provenance.clone(),
                manifest_provenance.clone(),
                provenance_provenance.clone(),
            ],
        );
        let summary = ResultSummary::new("NGS assets materialized")
            .with_line(format!("Provider: {}", outcome.provider))
            .with_line(format!("Accession: {}", outcome.accession))
            .with_line(format!("Runs: {}", outcome.run_count))
            .with_line(format!("Selected assets: {}", outcome.selected_asset_count))
            .with_line(format!("Failed records: {}", outcome.failed_record_count))
            .with_line(format!("Include raw: {}", outcome.include_raw))
            .with_line(format!("Output root: {}", outcome.output_root.display()))
            .with_line(format!("Manifest: {}", manifest_path.display()))
            .with_line(format!("Provenance: {}", provenance_path.display()));
        let body = render_ngsget_report(
            &outcome,
            &manifest_path,
            &provenance_path,
            provenance.download_records.len(),
        );
        let result = MethodResult::new(
            request.tool.clone(),
            ResultPayload::TextReport(TextReport::new(body).with_title("ngsget")),
            summary,
            report.clone(),
        )
        .with_artifact(
            ArtifactReference::new("manifest_tsv", ArtifactKind::Table)
                .with_label("NGS handoff manifest TSV")
                .with_local_path(manifest_path)
                .with_provenance(manifest_provenance),
        )
        .with_artifact(
            ArtifactReference::new("provenance_json", ArtifactKind::Auxiliary)
                .with_label("NGS acquisition provenance JSON")
                .with_local_path(provenance_path)
                .with_provenance(provenance_provenance),
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
                        epithema_core::AlignmentMode::Nucleotide => "nucleotide".to_owned(),
                        epithema_core::AlignmentMode::Protein => "protein".to_owned(),
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
                        epithema_core::AlignmentMode::Nucleotide => "nucleotide".to_owned(),
                        epithema_core::AlignmentMode::Protein => "protein".to_owned(),
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
        self.invoke_seqret_inner::<epithema_providers::ReqwestHttpClient>(request, descriptor, None)
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

    fn invoke_seqretsetall_with_client<C: ProviderHttpClient>(
        &self,
        request: InvocationRequest,
        descriptor: ToolDescriptor,
        client: Option<&C>,
    ) -> Result<InvocationResponse, ServiceError> {
        if help_requested(request.arguments()) {
            return Ok(self.help_response(request, descriptor, seqretsetall_help()));
        }

        if request.arguments.len() < 2 {
            return Err(tool_usage_error("seqretsetall", seqretsetall_help()));
        }

        let (outcome, provenance, diagnostics) =
            self.resolve_seqretsetall_inputs_with_client(&request.arguments, client)?;
        let output_provenance = ArtifactProvenance::generated_output("stdout")
            .with_description("partitioned normalized FASTA output");
        let mut report_provenance = provenance;
        report_provenance.push(output_provenance.clone());
        let report = self.success_report(
            &request.context,
            format!(
                "normalized {} input set(s) into {} total sequence record(s)",
                outcome.record_sets.len(),
                outcome.total_records
            ),
            diagnostics,
            report_provenance,
        );
        let result = MethodResult::new(
            request.tool.clone(),
            ResultPayload::SequencePartitions(outcome.record_sets),
            ResultSummary::new("Sequence retrieval set normalization completed")
                .with_line(format!("Input sets: {}", outcome.inputs.len()))
                .with_line(format!("Total records: {}", outcome.total_records))
                .with_line("Partition policy: preserve one ordered record set per resolved input")
                .with_line("Output format: FASTA partitions"),
            report.clone(),
        )
        .with_artifact(
            ArtifactReference::new("sequence-partitions", ArtifactKind::Sequence)
                .with_label("Partitioned normalized FASTA")
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

    fn invoke_seqretsplit_with_client<C: ProviderHttpClient>(
        &self,
        request: InvocationRequest,
        descriptor: ToolDescriptor,
        client: Option<&C>,
    ) -> Result<InvocationResponse, ServiceError> {
        if help_requested(request.arguments()) {
            return Ok(self.help_response(request, descriptor, seqretsplit_help()));
        }

        let [input]: [String; 1] = request
            .arguments
            .clone()
            .try_into()
            .map_err(|_| tool_usage_error("seqretsplit", seqretsplit_help()))?;

        let (outcome, provenance, diagnostics) =
            self.resolve_seqretsplit_input_with_client(&input, client)?;
        let output_provenance = ArtifactProvenance::generated_output("stdout")
            .with_description("split-output normalized FASTA partitions");
        let mut report_provenance = provenance;
        report_provenance.push(output_provenance.clone());
        let partitions = outcome
            .outputs
            .iter()
            .map(|output| vec![output.record.clone()])
            .collect::<Vec<_>>();
        let file_names = outcome
            .outputs
            .iter()
            .map(|output| output.file_name.clone())
            .collect::<Vec<_>>();
        let input_label = match &outcome.source {
            SeqretSource::LocalPath(path) => path.display().to_string(),
            SeqretSource::Retrieved {
                provider,
                accession,
            } => format!("{provider}:{accession}"),
        };
        let report = self.success_report(
            &request.context,
            format!(
                "normalized one input into {} split-output sequence file(s)",
                outcome.total_records
            ),
            diagnostics,
            report_provenance,
        );
        let mut result = MethodResult::new(
            request.tool.clone(),
            ResultPayload::SequencePartitions(partitions),
            ResultSummary::new("Sequence retrieval split normalization completed")
                .with_line(format!("Input source: {input_label}"))
                .with_line(format!("Split files: {}", outcome.total_records))
                .with_line("Partition policy: emit one normalized FASTA file per resolved record")
                .with_line(format!("File naming: {}", file_names.join(", "))),
            report.clone(),
        )
        .with_artifact(
            ArtifactReference::new("sequence-partitions", ArtifactKind::Sequence)
                .with_label("Split-output normalized FASTA partitions")
                .with_provenance(output_provenance),
        );

        for file_name in file_names {
            result = result.with_artifact(
                ArtifactReference::new(file_name.clone(), ArtifactKind::Sequence)
                    .with_label(file_name),
            );
        }

        Ok(InvocationResponse::completed(
            request.context,
            request.tool,
            descriptor,
            report,
            result,
        ))
    }

    /// Invokes `psiphi` over one bounded local protein-coordinate input.
    pub fn invoke_psiphi(
        &self,
        request: InvocationRequest,
        descriptor: ToolDescriptor,
    ) -> Result<InvocationResponse, ServiceError> {
        if help_requested(request.arguments()) {
            return Ok(self.help_response(request, descriptor, psiphi_help()));
        }

        let [input]: [String; 1] = request
            .arguments
            .clone()
            .try_into()
            .map_err(|_| tool_usage_error("psiphi", psiphi_help()))?;
        let (input, input_provenance, diagnostics) = self.resolve_local_coordinate_input(&input)?;
        let outcome = run_psiphi(PsiphiParams { input })?;
        let rows = outcome
            .profile
            .residues
            .iter()
            .map(|residue| {
                vec![
                    residue.ordinal.to_string(),
                    residue
                        .chain_id
                        .map(|ch| ch.to_string())
                        .unwrap_or_else(|| "-".to_owned()),
                    residue.residue_name.clone(),
                    residue.residue_number.to_string(),
                    residue
                        .insertion_code
                        .map(|ch| ch.to_string())
                        .unwrap_or_else(|| "-".to_owned()),
                    residue.has_backbone_n.to_string(),
                    residue.has_backbone_ca.to_string(),
                    residue.has_backbone_c.to_string(),
                    residue.previous_contiguous.to_string(),
                    residue.next_contiguous.to_string(),
                    residue
                        .phi_degrees
                        .map(|value| format!("{value:.6}"))
                        .unwrap_or_else(|| "-".to_owned()),
                    residue
                        .psi_degrees
                        .map(|value| format!("{value:.6}"))
                        .unwrap_or_else(|| "-".to_owned()),
                ]
            })
            .collect();

        let report = self.success_report(
            &request.context,
            format!(
                "reported bounded phi/psi torsion angles for {} residues",
                outcome.profile.residue_count
            ),
            diagnostics,
            vec![input_provenance],
        );
        let result = MethodResult::new(
            request.tool.clone(),
            ResultPayload::TableReport(TableReport::new(
                vec![
                    "ordinal".to_owned(),
                    "chain".to_owned(),
                    "residue_name".to_owned(),
                    "residue_number".to_owned(),
                    "insertion_code".to_owned(),
                    "has_backbone_n".to_owned(),
                    "has_backbone_ca".to_owned(),
                    "has_backbone_c".to_owned(),
                    "previous_contiguous".to_owned(),
                    "next_contiguous".to_owned(),
                    "phi_degrees".to_owned(),
                    "psi_degrees".to_owned(),
                ],
                rows,
            )),
            ResultSummary::new("Protein phi/psi torsion angles reported")
                .with_line(format!("Input: {}", outcome.input.path.display()))
                .with_line("Coordinate scope: local PDB ATOM backbone records only")
                .with_line("Backbone policy: retain only N, CA, and C atoms")
                .with_line(
                    "Continuity policy: same-chain, sequential, insertion-code-free residues only",
                )
                .with_line(format!("Residues: {}", outcome.profile.residue_count))
                .with_line(format!("Phi angles: {}", outcome.profile.phi_count))
                .with_line(format!("Psi angles: {}", outcome.profile.psi_count)),
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

    /// Invokes the bounded local `primersearch` result surface without shipping it.
    pub fn invoke_primersearch(
        &self,
        request: InvocationRequest,
        descriptor: ToolDescriptor,
    ) -> Result<InvocationResponse, ServiceError> {
        if help_requested(request.arguments()) {
            return Ok(self.help_response(request, descriptor, primersearch_help()));
        }

        let [input, primer_pairs]: [String; 2] = request
            .arguments
            .clone()
            .try_into()
            .map_err(|_| tool_usage_error("primersearch", primersearch_help()))?;
        let (input, input_provenance, mut diagnostics) =
            self.resolve_local_sequence_input(&input)?;
        let (primer_pairs, primer_pair_provenance, pair_diagnostics) =
            self.resolve_local_file_input(&primer_pairs)?;
        diagnostics.extend(pair_diagnostics);
        let outcome = run_primersearch(PrimersearchParams {
            input,
            primer_pairs: PrimersearchPairInput::new(primer_pairs),
        })?;
        let rows = outcome
            .rows
            .iter()
            .map(|row| {
                vec![
                    row.record_id.clone(),
                    row.primer_pair_name.clone(),
                    row.strand.clone(),
                    row.left_primer_start.to_string(),
                    row.left_primer_end.to_string(),
                    row.right_primer_start.to_string(),
                    row.right_primer_end.to_string(),
                    row.amplicon_start.to_string(),
                    row.amplicon_end.to_string(),
                    row.amplicon_length.to_string(),
                    row.left_matched.clone(),
                    row.right_matched.clone(),
                ]
            })
            .collect();

        let report = self.success_report(
            &request.context,
            format!(
                "reported bounded primer-pair hits across {} target records",
                outcome.record_count
            ),
            diagnostics,
            vec![input_provenance, primer_pair_provenance],
        );
        let result = MethodResult::new(
            request.tool.clone(),
            ResultPayload::TableReport(TableReport::new(
                vec![
                    "record".to_owned(),
                    "primer_pair".to_owned(),
                    "strand".to_owned(),
                    "left_primer_start".to_owned(),
                    "left_primer_end".to_owned(),
                    "right_primer_start".to_owned(),
                    "right_primer_end".to_owned(),
                    "amplicon_start".to_owned(),
                    "amplicon_end".to_owned(),
                    "amplicon_length".to_owned(),
                    "left_matched".to_owned(),
                    "right_matched".to_owned(),
                ],
                rows,
            )),
            ResultSummary::new("Primer-pair hits reported")
                .with_line(format!("Input: {}", outcome.input.path.display()))
                .with_line(format!(
                    "Primer pairs: {}",
                    outcome.primer_pairs.path.display()
                ))
                .with_line("Matching scope: exact or IUPAC-ambiguous primer text only")
                .with_line("Completion policy: report complete primer-pair hits only")
                .with_line(format!("Primer pairs: {}", outcome.primer_pair_count))
                .with_line(format!("Target records: {}", outcome.record_count))
                .with_line(format!("Hits: {}", outcome.rows.len())),
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

    /// Invokes the bounded local `eprimer3` result surface without shipping it.
    pub fn invoke_eprimer3(
        &self,
        request: InvocationRequest,
        descriptor: ToolDescriptor,
    ) -> Result<InvocationResponse, ServiceError> {
        if help_requested(request.arguments()) {
            return Ok(self.help_response(request, descriptor, eprimer3_help()));
        }

        let [input]: [String; 1] = request
            .arguments
            .clone()
            .try_into()
            .map_err(|_| tool_usage_error("eprimer3", eprimer3_help()))?;
        let (input, input_provenance, diagnostics) = self.resolve_local_sequence_input(&input)?;
        let outcome = run_eprimer3(Eprimer3Params { input })?;
        let rows = outcome
            .rows
            .iter()
            .map(|row| {
                vec![
                    row.record_id.clone(),
                    row.candidate_id.clone(),
                    row.strand.clone(),
                    row.oligo_start.to_string(),
                    row.oligo_end.to_string(),
                    row.oligo_length.to_string(),
                    row.oligo_sequence.clone(),
                    row.canonical_symbols.to_string(),
                    row.ambiguous_symbols.to_string(),
                    format!("{:.4}", row.gc_fraction),
                    format!("{:.2}", row.tm_celsius),
                    row.three_prime_gc_count.to_string(),
                ]
            })
            .collect();

        let report = self.success_report(
            &request.context,
            format!(
                "reported bounded primer-and-oligo candidates across {} target records",
                outcome.record_count
            ),
            diagnostics,
            vec![input_provenance],
        );
        let result = MethodResult::new(
            request.tool.clone(),
            ResultPayload::TableReport(TableReport::new(
                vec![
                    "record".to_owned(),
                    "candidate_id".to_owned(),
                    "strand".to_owned(),
                    "oligo_start".to_owned(),
                    "oligo_end".to_owned(),
                    "oligo_length".to_owned(),
                    "oligo_sequence".to_owned(),
                    "canonical_symbols".to_owned(),
                    "ambiguous_symbols".to_owned(),
                    "gc_fraction".to_owned(),
                    "tm_celsius".to_owned(),
                    "three_prime_gc_count".to_owned(),
                ],
                rows,
            )),
            ResultSummary::new("Primer-and-oligo candidates reported")
                .with_line(format!("Input: {}", outcome.input.path.display()))
                .with_line(format!(
                    "Design parameters: oligo_length={}..={} step={}",
                    outcome.parameters.min_oligo_length,
                    outcome.parameters.max_oligo_length,
                    outcome.parameters.step
                ))
                .with_line(format!(
                    "Bounds: gc_fraction={:.2}..={:.2} tm_celsius={:.1}..={:.1}",
                    outcome.parameters.min_gc_fraction,
                    outcome.parameters.max_gc_fraction,
                    outcome.parameters.min_tm_celsius,
                    outcome.parameters.max_tm_celsius
                ))
                .with_line("Design scope: deterministic local candidate generation only".to_owned())
                .with_line(format!("Target records: {}", outcome.record_count))
                .with_line(format!("Candidates: {}", outcome.rows.len())),
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

    /// Invokes the bounded local `sirna` result surface without shipping it.
    pub fn invoke_sirna(
        &self,
        request: InvocationRequest,
        descriptor: ToolDescriptor,
    ) -> Result<InvocationResponse, ServiceError> {
        if help_requested(request.arguments()) {
            return Ok(self.help_response(request, descriptor, sirna_help()));
        }

        let [input]: [String; 1] = request
            .arguments
            .clone()
            .try_into()
            .map_err(|_| tool_usage_error("sirna", sirna_help()))?;
        let (input, input_provenance, diagnostics) = self.resolve_local_sequence_input(&input)?;
        let outcome = run_sirna(SirnaParams { input })?;
        let rows = outcome
            .rows
            .iter()
            .map(|row| {
                vec![
                    row.record_id.clone(),
                    row.candidate_id.clone(),
                    row.strand.clone(),
                    row.target_start.to_string(),
                    row.target_end.to_string(),
                    row.duplex_length.to_string(),
                    row.sense_sequence.clone(),
                    row.guide_sequence.clone(),
                    row.canonical_symbols.to_string(),
                    row.ambiguous_symbols.to_string(),
                    format!("{:.4}", row.gc_fraction),
                    row.guide_five_prime_base.clone(),
                    row.guide_seed_au_count.to_string(),
                    row.max_homopolymer_run.to_string(),
                ]
            })
            .collect();

        let report = self.success_report(
            &request.context,
            format!(
                "reported bounded sirna candidates across {} target records",
                outcome.record_count
            ),
            diagnostics,
            vec![input_provenance],
        );
        let result = MethodResult::new(
            request.tool.clone(),
            ResultPayload::TableReport(TableReport::new(
                vec![
                    "record".to_owned(),
                    "candidate_id".to_owned(),
                    "strand".to_owned(),
                    "target_start".to_owned(),
                    "target_end".to_owned(),
                    "duplex_length".to_owned(),
                    "sense_sequence".to_owned(),
                    "guide_sequence".to_owned(),
                    "canonical_symbols".to_owned(),
                    "ambiguous_symbols".to_owned(),
                    "gc_fraction".to_owned(),
                    "guide_five_prime_base".to_owned(),
                    "guide_seed_au_count".to_owned(),
                    "max_homopolymer_run".to_owned(),
                ],
                rows,
            )),
            ResultSummary::new("siRNA candidates reported")
                .with_line(format!("Input: {}", outcome.input.path.display()))
                .with_line(format!(
                    "Design parameters: duplex_length={} step={}",
                    outcome.parameters.duplex_length, outcome.parameters.step
                ))
                .with_line(format!(
                    "Bounds: gc_fraction={:.2}..={:.2} seed_au_min={} max_homopolymer_run={}",
                    outcome.parameters.min_gc_fraction,
                    outcome.parameters.max_gc_fraction,
                    outcome.parameters.min_seed_au_count,
                    outcome.parameters.max_homopolymer_run
                ))
                .with_line(
                    "Design scope: deterministic local siRNA candidate generation only".to_owned(),
                )
                .with_line(format!("Target records: {}", outcome.record_count))
                .with_line(format!("Candidates: {}", outcome.rows.len())),
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

    /// Invokes the bounded shipped `wossname` result surface.
    pub fn invoke_wossname(
        &self,
        request: InvocationRequest,
        descriptor: ToolDescriptor,
    ) -> Result<InvocationResponse, ServiceError> {
        if help_requested(request.arguments()) {
            return Ok(self.help_response(request, descriptor, wossname_help()));
        }

        let [query]: [String; 1] = request
            .arguments
            .clone()
            .try_into()
            .map_err(|_| tool_usage_error("wossname", wossname_help()))?;
        let outcome = run_wossname(WossnameParams { query })?;
        let rows = outcome
            .rows
            .iter()
            .map(|row| {
                vec![
                    row.tool_name.clone(),
                    row.family.clone(),
                    row.short_description.clone(),
                    row.matched_terms.join(","),
                    row.matched_fields.join(","),
                ]
            })
            .collect();

        let report = self.success_report(
            &request.context,
            format!(
                "reported deterministic keyword matches across {} governed tool entries",
                outcome.searched_entry_count
            ),
            Vec::new(),
            Vec::new(),
        );
        let result = MethodResult::new(
            request.tool.clone(),
            ResultPayload::TableReport(TableReport::new(
                vec![
                    "tool".to_owned(),
                    "family".to_owned(),
                    "short_description".to_owned(),
                    "matched_terms".to_owned(),
                    "matched_fields".to_owned(),
                ],
                rows,
            )),
            ResultSummary::new("Keyword-matched tool rows reported")
                .with_line(format!("Query: {}", outcome.query))
                .with_line(format!(
                    "Normalized terms: {}",
                    outcome.normalized_terms.join(", ")
                ))
                .with_line(format!(
                    "Governed entries searched: {}",
                    outcome.searched_entry_count
                ))
                .with_line(
                    "Discovery scope: deterministic local governed-metadata keyword lookup only",
                )
                .with_line(format!("Matches: {}", outcome.rows.len())),
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

    /// Invokes the bounded shipped `seealso` result surface.
    pub fn invoke_seealso(
        &self,
        request: InvocationRequest,
        descriptor: ToolDescriptor,
    ) -> Result<InvocationResponse, ServiceError> {
        if help_requested(request.arguments()) {
            return Ok(self.help_response(request, descriptor, seealso_help()));
        }

        let [tool_name]: [String; 1] = request
            .arguments
            .clone()
            .try_into()
            .map_err(|_| tool_usage_error("seealso", seealso_help()))?;
        let outcome = run_seealso(SeealsoParams { tool_name })?;
        let rows = outcome
            .rows
            .iter()
            .map(|row| {
                vec![
                    row.query_tool.clone(),
                    row.related_tool.clone(),
                    row.related_family.clone(),
                    row.related_short_description.clone(),
                    row.relationship_terms.join(","),
                    row.relationship_fields.join(","),
                ]
            })
            .collect();

        let report = self.success_report(
            &request.context,
            format!(
                "reported deterministic related-program rows across {} governed tool entries",
                outcome.searched_entry_count
            ),
            Vec::new(),
            Vec::new(),
        );
        let result = MethodResult::new(
            request.tool.clone(),
            ResultPayload::TableReport(TableReport::new(
                vec![
                    "query_tool".to_owned(),
                    "related_tool".to_owned(),
                    "related_family".to_owned(),
                    "related_short_description".to_owned(),
                    "relationship_terms".to_owned(),
                    "relationship_fields".to_owned(),
                ],
                rows,
            )),
            ResultSummary::new("Related-program rows reported")
                .with_line(format!("Query tool: {}", outcome.query_tool_name))
                .with_line(format!(
                    "Resolved tool: {}",
                    outcome.resolved_tool_name
                ))
                .with_line(format!(
                    "Resolved family: {}",
                    outcome.resolved_family
                ))
                .with_line(format!(
                    "Governed entries searched: {}",
                    outcome.searched_entry_count
                ))
                .with_line(
                    "Discovery scope: deterministic local governed-metadata relationship lookup only",
                )
                .with_line(format!("Related tools: {}", outcome.rows.len())),
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

    fn invoke_seqretsetall(
        &self,
        request: InvocationRequest,
        descriptor: ToolDescriptor,
    ) -> Result<InvocationResponse, ServiceError> {
        self.invoke_seqretsetall_with_client::<epithema_providers::ReqwestHttpClient>(
            request, descriptor, None,
        )
    }

    fn invoke_seqretsplit(
        &self,
        request: InvocationRequest,
        descriptor: ToolDescriptor,
    ) -> Result<InvocationResponse, ServiceError> {
        self.invoke_seqretsplit_with_client::<epithema_providers::ReqwestHttpClient>(
            request, descriptor, None,
        )
    }

    fn invoke_refseqget(
        &self,
        request: InvocationRequest,
        descriptor: ToolDescriptor,
    ) -> Result<InvocationResponse, ServiceError> {
        self.invoke_refseqget_with_client::<epithema_providers::ReqwestHttpClient>(
            request, descriptor, None,
        )
    }

    fn invoke_whichdb(
        &self,
        request: InvocationRequest,
        descriptor: ToolDescriptor,
    ) -> Result<InvocationResponse, ServiceError> {
        if help_requested(request.arguments()) {
            return Ok(self.help_response(request, descriptor, whichdb_help()));
        }

        let [query]: [String; 1] = request
            .arguments
            .clone()
            .try_into()
            .map_err(|_| tool_usage_error("whichdb", whichdb_help()))?;
        let outcome = run_whichdb(WhichdbParams { query })?;
        let output_provenance = ArtifactProvenance::generated_output("stdout")
            .with_description("bounded provider-discovery table");
        let report = self.success_report(
            &request.context,
            format!(
                "reported {} provider-discovery route(s) for {}:{}",
                outcome.rows.len(),
                outcome.provider,
                outcome.normalized_query
            ),
            Vec::new(),
            vec![output_provenance.clone()],
        );
        let result = MethodResult::new(
            request.tool.clone(),
            ResultPayload::TableReport(TableReport::new(
                outcome.report_columns(),
                outcome.report_rows(),
            )),
            ResultSummary::new("Provider discovery report completed")
                .with_line(format!("Provider: {}", outcome.provider))
                .with_line(format!("Normalized query: {}", outcome.normalized_query))
                .with_line("Discovery policy: bounded provider-qualified reporting only")
                .with_line("Retrieval policy: no live lookup or payload retrieval is performed"),
            report.clone(),
        )
        .with_artifact(
            ArtifactReference::new("stdout", ArtifactKind::Table)
                .with_label("bounded provider-discovery table")
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

    /// Invokes `refseqget` using an explicit provider HTTP client.
    pub fn invoke_refseqget_with_client<C: ProviderHttpClient>(
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

    fn invoke_listor(
        &self,
        request: InvocationRequest,
        descriptor: ToolDescriptor,
    ) -> Result<InvocationResponse, ServiceError> {
        if help_requested(request.arguments()) {
            return Ok(self.help_response(request, descriptor, listor_help()));
        }

        let params = parse_listor_params(request.arguments())?;
        let first_path = params.first.path.display().to_string();
        let second_path = params.second.path.display().to_string();
        let (first, first_provenance, first_diagnostics) =
            self.resolve_local_sequence_input(&first_path)?;
        let (second, second_provenance, second_diagnostics) =
            self.resolve_local_sequence_input(&second_path)?;
        let outcome = run_listor(ListorParams {
            first,
            second,
            operator: params.operator,
        })?;

        let mut diagnostics = first_diagnostics;
        diagnostics.extend(second_diagnostics);
        let output_provenance = ArtifactProvenance::generated_output("stdout")
            .with_description("logical set FASTA output");
        let report = self.success_report(
            &request.context,
            format!(
                "applied {} set operation and returned {} records",
                outcome.operator.label(),
                outcome.records.len()
            ),
            diagnostics,
            vec![
                first_provenance,
                second_provenance,
                output_provenance.clone(),
            ],
        );
        let result = MethodResult::new(
            request.tool.clone(),
            ResultPayload::SequenceCollection(outcome.records),
            ResultSummary::new("Sequence set operation completed")
                .with_line(format!("First input: {}", outcome.first.path.display()))
                .with_line(format!("Second input: {}", outcome.second.path.display()))
                .with_line(format!("Operator: {}", outcome.operator.label()))
                .with_line(format!(
                    "Dropped duplicates in first input: {}",
                    outcome.first_duplicate_count
                ))
                .with_line(format!(
                    "Dropped duplicates in second input: {}",
                    outcome.second_duplicate_count
                ))
                .with_line("Comparison rule: exact normalized sequence content plus molecule kind")
                .with_line("Output format: fasta"),
            report.clone(),
        )
        .with_artifact(
            ArtifactReference::new("logical-set-sequences", ArtifactKind::Sequence)
                .with_label("Logical set result")
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

    fn invoke_skipredundant(
        &self,
        request: InvocationRequest,
        descriptor: ToolDescriptor,
    ) -> Result<InvocationResponse, ServiceError> {
        if help_requested(request.arguments()) {
            return Ok(self.help_response(request, descriptor, skipredundant_help()));
        }

        let [input]: [String; 1] = request
            .arguments
            .clone()
            .try_into()
            .map_err(|_| tool_usage_error("skipredundant", skipredundant_help()))?;
        let (input, input_provenance, input_diagnostics) =
            self.resolve_local_sequence_input(&input)?;
        let outcome = run_skipredundant(SkipredundantParams { input })?;

        let output_provenance = ArtifactProvenance::generated_output("stdout")
            .with_description("non-redundant FASTA output");
        let report = self.success_report(
            &request.context,
            format!("removed {} redundant records", outcome.redundant_count),
            input_diagnostics,
            vec![input_provenance, output_provenance.clone()],
        );
        let result = MethodResult::new(
            request.tool.clone(),
            ResultPayload::SequenceCollection(outcome.records),
            ResultSummary::new("Redundant sequences removed")
                .with_line(format!("Input: {}", outcome.input.path.display()))
                .with_line(format!("Input records: {}", outcome.total_count))
                .with_line(format!("Removed redundant: {}", outcome.redundant_count))
                .with_line(format!(
                    "Returned: {}",
                    outcome.total_count.saturating_sub(outcome.redundant_count)
                ))
                .with_line("Redundancy rule: exact normalized sequence content plus molecule kind")
                .with_line("Output format: fasta"),
            report.clone(),
        )
        .with_artifact(
            ArtifactReference::new("non-redundant-sequences", ArtifactKind::Sequence)
                .with_label("Non-redundant sequences")
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

    fn invoke_makenucseq(
        &self,
        request: InvocationRequest,
        descriptor: ToolDescriptor,
    ) -> Result<InvocationResponse, ServiceError> {
        if help_requested(request.arguments()) {
            return Ok(self.help_response(request, descriptor, makenucseq_help()));
        }

        let outcome = run_makenucseq(parse_makenucseq_params(request.arguments())?)?;
        let output_provenance = ArtifactProvenance::generated_output("stdout")
            .with_description("generated nucleotide FASTA output");
        let report = self.success_report(
            &request.context,
            format!("generated {} nucleotide records", outcome.records.len()),
            Vec::new(),
            vec![output_provenance.clone()],
        );
        let result = MethodResult::new(
            request.tool.clone(),
            ResultPayload::SequenceCollection(outcome.records),
            ResultSummary::new("Deterministic nucleotide sequences created")
                .with_line(format!("Identifier prefix: {}", outcome.identifier_prefix))
                .with_line(format!("Length: {}", outcome.length))
                .with_line(format!("Count: {}", outcome.count))
                .with_line(format!("Seed: {}", outcome.seed))
                .with_line(format!("Molecule: {}", outcome.molecule))
                .with_line("Output format: fasta"),
            report.clone(),
        )
        .with_artifact(
            ArtifactReference::new("generated-nucleotide-sequences", ArtifactKind::Sequence)
                .with_label("Generated nucleotide sequences")
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

    fn invoke_makeprotseq(
        &self,
        request: InvocationRequest,
        descriptor: ToolDescriptor,
    ) -> Result<InvocationResponse, ServiceError> {
        if help_requested(request.arguments()) {
            return Ok(self.help_response(request, descriptor, makeprotseq_help()));
        }

        let outcome = run_makeprotseq(parse_makeprotseq_params(request.arguments())?)?;
        let output_provenance = ArtifactProvenance::generated_output("stdout")
            .with_description("generated protein FASTA output");
        let report = self.success_report(
            &request.context,
            format!("generated {} protein records", outcome.records.len()),
            Vec::new(),
            vec![output_provenance.clone()],
        );
        let result = MethodResult::new(
            request.tool.clone(),
            ResultPayload::SequenceCollection(outcome.records),
            ResultSummary::new("Deterministic protein sequences created")
                .with_line(format!("Identifier prefix: {}", outcome.identifier_prefix))
                .with_line(format!("Length: {}", outcome.length))
                .with_line(format!("Count: {}", outcome.count))
                .with_line(format!("Seed: {}", outcome.seed))
                .with_line("Molecule: protein")
                .with_line("Output format: fasta"),
            report.clone(),
        )
        .with_artifact(
            ArtifactReference::new("generated-protein-sequences", ArtifactKind::Sequence)
                .with_label("Generated protein sequences")
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

    fn invoke_biosed(
        &self,
        request: InvocationRequest,
        descriptor: ToolDescriptor,
    ) -> Result<InvocationResponse, ServiceError> {
        if help_requested(request.arguments()) {
            return Ok(self.help_response(request, descriptor, biosed_help()));
        }

        let params = parse_biosed_params(request.arguments())?;
        let input_path = params.input.path.display().to_string();
        let (input, input_provenance, input_diagnostics) =
            self.resolve_local_sequence_input(&input_path)?;
        let outcome = run_biosed(BiosedParams {
            input,
            start: params.start,
            end: params.end,
            replacement: params.replacement,
        })?;

        let output_provenance =
            ArtifactProvenance::generated_output("stdout").with_description("biosed FASTA output");
        let report = self.success_report(
            &request.context,
            format!("edited {} records", outcome.records.len()),
            input_diagnostics,
            vec![input_provenance, output_provenance.clone()],
        );
        let result = MethodResult::new(
            request.tool.clone(),
            ResultPayload::SequenceCollection(outcome.records),
            ResultSummary::new("Sequence interval editing completed")
                .with_line(format!("Input: {}", outcome.input.path.display()))
                .with_line(format!("Start: {}", outcome.start))
                .with_line(format!("End: {}", outcome.end))
                .with_line(format!(
                    "Replacement: {}",
                    outcome.replacement.unwrap_or_else(|| "<delete>".to_owned())
                ))
                .with_line("Coordinate convention: 1-based inclusive")
                .with_line("Output format: fasta (feature annotations dropped in v1)"),
            report.clone(),
        )
        .with_artifact(
            ArtifactReference::new("biosed-sequences", ArtifactKind::Sequence)
                .with_label("Biosed output sequences")
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

    fn invoke_msbar(
        &self,
        request: InvocationRequest,
        descriptor: ToolDescriptor,
    ) -> Result<InvocationResponse, ServiceError> {
        if help_requested(request.arguments()) {
            return Ok(self.help_response(request, descriptor, msbar_help()));
        }

        let params = parse_msbar_params(request.arguments())?;
        let input_path = params.input.path.display().to_string();
        let (input, input_provenance, input_diagnostics) =
            self.resolve_local_sequence_input(&input_path)?;
        let mutation_count = params.mutations.len();
        let outcome = run_msbar(MsbarParams {
            input,
            mutations: params.mutations,
        })?;

        let output_provenance =
            ArtifactProvenance::generated_output("stdout").with_description("msbar FASTA output");
        let report = self.success_report(
            &request.context,
            format!("mutated {} records", outcome.records.len()),
            input_diagnostics,
            vec![input_provenance, output_provenance.clone()],
        );
        let result = MethodResult::new(
            request.tool.clone(),
            ResultPayload::SequenceCollection(outcome.records),
            ResultSummary::new("Point mutation editing completed")
                .with_line(format!("Input: {}", outcome.input.path.display()))
                .with_line(format!("Mutations: {}", mutation_count))
                .with_line("Mutation syntax: 1-based position:residue")
                .with_line("Output format: fasta (feature annotations dropped in v1)"),
            report.clone(),
        )
        .with_artifact(
            ArtifactReference::new("msbar-sequences", ArtifactKind::Sequence)
                .with_label("Mutated sequences")
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

    fn invoke_trimest(
        &self,
        request: InvocationRequest,
        descriptor: ToolDescriptor,
    ) -> Result<InvocationResponse, ServiceError> {
        if help_requested(request.arguments()) {
            return Ok(self.help_response(request, descriptor, trimest_help()));
        }

        let params = parse_trimest_params(request.arguments())?;
        let input_path = params.input.path.display().to_string();
        let (input, input_provenance, input_diagnostics) =
            self.resolve_local_sequence_input(&input_path)?;
        let outcome = run_trimest(TrimestParams {
            input,
            min_tail: params.min_tail,
        })?;

        let output_provenance =
            ArtifactProvenance::generated_output("stdout").with_description("trimest FASTA output");
        let report = self.success_report(
            &request.context,
            format!(
                "trimmed poly-A tails from {} records",
                outcome.records.len()
            ),
            input_diagnostics,
            vec![input_provenance, output_provenance.clone()],
        );
        let result = MethodResult::new(
            request.tool.clone(),
            ResultPayload::SequenceCollection(outcome.records),
            ResultSummary::new("Poly-A tail trimming completed")
                .with_line(format!("Input: {}", outcome.input.path.display()))
                .with_line(format!("Minimum tail length: {}", outcome.min_tail))
                .with_line("Trim rule: trailing 3' runs of 'A' only")
                .with_line("Output format: fasta (feature annotations dropped in v1)"),
            report.clone(),
        )
        .with_artifact(
            ArtifactReference::new("trimest-sequences", ArtifactKind::Sequence)
                .with_label("Trimmed nucleotide sequences")
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

    fn invoke_vectorstrip(
        &self,
        request: InvocationRequest,
        descriptor: ToolDescriptor,
    ) -> Result<InvocationResponse, ServiceError> {
        if help_requested(request.arguments()) {
            return Ok(self.help_response(request, descriptor, vectorstrip_help()));
        }

        let params = parse_vectorstrip_params(request.arguments())?;
        let input_path = params.input.path.display().to_string();
        let vector_path = params.vector.path.display().to_string();
        let (input, input_provenance, mut diagnostics) =
            self.resolve_local_sequence_input(&input_path)?;
        let (vector, vector_provenance, vector_diagnostics) =
            self.resolve_local_sequence_input(&vector_path)?;
        diagnostics.extend(vector_diagnostics);
        let outcome = run_vectorstrip(VectorstripParams { input, vector })?;

        let output_provenance = ArtifactProvenance::generated_output("stdout")
            .with_description("vectorstrip FASTA output");
        let report = self.success_report(
            &request.context,
            format!(
                "stripped vector matches from {} records",
                outcome.records.len()
            ),
            diagnostics,
            vec![
                input_provenance,
                vector_provenance,
                output_provenance.clone(),
            ],
        );
        let result = MethodResult::new(
            request.tool.clone(),
            ResultPayload::SequenceCollection(outcome.records),
            ResultSummary::new("Vector stripping completed")
                .with_line(format!("Input: {}", outcome.input.path.display()))
                .with_line(format!("Vector input: {}", outcome.vector.path.display()))
                .with_line("Match rule: exact full-length terminal vector matches only")
                .with_line("Output format: fasta (feature annotations dropped in v1)"),
            report.clone(),
        )
        .with_artifact(
            ArtifactReference::new("vectorstripped-sequences", ArtifactKind::Sequence)
                .with_label("Vector-stripped sequences")
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

    fn invoke_infoseq(
        &self,
        request: InvocationRequest,
        descriptor: ToolDescriptor,
    ) -> Result<InvocationResponse, ServiceError> {
        if help_requested(request.arguments()) {
            return Ok(self.help_response(request, descriptor, infoseq_help()));
        }

        let params = parse_infoseq_params(request.arguments())?;
        let input_path = params.input.path.display().to_string();
        let (input, input_provenance, input_diagnostics) =
            self.resolve_local_sequence_input(&input_path)?;
        let outcome = run_infoseq(InfoseqParams { input })?;

        let rows = outcome
            .rows
            .iter()
            .map(|row| {
                vec![
                    row.ordinal.to_string(),
                    row.identifier.clone(),
                    row.display_name.clone().unwrap_or_else(|| "-".to_owned()),
                    row.length.to_string(),
                    row.molecule.clone(),
                    row.alphabet.clone(),
                    row.gc_percent
                        .map(|gc| format!("{gc:.2}"))
                        .unwrap_or_else(|| "-".to_owned()),
                    row.feature_count.to_string(),
                    row.description.clone().unwrap_or_else(|| "-".to_owned()),
                    row.organism.clone().unwrap_or_else(|| "-".to_owned()),
                ]
            })
            .collect();

        let report = self.success_report(
            &request.context,
            format!(
                "reported basic sequence information for {} records",
                outcome.rows.len()
            ),
            input_diagnostics,
            vec![input_provenance],
        );
        let result = MethodResult::new(
            request.tool.clone(),
            ResultPayload::TableReport(TableReport::new(
                vec![
                    "ordinal".to_owned(),
                    "identifier".to_owned(),
                    "display_name".to_owned(),
                    "length".to_owned(),
                    "molecule".to_owned(),
                    "alphabet".to_owned(),
                    "gc_percent".to_owned(),
                    "feature_count".to_owned(),
                    "description".to_owned(),
                    "organism".to_owned(),
                ],
                rows,
            )),
            ResultSummary::new("Basic sequence information reported")
                .with_line(format!("Input: {}", outcome.input.path.display()))
                .with_line("Scope: one stable row per input record")
                .with_line("GC policy: reported only for nucleotide-like records")
                .with_line(format!("Records: {}", outcome.rows.len())),
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

    fn invoke_aaindexextract(
        &self,
        request: InvocationRequest,
        descriptor: ToolDescriptor,
    ) -> Result<InvocationResponse, ServiceError> {
        if help_requested(request.arguments()) {
            return Ok(self.help_response(request, descriptor, aaindexextract_help()));
        }

        let [index]: [String; 1] = request
            .arguments
            .clone()
            .try_into()
            .map_err(|_| tool_usage_error("aaindexextract", aaindexextract_help()))?;
        let outcome = run_aaindexextract(AaindexextractParams {
            index: parse_aaindexextract_index(&index)?,
        })?;

        let rows = outcome
            .rows
            .iter()
            .map(|row| {
                vec![
                    row.index.clone(),
                    row.residue.to_string(),
                    row.three_letter.clone(),
                    row.name.clone(),
                    row.value.clone(),
                    row.units.clone(),
                    row.notes.clone(),
                ]
            })
            .collect::<Vec<_>>();

        let report = self.success_report(
            &request.context,
            format!("reported {} amino-acid property rows", outcome.rows.len()),
            Vec::new(),
            Vec::new(),
        );
        let result = MethodResult::new(
            request.tool.clone(),
            ResultPayload::TableReport(TableReport::new(
                vec![
                    "index".to_owned(),
                    "residue".to_owned(),
                    "three_letter".to_owned(),
                    "name".to_owned(),
                    "value".to_owned(),
                    "units".to_owned(),
                    "notes".to_owned(),
                ],
                rows,
            )),
            ResultSummary::new("Amino-acid property index reported")
                .with_line(format!("Index: {}", outcome.index.name()))
                .with_line("Coverage: governed built-in subset, not full historical AAINDEX")
                .with_line(format!("Rows: {}", outcome.rows.len())),
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

    fn invoke_maskambignuc(
        &self,
        request: InvocationRequest,
        descriptor: ToolDescriptor,
    ) -> Result<InvocationResponse, ServiceError> {
        if help_requested(request.arguments()) {
            return Ok(self.help_response(request, descriptor, maskambignuc_help()));
        }

        let input =
            parse_maskambig_params("maskambignuc", request.arguments(), maskambignuc_help())?;
        let input_path = input.path.display().to_string();
        let (input, input_provenance, input_diagnostics) =
            self.resolve_local_sequence_input(&input_path)?;
        let outcome = run_maskambignuc(MaskambignucParams { input })?;

        let output_provenance = ArtifactProvenance::generated_output("stdout")
            .with_description("ambiguity-masked FASTA output");
        let report = self.success_report(
            &request.context,
            format!(
                "masked {} ambiguous nucleotide residues across {} records",
                outcome.masked_residue_count,
                outcome.records.len()
            ),
            input_diagnostics,
            vec![input_provenance, output_provenance.clone()],
        );
        let result = MethodResult::new(
            request.tool.clone(),
            ResultPayload::SequenceCollection(outcome.records),
            ResultSummary::new("Nucleotide ambiguity masking completed")
                .with_line(format!("Input: {}", outcome.input.path.display()))
                .with_line(format!(
                    "Masked ambiguity residues: {}",
                    outcome.masked_residue_count
                ))
                .with_line("Mask rule: conservative nucleotide ambiguity symbols -> N")
                .with_line("Output format: fasta (annotations retained in payload)"),
            report.clone(),
        )
        .with_artifact(
            ArtifactReference::new(
                "ambiguity-masked-nucleotide-sequences",
                ArtifactKind::Sequence,
            )
            .with_label("Ambiguity-masked nucleotide sequences")
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

    fn invoke_maskambigprot(
        &self,
        request: InvocationRequest,
        descriptor: ToolDescriptor,
    ) -> Result<InvocationResponse, ServiceError> {
        if help_requested(request.arguments()) {
            return Ok(self.help_response(request, descriptor, maskambigprot_help()));
        }

        let input =
            parse_maskambig_params("maskambigprot", request.arguments(), maskambigprot_help())?;
        let input_path = input.path.display().to_string();
        let (input, input_provenance, input_diagnostics) =
            self.resolve_local_sequence_input(&input_path)?;
        let outcome = run_maskambigprot(MaskambigprotParams { input })?;

        let output_provenance = ArtifactProvenance::generated_output("stdout")
            .with_description("ambiguity-masked protein FASTA output");
        let report = self.success_report(
            &request.context,
            format!(
                "masked {} ambiguous protein residues across {} records",
                outcome.masked_residue_count,
                outcome.records.len()
            ),
            input_diagnostics,
            vec![input_provenance, output_provenance.clone()],
        );
        let result = MethodResult::new(
            request.tool.clone(),
            ResultPayload::SequenceCollection(outcome.records),
            ResultSummary::new("Protein ambiguity masking completed")
                .with_line(format!("Input: {}", outcome.input.path.display()))
                .with_line(format!(
                    "Masked ambiguity residues: {}",
                    outcome.masked_residue_count
                ))
                .with_line("Mask rule: conservative protein ambiguity symbols -> X")
                .with_line("Output format: fasta (annotations retained in payload)"),
            report.clone(),
        )
        .with_artifact(
            ArtifactReference::new("ambiguity-masked-protein-sequences", ArtifactKind::Sequence)
                .with_label("Ambiguity-masked protein sequences")
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

    fn invoke_twofeat(
        &self,
        request: InvocationRequest,
        descriptor: ToolDescriptor,
    ) -> Result<InvocationResponse, ServiceError> {
        if help_requested(request.arguments()) {
            return Ok(self.help_response(request, descriptor, twofeat_help()));
        }

        let params = parse_twofeat_params(request.arguments())?;
        let input_path = params.input.path.display().to_string();
        let a_selector_summary = describe_selector(&params.a_selector);
        let b_selector_summary = describe_selector(&params.b_selector);
        let (input, input_provenance, input_diagnostics) =
            self.resolve_local_sequence_input(&input_path)?;
        let outcome = run_twofeat(TwofeatParams {
            input,
            a_selector: params.a_selector.clone(),
            b_selector: params.b_selector.clone(),
            min_range: params.min_range,
            max_range: params.max_range,
        })?;

        let output_provenance = ArtifactProvenance::generated_output("stdout")
            .with_description("neighbouring feature-pair report");
        let report = self.success_report(
            &request.context,
            format!("reported {} neighbouring feature pairs", outcome.rows.len()),
            input_diagnostics,
            vec![input_provenance, output_provenance.clone()],
        );
        let result = MethodResult::new(
            request.tool.clone(),
            ResultPayload::TableReport(TableReport::new(
                vec![
                    "record".to_owned(),
                    "a_kind".to_owned(),
                    "a_location".to_owned(),
                    "a_start".to_owned(),
                    "a_end".to_owned(),
                    "a_strand".to_owned(),
                    "a_name".to_owned(),
                    "b_kind".to_owned(),
                    "b_location".to_owned(),
                    "b_start".to_owned(),
                    "b_end".to_owned(),
                    "b_strand".to_owned(),
                    "b_name".to_owned(),
                    "gap".to_owned(),
                    "relation".to_owned(),
                    "strand_relation".to_owned(),
                ],
                outcome.rows,
            )),
            ResultSummary::new("Neighbouring feature pair report completed")
                .with_line(format!("Input: {}", outcome.input.path.display()))
                .with_line(format!("A selector: {a_selector_summary}"))
                .with_line(format!("B selector: {b_selector_summary}"))
                .with_line(format!(
                    "Distance constraints: {}",
                    describe_range_constraints(outcome.min_range, outcome.max_range)
                ))
                .with_line("Neighbour rule: adjacent features in source order")
                .with_line("Output format: governed table report"),
            report.clone(),
        )
        .with_artifact(
            ArtifactReference::new("feature-pair-report", ArtifactKind::Table)
                .with_label("Neighbouring feature pair report")
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

        let output_provenance =
            ArtifactProvenance::generated_output("stdout").with_description(if outcome.translate {
                "coding-feature translated FASTA output"
            } else {
                "coding-feature extracted FASTA output"
            });
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

    fn invoke_splitsource(
        &self,
        request: InvocationRequest,
        descriptor: ToolDescriptor,
    ) -> Result<InvocationResponse, ServiceError> {
        if help_requested(request.arguments()) {
            return Ok(self.help_response(request, descriptor, splitsource_help()));
        }

        let [input]: [String; 1] = request
            .arguments
            .clone()
            .try_into()
            .map_err(|_| tool_usage_error("splitsource", splitsource_help()))?;
        let (input, input_provenance, input_diagnostics) =
            self.resolve_local_sequence_input(&input)?;
        let outcome = run_splitsource(SplitsourceParams { input })?;

        let output_provenance = ArtifactProvenance::generated_output("stdout")
            .with_description("split source-fragment FASTA output");
        let report = self.success_report(
            &request.context,
            format!(
                "split synthetic input into {} source fragments",
                outcome.fragment_count
            ),
            input_diagnostics,
            vec![input_provenance, output_provenance.clone()],
        );
        let result = MethodResult::new(
            request.tool.clone(),
            ResultPayload::SequenceCollection(outcome.records),
            ResultSummary::new("Source fragments extracted")
                .with_line(format!("Input: {}", outcome.input.path.display()))
                .with_line(format!("Fragments: {}", outcome.fragment_count))
                .with_line("Selection rule: simple `source` features in source order")
                .with_line("Output format: fasta"),
            report.clone(),
        )
        .with_artifact(
            ArtifactReference::new("source-fragments", ArtifactKind::Sequence)
                .with_label("Source fragments")
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

        let output_provenance =
            ArtifactProvenance::generated_output("stdout").with_description("ORF FASTA output");
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
            format!(
                "rendered pretty sequence view for {} records",
                outcome.record_count
            ),
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
                .with_line(
                    "Translation policy: forward frame only, trailing partial codons ignored",
                )
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
            format!(
                "projected {} aligned rows into codon space",
                outcome.alignment.row_count()
            ),
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

    fn invoke_preg(
        &self,
        request: InvocationRequest,
        descriptor: ToolDescriptor,
    ) -> Result<InvocationResponse, ServiceError> {
        if help_requested(request.arguments()) {
            return Ok(self.help_response(request, descriptor, preg_help()));
        }

        let arguments: [String; 2] = request
            .arguments
            .clone()
            .try_into()
            .map_err(|_| tool_usage_error("preg", preg_help()))?;
        let (input, input_provenance, input_diagnostics) =
            self.resolve_local_sequence_input(&arguments[0])?;
        let outcome = run_preg(PregParams {
            input,
            pattern: parse_protein_regex("preg", &arguments[1])?,
        })?;

        let report = self.success_report(
            &request.context,
            format!("reported {} protein regex hits", outcome.hits.len()),
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
            ResultSummary::new("Protein regular-expression search completed")
                .with_line(format!("Input: {}", outcome.input.path.display()))
                .with_line(format!("Pattern: {}", outcome.pattern))
                .with_line("Coordinate convention: 1-based inclusive")
                .with_line("Pattern model: bounded Rust regex over protein residues")
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

    fn invoke_dreg(
        &self,
        request: InvocationRequest,
        descriptor: ToolDescriptor,
    ) -> Result<InvocationResponse, ServiceError> {
        if help_requested(request.arguments()) {
            return Ok(self.help_response(request, descriptor, dreg_help()));
        }

        let arguments: [String; 2] = request
            .arguments
            .clone()
            .try_into()
            .map_err(|_| tool_usage_error("dreg", dreg_help()))?;
        let (input, input_provenance, input_diagnostics) =
            self.resolve_local_sequence_input(&arguments[0])?;
        let outcome = run_dreg(DregParams {
            input,
            pattern: parse_nucleotide_regex("dreg", &arguments[1])?,
        })?;

        let report = self.success_report(
            &request.context,
            format!("reported {} nucleotide regex hits", outcome.hits.len()),
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
            ResultSummary::new("Nucleotide regular-expression search completed")
                .with_line(format!("Input: {}", outcome.input.path.display()))
                .with_line(format!("Pattern: {}", outcome.pattern))
                .with_line("Coordinate convention: 1-based inclusive")
                .with_line("Pattern model: bounded Rust regex over nucleotide residues")
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

    fn invoke_einverted(
        &self,
        request: InvocationRequest,
        descriptor: ToolDescriptor,
    ) -> Result<InvocationResponse, ServiceError> {
        if help_requested(request.arguments()) {
            return Ok(self.help_response(request, descriptor, einverted_help()));
        }

        let cli = parse_einverted_params(request.arguments())?;
        let input_argument = cli.input.path.display().to_string();
        let (input, input_provenance, input_diagnostics) =
            self.resolve_local_sequence_input(&input_argument)?;
        let outcome = run_einverted(EinvertedParams {
            input,
            min_arm_length: cli.min_arm_length,
            max_gap_length: cli.max_gap_length,
        })?;

        let report = self.success_report(
            &request.context,
            format!("reported {} exact inverted repeats", outcome.hits.len()),
            input_diagnostics,
            vec![input_provenance],
        );
        let rows = outcome
            .hits
            .iter()
            .map(|hit| {
                vec![
                    hit.record_id.clone(),
                    (hit.left_start + 1).to_string(),
                    hit.left_end.to_string(),
                    (hit.right_start + 1).to_string(),
                    hit.right_end.to_string(),
                    hit.gap_length.to_string(),
                    hit.left_arm.len().to_string(),
                    hit.left_arm.clone(),
                    hit.right_arm.clone(),
                ]
            })
            .collect();
        let result = MethodResult::new(
            request.tool.clone(),
            ResultPayload::TableReport(TableReport::new(
                vec![
                    "record".to_owned(),
                    "left_start".to_owned(),
                    "left_end".to_owned(),
                    "right_start".to_owned(),
                    "right_end".to_owned(),
                    "gap_length".to_owned(),
                    "arm_length".to_owned(),
                    "left_arm".to_owned(),
                    "right_arm".to_owned(),
                ],
                rows,
            )),
            ResultSummary::new("Exact inverted-repeat search completed")
                .with_line(format!("Input: {}", outcome.input.path.display()))
                .with_line(format!("Minimum arm length: {}", outcome.min_arm_length))
                .with_line(format!("Maximum gap length: {}", outcome.max_gap_length))
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

    fn invoke_patmatdb(
        &self,
        request: InvocationRequest,
        descriptor: ToolDescriptor,
    ) -> Result<InvocationResponse, ServiceError> {
        if help_requested(request.arguments()) {
            return Ok(self.help_response(request, descriptor, patmatdb_help()));
        }

        let cli = parse_patmatdb_params(request.arguments())?;
        let input_argument = cli.input.path.display().to_string();
        let (input, input_provenance, input_diagnostics) =
            self.resolve_local_sequence_input(&input_argument)?;
        let outcome = run_patmatdb(PatmatdbParams {
            input,
            database: cli.database,
        })?;

        let report = self.success_report(
            &request.context,
            format!("reported {} motif-database hits", outcome.hits.len()),
            input_diagnostics,
            vec![input_provenance],
        );
        let rows = outcome
            .hits
            .iter()
            .map(|hit| {
                vec![
                    hit.record_id.clone(),
                    hit.motif_id.clone(),
                    hit.pattern.clone(),
                    hit.description.clone().unwrap_or_else(|| "-".to_owned()),
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
                    "motif_id".to_owned(),
                    "pattern".to_owned(),
                    "description".to_owned(),
                    "start".to_owned(),
                    "end".to_owned(),
                    "matched".to_owned(),
                ],
                rows,
            )),
            ResultSummary::new("Protein motif-database search completed")
                .with_line(format!("Input: {}", outcome.input.path.display()))
                .with_line(format!("Database: {}", outcome.database.display()))
                .with_line("Coordinate convention: 1-based inclusive")
                .with_line(format!("Motifs loaded: {}", outcome.motifs.len()))
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

    fn invoke_palindrome(
        &self,
        request: InvocationRequest,
        descriptor: ToolDescriptor,
    ) -> Result<InvocationResponse, ServiceError> {
        if help_requested(request.arguments()) {
            return Ok(self.help_response(request, descriptor, palindrome_help()));
        }

        let cli = parse_palindrome_params(request.arguments())?;
        let input_argument = cli.input.path.display().to_string();
        let (input, input_provenance, input_diagnostics) =
            self.resolve_local_sequence_input(&input_argument)?;
        let outcome = run_palindrome(PalindromeParams {
            input,
            min_length: cli.min_length,
            max_length: cli.max_length,
        })?;

        let report = self.success_report(
            &request.context,
            format!("reported {} palindromic windows", outcome.hits.len()),
            input_diagnostics,
            vec![input_provenance],
        );
        let rows = outcome
            .hits
            .iter()
            .map(|hit| {
                vec![
                    hit.record_id.clone(),
                    (hit.start + 1).to_string(),
                    hit.end.to_string(),
                    hit.matched.len().to_string(),
                    hit.matched.clone(),
                ]
            })
            .collect();
        let result = MethodResult::new(
            request.tool.clone(),
            ResultPayload::TableReport(TableReport::new(
                vec![
                    "record".to_owned(),
                    "start".to_owned(),
                    "end".to_owned(),
                    "length".to_owned(),
                    "matched".to_owned(),
                ],
                rows,
            )),
            ResultSummary::new("Palindrome search completed")
                .with_line(format!("Input: {}", outcome.input.path.display()))
                .with_line(format!("Minimum length: {}", outcome.min_length))
                .with_line(format!("Maximum length: {}", outcome.max_length))
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

    fn invoke_seqmatchall(
        &self,
        request: InvocationRequest,
        descriptor: ToolDescriptor,
    ) -> Result<InvocationResponse, ServiceError> {
        if help_requested(request.arguments()) {
            return Ok(self.help_response(request, descriptor, seqmatchall_help()));
        }

        let cli = parse_seqmatchall_params(request.arguments())?;
        let input_argument = cli.input.path.display().to_string();
        let (input, input_provenance, input_diagnostics) =
            self.resolve_local_sequence_input(&input_argument)?;
        let outcome = run_seqmatchall(SeqmatchallParams {
            input,
            word_size: cli.word_size,
        })?;

        let rows = outcome
            .hits
            .iter()
            .map(|hit| {
                vec![
                    hit.left_id.clone(),
                    hit.right_id.clone(),
                    (hit.left_start + 1).to_string(),
                    hit.left_end.to_string(),
                    (hit.right_start + 1).to_string(),
                    hit.right_end.to_string(),
                    hit.matched.len().to_string(),
                    hit.matched.clone(),
                ]
            })
            .collect();
        let report = self.success_report(
            &request.context,
            format!(
                "reported {} all-against-all exact shared regions",
                outcome.hits.len()
            ),
            input_diagnostics,
            vec![input_provenance],
        );
        let result = MethodResult::new(
            request.tool.clone(),
            ResultPayload::TableReport(TableReport::new(
                vec![
                    "left".to_owned(),
                    "right".to_owned(),
                    "left_start".to_owned(),
                    "left_end".to_owned(),
                    "right_start".to_owned(),
                    "right_end".to_owned(),
                    "length".to_owned(),
                    "matched".to_owned(),
                ],
                rows,
            )),
            ResultSummary::new("All-against-all exact shared-region search completed")
                .with_line(format!("Input: {}", outcome.input.path.display()))
                .with_line(format!("Minimum word size: {}", outcome.word_size))
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

    fn invoke_wordmatch(
        &self,
        request: InvocationRequest,
        descriptor: ToolDescriptor,
    ) -> Result<InvocationResponse, ServiceError> {
        if help_requested(request.arguments()) {
            return Ok(self.help_response(request, descriptor, wordmatch_help()));
        }

        if request.arguments().len() < 2 {
            return Err(tool_usage_error("wordmatch", wordmatch_help()));
        }
        let query_argument = request.arguments()[0].clone();
        let target_argument = request.arguments()[1].clone();
        let (query, query_provenance, mut diagnostics) =
            self.resolve_local_sequence_input(&query_argument)?;
        let (target, target_provenance, target_diagnostics) =
            self.resolve_local_sequence_input(&target_argument)?;
        diagnostics.extend(target_diagnostics);
        let params = parse_wordmatch_params(request.arguments())?;
        let outcome = run_wordmatch(WordmatchParams {
            query,
            target,
            word_size: params.word_size,
        })?;

        let rows = outcome
            .hits
            .iter()
            .map(|hit| {
                vec![
                    hit.query_id.clone(),
                    hit.target_id.clone(),
                    (hit.query_start + 1).to_string(),
                    hit.query_end.to_string(),
                    (hit.target_start + 1).to_string(),
                    hit.target_end.to_string(),
                    hit.matched.len().to_string(),
                    hit.matched.clone(),
                ]
            })
            .collect();
        let report = self.success_report(
            &request.context,
            format!("reported {} exact shared regions", outcome.hits.len()),
            diagnostics,
            vec![query_provenance, target_provenance],
        );
        let result = MethodResult::new(
            request.tool.clone(),
            ResultPayload::TableReport(TableReport::new(
                vec![
                    "query".to_owned(),
                    "target".to_owned(),
                    "query_start".to_owned(),
                    "query_end".to_owned(),
                    "target_start".to_owned(),
                    "target_end".to_owned(),
                    "length".to_owned(),
                    "matched".to_owned(),
                ],
                rows,
            )),
            ResultSummary::new("Exact shared-region search completed")
                .with_line(format!("Query: {}", outcome.query.path.display()))
                .with_line(format!("Target: {}", outcome.target.path.display()))
                .with_line(format!("Minimum word size: {}", outcome.word_size))
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

    fn invoke_wordfinder(
        &self,
        request: InvocationRequest,
        descriptor: ToolDescriptor,
    ) -> Result<InvocationResponse, ServiceError> {
        if help_requested(request.arguments()) {
            return Ok(self.help_response(request, descriptor, wordfinder_help()));
        }

        if request.arguments().len() < 2 {
            return Err(tool_usage_error("wordfinder", wordfinder_help()));
        }
        let query_argument = request.arguments()[0].clone();
        let targets_argument = request.arguments()[1].clone();
        let (query, query_provenance, mut diagnostics) =
            self.resolve_local_sequence_input(&query_argument)?;
        let (targets, targets_provenance, target_diagnostics) =
            self.resolve_local_sequence_input(&targets_argument)?;
        diagnostics.extend(target_diagnostics);
        let params = parse_wordfinder_params(request.arguments())?;
        let outcome = run_wordfinder(WordfinderParams {
            query,
            targets,
            word_size: params.word_size,
        })?;

        let rows = outcome
            .hits
            .iter()
            .map(|hit| {
                vec![
                    hit.query_id.clone(),
                    hit.target_id.clone(),
                    (hit.query_start + 1).to_string(),
                    hit.query_end.to_string(),
                    (hit.target_start + 1).to_string(),
                    hit.target_end.to_string(),
                    hit.matched.len().to_string(),
                    hit.matched.clone(),
                ]
            })
            .collect();
        let report = self.success_report(
            &request.context,
            format!(
                "reported {} exact shared regions across target set",
                outcome.hits.len()
            ),
            diagnostics,
            vec![query_provenance, targets_provenance],
        );
        let result = MethodResult::new(
            request.tool.clone(),
            ResultPayload::TableReport(TableReport::new(
                vec![
                    "query".to_owned(),
                    "target".to_owned(),
                    "query_start".to_owned(),
                    "query_end".to_owned(),
                    "target_start".to_owned(),
                    "target_end".to_owned(),
                    "length".to_owned(),
                    "matched".to_owned(),
                ],
                rows,
            )),
            ResultSummary::new("Multi-target exact shared-region search completed")
                .with_line(format!("Query: {}", outcome.query.path.display()))
                .with_line(format!("Targets: {}", outcome.targets.path.display()))
                .with_line(format!("Minimum word size: {}", outcome.word_size))
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

    fn invoke_pepwindow(
        &self,
        request: InvocationRequest,
        descriptor: ToolDescriptor,
    ) -> Result<InvocationResponse, ServiceError> {
        if help_requested(request.arguments()) {
            return Ok(self.help_response(request, descriptor, pepwindow_help()));
        }

        let (params, plot_contract_out) = parse_pepwindow_params(request.arguments())?;
        let (input, input_provenance, input_diagnostics) =
            self.resolve_local_sequence_input(&params.input.path.display().to_string())?;
        let outcome = run_pepwindow(PepwindowParams {
            input,
            window: params.window,
            step: params.step,
        })?;
        let status_message = format!(
            "computed pepwindow hydropathy profile across {} windows",
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
                    "mean_hydropathy".to_owned(),
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
                            format!("{:.6}", window.mean_hydropathy),
                        ]
                    })
                    .collect(),
            )),
            ResultSummary::new("Protein hydropathy profile computed")
                .with_line(format!("Input: {}", outcome.input.path.display()))
                .with_line(format!("Sequence: {}", outcome.profile.identifier))
                .with_line(format!("Window: {}", outcome.profile.window))
                .with_line(format!("Step: {}", outcome.profile.step))
                .with_line("Scale: Kyte-Doolittle hydropathy")
                .with_line("X axis: 1-based window start")
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
                    format!("failed to serialize pepwindow plot contract: {error}"),
                )
                .with_code("service.pepwindow.plot.serialize_failed")
            })?;
            std::fs::write(path, json).map_err(|error| {
                PlatformError::new(
                    ErrorCategory::Configuration,
                    format!(
                        "failed to write pepwindow plot contract to {}",
                        path.display()
                    ),
                )
                .with_code("service.pepwindow.plot.write_failed")
                .with_detail(error.to_string())
            })?;

            let plot_provenance = ArtifactProvenance::generated_output(path.display().to_string())
                .with_description("pepwindow plot contract");
            provenance.push(plot_provenance.clone());
            result = result.with_artifact(
                ArtifactReference::new("pepwindow-plot-contract", ArtifactKind::Auxiliary)
                    .with_label("Pepwindow plot contract")
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

    fn invoke_hmoment(
        &self,
        request: InvocationRequest,
        descriptor: ToolDescriptor,
    ) -> Result<InvocationResponse, ServiceError> {
        if help_requested(request.arguments()) {
            return Ok(self.help_response(request, descriptor, hmoment_help()));
        }

        let (params, plot_contract_out) = parse_hmoment_params(request.arguments())?;
        let (input, input_provenance, input_diagnostics) =
            self.resolve_local_sequence_input(&params.input.path.display().to_string())?;
        let outcome = run_hmoment(HmomentParams {
            input,
            window: params.window,
            step: params.step,
            angle_degrees: params.angle_degrees,
        })?;
        let status_message = format!(
            "computed hydrophobic-moment profile across {} windows",
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
                    "hydrophobic_moment".to_owned(),
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
                            format!("{:.6}", window.hydrophobic_moment),
                        ]
                    })
                    .collect(),
            )),
            ResultSummary::new("Protein hydrophobic-moment profile computed")
                .with_line(format!("Input: {}", outcome.input.path.display()))
                .with_line(format!("Sequence: {}", outcome.profile.identifier))
                .with_line(format!("Window: {}", outcome.profile.window))
                .with_line(format!("Step: {}", outcome.profile.step))
                .with_line(format!(
                    "Angle: {:.1} degrees",
                    outcome.profile.angle_degrees
                ))
                .with_line("X axis: 1-based window start")
                .with_line("Y axis: hydrophobic moment")
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
                    format!("failed to serialize hmoment plot contract: {error}"),
                )
                .with_code("service.hmoment.plot.serialize_failed")
            })?;
            std::fs::write(path, json).map_err(|error| {
                PlatformError::new(
                    ErrorCategory::Configuration,
                    format!(
                        "failed to write hmoment plot contract to {}",
                        path.display()
                    ),
                )
                .with_code("service.hmoment.plot.write_failed")
                .with_detail(error.to_string())
            })?;

            let plot_provenance = ArtifactProvenance::generated_output(path.display().to_string())
                .with_description("hmoment plot contract");
            provenance.push(plot_provenance.clone());
            result = result.with_artifact(
                ArtifactReference::new("hmoment-plot-contract", ArtifactKind::Auxiliary)
                    .with_label("Hmoment plot contract")
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

    fn invoke_density(
        &self,
        request: InvocationRequest,
        descriptor: ToolDescriptor,
    ) -> Result<InvocationResponse, ServiceError> {
        if help_requested(request.arguments()) {
            return Ok(self.help_response(request, descriptor, density_help()));
        }

        let (params, plot_contract_out) = parse_density_params(request.arguments())?;
        let (input, input_provenance, input_diagnostics) =
            self.resolve_local_sequence_input(&params.input.path.display().to_string())?;
        let outcome = run_density(DensityParams {
            input,
            window: params.window,
            step: params.step,
        })?;
        let status_message = format!(
            "computed bounded nucleotide density profile across {} windows",
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
                    "canonical_symbols".to_owned(),
                    "ambiguous_symbols".to_owned(),
                    "ignored_gap_symbols".to_owned(),
                    "adenine_fraction".to_owned(),
                    "cytosine_fraction".to_owned(),
                    "guanine_fraction".to_owned(),
                    "thymine_or_uracil_fraction".to_owned(),
                    "at_fraction".to_owned(),
                    "gc_fraction".to_owned(),
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
                            window.canonical_symbols.to_string(),
                            window.ambiguous_symbols.to_string(),
                            window.ignored_gap_symbols.to_string(),
                            format!("{:.6}", window.adenine_fraction),
                            format!("{:.6}", window.cytosine_fraction),
                            format!("{:.6}", window.guanine_fraction),
                            format!("{:.6}", window.thymine_or_uracil_fraction),
                            format!("{:.6}", window.at_fraction),
                            format!("{:.6}", window.gc_fraction),
                        ]
                    })
                    .collect(),
            )),
            ResultSummary::new("Nucleotide density profile computed")
                .with_line(format!("Input: {}", outcome.input.path.display()))
                .with_line(format!("Sequence: {}", outcome.profile.identifier))
                .with_line(format!("Window: {}", outcome.profile.window))
                .with_line(format!("Step: {}", outcome.profile.step))
                .with_line("X axis: 1-based window start")
                .with_line("Y axis: GC fraction")
                .with_line("Analytical table also reports A/C/G/T(U), AT, GC, canonical, ambiguous, and gap counts")
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
                    format!("failed to serialize density plot contract: {error}"),
                )
                .with_code("service.density.plot.serialize_failed")
            })?;
            std::fs::write(path, json).map_err(|error| {
                PlatformError::new(
                    ErrorCategory::Configuration,
                    format!(
                        "failed to write density plot contract to {}",
                        path.display()
                    ),
                )
                .with_code("service.density.plot.write_failed")
                .with_detail(error.to_string())
            })?;

            let plot_provenance = ArtifactProvenance::generated_output(path.display().to_string())
                .with_description("density plot contract");
            provenance.push(plot_provenance.clone());
            result = result.with_artifact(
                ArtifactReference::new("density-plot-contract", ArtifactKind::Auxiliary)
                    .with_label("Density plot contract")
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

    fn invoke_banana(
        &self,
        request: InvocationRequest,
        descriptor: ToolDescriptor,
    ) -> Result<InvocationResponse, ServiceError> {
        if help_requested(request.arguments()) {
            return Ok(self.help_response(request, descriptor, banana_help()));
        }

        let (params, plot_contract_out) = parse_banana_params(request.arguments())?;
        let (input, input_provenance, input_diagnostics) =
            self.resolve_local_sequence_input(&params.input.path.display().to_string())?;
        let outcome = run_banana(BananaParams { input })?;
        let status_message = format!(
            "computed bounded banana profile across {} positions",
            outcome.profile.points.len()
        );

        let mut provenance = vec![input_provenance];
        let mut result = MethodResult::new(
            request.tool.clone(),
            ResultPayload::TableReport(TableReport::new(
                vec![
                    "sequence_id".to_owned(),
                    "position".to_owned(),
                    "residue".to_owned(),
                    "local_bend".to_owned(),
                    "curvature".to_owned(),
                ],
                outcome
                    .profile
                    .points
                    .iter()
                    .map(|point| {
                        vec![
                            outcome.profile.identifier.clone(),
                            point.position.to_string(),
                            point.residue.to_string(),
                            point
                                .local_bend
                                .map(|value| format!("{value:.6}"))
                                .unwrap_or_default(),
                            point
                                .curvature
                                .map(|value| format!("{value:.6}"))
                                .unwrap_or_default(),
                        ]
                    })
                    .collect(),
            )),
            ResultSummary::new("Banana profile computed")
                .with_line(format!("Input: {}", outcome.input.path.display()))
                .with_line(format!("Sequence: {}", outcome.profile.identifier))
                .with_line("X axis: 1-based position")
                .with_line("Y axis: curvature")
                .with_line(
                    "Analytical table also reports local bend values and leaves edge positions blank where the bounded model does not define bend or curvature",
                )
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
                    format!("failed to serialize banana plot contract: {error}"),
                )
                .with_code("service.banana.plot.serialize_failed")
            })?;
            std::fs::write(path, json).map_err(|error| {
                PlatformError::new(
                    ErrorCategory::Configuration,
                    format!("failed to write banana plot contract to {}", path.display()),
                )
                .with_code("service.banana.plot.write_failed")
                .with_detail(error.to_string())
            })?;

            let plot_provenance = ArtifactProvenance::generated_output(path.display().to_string())
                .with_description("banana plot contract");
            provenance.push(plot_provenance.clone());
            result = result.with_artifact(
                ArtifactReference::new("banana-plot-contract", ArtifactKind::Auxiliary)
                    .with_label("Banana plot contract")
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

    fn invoke_wobble(
        &self,
        request: InvocationRequest,
        descriptor: ToolDescriptor,
    ) -> Result<InvocationResponse, ServiceError> {
        if help_requested(request.arguments()) {
            return Ok(self.help_response(request, descriptor, wobble_help()));
        }

        let (params, plot_contract_out) = parse_wobble_params(request.arguments())?;
        let (input, input_provenance, input_diagnostics) =
            self.resolve_local_sequence_input(&params.input.path.display().to_string())?;
        let outcome = run_wobble(WobbleParams {
            input,
            codon_window: params.codon_window,
            codon_step: params.codon_step,
        })?;
        let status_message = format!(
            "computed bounded wobble variability profile across {} windows",
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
                    "codon_window_length".to_owned(),
                    "wobble_positions".to_owned(),
                    "adenine_fraction".to_owned(),
                    "cytosine_fraction".to_owned(),
                    "guanine_fraction".to_owned(),
                    "thymine_fraction".to_owned(),
                    "dominant_wobble_fraction".to_owned(),
                    "wobble_variability".to_owned(),
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
                            window.codon_window_length.to_string(),
                            window.wobble_positions.to_string(),
                            format!("{:.6}", window.adenine_fraction),
                            format!("{:.6}", window.cytosine_fraction),
                            format!("{:.6}", window.guanine_fraction),
                            format!("{:.6}", window.thymine_fraction),
                            format!("{:.6}", window.dominant_wobble_fraction),
                            format!("{:.6}", window.wobble_variability),
                        ]
                    })
                    .collect(),
            )),
            ResultSummary::new("Wobble variability profile computed")
                .with_line(format!("Input: {}", outcome.input.path.display()))
                .with_line(format!("Sequence: {}", outcome.profile.identifier))
                .with_line(format!("Codon window: {}", outcome.profile.codon_window))
                .with_line(format!("Codon step: {}", outcome.profile.codon_step))
                .with_line("X axis: 1-based window start")
                .with_line("Y axis: wobble variability")
                .with_line(
                    "Analytical table also reports third-position A/C/G/T composition and dominant wobble fraction",
                )
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
                    format!("failed to serialize wobble plot contract: {error}"),
                )
                .with_code("service.wobble.plot.serialize_failed")
            })?;
            std::fs::write(path, json).map_err(|error| {
                PlatformError::new(
                    ErrorCategory::Configuration,
                    format!("failed to write wobble plot contract to {}", path.display()),
                )
                .with_code("service.wobble.plot.write_failed")
                .with_detail(error.to_string())
            })?;

            let plot_provenance = ArtifactProvenance::generated_output(path.display().to_string())
                .with_description("wobble plot contract");
            provenance.push(plot_provenance.clone());
            result = result.with_artifact(
                ArtifactReference::new("wobble-plot-contract", ArtifactKind::Auxiliary)
                    .with_label("Wobble plot contract")
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

    fn invoke_isochore(
        &self,
        request: InvocationRequest,
        descriptor: ToolDescriptor,
    ) -> Result<InvocationResponse, ServiceError> {
        if help_requested(request.arguments()) {
            return Ok(self.help_response(request, descriptor, isochore_help()));
        }

        let (params, plot_contract_out) = parse_isochore_params(request.arguments())?;
        let (input, input_provenance, input_diagnostics) =
            self.resolve_local_sequence_input(&params.input.path.display().to_string())?;
        let outcome = run_isochore(IsochoreParams {
            input,
            window: params.window,
            step: params.step,
        })?;
        let status_message = format!(
            "computed bounded isochore profile across {} windows",
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
                    "canonical_symbols".to_owned(),
                    "ambiguous_symbols".to_owned(),
                    "ignored_gap_symbols".to_owned(),
                    "at_fraction".to_owned(),
                    "gc_fraction".to_owned(),
                    "gc_percent".to_owned(),
                    "isochore_band".to_owned(),
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
                            window.canonical_symbols.to_string(),
                            window.ambiguous_symbols.to_string(),
                            window.ignored_gap_symbols.to_string(),
                            format!("{:.6}", window.at_fraction),
                            format!("{:.6}", window.gc_fraction),
                            format!("{:.6}", window.gc_percent),
                            format!("{:?}", window.isochore_band),
                        ]
                    })
                    .collect(),
            )),
            ResultSummary::new("Isochore profile computed")
                .with_line(format!("Input: {}", outcome.input.path.display()))
                .with_line(format!("Sequence: {}", outcome.profile.identifier))
                .with_line(format!("Window: {}", outcome.profile.window))
                .with_line(format!("Step: {}", outcome.profile.step))
                .with_line("X axis: 1-based window start")
                .with_line("Y axis: GC percent")
                .with_line(
                    "Analytical table also reports canonical, ambiguous, gap, AT, GC, and bounded isochore-band columns",
                )
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
                    format!("failed to serialize isochore plot contract: {error}"),
                )
                .with_code("service.isochore.plot.serialize_failed")
            })?;
            std::fs::write(path, json).map_err(|error| {
                PlatformError::new(
                    ErrorCategory::Configuration,
                    format!(
                        "failed to write isochore plot contract to {}",
                        path.display()
                    ),
                )
                .with_code("service.isochore.plot.write_failed")
                .with_detail(error.to_string())
            })?;

            let plot_provenance = ArtifactProvenance::generated_output(path.display().to_string())
                .with_description("isochore plot contract");
            provenance.push(plot_provenance.clone());
            result = result.with_artifact(
                ArtifactReference::new("isochore-plot-contract", ArtifactKind::Auxiliary)
                    .with_label("Isochore plot contract")
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

    fn invoke_syco(
        &self,
        request: InvocationRequest,
        descriptor: ToolDescriptor,
    ) -> Result<InvocationResponse, ServiceError> {
        if help_requested(request.arguments()) {
            return Ok(self.help_response(request, descriptor, syco_help()));
        }

        let (params, plot_contract_out) = parse_syco_params(request.arguments())?;
        let (input, input_provenance, input_diagnostics) =
            self.resolve_local_sequence_input(&params.input.path.display().to_string())?;
        let outcome = run_syco(SycoParams {
            input,
            reference: params.reference,
            codon_window: params.codon_window,
            codon_step: params.codon_step,
        })?;
        let status_message = format!(
            "computed bounded syco preference profile across {} windows",
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
                    "codon_window_length".to_owned(),
                    "sense_codon_count".to_owned(),
                    "syco_score".to_owned(),
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
                            window.codon_window_length.to_string(),
                            window.sense_codon_count.to_string(),
                            format!("{:.6}", window.syco_score),
                        ]
                    })
                    .collect(),
            )),
            ResultSummary::new("Syco preference profile computed")
                .with_line(format!("Input: {}", outcome.input.path.display()))
                .with_line(format!("Reference: {}", outcome.reference.display()))
                .with_line(format!("Sequence: {}", outcome.profile.identifier))
                .with_line(format!("Codon window: {}", outcome.profile.codon_window))
                .with_line(format!("Codon step: {}", outcome.profile.codon_step))
                .with_line("X axis: 1-based window start")
                .with_line("Y axis: syco score")
                .with_line(
                    "Analytical table reports the bounded synonymous codon preference score derived from the same coding-window computation path",
                )
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
                    format!("failed to serialize syco plot contract: {error}"),
                )
                .with_code("service.syco.plot.serialize_failed")
            })?;
            std::fs::write(path, json).map_err(|error| {
                PlatformError::new(
                    ErrorCategory::Configuration,
                    format!("failed to write syco plot contract to {}", path.display()),
                )
                .with_code("service.syco.plot.write_failed")
                .with_detail(error.to_string())
            })?;

            let plot_provenance = ArtifactProvenance::generated_output(path.display().to_string())
                .with_description("syco plot contract");
            provenance.push(plot_provenance.clone());
            result = result.with_artifact(
                ArtifactReference::new("syco-plot-contract", ArtifactKind::Auxiliary)
                    .with_label("Syco plot contract")
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

    fn invoke_octanol(
        &self,
        request: InvocationRequest,
        descriptor: ToolDescriptor,
    ) -> Result<InvocationResponse, ServiceError> {
        if help_requested(request.arguments()) {
            return Ok(self.help_response(request, descriptor, octanol_help()));
        }

        let (params, plot_contract_out) = parse_octanol_params(request.arguments())?;
        let (input, input_provenance, input_diagnostics) =
            self.resolve_local_sequence_input(&params.input.path.display().to_string())?;
        let outcome = run_octanol(OctanolParams {
            input,
            window: params.window,
            step: params.step,
        })?;
        let status_message = format!(
            "computed White-Wimley interface-minus-octanol profile across {} windows",
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
                    "interface_minus_octanol".to_owned(),
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
                            format!("{:.6}", window.interface_minus_octanol),
                        ]
                    })
                    .collect(),
            )),
            ResultSummary::new("White-Wimley interface-minus-octanol profile computed")
                .with_line(format!("Input: {}", outcome.input.path.display()))
                .with_line(format!("Sequence: {}", outcome.profile.identifier))
                .with_line(format!("Window: {}", outcome.profile.window))
                .with_line(format!("Step: {}", outcome.profile.step))
                .with_line("X axis: 1-based window start")
                .with_line("Y axis: interface minus octanol")
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
                    format!("failed to serialize octanol plot contract: {error}"),
                )
                .with_code("service.octanol.plot.serialize_failed")
            })?;
            std::fs::write(path, json).map_err(|error| {
                PlatformError::new(
                    ErrorCategory::Configuration,
                    format!(
                        "failed to write octanol plot contract to {}",
                        path.display()
                    ),
                )
                .with_code("service.octanol.plot.write_failed")
                .with_detail(error.to_string())
            })?;

            let plot_provenance = ArtifactProvenance::generated_output(path.display().to_string())
                .with_description("octanol plot contract");
            provenance.push(plot_provenance.clone());
            result = result.with_artifact(
                ArtifactReference::new("octanol-plot-contract", ArtifactKind::Auxiliary)
                    .with_label("Octanol plot contract")
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

    fn invoke_pepinfo(
        &self,
        request: InvocationRequest,
        descriptor: ToolDescriptor,
    ) -> Result<InvocationResponse, ServiceError> {
        if help_requested(request.arguments()) {
            return Ok(self.help_response(request, descriptor, pepinfo_help()));
        }

        let (params, plot_contract_out) = parse_pepinfo_params(request.arguments())?;
        let (input, input_provenance, input_diagnostics) =
            self.resolve_local_sequence_input(&params.input.path.display().to_string())?;
        let outcome = run_pepinfo(PepinfoParams {
            input,
            window: params.window,
            step: params.step,
        })?;
        let status_message = format!(
            "computed bounded pepinfo profile across {} windows",
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
                    "mean_hydropathy".to_owned(),
                    "mean_residue_mass".to_owned(),
                    "charged_fraction".to_owned(),
                    "polar_fraction".to_owned(),
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
                            format!("{:.6}", window.mean_hydropathy),
                            format!("{:.6}", window.mean_residue_mass),
                            format!("{:.6}", window.charged_fraction),
                            format!("{:.6}", window.polar_fraction),
                        ]
                    })
                    .collect(),
            )),
            ResultSummary::new("Pepinfo protein profile computed")
                .with_line(format!("Input: {}", outcome.input.path.display()))
                .with_line(format!("Sequence: {}", outcome.profile.identifier))
                .with_line(format!("Window: {}", outcome.profile.window))
                .with_line(format!("Step: {}", outcome.profile.step))
                .with_line("X axis: 1-based window start")
                .with_line("Series: hydropathy, residue mass, charged fraction, polar fraction")
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
                    format!("failed to serialize pepinfo plot contract: {error}"),
                )
                .with_code("service.pepinfo.plot.serialize_failed")
            })?;
            std::fs::write(path, json).map_err(|error| {
                PlatformError::new(
                    ErrorCategory::Configuration,
                    format!(
                        "failed to write pepinfo plot contract to {}",
                        path.display()
                    ),
                )
                .with_code("service.pepinfo.plot.write_failed")
                .with_detail(error.to_string())
            })?;

            let plot_provenance = ArtifactProvenance::generated_output(path.display().to_string())
                .with_description("pepinfo plot contract");
            provenance.push(plot_provenance.clone());
            result = result.with_artifact(
                ArtifactReference::new("pepinfo-plot-contract", ArtifactKind::Auxiliary)
                    .with_label("Pepinfo plot contract")
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

    fn invoke_infobase(
        &self,
        request: InvocationRequest,
        descriptor: ToolDescriptor,
    ) -> Result<InvocationResponse, ServiceError> {
        if help_requested(request.arguments()) {
            return Ok(self.help_response(request, descriptor, infobase_help()));
        }

        let [symbol]: [String; 1] = request
            .arguments
            .clone()
            .try_into()
            .map_err(|_| tool_usage_error("infobase", infobase_help()))?;
        let symbol = parse_single_char_argument("infobase", &symbol, "base")?;
        let outcome = run_infobase(InfobaseParams { symbol })?;

        let report = self.success_report(
            &request.context,
            format!("reported nucleotide metadata for '{}'", outcome.info.symbol),
            Vec::new(),
            Vec::new(),
        );
        let result = MethodResult::new(
            request.tool.clone(),
            ResultPayload::TableReport(TableReport::new(
                vec![
                    "symbol".to_owned(),
                    "name".to_owned(),
                    "class".to_owned(),
                    "supported_molecules".to_owned(),
                    "canonical_expansion".to_owned(),
                    "dna_complement".to_owned(),
                    "rna_complement".to_owned(),
                ],
                vec![vec![
                    outcome.info.symbol.to_string(),
                    outcome.info.name.to_owned(),
                    outcome.info.base_class.to_owned(),
                    outcome.info.supported_molecules.to_owned(),
                    outcome.info.canonical_expansion.to_owned(),
                    outcome.info.dna_complement.to_owned(),
                    outcome.info.rna_complement.to_owned(),
                ]],
            )),
            ResultSummary::new("Nucleotide base metadata reported")
                .with_line(format!("Query: {}", outcome.info.symbol))
                .with_line(format!("Class: {}", outcome.info.base_class))
                .with_line(format!(
                    "Canonical expansion: {}",
                    outcome.info.canonical_expansion
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
            for codon in epithema_core::sense_codons() {
                let count = record.profile.count_for(codon);
                if count == 0 {
                    continue;
                }
                rows.push(vec![
                    "record".to_owned(),
                    record.record_id.clone(),
                    codon.to_owned(),
                    epithema_core::amino_acid_for_sense_codon(codon)
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
        for codon in epithema_core::sense_codons() {
            let count = outcome.aggregate.count_for(codon);
            if count == 0 {
                continue;
            }
            rows.push(vec![
                "aggregate".to_owned(),
                "ALL".to_owned(),
                codon.to_owned(),
                epithema_core::amino_acid_for_sense_codon(codon)
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

    fn invoke_cusp(
        &self,
        request: InvocationRequest,
        descriptor: ToolDescriptor,
    ) -> Result<InvocationResponse, ServiceError> {
        if help_requested(request.arguments()) {
            return Ok(self.help_response(request, descriptor, cusp_help()));
        }

        let [input]: [String; 1] = request
            .arguments
            .clone()
            .try_into()
            .map_err(|_| tool_usage_error("cusp", cusp_help()))?;
        let (input, input_provenance, input_diagnostics) =
            self.resolve_local_sequence_input(&input)?;
        let outcome = run_cusp(CuspParams { input })?;

        let mut rows = Vec::new();
        for record in &outcome.records {
            for codon in epithema_core::sense_codons() {
                rows.push(vec![
                    "record".to_owned(),
                    record.record_id.clone(),
                    codon.to_owned(),
                    epithema_core::amino_acid_for_sense_codon(codon)
                        .expect("sense codon should have amino acid")
                        .to_string(),
                    record.profile.count_for(codon).to_string(),
                    format!("{:.6}", record.profile.frequency_for(codon)),
                    record
                        .terminal_stop
                        .clone()
                        .unwrap_or_else(|| "-".to_owned()),
                ]);
            }
        }
        for codon in epithema_core::sense_codons() {
            rows.push(vec![
                "aggregate".to_owned(),
                "ALL".to_owned(),
                codon.to_owned(),
                epithema_core::amino_acid_for_sense_codon(codon)
                    .expect("sense codon should have amino acid")
                    .to_string(),
                outcome.aggregate.count_for(codon).to_string(),
                format!("{:.6}", outcome.aggregate.frequency_for(codon)),
                "-".to_owned(),
            ]);
        }

        let report = self.success_report(
            &request.context,
            format!(
                "reported codon usage tables for {} records",
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
                    "codon".to_owned(),
                    "amino_acid".to_owned(),
                    "count".to_owned(),
                    "frequency".to_owned(),
                    "terminal_stop".to_owned(),
                ],
                rows,
            )),
            ResultSummary::new("Codon usage table reported")
                .with_line(format!("Input: {}", outcome.input.path.display()))
                .with_line("Scope: complete 61-sense-codon rows per record plus aggregate")
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

    fn invoke_dan(
        &self,
        request: InvocationRequest,
        descriptor: ToolDescriptor,
    ) -> Result<InvocationResponse, ServiceError> {
        if help_requested(request.arguments()) {
            return Ok(self.help_response(request, descriptor, dan_help()));
        }

        let params = parse_dan_params(request.arguments())?;
        let input_path = params.input.path.display().to_string();
        let (input, input_provenance, input_diagnostics) =
            self.resolve_local_sequence_input(&input_path)?;
        let outcome = run_dan(DanParams {
            input,
            window: params.window,
            step: params.step,
        })?;

        let rows = outcome
            .windows
            .iter()
            .map(|window| {
                vec![
                    window.record_id.clone(),
                    window.window_index.to_string(),
                    (window.start + 1).to_string(),
                    window.end.to_string(),
                    (window.end - window.start).to_string(),
                    format!("{:.2}", window.gc.gc_percent()),
                    format!("{:.2}", window.tm_celsius),
                ]
            })
            .collect();

        let report = self.success_report(
            &request.context,
            format!(
                "reported melting estimates for {} windows",
                outcome.windows.len()
            ),
            input_diagnostics,
            vec![input_provenance],
        );
        let result = MethodResult::new(
            request.tool.clone(),
            ResultPayload::TableReport(TableReport::new(
                vec![
                    "record".to_owned(),
                    "window_index".to_owned(),
                    "start".to_owned(),
                    "end".to_owned(),
                    "length".to_owned(),
                    "gc_percent".to_owned(),
                    "tm_celsius".to_owned(),
                ],
                rows,
            )),
            ResultSummary::new("Melting estimates reported")
                .with_line(format!("Input: {}", outcome.input.path.display()))
                .with_line("Model: conservative Wallace/GC-length hybrid estimate")
                .with_line("Residue policy: canonical A/C/G/T/U only")
                .with_line(format!(
                    "Windowing: {}",
                    outcome
                        .window
                        .map(|window| format!("window={window} step={}", outcome.step))
                        .unwrap_or_else(|| "whole-record summaries".to_owned())
                ))
                .with_line(format!("Rows: {}", outcome.windows.len())),
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

    fn invoke_inforesidue(
        &self,
        request: InvocationRequest,
        descriptor: ToolDescriptor,
    ) -> Result<InvocationResponse, ServiceError> {
        if help_requested(request.arguments()) {
            return Ok(self.help_response(request, descriptor, inforesidue_help()));
        }

        let [residue]: [String; 1] = request
            .arguments
            .clone()
            .try_into()
            .map_err(|_| tool_usage_error("inforesidue", inforesidue_help()))?;
        let residue = parse_single_char_argument("inforesidue", &residue, "residue")?;
        let outcome = run_inforesidue(InforesidueParams { residue })?;

        let report = self.success_report(
            &request.context,
            format!(
                "reported amino-acid metadata for '{}'",
                outcome.property.residue
            ),
            Vec::new(),
            Vec::new(),
        );
        let result = MethodResult::new(
            request.tool.clone(),
            ResultPayload::TableReport(TableReport::new(
                vec![
                    "residue".to_owned(),
                    "three_letter".to_owned(),
                    "name".to_owned(),
                    "charge_class".to_owned(),
                    "polarity_class".to_owned(),
                    "average_mass".to_owned(),
                    "hydropathy".to_owned(),
                ],
                vec![vec![
                    outcome.property.residue.to_string(),
                    outcome.property.three_letter.to_owned(),
                    outcome.property.name.to_owned(),
                    outcome.property.charge_class.to_owned(),
                    outcome.property.polarity_class.to_owned(),
                    format!("{:.4}", outcome.property.average_mass),
                    format!("{:.3}", outcome.property.hydropathy),
                ]],
            )),
            ResultSummary::new("Amino-acid residue metadata reported")
                .with_line(format!("Query: {}", outcome.property.residue))
                .with_line(format!("Charge class: {}", outcome.property.charge_class))
                .with_line(format!(
                    "Hydropathy (Kyte-Doolittle): {:.3}",
                    outcome.property.hydropathy
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

    fn invoke_recoder(
        &self,
        request: InvocationRequest,
        descriptor: ToolDescriptor,
    ) -> Result<InvocationResponse, ServiceError> {
        if help_requested(request.arguments()) {
            return Ok(self.help_response(request, descriptor, recoder_help()));
        }

        let params = parse_recoder_params(request.arguments())?;
        let input_path = params.input.path.display().to_string();
        let (input, input_provenance, input_diagnostics) =
            self.resolve_local_sequence_input(&input_path)?;
        let outcome = run_recoder(RecoderParams {
            input,
            site: params.site,
        })?;

        let rows = outcome
            .candidates
            .iter()
            .map(|candidate| {
                vec![
                    candidate.record_id.clone(),
                    outcome.site.clone(),
                    candidate.occurrence_index.to_string(),
                    candidate.edit.site_start.to_string(),
                    candidate.edit.site_end.to_string(),
                    candidate.edit.codon_index.to_string(),
                    candidate.edit.codon_start.to_string(),
                    candidate.edit.codon_end.to_string(),
                    candidate.edit.amino_acid.to_string(),
                    candidate.edit.original_codon.clone(),
                    candidate.edit.replacement_codon.clone(),
                    candidate.edit.mutated_sequence.clone(),
                ]
            })
            .collect();

        let report = self.success_report(
            &request.context,
            format!(
                "reported {} synonymous restriction-site removal candidates",
                outcome.candidates.len()
            ),
            input_diagnostics,
            vec![input_provenance],
        );
        let result = MethodResult::new(
            request.tool.clone(),
            ResultPayload::TableReport(TableReport::new(
                vec![
                    "record".to_owned(),
                    "site".to_owned(),
                    "occurrence".to_owned(),
                    "site_start".to_owned(),
                    "site_end".to_owned(),
                    "codon_index".to_owned(),
                    "codon_start".to_owned(),
                    "codon_end".to_owned(),
                    "amino_acid".to_owned(),
                    "original_codon".to_owned(),
                    "replacement_codon".to_owned(),
                    "mutated_sequence".to_owned(),
                ],
                rows,
            )),
            ResultSummary::new("Restriction-site removal candidates reported")
                .with_line(format!("Input: {}", outcome.input.path.display()))
                .with_line(format!("Site: {}", outcome.site))
                .with_line("Model: synonymous single-codon disruption of exact forward DNA sites")
                .with_line(
                    "Coding validation: strict canonical DNA CDS with optional terminal stop",
                )
                .with_line(format!("Candidates: {}", outcome.candidates.len())),
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

    fn invoke_silent(
        &self,
        request: InvocationRequest,
        descriptor: ToolDescriptor,
    ) -> Result<InvocationResponse, ServiceError> {
        if help_requested(request.arguments()) {
            return Ok(self.help_response(request, descriptor, silent_help()));
        }

        let params = parse_silent_params(request.arguments())?;
        let input_path = params.input.path.display().to_string();
        let (input, input_provenance, input_diagnostics) =
            self.resolve_local_sequence_input(&input_path)?;
        let outcome = run_silent(SilentParams {
            input,
            site: params.site,
        })?;

        let rows = outcome
            .candidates
            .iter()
            .map(|candidate| {
                vec![
                    candidate.record_id.clone(),
                    outcome.site.clone(),
                    candidate.edit.site_start.to_string(),
                    candidate.edit.site_end.to_string(),
                    candidate.edit.codon_index.to_string(),
                    candidate.edit.codon_start.to_string(),
                    candidate.edit.codon_end.to_string(),
                    candidate.edit.amino_acid.to_string(),
                    candidate.edit.original_codon.clone(),
                    candidate.edit.replacement_codon.clone(),
                    candidate.edit.mutated_sequence.clone(),
                ]
            })
            .collect();

        let report = self.success_report(
            &request.context,
            format!(
                "reported {} synonymous restriction-site creation candidates",
                outcome.candidates.len()
            ),
            input_diagnostics,
            vec![input_provenance],
        );
        let result = MethodResult::new(
            request.tool.clone(),
            ResultPayload::TableReport(TableReport::new(
                vec![
                    "record".to_owned(),
                    "site".to_owned(),
                    "site_start".to_owned(),
                    "site_end".to_owned(),
                    "codon_index".to_owned(),
                    "codon_start".to_owned(),
                    "codon_end".to_owned(),
                    "amino_acid".to_owned(),
                    "original_codon".to_owned(),
                    "replacement_codon".to_owned(),
                    "mutated_sequence".to_owned(),
                ],
                rows,
            )),
            ResultSummary::new("Restriction-site creation candidates reported")
                .with_line(format!("Input: {}", outcome.input.path.display()))
                .with_line(format!("Site: {}", outcome.site))
                .with_line("Model: synonymous single-codon creation of exact forward DNA sites")
                .with_line(
                    "Coding validation: strict canonical DNA CDS with optional terminal stop",
                )
                .with_line(format!("Candidates: {}", outcome.candidates.len())),
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

    fn invoke_iep(
        &self,
        request: InvocationRequest,
        descriptor: ToolDescriptor,
    ) -> Result<InvocationResponse, ServiceError> {
        if help_requested(request.arguments()) {
            return Ok(self.help_response(request, descriptor, iep_help()));
        }

        let [input]: [String; 1] = request
            .arguments
            .clone()
            .try_into()
            .map_err(|_| tool_usage_error("iep", iep_help()))?;
        let (input, input_provenance, input_diagnostics) =
            self.resolve_local_sequence_input(&input)?;
        let outcome = run_iep(IepParams { input })?;

        let rows = outcome
            .records
            .iter()
            .map(|record| {
                vec![
                    record.record_id.clone(),
                    record.residue_length.to_string(),
                    record.titratable_counts.total_side_chains().to_string(),
                    record.titratable_counts.aspartate.to_string(),
                    record.titratable_counts.glutamate.to_string(),
                    record.titratable_counts.cysteine.to_string(),
                    record.titratable_counts.tyrosine.to_string(),
                    record.titratable_counts.histidine.to_string(),
                    record.titratable_counts.lysine.to_string(),
                    record.titratable_counts.arginine.to_string(),
                    format!("{:.6}", record.net_charge_at_ph7),
                    format!("{:.6}", record.estimated_pi),
                ]
            })
            .collect();

        let report = self.success_report(
            &request.context,
            format!(
                "reported isoelectric-point estimates for {} records",
                outcome.records.len()
            ),
            input_diagnostics,
            vec![input_provenance],
        );
        let result = MethodResult::new(
            request.tool.clone(),
            ResultPayload::TableReport(TableReport::new(
                vec![
                    "record".to_owned(),
                    "residue_length".to_owned(),
                    "titratable_side_chains".to_owned(),
                    "aspartate".to_owned(),
                    "glutamate".to_owned(),
                    "cysteine".to_owned(),
                    "tyrosine".to_owned(),
                    "histidine".to_owned(),
                    "lysine".to_owned(),
                    "arginine".to_owned(),
                    "net_charge_ph7".to_owned(),
                    "estimated_pi".to_owned(),
                ],
                rows,
            )),
            ResultSummary::new("Protein isoelectric-point estimates reported")
                .with_line(format!("Input: {}", outcome.input.path.display()))
                .with_line("Model: fixed explicit pKa set for termini and D/E/C/Y/H/K/R")
                .with_line("Net charge reported at pH 7.0")
                .with_line("Stop symbols are ignored before estimation")
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

    fn invoke_pepdigest(
        &self,
        request: InvocationRequest,
        descriptor: ToolDescriptor,
    ) -> Result<InvocationResponse, ServiceError> {
        if help_requested(request.arguments()) {
            return Ok(self.help_response(request, descriptor, pepdigest_help()));
        }

        let params = parse_pepdigest_params(request.arguments())?;
        let input_path = params.input.path.display().to_string();
        let (input, input_provenance, input_diagnostics) =
            self.resolve_local_sequence_input(&input_path)?;
        let outcome = run_pepdigest(PepdigestParams {
            input,
            protease: params.protease,
        })?;

        let rows = outcome
            .peptides
            .iter()
            .map(|peptide| {
                vec![
                    peptide.record_id.clone(),
                    peptide.protease.label().to_owned(),
                    peptide.peptide_index.to_string(),
                    peptide.start.to_string(),
                    peptide.end.to_string(),
                    peptide
                        .cleavage_after
                        .map(|value| value.to_string())
                        .unwrap_or_else(|| "-".to_owned()),
                    peptide.sequence.clone(),
                ]
            })
            .collect();

        let report = self.success_report(
            &request.context,
            format!(
                "reported {} peptide fragments across digested records",
                outcome.peptides.len()
            ),
            input_diagnostics,
            vec![input_provenance],
        );
        let result = MethodResult::new(
            request.tool.clone(),
            ResultPayload::TableReport(TableReport::new(
                vec![
                    "record".to_owned(),
                    "protease".to_owned(),
                    "peptide_index".to_owned(),
                    "start".to_owned(),
                    "end".to_owned(),
                    "cleavage_after".to_owned(),
                    "sequence".to_owned(),
                ],
                rows,
            )),
            ResultSummary::new("Protein digest fragments reported")
                .with_line(format!("Input: {}", outcome.input.path.display()))
                .with_line(format!("Protease: {}", outcome.protease.label()))
                .with_line("Digest mode: full deterministic cleavage")
                .with_line("Trypsin blocks cleavage before proline")
                .with_line(format!("Peptides: {}", outcome.peptides.len())),
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

    fn invoke_wordcount(
        &self,
        request: InvocationRequest,
        descriptor: ToolDescriptor,
    ) -> Result<InvocationResponse, ServiceError> {
        if help_requested(request.arguments()) {
            return Ok(self.help_response(request, descriptor, wordcount_help()));
        }

        let (params, plot_contract_out) = parse_wordcount_params(request.arguments())?;
        let input_path = params.input.path.display().to_string();
        let (input, input_provenance, input_diagnostics) =
            self.resolve_local_sequence_input(&input_path)?;
        let outcome = run_wordcount(WordcountParams {
            input,
            word_size: params.word_size,
            min_count: params.min_count,
        })?;

        let mut rows = Vec::new();
        for record in &outcome.records {
            for (word, count) in &record.counts {
                if *count < outcome.min_count {
                    continue;
                }
                rows.push(vec![
                    "record".to_owned(),
                    record.record_id.clone(),
                    record.molecule.as_str().to_owned(),
                    outcome.word_size.to_string(),
                    word.clone(),
                    count.to_string(),
                    format!("{:.6}", word_frequency(record, word)),
                    record.skipped_gap_windows.to_string(),
                ]);
            }
        }
        for (word, count) in &outcome.aggregate.counts {
            if *count < outcome.min_count {
                continue;
            }
            rows.push(vec![
                "aggregate".to_owned(),
                "ALL".to_owned(),
                "mixed".to_owned(),
                outcome.word_size.to_string(),
                word.clone(),
                count.to_string(),
                format!("{:.6}", word_frequency(&outcome.aggregate, word)),
                outcome.aggregate.skipped_gap_windows.to_string(),
            ]);
        }

        let status_message = format!(
            "reported sequence words for {} records",
            outcome.records.len()
        );
        let mut provenance = vec![input_provenance];
        let mut report = self.success_report(
            &request.context,
            status_message.clone(),
            input_diagnostics.clone(),
            Vec::new(),
        );
        let mut result = MethodResult::new(
            request.tool.clone(),
            ResultPayload::TableReport(TableReport::new(
                vec![
                    "scope".to_owned(),
                    "record".to_owned(),
                    "molecule".to_owned(),
                    "word_size".to_owned(),
                    "word".to_owned(),
                    "count".to_owned(),
                    "frequency".to_owned(),
                    "skipped_gap_windows".to_owned(),
                ],
                rows,
            )),
            ResultSummary::new("Sequence word counts reported")
                .with_line(format!("Input: {}", outcome.input.path.display()))
                .with_line("Word model: overlapping normalized windows")
                .with_line("Gap policy: windows containing '-' are skipped")
                .with_line(format!("Word size: {}", outcome.word_size))
                .with_line(format!("Minimum reported count: {}", outcome.min_count))
                .with_line(format!("Records: {}", outcome.records.len()))
                .with_line(format!(
                    "Plot contract: {}",
                    plot_contract_out
                        .as_ref()
                        .map(|path| path.display().to_string())
                        .unwrap_or_else(|| "attached in method result only".to_owned())
                )),
            report.clone(),
        );

        if let Some(plot) = outcome.plot.clone() {
            result = result.with_plot(plot.clone());
            if let Some(path) = &plot_contract_out {
                let json = plot.to_json_pretty().map_err(|error| {
                    PlatformError::new(
                        ErrorCategory::Validation,
                        format!("failed to serialize wordcount plot contract: {error}"),
                    )
                    .with_code("service.wordcount.plot.serialize_failed")
                })?;
                std::fs::write(path, json).map_err(|error| {
                    PlatformError::new(
                        ErrorCategory::Configuration,
                        format!(
                            "failed to write wordcount plot contract to {}",
                            path.display()
                        ),
                    )
                    .with_code("service.wordcount.plot.write_failed")
                    .with_detail(error.to_string())
                })?;
                let plot_provenance =
                    ArtifactProvenance::generated_output(path.display().to_string())
                        .with_description("wordcount plot contract");
                provenance.push(plot_provenance.clone());
                result = result.with_artifact(
                    ArtifactReference::new("wordcount-plot-contract", ArtifactKind::Auxiliary)
                        .with_label("Wordcount plot contract")
                        .with_local_path(path)
                        .with_provenance(plot_provenance),
                );
            }
        }

        report = self.success_report(
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

    fn invoke_oddcomp(
        &self,
        request: InvocationRequest,
        descriptor: ToolDescriptor,
    ) -> Result<InvocationResponse, ServiceError> {
        if help_requested(request.arguments()) {
            return Ok(self.help_response(request, descriptor, oddcomp_help()));
        }

        let params = parse_oddcomp_params(request.arguments())?;
        let input_path = params.input.path.display().to_string();
        let (input, input_provenance, input_diagnostics) =
            self.resolve_local_sequence_input(&input_path)?;
        let outcome = run_oddcomp(OddcompParams {
            input,
            query_words: params.query_words,
        })?;

        let mut rows = Vec::new();
        for row in &outcome.rows {
            rows.push(vec![
                row.record_id.clone(),
                row.query_word.clone(),
                row.word_length.to_string(),
                row.count.to_string(),
                if row.counted_windows == 0 {
                    "0.000000".to_owned()
                } else {
                    format!("{:.6}", row.count as f64 / row.counted_windows as f64)
                },
                row.contains.to_string(),
                row.counted_windows.to_string(),
            ]);
        }

        let report = self.success_report(
            &request.context,
            format!(
                "reported exact protein word composition for {} records",
                outcome
                    .rows
                    .iter()
                    .map(|row| &row.record_id)
                    .collect::<std::collections::BTreeSet<_>>()
                    .len()
            ),
            input_diagnostics,
            vec![input_provenance],
        );
        let result = MethodResult::new(
            request.tool.clone(),
            ResultPayload::TableReport(TableReport::new(
                vec![
                    "record".to_owned(),
                    "query_word".to_owned(),
                    "word_length".to_owned(),
                    "count".to_owned(),
                    "frequency".to_owned(),
                    "contains".to_owned(),
                    "counted_windows".to_owned(),
                ],
                rows,
            )),
            ResultSummary::new("Protein word composition reported")
                .with_line(format!("Input: {}", outcome.input.path.display()))
                .with_line(format!("Query words: {}", outcome.query_words.join(", ")))
                .with_line("Counting model: overlapping exact literal protein words")
                .with_line(format!("Rows: {}", outcome.rows.len())),
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

    fn invoke_pasteseq(
        &self,
        request: InvocationRequest,
        descriptor: ToolDescriptor,
    ) -> Result<InvocationResponse, ServiceError> {
        if help_requested(request.arguments()) {
            return Ok(self.help_response(request, descriptor, pasteseq_help()));
        }

        let params = parse_pasteseq_params(request.arguments())?;
        let main_path = params.asequence.path.display().to_string();
        let insert_path = params.bsequence.path.display().to_string();
        let (asequence, a_provenance, mut diagnostics) =
            self.resolve_local_sequence_input(&main_path)?;
        let (bsequence, b_provenance, b_diagnostics) =
            self.resolve_local_sequence_input(&insert_path)?;
        diagnostics.extend(b_diagnostics);
        let outcome = run_pasteseq(PasteseqParams {
            asequence,
            bsequence,
            position: params.position,
        })?;

        let output_provenance =
            ArtifactProvenance::generated_output("stdout").with_description("pasted FASTA output");
        let report = self.success_report(
            &request.context,
            format!(
                "inserted one sequence into '{}' at position {}",
                outcome.record.identifier().accession(),
                outcome.position
            ),
            diagnostics,
            vec![a_provenance, b_provenance, output_provenance.clone()],
        );
        let result = MethodResult::new(
            request.tool.clone(),
            ResultPayload::Sequence(outcome.record),
            ResultSummary::new("Sequence insertion completed")
                .with_line(format!("Main input: {}", outcome.asequence.path.display()))
                .with_line(format!(
                    "Inserted input: {}",
                    outcome.bsequence.path.display()
                ))
                .with_line(format!("Insert after position: {}", outcome.position))
                .with_line("Coordinate convention: 1-based, with 0 meaning before the start")
                .with_line("Output format: fasta (feature annotations dropped in v1)"),
            report.clone(),
        )
        .with_artifact(
            ArtifactReference::new("pasted-sequence", ArtifactKind::Sequence)
                .with_label("Pasted sequence")
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

    fn invoke_merger(
        &self,
        request: InvocationRequest,
        descriptor: ToolDescriptor,
    ) -> Result<InvocationResponse, ServiceError> {
        if help_requested(request.arguments()) {
            return Ok(self.help_response(request, descriptor, merger_help()));
        }

        let arguments: [String; 2] = request
            .arguments
            .clone()
            .try_into()
            .map_err(|_| tool_usage_error("merger", merger_help()))?;
        let (left, left_provenance, mut diagnostics) =
            self.resolve_local_sequence_input(&arguments[0])?;
        let (right, right_provenance, right_diagnostics) =
            self.resolve_local_sequence_input(&arguments[1])?;
        diagnostics.extend(right_diagnostics);
        let outcome = run_merger(MergerParams { left, right })?;

        let output_provenance =
            ArtifactProvenance::generated_output("stdout").with_description("merged FASTA output");
        let report = self.success_report(
            &request.context,
            "merged two overlapping sequence inputs",
            diagnostics,
            vec![left_provenance, right_provenance, output_provenance.clone()],
        );
        let result = MethodResult::new(
            request.tool.clone(),
            ResultPayload::Sequence(outcome.record.clone()),
            ResultSummary::new("Sequence merge completed")
                .with_line(format!("Left input: {}", outcome.left.path.display()))
                .with_line(format!("Right input: {}", outcome.right.path.display()))
                .with_line(format!("Overlap length: {}", outcome.overlap_length))
                .with_line("Merge rule: longest positive exact suffix/prefix overlap")
                .with_line("Output format: fasta"),
            report.clone(),
        )
        .with_artifact(
            ArtifactReference::new("merged-sequence", ArtifactKind::Sequence)
                .with_label("Merged sequence record")
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

    fn invoke_megamerger(
        &self,
        request: InvocationRequest,
        descriptor: ToolDescriptor,
    ) -> Result<InvocationResponse, ServiceError> {
        if help_requested(request.arguments()) {
            return Ok(self.help_response(request, descriptor, megamerger_help()));
        }

        let arguments: [String; 2] = request
            .arguments
            .clone()
            .try_into()
            .map_err(|_| tool_usage_error("megamerger", megamerger_help()))?;
        let (left, left_provenance, mut diagnostics) =
            self.resolve_local_sequence_input(&arguments[0])?;
        let (right, right_provenance, right_diagnostics) =
            self.resolve_local_sequence_input(&arguments[1])?;
        diagnostics.extend(right_diagnostics);
        let outcome = run_megamerger(MegamergerParams { left, right })?;

        let output_provenance = ArtifactProvenance::generated_output("stdout")
            .with_description("merged DNA FASTA output");
        let report = self.success_report(
            &request.context,
            "merged two overlapping DNA inputs",
            diagnostics,
            vec![left_provenance, right_provenance, output_provenance.clone()],
        );
        let result = MethodResult::new(
            request.tool.clone(),
            ResultPayload::Sequence(outcome.record.clone()),
            ResultSummary::new("DNA sequence merge completed")
                .with_line(format!("Left input: {}", outcome.left.path.display()))
                .with_line(format!("Right input: {}", outcome.right.path.display()))
                .with_line(format!("Overlap length: {}", outcome.overlap_length))
                .with_line("Merge rule: longest positive exact suffix/prefix overlap")
                .with_line("Molecule policy: DNA only")
                .with_line("Output format: fasta"),
            report.clone(),
        )
        .with_artifact(
            ArtifactReference::new("merged-dna-sequence", ArtifactKind::Sequence)
                .with_label("Merged DNA sequence record")
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

    fn invoke_sizeseq(
        &self,
        request: InvocationRequest,
        descriptor: ToolDescriptor,
    ) -> Result<InvocationResponse, ServiceError> {
        if help_requested(request.arguments()) {
            return Ok(self.help_response(request, descriptor, sizeseq_help()));
        }

        let arguments: [String; 1] = request
            .arguments
            .clone()
            .try_into()
            .map_err(|_| tool_usage_error("sizeseq", sizeseq_help()))?;
        let (input, input_provenance, diagnostics) =
            self.resolve_local_sequence_input(&arguments[0])?;
        let outcome = run_sizeseq(SizeseqParams { input })?;

        let output_provenance = ArtifactProvenance::generated_output("stdout")
            .with_description("size-sorted FASTA output");
        let report = self.success_report(
            &request.context,
            format!("sorted {} records by size", outcome.records.len()),
            diagnostics,
            vec![input_provenance, output_provenance.clone()],
        );
        let result = MethodResult::new(
            request.tool.clone(),
            ResultPayload::SequenceCollection(outcome.records),
            ResultSummary::new("Sequence size sort completed")
                .with_line(format!("Input: {}", outcome.input.path.display()))
                .with_line("Ordering: descending length, stable ties")
                .with_line("Output format: fasta"),
            report.clone(),
        )
        .with_artifact(
            ArtifactReference::new("size-sorted-sequences", ArtifactKind::Sequence)
                .with_label("Size-sorted sequence stream")
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

    fn invoke_shuffleseq(
        &self,
        request: InvocationRequest,
        descriptor: ToolDescriptor,
    ) -> Result<InvocationResponse, ServiceError> {
        if help_requested(request.arguments()) {
            return Ok(self.help_response(request, descriptor, shuffleseq_help()));
        }

        let (input_path, seed) = parse_shuffleseq_params(&request.arguments)?;
        let (input, input_provenance, diagnostics) =
            self.resolve_local_sequence_input(&input_path)?;
        let outcome = run_shuffleseq(ShuffleseqParams { input, seed })?;

        let output_provenance = ArtifactProvenance::generated_output("stdout")
            .with_description("composition-preserving shuffled FASTA output");
        let report = self.success_report(
            &request.context,
            format!("shuffled {} sequence records", outcome.records.len()),
            diagnostics,
            vec![input_provenance, output_provenance.clone()],
        );
        let result = MethodResult::new(
            request.tool.clone(),
            ResultPayload::SequenceCollection(outcome.records),
            ResultSummary::new("Deterministic sequence shuffle completed")
                .with_line(format!("Input: {}", outcome.input.path.display()))
                .with_line(format!("Seed: {}", outcome.seed))
                .with_line("Shuffle rule: deterministic per-record residue permutation")
                .with_line("Composition: preserved exactly")
                .with_line("Output format: fasta"),
            report.clone(),
        )
        .with_artifact(
            ArtifactReference::new("shuffled-sequences", ArtifactKind::Sequence)
                .with_label("Shuffled sequence stream")
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
            Vec<epithema_core::SequenceRecord>,
            Vec<ArtifactProvenance>,
            Vec<Diagnostic>,
        ),
        ServiceError,
    > {
        let reference = self.classify_input(raw.to_owned())?;
        match self.resolve_input(
            reference,
            epithema_providers::ResolutionIntent::SequenceInput,
        )? {
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

    fn resolve_seqretsetall_inputs_with_client<C: ProviderHttpClient>(
        &self,
        raw_inputs: &[String],
        client: Option<&C>,
    ) -> Result<
        (
            epithema_tools::retrieval_tools::SeqretsetallOutcome,
            Vec<ArtifactProvenance>,
            Vec<Diagnostic>,
        ),
        ServiceError,
    > {
        let mut input_sets = Vec::with_capacity(raw_inputs.len());
        let mut provenance = Vec::new();
        let mut diagnostics = Vec::new();

        for raw in raw_inputs {
            let (source, records, set_provenance, set_diagnostics) =
                self.resolve_seqret_records_with_client(raw, client)?;
            input_sets.push(SeqretsetallInputSet { source, records });
            provenance.extend(set_provenance);
            diagnostics.extend(set_diagnostics);
        }

        let outcome = run_seqretsetall(SeqretsetallParams { inputs: input_sets })?;
        Ok((outcome, provenance, diagnostics))
    }

    fn resolve_seqretsplit_input_with_client<C: ProviderHttpClient>(
        &self,
        raw_input: &str,
        client: Option<&C>,
    ) -> Result<
        (
            epithema_tools::retrieval_tools::SeqretsplitOutcome,
            Vec<ArtifactProvenance>,
            Vec<Diagnostic>,
        ),
        ServiceError,
    > {
        let (source, records, provenance, diagnostics) =
            self.resolve_seqret_records_with_client(raw_input, client)?;
        let outcome = run_seqretsplit(SeqretsplitParams { source, records })?;
        Ok((outcome, provenance, diagnostics))
    }

    fn resolve_refseqget_record_with_client<C: ProviderHttpClient>(
        &self,
        raw: &str,
        client: Option<&C>,
    ) -> Result<(ArtifactProvenance, RetrievedSequence, Vec<Diagnostic>), ServiceError> {
        let reference = self.classify_input(raw.to_owned())?;
        match self.resolve_input(
            reference,
            epithema_providers::ResolutionIntent::SequenceInput,
        )? {
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
        match self.resolve_input(
            reference,
            epithema_providers::ResolutionIntent::ArchiveAsset,
        )? {
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
        match self.resolve_input(
            reference,
            epithema_providers::ResolutionIntent::ArchiveAsset,
        )? {
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

    fn resolve_infoassembly_with_client<C: ProviderHttpClient>(
        &self,
        raw: &str,
        client: Option<&C>,
    ) -> Result<
        (
            epithema_tools::archive_tools::InfoassemblyOutcome,
            Vec<ArtifactProvenance>,
            Vec<Diagnostic>,
        ),
        ServiceError,
    > {
        let (route_provenance, metadata, diagnostics) =
            self.resolve_archive_metadata_with_client(raw, client)?;
        let outcome = run_infoassembly(InfoassemblyParams {
            provider: metadata.provider.as_str().to_owned(),
            accession: metadata.requested_accession.clone(),
            object_class: metadata.object_class.as_str().to_owned(),
            assembly_accession: metadata.study_accession.clone(),
            run_accession: metadata.run_accession.clone(),
            experiment_accession: metadata.experiment_accession.clone(),
            sample_accession: metadata.sample_accession.clone(),
            platform: metadata.platform.clone(),
            instrument_model: metadata.instrument_model.clone(),
            library_layout: metadata.library_layout.clone(),
            library_strategy: metadata.library_strategy.clone(),
            library_source: metadata.library_source.clone(),
            file_count: metadata.files.len(),
            total_size_bytes: metadata.total_size_bytes(),
            route_endpoint: metadata.route.endpoint.clone(),
        })?;

        Ok((
            outcome,
            vec![route_provenance, metadata.provenance.clone()],
            diagnostics,
        ))
    }

    fn resolve_assemblyget_with_client<C: ProviderHttpClient>(
        &self,
        raw: &str,
        client: Option<&C>,
    ) -> Result<
        (
            epithema_tools::archive_tools::AssemblygetOutcome,
            Vec<ArtifactProvenance>,
            Vec<Diagnostic>,
        ),
        ServiceError,
    > {
        let reference = self.classify_input(raw.to_owned())?;
        let (request, route_provenance, diagnostics) = match self.resolve_input(
            reference,
            epithema_providers::ResolutionIntent::ArchiveAsset,
        )? {
            ToolInputResolution::ProviderRouted {
                request,
                provenance,
                diagnostics,
                ..
            } => (request, provenance, diagnostics),
            ToolInputResolution::LocalFile { provenance, .. } => {
                return Err(PlatformError::new(
                    ErrorCategory::Validation,
                    "assemblyget expects provider-qualified archive accessions, not a local file",
                )
                .with_code("service.assemblyget.local_input_not_supported")
                .with_detail(provenance.locator().to_owned()));
            }
            ToolInputResolution::InlineSequence { .. } => {
                return Err(PlatformError::new(
                    ErrorCategory::Validation,
                    "assemblyget does not accept inline sequence literals",
                )
                .with_code("service.assemblyget.inline_not_supported"));
            }
            ToolInputResolution::Unresolved {
                reference,
                diagnostics,
            } => {
                return Err(PlatformError::new(
                    ErrorCategory::Validation,
                    format!("could not resolve assemblyget input '{}'", reference.raw()),
                )
                .with_code("service.assemblyget.input.unresolved")
                .with_detail(
                    diagnostics
                        .iter()
                        .map(|diagnostic| diagnostic.message().to_owned())
                        .collect::<Vec<_>>()
                        .join("; "),
                ));
            }
        };

        let metadata = self.retrieve_archive_metadata_request_with_client(&request, client)?;
        let outcome = run_assemblyget(AssemblygetParams {
            query: format!(
                "{}:{}",
                metadata.provider.as_str(),
                metadata.requested_accession
            ),
            object_class: metadata.object_class.as_str().to_owned(),
            assembly_accession: metadata
                .study_accession
                .clone()
                .unwrap_or_else(|| "-".to_owned()),
            run_accession: metadata.run_accession.clone(),
            route_endpoint: metadata.route.endpoint.clone(),
            manifest_mode: "manifest_intent_only".to_owned(),
            file_count: metadata.files.len(),
            total_size_bytes: metadata.total_size_bytes(),
        })?;

        Ok((
            outcome,
            vec![route_provenance, metadata.provenance.clone()],
            diagnostics,
        ))
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

    fn retrieve_ngs_manifest_with_client<C: ProviderHttpClient>(
        &self,
        query: &NgsQuery,
        client: Option<&C>,
    ) -> Result<NgsManifest, ServiceError> {
        match client {
            Some(client) => ServiceNgsRetrieval::with_client(&self.config, &self.providers, client)
                .list_manifest(query),
            None => self.ngs_retrieval()?.list_manifest(query),
        }
    }

    fn resolve_local_sequence_input(
        &self,
        raw: &str,
    ) -> Result<(SequenceInput, ArtifactProvenance, Vec<Diagnostic>), ServiceError> {
        let reference = self.classify_input(raw.to_owned())?;
        match self.resolve_input(
            reference,
            epithema_providers::ResolutionIntent::SequenceInput,
        )? {
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

    fn resolve_local_coordinate_input(
        &self,
        raw: &str,
    ) -> Result<(PsiphiInput, ArtifactProvenance, Vec<Diagnostic>), ServiceError> {
        let reference = self.classify_input(raw.to_owned())?;
        match self.resolve_input(reference, epithema_providers::ResolutionIntent::SequenceInput)? {
            ToolInputResolution::LocalFile {
                canonical_path,
                provenance,
                diagnostics,
                ..
            } => Ok((PsiphiInput::new(canonical_path), provenance, diagnostics)),
            ToolInputResolution::ProviderRouted { provenance, .. } => Err(PlatformError::new(
                ErrorCategory::NotImplemented,
                "provider-backed coordinate acquisition is not implemented for this tool cohort yet",
            )
            .with_code("service.tool.input.provider_not_supported")
            .with_detail(provenance.locator().to_owned())),
            ToolInputResolution::InlineSequence { .. } => Err(PlatformError::new(
                ErrorCategory::NotImplemented,
                "inline coordinate literals are not accepted for protein-coordinate input files",
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
        match self.resolve_input(
            reference,
            epithema_providers::ResolutionIntent::SequenceInput,
        )? {
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
        match self.resolve_input(
            reference,
            epithema_providers::ResolutionIntent::SequenceInput,
        )? {
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

fn parse_makenucseq_params(arguments: &[String]) -> Result<MakenucseqParams, ServiceError> {
    if arguments.len() < 2 {
        return Err(tool_usage_error("makenucseq", makenucseq_help()));
    }

    let identifier_prefix = arguments[0].clone();
    let length = parse_positive_count("makenucseq", &arguments[1], "<length>")?;
    let mut count = 1usize;
    let mut seed = 1_u64;
    let mut molecule = MoleculeKind::Dna;
    let mut description = None;
    let mut index = 2usize;

    while index < arguments.len() {
        let argument = &arguments[index];
        if let Some(value) = argument.strip_prefix("--count=") {
            count = parse_positive_count("makenucseq", value, "--count")?;
            index += 1;
            continue;
        }
        if argument == "--count" {
            let value = arguments.get(index + 1).ok_or_else(|| {
                PlatformError::new(ErrorCategory::Validation, "missing value for --count")
                    .with_code("service.tool.makenucseq.count_missing")
            })?;
            count = parse_positive_count("makenucseq", value, "--count")?;
            index += 2;
            continue;
        }
        if let Some(value) = argument.strip_prefix("--seed=") {
            seed = parse_u64_value("makenucseq", value, "--seed")?;
            index += 1;
            continue;
        }
        if argument == "--seed" {
            let value = arguments.get(index + 1).ok_or_else(|| {
                PlatformError::new(ErrorCategory::Validation, "missing value for --seed")
                    .with_code("service.tool.makenucseq.seed_missing")
            })?;
            seed = parse_u64_value("makenucseq", value, "--seed")?;
            index += 2;
            continue;
        }
        if let Some(value) = argument.strip_prefix("--molecule=") {
            molecule = parse_makenucseq_molecule(value)?;
            index += 1;
            continue;
        }
        if argument == "--molecule" {
            let value = arguments.get(index + 1).ok_or_else(|| {
                PlatformError::new(ErrorCategory::Validation, "missing value for --molecule")
                    .with_code("service.tool.makenucseq.molecule_missing")
            })?;
            molecule = parse_makenucseq_molecule(value)?;
            index += 2;
            continue;
        }
        if let Some(value) = argument.strip_prefix("--description=") {
            description = Some(value.to_owned());
            index += 1;
            continue;
        }
        if argument == "--description" {
            let value = arguments.get(index + 1).ok_or_else(|| {
                PlatformError::new(ErrorCategory::Validation, "missing value for --description")
                    .with_code("service.tool.makenucseq.description_missing")
            })?;
            description = Some(value.clone());
            index += 2;
            continue;
        }

        return Err(PlatformError::new(
            ErrorCategory::Validation,
            format!("unknown makenucseq argument '{argument}'"),
        )
        .with_code("service.tool.makenucseq.argument_unknown")
        .with_detail(makenucseq_help()));
    }

    Ok(MakenucseqParams {
        identifier_prefix,
        length,
        count,
        seed,
        molecule,
        description,
    })
}

fn parse_makenucseq_molecule(value: &str) -> Result<MoleculeKind, ServiceError> {
    match value.to_ascii_lowercase().as_str() {
        "dna" => Ok(MoleculeKind::Dna),
        "rna" => Ok(MoleculeKind::Rna),
        _ => Err(PlatformError::new(
            ErrorCategory::Validation,
            "makenucseq molecule must be dna or rna",
        )
        .with_code("service.tool.makenucseq.molecule_invalid")
        .with_detail(value.to_owned())),
    }
}

fn parse_makeprotseq_params(arguments: &[String]) -> Result<MakeprotseqParams, ServiceError> {
    if arguments.len() < 2 {
        return Err(tool_usage_error("makeprotseq", makeprotseq_help()));
    }

    let identifier_prefix = arguments[0].clone();
    let length = parse_positive_count("makeprotseq", &arguments[1], "<length>")?;
    let mut count = 1usize;
    let mut seed = 1_u64;
    let mut description = None;
    let mut index = 2usize;

    while index < arguments.len() {
        let argument = &arguments[index];
        if let Some(value) = argument.strip_prefix("--count=") {
            count = parse_positive_count("makeprotseq", value, "--count")?;
            index += 1;
            continue;
        }
        if argument == "--count" {
            let value = arguments.get(index + 1).ok_or_else(|| {
                PlatformError::new(ErrorCategory::Validation, "missing value for --count")
                    .with_code("service.tool.makeprotseq.count_missing")
            })?;
            count = parse_positive_count("makeprotseq", value, "--count")?;
            index += 2;
            continue;
        }
        if let Some(value) = argument.strip_prefix("--seed=") {
            seed = parse_u64_value("makeprotseq", value, "--seed")?;
            index += 1;
            continue;
        }
        if argument == "--seed" {
            let value = arguments.get(index + 1).ok_or_else(|| {
                PlatformError::new(ErrorCategory::Validation, "missing value for --seed")
                    .with_code("service.tool.makeprotseq.seed_missing")
            })?;
            seed = parse_u64_value("makeprotseq", value, "--seed")?;
            index += 2;
            continue;
        }
        if let Some(value) = argument.strip_prefix("--description=") {
            description = Some(value.to_owned());
            index += 1;
            continue;
        }
        if argument == "--description" {
            let value = arguments.get(index + 1).ok_or_else(|| {
                PlatformError::new(ErrorCategory::Validation, "missing value for --description")
                    .with_code("service.tool.makeprotseq.description_missing")
            })?;
            description = Some(value.clone());
            index += 2;
            continue;
        }

        return Err(PlatformError::new(
            ErrorCategory::Validation,
            format!("unknown makeprotseq argument '{argument}'"),
        )
        .with_code("service.tool.makeprotseq.argument_unknown")
        .with_detail(makeprotseq_help()));
    }

    Ok(MakeprotseqParams {
        identifier_prefix,
        length,
        count,
        seed,
        description,
    })
}

fn parse_infoseq_params(arguments: &[String]) -> Result<InfoseqParams, ServiceError> {
    if arguments.len() != 1 {
        return Err(tool_usage_error("infoseq", infoseq_help()));
    }

    Ok(InfoseqParams {
        input: SequenceInput::new(arguments[0].clone()),
    })
}

fn parse_single_char_argument(tool: &str, value: &str, label: &str) -> Result<char, ServiceError> {
    let mut chars = value.chars();
    let character = chars.next().ok_or_else(|| {
        PlatformError::new(
            ErrorCategory::Validation,
            format!("{tool} requires one {label} character"),
        )
        .with_code(format!("service.tool.{tool}.{label}_empty"))
        .with_detail("expected a single non-empty character")
    })?;

    if chars.next().is_some() {
        return Err(PlatformError::new(
            ErrorCategory::Validation,
            format!("{tool} requires exactly one {label} character"),
        )
        .with_code(format!("service.tool.{tool}.{label}_length"))
        .with_detail(value.to_owned()));
    }

    Ok(character)
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

fn parse_protein_regex(
    tool: &str,
    value: &str,
) -> Result<epithema_tools::pattern_tools::CompiledProteinRegex, ServiceError> {
    epithema_tools::pattern_tools::CompiledProteinRegex::parse(value).map_err(|error| {
        PlatformError::new(ErrorCategory::Validation, error.to_string())
            .with_code(format!("service.tool.{tool}.pattern.regex_invalid"))
    })
}

fn parse_nucleotide_regex(
    tool: &str,
    value: &str,
) -> Result<epithema_tools::pattern_tools::CompiledNucleotideRegex, ServiceError> {
    epithema_tools::pattern_tools::CompiledNucleotideRegex::parse(value).map_err(|error| {
        PlatformError::new(ErrorCategory::Validation, error.to_string())
            .with_code(format!("service.tool.{tool}.pattern.regex_invalid"))
    })
}

fn parse_palindrome_params(arguments: &[String]) -> Result<PalindromeParams, ServiceError> {
    if arguments.is_empty() {
        return Err(tool_usage_error("palindrome", palindrome_help()));
    }

    let input = SequenceInput::new(arguments[0].clone());
    let mut min_length = 4usize;
    let mut max_length = 12usize;
    let mut index = 1usize;

    while index < arguments.len() {
        let argument = &arguments[index];
        if let Some(value) = argument.strip_prefix("--min-length=") {
            min_length = parse_positive_count("palindrome", value, "--min-length")?;
            index += 1;
            continue;
        }
        if let Some(value) = argument.strip_prefix("--max-length=") {
            max_length = parse_positive_count("palindrome", value, "--max-length")?;
            index += 1;
            continue;
        }
        if argument == "--min-length" {
            let value = arguments.get(index + 1).ok_or_else(|| {
                PlatformError::new(ErrorCategory::Validation, "missing value for --min-length")
                    .with_code("service.tool.palindrome.min_length_missing")
            })?;
            min_length = parse_positive_count("palindrome", value, "--min-length")?;
            index += 2;
            continue;
        }
        if argument == "--max-length" {
            let value = arguments.get(index + 1).ok_or_else(|| {
                PlatformError::new(ErrorCategory::Validation, "missing value for --max-length")
                    .with_code("service.tool.palindrome.max_length_missing")
            })?;
            max_length = parse_positive_count("palindrome", value, "--max-length")?;
            index += 2;
            continue;
        }

        return Err(PlatformError::new(
            ErrorCategory::Validation,
            format!("unknown palindrome argument '{argument}'"),
        )
        .with_code("service.tool.palindrome.argument_unknown")
        .with_detail(palindrome_help()));
    }

    Ok(PalindromeParams {
        input,
        min_length,
        max_length,
    })
}

fn parse_einverted_params(arguments: &[String]) -> Result<EinvertedParams, ServiceError> {
    if arguments.is_empty() {
        return Err(tool_usage_error("einverted", einverted_help()));
    }

    let input = SequenceInput::new(arguments[0].clone());
    let mut min_arm_length = 4usize;
    let mut max_gap_length = 3usize;
    let mut index = 1usize;

    while index < arguments.len() {
        let argument = &arguments[index];
        if let Some(value) = argument.strip_prefix("--min-arm-length=") {
            min_arm_length = parse_positive_count("einverted", value, "--min-arm-length")?;
            index += 1;
            continue;
        }
        if let Some(value) = argument.strip_prefix("--max-gap-length=") {
            max_gap_length = parse_non_negative_count("einverted", value)?;
            index += 1;
            continue;
        }
        if argument == "--min-arm-length" {
            let value = arguments.get(index + 1).ok_or_else(|| {
                PlatformError::new(
                    ErrorCategory::Validation,
                    "missing value for --min-arm-length",
                )
                .with_code("service.tool.einverted.min_arm_length_missing")
            })?;
            min_arm_length = parse_positive_count("einverted", value, "--min-arm-length")?;
            index += 2;
            continue;
        }
        if argument == "--max-gap-length" {
            let value = arguments.get(index + 1).ok_or_else(|| {
                PlatformError::new(
                    ErrorCategory::Validation,
                    "missing value for --max-gap-length",
                )
                .with_code("service.tool.einverted.max_gap_length_missing")
            })?;
            max_gap_length = parse_non_negative_count("einverted", value)?;
            index += 2;
            continue;
        }

        return Err(PlatformError::new(
            ErrorCategory::Validation,
            format!("unknown einverted argument '{argument}'"),
        )
        .with_code("service.tool.einverted.argument_unknown")
        .with_detail(einverted_help()));
    }

    Ok(EinvertedParams {
        input,
        min_arm_length,
        max_gap_length,
    })
}

fn parse_patmatdb_params(arguments: &[String]) -> Result<PatmatdbParams, ServiceError> {
    if arguments.len() != 2 {
        return Err(tool_usage_error("patmatdb", patmatdb_help()));
    }

    Ok(PatmatdbParams {
        input: SequenceInput::new(arguments[0].clone()),
        database: PathBuf::from(arguments[1].clone()),
    })
}

fn parse_wordmatch_params(arguments: &[String]) -> Result<WordmatchParams, ServiceError> {
    if arguments.len() < 2 {
        return Err(tool_usage_error("wordmatch", wordmatch_help()));
    }

    let query = SequenceInput::new(arguments[0].clone());
    let target = SequenceInput::new(arguments[1].clone());
    let mut word_size = 4usize;
    let mut index = 2usize;

    while index < arguments.len() {
        let argument = &arguments[index];
        if let Some(value) = argument.strip_prefix("--word-size=") {
            word_size = parse_positive_count("wordmatch", value, "--word-size")?;
            index += 1;
            continue;
        }
        if argument == "--word-size" {
            let value = arguments.get(index + 1).ok_or_else(|| {
                PlatformError::new(ErrorCategory::Validation, "missing value for --word-size")
                    .with_code("service.tool.wordmatch.word_size_missing")
            })?;
            word_size = parse_positive_count("wordmatch", value, "--word-size")?;
            index += 2;
            continue;
        }

        return Err(PlatformError::new(
            ErrorCategory::Validation,
            format!("unknown wordmatch argument '{argument}'"),
        )
        .with_code("service.tool.wordmatch.argument_unknown")
        .with_detail(wordmatch_help()));
    }

    Ok(WordmatchParams {
        query,
        target,
        word_size,
    })
}

fn parse_wordfinder_params(arguments: &[String]) -> Result<WordfinderParams, ServiceError> {
    if arguments.len() < 2 {
        return Err(tool_usage_error("wordfinder", wordfinder_help()));
    }

    let query = SequenceInput::new(arguments[0].clone());
    let targets = SequenceInput::new(arguments[1].clone());
    let mut word_size = 4usize;
    let mut index = 2usize;

    while index < arguments.len() {
        let argument = &arguments[index];
        if let Some(value) = argument.strip_prefix("--word-size=") {
            word_size = parse_positive_count("wordfinder", value, "--word-size")?;
            index += 1;
            continue;
        }
        if argument == "--word-size" {
            let value = arguments.get(index + 1).ok_or_else(|| {
                PlatformError::new(ErrorCategory::Validation, "missing value for --word-size")
                    .with_code("service.tool.wordfinder.word_size_missing")
            })?;
            word_size = parse_positive_count("wordfinder", value, "--word-size")?;
            index += 2;
            continue;
        }

        return Err(PlatformError::new(
            ErrorCategory::Validation,
            format!("unknown wordfinder argument '{argument}'"),
        )
        .with_code("service.tool.wordfinder.argument_unknown")
        .with_detail(wordfinder_help()));
    }

    Ok(WordfinderParams {
        query,
        targets,
        word_size,
    })
}

fn parse_seqmatchall_params(arguments: &[String]) -> Result<SeqmatchallParams, ServiceError> {
    if arguments.is_empty() {
        return Err(tool_usage_error("seqmatchall", seqmatchall_help()));
    }

    let input = SequenceInput::new(arguments[0].clone());
    let mut word_size = 4usize;
    let mut index = 1usize;

    while index < arguments.len() {
        let argument = &arguments[index];
        if let Some(value) = argument.strip_prefix("--word-size=") {
            word_size = parse_positive_count("seqmatchall", value, "--word-size")?;
            index += 1;
            continue;
        }
        if argument == "--word-size" {
            let value = arguments.get(index + 1).ok_or_else(|| {
                PlatformError::new(ErrorCategory::Validation, "missing value for --word-size")
                    .with_code("service.tool.seqmatchall.word_size_missing")
            })?;
            word_size = parse_positive_count("seqmatchall", value, "--word-size")?;
            index += 2;
            continue;
        }

        return Err(PlatformError::new(
            ErrorCategory::Validation,
            format!("unknown seqmatchall argument '{argument}'"),
        )
        .with_code("service.tool.seqmatchall.argument_unknown")
        .with_detail(seqmatchall_help()));
    }

    Ok(SeqmatchallParams { input, word_size })
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

fn parse_dan_params(arguments: &[String]) -> Result<DanParams, ServiceError> {
    if arguments.is_empty() {
        return Err(tool_usage_error("dan", dan_help()));
    }

    let input = SequenceInput::new(arguments[0].clone());
    let mut window = None;
    let mut step = 1usize;
    let mut index = 1usize;

    while index < arguments.len() {
        let argument = &arguments[index];
        if let Some(value) = argument.strip_prefix("--window=") {
            window = Some(parse_positive_count("dan", value, "--window")?);
            index += 1;
            continue;
        }
        if argument == "--window" {
            let value = arguments.get(index + 1).ok_or_else(|| {
                PlatformError::new(ErrorCategory::Validation, "missing value for --window")
                    .with_code("service.tool.dan.window_missing")
            })?;
            window = Some(parse_positive_count("dan", value, "--window")?);
            index += 2;
            continue;
        }
        if let Some(value) = argument.strip_prefix("--step=") {
            step = parse_positive_count("dan", value, "--step")?;
            index += 1;
            continue;
        }
        if argument == "--step" {
            let value = arguments.get(index + 1).ok_or_else(|| {
                PlatformError::new(ErrorCategory::Validation, "missing value for --step")
                    .with_code("service.tool.dan.step_missing")
            })?;
            step = parse_positive_count("dan", value, "--step")?;
            index += 2;
            continue;
        }

        return Err(PlatformError::new(
            ErrorCategory::Validation,
            format!("unknown dan argument '{argument}'"),
        )
        .with_code("service.tool.dan.argument_unknown")
        .with_detail(dan_help()));
    }

    Ok(DanParams {
        input,
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

fn parse_hmoment_params(
    arguments: &[String],
) -> Result<(HmomentParams, Option<PathBuf>), ServiceError> {
    if arguments.is_empty() {
        return Err(tool_usage_error("hmoment", hmoment_help()));
    }

    let input = SequenceInput::new(arguments[0].clone());
    let mut window = 11usize;
    let mut step = 1usize;
    let mut angle_degrees = 100.0f64;
    let mut plot_contract_out = None;
    let mut index = 1usize;

    while index < arguments.len() {
        let argument = &arguments[index];
        if let Some(value) = argument.strip_prefix("--window=") {
            window = parse_positive_count("hmoment", value, "--window")?;
            index += 1;
            continue;
        }
        if argument == "--window" {
            let value = arguments.get(index + 1).ok_or_else(|| {
                PlatformError::new(ErrorCategory::Validation, "missing value for --window")
                    .with_code("service.tool.hmoment.window_missing")
            })?;
            window = parse_positive_count("hmoment", value, "--window")?;
            index += 2;
            continue;
        }
        if let Some(value) = argument.strip_prefix("--step=") {
            step = parse_positive_count("hmoment", value, "--step")?;
            index += 1;
            continue;
        }
        if argument == "--step" {
            let value = arguments.get(index + 1).ok_or_else(|| {
                PlatformError::new(ErrorCategory::Validation, "missing value for --step")
                    .with_code("service.tool.hmoment.step_missing")
            })?;
            step = parse_positive_count("hmoment", value, "--step")?;
            index += 2;
            continue;
        }
        if let Some(value) = argument.strip_prefix("--angle-degrees=") {
            angle_degrees = value.parse::<f64>().map_err(|_| {
                PlatformError::new(
                    ErrorCategory::Validation,
                    "--angle-degrees must be a finite floating-point number",
                )
                .with_code("service.tool.hmoment.angle_degrees_invalid")
                .with_detail(value.to_owned())
            })?;
            if !angle_degrees.is_finite() {
                return Err(PlatformError::new(
                    ErrorCategory::Validation,
                    "--angle-degrees must be a finite floating-point number",
                )
                .with_code("service.tool.hmoment.angle_degrees_invalid")
                .with_detail(value.to_owned()));
            }
            index += 1;
            continue;
        }
        if argument == "--angle-degrees" {
            let value = arguments.get(index + 1).ok_or_else(|| {
                PlatformError::new(
                    ErrorCategory::Validation,
                    "missing value for --angle-degrees",
                )
                .with_code("service.tool.hmoment.angle_degrees_missing")
            })?;
            angle_degrees = value.parse::<f64>().map_err(|_| {
                PlatformError::new(
                    ErrorCategory::Validation,
                    "--angle-degrees must be a finite floating-point number",
                )
                .with_code("service.tool.hmoment.angle_degrees_invalid")
                .with_detail(value.to_owned())
            })?;
            if !angle_degrees.is_finite() {
                return Err(PlatformError::new(
                    ErrorCategory::Validation,
                    "--angle-degrees must be a finite floating-point number",
                )
                .with_code("service.tool.hmoment.angle_degrees_invalid")
                .with_detail(value.to_owned()));
            }
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
                .with_code("service.tool.hmoment.plot_contract_out_missing")
            })?;
            plot_contract_out = Some(PathBuf::from(value));
            index += 2;
            continue;
        }

        return Err(PlatformError::new(
            ErrorCategory::Validation,
            format!("unknown hmoment argument '{argument}'"),
        )
        .with_code("service.tool.hmoment.argument_unknown")
        .with_detail(hmoment_help()));
    }

    Ok((
        HmomentParams {
            input,
            window,
            step,
            angle_degrees,
        },
        plot_contract_out,
    ))
}

fn parse_banana_params(
    arguments: &[String],
) -> Result<(BananaParams, Option<PathBuf>), ServiceError> {
    if arguments.is_empty() {
        return Err(tool_usage_error("banana", banana_help()));
    }

    let input = SequenceInput::new(arguments[0].clone());
    let mut plot_contract_out = None;
    let mut index = 1usize;

    while index < arguments.len() {
        let argument = &arguments[index];
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
                .with_code("service.tool.banana.plot_contract_out_missing")
            })?;
            plot_contract_out = Some(PathBuf::from(value));
            index += 2;
            continue;
        }

        return Err(PlatformError::new(
            ErrorCategory::Validation,
            format!("unknown banana argument '{argument}'"),
        )
        .with_code("service.tool.banana.argument_unknown")
        .with_detail(banana_help()));
    }

    Ok((BananaParams { input }, plot_contract_out))
}

fn parse_density_params(
    arguments: &[String],
) -> Result<(DensityParams, Option<PathBuf>), ServiceError> {
    if arguments.is_empty() {
        return Err(tool_usage_error("density", density_help()));
    }

    let input = SequenceInput::new(arguments[0].clone());
    let mut window = 11usize;
    let mut step = 1usize;
    let mut plot_contract_out = None;
    let mut index = 1usize;

    while index < arguments.len() {
        let argument = &arguments[index];
        if let Some(value) = argument.strip_prefix("--window=") {
            window = parse_positive_count("density", value, "--window")?;
            index += 1;
            continue;
        }
        if argument == "--window" {
            let value = arguments.get(index + 1).ok_or_else(|| {
                PlatformError::new(ErrorCategory::Validation, "missing value for --window")
                    .with_code("service.tool.density.window_missing")
            })?;
            window = parse_positive_count("density", value, "--window")?;
            index += 2;
            continue;
        }
        if let Some(value) = argument.strip_prefix("--step=") {
            step = parse_positive_count("density", value, "--step")?;
            index += 1;
            continue;
        }
        if argument == "--step" {
            let value = arguments.get(index + 1).ok_or_else(|| {
                PlatformError::new(ErrorCategory::Validation, "missing value for --step")
                    .with_code("service.tool.density.step_missing")
            })?;
            step = parse_positive_count("density", value, "--step")?;
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
                .with_code("service.tool.density.plot_contract_out_missing")
            })?;
            plot_contract_out = Some(PathBuf::from(value));
            index += 2;
            continue;
        }

        return Err(PlatformError::new(
            ErrorCategory::Validation,
            format!("unknown density argument '{argument}'"),
        )
        .with_code("service.tool.density.argument_unknown")
        .with_detail(density_help()));
    }

    Ok((
        DensityParams {
            input,
            window,
            step,
        },
        plot_contract_out,
    ))
}

fn parse_wobble_params(
    arguments: &[String],
) -> Result<(WobbleParams, Option<PathBuf>), ServiceError> {
    if arguments.is_empty() {
        return Err(tool_usage_error("wobble", wobble_help()));
    }

    let input = SequenceInput::new(arguments[0].clone());
    let mut codon_window = 11usize;
    let mut codon_step = 1usize;
    let mut plot_contract_out = None;
    let mut index = 1usize;

    while index < arguments.len() {
        let argument = &arguments[index];
        if let Some(value) = argument.strip_prefix("--codon-window=") {
            codon_window = parse_positive_count("wobble", value, "--codon-window")?;
            index += 1;
            continue;
        }
        if argument == "--codon-window" {
            let value = arguments.get(index + 1).ok_or_else(|| {
                PlatformError::new(
                    ErrorCategory::Validation,
                    "missing value for --codon-window",
                )
                .with_code("service.tool.wobble.codon_window_missing")
            })?;
            codon_window = parse_positive_count("wobble", value, "--codon-window")?;
            index += 2;
            continue;
        }
        if let Some(value) = argument.strip_prefix("--codon-step=") {
            codon_step = parse_positive_count("wobble", value, "--codon-step")?;
            index += 1;
            continue;
        }
        if argument == "--codon-step" {
            let value = arguments.get(index + 1).ok_or_else(|| {
                PlatformError::new(ErrorCategory::Validation, "missing value for --codon-step")
                    .with_code("service.tool.wobble.codon_step_missing")
            })?;
            codon_step = parse_positive_count("wobble", value, "--codon-step")?;
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
                .with_code("service.tool.wobble.plot_contract_out_missing")
            })?;
            plot_contract_out = Some(PathBuf::from(value));
            index += 2;
            continue;
        }

        return Err(PlatformError::new(
            ErrorCategory::Validation,
            format!("unknown wobble argument '{argument}'"),
        )
        .with_code("service.tool.wobble.argument_unknown")
        .with_detail(wobble_help()));
    }

    Ok((
        WobbleParams {
            input,
            codon_window,
            codon_step,
        },
        plot_contract_out,
    ))
}

fn parse_isochore_params(
    arguments: &[String],
) -> Result<(IsochoreParams, Option<PathBuf>), ServiceError> {
    if arguments.is_empty() {
        return Err(tool_usage_error("isochore", isochore_help()));
    }

    let input = SequenceInput::new(arguments[0].clone());
    let mut window = 11usize;
    let mut step = 1usize;
    let mut plot_contract_out = None;
    let mut index = 1usize;

    while index < arguments.len() {
        let argument = &arguments[index];
        if let Some(value) = argument.strip_prefix("--window=") {
            window = parse_positive_count("isochore", value, "--window")?;
            index += 1;
            continue;
        }
        if argument == "--window" {
            let value = arguments.get(index + 1).ok_or_else(|| {
                PlatformError::new(ErrorCategory::Validation, "missing value for --window")
                    .with_code("service.tool.isochore.window_missing")
            })?;
            window = parse_positive_count("isochore", value, "--window")?;
            index += 2;
            continue;
        }
        if let Some(value) = argument.strip_prefix("--step=") {
            step = parse_positive_count("isochore", value, "--step")?;
            index += 1;
            continue;
        }
        if argument == "--step" {
            let value = arguments.get(index + 1).ok_or_else(|| {
                PlatformError::new(ErrorCategory::Validation, "missing value for --step")
                    .with_code("service.tool.isochore.step_missing")
            })?;
            step = parse_positive_count("isochore", value, "--step")?;
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
                .with_code("service.tool.isochore.plot_contract_out_missing")
            })?;
            plot_contract_out = Some(PathBuf::from(value));
            index += 2;
            continue;
        }

        return Err(PlatformError::new(
            ErrorCategory::Validation,
            format!("unknown isochore argument '{argument}'"),
        )
        .with_code("service.tool.isochore.argument_unknown")
        .with_detail(isochore_help()));
    }

    Ok((
        IsochoreParams {
            input,
            window,
            step,
        },
        plot_contract_out,
    ))
}

fn parse_syco_params(arguments: &[String]) -> Result<(SycoParams, Option<PathBuf>), ServiceError> {
    if arguments.len() < 2 {
        return Err(tool_usage_error("syco", syco_help()));
    }

    let input = SequenceInput::new(arguments[0].clone());
    let reference = PathBuf::from(&arguments[1]);
    let mut codon_window = 11usize;
    let mut codon_step = 1usize;
    let mut plot_contract_out = None;
    let mut index = 2usize;

    while index < arguments.len() {
        let argument = &arguments[index];
        if let Some(value) = argument.strip_prefix("--codon-window=") {
            codon_window = parse_positive_count("syco", value, "--codon-window")?;
            index += 1;
            continue;
        }
        if argument == "--codon-window" {
            let value = arguments.get(index + 1).ok_or_else(|| {
                PlatformError::new(
                    ErrorCategory::Validation,
                    "missing value for --codon-window",
                )
                .with_code("service.tool.syco.codon_window_missing")
            })?;
            codon_window = parse_positive_count("syco", value, "--codon-window")?;
            index += 2;
            continue;
        }
        if let Some(value) = argument.strip_prefix("--codon-step=") {
            codon_step = parse_positive_count("syco", value, "--codon-step")?;
            index += 1;
            continue;
        }
        if argument == "--codon-step" {
            let value = arguments.get(index + 1).ok_or_else(|| {
                PlatformError::new(ErrorCategory::Validation, "missing value for --codon-step")
                    .with_code("service.tool.syco.codon_step_missing")
            })?;
            codon_step = parse_positive_count("syco", value, "--codon-step")?;
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
                .with_code("service.tool.syco.plot_contract_out_missing")
            })?;
            plot_contract_out = Some(PathBuf::from(value));
            index += 2;
            continue;
        }

        return Err(PlatformError::new(
            ErrorCategory::Validation,
            format!("unknown syco argument '{argument}'"),
        )
        .with_code("service.tool.syco.argument_unknown")
        .with_detail(syco_help()));
    }

    Ok((
        SycoParams {
            input,
            reference,
            codon_window,
            codon_step,
        },
        plot_contract_out,
    ))
}

fn parse_octanol_params(
    arguments: &[String],
) -> Result<(OctanolParams, Option<PathBuf>), ServiceError> {
    if arguments.is_empty() {
        return Err(tool_usage_error("octanol", octanol_help()));
    }

    let input = SequenceInput::new(arguments[0].clone());
    let mut window = 19usize;
    let mut step = 1usize;
    let mut plot_contract_out = None;
    let mut index = 1usize;

    while index < arguments.len() {
        let argument = &arguments[index];
        if let Some(value) = argument.strip_prefix("--window=") {
            window = parse_positive_count("octanol", value, "--window")?;
            index += 1;
            continue;
        }
        if argument == "--window" {
            let value = arguments.get(index + 1).ok_or_else(|| {
                PlatformError::new(ErrorCategory::Validation, "missing value for --window")
                    .with_code("service.tool.octanol.window_missing")
            })?;
            window = parse_positive_count("octanol", value, "--window")?;
            index += 2;
            continue;
        }
        if let Some(value) = argument.strip_prefix("--step=") {
            step = parse_positive_count("octanol", value, "--step")?;
            index += 1;
            continue;
        }
        if argument == "--step" {
            let value = arguments.get(index + 1).ok_or_else(|| {
                PlatformError::new(ErrorCategory::Validation, "missing value for --step")
                    .with_code("service.tool.octanol.step_missing")
            })?;
            step = parse_positive_count("octanol", value, "--step")?;
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
                .with_code("service.tool.octanol.plot_contract_out_missing")
            })?;
            plot_contract_out = Some(PathBuf::from(value));
            index += 2;
            continue;
        }

        return Err(PlatformError::new(
            ErrorCategory::Validation,
            format!("unknown octanol argument '{argument}'"),
        )
        .with_code("service.tool.octanol.argument_unknown")
        .with_detail(octanol_help()));
    }

    Ok((
        OctanolParams {
            input,
            window,
            step,
        },
        plot_contract_out,
    ))
}

fn parse_pepinfo_params(
    arguments: &[String],
) -> Result<(PepinfoParams, Option<PathBuf>), ServiceError> {
    if arguments.is_empty() {
        return Err(tool_usage_error("pepinfo", pepinfo_help()));
    }

    let input = SequenceInput::new(arguments[0].clone());
    let mut window = 9usize;
    let mut step = 1usize;
    let mut plot_contract_out = None;
    let mut index = 1usize;

    while index < arguments.len() {
        let argument = &arguments[index];
        if let Some(value) = argument.strip_prefix("--window=") {
            window = parse_positive_count("pepinfo", value, "--window")?;
            index += 1;
            continue;
        }
        if argument == "--window" {
            let value = arguments.get(index + 1).ok_or_else(|| {
                PlatformError::new(ErrorCategory::Validation, "missing value for --window")
                    .with_code("service.tool.pepinfo.window_missing")
            })?;
            window = parse_positive_count("pepinfo", value, "--window")?;
            index += 2;
            continue;
        }
        if let Some(value) = argument.strip_prefix("--step=") {
            step = parse_positive_count("pepinfo", value, "--step")?;
            index += 1;
            continue;
        }
        if argument == "--step" {
            let value = arguments.get(index + 1).ok_or_else(|| {
                PlatformError::new(ErrorCategory::Validation, "missing value for --step")
                    .with_code("service.tool.pepinfo.step_missing")
            })?;
            step = parse_positive_count("pepinfo", value, "--step")?;
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
                .with_code("service.tool.pepinfo.plot_contract_out_missing")
            })?;
            plot_contract_out = Some(PathBuf::from(value));
            index += 2;
            continue;
        }

        return Err(PlatformError::new(
            ErrorCategory::Validation,
            format!("unknown pepinfo argument '{argument}'"),
        )
        .with_code("service.tool.pepinfo.argument_unknown")
        .with_detail(pepinfo_help()));
    }

    Ok((
        PepinfoParams {
            input,
            window,
            step,
        },
        plot_contract_out,
    ))
}

fn parse_pepwindow_params(
    arguments: &[String],
) -> Result<(PepwindowParams, Option<PathBuf>), ServiceError> {
    if arguments.is_empty() {
        return Err(tool_usage_error("pepwindow", pepwindow_help()));
    }

    let input = SequenceInput::new(arguments[0].clone());
    let mut window = 19usize;
    let mut step = 1usize;
    let mut plot_contract_out = None;
    let mut index = 1usize;

    while index < arguments.len() {
        let argument = &arguments[index];
        if let Some(value) = argument.strip_prefix("--window=") {
            window = parse_positive_count("pepwindow", value, "--window")?;
            index += 1;
            continue;
        }
        if argument == "--window" {
            let value = arguments.get(index + 1).ok_or_else(|| {
                PlatformError::new(ErrorCategory::Validation, "missing value for --window")
                    .with_code("service.tool.pepwindow.window_missing")
            })?;
            window = parse_positive_count("pepwindow", value, "--window")?;
            index += 2;
            continue;
        }
        if let Some(value) = argument.strip_prefix("--step=") {
            step = parse_positive_count("pepwindow", value, "--step")?;
            index += 1;
            continue;
        }
        if argument == "--step" {
            let value = arguments.get(index + 1).ok_or_else(|| {
                PlatformError::new(ErrorCategory::Validation, "missing value for --step")
                    .with_code("service.tool.pepwindow.step_missing")
            })?;
            step = parse_positive_count("pepwindow", value, "--step")?;
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
                .with_code("service.tool.pepwindow.plot_contract_out_missing")
            })?;
            plot_contract_out = Some(PathBuf::from(value));
            index += 2;
            continue;
        }

        return Err(PlatformError::new(
            ErrorCategory::Validation,
            format!("unknown pepwindow argument '{argument}'"),
        )
        .with_code("service.tool.pepwindow.argument_unknown")
        .with_detail(pepwindow_help()));
    }

    Ok((
        PepwindowParams {
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

fn parse_diffseq_params(arguments: &[String]) -> Result<DiffseqParams, ServiceError> {
    if arguments.len() < 2 {
        return Err(tool_usage_error("diffseq", diffseq_help()));
    }

    let asequence = SequenceInput::new(arguments[0].clone());
    let bsequence = SequenceInput::new(arguments[1].clone());
    let mut gap_open = 5;
    let mut gap_extend = 1;
    let mut index = 2usize;

    while index < arguments.len() {
        let argument = &arguments[index];
        if let Some(value) = argument.strip_prefix("--gap-open=") {
            gap_open = parse_positive_i32("diffseq", value, "--gap-open")?;
            index += 1;
            continue;
        }
        if argument == "--gap-open" {
            let value = arguments.get(index + 1).ok_or_else(|| {
                PlatformError::new(ErrorCategory::Validation, "missing value for --gap-open")
                    .with_code("service.tool.diffseq.gap_open_missing")
            })?;
            gap_open = parse_positive_i32("diffseq", value, "--gap-open")?;
            index += 2;
            continue;
        }
        if let Some(value) = argument.strip_prefix("--gap-extend=") {
            gap_extend = parse_positive_i32("diffseq", value, "--gap-extend")?;
            index += 1;
            continue;
        }
        if argument == "--gap-extend" {
            let value = arguments.get(index + 1).ok_or_else(|| {
                PlatformError::new(ErrorCategory::Validation, "missing value for --gap-extend")
                    .with_code("service.tool.diffseq.gap_extend_missing")
            })?;
            gap_extend = parse_positive_i32("diffseq", value, "--gap-extend")?;
            index += 2;
            continue;
        }

        return Err(PlatformError::new(
            ErrorCategory::Validation,
            format!("unknown diffseq argument '{argument}'"),
        )
        .with_code("service.tool.diffseq.argument_unknown")
        .with_detail(diffseq_help()));
    }

    Ok(DiffseqParams {
        asequence,
        bsequence,
        gap_open,
        gap_extend,
    })
}

fn parse_edialign_params(arguments: &[String]) -> Result<EdialignParams, ServiceError> {
    if arguments.is_empty() {
        return Err(tool_usage_error("edialign", edialign_help()));
    }

    let input = SequenceInput::new(arguments[0].clone());
    let mut min_length = 2usize;
    let mut index = 1usize;

    while index < arguments.len() {
        let argument = &arguments[index];
        if let Some(value) = argument.strip_prefix("--min-length=") {
            min_length = parse_positive_count("edialign", value, "--min-length")?;
            index += 1;
            continue;
        }
        if argument == "--min-length" {
            let value = arguments.get(index + 1).ok_or_else(|| {
                PlatformError::new(ErrorCategory::Validation, "missing value for --min-length")
                    .with_code("service.tool.edialign.min_length_missing")
            })?;
            min_length = parse_positive_count("edialign", value, "--min-length")?;
            index += 2;
            continue;
        }

        return Err(PlatformError::new(
            ErrorCategory::Validation,
            format!("unknown edialign argument '{argument}'"),
        )
        .with_code("service.tool.edialign.argument_unknown")
        .with_detail(edialign_help()));
    }

    Ok(EdialignParams { input, min_length })
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

struct NgslistCliParams {
    accession: String,
    provider: String,
    format: NgslistFormat,
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct NgsgetCliParams {
    accession: String,
    provider: String,
    output_root: PathBuf,
    include_raw: bool,
    existing_download_roots: Vec<PathBuf>,
}

fn parse_ngslist_arguments(arguments: &[String]) -> Result<NgslistCliParams, ServiceError> {
    if arguments.is_empty() {
        return Err(tool_usage_error("ngslist", ngslist_help()));
    }

    let mut accession = None;
    let mut provider = "auto".to_owned();
    let mut format = NgslistFormat::Table;
    let mut index = 0usize;

    while index < arguments.len() {
        let argument = &arguments[index];
        if let Some(value) = argument.strip_prefix("--provider=") {
            provider = parse_ngslist_provider(value)?;
            index += 1;
            continue;
        }
        if argument == "--provider" {
            let value = arguments.get(index + 1).ok_or_else(|| {
                PlatformError::new(ErrorCategory::Validation, "missing value for --provider")
                    .with_code("service.ngslist.provider_missing")
                    .with_detail(ngslist_help())
            })?;
            provider = parse_ngslist_provider(value)?;
            index += 2;
            continue;
        }
        if let Some(value) = argument.strip_prefix("--format=") {
            format = parse_ngslist_format(value)?;
            index += 1;
            continue;
        }
        if argument == "--format" {
            let value = arguments.get(index + 1).ok_or_else(|| {
                PlatformError::new(ErrorCategory::Validation, "missing value for --format")
                    .with_code("service.ngslist.format_missing")
                    .with_detail(ngslist_help())
            })?;
            format = parse_ngslist_format(value)?;
            index += 2;
            continue;
        }
        if argument.starts_with("--") {
            return Err(PlatformError::new(
                ErrorCategory::Validation,
                format!("unknown ngslist argument '{argument}'"),
            )
            .with_code("service.ngslist.argument_unknown")
            .with_detail(ngslist_help()));
        }
        if accession.is_some() {
            return Err(tool_usage_error("ngslist", ngslist_help()));
        }
        accession = Some(argument.clone());
        index += 1;
    }

    let accession = accession.ok_or_else(|| tool_usage_error("ngslist", ngslist_help()))?;
    Ok(NgslistCliParams {
        accession,
        provider,
        format,
    })
}

fn parse_ngslist_provider(value: &str) -> Result<String, ServiceError> {
    match value.trim().to_ascii_lowercase().as_str() {
        "auto" => Ok("auto".to_owned()),
        "ena" => Ok("ena".to_owned()),
        "sra" => Ok("sra".to_owned()),
        _ => Err(PlatformError::new(
            ErrorCategory::Validation,
            "ngslist --provider must be one of auto, ena, or sra",
        )
        .with_code("service.ngslist.provider_invalid")
        .with_detail(value.to_owned())),
    }
}

fn parse_ngslist_format(value: &str) -> Result<NgslistFormat, ServiceError> {
    match value.trim().to_ascii_lowercase().as_str() {
        "table" => Ok(NgslistFormat::Table),
        "json" => Ok(NgslistFormat::Json),
        _ => Err(PlatformError::new(
            ErrorCategory::Validation,
            "ngslist --format must be one of table or json",
        )
        .with_code("service.ngslist.format_invalid")
        .with_detail(value.to_owned())),
    }
}

fn build_ngslist_query(accession: &str, provider: &str) -> Result<NgsQuery, ServiceError> {
    let query = NgsQuery::classify(accession)?;
    if provider == "auto" {
        return Ok(query);
    }

    let requested_provider = ProviderId::new(provider.to_owned())?;
    if let Some(query_provider) = &query.provider {
        if query_provider.as_str() != requested_provider.as_str() {
            return Err(PlatformError::new(
                ErrorCategory::Validation,
                "ngslist provider flag conflicts with the provider-qualified accession",
            )
            .with_code("service.ngslist.provider_conflict")
            .with_detail(format!(
                "query={} flag={}",
                query_provider.as_str(),
                requested_provider.as_str()
            )));
        }
        return Ok(query);
    }

    Ok(query.with_provider(requested_provider))
}

fn parse_ngsget_arguments(arguments: &[String]) -> Result<NgsgetCliParams, ServiceError> {
    if arguments.is_empty() {
        return Err(tool_usage_error("ngsget", ngsget_help()));
    }

    let mut accession = None;
    let mut provider = "auto".to_owned();
    let mut output_root = PathBuf::from("ngsget-out");
    let mut include_raw = false;
    let mut existing_download_roots = Vec::new();
    let mut index = 0usize;

    while index < arguments.len() {
        let argument = &arguments[index];
        if let Some(value) = argument.strip_prefix("--provider=") {
            provider = parse_ngsget_provider(value)?;
            index += 1;
            continue;
        }
        if argument == "--provider" {
            let value = arguments.get(index + 1).ok_or_else(|| {
                PlatformError::new(ErrorCategory::Validation, "missing value for --provider")
                    .with_code("service.ngsget.provider_missing")
                    .with_detail(ngsget_help())
            })?;
            provider = parse_ngsget_provider(value)?;
            index += 2;
            continue;
        }
        if let Some(value) = argument.strip_prefix("--out=") {
            output_root = PathBuf::from(value);
            index += 1;
            continue;
        }
        if argument == "--out" {
            let value = arguments.get(index + 1).ok_or_else(|| {
                PlatformError::new(ErrorCategory::Validation, "missing value for --out")
                    .with_code("service.ngsget.out_missing")
                    .with_detail(ngsget_help())
            })?;
            output_root = PathBuf::from(value);
            index += 2;
            continue;
        }
        if let Some(value) = argument.strip_prefix("--check-downloads=") {
            existing_download_roots.push(PathBuf::from(value));
            index += 1;
            continue;
        }
        if argument == "--check-downloads" {
            let value = arguments.get(index + 1).ok_or_else(|| {
                PlatformError::new(
                    ErrorCategory::Validation,
                    "missing value for --check-downloads",
                )
                .with_code("service.ngsget.check_downloads_missing")
                .with_detail(ngsget_help())
            })?;
            existing_download_roots.push(PathBuf::from(value));
            index += 2;
            continue;
        }
        if argument == "--raw" {
            include_raw = true;
            index += 1;
            continue;
        }
        if argument == "--container" || argument.starts_with("--container=") {
            return Err(PlatformError::new(
                ErrorCategory::Invocation,
                "ngsget --container is not implemented yet; the service uses the pinned default SRA Toolkit container",
            )
            .with_code("service.ngsget.container_not_supported")
            .with_detail(ngsget_help()));
        }
        if argument.starts_with("--") {
            return Err(PlatformError::new(
                ErrorCategory::Validation,
                format!("unknown ngsget argument '{argument}'"),
            )
            .with_code("service.ngsget.argument_unknown")
            .with_detail(ngsget_help()));
        }
        if accession.is_some() {
            return Err(tool_usage_error("ngsget", ngsget_help()));
        }
        accession = Some(argument.clone());
        index += 1;
    }

    let accession = accession.ok_or_else(|| tool_usage_error("ngsget", ngsget_help()))?;
    Ok(NgsgetCliParams {
        accession,
        provider,
        output_root,
        include_raw,
        existing_download_roots,
    })
}

fn parse_ngsget_provider(value: &str) -> Result<String, ServiceError> {
    match value.trim().to_ascii_lowercase().as_str() {
        "auto" => Ok("auto".to_owned()),
        "ena" => Ok("ena".to_owned()),
        "sra" => Ok("sra".to_owned()),
        _ => Err(PlatformError::new(
            ErrorCategory::Validation,
            "ngsget --provider must be one of auto, ena, or sra",
        )
        .with_code("service.ngsget.provider_invalid")
        .with_detail(value.to_owned())),
    }
}

fn build_ngsget_query(accession: &str, provider: &str) -> Result<NgsQuery, ServiceError> {
    let query = NgsQuery::classify(accession)?;
    if provider == "auto" {
        return Ok(query);
    }

    let requested_provider = ProviderId::new(provider.to_owned())?;
    if let Some(query_provider) = &query.provider {
        if query_provider.as_str() != requested_provider.as_str() {
            return Err(PlatformError::new(
                ErrorCategory::Validation,
                "ngsget provider flag conflicts with the provider-qualified accession",
            )
            .with_code("service.ngsget.provider_conflict")
            .with_detail(format!(
                "query={} flag={}",
                query_provider.as_str(),
                requested_provider.as_str()
            )));
        }
        return Ok(query);
    }

    Ok(query.with_provider(requested_provider))
}

fn archive_file_rows(files: &[epithema_providers::ArchiveFile]) -> Vec<Vec<String>> {
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

fn ngs_manifest_columns() -> Vec<String> {
    [
        "provider",
        "query_accession",
        "query_object_class",
        "study_accession",
        "study_title",
        "sample_accession",
        "sample_title",
        "experiment_accession",
        "run_accession",
        "instrument_platform",
        "instrument_model",
        "library_strategy",
        "library_source",
        "library_selection",
        "library_layout",
        "asset_role",
        "asset_format",
        "source_url",
        "size_bytes",
        "checksum_md5",
    ]
    .iter()
    .map(|column| (*column).to_owned())
    .collect()
}

fn ngs_manifest_rows(manifest: &NgsManifest) -> Vec<Vec<String>> {
    let provider = manifest.provider.as_str().to_owned();
    let query_accession = manifest.query.accession.clone();
    let query_object_class = manifest
        .query
        .object_class
        .map(|object_class| object_class.as_str().to_owned())
        .unwrap_or_else(|| "-".to_owned());

    manifest
        .runs
        .iter()
        .flat_map(|run| {
            run.assets.iter().map({
                let provider = provider.clone();
                let query_accession = query_accession.clone();
                let query_object_class = query_object_class.clone();
                move |asset| {
                    vec![
                        provider.clone(),
                        query_accession.clone(),
                        query_object_class.clone(),
                        optional_string(&run.metadata.study_accession),
                        optional_string(&run.metadata.study_title),
                        optional_string(&run.metadata.sample_accession),
                        optional_string(&run.metadata.sample_title),
                        optional_string(&run.metadata.experiment_accession),
                        run.metadata.run_accession.clone(),
                        optional_string(&run.metadata.instrument_platform),
                        optional_string(&run.metadata.instrument_model),
                        optional_string(&run.metadata.library_strategy),
                        optional_string(&run.metadata.library_source),
                        optional_string(&run.metadata.library_selection),
                        optional_string(&run.metadata.library_layout),
                        asset.role.as_str().to_owned(),
                        asset.format.clone(),
                        asset.source_url.clone(),
                        asset
                            .size_bytes
                            .map(|value| value.to_string())
                            .unwrap_or_else(|| "-".to_owned()),
                        asset.checksum_md5.clone().unwrap_or_else(|| "-".to_owned()),
                    ]
                }
            })
        })
        .collect()
}

fn optional_string(value: &Option<String>) -> String {
    value.clone().unwrap_or_else(|| "-".to_owned())
}

fn render_ngs_manifest_json(manifest: &NgsManifest) -> String {
    let columns = ngs_manifest_columns();
    let rows = ngs_manifest_rows(manifest);
    let query_object_class = manifest
        .query
        .object_class
        .map(|object_class| object_class.as_str())
        .unwrap_or("-");
    let mut rendered = String::new();
    rendered.push_str("{\n");
    rendered.push_str("  \"schema\": \"epithema.ngslist/v1\",\n");
    rendered.push_str(&format!(
        "  \"provider\": \"{}\",\n",
        json_escape(manifest.provider.as_str())
    ));
    rendered.push_str(&format!(
        "  \"query_accession\": \"{}\",\n",
        json_escape(&manifest.query.accession)
    ));
    rendered.push_str(&format!(
        "  \"query_object_class\": \"{}\",\n",
        json_escape(query_object_class)
    ));
    rendered.push_str(&format!(
        "  \"route_endpoint\": \"{}\",\n",
        json_escape(&manifest.route.endpoint)
    ));
    rendered.push_str(&format!("  \"run_count\": {},\n", manifest.runs.len()));
    rendered.push_str(&format!("  \"asset_count\": {},\n", rows.len()));
    rendered.push_str("  \"assets\": [\n");
    for (row_index, row) in rows.iter().enumerate() {
        rendered.push_str("    {\n");
        for (column_index, column) in columns.iter().enumerate() {
            let comma = if column_index + 1 == columns.len() {
                ""
            } else {
                ","
            };
            rendered.push_str(&format!(
                "      \"{}\": \"{}\"{}\n",
                json_escape(column),
                json_escape(row.get(column_index).map_or("-", String::as_str)),
                comma
            ));
        }
        let comma = if row_index + 1 == rows.len() { "" } else { "," };
        rendered.push_str(&format!("    }}{comma}\n"));
    }
    rendered.push_str("  ]\n");
    rendered.push('}');
    rendered
}

fn render_ngsget_report(
    outcome: &epithema_tools::archive_tools::NgsgetOutcome,
    manifest_path: &std::path::Path,
    provenance_path: &std::path::Path,
    materialization_record_count: usize,
) -> String {
    let mut rendered = String::new();
    rendered.push_str("ngsget acquisition summary\n");
    rendered.push_str(&format!("provider\t{}\n", outcome.provider));
    rendered.push_str(&format!("accession\t{}\n", outcome.accession));
    rendered.push_str(&format!("output_root\t{}\n", outcome.output_root.display()));
    rendered.push_str(&format!("include_raw\t{}\n", outcome.include_raw));
    rendered.push_str(&format!("runs\t{}\n", outcome.run_count));
    rendered.push_str(&format!(
        "selected_assets\t{}\n",
        outcome.selected_asset_count
    ));
    rendered.push_str(&format!(
        "materialization_records\t{}\n",
        materialization_record_count
    ));
    rendered.push_str(&format!(
        "failed_records\t{}\n",
        outcome.failed_record_count
    ));
    if outcome.failed_record_count > 0 {
        rendered.push_str(
            "warning\tone or more selected assets failed materialization or verification; inspect provenance.json for failure_reason entries\n",
        );
    }
    rendered.push_str(&format!("manifest\t{}\n", manifest_path.display()));
    rendered.push_str(&format!("provenance\t{}\n", provenance_path.display()));
    rendered
}

fn json_escape(value: &str) -> String {
    let mut escaped = String::new();
    for ch in value.chars() {
        match ch {
            '"' => escaped.push_str("\\\""),
            '\\' => escaped.push_str("\\\\"),
            '\n' => escaped.push_str("\\n"),
            '\r' => escaped.push_str("\\r"),
            '\t' => escaped.push_str("\\t"),
            ch if ch.is_control() => escaped.push_str(&format!("\\u{:04x}", ch as u32)),
            ch => escaped.push(ch),
        }
    }
    escaped
}

fn infoassembly_rows(
    outcome: &epithema_tools::archive_tools::InfoassemblyOutcome,
) -> Vec<Vec<String>> {
    vec![
        vec!["provider".to_owned(), outcome.provider.clone()],
        vec!["accession".to_owned(), outcome.accession.clone()],
        vec!["object_class".to_owned(), outcome.object_class.clone()],
        vec![
            "assembly_accession".to_owned(),
            outcome.assembly_accession.clone(),
        ],
        vec![
            "run_accession".to_owned(),
            outcome
                .run_accession
                .clone()
                .unwrap_or_else(|| "-".to_owned()),
        ],
        vec![
            "experiment_accession".to_owned(),
            outcome
                .experiment_accession
                .clone()
                .unwrap_or_else(|| "-".to_owned()),
        ],
        vec![
            "sample_accession".to_owned(),
            outcome
                .sample_accession
                .clone()
                .unwrap_or_else(|| "-".to_owned()),
        ],
        vec![
            "platform".to_owned(),
            outcome.platform.clone().unwrap_or_else(|| "-".to_owned()),
        ],
        vec![
            "instrument_model".to_owned(),
            outcome
                .instrument_model
                .clone()
                .unwrap_or_else(|| "-".to_owned()),
        ],
        vec![
            "library_layout".to_owned(),
            outcome
                .library_layout
                .clone()
                .unwrap_or_else(|| "-".to_owned()),
        ],
        vec![
            "library_strategy".to_owned(),
            outcome
                .library_strategy
                .clone()
                .unwrap_or_else(|| "-".to_owned()),
        ],
        vec![
            "library_source".to_owned(),
            outcome
                .library_source
                .clone()
                .unwrap_or_else(|| "-".to_owned()),
        ],
        vec!["file_count".to_owned(), outcome.file_count.to_string()],
        vec![
            "total_size_bytes".to_owned(),
            outcome
                .total_size_bytes
                .map_or_else(|| "-".to_owned(), |value| value.to_string()),
        ],
        vec!["route_endpoint".to_owned(), outcome.route_endpoint.clone()],
    ]
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

fn parse_positive_i32(tool: &str, value: &str, flag: &str) -> Result<i32, ServiceError> {
    let parsed = value.parse::<i32>().map_err(|_| {
        PlatformError::new(
            ErrorCategory::Validation,
            format!("{flag} must be a positive integer"),
        )
        .with_code(format!("service.tool.{tool}.{flag}_invalid"))
        .with_detail(value.to_owned())
    })?;

    if parsed <= 0 {
        return Err(PlatformError::new(
            ErrorCategory::Validation,
            format!("{flag} must be a positive integer"),
        )
        .with_code(format!("service.tool.{tool}.{flag}_invalid"))
        .with_detail(value.to_owned()));
    }

    Ok(parsed)
}

fn parse_u64_value(tool: &str, value: &str, flag: &str) -> Result<u64, ServiceError> {
    value.parse::<u64>().map_err(|_| {
        PlatformError::new(
            ErrorCategory::Validation,
            format!("{tool} requires {flag} to be an unsigned 64-bit integer"),
        )
        .with_code(format!("service.tool.{tool}.{flag}_invalid"))
        .with_detail(value.to_owned())
    })
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

fn parse_biosed_params(arguments: &[String]) -> Result<BiosedParams, ServiceError> {
    if arguments.len() < 3 {
        return Err(tool_usage_error("biosed", biosed_help()));
    }

    let input = SequenceInput::new(arguments[0].clone());
    let start = parse_positive_count("biosed", &arguments[1], "<start>")?;
    let end = parse_positive_count("biosed", &arguments[2], "<end>")?;
    let mut replacement = None;
    let mut index = 3usize;

    while index < arguments.len() {
        let argument = &arguments[index];
        if let Some(value) = argument.strip_prefix("--replace=") {
            replacement = Some(value.to_owned());
            index += 1;
            continue;
        }
        if argument == "--replace" {
            let value = arguments.get(index + 1).ok_or_else(|| {
                PlatformError::new(ErrorCategory::Validation, "missing value for --replace")
                    .with_code("service.tool.biosed.replace_missing")
            })?;
            replacement = Some(value.clone());
            index += 2;
            continue;
        }

        return Err(PlatformError::new(
            ErrorCategory::Validation,
            format!("unknown biosed argument '{argument}'"),
        )
        .with_code("service.tool.biosed.argument_unknown")
        .with_detail(biosed_help()));
    }

    Ok(BiosedParams {
        input,
        start,
        end,
        replacement,
    })
}

fn parse_msbar_params(arguments: &[String]) -> Result<MsbarParams, ServiceError> {
    if arguments.len() < 2 {
        return Err(tool_usage_error("msbar", msbar_help()));
    }

    let input = SequenceInput::new(arguments[0].clone());
    let mutations = arguments[1..]
        .iter()
        .map(|argument| parse_msbar_mutation(argument))
        .collect::<Result<Vec<_>, _>>()?;

    Ok(MsbarParams { input, mutations })
}

fn parse_msbar_mutation(value: &str) -> Result<MsbarMutation, ServiceError> {
    let (position, residue) = value.split_once(':').ok_or_else(|| {
        PlatformError::new(
            ErrorCategory::Validation,
            "msbar mutations must use position:residue syntax",
        )
        .with_code("service.tool.msbar.mutation_invalid")
        .with_detail(value.to_owned())
    })?;
    Ok(MsbarMutation {
        position: parse_positive_count("msbar", position, "<position>")?,
        residue: parse_single_char_argument("msbar", residue, "mutation")?,
    })
}

fn parse_trimest_params(arguments: &[String]) -> Result<TrimestParams, ServiceError> {
    if arguments.is_empty() {
        return Err(tool_usage_error("trimest", trimest_help()));
    }

    let input = SequenceInput::new(arguments[0].clone());
    let mut min_tail = 4usize;
    let mut index = 1usize;

    while index < arguments.len() {
        let argument = &arguments[index];
        if let Some(value) = argument.strip_prefix("--min-tail=") {
            min_tail = parse_positive_count("trimest", value, "--min-tail")?;
            index += 1;
            continue;
        }
        if argument == "--min-tail" {
            let value = arguments.get(index + 1).ok_or_else(|| {
                PlatformError::new(ErrorCategory::Validation, "missing value for --min-tail")
                    .with_code("service.tool.trimest.min_tail_missing")
            })?;
            min_tail = parse_positive_count("trimest", value, "--min-tail")?;
            index += 2;
            continue;
        }

        return Err(PlatformError::new(
            ErrorCategory::Validation,
            format!("unknown trimest argument '{argument}'"),
        )
        .with_code("service.tool.trimest.argument_unknown")
        .with_detail(trimest_help()));
    }

    Ok(TrimestParams { input, min_tail })
}

fn parse_vectorstrip_params(arguments: &[String]) -> Result<VectorstripParams, ServiceError> {
    let [input, vector]: [String; 2] = arguments
        .to_vec()
        .try_into()
        .map_err(|_| tool_usage_error("vectorstrip", vectorstrip_help()))?;
    Ok(VectorstripParams {
        input: SequenceInput::new(input),
        vector: SequenceInput::new(vector),
    })
}

fn parse_shuffleseq_params(arguments: &[String]) -> Result<(String, u64), ServiceError> {
    if arguments.is_empty() {
        return Err(tool_usage_error("shuffleseq", shuffleseq_help()));
    }

    let mut input = None;
    let mut seed = 1_u64;
    let mut index = 0usize;

    while index < arguments.len() {
        match arguments[index].as_str() {
            "--seed" => {
                let value = arguments
                    .get(index + 1)
                    .ok_or_else(|| tool_usage_error("shuffleseq", shuffleseq_help()))?;
                seed = parse_u64_value("shuffleseq", value, "--seed")?;
                index += 2;
            }
            argument if !argument.starts_with("--") && input.is_none() => {
                input = Some(argument.to_owned());
                index += 1;
            }
            _ => return Err(tool_usage_error("shuffleseq", shuffleseq_help())),
        }
    }

    let input = input.ok_or_else(|| tool_usage_error("shuffleseq", shuffleseq_help()))?;
    Ok((input, seed))
}

fn parse_pasteseq_params(arguments: &[String]) -> Result<PasteseqParams, ServiceError> {
    let [asequence, bsequence, position]: [String; 3] = arguments
        .to_vec()
        .try_into()
        .map_err(|_| tool_usage_error("pasteseq", pasteseq_help()))?;
    Ok(PasteseqParams {
        asequence: SequenceInput::new(asequence),
        bsequence: SequenceInput::new(bsequence),
        position: parse_non_negative_count("pasteseq", &position)?,
    })
}

fn parse_listor_params(arguments: &[String]) -> Result<ListorParams, ServiceError> {
    if arguments.len() < 2 {
        return Err(tool_usage_error("listor", listor_help()));
    }

    let mut positional = Vec::new();
    let mut operator = SequenceSetOperator::Or;
    let mut index = 0usize;

    while index < arguments.len() {
        let argument = &arguments[index];
        if let Some(value) = argument.strip_prefix("--operator=") {
            operator = parse_listor_operator(value)?;
            index += 1;
            continue;
        }
        if argument == "--operator" {
            let value = arguments
                .get(index + 1)
                .ok_or_else(|| tool_usage_error("listor", listor_help()))?;
            operator = parse_listor_operator(value)?;
            index += 2;
            continue;
        }
        if argument.starts_with("--") {
            return Err(tool_usage_error("listor", listor_help()));
        }
        positional.push(argument.clone());
        index += 1;
    }

    if positional.len() != 2 {
        return Err(tool_usage_error("listor", listor_help()));
    }

    Ok(ListorParams {
        first: SequenceInput::new(positional[0].clone()),
        second: SequenceInput::new(positional[1].clone()),
        operator,
    })
}

fn parse_listor_operator(value: &str) -> Result<SequenceSetOperator, ServiceError> {
    match value.to_ascii_uppercase().as_str() {
        "OR" => Ok(SequenceSetOperator::Or),
        "AND" => Ok(SequenceSetOperator::And),
        "XOR" => Ok(SequenceSetOperator::Xor),
        "NOT" => Ok(SequenceSetOperator::Not),
        _ => Err(PlatformError::new(
            ErrorCategory::Validation,
            format!("unsupported listor operator '{value}'"),
        )
        .with_code("service.tool.listor.operator_invalid")
        .with_detail(listor_help())),
    }
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

fn parse_wordcount_params(
    arguments: &[String],
) -> Result<(WordcountParams, Option<PathBuf>), ServiceError> {
    if arguments.is_empty() {
        return Err(tool_usage_error("wordcount", wordcount_help()));
    }

    let input = SequenceInput::new(arguments[0].clone());
    let mut word_size = None;
    let mut min_count = 1usize;
    let mut plot_contract_out = None;
    let mut index = 1usize;

    while index < arguments.len() {
        let argument = &arguments[index];
        if let Some(value) = argument
            .strip_prefix("--word-size=")
            .or_else(|| argument.strip_prefix("--wordsize="))
        {
            word_size = Some(parse_positive_count("wordcount", value, "--word-size")?);
            index += 1;
            continue;
        }
        if argument == "--word-size" || argument == "--wordsize" {
            let value = arguments.get(index + 1).ok_or_else(|| {
                PlatformError::new(ErrorCategory::Validation, "missing value for --word-size")
                    .with_code("service.tool.wordcount.word_size_missing")
            })?;
            word_size = Some(parse_positive_count("wordcount", value, "--word-size")?);
            index += 2;
            continue;
        }
        if let Some(value) = argument
            .strip_prefix("--min-count=")
            .or_else(|| argument.strip_prefix("--mincount="))
        {
            min_count = parse_positive_count("wordcount", value, "--min-count")?;
            index += 1;
            continue;
        }
        if argument == "--min-count" || argument == "--mincount" {
            let value = arguments.get(index + 1).ok_or_else(|| {
                PlatformError::new(ErrorCategory::Validation, "missing value for --min-count")
                    .with_code("service.tool.wordcount.min_count_missing")
            })?;
            min_count = parse_positive_count("wordcount", value, "--min-count")?;
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
                .with_code("service.tool.wordcount.plot_contract_out_missing")
            })?;
            plot_contract_out = Some(PathBuf::from(value));
            index += 2;
            continue;
        }

        return Err(PlatformError::new(
            ErrorCategory::Validation,
            format!("unknown wordcount argument '{argument}'"),
        )
        .with_code("service.tool.wordcount.argument_unknown")
        .with_detail(wordcount_help()));
    }

    Ok((
        WordcountParams {
            input,
            word_size: word_size.ok_or_else(|| tool_usage_error("wordcount", wordcount_help()))?,
            min_count,
        },
        plot_contract_out,
    ))
}

fn parse_oddcomp_params(arguments: &[String]) -> Result<OddcompParams, ServiceError> {
    if arguments.len() < 3 {
        return Err(tool_usage_error("oddcomp", oddcomp_help()));
    }

    let input = SequenceInput::new(arguments[0].clone());
    let mut query_words = Vec::new();
    let mut index = 1usize;

    while index < arguments.len() {
        let argument = &arguments[index];
        if argument == "--word" {
            let value = arguments.get(index + 1).ok_or_else(|| {
                PlatformError::new(ErrorCategory::Validation, "missing value for --word")
                    .with_code("service.tool.oddcomp.word_missing")
            })?;
            query_words.push(value.clone());
            index += 2;
            continue;
        }
        if let Some(value) = argument.strip_prefix("--word=") {
            query_words.push(value.to_owned());
            index += 1;
            continue;
        }

        return Err(PlatformError::new(
            ErrorCategory::Validation,
            format!("unknown oddcomp argument '{argument}'"),
        )
        .with_code("service.tool.oddcomp.argument_unknown")
        .with_detail(oddcomp_help()));
    }

    if query_words.is_empty() {
        return Err(tool_usage_error("oddcomp", oddcomp_help()));
    }

    Ok(OddcompParams { input, query_words })
}

fn parse_pepdigest_params(arguments: &[String]) -> Result<PepdigestParams, ServiceError> {
    if arguments.is_empty() {
        return Err(tool_usage_error("pepdigest", pepdigest_help()));
    }

    let input = SequenceInput::new(arguments[0].clone());
    let mut protease = PepdigestProtease::Trypsin;
    let mut index = 1usize;

    while index < arguments.len() {
        let argument = &arguments[index];
        if let Some(value) = argument.strip_prefix("--protease=") {
            protease = parse_pepdigest_protease(value)?;
            index += 1;
            continue;
        }
        if argument == "--protease" {
            let value = arguments.get(index + 1).ok_or_else(|| {
                PlatformError::new(ErrorCategory::Validation, "missing value for --protease")
                    .with_code("service.tool.pepdigest.protease_missing")
            })?;
            protease = parse_pepdigest_protease(value)?;
            index += 2;
            continue;
        }

        return Err(PlatformError::new(
            ErrorCategory::Validation,
            format!("unknown pepdigest argument '{argument}'"),
        )
        .with_code("service.tool.pepdigest.argument_unknown")
        .with_detail(pepdigest_help()));
    }

    Ok(PepdigestParams { input, protease })
}

fn parse_pepdigest_protease(value: &str) -> Result<PepdigestProtease, ServiceError> {
    PepdigestProtease::from_str(value).map_err(|_| {
        PlatformError::new(
            ErrorCategory::Validation,
            "protease must be one of trypsin, lys-c, arg-c, or cnbr",
        )
        .with_code("service.tool.pepdigest.protease_invalid")
        .with_detail(value.to_owned())
    })
}

fn parse_recoder_params(arguments: &[String]) -> Result<RecoderParams, ServiceError> {
    let [input, site]: [String; 2] = arguments
        .to_vec()
        .try_into()
        .map_err(|_| tool_usage_error("recoder", recoder_help()))?;
    Ok(RecoderParams {
        input: SequenceInput::new(input),
        site,
    })
}

fn parse_silent_params(arguments: &[String]) -> Result<SilentParams, ServiceError> {
    let [input, site]: [String; 2] = arguments
        .to_vec()
        .try_into()
        .map_err(|_| tool_usage_error("silent", silent_help()))?;
    Ok(SilentParams {
        input: SequenceInput::new(input),
        site,
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

fn parse_maskambig_params(
    tool: &str,
    arguments: &[String],
    help: &str,
) -> Result<SequenceInput, ServiceError> {
    let [input]: [String; 1] = arguments
        .to_vec()
        .try_into()
        .map_err(|_| tool_usage_error(tool, help))?;
    Ok(SequenceInput::new(input))
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
    let (selector, mask_char) = parse_feature_selector_flags("featmerge", &arguments[2..], false)?;
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
    let (selector, mask_char) = parse_feature_selector_flags("featreport", &arguments[1..], false)?;
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
    let (selector, mask_char) = parse_feature_selector_flags("feattext", &arguments[1..], false)?;
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

fn parse_twofeat_params(arguments: &[String]) -> Result<TwofeatParams, ServiceError> {
    if arguments.is_empty() {
        return Err(tool_usage_error("twofeat", twofeat_help()));
    }

    let input = SequenceInput::new(arguments[0].clone());
    let mut min_range = None;
    let mut max_range = None;
    let mut a_selectors = Vec::new();
    let mut b_selectors = Vec::new();
    let mut index = 1usize;

    while index < arguments.len() {
        let argument = &arguments[index];
        if let Some(value) = argument.strip_prefix("--min-range=") {
            min_range = Some(parse_non_negative_count("twofeat", value)?);
            index += 1;
            continue;
        }
        if argument == "--min-range" {
            let value = arguments.get(index + 1).ok_or_else(|| {
                PlatformError::new(ErrorCategory::Validation, "missing value for --min-range")
                    .with_code("service.tool.twofeat.min_range_missing")
            })?;
            min_range = Some(parse_non_negative_count("twofeat", value)?);
            index += 2;
            continue;
        }
        if let Some(value) = argument.strip_prefix("--max-range=") {
            max_range = Some(parse_non_negative_count("twofeat", value)?);
            index += 1;
            continue;
        }
        if argument == "--max-range" {
            let value = arguments.get(index + 1).ok_or_else(|| {
                PlatformError::new(ErrorCategory::Validation, "missing value for --max-range")
                    .with_code("service.tool.twofeat.max_range_missing")
            })?;
            max_range = Some(parse_non_negative_count("twofeat", value)?);
            index += 2;
            continue;
        }
        if let Some(value) = argument.strip_prefix("--a-kind=") {
            a_selectors.push(FeatureSelector::Kind(parse_feature_kind(value)));
            index += 1;
            continue;
        }
        if argument == "--a-kind" {
            let value = arguments.get(index + 1).ok_or_else(|| {
                PlatformError::new(ErrorCategory::Validation, "missing value for --a-kind")
                    .with_code("service.tool.twofeat.a_kind_missing")
            })?;
            a_selectors.push(FeatureSelector::Kind(parse_feature_kind(value)));
            index += 2;
            continue;
        }
        if let Some(value) = argument.strip_prefix("--a-name=") {
            a_selectors.push(FeatureSelector::Name(value.to_owned()));
            index += 1;
            continue;
        }
        if argument == "--a-name" {
            let value = arguments.get(index + 1).ok_or_else(|| {
                PlatformError::new(ErrorCategory::Validation, "missing value for --a-name")
                    .with_code("service.tool.twofeat.a_name_missing")
            })?;
            a_selectors.push(FeatureSelector::Name(value.clone()));
            index += 2;
            continue;
        }
        if let Some(value) = argument.strip_prefix("--a-qualifier=") {
            a_selectors.push(parse_feature_qualifier_selector(value)?);
            index += 1;
            continue;
        }
        if argument == "--a-qualifier" {
            let value = arguments.get(index + 1).ok_or_else(|| {
                PlatformError::new(ErrorCategory::Validation, "missing value for --a-qualifier")
                    .with_code("service.tool.twofeat.a_qualifier_missing")
            })?;
            a_selectors.push(parse_feature_qualifier_selector(value)?);
            index += 2;
            continue;
        }
        if let Some(value) = argument.strip_prefix("--a-strand=") {
            a_selectors.push(FeatureSelector::Strand(parse_feature_strand(value)?));
            index += 1;
            continue;
        }
        if argument == "--a-strand" {
            let value = arguments.get(index + 1).ok_or_else(|| {
                PlatformError::new(ErrorCategory::Validation, "missing value for --a-strand")
                    .with_code("service.tool.twofeat.a_strand_missing")
            })?;
            a_selectors.push(FeatureSelector::Strand(parse_feature_strand(value)?));
            index += 2;
            continue;
        }
        if let Some(value) = argument.strip_prefix("--b-kind=") {
            b_selectors.push(FeatureSelector::Kind(parse_feature_kind(value)));
            index += 1;
            continue;
        }
        if argument == "--b-kind" {
            let value = arguments.get(index + 1).ok_or_else(|| {
                PlatformError::new(ErrorCategory::Validation, "missing value for --b-kind")
                    .with_code("service.tool.twofeat.b_kind_missing")
            })?;
            b_selectors.push(FeatureSelector::Kind(parse_feature_kind(value)));
            index += 2;
            continue;
        }
        if let Some(value) = argument.strip_prefix("--b-name=") {
            b_selectors.push(FeatureSelector::Name(value.to_owned()));
            index += 1;
            continue;
        }
        if argument == "--b-name" {
            let value = arguments.get(index + 1).ok_or_else(|| {
                PlatformError::new(ErrorCategory::Validation, "missing value for --b-name")
                    .with_code("service.tool.twofeat.b_name_missing")
            })?;
            b_selectors.push(FeatureSelector::Name(value.clone()));
            index += 2;
            continue;
        }
        if let Some(value) = argument.strip_prefix("--b-qualifier=") {
            b_selectors.push(parse_feature_qualifier_selector(value)?);
            index += 1;
            continue;
        }
        if argument == "--b-qualifier" {
            let value = arguments.get(index + 1).ok_or_else(|| {
                PlatformError::new(ErrorCategory::Validation, "missing value for --b-qualifier")
                    .with_code("service.tool.twofeat.b_qualifier_missing")
            })?;
            b_selectors.push(parse_feature_qualifier_selector(value)?);
            index += 2;
            continue;
        }
        if let Some(value) = argument.strip_prefix("--b-strand=") {
            b_selectors.push(FeatureSelector::Strand(parse_feature_strand(value)?));
            index += 1;
            continue;
        }
        if argument == "--b-strand" {
            let value = arguments.get(index + 1).ok_or_else(|| {
                PlatformError::new(ErrorCategory::Validation, "missing value for --b-strand")
                    .with_code("service.tool.twofeat.b_strand_missing")
            })?;
            b_selectors.push(FeatureSelector::Strand(parse_feature_strand(value)?));
            index += 2;
            continue;
        }

        return Err(PlatformError::new(
            ErrorCategory::Validation,
            format!("unknown twofeat argument '{argument}'"),
        )
        .with_code("service.tool.twofeat.argument_unknown")
        .with_detail(twofeat_help()));
    }

    let a_selector = match a_selectors.len() {
        0 => FeatureSelector::Any,
        1 => a_selectors.remove(0),
        _ => FeatureSelector::All(a_selectors),
    };
    let b_selector = match b_selectors.len() {
        0 => FeatureSelector::Any,
        1 => b_selectors.remove(0),
        _ => FeatureSelector::All(b_selectors),
    };

    Ok(TwofeatParams {
        input,
        a_selector,
        b_selector,
        min_range,
        max_range,
    })
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

fn describe_range_constraints(min_range: Option<usize>, max_range: Option<usize>) -> String {
    match (min_range, max_range) {
        (None, None) => "none".to_owned(),
        (Some(minimum), None) => format!("min={minimum}"),
        (None, Some(maximum)) => format!("max={maximum}"),
        (Some(minimum), Some(maximum)) => format!("min={minimum}, max={maximum}"),
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
        "seqretsetall" => seqretsetall_help(),
        "seqretsplit" => seqretsplit_help(),
        "refseqget" => refseqget_help(),
        "whichdb" => whichdb_help(),
        "infoassembly" => infoassembly_help(),
        "runinfo" => runinfo_help(),
        "runget" => runget_help(),
        "seqcount" => seqcount_help(),
        "nthseq" => nthseq_help(),
        "skipseq" => skipseq_help(),
        "listor" => listor_help(),
        "skipredundant" => skipredundant_help(),
        "notseq" => notseq_help(),
        "newseq" => newseq_help(),
        "makenucseq" => makenucseq_help(),
        "makeprotseq" => makeprotseq_help(),
        "biosed" => biosed_help(),
        "degapseq" => degapseq_help(),
        "revseq" => revseq_help(),
        "msbar" => msbar_help(),
        "trimest" => trimest_help(),
        "trimseq" => trimseq_help(),
        "descseq" => descseq_help(),
        "vectorstrip" => vectorstrip_help(),
        "infoseq" => infoseq_help(),
        "maskambignuc" => maskambignuc_help(),
        "maskambigprot" => maskambigprot_help(),
        "aligncopy" => aligncopy_help(),
        "aligncopypair" => aligncopypair_help(),
        "infoalign" => infoalign_help(),
        "extractalign" => extractalign_help(),
        "nthseqset" => nthseqset_help(),
        "maskfeat" => maskfeat_help(),
        "extractfeat" => extractfeat_help(),
        "featcopy" => featcopy_help(),
        "coderet" => coderet_help(),
        "featmerge" => featmerge_help(),
        "featreport" => featreport_help(),
        "feattext" => feattext_help(),
        "splitsource" => splitsource_help(),
        "twofeat" => twofeat_help(),
        "dreg" => dreg_help(),
        "einverted" => einverted_help(),
        "fuzznuc" => fuzznuc_help(),
        "fuzzpro" => fuzzpro_help(),
        "fuzztran" => fuzztran_help(),
        "palindrome" => palindrome_help(),
        "preg" => preg_help(),
        "patmatdb" => patmatdb_help(),
        "recoder" => recoder_help(),
        "silent" => silent_help(),
        "seqmatchall" => seqmatchall_help(),
        "wordmatch" => wordmatch_help(),
        "wordfinder" => wordfinder_help(),
        "seealso" => seealso_help(),
        "wossname" => wossname_help(),
        "banana" => banana_help(),
        "density" => density_help(),
        "wobble" => wobble_help(),
        "isochore" => isochore_help(),
        "syco" => syco_help(),
        "charge" => charge_help(),
        "hmoment" => hmoment_help(),
        "octanol" => octanol_help(),
        "pepinfo" => pepinfo_help(),
        "pepwindow" => pepwindow_help(),
        "eprimer3" => eprimer3_help(),
        "primersearch" => primersearch_help(),
        "sirna" => sirna_help(),
        "psiphi" => psiphi_help(),
        "aaindexextract" => aaindexextract_help(),
        "complex" => complex_help(),
        "compseq" => compseq_help(),
        "dan" => dan_help(),
        "geecee" => geecee_help(),
        "infobase" => infobase_help(),
        "inforesidue" => inforesidue_help(),
        "iep" => iep_help(),
        "oddcomp" => oddcomp_help(),
        "pepdigest" => pepdigest_help(),
        "pepstats" => pepstats_help(),
        "wordcount" => wordcount_help(),
        "chips" => chips_help(),
        "cusp" => cusp_help(),
        "codcopy" => codcopy_help(),
        "cai" => cai_help(),
        "codcmp" => codcmp_help(),
        "merger" => merger_help(),
        "megamerger" => megamerger_help(),
        "sizeseq" => sizeseq_help(),
        "shuffleseq" => shuffleseq_help(),
        "pasteseq" => pasteseq_help(),
        "splitter" => splitter_help(),
        "union" => union_help(),
        _ => "",
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::fs;
    use std::time::{SystemTime, UNIX_EPOCH};

    use epithema_core::MoleculeKind;
    use epithema_diagnostics::PlatformError;
    use epithema_providers::{
        EnaNgsAdapter, HttpRequest, HttpResponse, NgsQuery, ProviderHttpClient,
    };
    use epithema_tools::{ToolDescriptor, governed_tool_descriptors};

    use super::EpithemaService;
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
                    epithema_diagnostics::ErrorCategory::Invocation,
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

        let service = EpithemaService::new(registry);
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
        let service = EpithemaService::empty();
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("water").expect("tool name should be valid"),
        );

        assert!(service.invoke(request).is_err());
    }

    #[test]
    fn starts_with_default_platform_configuration_and_builtin_sequence_providers() {
        let service = EpithemaService::empty();
        assert!(
            service
                .providers()
                .find(&epithema_providers::ProviderId::new("ena").expect("valid provider"))
                .is_some()
        );
        assert!(
            service
                .providers()
                .find(&epithema_providers::ProviderId::new("ncbi").expect("valid provider"))
                .is_some()
        );
        assert!(
            service
                .providers()
                .find(&epithema_providers::ProviderId::new("sra").expect("valid provider"))
                .is_some()
        );
        assert!(service.config().acquisition.allow_remote_acquisition);
    }

    #[test]
    fn classifies_provider_qualified_inputs_through_service() {
        let service = EpithemaService::empty();
        let reference = service
            .classify_input("ena:AB000263")
            .expect("input should classify");
        assert_eq!(reference.kind(), ToolInputKind::ProviderQualified);
    }

    #[test]
    fn resolves_accessions_through_shared_service_seam() {
        let service = EpithemaService::empty();
        let reference = service
            .classify_input("AB000263")
            .expect("input should classify");

        let resolution = service
            .resolve_input(
                reference,
                epithema_providers::ResolutionIntent::SequenceInput,
            )
            .expect("resolution should succeed");
        assert!(matches!(
            resolution,
            ToolInputResolution::ProviderRouted { .. }
        ));
    }

    fn sequence_fixture() -> std::path::PathBuf {
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../epithema-tools/tests/fixtures/three_records.fasta")
    }

    fn second_sequence_fixture() -> std::path::PathBuf {
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../epithema-tools/tests/fixtures/two_records.fasta")
    }

    fn gapped_sequence_fixture() -> std::path::PathBuf {
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../epithema-tools/tests/fixtures/gapped_records.fasta")
    }

    fn wordcount_plot_fixture() -> std::path::PathBuf {
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../epithema-tools/tests/fixtures/wordcount_plot_contract.json")
    }

    fn annotated_feature_fixture() -> std::path::PathBuf {
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../epithema-tools/tests/fixtures/annotated_feature.gbk")
    }

    fn annotated_complex_fixture() -> std::path::PathBuf {
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../epithema-tools/tests/fixtures/annotated_complex.gbk")
    }

    fn featcopy_target_fixture() -> std::path::PathBuf {
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../epithema-tools/tests/fixtures/featcopy_target.fasta")
    }

    fn featmerge_right_fixture() -> std::path::PathBuf {
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../epithema-tools/tests/fixtures/annotated_merge_right.gbk")
    }

    fn featcopy_mismatch_fixture() -> std::path::PathBuf {
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../epithema-tools/tests/fixtures/featcopy_mismatch.fasta")
    }

    fn protein_fixture() -> std::path::PathBuf {
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../epithema-tools/tests/fixtures/protein_records.fasta")
    }

    fn nucleotide_pattern_fixture() -> std::path::PathBuf {
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../epithema-tools/tests/fixtures/nucleotide_pattern_records.fasta")
    }

    fn complex_fixture() -> std::path::PathBuf {
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../epithema-tools/tests/fixtures/complex_records.fasta")
    }

    fn complex_invalid_fixture() -> std::path::PathBuf {
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../epithema-tools/tests/fixtures/complex_invalid.fasta")
    }

    fn charge_fixture() -> std::path::PathBuf {
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../epithema-tools/tests/fixtures/charge_protein.fasta")
    }

    fn charge_invalid_fixture() -> std::path::PathBuf {
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../epithema-tools/tests/fixtures/charge_invalid.fasta")
    }

    fn charge_plot_fixture() -> std::path::PathBuf {
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../epithema-tools/tests/fixtures/charge_plot_contract.json")
    }

    fn hmoment_fixture() -> std::path::PathBuf {
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../epithema-tools/tests/fixtures/hmoment_protein.fasta")
    }

    fn density_fixture() -> std::path::PathBuf {
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../epithema-tools/tests/fixtures/density_nucleotide.fasta")
    }

    fn banana_fixture() -> std::path::PathBuf {
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../epithema-tools/tests/fixtures/banana_nucleotide.fasta")
    }

    fn wobble_fixture() -> std::path::PathBuf {
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../epithema-tools/tests/fixtures/wobble_coding_nucleotide.fasta")
    }

    fn isochore_fixture() -> std::path::PathBuf {
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../epithema-tools/tests/fixtures/isochore_nucleotide.fasta")
    }

    fn syco_fixture() -> std::path::PathBuf {
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../epithema-tools/tests/fixtures/syco_coding_nucleotide.fasta")
    }

    fn octanol_fixture() -> std::path::PathBuf {
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../epithema-tools/tests/fixtures/octanol_protein.fasta")
    }

    fn pepinfo_fixture() -> std::path::PathBuf {
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../epithema-tools/tests/fixtures/pepinfo_protein.fasta")
    }

    fn pepwindow_fixture() -> std::path::PathBuf {
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../epithema-tools/tests/fixtures/pepwindow_protein.fasta")
    }

    fn pepwindow_invalid_fixture() -> std::path::PathBuf {
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../epithema-tools/tests/fixtures/pepwindow_invalid.fasta")
    }

    fn pepwindow_plot_fixture() -> std::path::PathBuf {
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../epithema-tools/tests/fixtures/pepwindow_plot_contract.json")
    }

    fn psiphi_fixture() -> std::path::PathBuf {
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../epithema-tools/tests/fixtures/psiphi_backbone.txt")
    }

    fn primersearch_fixture() -> std::path::PathBuf {
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../epithema-tools/tests/fixtures/primersearch_targets.fasta")
    }

    fn eprimer3_fixture() -> std::path::PathBuf {
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../epithema-tools/tests/fixtures/eprimer3_targets.fasta")
    }

    fn sirna_fixture() -> std::path::PathBuf {
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../epithema-tools/tests/fixtures/sirna_targets.fasta")
    }

    fn primersearch_pairs_fixture() -> std::path::PathBuf {
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../epithema-tools/tests/fixtures/primersearch_pairs.tsv")
    }

    fn psiphi_invalid_fixture() -> std::path::PathBuf {
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../epithema-tools/tests/fixtures/psiphi_no_backbone.txt")
    }

    fn protein_stats_fixture() -> std::path::PathBuf {
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../epithema-tools/tests/fixtures/protein_stats_records.fasta")
    }

    fn iep_fixture() -> std::path::PathBuf {
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../epithema-tools/tests/fixtures/iep_records.fasta")
    }

    fn pepdigest_fixture() -> std::path::PathBuf {
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../epithema-tools/tests/fixtures/pepdigest_records.fasta")
    }

    fn recoder_fixture() -> std::path::PathBuf {
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../epithema-tools/tests/fixtures/recoder_records.fasta")
    }

    fn silent_fixture() -> std::path::PathBuf {
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../epithema-tools/tests/fixtures/silent_records.fasta")
    }

    fn oddcomp_fixture() -> std::path::PathBuf {
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../epithema-tools/tests/fixtures/oddcomp_records.fasta")
    }

    fn listor_first_fixture() -> std::path::PathBuf {
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../epithema-tools/tests/fixtures/listor_first.fasta")
    }

    fn listor_second_fixture() -> std::path::PathBuf {
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../epithema-tools/tests/fixtures/listor_second.fasta")
    }

    fn skipredundant_fixture() -> std::path::PathBuf {
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../epithema-tools/tests/fixtures/skipredundant_records.fasta")
    }

    fn nthseqset_fixture() -> std::path::PathBuf {
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../epithema-tools/tests/fixtures/nthseqset_alignments.sto")
    }

    fn splitsource_fixture() -> std::path::PathBuf {
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../epithema-tools/tests/fixtures/splitsource_annotated.gbk")
    }

    fn codon_reference_fixture() -> std::path::PathBuf {
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../epithema-tools/tests/fixtures/codon_reference.fasta")
    }

    fn codon_query_fixture() -> std::path::PathBuf {
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../epithema-tools/tests/fixtures/codon_query.fasta")
    }

    fn codon_compare_right_fixture() -> std::path::PathBuf {
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../epithema-tools/tests/fixtures/codon_compare_right.fasta")
    }

    fn pairwise_alignment_fixture() -> std::path::PathBuf {
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../epithema-tools/tests/fixtures/pairwise_alignment.sto")
    }

    fn multiple_alignment_fixture() -> std::path::PathBuf {
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../epithema-tools/tests/fixtures/multiple_alignment.sto")
    }

    fn needle_query_fixture() -> std::path::PathBuf {
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../epithema-tools/tests/fixtures/needle_query.fasta")
    }

    fn needle_target_fixture() -> std::path::PathBuf {
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../epithema-tools/tests/fixtures/needle_target.fasta")
    }

    fn water_query_fixture() -> std::path::PathBuf {
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../epithema-tools/tests/fixtures/water_query.fasta")
    }

    fn water_target_fixture() -> std::path::PathBuf {
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../epithema-tools/tests/fixtures/water_target.fasta")
    }

    fn merger_left_fixture() -> std::path::PathBuf {
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../epithema-tools/tests/fixtures/merger_left.fasta")
    }

    fn merger_right_fixture() -> std::path::PathBuf {
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../epithema-tools/tests/fixtures/merger_right.fasta")
    }

    fn sizeseq_fixture() -> std::path::PathBuf {
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../epithema-tools/tests/fixtures/sizeseq_records.fasta")
    }

    fn biosed_fixture() -> std::path::PathBuf {
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../epithema-tools/tests/fixtures/biosed_records.fasta")
    }

    fn msbar_fixture() -> std::path::PathBuf {
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../epithema-tools/tests/fixtures/msbar_records.fasta")
    }

    fn trimest_fixture() -> std::path::PathBuf {
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../epithema-tools/tests/fixtures/trimest_records.fasta")
    }

    fn vectorstrip_records_fixture() -> std::path::PathBuf {
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../epithema-tools/tests/fixtures/vectorstrip_records.fasta")
    }

    fn vectorstrip_vector_fixture() -> std::path::PathBuf {
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../epithema-tools/tests/fixtures/vectorstrip_vector.fasta")
    }

    fn ambiguous_nucleotide_fixture() -> std::path::PathBuf {
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../epithema-tools/tests/fixtures/ambiguous_nucleotide_records.fasta")
    }

    fn ambiguous_protein_fixture() -> std::path::PathBuf {
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../epithema-tools/tests/fixtures/ambiguous_protein_records.fasta")
    }

    fn pasteseq_main_fixture() -> std::path::PathBuf {
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../epithema-tools/tests/fixtures/pasteseq_main.fasta")
    }

    fn pasteseq_insert_fixture() -> std::path::PathBuf {
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../epithema-tools/tests/fixtures/pasteseq_insert.fasta")
    }

    fn diffseq_left_fixture() -> std::path::PathBuf {
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../epithema-tools/tests/fixtures/diffseq_left.fasta")
    }

    fn diffseq_right_fixture() -> std::path::PathBuf {
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../epithema-tools/tests/fixtures/diffseq_right.fasta")
    }

    fn edialign_fixture() -> std::path::PathBuf {
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../epithema-tools/tests/fixtures/edialign_records.fasta")
    }

    fn needleall_queries_fixture() -> std::path::PathBuf {
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../epithema-tools/tests/fixtures/needleall_queries.fasta")
    }

    fn needleall_targets_fixture() -> std::path::PathBuf {
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../epithema-tools/tests/fixtures/needleall_targets.fasta")
    }

    fn checktrans_nucleotide_fixture() -> std::path::PathBuf {
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../epithema-tools/tests/fixtures/checktrans_nucleotide.fasta")
    }

    fn checktrans_protein_fixture() -> std::path::PathBuf {
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../epithema-tools/tests/fixtures/checktrans_protein.fasta")
    }

    fn checktrans_mismatch_fixture() -> std::path::PathBuf {
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../epithema-tools/tests/fixtures/checktrans_mismatch.fasta")
    }

    fn checktrans_invalid_codon_fixture() -> std::path::PathBuf {
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../epithema-tools/tests/fixtures/checktrans_invalid_codon.fasta")
    }

    fn getorf_fixture() -> std::path::PathBuf {
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../epithema-tools/tests/fixtures/getorf_records.fasta")
    }

    fn tranalign_protein_alignment_fixture() -> std::path::PathBuf {
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../epithema-tools/tests/fixtures/tranalign_protein_alignment.sto")
    }

    fn preg_fixture() -> std::path::PathBuf {
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../epithema-tools/tests/fixtures/preg_records.fasta")
    }

    fn patmatdb_records_fixture() -> std::path::PathBuf {
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../epithema-tools/tests/fixtures/patmatdb_records.fasta")
    }

    fn patmatdb_motifs_fixture() -> std::path::PathBuf {
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../epithema-tools/tests/fixtures/patmatdb_motifs.tsv")
    }

    fn wordmatch_query_fixture() -> std::path::PathBuf {
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../epithema-tools/tests/fixtures/wordmatch_query.fasta")
    }

    fn wordmatch_target_fixture() -> std::path::PathBuf {
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../epithema-tools/tests/fixtures/wordmatch_target.fasta")
    }

    fn wordfinder_targets_fixture() -> std::path::PathBuf {
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../epithema-tools/tests/fixtures/wordfinder_targets.fasta")
    }

    fn dreg_fixture() -> std::path::PathBuf {
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../epithema-tools/tests/fixtures/dreg_records.fasta")
    }

    fn palindrome_fixture() -> std::path::PathBuf {
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../epithema-tools/tests/fixtures/palindrome_records.fasta")
    }

    fn einverted_fixture() -> std::path::PathBuf {
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../epithema-tools/tests/fixtures/einverted_records.fasta")
    }

    fn seqmatchall_fixture() -> std::path::PathBuf {
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../epithema-tools/tests/fixtures/seqmatchall_records.fasta")
    }

    fn implemented_service() -> EpithemaService {
        let mut registry = ServiceRegistry::new();
        for descriptor in governed_tool_descriptors() {
            registry
                .register(*descriptor)
                .expect("tool registration should succeed");
        }
        EpithemaService::new(registry)
    }

    fn temp_service_output_root(label: &str) -> std::path::PathBuf {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("system time should be after unix epoch")
            .as_nanos();
        std::env::temp_dir().join(format!(
            "epithema-service-{label}-{}-{nanos}",
            std::process::id()
        ))
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
    fn resolves_seqretsetall_against_two_local_fixtures_in_stable_order() {
        let service = implemented_service();
        let raw_inputs = vec![
            sequence_fixture().display().to_string(),
            second_sequence_fixture().display().to_string(),
        ];

        let (outcome, provenance, diagnostics) = service
            .resolve_seqretsetall_inputs_with_client::<MockHttpClient>(&raw_inputs, None)
            .expect("seqretsetall core should resolve local fixtures");

        assert_eq!(outcome.record_sets.len(), 2);
        assert_eq!(outcome.record_sets[0].len(), 3);
        assert_eq!(outcome.record_sets[1].len(), 2);
        assert_eq!(outcome.total_records, 5);
        assert_eq!(outcome.record_sets[0][0].identifier().accession(), "alpha");
        assert_eq!(outcome.record_sets[1][0].identifier().accession(), "delta");
        assert_eq!(provenance.len(), 2);
        assert_eq!(diagnostics.len(), 2);
        assert_eq!(diagnostics[0].code(), Some("service.input.local.resolved"));
        assert_eq!(diagnostics[1].code(), Some("service.input.local.resolved"));
    }

    #[test]
    fn resolves_seqretsetall_against_mixed_local_and_provider_inputs() {
        let service = implemented_service();
        let raw_inputs = vec![
            sequence_fixture().display().to_string(),
            "ena:AB000263".to_owned(),
        ];
        let client = MockHttpClient::default().with_response(
            "https://www.ebi.ac.uk/ena/browser/api/fasta/AB000263",
            HttpResponse::new(200, ">AB000263 example\nACGT\n"),
        );

        let (outcome, provenance, diagnostics) = service
            .resolve_seqretsetall_inputs_with_client(&raw_inputs, Some(&client))
            .expect("seqretsetall core should resolve mixed inputs");

        assert_eq!(outcome.record_sets.len(), 2);
        assert_eq!(outcome.record_sets[0].len(), 3);
        assert_eq!(outcome.record_sets[1].len(), 1);
        assert_eq!(
            outcome.record_sets[1][0].identifier().accession(),
            "AB000263"
        );
        assert_eq!(
            outcome.record_sets[1][0].metadata().source.as_deref(),
            Some("ena")
        );
        assert_eq!(provenance.len(), 3);
        assert_eq!(diagnostics.len(), 2);
        assert_eq!(diagnostics[0].code(), Some("service.input.local.resolved"));
        assert_eq!(
            diagnostics[1].code(),
            Some("service.input.provider_qualified")
        );
    }

    #[test]
    fn resolves_seqretsplit_against_local_fixture_into_deterministic_output_files() {
        let service = implemented_service();

        let (outcome, provenance, diagnostics) = service
            .resolve_seqretsplit_input_with_client::<MockHttpClient>(
                &sequence_fixture().display().to_string(),
                None,
            )
            .expect("seqretsplit core should resolve local fixture");

        assert_eq!(outcome.outputs.len(), 3);
        assert_eq!(outcome.total_records, 3);
        assert_eq!(outcome.outputs[0].file_name, "three_records__alpha.fasta");
        assert_eq!(outcome.outputs[1].file_name, "three_records__beta.fasta");
        assert_eq!(outcome.outputs[2].file_name, "three_records__gamma.fasta");
        assert_eq!(provenance.len(), 1);
        assert_eq!(diagnostics.len(), 1);
        assert_eq!(diagnostics[0].code(), Some("service.input.local.resolved"));
    }

    #[test]
    fn resolves_seqretsplit_against_provider_input_with_mocked_client() {
        let service = implemented_service();
        let client = MockHttpClient::default().with_response(
            "https://www.ebi.ac.uk/ena/browser/api/fasta/AB000263",
            HttpResponse::new(200, ">AB000263 example\nACGT\n"),
        );

        let (outcome, provenance, diagnostics) = service
            .resolve_seqretsplit_input_with_client("ena:AB000263", Some(&client))
            .expect("seqretsplit core should resolve provider input");

        assert_eq!(outcome.outputs.len(), 1);
        assert_eq!(outcome.outputs[0].file_name, "ena_AB000263.fasta");
        assert_eq!(
            outcome.outputs[0].record.identifier().accession(),
            "AB000263"
        );
        assert_eq!(
            outcome.outputs[0].record.metadata().source.as_deref(),
            Some("ena")
        );
        assert_eq!(provenance.len(), 2);
        assert_eq!(diagnostics.len(), 1);
        assert_eq!(
            diagnostics[0].code(),
            Some("service.input.provider_qualified")
        );
    }

    #[test]
    fn resolves_infoassembly_against_mocked_ena_archive_metadata() {
        let service = implemented_service();
        let client = MockHttpClient::default().with_response(
            "https://www.ebi.ac.uk/ena/portal/api/filereport?accession=ERR123456&result=read_run&fields=run_accession%2Cstudy_accession%2Cexperiment_accession%2Csample_accession%2Cinstrument_platform%2Cinstrument_model%2Clibrary_layout%2Clibrary_strategy%2Clibrary_source%2Cfastq_ftp%2Cfastq_md5%2Cfastq_bytes%2Csubmitted_ftp%2Csubmitted_md5%2Csubmitted_bytes%2Csra_ftp%2Csra_md5%2Csra_bytes&format=tsv&download=false",
            HttpResponse::new(200, "run_accession\tstudy_accession\texperiment_accession\tsample_accession\tinstrument_platform\tinstrument_model\tlibrary_layout\tlibrary_strategy\tlibrary_source\tfastq_ftp\tfastq_md5\tfastq_bytes\tsubmitted_ftp\tsubmitted_md5\tsubmitted_bytes\tsra_ftp\tsra_md5\tsra_bytes\nERR123456\tERP000001\tERX000001\tERS000001\tILLUMINA\tNovaSeq 6000\tPAIRED\tWGS\tGENOMIC\tftp.sra.ebi.ac.uk/vol1/fastq/ERR123/ERR123456/ERR123456_1.fastq.gz;ftp.sra.ebi.ac.uk/vol1/fastq/ERR123/ERR123456/ERR123456_2.fastq.gz\tmd51;md52\t10;12\t\t\t\t\t\t\n"),
        );

        let (outcome, provenance, diagnostics) = service
            .resolve_infoassembly_with_client("ena:ERR123456", Some(&client))
            .expect("infoassembly core should resolve ENA metadata");

        assert_eq!(outcome.provider, "ena");
        assert_eq!(outcome.accession, "ERR123456");
        assert_eq!(outcome.assembly_accession, "ERP000001");
        assert_eq!(outcome.file_count, 2);
        assert_eq!(outcome.total_size_bytes, Some(22));
        assert_eq!(outcome.route_endpoint, "ena.portal.filereport");
        assert_eq!(provenance.len(), 2);
        assert_eq!(diagnostics.len(), 1);
        assert_eq!(
            diagnostics[0].code(),
            Some("service.input.provider_qualified")
        );
    }

    #[test]
    fn resolves_infoassembly_against_mocked_sra_archive_metadata() {
        let service = implemented_service();
        let client = MockHttpClient::default().with_response(
            "https://trace.ncbi.nlm.nih.gov/Traces/sra-db-be/runinfo?acc=SRR123456",
            HttpResponse::new(200, "Run,ReleaseDate,LoadDate,spots,bases,spots_with_mates,avgLength,size_MB,AssemblyName,download_path,Experiment,LibraryName,LibraryStrategy,LibrarySelection,LibrarySource,LibraryLayout,InsertSize,InsertDev,Platform,Model,SRAStudy,BioProject,Study_Pubmed_id,ProjectID,Sample,BioSample,SampleType,TaxID,ScientificName,SampleName,CenterName,Submission,dbgap_study_accession,Consent,RunHash,ReadHash\nSRR123456,2024-01-01,2024-01-02,1,100,1,100,1,,https://example.invalid/SRR123456,SRX123456,,WGS,,GENOMIC,PAIRED,,,ILLUMINA,NextSeq 2000,SRP000001,PRJNA1,,1,SRS123456,SAMN1,,9606,Homo sapiens,,NCBI,SRA000001,,,runhash,readhash\n"),
        );

        let (outcome, provenance, diagnostics) = service
            .resolve_infoassembly_with_client("sra:SRR123456", Some(&client))
            .expect("infoassembly core should resolve SRA metadata");

        assert_eq!(outcome.provider, "sra");
        assert_eq!(outcome.accession, "SRR123456");
        assert_eq!(outcome.assembly_accession, "SRP000001");
        assert_eq!(outcome.file_count, 0);
        assert_eq!(outcome.total_size_bytes, None);
        assert_eq!(outcome.route_endpoint, "sra.runinfo");
        assert_eq!(provenance.len(), 2);
        assert_eq!(diagnostics.len(), 1);
        assert_eq!(
            diagnostics[0].code(),
            Some("service.input.provider_qualified")
        );
    }

    #[test]
    fn invokes_seqretsplit_with_local_fixture_into_partitioned_payload() {
        let service = implemented_service();
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("seqretsplit").expect("tool name should be valid"),
        )
        .with_arguments(vec![sequence_fixture().display().to_string()]);

        let response = service
            .invoke_seqretsplit_with_client(
                request,
                epithema_tools::retrieval_tools::SEQRETSPLIT_DESCRIPTOR,
                None::<&MockHttpClient>,
            )
            .expect("seqretsplit surface should execute");

        match &response.result.payload {
            ResultPayload::SequencePartitions(partitions) => {
                assert_eq!(partitions.len(), 3);
                assert_eq!(partitions[0].len(), 1);
                assert_eq!(partitions[0][0].identifier().accession(), "alpha");
            }
            payload => panic!("unexpected payload: {payload:?}"),
        }
        assert_eq!(
            response.result.summary.title,
            "Sequence retrieval split normalization completed"
        );
        assert_eq!(response.result.artifacts.len(), 4);
        assert_eq!(
            response.result.artifacts[1].label.as_deref(),
            Some("three_records__alpha.fasta")
        );
    }

    #[test]
    fn invokes_seqretsplit_with_provider_input_into_partitioned_payload() {
        let service = implemented_service();
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("seqretsplit").expect("tool name should be valid"),
        )
        .with_arguments(vec!["ena:AB000263".to_owned()]);
        let client = MockHttpClient::default().with_response(
            "https://www.ebi.ac.uk/ena/browser/api/fasta/AB000263",
            HttpResponse::new(200, ">AB000263 example\nACGT\n"),
        );

        let response = service
            .invoke_seqretsplit_with_client(
                request,
                epithema_tools::retrieval_tools::SEQRETSPLIT_DESCRIPTOR,
                Some(&client),
            )
            .expect("seqretsplit surface should execute");

        match &response.result.payload {
            ResultPayload::SequencePartitions(partitions) => {
                assert_eq!(partitions.len(), 1);
                assert_eq!(partitions[0][0].identifier().accession(), "AB000263");
            }
            payload => panic!("unexpected payload: {payload:?}"),
        }
        assert_eq!(response.result.artifacts.len(), 2);
        assert_eq!(
            response.result.artifacts[1].label.as_deref(),
            Some("ena_AB000263.fasta")
        );
        assert_eq!(response.report.provenance().len(), 3);
    }

    #[test]
    fn dispatches_seqretsplit_through_the_governed_service_surface() {
        let service = implemented_service();
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("seqretsplit").expect("tool name should be valid"),
        )
        .with_arguments(vec![sequence_fixture().display().to_string()]);

        let response = service
            .invoke(request)
            .expect("seqretsplit should dispatch through the governed service");

        assert_eq!(response.tool.as_str(), "seqretsplit");
        match &response.result.payload {
            ResultPayload::SequencePartitions(partitions) => {
                assert_eq!(partitions.len(), 3);
                assert_eq!(partitions[0].len(), 1);
                assert_eq!(partitions[0][0].identifier().accession(), "alpha");
            }
            payload => panic!("unexpected payload: {payload:?}"),
        }
    }

    #[test]
    fn invokes_seqretsetall_with_local_fixtures_into_partitioned_payload() {
        let service = implemented_service();
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("seqretsetall").expect("tool name should be valid"),
        )
        .with_arguments(vec![
            sequence_fixture().display().to_string(),
            second_sequence_fixture().display().to_string(),
        ]);

        let response = service
            .invoke_seqretsetall_with_client(
                request,
                epithema_tools::retrieval_tools::SEQRETSETALL_DESCRIPTOR,
                None::<&MockHttpClient>,
            )
            .expect("seqretsetall surface should execute");

        match &response.result.payload {
            ResultPayload::SequencePartitions(partitions) => {
                assert_eq!(partitions.len(), 2);
                assert_eq!(partitions[0].len(), 3);
                assert_eq!(partitions[1].len(), 2);
            }
            payload => panic!("unexpected payload: {payload:?}"),
        }
        assert_eq!(
            response.result.summary.title,
            "Sequence retrieval set normalization completed"
        );
        assert_eq!(response.result.artifacts.len(), 1);
    }

    #[test]
    fn dispatches_seqretsetall_through_the_governed_service_surface() {
        let service = implemented_service();
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("seqretsetall").expect("tool name should be valid"),
        )
        .with_arguments(vec![
            sequence_fixture().display().to_string(),
            second_sequence_fixture().display().to_string(),
        ]);

        let response = service
            .invoke(request)
            .expect("seqretsetall should dispatch through the governed service");

        assert_eq!(response.tool.as_str(), "seqretsetall");
        match &response.result.payload {
            ResultPayload::SequencePartitions(partitions) => {
                assert_eq!(partitions.len(), 2);
                assert_eq!(partitions[0].len(), 3);
                assert_eq!(partitions[1].len(), 2);
            }
            payload => panic!("unexpected payload: {payload:?}"),
        }
    }

    #[test]
    fn invokes_seqretsetall_with_mixed_inputs_into_partitioned_payload() {
        let service = implemented_service();
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("seqretsetall").expect("tool name should be valid"),
        )
        .with_arguments(vec![
            sequence_fixture().display().to_string(),
            "ena:AB000263".to_owned(),
        ]);
        let client = MockHttpClient::default().with_response(
            "https://www.ebi.ac.uk/ena/browser/api/fasta/AB000263",
            HttpResponse::new(200, ">AB000263 example\nACGT\n"),
        );

        let response = service
            .invoke_seqretsetall_with_client(
                request,
                epithema_tools::retrieval_tools::SEQRETSETALL_DESCRIPTOR,
                Some(&client),
            )
            .expect("seqretsetall surface should execute");

        match &response.result.payload {
            ResultPayload::SequencePartitions(partitions) => {
                assert_eq!(partitions.len(), 2);
                assert_eq!(partitions[0].len(), 3);
                assert_eq!(partitions[1].len(), 1);
                assert_eq!(partitions[1][0].identifier().accession(), "AB000263");
            }
            payload => panic!("unexpected payload: {payload:?}"),
        }
        assert_eq!(response.report.provenance().len(), 4);
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
            .invoke_refseqget_with_client(request, descriptor, Some(&client))
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
    fn dispatches_whichdb_through_the_governed_service_surface() {
        let service = implemented_service();
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("whichdb").expect("tool name should be valid"),
        )
        .with_arguments(vec!["ena:AB000263".to_owned()]);

        let response = service
            .invoke(request)
            .expect("whichdb should dispatch through the governed service");

        assert_eq!(response.tool.as_str(), "whichdb");
        match &response.result.payload {
            ResultPayload::TableReport(table) => {
                assert_eq!(
                    table.columns,
                    vec![
                        "provider",
                        "normalized_query",
                        "route_label",
                        "discovery_status",
                        "next_methods"
                    ]
                );
                assert_eq!(table.rows.len(), 1);
                assert_eq!(table.rows[0][0], "ena");
                assert_eq!(table.rows[0][1], "AB000263");
                assert_eq!(table.rows[0][2], "ena.sequence-or-archive-discovery");
                assert_eq!(table.rows[0][3], "supported_provider");
                assert_eq!(table.rows[0][4], "seqret,runinfo,runget,infoassembly");
            }
            payload => panic!("unexpected payload: {payload:?}"),
        }
        assert_eq!(
            response.result.summary.lines[2],
            "Discovery policy: bounded provider-qualified reporting only"
        );
        assert_eq!(
            response.result.summary.lines[3],
            "Retrieval policy: no live lookup or payload retrieval is performed"
        );
    }

    #[test]
    fn whichdb_reports_unsupported_provider_without_fallback() {
        let service = implemented_service();
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("whichdb").expect("tool name should be valid"),
        )
        .with_arguments(vec!["uniprot:P12345".to_owned()]);

        let response = service
            .invoke(request)
            .expect("whichdb should report unsupported provider scope");

        match &response.result.payload {
            ResultPayload::TableReport(table) => {
                assert_eq!(table.rows.len(), 1);
                assert_eq!(table.rows[0][0], "uniprot");
                assert_eq!(table.rows[0][2], "unsupported-provider");
                assert_eq!(table.rows[0][3], "unsupported_provider");
                assert_eq!(table.rows[0][4], "");
            }
            payload => panic!("unexpected payload: {payload:?}"),
        }
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
            .invoke_runinfo_with_client(request, descriptor, Some(&client))
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
            .invoke_runinfo_with_client(request, descriptor, Some(&client))
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
            .invoke_runget_with_client(request, descriptor, Some(&client))
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
    fn executes_ngslist_against_mocked_ena_study_manifest() {
        let service = implemented_service();
        let tool = ToolName::new("ngslist").expect("tool name should be valid");
        let descriptor = service
            .registry()
            .find(&tool)
            .copied()
            .expect("ngslist should be registered");
        let request =
            InvocationRequest::new(ExecutionContext::default(), tool).with_arguments(vec![
                "PRJNA1011899".to_owned(),
                "--provider".to_owned(),
                "ena".to_owned(),
            ]);
        let query = NgsQuery::classify("ena:PRJNA1011899").expect("query should classify");
        let provider_request = EnaNgsAdapter::new()
            .build_manifest_request(&query)
            .expect("ENA NGS request should build");
        let body = concat!(
            "run_accession\tstudy_accession\tsecondary_study_accession\texperiment_accession\tsample_accession\tsecondary_sample_accession\tstudy_title\tsample_title\texperiment_title\tscientific_name\tinstrument_platform\tinstrument_model\tlibrary_strategy\tlibrary_source\tlibrary_selection\tlibrary_layout\tfastq_ftp\tfastq_md5\tfastq_bytes\tsubmitted_ftp\tsubmitted_md5\tsubmitted_bytes\tsra_ftp\tsra_md5\tsra_bytes\n",
            "ERR1\tERP1\tPRJNA1011899\tERX1\tERS1\tSAMN1\tStudy title\tSample one\tExperiment one\tHomo sapiens\tILLUMINA\tNovaSeq 6000\tWGS\tGENOMIC\tRANDOM\tPAIRED\tftp.sra.ebi.ac.uk/vol1/fastq/ERR1/ERR1_1.fastq.gz;ftp.sra.ebi.ac.uk/vol1/fastq/ERR1/ERR1_2.fastq.gz\tmd51;md52\t10;12\tftp.sra.ebi.ac.uk/vol1/submitted/ERR1/reads.pod5\tmd5raw\t20\t\t\t\n",
            "ERR2\tERP1\tPRJNA1011899\tERX2\tERS2\tSAMN2\tStudy title\tSample two\tExperiment two\tHomo sapiens\tOXFORD_NANOPORE\tPromethION\tRNA-Seq\tTRANSCRIPTOMIC\tcDNA\tSINGLE\tftp.sra.ebi.ac.uk/vol1/fastq/ERR2/ERR2.fastq.gz\tmd53\t14\tftp.sra.ebi.ac.uk/vol1/submitted/ERR2/alignment.bam\tmd5bam\t40\t\t\t\n"
        );
        let client = MockHttpClient::default()
            .with_response(provider_request.url, HttpResponse::new(200, body));

        let response = service
            .invoke_ngslist_with_client(request, descriptor, Some(&client))
            .expect("ngslist should execute with mocked ENA study manifest");

        match &response.result.payload {
            ResultPayload::TableReport(table) => {
                assert_eq!(
                    table.columns,
                    vec![
                        "provider",
                        "query_accession",
                        "query_object_class",
                        "study_accession",
                        "study_title",
                        "sample_accession",
                        "sample_title",
                        "experiment_accession",
                        "run_accession",
                        "instrument_platform",
                        "instrument_model",
                        "library_strategy",
                        "library_source",
                        "library_selection",
                        "library_layout",
                        "asset_role",
                        "asset_format",
                        "source_url",
                        "size_bytes",
                        "checksum_md5",
                    ]
                );
                assert_eq!(table.rows.len(), 5);
                assert_eq!(table.rows[0][0], "ena");
                assert_eq!(table.rows[0][1], "PRJNA1011899");
                assert_eq!(table.rows[0][2], "study");
                assert_eq!(table.rows[0][3], "ERP1");
                assert_eq!(table.rows[0][15], "generated_fastq");
                assert_eq!(table.rows[2][15], "submitted_raw");
                assert_eq!(table.rows[4][15], "submitted_alignment");
            }
            payload => panic!("unexpected payload: {payload:?}"),
        }
        assert!(
            response
                .result
                .summary
                .lines
                .iter()
                .any(|line| line == "Runs: 2")
        );
        assert!(
            response
                .result
                .summary
                .lines
                .iter()
                .any(|line| line == "Assets: 5")
        );
    }

    #[test]
    fn executes_ngslist_json_against_mocked_ena_run_manifest() {
        let service = implemented_service();
        let tool = ToolName::new("ngslist").expect("tool name should be valid");
        let descriptor = service
            .registry()
            .find(&tool)
            .copied()
            .expect("ngslist should be registered");
        let request =
            InvocationRequest::new(ExecutionContext::default(), tool).with_arguments(vec![
                "ena:ERR123456".to_owned(),
                "--format".to_owned(),
                "json".to_owned(),
            ]);
        let query = NgsQuery::classify("ena:ERR123456").expect("query should classify");
        let provider_request = EnaNgsAdapter::new()
            .build_manifest_request(&query)
            .expect("ENA NGS request should build");
        let body = concat!(
            "run_accession\tstudy_accession\tsecondary_study_accession\texperiment_accession\tsample_accession\tsecondary_sample_accession\tstudy_title\tsample_title\texperiment_title\tscientific_name\tinstrument_platform\tinstrument_model\tlibrary_strategy\tlibrary_source\tlibrary_selection\tlibrary_layout\tfastq_ftp\tfastq_md5\tfastq_bytes\tsubmitted_ftp\tsubmitted_md5\tsubmitted_bytes\tsra_ftp\tsra_md5\tsra_bytes\n",
            "ERR123456\tERP1\tPRJNA1\tERX1\tERS1\tSAMN1\tStudy title\tSample one\tExperiment one\tHomo sapiens\tILLUMINA\tNovaSeq 6000\tWGS\tGENOMIC\tRANDOM\tPAIRED\tftp.sra.ebi.ac.uk/vol1/fastq/ERR123/ERR123456/ERR123456.fastq.gz\tmd51\t10\t\t\t\t\t\t\n"
        );
        let client = MockHttpClient::default()
            .with_response(provider_request.url, HttpResponse::new(200, body));

        let response = service
            .invoke_ngslist_with_client(request, descriptor, Some(&client))
            .expect("ngslist JSON should execute with mocked ENA run manifest");

        match &response.result.payload {
            ResultPayload::TextReport(report) => {
                assert!(report.body.contains("\"schema\": \"epithema.ngslist/v1\""));
                assert!(report.body.contains("\"query_accession\": \"ERR123456\""));
                assert!(report.body.contains("\"asset_role\": \"generated_fastq\""));
                assert!(report.body.contains("\"asset_count\": 1"));
            }
            payload => panic!("unexpected payload: {payload:?}"),
        }
        assert!(
            response
                .result
                .summary
                .lines
                .iter()
                .any(|line| line == "Format: json")
        );
    }

    #[test]
    fn executes_ngsget_with_existing_download_check() {
        let service = implemented_service();
        let tool = ToolName::new("ngsget").expect("tool name should be valid");
        let descriptor = service
            .registry()
            .find(&tool)
            .copied()
            .expect("ngsget should be registered");
        let output_root = temp_service_output_root("ngsget-output");
        let cache_root = temp_service_output_root("ngsget-cache");
        let cached_fastq = cache_root.join("nested/ERR123456.fastq.gz");
        fs::create_dir_all(
            cached_fastq
                .parent()
                .expect("cached file should have parent"),
        )
        .expect("cache directory should be created");
        let body = b"ACGT\n".to_vec();
        let checksum = format!("{:x}", md5::compute(&body));
        fs::write(&cached_fastq, &body).expect("cached FASTQ should be written");
        let request =
            InvocationRequest::new(ExecutionContext::default(), tool).with_arguments(vec![
                "ena:ERR123456".to_owned(),
                "--out".to_owned(),
                output_root.display().to_string(),
                "--check-downloads".to_owned(),
                cache_root.display().to_string(),
            ]);
        let query = NgsQuery::classify("ena:ERR123456").expect("query should classify");
        let provider_request = EnaNgsAdapter::new()
            .build_manifest_request(&query)
            .expect("ENA NGS request should build");
        let manifest_body = format!(
            "{}\n{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\n",
            "run_accession\tstudy_accession\tsecondary_study_accession\texperiment_accession\tsample_accession\tsecondary_sample_accession\tstudy_title\tsample_title\texperiment_title\tscientific_name\tinstrument_platform\tinstrument_model\tlibrary_strategy\tlibrary_source\tlibrary_selection\tlibrary_layout\tfastq_ftp\tfastq_md5\tfastq_bytes\tsubmitted_ftp\tsubmitted_md5\tsubmitted_bytes\tsra_ftp\tsra_md5\tsra_bytes",
            "ERR123456",
            "ERP1",
            "PRJNA1",
            "ERX1",
            "ERS1",
            "SAMN1",
            "Study title",
            "Sample one",
            "Experiment one",
            "Homo sapiens",
            "ILLUMINA",
            "NovaSeq 6000",
            "WGS",
            "GENOMIC",
            "RANDOM",
            "PAIRED",
            "example.invalid/ERR123456.fastq.gz",
            checksum,
            body.len(),
            "",
            "",
            "",
            "",
            "",
            "",
        );
        let client = MockHttpClient::default()
            .with_response(provider_request.url, HttpResponse::new(200, manifest_body));

        let response = service
            .invoke_ngsget_with_client(request, descriptor, Some(&client))
            .expect("ngsget should execute with mocked ENA manifest and local cache");

        match &response.result.payload {
            ResultPayload::TextReport(report) => {
                assert!(report.body.contains("failed_records\t0"));
                assert!(report.body.contains("selected_assets\t1"));
            }
            payload => panic!("unexpected payload: {payload:?}"),
        }
        let materialized = output_root.join("runs/ERR123456/fastq/ERR123456.fastq.gz");
        assert_eq!(
            fs::read(&materialized).expect("materialized FASTQ should be readable"),
            body
        );
        assert_eq!(
            fs::read(&cached_fastq).expect("cached FASTQ should remain readable"),
            b"ACGT\n".to_vec()
        );
        assert!(output_root.join("manifest.tsv").exists());
        assert!(output_root.join("provenance.json").exists());
        assert_eq!(response.result.artifacts.len(), 2);
        fs::remove_dir_all(output_root).ok();
        fs::remove_dir_all(cache_root).ok();
    }

    #[test]
    fn invokes_infoassembly_with_mocked_ena_metadata_into_table_payload() {
        let service = implemented_service();
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("infoassembly").expect("tool name should be valid"),
        )
        .with_arguments(vec!["ena:ERR123456".to_owned()]);
        let client = MockHttpClient::default().with_response(
            "https://www.ebi.ac.uk/ena/portal/api/filereport?accession=ERR123456&result=read_run&fields=run_accession%2Cstudy_accession%2Cexperiment_accession%2Csample_accession%2Cinstrument_platform%2Cinstrument_model%2Clibrary_layout%2Clibrary_strategy%2Clibrary_source%2Cfastq_ftp%2Cfastq_md5%2Cfastq_bytes%2Csubmitted_ftp%2Csubmitted_md5%2Csubmitted_bytes%2Csra_ftp%2Csra_md5%2Csra_bytes&format=tsv&download=false",
            HttpResponse::new(200, "run_accession\tstudy_accession\texperiment_accession\tsample_accession\tinstrument_platform\tinstrument_model\tlibrary_layout\tlibrary_strategy\tlibrary_source\tfastq_ftp\tfastq_md5\tfastq_bytes\tsubmitted_ftp\tsubmitted_md5\tsubmitted_bytes\tsra_ftp\tsra_md5\tsra_bytes\nERR123456\tERP000001\tERX000001\tERS000001\tILLUMINA\tNovaSeq 6000\tPAIRED\tWGS\tGENOMIC\tftp.sra.ebi.ac.uk/vol1/fastq/ERR123/ERR123456/ERR123456_1.fastq.gz;ftp.sra.ebi.ac.uk/vol1/fastq/ERR123/ERR123456/ERR123456_2.fastq.gz\tmd51;md52\t10;12\t\t\t\t\t\t\n"),
        );

        let response = service
            .invoke_infoassembly_with_client(
                request,
                epithema_tools::archive_tools::INFOASSEMBLY_DESCRIPTOR,
                Some(&client),
            )
            .expect("infoassembly surface should execute");

        match &response.result.payload {
            ResultPayload::TableReport(table) => {
                assert_eq!(table.columns, vec!["field", "value"]);
                assert!(table.rows.iter().any(|row| {
                    row == &vec!["assembly_accession".to_owned(), "ERP000001".to_owned()]
                }));
                assert!(
                    table
                        .rows
                        .iter()
                        .any(|row| { row == &vec!["file_count".to_owned(), "2".to_owned()] })
                );
            }
            payload => panic!("unexpected payload: {payload:?}"),
        }
        assert_eq!(response.report.provenance().len(), 2);
    }

    #[test]
    fn invokes_assemblyget_with_mocked_ena_metadata_into_manifest_intent_table() {
        let service = implemented_service();
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("assemblyget").expect("tool name should be valid"),
        )
        .with_arguments(vec!["ena:ERR123456".to_owned()]);
        let client = MockHttpClient::default().with_response(
            "https://www.ebi.ac.uk/ena/portal/api/filereport?accession=ERR123456&result=read_run&fields=run_accession%2Cstudy_accession%2Cexperiment_accession%2Csample_accession%2Cinstrument_platform%2Cinstrument_model%2Clibrary_layout%2Clibrary_strategy%2Clibrary_source%2Cfastq_ftp%2Cfastq_md5%2Cfastq_bytes%2Csubmitted_ftp%2Csubmitted_md5%2Csubmitted_bytes%2Csra_ftp%2Csra_md5%2Csra_bytes&format=tsv&download=false",
            HttpResponse::new(200, "run_accession\tstudy_accession\texperiment_accession\tsample_accession\tinstrument_platform\tinstrument_model\tlibrary_layout\tlibrary_strategy\tlibrary_source\tfastq_ftp\tfastq_md5\tfastq_bytes\tsubmitted_ftp\tsubmitted_md5\tsubmitted_bytes\tsra_ftp\tsra_md5\tsra_bytes\nERR123456\tERP000001\tERX000001\tERS000001\tILLUMINA\tNovaSeq 6000\tPAIRED\tWGS\tGENOMIC\tftp.sra.ebi.ac.uk/vol1/fastq/ERR123/ERR123456/ERR123456_1.fastq.gz;ftp.sra.ebi.ac.uk/vol1/fastq/ERR123/ERR123456/ERR123456_2.fastq.gz\tmd51;md52\t10;12\t\t\t\t\t\t\n"),
        );

        let response = service
            .invoke_assemblyget_with_client(
                request,
                epithema_tools::archive_tools::ASSEMBLYGET_DESCRIPTOR,
                Some(&client),
            )
            .expect("assemblyget surface should execute");

        assert_eq!(response.tool.as_str(), "assemblyget");
        match &response.result.payload {
            ResultPayload::TableReport(table) => {
                assert_eq!(
                    table.columns,
                    vec![
                        "provider",
                        "requested_accession",
                        "object_class",
                        "assembly_accession",
                        "run_accession",
                        "route_endpoint",
                        "manifest_mode",
                        "file_count",
                        "total_size_bytes",
                        "materialization_status"
                    ]
                );
                assert_eq!(table.rows.len(), 1);
                assert_eq!(table.rows[0][0], "ena");
                assert_eq!(table.rows[0][1], "ERR123456");
                assert_eq!(table.rows[0][2], "run");
                assert_eq!(table.rows[0][3], "ERP000001");
                assert_eq!(table.rows[0][4], "ERR123456");
                assert_eq!(table.rows[0][5], "ena.portal.filereport");
                assert_eq!(table.rows[0][6], "manifest_intent_only");
                assert_eq!(table.rows[0][7], "2");
                assert_eq!(table.rows[0][8], "22");
                assert_eq!(table.rows[0][9], "not_materialized");
            }
            payload => panic!("unexpected payload: {payload:?}"),
        }
        assert!(
            response
                .result
                .summary
                .lines
                .iter()
                .any(|line| line == "Materialization: not_materialized")
        );
        assert!(response.result.summary.lines.iter().any(
            |line| line == "Acquisition policy: manifest intent only; no files are downloaded"
        ));
        assert_eq!(response.report.provenance().len(), 3);
    }

    #[test]
    fn assemblyget_rejects_local_file_inputs_through_service_surface() {
        let service = implemented_service();
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("assemblyget").expect("tool name should be valid"),
        )
        .with_arguments(vec![sequence_fixture().display().to_string()]);

        let error = service
            .invoke(request)
            .expect_err("assemblyget should reject local file inputs");

        assert_eq!(
            error.code(),
            Some("service.assemblyget.local_input_not_supported")
        );
    }

    #[test]
    fn invokes_infoassembly_with_mocked_sra_metadata_into_table_payload() {
        let service = implemented_service();
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("infoassembly").expect("tool name should be valid"),
        )
        .with_arguments(vec!["sra:SRR123456".to_owned()]);
        let client = MockHttpClient::default().with_response(
            "https://trace.ncbi.nlm.nih.gov/Traces/sra-db-be/runinfo?acc=SRR123456",
            HttpResponse::new(200, "Run,ReleaseDate,LoadDate,spots,bases,spots_with_mates,avgLength,size_MB,AssemblyName,download_path,Experiment,LibraryName,LibraryStrategy,LibrarySelection,LibrarySource,LibraryLayout,InsertSize,InsertDev,Platform,Model,SRAStudy,BioProject,Study_Pubmed_id,ProjectID,Sample,BioSample,SampleType,TaxID,ScientificName,SampleName,CenterName,Submission,dbgap_study_accession,Consent,RunHash,ReadHash\nSRR123456,2024-01-01,2024-01-02,1,100,1,100,1,,https://example.invalid/SRR123456,SRX123456,,WGS,,GENOMIC,PAIRED,,,ILLUMINA,NextSeq 2000,SRP000001,PRJNA1,,1,SRS123456,SAMN1,,9606,Homo sapiens,,NCBI,SRA000001,,,runhash,readhash\n"),
        );

        let response = service
            .invoke_infoassembly_with_client(
                request,
                epithema_tools::archive_tools::INFOASSEMBLY_DESCRIPTOR,
                Some(&client),
            )
            .expect("infoassembly surface should execute");

        match &response.result.payload {
            ResultPayload::TableReport(table) => {
                assert!(table.rows.iter().any(|row| {
                    row == &vec!["assembly_accession".to_owned(), "SRP000001".to_owned()]
                }));
                assert!(table.rows.iter().any(|row| {
                    row == &vec!["route_endpoint".to_owned(), "sra.runinfo".to_owned()]
                }));
            }
            payload => panic!("unexpected payload: {payload:?}"),
        }
        assert_eq!(response.report.provenance().len(), 2);
    }

    #[test]
    fn dispatches_infoassembly_through_the_governed_service_surface() {
        let service = implemented_service();
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("infoassembly").expect("tool name should be valid"),
        )
        .with_arguments(vec!["ena:ERR123456".to_owned()]);
        let client = MockHttpClient::default().with_response(
            "https://www.ebi.ac.uk/ena/portal/api/filereport?accession=ERR123456&result=read_run&fields=run_accession%2Cstudy_accession%2Cexperiment_accession%2Csample_accession%2Cinstrument_platform%2Cinstrument_model%2Clibrary_layout%2Clibrary_strategy%2Clibrary_source%2Cfastq_ftp%2Cfastq_md5%2Cfastq_bytes%2Csubmitted_ftp%2Csubmitted_md5%2Csubmitted_bytes%2Csra_ftp%2Csra_md5%2Csra_bytes&format=tsv&download=false",
            HttpResponse::new(200, "run_accession\tstudy_accession\texperiment_accession\tsample_accession\tinstrument_platform\tinstrument_model\tlibrary_layout\tlibrary_strategy\tlibrary_source\tfastq_ftp\tfastq_md5\tfastq_bytes\tsubmitted_ftp\tsubmitted_md5\tsubmitted_bytes\tsra_ftp\tsra_md5\tsra_bytes\nERR123456\tERP000001\tERX000001\tERS000001\tILLUMINA\tNovaSeq 6000\tPAIRED\tWGS\tGENOMIC\tftp.sra.ebi.ac.uk/vol1/fastq/ERR123/ERR123456/ERR123456_1.fastq.gz;ftp.sra.ebi.ac.uk/vol1/fastq/ERR123/ERR123456/ERR123456_2.fastq.gz\tmd51;md52\t10;12\t\t\t\t\t\t\n"),
        );

        let response = service
            .invoke_infoassembly_with_client(
                request,
                epithema_tools::archive_tools::INFOASSEMBLY_DESCRIPTOR,
                Some(&client),
            )
            .expect("infoassembly should dispatch through the governed service");

        assert_eq!(response.tool.as_str(), "infoassembly");
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
            .invoke_runget_with_client(request, descriptor, Some(&client))
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
            "epithema-seqcount-malformed-{}.fasta",
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
    fn executes_diffseq_against_real_fixtures() {
        let service = implemented_service();
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("diffseq").expect("tool name should be valid"),
        )
        .with_arguments(vec![
            diffseq_left_fixture().display().to_string(),
            diffseq_right_fixture().display().to_string(),
        ]);

        let response = service.invoke(request).expect("diffseq should execute");
        match &response.result.payload {
            ResultPayload::TableReport(table) => {
                assert_eq!(table.rows.len(), 1);
                assert_eq!(table.rows[0][1], "substitution");
                assert_eq!(table.rows[0][2], "5");
                assert_eq!(table.rows[0][4], "5");
                assert_eq!(table.rows[0][6], "A");
                assert_eq!(table.rows[0][7], "T");
            }
            payload => panic!("unexpected payload: {payload:?}"),
        }
    }

    #[test]
    fn executes_edialign_against_real_fixture() {
        let service = implemented_service();
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("edialign").expect("tool name should be valid"),
        )
        .with_arguments(vec![edialign_fixture().display().to_string()]);

        let response = service.invoke(request).expect("edialign should execute");
        match &response.result.payload {
            ResultPayload::Alignment(alignment) => {
                assert_eq!(alignment.row_count(), 3);
                assert_eq!(alignment.column_count(), 4);
                assert_eq!(alignment.rows()[0].aligned(), "TACG");
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
    fn executes_nthseqset_against_multiple_alignment_fixture() {
        let service = implemented_service();
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("nthseqset").expect("tool name should be valid"),
        )
        .with_arguments(vec![
            nthseqset_fixture().display().to_string(),
            "2".to_owned(),
        ]);

        let response = service.invoke(request).expect("nthseqset should execute");
        match &response.result.payload {
            ResultPayload::Alignment(alignment) => {
                assert_eq!(alignment.row_count(), 2);
                assert_eq!(alignment.column_count(), 5);
                assert_eq!(alignment.rows()[0].identifier().accession(), "gamma");
            }
            payload => panic!("unexpected payload: {payload:?}"),
        }
        assert_eq!(response.result.summary.lines[1], "Selected set: 2");
        assert_eq!(response.result.summary.lines[2], "Total sets: 2");
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
    fn executes_listor_against_real_fixtures() {
        let service = implemented_service();
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("listor").expect("tool name should be valid"),
        )
        .with_arguments(vec![
            listor_first_fixture().display().to_string(),
            listor_second_fixture().display().to_string(),
            "--operator".to_owned(),
            "xor".to_owned(),
        ]);

        let response = service.invoke(request).expect("listor should execute");
        match &response.result.payload {
            ResultPayload::SequenceCollection(records) => {
                let ids: Vec<_> = records
                    .iter()
                    .map(|record| record.identifier().accession().to_owned())
                    .collect();
                assert_eq!(ids, vec!["beta", "delta"]);
            }
            payload => panic!("unexpected payload: {payload:?}"),
        }
        assert_eq!(response.result.summary.lines[2], "Operator: XOR");
    }

    #[test]
    fn executes_skipredundant_against_real_fixture() {
        let service = implemented_service();
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("skipredundant").expect("tool name should be valid"),
        )
        .with_arguments(vec![skipredundant_fixture().display().to_string()]);

        let response = service
            .invoke(request)
            .expect("skipredundant should execute");
        match &response.result.payload {
            ResultPayload::SequenceCollection(records) => {
                let ids: Vec<_> = records
                    .iter()
                    .map(|record| record.identifier().accession().to_owned())
                    .collect();
                assert_eq!(ids, vec!["keep_alpha", "keep_gamma"]);
            }
            payload => panic!("unexpected payload: {payload:?}"),
        }
        assert_eq!(response.result.summary.lines[2], "Removed redundant: 2");
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
    fn executes_makenucseq_with_deterministic_count_and_seed() {
        let service = implemented_service();
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("makenucseq").expect("tool name should be valid"),
        )
        .with_arguments(vec![
            "made_nuc".to_owned(),
            "6".to_owned(),
            "--count".to_owned(),
            "2".to_owned(),
            "--seed".to_owned(),
            "7".to_owned(),
            "--molecule".to_owned(),
            "rna".to_owned(),
        ]);

        let response = service.invoke(request).expect("makenucseq should execute");
        match &response.result.payload {
            ResultPayload::SequenceCollection(records) => {
                assert_eq!(records.len(), 2);
                assert_eq!(records[0].identifier().accession(), "made_nuc_1");
                assert_eq!(records[0].molecule().to_string(), "rna");
            }
            payload => panic!("unexpected payload: {payload:?}"),
        }
        assert_eq!(response.result.summary.lines[3], "Seed: 7");
    }

    #[test]
    fn executes_makeprotseq_with_deterministic_seed() {
        let service = implemented_service();
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("makeprotseq").expect("tool name should be valid"),
        )
        .with_arguments(vec![
            "made_prot".to_owned(),
            "5".to_owned(),
            "--seed".to_owned(),
            "9".to_owned(),
        ]);

        let response = service.invoke(request).expect("makeprotseq should execute");
        match &response.result.payload {
            ResultPayload::SequenceCollection(records) => {
                assert_eq!(records.len(), 1);
                assert_eq!(records[0].identifier().accession(), "made_prot");
                assert_eq!(records[0].molecule().to_string(), "protein");
            }
            payload => panic!("unexpected payload: {payload:?}"),
        }
        assert_eq!(response.result.summary.lines[3], "Seed: 9");
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
        assert!(error.to_string().contains("Usage: epithema union"));
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
    fn executes_merger_against_real_fixture() {
        let service = implemented_service();
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("merger").expect("tool name should be valid"),
        )
        .with_arguments(vec![
            merger_left_fixture().display().to_string(),
            merger_right_fixture().display().to_string(),
        ]);

        let response = service.invoke(request).expect("merger should execute");
        match &response.result.payload {
            ResultPayload::Sequence(record) => {
                assert_eq!(record.identifier().accession(), "left+right");
                assert_eq!(record.residues(), "ACGTAAGGG");
            }
            payload => panic!("unexpected payload: {payload:?}"),
        }
        assert_eq!(response.result.summary.lines[2], "Overlap length: 3");
    }

    #[test]
    fn executes_megamerger_against_real_fixture() {
        let service = implemented_service();
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("megamerger").expect("tool name should be valid"),
        )
        .with_arguments(vec![
            merger_left_fixture().display().to_string(),
            merger_right_fixture().display().to_string(),
        ]);

        let response = service.invoke(request).expect("megamerger should execute");
        match &response.result.payload {
            ResultPayload::Sequence(record) => {
                assert_eq!(record.molecule(), MoleculeKind::Dna);
                assert_eq!(record.residues(), "ACGTAAGGG");
            }
            payload => panic!("unexpected payload: {payload:?}"),
        }
        assert_eq!(
            response.result.summary.lines[4],
            "Molecule policy: DNA only"
        );
    }

    #[test]
    fn executes_sizeseq_against_real_fixture() {
        let service = implemented_service();
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("sizeseq").expect("tool name should be valid"),
        )
        .with_arguments(vec![sizeseq_fixture().display().to_string()]);

        let response = service.invoke(request).expect("sizeseq should execute");
        match &response.result.payload {
            ResultPayload::SequenceCollection(records) => {
                let ids: Vec<_> = records
                    .iter()
                    .map(|record| record.identifier().accession().to_owned())
                    .collect();
                assert_eq!(ids, vec!["long", "middle", "short", "short_tie"]);
            }
            payload => panic!("unexpected payload: {payload:?}"),
        }
        assert_eq!(
            response.result.summary.lines[1],
            "Ordering: descending length, stable ties"
        );
    }

    #[test]
    fn executes_shuffleseq_against_real_fixture() {
        let service = implemented_service();
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("shuffleseq").expect("tool name should be valid"),
        )
        .with_arguments(vec![
            sequence_fixture().display().to_string(),
            "--seed".to_owned(),
            "7".to_owned(),
        ]);

        let response = service.invoke(request).expect("shuffleseq should execute");
        match &response.result.payload {
            ResultPayload::SequenceCollection(records) => {
                assert_eq!(records[0].residues(), "CGTA");
                assert_eq!(records[1].residues(), "TTTT");
                assert_eq!(records[2].residues(), "CCGG");
            }
            payload => panic!("unexpected payload: {payload:?}"),
        }
        assert_eq!(response.result.summary.lines[1], "Seed: 7");
    }

    #[test]
    fn executes_pasteseq_against_real_fixture() {
        let service = implemented_service();
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("pasteseq").expect("tool name should be valid"),
        )
        .with_arguments(vec![
            pasteseq_main_fixture().display().to_string(),
            pasteseq_insert_fixture().display().to_string(),
            "2".to_owned(),
        ]);

        let response = service.invoke(request).expect("pasteseq should execute");
        match &response.result.payload {
            ResultPayload::Sequence(record) => {
                assert_eq!(record.identifier().accession(), "paste_main");
                assert_eq!(record.residues(), "ACTTGT");
            }
            payload => panic!("unexpected payload: {payload:?}"),
        }
        assert_eq!(response.result.summary.lines[2], "Insert after position: 2");
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
    fn executes_biosed_against_real_fixture() {
        let service = implemented_service();
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("biosed").expect("tool name should be valid"),
        )
        .with_arguments(vec![
            biosed_fixture().display().to_string(),
            "2".to_owned(),
            "3".to_owned(),
            "--replace".to_owned(),
            "NN".to_owned(),
        ]);

        let response = service.invoke(request).expect("biosed should execute");
        match &response.result.payload {
            ResultPayload::SequenceCollection(records) => {
                assert_eq!(records[0].residues(), "ANNG");
                assert_eq!(records[1].residues(), "TNNN");
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
    fn executes_msbar_against_real_fixture() {
        let service = implemented_service();
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("msbar").expect("tool name should be valid"),
        )
        .with_arguments(vec![
            msbar_fixture().display().to_string(),
            "2:T".to_owned(),
            "4:A".to_owned(),
        ]);

        let response = service.invoke(request).expect("msbar should execute");
        match &response.result.payload {
            ResultPayload::SequenceCollection(records) => {
                assert_eq!(records[0].residues(), "ATGA");
            }
            payload => panic!("unexpected payload: {payload:?}"),
        }
    }

    #[test]
    fn executes_trimest_against_real_fixture() {
        let service = implemented_service();
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("trimest").expect("tool name should be valid"),
        )
        .with_arguments(vec![
            trimest_fixture().display().to_string(),
            "--min-tail".to_owned(),
            "4".to_owned(),
        ]);

        let response = service.invoke(request).expect("trimest should execute");
        match &response.result.payload {
            ResultPayload::SequenceCollection(records) => {
                assert_eq!(records[0].residues(), "ACGT");
                assert_eq!(records[1].residues(), "TTGCAAA");
            }
            payload => panic!("unexpected payload: {payload:?}"),
        }
    }

    #[test]
    fn executes_vectorstrip_against_real_fixture() {
        let service = implemented_service();
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("vectorstrip").expect("tool name should be valid"),
        )
        .with_arguments(vec![
            vectorstrip_records_fixture().display().to_string(),
            vectorstrip_vector_fixture().display().to_string(),
        ]);

        let response = service.invoke(request).expect("vectorstrip should execute");
        match &response.result.payload {
            ResultPayload::SequenceCollection(records) => {
                assert_eq!(records[0].residues(), "ACGT");
                assert_eq!(records[1].residues(), "TTAA");
                assert_eq!(records[2].residues(), "GGCC");
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
    fn executes_infoseq_against_real_fixture() {
        let service = implemented_service();
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("infoseq").expect("tool name should be valid"),
        )
        .with_arguments(vec![sequence_fixture().display().to_string()]);

        let response = service.invoke(request).expect("infoseq should execute");
        match &response.result.payload {
            ResultPayload::TableReport(table) => {
                assert_eq!(
                    table.columns,
                    vec![
                        "ordinal",
                        "identifier",
                        "display_name",
                        "length",
                        "molecule",
                        "alphabet",
                        "gc_percent",
                        "feature_count",
                        "description",
                        "organism",
                    ]
                );
                assert_eq!(table.rows[0][1], "alpha");
                assert_eq!(table.rows[0][3], "4");
                assert_eq!(table.rows[0][4], "dna");
                assert_eq!(table.rows[0][6], "50.00");
                assert_eq!(table.rows[0][8], "first example");
            }
            payload => panic!("unexpected payload: {payload:?}"),
        }
        assert_eq!(
            response.result.summary.lines[1],
            "Scope: one stable row per input record"
        );
    }

    #[test]
    fn executes_aaindexextract_against_builtin_index() {
        let service = implemented_service();
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("aaindexextract").expect("tool name should be valid"),
        )
        .with_arguments(vec!["hydropathy".to_owned()]);

        let response = service
            .invoke(request)
            .expect("aaindexextract should execute");
        match &response.result.payload {
            ResultPayload::TableReport(table) => {
                assert_eq!(
                    table.columns,
                    vec![
                        "index",
                        "residue",
                        "three_letter",
                        "name",
                        "value",
                        "units",
                        "notes",
                    ]
                );
                assert_eq!(table.rows.len(), 20);
                assert_eq!(table.rows[0][0], "hydropathy_kyte_doolittle");
                assert_eq!(table.rows[0][1], "A");
                assert_eq!(table.rows[0][4], "1.800");
            }
            payload => panic!("unexpected payload: {payload:?}"),
        }
    }

    #[test]
    fn executes_infobase_against_ambiguity_symbol() {
        let service = implemented_service();
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("infobase").expect("tool name should be valid"),
        )
        .with_arguments(vec!["N".to_owned()]);

        let response = service.invoke(request).expect("infobase should execute");
        match &response.result.payload {
            ResultPayload::TableReport(table) => {
                assert_eq!(table.rows.len(), 1);
                assert_eq!(table.rows[0][0], "N");
                assert_eq!(table.rows[0][2], "ambiguity");
                assert_eq!(table.rows[0][4], "ACGTU");
            }
            payload => panic!("unexpected payload: {payload:?}"),
        }
    }

    #[test]
    fn executes_inforesidue_against_canonical_residue() {
        let service = implemented_service();
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("inforesidue").expect("tool name should be valid"),
        )
        .with_arguments(vec!["K".to_owned()]);

        let response = service.invoke(request).expect("inforesidue should execute");
        match &response.result.payload {
            ResultPayload::TableReport(table) => {
                assert_eq!(table.rows.len(), 1);
                assert_eq!(table.rows[0][0], "K");
                assert_eq!(table.rows[0][1], "Lys");
                assert_eq!(table.rows[0][3], "positive");
                assert_eq!(table.rows[0][6], "-3.900");
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
    fn executes_maskambignuc_against_real_fixture() {
        let service = implemented_service();
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("maskambignuc").expect("tool name should be valid"),
        )
        .with_arguments(vec![ambiguous_nucleotide_fixture().display().to_string()]);

        let response = service
            .invoke(request)
            .expect("maskambignuc should execute");
        match &response.result.payload {
            ResultPayload::SequenceCollection(records) => {
                assert_eq!(records.len(), 1);
                assert_eq!(records[0].residues(), "ACGTNNN");
            }
            payload => panic!("unexpected payload: {payload:?}"),
        }
        assert_eq!(
            response.result.summary.lines[2],
            "Mask rule: conservative nucleotide ambiguity symbols -> N"
        );
    }

    #[test]
    fn executes_maskambigprot_against_real_fixture() {
        let service = implemented_service();
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("maskambigprot").expect("tool name should be valid"),
        )
        .with_arguments(vec![ambiguous_protein_fixture().display().to_string()]);

        let response = service
            .invoke(request)
            .expect("maskambigprot should execute");
        match &response.result.payload {
            ResultPayload::SequenceCollection(records) => {
                assert_eq!(records.len(), 1);
                assert_eq!(records[0].residues(), "MXXXXUO-*");
            }
            payload => panic!("unexpected payload: {payload:?}"),
        }
        assert_eq!(
            response.result.summary.lines[2],
            "Mask rule: conservative protein ambiguity symbols -> X"
        );
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
    fn executes_twofeat_against_annotated_fixture() {
        let service = implemented_service();
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("twofeat").expect("tool name should be valid"),
        )
        .with_arguments(vec![
            annotated_feature_fixture().display().to_string(),
            "--a-kind".to_owned(),
            "gene".to_owned(),
            "--b-kind".to_owned(),
            "cds".to_owned(),
        ]);

        let response = service.invoke(request).expect("twofeat should execute");
        match &response.result.payload {
            ResultPayload::TableReport(table) => {
                assert_eq!(table.columns[0], "record");
                assert_eq!(table.rows.len(), 1);
                assert_eq!(table.rows[0][0], "FEAT1");
                assert_eq!(table.rows[0][1], "gene");
                assert_eq!(table.rows[0][7], "CDS");
                assert_eq!(table.rows[0][13], "1");
                assert_eq!(table.rows[0][14], "separated");
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
                assert!(
                    report
                        .body
                        .contains("FEATURES             Location/Qualifiers")
                );
                assert!(report.body.contains("/product=\"short peptide\""));
            }
            payload => panic!("unexpected payload: {payload:?}"),
        }
    }

    #[test]
    fn executes_splitsource_against_synthetic_fixture() {
        let service = implemented_service();
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("splitsource").expect("tool name should be valid"),
        )
        .with_arguments(vec![splitsource_fixture().display().to_string()]);

        let response = service.invoke(request).expect("splitsource should execute");
        match &response.result.payload {
            ResultPayload::SequenceCollection(records) => {
                assert_eq!(records.len(), 2);
                assert_eq!(records[0].residues(), "AAAT");
                assert_eq!(records[1].residues(), "GGGC");
            }
            payload => panic!("unexpected payload: {payload:?}"),
        }
        assert_eq!(response.result.summary.lines[1], "Fragments: 2");
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
                .ends_with("crates/epithema-tools/tests/fixtures/checktrans_nucleotide.fasta")
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
        assert_eq!(
            response.result.summary.lines[1],
            "Frame policy: forward frames 1-3"
        );
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
                .ends_with("crates/epithema-tools/tests/fixtures/nucleotide_pattern_records.fasta")
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
                .ends_with("crates/epithema-tools/tests/fixtures/protein_records.fasta")
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
                .ends_with("crates/epithema-tools/tests/fixtures/checktrans_nucleotide.fasta")
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
            "epithema-fuzzpro-overlap-{}.fasta",
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
            "epithema-fuzztran-overlap-{}.fasta",
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
            "epithema-fuzztran-ambiguous-{}.fasta",
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
            "epithema-fuzznuc-overlap-{}.fasta",
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
    fn executes_preg_against_protein_fixture() {
        let service = implemented_service();
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("preg").expect("tool name should be valid"),
        )
        .with_arguments(vec![preg_fixture().display().to_string(), "MAM".to_owned()]);

        let response = service.invoke(request).expect("preg should execute");
        match &response.result.payload {
            ResultPayload::TableReport(table) => {
                assert_eq!(table.rows.len(), 2);
                assert_eq!(table.rows[0][0], "pregA");
                assert_eq!(table.rows[0][1], "MAM");
                assert_eq!(table.rows[0][2], "1");
                assert_eq!(table.rows[0][3], "3");
                assert_eq!(table.rows[1][2], "3");
                assert_eq!(table.rows[1][3], "5");
            }
            payload => panic!("unexpected payload: {payload:?}"),
        }
    }

    #[test]
    fn executes_dreg_against_nucleotide_fixture() {
        let service = implemented_service();
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("dreg").expect("tool name should be valid"),
        )
        .with_arguments(vec![dreg_fixture().display().to_string(), "ATA".to_owned()]);

        let response = service.invoke(request).expect("dreg should execute");
        match &response.result.payload {
            ResultPayload::TableReport(table) => {
                assert_eq!(table.rows.len(), 2);
                assert_eq!(table.rows[0][0], "dregA");
                assert_eq!(table.rows[0][2], "1");
                assert_eq!(table.rows[0][3], "3");
                assert_eq!(table.rows[1][2], "3");
                assert_eq!(table.rows[1][3], "5");
            }
            payload => panic!("unexpected payload: {payload:?}"),
        }
    }

    #[test]
    fn executes_palindrome_against_fixture() {
        let service = implemented_service();
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("palindrome").expect("tool name should be valid"),
        )
        .with_arguments(vec![
            palindrome_fixture().display().to_string(),
            "--min-length".to_owned(),
            "6".to_owned(),
            "--max-length".to_owned(),
            "6".to_owned(),
        ]);

        let response = service.invoke(request).expect("palindrome should execute");
        match &response.result.payload {
            ResultPayload::TableReport(table) => {
                assert!(!table.rows.is_empty());
                assert_eq!(table.rows[0][0], "palA");
                assert_eq!(table.rows[0][1], "1");
                assert_eq!(table.rows[0][2], "6");
                assert_eq!(table.rows[0][4], "ATGCAT");
            }
            payload => panic!("unexpected payload: {payload:?}"),
        }
    }

    #[test]
    fn executes_einverted_against_fixture() {
        let service = implemented_service();
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("einverted").expect("tool name should be valid"),
        )
        .with_arguments(vec![
            einverted_fixture().display().to_string(),
            "--min-arm-length".to_owned(),
            "4".to_owned(),
            "--max-gap-length".to_owned(),
            "2".to_owned(),
        ]);

        let response = service.invoke(request).expect("einverted should execute");
        match &response.result.payload {
            ResultPayload::TableReport(table) => {
                assert!(!table.rows.is_empty());
                assert_eq!(table.rows[0][0], "invA");
                assert_eq!(table.rows[0][1], "1");
                assert_eq!(table.rows[0][3], "7");
                assert_eq!(table.rows[0][5], "2");
                assert_eq!(table.rows[0][7], "ATGC");
                assert_eq!(table.rows[0][8], "GCAT");
            }
            payload => panic!("unexpected payload: {payload:?}"),
        }
    }

    #[test]
    fn executes_patmatdb_against_fixtures() {
        let service = implemented_service();
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("patmatdb").expect("tool name should be valid"),
        )
        .with_arguments(vec![
            patmatdb_records_fixture().display().to_string(),
            patmatdb_motifs_fixture().display().to_string(),
        ]);

        let response = service.invoke(request).expect("patmatdb should execute");
        match &response.result.payload {
            ResultPayload::TableReport(table) => {
                assert_eq!(table.rows.len(), 2);
                assert_eq!(table.rows[0][0], "patmatA");
                assert_eq!(table.rows[0][1], "motif_a");
                assert_eq!(table.rows[1][1], "motif_b");
            }
            payload => panic!("unexpected payload: {payload:?}"),
        }
    }

    #[test]
    fn executes_seqmatchall_against_fixture() {
        let service = implemented_service();
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("seqmatchall").expect("tool name should be valid"),
        )
        .with_arguments(vec![
            seqmatchall_fixture().display().to_string(),
            "--word-size".to_owned(),
            "4".to_owned(),
        ]);

        let response = service.invoke(request).expect("seqmatchall should execute");
        match &response.result.payload {
            ResultPayload::TableReport(table) => {
                assert_eq!(table.rows.len(), 3);
                assert_eq!(table.rows[0][0], "sm_a");
                assert_eq!(table.rows[0][1], "sm_b");
                assert_eq!(table.rows[1][1], "sm_c");
                assert_eq!(table.rows[2][0], "sm_b");
                assert_eq!(table.rows[2][1], "sm_c");
            }
            payload => panic!("unexpected payload: {payload:?}"),
        }
    }

    #[test]
    fn executes_wordmatch_against_fixtures() {
        let service = implemented_service();
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("wordmatch").expect("tool name should be valid"),
        )
        .with_arguments(vec![
            wordmatch_query_fixture().display().to_string(),
            wordmatch_target_fixture().display().to_string(),
            "--word-size".to_owned(),
            "4".to_owned(),
        ]);

        let response = service.invoke(request).expect("wordmatch should execute");
        match &response.result.payload {
            ResultPayload::TableReport(table) => {
                assert_eq!(table.rows.len(), 1);
                assert_eq!(table.rows[0][0], "wm_query");
                assert_eq!(table.rows[0][1], "wm_target");
                assert_eq!(table.rows[0][6], "4");
                assert_eq!(table.rows[0][7], "ACGT");
            }
            payload => panic!("unexpected payload: {payload:?}"),
        }
    }

    #[test]
    fn executes_wordfinder_against_fixtures() {
        let service = implemented_service();
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("wordfinder").expect("tool name should be valid"),
        )
        .with_arguments(vec![
            wordmatch_query_fixture().display().to_string(),
            wordfinder_targets_fixture().display().to_string(),
            "--word-size".to_owned(),
            "4".to_owned(),
        ]);

        let response = service.invoke(request).expect("wordfinder should execute");
        match &response.result.payload {
            ResultPayload::TableReport(table) => {
                assert_eq!(table.rows.len(), 1);
                assert_eq!(table.rows[0][0], "wm_query");
                assert_eq!(table.rows[0][1], "wf_target_a");
                assert_eq!(table.rows[0][6], "4");
                assert_eq!(table.rows[0][7], "ACGT");
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
                .ends_with("crates/epithema-tools/tests/fixtures/nucleotide_pattern_records.fasta")
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
    fn executes_wordcount_against_sequence_fixture() {
        let service = implemented_service();
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("wordcount").expect("tool name should be valid"),
        )
        .with_arguments(vec![
            sequence_fixture().display().to_string(),
            "--word-size".to_owned(),
            "2".to_owned(),
        ]);

        let response = service.invoke(request).expect("wordcount should execute");
        match &response.result.payload {
            ResultPayload::TableReport(table) => {
                assert_eq!(
                    table.columns,
                    vec![
                        "scope",
                        "record",
                        "molecule",
                        "word_size",
                        "word",
                        "count",
                        "frequency",
                        "skipped_gap_windows",
                    ]
                );
                assert!(table.rows.iter().any(|row| {
                    row[0] == "record"
                        && row[1] == "alpha"
                        && row[4] == "AC"
                        && row[5] == "1"
                        && row[6] == "0.333333"
                }));
                assert!(table.rows.iter().any(|row| {
                    row[0] == "aggregate" && row[1] == "ALL" && row[4] == "TT" && row[5] == "3"
                }));
            }
            payload => panic!("unexpected payload: {payload:?}"),
        }
        assert_eq!(
            response.result.summary.lines[1],
            "Word model: overlapping normalized windows"
        );
        let plot = response
            .result
            .plot
            .as_ref()
            .expect("wordcount should attach an aggregate plot payload");
        assert_eq!(plot.kind.as_str(), "bar");
    }

    #[test]
    fn wordcount_writes_canonical_plot_contract_fixture() {
        let output_path =
            std::env::temp_dir().join(format!("emboss-wordcount-plot-{}.json", std::process::id()));
        let service = implemented_service();
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("wordcount").expect("tool name should be valid"),
        )
        .with_arguments(vec![
            sequence_fixture().display().to_string(),
            "--word-size".to_owned(),
            "2".to_owned(),
            "--plot-contract-out".to_owned(),
            output_path.display().to_string(),
        ]);

        let response = service.invoke(request).expect("wordcount should execute");
        let emitted =
            std::fs::read_to_string(&output_path).expect("wordcount should write a plot contract");
        let canonical = std::fs::read_to_string(wordcount_plot_fixture())
            .expect("canonical wordcount plot contract should be readable");
        std::fs::remove_file(&output_path).ok();

        assert_eq!(emitted, canonical);
        assert!(response.result.artifacts.iter().any(|artifact| {
            artifact.id == "wordcount-plot-contract"
                && artifact.local_path.as_ref() == Some(&output_path)
        }));
    }

    #[test]
    fn executes_oddcomp_against_protein_fixture() {
        let service = implemented_service();
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("oddcomp").expect("tool name should be valid"),
        )
        .with_arguments(vec![
            oddcomp_fixture().display().to_string(),
            "--word".to_owned(),
            "MAM".to_owned(),
            "--word".to_owned(),
            "QQQ".to_owned(),
        ]);

        let response = service.invoke(request).expect("oddcomp should execute");
        match &response.result.payload {
            ResultPayload::TableReport(table) => {
                assert_eq!(
                    table.columns,
                    vec![
                        "record",
                        "query_word",
                        "word_length",
                        "count",
                        "frequency",
                        "contains",
                        "counted_windows",
                    ]
                );
                assert_eq!(table.rows[0][0], "oddA");
                assert_eq!(table.rows[0][1], "MAM");
                assert_eq!(table.rows[0][3], "1");
                assert_eq!(table.rows[3][0], "oddB");
                assert_eq!(table.rows[3][1], "QQQ");
                assert_eq!(table.rows[3][3], "2");
            }
            payload => panic!("unexpected payload: {payload:?}"),
        }
        assert_eq!(
            response.result.summary.lines[2],
            "Counting model: overlapping exact literal protein words"
        );
    }

    #[test]
    fn executes_dan_against_sequence_fixture() {
        let service = implemented_service();
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("dan").expect("tool name should be valid"),
        )
        .with_arguments(vec![
            sequence_fixture().display().to_string(),
            "--window".to_owned(),
            "2".to_owned(),
        ]);

        let response = service.invoke(request).expect("dan should execute");
        match &response.result.payload {
            ResultPayload::TableReport(table) => {
                assert_eq!(
                    table.columns,
                    vec![
                        "record",
                        "window_index",
                        "start",
                        "end",
                        "length",
                        "gc_percent",
                        "tm_celsius",
                    ]
                );
                assert_eq!(table.rows[0][0], "alpha");
                assert_eq!(table.rows[0][1], "1");
                assert_eq!(table.rows[0][2], "1");
                assert_eq!(table.rows[0][3], "2");
                assert_eq!(table.rows[0][4], "2");
                assert_eq!(table.rows[0][5], "50.00");
                assert_eq!(table.rows[0][6], "6.00");
            }
            payload => panic!("unexpected payload: {payload:?}"),
        }
        assert_eq!(
            response.result.summary.lines[1],
            "Model: conservative Wallace/GC-length hybrid estimate"
        );
    }

    #[test]
    fn rejects_ambiguous_input_for_dan() {
        let service = implemented_service();
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("dan").expect("tool name should be valid"),
        )
        .with_arguments(vec![nucleotide_pattern_fixture().display().to_string()]);

        let error = service
            .invoke(request)
            .expect_err("ambiguous nucleotide input should fail for dan");
        assert!(error.to_string().contains("canonical A/C/G/T/U"));
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
    fn executes_pepwindow_against_protein_fixture() {
        let service = implemented_service();
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("pepwindow").expect("tool name should be valid"),
        )
        .with_arguments(vec![
            pepwindow_fixture().display().to_string(),
            "--window".to_owned(),
            "5".to_owned(),
            "--step".to_owned(),
            "2".to_owned(),
        ]);

        let response = service.invoke(request).expect("pepwindow should execute");
        match &response.result.payload {
            ResultPayload::TableReport(table) => {
                assert_eq!(table.rows.len(), 10);
                assert_eq!(table.rows[0][0], "pepwindow_example");
                assert_eq!(table.rows[0][1], "1");
                assert_eq!(table.rows[0][2], "5");
            }
            payload => panic!("unexpected payload: {payload:?}"),
        }
        let plot = response
            .result
            .plot
            .as_ref()
            .expect("pepwindow should attach a plot payload");
        assert_eq!(plot.kind.as_str(), "line");
    }

    #[test]
    fn pepwindow_writes_canonical_plot_contract_fixture() {
        let service = implemented_service();
        let output_path = std::env::temp_dir().join(format!(
            "emboss-pepwindow-plot-{}.json",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("time should advance")
                .as_nanos()
        ));
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("pepwindow").expect("tool name should be valid"),
        )
        .with_arguments(vec![
            pepwindow_fixture().display().to_string(),
            "--window".to_owned(),
            "5".to_owned(),
            "--step".to_owned(),
            "2".to_owned(),
            "--plot-contract-out".to_owned(),
            output_path.display().to_string(),
        ]);

        let response = service.invoke(request).expect("pepwindow should execute");
        let emitted =
            std::fs::read_to_string(&output_path).expect("plot contract file should exist");
        let canonical = std::fs::read_to_string(pepwindow_plot_fixture())
            .expect("canonical fixture should exist");
        assert_eq!(emitted.trim(), canonical.trim());
        assert!(response.result.artifacts.iter().any(|artifact| {
            artifact.id == "pepwindow-plot-contract"
                && artifact.local_path.as_ref() == Some(&output_path)
        }));
        let _ = std::fs::remove_file(output_path);
    }

    #[test]
    fn pepwindow_rejects_unsupported_residues() {
        let service = implemented_service();
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("pepwindow").expect("tool name should be valid"),
        )
        .with_arguments(vec![
            pepwindow_invalid_fixture().display().to_string(),
            "--window".to_owned(),
            "5".to_owned(),
        ]);

        let error = service
            .invoke(request)
            .expect_err("unsupported residues should fail");
        assert_eq!(
            error.code(),
            Some("tools.pepwindow.input.unsupported_residue")
        );
    }

    #[test]
    fn executes_hmoment_against_protein_fixture() {
        let service = implemented_service();
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("hmoment").expect("tool name should be valid"),
        )
        .with_arguments(vec![
            hmoment_fixture().display().to_string(),
            "--window".to_owned(),
            "4".to_owned(),
        ]);

        let response = service.invoke(request).expect("hmoment should execute");
        match &response.result.payload {
            ResultPayload::TableReport(table) => {
                assert_eq!(table.rows.len(), 2);
                assert_eq!(table.rows[0][0], "hmoment_example");
                assert_eq!(table.rows[0][1], "1");
                assert_eq!(table.rows[0][2], "4");
            }
            payload => panic!("unexpected payload: {payload:?}"),
        }
        let plot = response
            .result
            .plot
            .as_ref()
            .expect("hmoment should attach a plot payload");
        assert_eq!(plot.kind.as_str(), "line");
    }

    #[test]
    fn executes_density_against_nucleotide_fixture() {
        let service = implemented_service();
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("density").expect("tool name should be valid"),
        )
        .with_arguments(vec![
            density_fixture().display().to_string(),
            "--window".to_owned(),
            "4".to_owned(),
        ]);

        let response = service.invoke(request).expect("density should execute");
        match &response.result.payload {
            ResultPayload::TableReport(table) => {
                assert_eq!(table.rows.len(), 4);
                assert_eq!(table.rows[0][0], "density_example");
                assert_eq!(table.rows[0][1], "1");
                assert_eq!(table.rows[0][2], "4");
                assert_eq!(table.rows[0][12], "0.500000");
            }
            payload => panic!("unexpected payload: {payload:?}"),
        }
        let plot = response
            .result
            .plot
            .as_ref()
            .expect("density should attach a plot payload");
        assert_eq!(plot.kind.as_str(), "line");
        assert_eq!(plot.series.len(), 1);
    }

    #[test]
    fn density_writes_plot_contract_output() {
        let service = implemented_service();
        let output_path = std::env::temp_dir().join(format!(
            "emboss-density-plot-{}.json",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("time should advance")
                .as_nanos()
        ));
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("density").expect("tool name should be valid"),
        )
        .with_arguments(vec![
            density_fixture().display().to_string(),
            "--window".to_owned(),
            "4".to_owned(),
            "--plot-contract-out".to_owned(),
            output_path.display().to_string(),
        ]);

        let response = service.invoke(request).expect("density should execute");
        let emitted =
            std::fs::read_to_string(&output_path).expect("plot contract file should exist");
        assert!(emitted.contains("\"tool\": \"density\""));
        assert!(emitted.contains("\"method\": \"nucleotide_density_profile\""));
        assert!(response.result.artifacts.iter().any(|artifact| {
            artifact.id == "density-plot-contract"
                && artifact.local_path.as_ref() == Some(&output_path)
        }));
        let _ = std::fs::remove_file(output_path);
    }

    #[test]
    fn density_rejects_invalid_window() {
        let service = implemented_service();
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("density").expect("tool name should be valid"),
        )
        .with_arguments(vec![
            density_fixture().display().to_string(),
            "--window".to_owned(),
            "0".to_owned(),
        ]);

        let error = service
            .invoke(request)
            .expect_err("invalid window should fail");
        assert_eq!(error.code(), Some("service.tool.density.--window_invalid"));
    }

    #[test]
    fn executes_banana_against_nucleotide_fixture() {
        let service = implemented_service();
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("banana").expect("tool name should be valid"),
        )
        .with_arguments(vec![banana_fixture().display().to_string()]);

        let response = service.invoke(request).expect("banana should execute");
        match &response.result.payload {
            ResultPayload::TableReport(table) => {
                assert_eq!(table.rows.len(), 45);
                assert_eq!(table.rows[0][0], "banana_example");
                assert_eq!(table.rows[0][1], "1");
                assert_eq!(table.rows[0][2], "A");
                assert_eq!(table.rows[0][3], "");
                assert_eq!(table.rows[0][4], "");
            }
            payload => panic!("unexpected payload: {payload:?}"),
        }
        let plot = response
            .result
            .plot
            .as_ref()
            .expect("banana should attach a plot payload");
        assert_eq!(plot.kind.as_str(), "line");
        assert_eq!(plot.series.len(), 1);
    }

    #[test]
    fn banana_writes_plot_contract_output() {
        let service = implemented_service();
        let output_path = std::env::temp_dir().join(format!(
            "emboss-banana-plot-{}.json",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("time should advance")
                .as_nanos()
        ));
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("banana").expect("tool name should be valid"),
        )
        .with_arguments(vec![
            banana_fixture().display().to_string(),
            "--plot-contract-out".to_owned(),
            output_path.display().to_string(),
        ]);

        let response = service.invoke(request).expect("banana should execute");
        let emitted =
            std::fs::read_to_string(&output_path).expect("plot contract file should exist");
        assert!(emitted.contains("\"tool\": \"banana\""));
        assert!(emitted.contains("\"method\": \"nucleotide_banana_profile\""));
        assert!(response.result.artifacts.iter().any(|artifact| {
            artifact.id == "banana-plot-contract"
                && artifact.local_path.as_ref() == Some(&output_path)
        }));
        let _ = std::fs::remove_file(output_path);
    }

    #[test]
    fn banana_rejects_unknown_argument() {
        let service = implemented_service();
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("banana").expect("tool name should be valid"),
        )
        .with_arguments(vec![
            banana_fixture().display().to_string(),
            "--window".to_owned(),
            "4".to_owned(),
        ]);

        let error = service
            .invoke(request)
            .expect_err("unknown argument should fail");
        assert_eq!(error.code(), Some("service.tool.banana.argument_unknown"));
    }

    #[test]
    fn executes_wobble_against_nucleotide_fixture() {
        let service = implemented_service();
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("wobble").expect("tool name should be valid"),
        )
        .with_arguments(vec![
            wobble_fixture().display().to_string(),
            "--codon-window".to_owned(),
            "3".to_owned(),
        ]);

        let response = service.invoke(request).expect("wobble should execute");
        match &response.result.payload {
            ResultPayload::TableReport(table) => {
                assert_eq!(table.rows.len(), 3);
                assert_eq!(table.rows[0][0], "wobble_example");
                assert_eq!(table.rows[0][1], "1");
                assert_eq!(table.rows[0][2], "9");
                assert_eq!(table.rows[0][11], "0.333333");
            }
            payload => panic!("unexpected payload: {payload:?}"),
        }
        let plot = response
            .result
            .plot
            .as_ref()
            .expect("wobble should attach a plot payload");
        assert_eq!(plot.kind.as_str(), "line");
        assert_eq!(plot.series.len(), 1);
    }

    #[test]
    fn wobble_writes_plot_contract_output() {
        let service = implemented_service();
        let output_path = std::env::temp_dir().join(format!(
            "emboss-wobble-plot-{}.json",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("time should advance")
                .as_nanos()
        ));
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("wobble").expect("tool name should be valid"),
        )
        .with_arguments(vec![
            wobble_fixture().display().to_string(),
            "--codon-window".to_owned(),
            "3".to_owned(),
            "--plot-contract-out".to_owned(),
            output_path.display().to_string(),
        ]);

        let response = service.invoke(request).expect("wobble should execute");
        let emitted =
            std::fs::read_to_string(&output_path).expect("plot contract file should exist");
        assert!(emitted.contains("\"tool\": \"wobble\""));
        assert!(emitted.contains("\"method\": \"nucleotide_wobble_profile\""));
        assert!(response.result.artifacts.iter().any(|artifact| {
            artifact.id == "wobble-plot-contract"
                && artifact.local_path.as_ref() == Some(&output_path)
        }));
        let _ = std::fs::remove_file(output_path);
    }

    #[test]
    fn wobble_rejects_invalid_codon_window() {
        let service = implemented_service();
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("wobble").expect("tool name should be valid"),
        )
        .with_arguments(vec![
            wobble_fixture().display().to_string(),
            "--codon-window".to_owned(),
            "0".to_owned(),
        ]);

        let error = service
            .invoke(request)
            .expect_err("invalid codon window should fail");
        assert_eq!(
            error.code(),
            Some("service.tool.wobble.--codon-window_invalid")
        );
    }

    #[test]
    fn executes_isochore_against_nucleotide_fixture() {
        let service = implemented_service();
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("isochore").expect("tool name should be valid"),
        )
        .with_arguments(vec![
            isochore_fixture().display().to_string(),
            "--window".to_owned(),
            "4".to_owned(),
            "--step".to_owned(),
            "4".to_owned(),
        ]);

        let response = service.invoke(request).expect("isochore should execute");
        match &response.result.payload {
            ResultPayload::TableReport(table) => {
                assert_eq!(table.rows.len(), 3);
                assert_eq!(table.rows[0][0], "isochore_example");
                assert_eq!(table.rows[0][1], "1");
                assert_eq!(table.rows[0][2], "4");
                assert_eq!(table.rows[0][9], "0.000000");
                assert_eq!(table.rows[0][10], "L1");
            }
            payload => panic!("unexpected payload: {payload:?}"),
        }
        let plot = response
            .result
            .plot
            .as_ref()
            .expect("isochore should attach a plot payload");
        assert_eq!(plot.kind.as_str(), "line");
        assert_eq!(plot.series.len(), 1);
    }

    #[test]
    fn isochore_writes_plot_contract_output() {
        let service = implemented_service();
        let output_path = std::env::temp_dir().join(format!(
            "emboss-isochore-plot-{}.json",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("time should advance")
                .as_nanos()
        ));
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("isochore").expect("tool name should be valid"),
        )
        .with_arguments(vec![
            isochore_fixture().display().to_string(),
            "--window".to_owned(),
            "4".to_owned(),
            "--step".to_owned(),
            "4".to_owned(),
            "--plot-contract-out".to_owned(),
            output_path.display().to_string(),
        ]);

        let response = service.invoke(request).expect("isochore should execute");
        let emitted =
            std::fs::read_to_string(&output_path).expect("plot contract file should exist");
        assert!(emitted.contains("\"tool\": \"isochore\""));
        assert!(emitted.contains("\"method\": \"nucleotide_isochore_profile\""));
        assert!(response.result.artifacts.iter().any(|artifact| {
            artifact.id == "isochore-plot-contract"
                && artifact.local_path.as_ref() == Some(&output_path)
        }));
        let _ = std::fs::remove_file(output_path);
    }

    #[test]
    fn isochore_rejects_invalid_window() {
        let service = implemented_service();
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("isochore").expect("tool name should be valid"),
        )
        .with_arguments(vec![
            isochore_fixture().display().to_string(),
            "--window".to_owned(),
            "0".to_owned(),
        ]);

        let error = service
            .invoke(request)
            .expect_err("invalid window should fail");
        assert_eq!(error.code(), Some("service.tool.isochore.--window_invalid"));
    }

    #[test]
    fn executes_syco_against_coding_fixture() {
        let service = implemented_service();
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("syco").expect("tool name should be valid"),
        )
        .with_arguments(vec![
            syco_fixture().display().to_string(),
            codon_reference_fixture().display().to_string(),
            "--codon-window".to_owned(),
            "2".to_owned(),
        ]);

        let response = service.invoke(request).expect("syco should execute");
        match &response.result.payload {
            ResultPayload::TableReport(table) => {
                assert_eq!(table.rows.len(), 3);
                assert_eq!(table.rows[0][0], "syco_example");
                assert_eq!(table.rows[0][1], "1");
                assert_eq!(table.rows[0][2], "6");
                assert_eq!(table.rows[0][6], "0.200000");
            }
            payload => panic!("unexpected payload: {payload:?}"),
        }
        let plot = response
            .result
            .plot
            .as_ref()
            .expect("syco should attach a plot payload");
        assert_eq!(plot.kind.as_str(), "line");
        assert_eq!(plot.series.len(), 1);
    }

    #[test]
    fn syco_writes_plot_contract_output() {
        let service = implemented_service();
        let output_path = std::env::temp_dir().join(format!(
            "emboss-syco-plot-{}.json",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("time should advance")
                .as_nanos()
        ));
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("syco").expect("tool name should be valid"),
        )
        .with_arguments(vec![
            syco_fixture().display().to_string(),
            codon_reference_fixture().display().to_string(),
            "--codon-window".to_owned(),
            "2".to_owned(),
            "--plot-contract-out".to_owned(),
            output_path.display().to_string(),
        ]);

        let response = service.invoke(request).expect("syco should execute");
        let emitted =
            std::fs::read_to_string(&output_path).expect("plot contract file should exist");
        assert!(emitted.contains("\"tool\": \"syco\""));
        assert!(emitted.contains("\"method\": \"nucleotide_syco_profile\""));
        assert!(response.result.artifacts.iter().any(|artifact| {
            artifact.id == "syco-plot-contract"
                && artifact.local_path.as_ref() == Some(&output_path)
        }));
        let _ = std::fs::remove_file(output_path);
    }

    #[test]
    fn syco_rejects_invalid_codon_window() {
        let service = implemented_service();
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("syco").expect("tool name should be valid"),
        )
        .with_arguments(vec![
            syco_fixture().display().to_string(),
            codon_reference_fixture().display().to_string(),
            "--codon-window".to_owned(),
            "0".to_owned(),
        ]);

        let error = service
            .invoke(request)
            .expect_err("invalid codon window should fail");
        assert_eq!(
            error.code(),
            Some("service.tool.syco.--codon-window_invalid")
        );
    }

    #[test]
    fn hmoment_writes_plot_contract_output() {
        let service = implemented_service();
        let output_path = std::env::temp_dir().join(format!(
            "emboss-hmoment-plot-{}.json",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("time should advance")
                .as_nanos()
        ));
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("hmoment").expect("tool name should be valid"),
        )
        .with_arguments(vec![
            hmoment_fixture().display().to_string(),
            "--window".to_owned(),
            "4".to_owned(),
            "--plot-contract-out".to_owned(),
            output_path.display().to_string(),
        ]);

        let response = service.invoke(request).expect("hmoment should execute");
        let emitted =
            std::fs::read_to_string(&output_path).expect("plot contract file should exist");
        assert!(emitted.contains("\"tool\": \"hmoment\""));
        assert!(emitted.contains("\"method\": \"protein_hydrophobic_moment_profile\""));
        assert!(response.result.artifacts.iter().any(|artifact| {
            artifact.id == "hmoment-plot-contract"
                && artifact.local_path.as_ref() == Some(&output_path)
        }));
        let _ = std::fs::remove_file(output_path);
    }

    #[test]
    fn hmoment_rejects_invalid_angle() {
        let service = implemented_service();
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("hmoment").expect("tool name should be valid"),
        )
        .with_arguments(vec![
            hmoment_fixture().display().to_string(),
            "--window".to_owned(),
            "4".to_owned(),
            "--angle-degrees".to_owned(),
            "0".to_owned(),
        ]);

        let error = service
            .invoke(request)
            .expect_err("invalid angle should fail");
        assert_eq!(error.code(), Some("tools.hmoment.angle.invalid"));
    }

    #[test]
    fn executes_octanol_against_protein_fixture() {
        let service = implemented_service();
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("octanol").expect("tool name should be valid"),
        )
        .with_arguments(vec![
            octanol_fixture().display().to_string(),
            "--window".to_owned(),
            "3".to_owned(),
        ]);

        let response = service.invoke(request).expect("octanol should execute");
        match &response.result.payload {
            ResultPayload::TableReport(table) => {
                assert_eq!(table.rows.len(), 4);
                assert_eq!(table.rows[0][0], "octanol_example");
                assert_eq!(table.rows[0][1], "1");
                assert_eq!(table.rows[0][2], "3");
            }
            payload => panic!("unexpected payload: {payload:?}"),
        }
        let plot = response
            .result
            .plot
            .as_ref()
            .expect("octanol should attach a plot payload");
        assert_eq!(plot.kind.as_str(), "line");
    }

    #[test]
    fn octanol_writes_plot_contract_output() {
        let service = implemented_service();
        let output_path = std::env::temp_dir().join(format!(
            "emboss-octanol-plot-{}.json",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("time should advance")
                .as_nanos()
        ));
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("octanol").expect("tool name should be valid"),
        )
        .with_arguments(vec![
            octanol_fixture().display().to_string(),
            "--window".to_owned(),
            "3".to_owned(),
            "--plot-contract-out".to_owned(),
            output_path.display().to_string(),
        ]);

        let response = service.invoke(request).expect("octanol should execute");
        let emitted =
            std::fs::read_to_string(&output_path).expect("plot contract file should exist");
        assert!(emitted.contains("\"tool\": \"octanol\""));
        assert!(emitted.contains("\"method\": \"protein_octanol_profile\""));
        assert!(response.result.artifacts.iter().any(|artifact| {
            artifact.id == "octanol-plot-contract"
                && artifact.local_path.as_ref() == Some(&output_path)
        }));
        let _ = std::fs::remove_file(output_path);
    }

    #[test]
    fn octanol_rejects_invalid_window() {
        let service = implemented_service();
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("octanol").expect("tool name should be valid"),
        )
        .with_arguments(vec![
            octanol_fixture().display().to_string(),
            "--window".to_owned(),
            "0".to_owned(),
        ]);

        let error = service
            .invoke(request)
            .expect_err("invalid window should fail");
        assert_eq!(error.code(), Some("service.tool.octanol.--window_invalid"));
    }

    #[test]
    fn executes_pepinfo_against_protein_fixture() {
        let service = implemented_service();
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("pepinfo").expect("tool name should be valid"),
        )
        .with_arguments(vec![
            pepinfo_fixture().display().to_string(),
            "--window".to_owned(),
            "3".to_owned(),
        ]);

        let response = service.invoke(request).expect("pepinfo should execute");
        match &response.result.payload {
            ResultPayload::TableReport(table) => {
                assert_eq!(table.rows.len(), 7);
                assert_eq!(table.rows[0][0], "pepinfo_example");
                assert_eq!(table.rows[0][1], "1");
                assert_eq!(table.rows[0][2], "3");
            }
            payload => panic!("unexpected payload: {payload:?}"),
        }
        let plot = response
            .result
            .plot
            .as_ref()
            .expect("pepinfo should attach a plot payload");
        assert_eq!(plot.kind.as_str(), "line");
        assert_eq!(plot.series.len(), 4);
    }

    #[test]
    fn pepinfo_writes_plot_contract_output() {
        let service = implemented_service();
        let output_path = std::env::temp_dir().join(format!(
            "emboss-pepinfo-plot-{}.json",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("time should advance")
                .as_nanos()
        ));
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("pepinfo").expect("tool name should be valid"),
        )
        .with_arguments(vec![
            pepinfo_fixture().display().to_string(),
            "--window".to_owned(),
            "3".to_owned(),
            "--plot-contract-out".to_owned(),
            output_path.display().to_string(),
        ]);

        let response = service.invoke(request).expect("pepinfo should execute");
        let emitted =
            std::fs::read_to_string(&output_path).expect("plot contract file should exist");
        assert!(emitted.contains("\"tool\": \"pepinfo\""));
        assert!(emitted.contains("\"method\": \"protein_pepinfo_profile\""));
        assert!(response.result.artifacts.iter().any(|artifact| {
            artifact.id == "pepinfo-plot-contract"
                && artifact.local_path.as_ref() == Some(&output_path)
        }));
        let _ = std::fs::remove_file(output_path);
    }

    #[test]
    fn pepinfo_rejects_invalid_window() {
        let service = implemented_service();
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("pepinfo").expect("tool name should be valid"),
        )
        .with_arguments(vec![
            pepinfo_fixture().display().to_string(),
            "--window".to_owned(),
            "0".to_owned(),
        ]);

        let error = service
            .invoke(request)
            .expect_err("invalid window should fail");
        assert_eq!(error.code(), Some("service.tool.pepinfo.--window_invalid"));
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
                .ends_with("crates/epithema-tools/tests/fixtures/nucleotide_pattern_records.fasta")
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
                .ends_with("crates/epithema-tools/tests/fixtures/protein_stats_records.fasta")
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
    fn executes_psiphi_against_coordinate_fixture() {
        let service = implemented_service();
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("psiphi").expect("tool name should be valid"),
        )
        .with_arguments(vec![psiphi_fixture().display().to_string()]);

        let response = service
            .invoke_psiphi(
                request,
                epithema_tools::protein_coordinates::PSIPHI_DESCRIPTOR,
            )
            .expect("psiphi should execute");
        match &response.result.payload {
            ResultPayload::TableReport(table) => {
                assert_eq!(
                    table.columns,
                    vec![
                        "ordinal",
                        "chain",
                        "residue_name",
                        "residue_number",
                        "insertion_code",
                        "has_backbone_n",
                        "has_backbone_ca",
                        "has_backbone_c",
                        "previous_contiguous",
                        "next_contiguous",
                        "phi_degrees",
                        "psi_degrees",
                    ]
                );
                assert_eq!(table.rows.len(), 3);
                assert!(table.rows.iter().any(|row| {
                    row[0] == "2"
                        && row[1] == "A"
                        && row[2] == "ALA"
                        && row[3] == "2"
                        && row[5] == "true"
                        && row[6] == "true"
                        && row[7] == "true"
                        && row[8] == "true"
                        && row[9] == "true"
                        && row[10] == "143.701451"
                        && row[11] == "165.846421"
                }));
            }
            payload => panic!("unexpected payload: {payload:?}"),
        }
        assert!(
            response.result.summary.lines[0]
                .ends_with("crates/epithema-tools/tests/fixtures/psiphi_backbone.txt")
        );
        assert_eq!(
            response.result.summary.lines[1],
            "Coordinate scope: local PDB ATOM backbone records only"
        );
        assert_eq!(
            response.result.summary.lines[2],
            "Backbone policy: retain only N, CA, and C atoms"
        );
    }

    #[test]
    fn executes_primersearch_against_local_fixtures() {
        let service = implemented_service();
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("primersearch").expect("tool name should be valid"),
        )
        .with_arguments(vec![
            primersearch_fixture().display().to_string(),
            primersearch_pairs_fixture().display().to_string(),
        ]);

        let response = service
            .invoke_primersearch(
                request,
                epithema_tools::primer_tools::PRIMERSEARCH_DESCRIPTOR,
            )
            .expect("primersearch should execute");

        match &response.result.payload {
            ResultPayload::TableReport(table) => {
                assert_eq!(
                    table.columns,
                    vec![
                        "record",
                        "primer_pair",
                        "strand",
                        "left_primer_start",
                        "left_primer_end",
                        "right_primer_start",
                        "right_primer_end",
                        "amplicon_start",
                        "amplicon_end",
                        "amplicon_length",
                        "left_matched",
                        "right_matched",
                    ]
                );
                assert_eq!(table.rows.len(), 2);
                assert_eq!(table.rows[0][0], "targetA");
                assert_eq!(table.rows[0][1], "pair1");
                assert_eq!(table.rows[0][2], "forward");
                assert_eq!(table.rows[1][0], "targetB");
                assert_eq!(table.rows[1][1], "pair2");
                assert_eq!(table.rows[1][2], "reverse");
            }
            payload => panic!("unexpected payload: {payload:?}"),
        }
        assert!(
            response.result.summary.lines[0]
                .ends_with("crates/epithema-tools/tests/fixtures/primersearch_targets.fasta")
        );
        assert!(
            response.result.summary.lines[1]
                .ends_with("crates/epithema-tools/tests/fixtures/primersearch_pairs.tsv")
        );
        assert_eq!(
            response.result.summary.lines[2],
            "Matching scope: exact or IUPAC-ambiguous primer text only"
        );
        assert_eq!(
            response.result.summary.lines[3],
            "Completion policy: report complete primer-pair hits only"
        );
    }

    #[test]
    fn rejects_provider_backed_sequence_input_for_primersearch() {
        let service = implemented_service();
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("primersearch").expect("tool name should be valid"),
        )
        .with_arguments(vec![
            "AB000263".to_owned(),
            primersearch_pairs_fixture().display().to_string(),
        ]);

        let error = service
            .invoke_primersearch(
                request,
                epithema_tools::primer_tools::PRIMERSEARCH_DESCRIPTOR,
            )
            .expect_err("provider-backed target inputs should fail");
        assert!(
            error
                .to_string()
                .contains("provider-backed sequence acquisition")
        );
    }

    #[test]
    fn rejects_invalid_primer_pair_file_for_primersearch() {
        let service = implemented_service();
        let path = std::env::temp_dir().join(format!(
            "epithema-primersearch-service-invalid-{}-{}.tsv",
            std::process::id(),
            std::thread::current().name().unwrap_or("main")
        ));
        std::fs::write(&path, "pair1\tATGC\n").expect("fixture should write");

        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("primersearch").expect("tool name should be valid"),
        )
        .with_arguments(vec![
            primersearch_fixture().display().to_string(),
            path.display().to_string(),
        ]);

        let error = service
            .invoke_primersearch(
                request,
                epithema_tools::primer_tools::PRIMERSEARCH_DESCRIPTOR,
            )
            .expect_err("invalid primer-pair file should fail");
        std::fs::remove_file(path).ok();

        assert!(
            error
                .to_string()
                .contains("must contain exactly 3 tab-delimited fields")
        );
    }

    #[test]
    fn dispatches_primersearch_through_the_governed_service_surface() {
        let service = implemented_service();
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("primersearch").expect("tool name should be valid"),
        )
        .with_arguments(vec![
            primersearch_fixture().display().to_string(),
            primersearch_pairs_fixture().display().to_string(),
        ]);

        let response = service
            .invoke(request)
            .expect("primersearch should dispatch through the governed service");

        assert_eq!(response.tool.as_str(), "primersearch");
    }

    #[test]
    fn executes_wossname_against_governed_tool_metadata() {
        let service = implemented_service();
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("wossname").expect("tool name should be valid"),
        )
        .with_arguments(vec!["pairwise align".to_owned()]);

        let response = service
            .invoke_wossname(request, epithema_tools::command_tools::WOSSNAME_DESCRIPTOR)
            .expect("wossname should execute");

        match &response.result.payload {
            ResultPayload::TableReport(table) => {
                assert_eq!(
                    table.columns,
                    vec![
                        "tool",
                        "family",
                        "short_description",
                        "matched_terms",
                        "matched_fields",
                    ]
                );
                assert_eq!(table.rows.len(), 4);
                assert_eq!(
                    table
                        .rows
                        .iter()
                        .map(|row| row[0].as_str())
                        .collect::<Vec<_>>(),
                    vec!["aligncopypair", "needle", "needleall", "water"]
                );
                assert_eq!(table.rows[0][1], "alignment_tools");
                assert_eq!(table.rows[0][3], "pairwise,align");
                assert_eq!(table.rows[0][4], "tool_name,short_description");
            }
            payload => panic!("unexpected payload: {payload:?}"),
        }
        assert_eq!(response.result.summary.lines[0], "Query: pairwise align");
        assert_eq!(
            response.result.summary.lines[1],
            "Normalized terms: pairwise, align"
        );
        assert_eq!(
            response.result.summary.lines[3],
            "Discovery scope: deterministic local governed-metadata keyword lookup only"
        );
    }

    #[test]
    fn rejects_blank_query_for_wossname() {
        let service = implemented_service();
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("wossname").expect("tool name should be valid"),
        )
        .with_arguments(vec!["   ".to_owned()]);

        let error = service
            .invoke_wossname(request, epithema_tools::command_tools::WOSSNAME_DESCRIPTOR)
            .expect_err("blank queries should fail");
        assert!(
            error
                .to_string()
                .contains("requires at least one keyword query term")
        );
    }

    #[test]
    fn dispatches_wossname_through_the_governed_service_surface() {
        let service = implemented_service();
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("wossname").expect("tool name should be valid"),
        )
        .with_arguments(vec!["pairwise align".to_owned()]);

        let response = service
            .invoke(request)
            .expect("wossname should dispatch through the governed service");

        assert_eq!(response.tool.as_str(), "wossname");
    }

    #[test]
    fn executes_seealso_against_governed_tool_metadata() {
        let service = implemented_service();
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("seealso").expect("tool name should be valid"),
        )
        .with_arguments(vec!["needle".to_owned()]);

        let response = service
            .invoke_seealso(request, epithema_tools::command_tools::SEEALSO_DESCRIPTOR)
            .expect("seealso should execute");

        match &response.result.payload {
            ResultPayload::TableReport(table) => {
                assert_eq!(
                    table.columns,
                    vec![
                        "query_tool",
                        "related_tool",
                        "related_family",
                        "related_short_description",
                        "relationship_terms",
                        "relationship_fields",
                    ]
                );
                assert!(table.rows.iter().any(|row| row[1] == "water"));
                let water = table
                    .rows
                    .iter()
                    .find(|row| row[1] == "water")
                    .expect("water should be related to needle");
                assert_eq!(water[0], "needle");
                assert_eq!(water[2], "pairwise_alignment");
                assert!(water[5].contains("family"));
            }
            payload => panic!("unexpected payload: {payload:?}"),
        }
        assert_eq!(response.result.summary.lines[0], "Query tool: needle");
        assert_eq!(response.result.summary.lines[1], "Resolved tool: needle");
        assert_eq!(
            response.result.summary.lines[4],
            "Discovery scope: deterministic local governed-metadata relationship lookup only"
        );
    }

    #[test]
    fn rejects_unknown_tool_for_seealso() {
        let service = implemented_service();
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("seealso").expect("tool name should be valid"),
        )
        .with_arguments(vec!["missing-tool".to_owned()]);

        let error = service
            .invoke_seealso(request, epithema_tools::command_tools::SEEALSO_DESCRIPTOR)
            .expect_err("unknown tool should fail");
        assert!(error.to_string().contains("could not find governed tool"));
    }

    #[test]
    fn dispatches_seealso_through_the_governed_service_surface() {
        let service = implemented_service();
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("seealso").expect("tool name should be valid"),
        )
        .with_arguments(vec!["needle".to_owned()]);

        let response = service
            .invoke(request)
            .expect("seealso should dispatch through the governed service");
        assert_eq!(response.tool.as_str(), "seealso");
    }

    #[test]
    fn executes_eprimer3_against_local_fixture() {
        let service = implemented_service();
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("eprimer3").expect("tool name should be valid"),
        )
        .with_arguments(vec![eprimer3_fixture().display().to_string()]);

        let response = service
            .invoke_eprimer3(request, epithema_tools::primer_tools::EPRIMER3_DESCRIPTOR)
            .expect("eprimer3 should execute");

        match &response.result.payload {
            ResultPayload::TableReport(table) => {
                assert_eq!(
                    table.columns,
                    vec![
                        "record",
                        "candidate_id",
                        "strand",
                        "oligo_start",
                        "oligo_end",
                        "oligo_length",
                        "oligo_sequence",
                        "canonical_symbols",
                        "ambiguous_symbols",
                        "gc_fraction",
                        "tm_celsius",
                        "three_prime_gc_count",
                    ]
                );
                assert_eq!(table.rows.len(), 24);
                assert_eq!(table.rows[0][0], "ep3targetA");
                assert_eq!(table.rows[0][1], "ep3targetA:forward:8-26");
                assert_eq!(table.rows[0][2], "forward");
                assert_eq!(table.rows[1][1], "ep3targetA:reverse:8-26");
                assert_eq!(table.rows[1][2], "reverse");
            }
            payload => panic!("unexpected payload: {payload:?}"),
        }
        assert!(
            response.result.summary.lines[0]
                .ends_with("crates/epithema-tools/tests/fixtures/eprimer3_targets.fasta")
        );
        assert_eq!(
            response.result.summary.lines[3],
            "Design scope: deterministic local candidate generation only"
        );
    }

    #[test]
    fn rejects_provider_backed_input_for_eprimer3() {
        let service = implemented_service();
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("eprimer3").expect("tool name should be valid"),
        )
        .with_arguments(vec!["AB000263".to_owned()]);

        let error = service
            .invoke_eprimer3(request, epithema_tools::primer_tools::EPRIMER3_DESCRIPTOR)
            .expect_err("provider-backed inputs should fail");
        assert!(
            error
                .to_string()
                .contains("provider-backed sequence acquisition")
        );
    }

    #[test]
    fn dispatches_eprimer3_through_the_governed_service_surface() {
        let service = implemented_service();
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("eprimer3").expect("tool name should be valid"),
        )
        .with_arguments(vec![eprimer3_fixture().display().to_string()]);

        let response = service
            .invoke(request)
            .expect("eprimer3 should dispatch through the governed service");

        assert_eq!(response.tool.as_str(), "eprimer3");
    }

    #[test]
    fn executes_sirna_against_local_fixture() {
        let service = implemented_service();
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("sirna").expect("tool name should be valid"),
        )
        .with_arguments(vec![sirna_fixture().display().to_string()]);

        let response = service
            .invoke_sirna(request, epithema_tools::primer_tools::SIRNA_DESCRIPTOR)
            .expect("sirna should execute");

        match &response.result.payload {
            ResultPayload::TableReport(table) => {
                assert_eq!(
                    table.columns,
                    vec![
                        "record",
                        "candidate_id",
                        "strand",
                        "target_start",
                        "target_end",
                        "duplex_length",
                        "sense_sequence",
                        "guide_sequence",
                        "canonical_symbols",
                        "ambiguous_symbols",
                        "gc_fraction",
                        "guide_five_prime_base",
                        "guide_seed_au_count",
                        "max_homopolymer_run",
                    ]
                );
                assert_eq!(table.rows.len(), 1);
                assert_eq!(table.rows[0][0], "sirnatargetA");
                assert_eq!(table.rows[0][1], "sirna-00001");
                assert_eq!(table.rows[0][2], "forward");
                assert_eq!(table.rows[0][3], "1");
                assert_eq!(table.rows[0][4], "21");
            }
            payload => panic!("unexpected payload: {payload:?}"),
        }
        assert!(
            response.result.summary.lines[0]
                .ends_with("crates/epithema-tools/tests/fixtures/sirna_targets.fasta")
        );
        assert_eq!(
            response.result.summary.lines[3],
            "Design scope: deterministic local siRNA candidate generation only"
        );
    }

    #[test]
    fn rejects_provider_backed_input_for_sirna() {
        let service = implemented_service();
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("sirna").expect("tool name should be valid"),
        )
        .with_arguments(vec!["AB000263".to_owned()]);

        let error = service
            .invoke_sirna(request, epithema_tools::primer_tools::SIRNA_DESCRIPTOR)
            .expect_err("provider-backed inputs should fail");
        assert!(
            error
                .to_string()
                .contains("provider-backed sequence acquisition")
        );
    }

    #[test]
    fn dispatches_sirna_through_the_governed_service_surface() {
        let service = implemented_service();
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("sirna").expect("tool name should be valid"),
        )
        .with_arguments(vec![sirna_fixture().display().to_string()]);

        let response = service
            .invoke(request)
            .expect("sirna should dispatch through the governed service");

        assert_eq!(response.tool.as_str(), "sirna");
    }

    #[test]
    fn rejects_provider_backed_input_for_psiphi() {
        let service = implemented_service();
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("psiphi").expect("tool name should be valid"),
        )
        .with_arguments(vec!["AB000263".to_owned()]);

        let error = service
            .invoke_psiphi(
                request,
                epithema_tools::protein_coordinates::PSIPHI_DESCRIPTOR,
            )
            .expect_err("provider-backed inputs should fail");
        assert!(
            error
                .to_string()
                .contains("provider-backed coordinate acquisition")
        );
    }

    #[test]
    fn rejects_backbone_free_coordinate_fixture_for_psiphi() {
        let service = implemented_service();
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("psiphi").expect("tool name should be valid"),
        )
        .with_arguments(vec![psiphi_invalid_fixture().display().to_string()]);

        let error = service
            .invoke_psiphi(
                request,
                epithema_tools::protein_coordinates::PSIPHI_DESCRIPTOR,
            )
            .expect_err("backbone-free fixture should fail");
        assert!(error.to_string().contains("requires PDB ATOM input"));
    }

    #[test]
    fn dispatches_psiphi_through_the_governed_service_surface() {
        let service = implemented_service();
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("psiphi").expect("tool name should be valid"),
        )
        .with_arguments(vec![psiphi_fixture().display().to_string()]);

        let response = service
            .invoke(request)
            .expect("psiphi should dispatch through the governed service");

        assert_eq!(response.tool.as_str(), "psiphi");
        match &response.result.payload {
            ResultPayload::TableReport(table) => {
                assert_eq!(table.rows.len(), 3);
                assert_eq!(table.rows[1][2], "ALA");
            }
            payload => panic!("unexpected payload: {payload:?}"),
        }
    }

    #[test]
    fn executes_iep_against_protein_fixture() {
        let service = implemented_service();
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("iep").expect("tool name should be valid"),
        )
        .with_arguments(vec![iep_fixture().display().to_string()]);

        let response = service.invoke(request).expect("iep should execute");
        match &response.result.payload {
            ResultPayload::TableReport(table) => {
                assert_eq!(
                    table.columns,
                    vec![
                        "record",
                        "residue_length",
                        "titratable_side_chains",
                        "aspartate",
                        "glutamate",
                        "cysteine",
                        "tyrosine",
                        "histidine",
                        "lysine",
                        "arginine",
                        "net_charge_ph7",
                        "estimated_pi",
                    ]
                );
                assert_eq!(table.rows[0][0], "iep_basic");
                assert_eq!(table.rows[0][1], "4");
                assert_eq!(table.rows[0][8], "4");
                assert_eq!(table.rows[1][0], "iep_mixed");
                assert_eq!(table.rows[1][3], "1");
                assert_eq!(table.rows[1][8], "1");
            }
            payload => panic!("unexpected payload: {payload:?}"),
        }
        assert!(
            response.result.summary.lines[0]
                .ends_with("crates/epithema-tools/tests/fixtures/iep_records.fasta")
        );
        assert_eq!(
            response.result.summary.lines[1],
            "Model: fixed explicit pKa set for termini and D/E/C/Y/H/K/R"
        );
        assert_eq!(
            response.result.summary.lines[2],
            "Net charge reported at pH 7.0"
        );
    }

    #[test]
    fn rejects_nucleotide_input_for_iep() {
        let service = implemented_service();
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("iep").expect("tool name should be valid"),
        )
        .with_arguments(vec![nucleotide_pattern_fixture().display().to_string()]);

        let error = service
            .invoke(request)
            .expect_err("nucleotide input should fail for iep");
        assert!(error.to_string().contains("expects protein input"));
    }

    #[test]
    fn executes_pepdigest_against_protein_fixture() {
        let service = implemented_service();
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("pepdigest").expect("tool name should be valid"),
        )
        .with_arguments(vec![pepdigest_fixture().display().to_string()]);

        let response = service.invoke(request).expect("pepdigest should execute");
        match &response.result.payload {
            ResultPayload::TableReport(table) => {
                assert_eq!(
                    table.columns,
                    vec![
                        "record",
                        "protease",
                        "peptide_index",
                        "start",
                        "end",
                        "cleavage_after",
                        "sequence",
                    ]
                );
                assert_eq!(
                    table.rows[0],
                    vec![
                        "digestA".to_owned(),
                        "trypsin".to_owned(),
                        "1".to_owned(),
                        "1".to_owned(),
                        "2".to_owned(),
                        "2".to_owned(),
                        "AK".to_owned(),
                    ]
                );
                assert_eq!(
                    table.rows[1],
                    vec![
                        "digestA".to_owned(),
                        "trypsin".to_owned(),
                        "2".to_owned(),
                        "3".to_owned(),
                        "7".to_owned(),
                        "7".to_owned(),
                        "RPQMK".to_owned(),
                    ]
                );
                assert_eq!(
                    table.rows[2],
                    vec![
                        "digestB".to_owned(),
                        "trypsin".to_owned(),
                        "1".to_owned(),
                        "1".to_owned(),
                        "4".to_owned(),
                        "4".to_owned(),
                        "MAMK".to_owned(),
                    ]
                );
            }
            payload => panic!("unexpected payload: {payload:?}"),
        }
        assert!(
            response.result.summary.lines[0]
                .ends_with("crates/epithema-tools/tests/fixtures/pepdigest_records.fasta")
        );
        assert_eq!(response.result.summary.lines[1], "Protease: trypsin");
        assert_eq!(
            response.result.summary.lines[2],
            "Digest mode: full deterministic cleavage"
        );
    }

    #[test]
    fn executes_pepdigest_with_cnbr_against_protein_fixture() {
        let service = implemented_service();
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("pepdigest").expect("tool name should be valid"),
        )
        .with_arguments(vec![
            pepdigest_fixture().display().to_string(),
            "--protease".to_owned(),
            "cnbr".to_owned(),
        ]);

        let response = service.invoke(request).expect("pepdigest should execute");
        match &response.result.payload {
            ResultPayload::TableReport(table) => {
                assert_eq!(table.rows[0][1], "cnbr");
                assert_eq!(table.rows[0][6], "AKRPQM");
                assert_eq!(table.rows[1][6], "K");
                assert_eq!(table.rows[2][6], "M");
                assert_eq!(table.rows[3][6], "AM");
                assert_eq!(table.rows[4][6], "K");
            }
            payload => panic!("unexpected payload: {payload:?}"),
        }
    }

    #[test]
    fn rejects_nucleotide_input_for_pepdigest() {
        let service = implemented_service();
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("pepdigest").expect("tool name should be valid"),
        )
        .with_arguments(vec![nucleotide_pattern_fixture().display().to_string()]);

        let error = service
            .invoke(request)
            .expect_err("nucleotide input should fail for pepdigest");
        assert!(error.to_string().contains("expects protein input"));
    }

    #[test]
    fn executes_recoder_against_coding_fixture() {
        let service = implemented_service();
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("recoder").expect("tool name should be valid"),
        )
        .with_arguments(vec![
            recoder_fixture().display().to_string(),
            "GAATTC".to_owned(),
        ]);

        let response = service.invoke(request).expect("recoder should execute");
        match &response.result.payload {
            ResultPayload::TableReport(table) => {
                assert_eq!(
                    table.columns,
                    vec![
                        "record",
                        "site",
                        "occurrence",
                        "site_start",
                        "site_end",
                        "codon_index",
                        "codon_start",
                        "codon_end",
                        "amino_acid",
                        "original_codon",
                        "replacement_codon",
                        "mutated_sequence",
                    ]
                );
                assert_eq!(table.rows.len(), 2);
                assert_eq!(table.rows[0][0], "recoderA");
                assert_eq!(table.rows[0][1], "GAATTC");
                assert_eq!(table.rows[0][3], "7");
                assert_eq!(table.rows[0][9], "GAA");
                assert_eq!(table.rows[0][10], "GAG");
                assert_eq!(table.rows[1][9], "TTC");
                assert_eq!(table.rows[1][10], "TTT");
            }
            payload => panic!("unexpected payload: {payload:?}"),
        }
        assert_eq!(response.result.summary.lines[1], "Site: GAATTC");
    }

    #[test]
    fn executes_silent_against_coding_fixture() {
        let service = implemented_service();
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("silent").expect("tool name should be valid"),
        )
        .with_arguments(vec![
            silent_fixture().display().to_string(),
            "GAATTC".to_owned(),
        ]);

        let response = service.invoke(request).expect("silent should execute");
        match &response.result.payload {
            ResultPayload::TableReport(table) => {
                assert_eq!(
                    table.columns,
                    vec![
                        "record",
                        "site",
                        "site_start",
                        "site_end",
                        "codon_index",
                        "codon_start",
                        "codon_end",
                        "amino_acid",
                        "original_codon",
                        "replacement_codon",
                        "mutated_sequence",
                    ]
                );
                assert_eq!(table.rows.len(), 1);
                assert_eq!(table.rows[0][0], "silentA");
                assert_eq!(table.rows[0][1], "GAATTC");
                assert_eq!(table.rows[0][2], "7");
                assert_eq!(table.rows[0][8], "GAG");
                assert_eq!(table.rows[0][9], "GAA");
            }
            payload => panic!("unexpected payload: {payload:?}"),
        }
        assert_eq!(response.result.summary.lines[1], "Site: GAATTC");
    }

    #[test]
    fn rejects_noncoding_input_for_recoder() {
        let service = implemented_service();
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("recoder").expect("tool name should be valid"),
        )
        .with_arguments(vec![
            nucleotide_pattern_fixture().display().to_string(),
            "GAATTC".to_owned(),
        ]);

        let error = service
            .invoke(request)
            .expect_err("noncoding input should fail for recoder");
        assert!(error.to_string().contains("strict coding DNA input"));
    }

    #[test]
    fn rejects_noncoding_input_for_silent() {
        let service = implemented_service();
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("silent").expect("tool name should be valid"),
        )
        .with_arguments(vec![
            nucleotide_pattern_fixture().display().to_string(),
            "GAATTC".to_owned(),
        ]);

        let error = service
            .invoke(request)
            .expect_err("noncoding input should fail for silent");
        assert!(error.to_string().contains("strict coding DNA input"));
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
    fn executes_cusp_against_coding_fixture() {
        let service = implemented_service();
        let request = InvocationRequest::new(
            ExecutionContext::default(),
            ToolName::new("cusp").expect("tool name should be valid"),
        )
        .with_arguments(vec![codon_reference_fixture().display().to_string()]);

        let response = service.invoke(request).expect("cusp should execute");
        match &response.result.payload {
            ResultPayload::TableReport(table) => {
                assert!(table.rows.iter().any(|row| {
                    row[0] == "record" && row[1] == "ref_pref" && row[2] == "CTT" && row[4] == "3"
                }));
                assert!(table.rows.iter().any(|row| {
                    row[0] == "aggregate" && row[1] == "ALL" && row[2] == "ATG" && row[4] == "2"
                }));
                assert_eq!(table.rows.len(), 61 * 3);
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
        let profile_path = std::env::temp_dir().join("epithema-codon-profile.tsv");
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

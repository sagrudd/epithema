//! Executable acceptance anchors for a small cross-family validation cohort.
//!
//! These anchors turn declared autodoc examples into real executed and compared
//! evidence without widening the scope to a full historical-harvest framework.

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use emboss_core::SequenceRecord;
use emboss_diagnostics::{ErrorCategory, PlatformError};
use emboss_docgen::{LegacyReference, load_document_from_path};
use emboss_io::{write_fasta_string, write_stockholm_string};
use emboss_providers::{HttpRequest, HttpResponse, ProviderHttpClient};
use emboss_service::{
    EmbossService, ExecutionContext, InvocationRequest, ResultPayload, ServiceRegistry,
    ToolCatalog, ToolName,
};
use emboss_tools::governed_tool_descriptors;

use crate::evidence::{
    ComparisonStatus, EvidenceDeclarationStatus, EvidenceSourceKind, ExecutionStatus,
};
use crate::projection::{derive_validation_report, write_validation_report_json};
use crate::report::ToolValidationReport;

/// Stable anchor specification used to execute and compare one accepted case.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct AcceptanceAnchorSpec {
    /// Tool name in the governed registry.
    pub tool_name: &'static str,
    /// Relative path to the committed autodoc contract.
    pub autodoc_contract: &'static str,
    /// Example ID inside the autodoc contract.
    pub example_id: &'static str,
    /// Relative path to the checked-in expected output fixture.
    pub expected_output: &'static str,
    /// Human-readable historical source label.
    pub legacy_source: &'static str,
    /// Historical source locator.
    pub legacy_locator: &'static str,
    /// Historical invocation form associated with the example.
    pub legacy_invocation: &'static str,
}

#[derive(Debug)]
struct AnchorActualResult {
    payload: String,
    plot: Option<String>,
}

const ACCEPTANCE_ANCHORS: &[AcceptanceAnchorSpec] = &[
    AcceptanceAnchorSpec {
        tool_name: "needle",
        autodoc_contract: "docs/autodoc/tools/needle.json",
        example_id: "basic_alignment",
        expected_output: "crates/emboss-testkit/tests/fixtures/acceptance_anchors/needle_basic_alignment.sto",
        legacy_source: "EMBOSS needle application",
        legacy_locator: "https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/needle.acd",
        legacy_invocation: "needle -asequence needle_query.fasta -bsequence needle_target.fasta -gapopen 10 -gapextend 0.5",
    },
    AcceptanceAnchorSpec {
        tool_name: "seqret",
        autodoc_contract: "docs/autodoc/tools/seqret.json",
        example_id: "normalize_local_fasta_records",
        expected_output: "crates/emboss-testkit/tests/fixtures/acceptance_anchors/seqret_normalize_local_fasta_records.fasta",
        legacy_source: "EMBOSS seqret application",
        legacy_locator: "https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/seqret.acd",
        legacy_invocation: "seqret -sequence three_records.fasta -outseq stdout",
    },
    AcceptanceAnchorSpec {
        tool_name: "seqretsetall",
        autodoc_contract: "docs/autodoc/tools/seqretsetall.json",
        example_id: "normalize_multiple_sequence_inputs_into_ordered_partitions",
        expected_output: "crates/emboss-testkit/tests/fixtures/acceptance_anchors/seqretsetall_normalize_multiple_sequence_inputs_into_ordered_partitions.txt",
        legacy_source: "EMBOSS seqretsetall application",
        legacy_locator: "https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/seqretsetall.acd",
        legacy_invocation: "seqretsetall -sequence three_records.fasta two_records.fasta -outseq stdout",
    },
    AcceptanceAnchorSpec {
        tool_name: "seqretsplit",
        autodoc_contract: "docs/autodoc/tools/seqretsplit.json",
        example_id: "normalize_one_sequence_input_into_deterministic_split_partitions",
        expected_output: "crates/emboss-testkit/tests/fixtures/acceptance_anchors/seqretsplit_normalize_one_sequence_input_into_deterministic_split_partitions.txt",
        legacy_source: "EMBOSS seqretsplit application",
        legacy_locator: "https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/seqretsplit.acd",
        legacy_invocation: "seqretsplit -sequence three_records.fasta -outseq stdout",
    },
    AcceptanceAnchorSpec {
        tool_name: "refseqget",
        autodoc_contract: "docs/autodoc/tools/refseqget.json",
        example_id: "retrieve_provider_qualified_reference_sequence",
        expected_output: "crates/emboss-testkit/tests/fixtures/acceptance_anchors/refseqget_retrieve_provider_qualified_reference_sequence.fasta",
        legacy_source: "EMBOSS refseqget application",
        legacy_locator: "https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/refseqget.acd",
        legacy_invocation: "refseqget -sequence ncbi:protein:NP_000537.3 -outseq stdout",
    },
    AcceptanceAnchorSpec {
        tool_name: "runinfo",
        autodoc_contract: "docs/autodoc/tools/runinfo.json",
        example_id: "normalize_ena_run_metadata",
        expected_output: "crates/emboss-testkit/tests/fixtures/acceptance_anchors/runinfo_normalize_ena_run_metadata.tsv",
        legacy_source: "EMBOSS runinfo application",
        legacy_locator: "https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/runinfo.acd",
        legacy_invocation: "runinfo -sequence ena:ERR123456 -stdout yes",
    },
    AcceptanceAnchorSpec {
        tool_name: "runget",
        autodoc_contract: "docs/autodoc/tools/runget.json",
        example_id: "report_ena_run_manifest",
        expected_output: "crates/emboss-testkit/tests/fixtures/acceptance_anchors/runget_report_ena_run_manifest.tsv",
        legacy_source: "EMBOSS runget application",
        legacy_locator: "https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/runget.acd",
        legacy_invocation: "runget -sequence ena:ERR123456 -stdout yes",
    },
    AcceptanceAnchorSpec {
        tool_name: "infoassembly",
        autodoc_contract: "docs/autodoc/tools/infoassembly.json",
        example_id: "normalize_ena_assembly_metadata",
        expected_output: "crates/emboss-testkit/tests/fixtures/acceptance_anchors/infoassembly_normalize_ena_assembly_metadata.tsv",
        legacy_source: "EMBOSS infoassembly application",
        legacy_locator: "https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/infoassembly.acd",
        legacy_invocation: "infoassembly -sequence ena:ERR123456 -stdout yes",
    },
    AcceptanceAnchorSpec {
        tool_name: "psiphi",
        autodoc_contract: "docs/autodoc/tools/psiphi.json",
        example_id: "psiphi_profile_example",
        expected_output: "crates/emboss-testkit/tests/fixtures/acceptance_anchors/psiphi_psiphi_profile_example.tsv",
        legacy_source: "EMBOSS psiphi application",
        legacy_locator: "https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/psiphi.acd",
        legacy_invocation: "psiphi -infile psiphi_backbone.txt -outfile stdout",
    },
    AcceptanceAnchorSpec {
        tool_name: "eprimer3",
        autodoc_contract: "docs/autodoc/tools/eprimer3.json",
        example_id: "eprimer3_candidates_example",
        expected_output: "crates/emboss-testkit/tests/fixtures/acceptance_anchors/eprimer3_generate_deterministic_bounded_primer_candidates.tsv",
        legacy_source: "EMBOSS eprimer3 application",
        legacy_locator: "https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/eprimer3.acd",
        legacy_invocation: "eprimer3 -sequence eprimer3_targets.fasta -outfile stdout",
    },
    AcceptanceAnchorSpec {
        tool_name: "primersearch",
        autodoc_contract: "docs/autodoc/tools/primersearch.json",
        example_id: "primersearch_hits_example",
        expected_output: "crates/emboss-testkit/tests/fixtures/acceptance_anchors/primersearch_search_one_local_nucleotide_input_for_deterministic_complete_primer_pair_hits.tsv",
        legacy_source: "EMBOSS primersearch application",
        legacy_locator: "https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/primersearch.acd",
        legacy_invocation: "primersearch -seqall primersearch_targets.fasta -infile primersearch_pairs.tsv -outfile stdout",
    },
    AcceptanceAnchorSpec {
        tool_name: "sirna",
        autodoc_contract: "docs/autodoc/tools/sirna.json",
        example_id: "sirna_candidates_example",
        expected_output: "crates/emboss-testkit/tests/fixtures/acceptance_anchors/sirna_generate_deterministic_bounded_sirna_candidates.tsv",
        legacy_source: "EMBOSS sirna application",
        legacy_locator: "https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/sirna.acd",
        legacy_invocation: "sirna -sequence sirna_targets.fasta -outfile stdout",
    },
    AcceptanceAnchorSpec {
        tool_name: "wossname",
        autodoc_contract: "docs/autodoc/tools/wossname.json",
        example_id: "wossname_keyword_matches_example",
        expected_output: "crates/emboss-testkit/tests/fixtures/acceptance_anchors/wossname_report_deterministic_keyword_matches_against_governed_local_tool_metadata.tsv",
        legacy_source: "EMBOSS wossname application",
        legacy_locator: "https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/wossname.acd",
        legacy_invocation: "wossname -search \"pairwise align\" -outfile stdout",
    },
    AcceptanceAnchorSpec {
        tool_name: "seealso",
        autodoc_contract: "docs/autodoc/tools/seealso.json",
        example_id: "seealso_related_programs_example",
        expected_output: "crates/emboss-testkit/tests/fixtures/acceptance_anchors/seealso_report_deterministic_related_program_rows_for_a_governed_local_tool.tsv",
        legacy_source: "EMBOSS seealso application",
        legacy_locator: "https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/seealso.acd",
        legacy_invocation: "seealso -program needle -outfile stdout",
    },
    AcceptanceAnchorSpec {
        tool_name: "newseq",
        autodoc_contract: "docs/autodoc/tools/newseq.json",
        example_id: "create_dna_record",
        expected_output: "crates/emboss-testkit/tests/fixtures/acceptance_anchors/newseq_create_dna_record.fasta",
        legacy_source: "EMBOSS newseq application",
        legacy_locator: "https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/newseq.acd",
        legacy_invocation: "newseq -name created -sequence ACGTAC -desc 'created example' -type dna -outseq stdout",
    },
    AcceptanceAnchorSpec {
        tool_name: "makenucseq",
        autodoc_contract: "docs/autodoc/tools/makenucseq.json",
        example_id: "generate_two_rna_records",
        expected_output: "crates/emboss-testkit/tests/fixtures/acceptance_anchors/makenucseq_generate_two_rna_records.fasta",
        legacy_source: "EMBOSS makenucseq application",
        legacy_locator: "https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/makenucseq.acd",
        legacy_invocation: "makenucseq -name made_nuc -length 6 -number 2 -seed 7 -type rna -outseq stdout",
    },
    AcceptanceAnchorSpec {
        tool_name: "makeprotseq",
        autodoc_contract: "docs/autodoc/tools/makeprotseq.json",
        example_id: "generate_one_protein_record",
        expected_output: "crates/emboss-testkit/tests/fixtures/acceptance_anchors/makeprotseq_generate_one_protein_record.fasta",
        legacy_source: "EMBOSS makeprotseq application",
        legacy_locator: "https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/makeprotseq.acd",
        legacy_invocation: "makeprotseq -name made_prot -length 5 -seed 9 -outseq stdout",
    },
    AcceptanceAnchorSpec {
        tool_name: "seqcount",
        autodoc_contract: "docs/autodoc/tools/seqcount.json",
        example_id: "count_three_fasta_records",
        expected_output: "crates/emboss-testkit/tests/fixtures/acceptance_anchors/seqcount_count_three_fasta_records.tsv",
        legacy_source: "EMBOSS seqcount application",
        legacy_locator: "https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/seqcount.acd",
        legacy_invocation: "seqcount -sequence three_records.fasta -stdout yes",
    },
    AcceptanceAnchorSpec {
        tool_name: "notseq",
        autodoc_contract: "docs/autodoc/tools/notseq.json",
        example_id: "exclude_second_record",
        expected_output: "crates/emboss-testkit/tests/fixtures/acceptance_anchors/notseq_exclude_second_record.fasta",
        legacy_source: "EMBOSS notseq application",
        legacy_locator: "https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/notseq.acd",
        legacy_invocation: "notseq -sequence three_records.fasta -exclude 2 -outseq stdout",
    },
    AcceptanceAnchorSpec {
        tool_name: "nthseq",
        autodoc_contract: "docs/autodoc/tools/nthseq.json",
        example_id: "select_second_record",
        expected_output: "crates/emboss-testkit/tests/fixtures/acceptance_anchors/nthseq_select_second_record.fasta",
        legacy_source: "EMBOSS nthseq application",
        legacy_locator: "https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/nthseq.acd",
        legacy_invocation: "nthseq -sequence three_records.fasta -number 2 -outseq stdout",
    },
    AcceptanceAnchorSpec {
        tool_name: "skipseq",
        autodoc_contract: "docs/autodoc/tools/skipseq.json",
        example_id: "skip_first_record",
        expected_output: "crates/emboss-testkit/tests/fixtures/acceptance_anchors/skipseq_skip_first_record.fasta",
        legacy_source: "EMBOSS skipseq application",
        legacy_locator: "https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/skipseq.acd",
        legacy_invocation: "skipseq -sequence three_records.fasta -skip 1 -outseq stdout",
    },
    AcceptanceAnchorSpec {
        tool_name: "listor",
        autodoc_contract: "docs/autodoc/tools/listor.json",
        example_id: "xor_two_sequence_sets",
        expected_output: "crates/emboss-testkit/tests/fixtures/acceptance_anchors/listor_xor_two_sequence_sets.fasta",
        legacy_source: "EMBOSS listor application",
        legacy_locator: "https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/listor.acd",
        legacy_invocation: "listor -first listor_first.fasta -second listor_second.fasta -operator xor -outseq stdout",
    },
    AcceptanceAnchorSpec {
        tool_name: "skipredundant",
        autodoc_contract: "docs/autodoc/tools/skipredundant.json",
        example_id: "remove_exact_duplicate_records",
        expected_output: "crates/emboss-testkit/tests/fixtures/acceptance_anchors/skipredundant_remove_exact_duplicate_records.fasta",
        legacy_source: "EMBOSS skipredundant application",
        legacy_locator: "https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/skipredundant.acd",
        legacy_invocation: "skipredundant -sequence skipredundant_records.fasta -outseq stdout",
    },
    AcceptanceAnchorSpec {
        tool_name: "extractfeat",
        autodoc_contract: "docs/autodoc/tools/extractfeat.json",
        example_id: "extract_selected_gene_feature",
        expected_output: "crates/emboss-testkit/tests/fixtures/acceptance_anchors/extractfeat_extract_selected_gene_feature.fasta",
        legacy_source: "EMBOSS extractfeat application",
        legacy_locator: "https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/extractfeat.acd",
        legacy_invocation: "extractfeat -sequence annotated_feature.gbk -type gene -outseq stdout",
    },
    AcceptanceAnchorSpec {
        tool_name: "featcopy",
        autodoc_contract: "docs/autodoc/tools/featcopy.json",
        example_id: "copy_selected_gene_feature_to_matching_target",
        expected_output: "crates/emboss-testkit/tests/fixtures/acceptance_anchors/featcopy_copy_selected_gene_feature_to_matching_target.fasta",
        legacy_source: "EMBOSS featcopy application",
        legacy_locator: "https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/featcopy.acd",
        legacy_invocation: "featcopy -source annotated_feature.gbk -target featcopy_target.fasta -type gene -outseq stdout",
    },
    AcceptanceAnchorSpec {
        tool_name: "maskfeat",
        autodoc_contract: "docs/autodoc/tools/maskfeat.json",
        example_id: "mask_selected_gene_feature",
        expected_output: "crates/emboss-testkit/tests/fixtures/acceptance_anchors/maskfeat_mask_selected_gene_feature.fasta",
        legacy_source: "EMBOSS maskfeat application",
        legacy_locator: "https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/maskfeat.acd",
        legacy_invocation: "maskfeat -sequence annotated_feature.gbk -type gene -outseq stdout",
    },
    AcceptanceAnchorSpec {
        tool_name: "coderet",
        autodoc_contract: "docs/autodoc/tools/coderet.json",
        example_id: "translate_default_cds_selection",
        expected_output: "crates/emboss-testkit/tests/fixtures/acceptance_anchors/coderet_translate_default_cds_selection.fasta",
        legacy_source: "EMBOSS coderet application",
        legacy_locator: "https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/coderet.acd",
        legacy_invocation: "coderet -sequence annotated_feature.gbk -translate -outseq stdout",
    },
    AcceptanceAnchorSpec {
        tool_name: "featmerge",
        autodoc_contract: "docs/autodoc/tools/featmerge.json",
        example_id: "merge_right_annotations",
        expected_output: "crates/emboss-testkit/tests/fixtures/acceptance_anchors/featmerge_merge_right_annotations.fasta",
        legacy_source: "EMBOSS featmerge application",
        legacy_locator: "https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/featmerge.acd",
        legacy_invocation: "featmerge -left annotated_feature.gbk -right annotated_merge_right.gbk -outseq stdout",
    },
    AcceptanceAnchorSpec {
        tool_name: "featreport",
        autodoc_contract: "docs/autodoc/tools/featreport.json",
        example_id: "report_all_features",
        expected_output: "crates/emboss-testkit/tests/fixtures/acceptance_anchors/featreport_report_all_features.tsv",
        legacy_source: "EMBOSS featreport application",
        legacy_locator: "https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/featreport.acd",
        legacy_invocation: "featreport -sequence annotated_feature.gbk -outfile stdout",
    },
    AcceptanceAnchorSpec {
        tool_name: "feattext",
        autodoc_contract: "docs/autodoc/tools/feattext.json",
        example_id: "render_normalized_feature_text",
        expected_output: "crates/emboss-testkit/tests/fixtures/acceptance_anchors/feattext_render_normalized_feature_text.txt",
        legacy_source: "EMBOSS feattext application",
        legacy_locator: "https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/feattext.acd",
        legacy_invocation: "feattext -sequence annotated_feature.gbk -outfile stdout",
    },
    AcceptanceAnchorSpec {
        tool_name: "splitsource",
        autodoc_contract: "docs/autodoc/tools/splitsource.json",
        example_id: "split_record_by_source_features",
        expected_output: "crates/emboss-testkit/tests/fixtures/acceptance_anchors/splitsource_split_record_by_source_features.fasta",
        legacy_source: "EMBOSS splitsource application",
        legacy_locator: "https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/splitsource.acd",
        legacy_invocation: "splitsource -sequence splitsource_annotated.gbk -outseq stdout",
    },
    AcceptanceAnchorSpec {
        tool_name: "twofeat",
        autodoc_contract: "docs/autodoc/tools/twofeat.json",
        example_id: "report_gene_cds_neighbour_pair",
        expected_output: "crates/emboss-testkit/tests/fixtures/acceptance_anchors/twofeat_report_gene_cds_neighbour_pair.tsv",
        legacy_source: "EMBOSS twofeat application",
        legacy_locator: "https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/twofeat.acd",
        legacy_invocation: "twofeat -sequence annotated_feature.gbk -atype gene -btype cds -stdout yes",
    },
    AcceptanceAnchorSpec {
        tool_name: "maskseq",
        autodoc_contract: "docs/autodoc/tools/maskseq.json",
        example_id: "mask_positions_two_to_three",
        expected_output: "crates/emboss-testkit/tests/fixtures/acceptance_anchors/maskseq_mask_positions_two_to_three.fasta",
        legacy_source: "EMBOSS maskseq application",
        legacy_locator: "https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/maskseq.acd",
        legacy_invocation: "maskseq -sequence three_records.fasta -regions 2:3 -outseq stdout",
    },
    AcceptanceAnchorSpec {
        tool_name: "biosed",
        autodoc_contract: "docs/autodoc/tools/biosed.json",
        example_id: "replace_interval_across_records",
        expected_output: "crates/emboss-testkit/tests/fixtures/acceptance_anchors/biosed_replace_interval_across_records.fasta",
        legacy_source: "EMBOSS biosed application",
        legacy_locator: "https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/biosed.acd",
        legacy_invocation: "biosed -sequence biosed_records.fasta -start 2 -end 3 -replace NN -outseq stdout",
    },
    AcceptanceAnchorSpec {
        tool_name: "degapseq",
        autodoc_contract: "docs/autodoc/tools/degapseq.json",
        example_id: "remove_gap_markers",
        expected_output: "crates/emboss-testkit/tests/fixtures/acceptance_anchors/degapseq_remove_gap_markers.fasta",
        legacy_source: "EMBOSS degapseq application",
        legacy_locator: "https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/degapseq.acd",
        legacy_invocation: "degapseq -sequence gapped_records.fasta -outseq stdout",
    },
    AcceptanceAnchorSpec {
        tool_name: "revseq",
        autodoc_contract: "docs/autodoc/tools/revseq.json",
        example_id: "auto_reverse_complement_fixture",
        expected_output: "crates/emboss-testkit/tests/fixtures/acceptance_anchors/revseq_auto_reverse_complement_fixture.fasta",
        legacy_source: "EMBOSS revseq application",
        legacy_locator: "https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/revseq.acd",
        legacy_invocation: "revseq -sequence three_records.fasta -outseq stdout",
    },
    AcceptanceAnchorSpec {
        tool_name: "msbar",
        autodoc_contract: "docs/autodoc/tools/msbar.json",
        example_id: "apply_point_mutations",
        expected_output: "crates/emboss-testkit/tests/fixtures/acceptance_anchors/msbar_apply_point_mutations.fasta",
        legacy_source: "EMBOSS msbar application",
        legacy_locator: "https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/msbar.acd",
        legacy_invocation: "msbar -sequence msbar_records.fasta -mutation 2:T -mutation 4:A -outseq stdout",
    },
    AcceptanceAnchorSpec {
        tool_name: "trimest",
        autodoc_contract: "docs/autodoc/tools/trimest.json",
        example_id: "trim_terminal_poly_a",
        expected_output: "crates/emboss-testkit/tests/fixtures/acceptance_anchors/trimest_trim_terminal_poly_a.fasta",
        legacy_source: "EMBOSS trimest application",
        legacy_locator: "https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/trimest.acd",
        legacy_invocation: "trimest -sequence trimest_records.fasta -minpoly 4 -outseq stdout",
    },
    AcceptanceAnchorSpec {
        tool_name: "trimseq",
        autodoc_contract: "docs/autodoc/tools/trimseq.json",
        example_id: "trim_one_residue_from_each_end",
        expected_output: "crates/emboss-testkit/tests/fixtures/acceptance_anchors/trimseq_trim_one_residue_from_each_end.fasta",
        legacy_source: "EMBOSS trimseq application",
        legacy_locator: "https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/trimseq.acd",
        legacy_invocation: "trimseq -sequence three_records.fasta -left 1 -right 1 -outseq stdout",
    },
    AcceptanceAnchorSpec {
        tool_name: "vectorstrip",
        autodoc_contract: "docs/autodoc/tools/vectorstrip.json",
        example_id: "strip_exact_terminal_vector_matches",
        expected_output: "crates/emboss-testkit/tests/fixtures/acceptance_anchors/vectorstrip_strip_exact_terminal_vector_matches.fasta",
        legacy_source: "EMBOSS vectorstrip application",
        legacy_locator: "https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/vectorstrip.acd",
        legacy_invocation: "vectorstrip -sequence vectorstrip_records.fasta -vector vectorstrip_vector.fasta -outseq stdout",
    },
    AcceptanceAnchorSpec {
        tool_name: "maskambignuc",
        autodoc_contract: "docs/autodoc/tools/maskambignuc.json",
        example_id: "mask_nucleotide_ambiguities",
        expected_output: "crates/emboss-testkit/tests/fixtures/acceptance_anchors/maskambignuc_mask_nucleotide_ambiguities.fasta",
        legacy_source: "EMBOSS maskambignuc application",
        legacy_locator: "https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/maskambignuc.acd",
        legacy_invocation: "maskambignuc -sequence ambiguous_nucleotide_records.fasta -outseq stdout",
    },
    AcceptanceAnchorSpec {
        tool_name: "maskambigprot",
        autodoc_contract: "docs/autodoc/tools/maskambigprot.json",
        example_id: "mask_protein_ambiguities",
        expected_output: "crates/emboss-testkit/tests/fixtures/acceptance_anchors/maskambigprot_mask_protein_ambiguities.fasta",
        legacy_source: "EMBOSS maskambigprot application",
        legacy_locator: "https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/maskambigprot.acd",
        legacy_invocation: "maskambigprot -sequence ambiguous_protein_records.fasta -outseq stdout",
    },
    AcceptanceAnchorSpec {
        tool_name: "extractseq",
        autodoc_contract: "docs/autodoc/tools/extractseq.json",
        example_id: "extract_region_two_to_three",
        expected_output: "crates/emboss-testkit/tests/fixtures/acceptance_anchors/extractseq_extract_region_two_to_three.fasta",
        legacy_source: "EMBOSS extractseq application",
        legacy_locator: "https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/extractseq.acd",
        legacy_invocation: "extractseq -sequence three_records.fasta -regions 2:3 -outseq stdout",
    },
    AcceptanceAnchorSpec {
        tool_name: "cutseq",
        autodoc_contract: "docs/autodoc/tools/cutseq.json",
        example_id: "cut_after_second_position",
        expected_output: "crates/emboss-testkit/tests/fixtures/acceptance_anchors/cutseq_cut_after_second_position.fasta",
        legacy_source: "EMBOSS cutseq application",
        legacy_locator: "https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/cutseq.acd",
        legacy_invocation: "cutseq -sequence three_records.fasta -position 2 -outseq stdout",
    },
    AcceptanceAnchorSpec {
        tool_name: "union",
        autodoc_contract: "docs/autodoc/tools/union.json",
        example_id: "concatenate_two_sequence_inputs",
        expected_output: "crates/emboss-testkit/tests/fixtures/acceptance_anchors/union_concatenate_two_sequence_inputs.fasta",
        legacy_source: "EMBOSS union application",
        legacy_locator: "https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/union.acd",
        legacy_invocation: "union -first three_records.fasta -second two_records.fasta -outseq stdout",
    },
    AcceptanceAnchorSpec {
        tool_name: "pasteseq",
        autodoc_contract: "docs/autodoc/tools/pasteseq.json",
        example_id: "insert_short_sequence_after_position_two",
        expected_output: "crates/emboss-testkit/tests/fixtures/acceptance_anchors/pasteseq_insert_short_sequence_after_position_two.fasta",
        legacy_source: "EMBOSS pasteseq application",
        legacy_locator: "https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/pasteseq.acd",
        legacy_invocation: "pasteseq -asequence pasteseq_main.fasta -bsequence pasteseq_insert.fasta -position 2 -outseq stdout",
    },
    AcceptanceAnchorSpec {
        tool_name: "splitter",
        autodoc_contract: "docs/autodoc/tools/splitter.json",
        example_id: "split_three_records_into_two_partitions",
        expected_output: "crates/emboss-testkit/tests/fixtures/acceptance_anchors/splitter_split_three_records_into_two_partitions.txt",
        legacy_source: "EMBOSS splitter application",
        legacy_locator: "https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/splitter.acd",
        legacy_invocation: "splitter -sequence three_records.fasta -size 2 -stdout yes",
    },
    AcceptanceAnchorSpec {
        tool_name: "merger",
        autodoc_contract: "docs/autodoc/tools/merger.json",
        example_id: "merge_two_overlapping_records",
        expected_output: "crates/emboss-testkit/tests/fixtures/acceptance_anchors/merger_merge_two_overlapping_records.fasta",
        legacy_source: "EMBOSS merger application",
        legacy_locator: "https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/merger.acd",
        legacy_invocation: "merger -asequence merger_left.fasta -bsequence merger_right.fasta -outseq stdout",
    },
    AcceptanceAnchorSpec {
        tool_name: "megamerger",
        autodoc_contract: "docs/autodoc/tools/megamerger.json",
        example_id: "merge_two_overlapping_dna_records",
        expected_output: "crates/emboss-testkit/tests/fixtures/acceptance_anchors/megamerger_merge_two_overlapping_dna_records.fasta",
        legacy_source: "EMBOSS megamerger application",
        legacy_locator: "https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/megamerger.acd",
        legacy_invocation: "megamerger -asequence merger_left.fasta -bsequence merger_right.fasta -outseq stdout",
    },
    AcceptanceAnchorSpec {
        tool_name: "sizeseq",
        autodoc_contract: "docs/autodoc/tools/sizeseq.json",
        example_id: "sort_records_by_descending_size",
        expected_output: "crates/emboss-testkit/tests/fixtures/acceptance_anchors/sizeseq_sort_records_by_descending_size.fasta",
        legacy_source: "EMBOSS sizeseq application",
        legacy_locator: "https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/sizeseq.acd",
        legacy_invocation: "sizeseq -sequence sizeseq_records.fasta -outseq stdout",
    },
    AcceptanceAnchorSpec {
        tool_name: "shuffleseq",
        autodoc_contract: "docs/autodoc/tools/shuffleseq.json",
        example_id: "shuffle_records_with_seed_7",
        expected_output: "crates/emboss-testkit/tests/fixtures/acceptance_anchors/shuffleseq_shuffle_records_with_seed_7.fasta",
        legacy_source: "EMBOSS shuffleseq application",
        legacy_locator: "https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/shuffleseq.acd",
        legacy_invocation: "shuffleseq -sequence three_records.fasta -seed 7 -outseq stdout",
    },
    AcceptanceAnchorSpec {
        tool_name: "backtranseq",
        autodoc_contract: "docs/autodoc/tools/backtranseq.json",
        example_id: "representative_backtranslation",
        expected_output: "crates/emboss-testkit/tests/fixtures/acceptance_anchors/backtranseq_representative_backtranslation.fasta",
        legacy_source: "EMBOSS backtranseq application",
        legacy_locator: "https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/backtranseq.acd",
        legacy_invocation: "backtranseq -sequence protein_stats_records.fasta -outseq stdout",
    },
    AcceptanceAnchorSpec {
        tool_name: "backtranambig",
        autodoc_contract: "docs/autodoc/tools/backtranambig.json",
        example_id: "ambiguous_backtranslation",
        expected_output: "crates/emboss-testkit/tests/fixtures/acceptance_anchors/backtranambig_ambiguous_backtranslation.fasta",
        legacy_source: "EMBOSS backtranambig application",
        legacy_locator: "https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/backtranambig.acd",
        legacy_invocation: "backtranambig -sequence protein_stats_records.fasta -outseq stdout",
    },
    AcceptanceAnchorSpec {
        tool_name: "checktrans",
        autodoc_contract: "docs/autodoc/tools/checktrans.json",
        example_id: "compare_matching_translation_pair",
        expected_output: "crates/emboss-testkit/tests/fixtures/acceptance_anchors/checktrans_compare_matching_translation_pair.tsv",
        legacy_source: "EMBOSS checktrans application",
        legacy_locator: "https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/checktrans.acd",
        legacy_invocation: "checktrans -sequence checktrans_nucleotide.fasta -translation checktrans_protein.fasta -stdout yes",
    },
    AcceptanceAnchorSpec {
        tool_name: "transeq",
        autodoc_contract: "docs/autodoc/tools/transeq.json",
        example_id: "forward_frame_one_translation",
        expected_output: "crates/emboss-testkit/tests/fixtures/acceptance_anchors/transeq_forward_frame_one_translation.fasta",
        legacy_source: "EMBOSS transeq application",
        legacy_locator: "https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/transeq.acd",
        legacy_invocation: "transeq -sequence checktrans_nucleotide.fasta -frame 1 -outseq stdout",
    },
    AcceptanceAnchorSpec {
        tool_name: "getorf",
        autodoc_contract: "docs/autodoc/tools/getorf.json",
        example_id: "extract_stop_bounded_forward_orfs",
        expected_output: "crates/emboss-testkit/tests/fixtures/acceptance_anchors/getorf_extract_stop_bounded_forward_orfs.fasta",
        legacy_source: "EMBOSS getorf application",
        legacy_locator: "https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/getorf.acd",
        legacy_invocation: "getorf -sequence getorf_records.fasta -outseq stdout",
    },
    AcceptanceAnchorSpec {
        tool_name: "prettyseq",
        autodoc_contract: "docs/autodoc/tools/prettyseq.json",
        example_id: "render_forward_frame_report",
        expected_output: "crates/emboss-testkit/tests/fixtures/acceptance_anchors/prettyseq_render_forward_frame_report.txt",
        legacy_source: "EMBOSS prettyseq application",
        legacy_locator: "https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/prettyseq.acd",
        legacy_invocation: "prettyseq -sequence checktrans_nucleotide.fasta -frame 1 -stdout yes",
    },
    AcceptanceAnchorSpec {
        tool_name: "tranalign",
        autodoc_contract: "docs/autodoc/tools/tranalign.json",
        example_id: "project_protein_alignment_to_codons",
        expected_output: "crates/emboss-testkit/tests/fixtures/acceptance_anchors/tranalign_project_protein_alignment_to_codons.sto",
        legacy_source: "EMBOSS tranalign application",
        legacy_locator: "https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/tranalign.acd",
        legacy_invocation: "tranalign -asequence tranalign_protein_alignment.sto -bsequence checktrans_nucleotide.fasta -outseq stdout",
    },
    AcceptanceAnchorSpec {
        tool_name: "aligncopy",
        autodoc_contract: "docs/autodoc/tools/aligncopy.json",
        example_id: "copy_multiple_alignment_stockholm",
        expected_output: "crates/emboss-testkit/tests/fixtures/acceptance_anchors/aligncopy_copy_multiple_alignment_stockholm.sto",
        legacy_source: "EMBOSS aligncopy application",
        legacy_locator: "https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/aligncopy.acd",
        legacy_invocation: "aligncopy -sequence multiple_alignment.sto -outseq stdout",
    },
    AcceptanceAnchorSpec {
        tool_name: "aligncopypair",
        autodoc_contract: "docs/autodoc/tools/aligncopypair.json",
        example_id: "copy_pairwise_alignment_stockholm",
        expected_output: "crates/emboss-testkit/tests/fixtures/acceptance_anchors/aligncopypair_copy_pairwise_alignment_stockholm.sto",
        legacy_source: "EMBOSS aligncopypair application",
        legacy_locator: "https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/aligncopypair.acd",
        legacy_invocation: "aligncopypair -sequence pairwise_alignment.sto -outseq stdout",
    },
    AcceptanceAnchorSpec {
        tool_name: "infoalign",
        autodoc_contract: "docs/autodoc/tools/infoalign.json",
        example_id: "summarize_multiple_alignment_statistics",
        expected_output: "crates/emboss-testkit/tests/fixtures/acceptance_anchors/infoalign_summarize_multiple_alignment_statistics.tsv",
        legacy_source: "EMBOSS infoalign application",
        legacy_locator: "https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/infoalign.acd",
        legacy_invocation: "infoalign -sequence multiple_alignment.sto -stdout yes",
    },
    AcceptanceAnchorSpec {
        tool_name: "extractalign",
        autodoc_contract: "docs/autodoc/tools/extractalign.json",
        example_id: "extract_selected_rows_and_columns",
        expected_output: "crates/emboss-testkit/tests/fixtures/acceptance_anchors/extractalign_extract_selected_rows_and_columns.sto",
        legacy_source: "EMBOSS extractalign application",
        legacy_locator: "https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/extractalign.acd",
        legacy_invocation: "extractalign -sequence multiple_alignment.sto -rowid alpha -row 3 -start 2 -end 4 -outseq stdout",
    },
    AcceptanceAnchorSpec {
        tool_name: "nthseqset",
        autodoc_contract: "docs/autodoc/tools/nthseqset.json",
        example_id: "select_second_alignment_set",
        expected_output: "crates/emboss-testkit/tests/fixtures/acceptance_anchors/nthseqset_select_second_alignment_set.sto",
        legacy_source: "EMBOSS nthseqset application",
        legacy_locator: "https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/nthseqset.acd",
        legacy_invocation: "nthseqset -sequence nthseqset_alignments.sto -number 2 -outseq stdout",
    },
    AcceptanceAnchorSpec {
        tool_name: "matcher",
        autodoc_contract: "docs/autodoc/tools/matcher.json",
        example_id: "compare_singleton_sequences_without_gaps",
        expected_output: "crates/emboss-testkit/tests/fixtures/acceptance_anchors/matcher_compare_singleton_sequences_without_gaps.tsv",
        legacy_source: "EMBOSS matcher application",
        legacy_locator: "https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/matcher.acd",
        legacy_invocation: "matcher -asequence needle_query.fasta -bsequence needle_target.fasta -stdout yes",
    },
    AcceptanceAnchorSpec {
        tool_name: "distmat",
        autodoc_contract: "docs/autodoc/tools/distmat.json",
        example_id: "compute_p_distance_matrix_for_equal_length_records",
        expected_output: "crates/emboss-testkit/tests/fixtures/acceptance_anchors/distmat_compute_p_distance_matrix_for_equal_length_records.tsv",
        legacy_source: "EMBOSS distmat application",
        legacy_locator: "https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/distmat.acd",
        legacy_invocation: "distmat -sequence three_records.fasta -stdout yes",
    },
    AcceptanceAnchorSpec {
        tool_name: "cons",
        autodoc_contract: "docs/autodoc/tools/cons.json",
        example_id: "derive_simple_consensus_from_stockholm_alignment",
        expected_output: "crates/emboss-testkit/tests/fixtures/acceptance_anchors/cons_derive_simple_consensus_from_stockholm_alignment.fasta",
        legacy_source: "EMBOSS cons application",
        legacy_locator: "https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/cons.acd",
        legacy_invocation: "cons -sequence multiple_alignment.sto -outseq stdout",
    },
    AcceptanceAnchorSpec {
        tool_name: "consambig",
        autodoc_contract: "docs/autodoc/tools/consambig.json",
        example_id: "derive_ambiguity_aware_consensus_from_stockholm_alignment",
        expected_output: "crates/emboss-testkit/tests/fixtures/acceptance_anchors/consambig_derive_ambiguity_aware_consensus_from_stockholm_alignment.fasta",
        legacy_source: "EMBOSS consambig application",
        legacy_locator: "https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/consambig.acd",
        legacy_invocation: "consambig -sequence multiple_alignment.sto -outseq stdout",
    },
    AcceptanceAnchorSpec {
        tool_name: "needleall",
        autodoc_contract: "docs/autodoc/tools/needleall.json",
        example_id: "align_all_query_target_pairs",
        expected_output: "crates/emboss-testkit/tests/fixtures/acceptance_anchors/needleall_align_all_query_target_pairs.tsv",
        legacy_source: "EMBOSS needleall application",
        legacy_locator: "https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/needleall.acd",
        legacy_invocation: "needleall -asequence needleall_queries.fasta -bsequence needleall_targets.fasta -stdout yes",
    },
    AcceptanceAnchorSpec {
        tool_name: "diffseq",
        autodoc_contract: "docs/autodoc/tools/diffseq.json",
        example_id: "report_single_substitution_block",
        expected_output: "crates/emboss-testkit/tests/fixtures/acceptance_anchors/diffseq_report_single_substitution_block.tsv",
        legacy_source: "EMBOSS diffseq application",
        legacy_locator: "https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/diffseq.acd",
        legacy_invocation: "diffseq -asequence diffseq_left.fasta -bsequence diffseq_right.fasta -stdout yes",
    },
    AcceptanceAnchorSpec {
        tool_name: "edialign",
        autodoc_contract: "docs/autodoc/tools/edialign.json",
        example_id: "derive_shared_exact_block_alignment",
        expected_output: "crates/emboss-testkit/tests/fixtures/acceptance_anchors/edialign_derive_shared_exact_block_alignment.sto",
        legacy_source: "EMBOSS edialign application",
        legacy_locator: "https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/edialign.acd",
        legacy_invocation: "edialign -sequence edialign_records.fasta -outseq stdout",
    },
    AcceptanceAnchorSpec {
        tool_name: "water",
        autodoc_contract: "docs/autodoc/tools/water.json",
        example_id: "basic_local_alignment",
        expected_output: "crates/emboss-testkit/tests/fixtures/acceptance_anchors/water_basic_local_alignment.sto",
        legacy_source: "EMBOSS water application",
        legacy_locator: "https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/water.acd",
        legacy_invocation: "water -asequence water_query.fasta -bsequence water_target.fasta -gapopen 5 -gapextend 1",
    },
    AcceptanceAnchorSpec {
        tool_name: "descseq",
        autodoc_contract: "docs/autodoc/tools/descseq.json",
        example_id: "summarize_plain_fasta_records",
        expected_output: "crates/emboss-testkit/tests/fixtures/acceptance_anchors/descseq_summarize_plain_fasta_records.tsv",
        legacy_source: "EMBOSS descseq application",
        legacy_locator: "https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/descseq.acd",
        legacy_invocation: "descseq -sequence annotated_feature.gbk -stdout yes",
    },
    AcceptanceAnchorSpec {
        tool_name: "geecee",
        autodoc_contract: "docs/autodoc/tools/geecee.json",
        example_id: "per_record_and_aggregate_gc",
        expected_output: "crates/emboss-testkit/tests/fixtures/acceptance_anchors/geecee_per_record_and_aggregate_gc.tsv",
        legacy_source: "EMBOSS geecee application",
        legacy_locator: "https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/geecee.acd",
        legacy_invocation: "geecee -sequence nucleotide_pattern_records.fasta -stdout yes",
    },
    AcceptanceAnchorSpec {
        tool_name: "infoseq",
        autodoc_contract: "docs/autodoc/tools/infoseq.json",
        example_id: "report_basic_sequence_information",
        expected_output: "crates/emboss-testkit/tests/fixtures/acceptance_anchors/infoseq_report_basic_sequence_information.tsv",
        legacy_source: "EMBOSS infoseq application",
        legacy_locator: "https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/infoseq.acd",
        legacy_invocation: "infoseq -sequence three_records.fasta -stdout yes",
    },
    AcceptanceAnchorSpec {
        tool_name: "cusp",
        autodoc_contract: "docs/autodoc/tools/cusp.json",
        example_id: "report_complete_codon_usage_table",
        expected_output: "crates/emboss-testkit/tests/fixtures/acceptance_anchors/cusp_report_complete_codon_usage_table.tsv",
        legacy_source: "EMBOSS cusp application",
        legacy_locator: "https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/cusp.acd",
        legacy_invocation: "cusp -sequence codon_reference.fasta -stdout yes",
    },
    AcceptanceAnchorSpec {
        tool_name: "fuzznuc",
        autodoc_contract: "docs/autodoc/tools/fuzznuc.json",
        example_id: "iupac_forward_search",
        expected_output: "crates/emboss-testkit/tests/fixtures/acceptance_anchors/fuzznuc_iupac_forward_search.tsv",
        legacy_source: "EMBOSS fuzznuc application",
        legacy_locator: "https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/fuzznuc.acd",
        legacy_invocation: "fuzznuc -sequence nucleotide_pattern_records.fasta -pattern ACGN -stdout yes",
    },
    AcceptanceAnchorSpec {
        tool_name: "fuzzpro",
        autodoc_contract: "docs/autodoc/tools/fuzzpro.json",
        example_id: "wildcard_forward_search",
        expected_output: "crates/emboss-testkit/tests/fixtures/acceptance_anchors/fuzzpro_wildcard_forward_search.tsv",
        legacy_source: "EMBOSS fuzzpro application",
        legacy_locator: "https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/fuzzpro.acd",
        legacy_invocation: "fuzzpro -sequence protein_records.fasta -pattern MX -stdout yes",
    },
    AcceptanceAnchorSpec {
        tool_name: "fuzztran",
        autodoc_contract: "docs/autodoc/tools/fuzztran.json",
        example_id: "forward_frame_search",
        expected_output: "crates/emboss-testkit/tests/fixtures/acceptance_anchors/fuzztran_forward_frame_search.tsv",
        legacy_source: "EMBOSS fuzztran application",
        legacy_locator: "https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/fuzztran.acd",
        legacy_invocation: "fuzztran -sequence checktrans_nucleotide.fasta -pattern MA -stdout yes",
    },
    AcceptanceAnchorSpec {
        tool_name: "preg",
        autodoc_contract: "docs/autodoc/tools/preg.json",
        example_id: "report_overlapping_protein_regex_hits",
        expected_output: "crates/emboss-testkit/tests/fixtures/acceptance_anchors/preg_report_overlapping_protein_regex_hits.tsv",
        legacy_source: "EMBOSS preg application",
        legacy_locator: "https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/preg.acd",
        legacy_invocation: "preg -sequence preg_records.fasta -pattern MAM -stdout yes",
    },
    AcceptanceAnchorSpec {
        tool_name: "patmatdb",
        autodoc_contract: "docs/autodoc/tools/patmatdb.json",
        example_id: "report_hits_from_local_motif_database",
        expected_output: "crates/emboss-testkit/tests/fixtures/acceptance_anchors/patmatdb_report_hits_from_local_motif_database.tsv",
        legacy_source: "EMBOSS patmatdb application",
        legacy_locator: "https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/patmatdb.acd",
        legacy_invocation: "patmatdb -sequence patmatdb_records.fasta -full patmatdb_motifs.tsv -stdout yes",
    },
    AcceptanceAnchorSpec {
        tool_name: "wordmatch",
        autodoc_contract: "docs/autodoc/tools/wordmatch.json",
        example_id: "report_one_exact_shared_region",
        expected_output: "crates/emboss-testkit/tests/fixtures/acceptance_anchors/wordmatch_report_one_exact_shared_region.tsv",
        legacy_source: "EMBOSS wordmatch application",
        legacy_locator: "https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/wordmatch.acd",
        legacy_invocation: "wordmatch -asequence wordmatch_query.fasta -bsequence wordmatch_target.fasta -wordsize 4 -stdout yes",
    },
    AcceptanceAnchorSpec {
        tool_name: "wordfinder",
        autodoc_contract: "docs/autodoc/tools/wordfinder.json",
        example_id: "report_one_exact_shared_region_across_target_set",
        expected_output: "crates/emboss-testkit/tests/fixtures/acceptance_anchors/wordfinder_report_one_exact_shared_region_across_target_set.tsv",
        legacy_source: "EMBOSS wordfinder application",
        legacy_locator: "https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/wordfinder.acd",
        legacy_invocation: "wordfinder -asequence wordmatch_query.fasta -bsequence wordfinder_targets.fasta -wordsize 4 -stdout yes",
    },
    AcceptanceAnchorSpec {
        tool_name: "dreg",
        autodoc_contract: "docs/autodoc/tools/dreg.json",
        example_id: "report_overlapping_regex_hits",
        expected_output: "crates/emboss-testkit/tests/fixtures/acceptance_anchors/dreg_report_overlapping_regex_hits.tsv",
        legacy_source: "EMBOSS dreg application",
        legacy_locator: "https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/dreg.acd",
        legacy_invocation: "dreg -sequence dreg_records.fasta -pattern ATA -stdout yes",
    },
    AcceptanceAnchorSpec {
        tool_name: "palindrome",
        autodoc_contract: "docs/autodoc/tools/palindrome.json",
        example_id: "report_bounded_exact_palindromes",
        expected_output: "crates/emboss-testkit/tests/fixtures/acceptance_anchors/palindrome_report_bounded_exact_palindromes.tsv",
        legacy_source: "EMBOSS palindrome application",
        legacy_locator: "https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/palindrome.acd",
        legacy_invocation: "palindrome -sequence palindrome_records.fasta -minpallen 6 -maxpallen 6 -stdout yes",
    },
    AcceptanceAnchorSpec {
        tool_name: "einverted",
        autodoc_contract: "docs/autodoc/tools/einverted.json",
        example_id: "report_exact_inverted_repeat_arms",
        expected_output: "crates/emboss-testkit/tests/fixtures/acceptance_anchors/einverted_report_exact_inverted_repeat_arms.tsv",
        legacy_source: "EMBOSS einverted application",
        legacy_locator: "https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/einverted.acd",
        legacy_invocation: "einverted -sequence einverted_records.fasta -minrepeat 4 -maxgap 2 -stdout yes",
    },
    AcceptanceAnchorSpec {
        tool_name: "seqmatchall",
        autodoc_contract: "docs/autodoc/tools/seqmatchall.json",
        example_id: "report_all_against_all_exact_regions",
        expected_output: "crates/emboss-testkit/tests/fixtures/acceptance_anchors/seqmatchall_report_all_against_all_exact_regions.tsv",
        legacy_source: "EMBOSS seqmatchall application",
        legacy_locator: "https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/seqmatchall.acd",
        legacy_invocation: "seqmatchall -sequence seqmatchall_records.fasta -wordsize 4 -stdout yes",
    },
    AcceptanceAnchorSpec {
        tool_name: "recoder",
        autodoc_contract: "docs/autodoc/tools/recoder.json",
        example_id: "remove_exact_ecori_site_with_synonymous_edits",
        expected_output: "crates/emboss-testkit/tests/fixtures/acceptance_anchors/recoder_report_synonymous_site_removal_candidates.tsv",
        legacy_source: "EMBOSS recoder application",
        legacy_locator: "https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/recoder.acd",
        legacy_invocation: "recoder -sequence recoder_records.fasta -site GAATTC -outfile stdout",
    },
    AcceptanceAnchorSpec {
        tool_name: "silent",
        autodoc_contract: "docs/autodoc/tools/silent.json",
        example_id: "create_exact_ecori_site_with_synonymous_edit",
        expected_output: "crates/emboss-testkit/tests/fixtures/acceptance_anchors/silent_report_synonymous_site_creation_candidates.tsv",
        legacy_source: "EMBOSS silent application",
        legacy_locator: "https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/silent.acd",
        legacy_invocation: "silent -sequence silent_records.fasta -site GAATTC -outfile stdout",
    },
    AcceptanceAnchorSpec {
        tool_name: "charge",
        autodoc_contract: "docs/autodoc/tools/charge.json",
        example_id: "charge_profile_example",
        expected_output: "crates/emboss-testkit/tests/fixtures/acceptance_anchors/charge_charge_profile_example.tsv",
        legacy_source: "EMBOSS charge application",
        legacy_locator: "https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/charge.acd",
        legacy_invocation: "charge -sequence charge_protein.fasta -winsize 5 -step 1 -graph data",
    },
    AcceptanceAnchorSpec {
        tool_name: "pepwindow",
        autodoc_contract: "docs/autodoc/tools/pepwindow.json",
        example_id: "pepwindow_profile_example",
        expected_output: "crates/emboss-testkit/tests/fixtures/acceptance_anchors/pepwindow_pepwindow_profile_example.tsv",
        legacy_source: "EMBOSS pepwindow application",
        legacy_locator: "https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/pepwindow.acd",
        legacy_invocation: "pepwindow -sequence pepwindow_protein.fasta -winsize 5 -step 2 -graph data",
    },
    AcceptanceAnchorSpec {
        tool_name: "hmoment",
        autodoc_contract: "docs/autodoc/tools/hmoment.json",
        example_id: "hmoment_profile_example",
        expected_output: "crates/emboss-testkit/tests/fixtures/acceptance_anchors/hmoment_hmoment_profile_example.tsv",
        legacy_source: "EMBOSS hmoment application",
        legacy_locator: "https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/hmoment.acd",
        legacy_invocation: "hmoment -sequence hmoment_protein.fasta -winsize 4 -step 1 -graph data",
    },
    AcceptanceAnchorSpec {
        tool_name: "octanol",
        autodoc_contract: "docs/autodoc/tools/octanol.json",
        example_id: "octanol_profile_example",
        expected_output: "crates/emboss-testkit/tests/fixtures/acceptance_anchors/octanol_octanol_profile_example.tsv",
        legacy_source: "EMBOSS octanol application",
        legacy_locator: "https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/octanol.acd",
        legacy_invocation: "octanol -sequence octanol_protein.fasta -width 3 -graph data",
    },
    AcceptanceAnchorSpec {
        tool_name: "pepinfo",
        autodoc_contract: "docs/autodoc/tools/pepinfo.json",
        example_id: "pepinfo_profile_example",
        expected_output: "crates/emboss-testkit/tests/fixtures/acceptance_anchors/pepinfo_pepinfo_profile_example.tsv",
        legacy_source: "EMBOSS pepinfo application",
        legacy_locator: "https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/pepinfo.acd",
        legacy_invocation: "pepinfo -sequence pepinfo_protein.fasta -winsize 3 -step 1 -graph data",
    },
    AcceptanceAnchorSpec {
        tool_name: "density",
        autodoc_contract: "docs/autodoc/tools/density.json",
        example_id: "density_profile_example",
        expected_output: "crates/emboss-testkit/tests/fixtures/acceptance_anchors/density_density_profile_example.tsv",
        legacy_source: "EMBOSS density application",
        legacy_locator: "https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/density.acd",
        legacy_invocation: "density -sequence density_nucleotide.fasta -window 4 -graph data",
    },
    AcceptanceAnchorSpec {
        tool_name: "wobble",
        autodoc_contract: "docs/autodoc/tools/wobble.json",
        example_id: "wobble_profile_example",
        expected_output: "crates/emboss-testkit/tests/fixtures/acceptance_anchors/wobble_wobble_profile_example.tsv",
        legacy_source: "EMBOSS wobble application",
        legacy_locator: "https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/wobble.acd",
        legacy_invocation: "wobble -sequence wobble_coding_nucleotide.fasta -window 3 -graph data",
    },
    AcceptanceAnchorSpec {
        tool_name: "isochore",
        autodoc_contract: "docs/autodoc/tools/isochore.json",
        example_id: "isochore_profile_example",
        expected_output: "crates/emboss-testkit/tests/fixtures/acceptance_anchors/isochore_isochore_profile_example.tsv",
        legacy_source: "EMBOSS isochore application",
        legacy_locator: "https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/isochore.acd",
        legacy_invocation: "isochore -sequence isochore_nucleotide.fasta -window 4 -graph data",
    },
    AcceptanceAnchorSpec {
        tool_name: "banana",
        autodoc_contract: "docs/autodoc/tools/banana.json",
        example_id: "banana_profile_example",
        expected_output: "crates/emboss-testkit/tests/fixtures/acceptance_anchors/banana_banana_profile_example.tsv",
        legacy_source: "EMBOSS banana application",
        legacy_locator: "https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/banana.acd",
        legacy_invocation: "banana -sequence banana_nucleotide.fasta -graph data",
    },
    AcceptanceAnchorSpec {
        tool_name: "syco",
        autodoc_contract: "docs/autodoc/tools/syco.json",
        example_id: "syco_profile_example",
        expected_output: "crates/emboss-testkit/tests/fixtures/acceptance_anchors/syco_syco_profile_example.tsv",
        legacy_source: "EMBOSS syco application",
        legacy_locator: "https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/syco.acd",
        legacy_invocation: "syco -seqall syco_coding_nucleotide.fasta -cfile codon_reference.fasta -graph data",
    },
    AcceptanceAnchorSpec {
        tool_name: "compseq",
        autodoc_contract: "docs/autodoc/tools/compseq.json",
        example_id: "per_record_and_aggregate_composition",
        expected_output: "crates/emboss-testkit/tests/fixtures/acceptance_anchors/compseq_per_record_and_aggregate_composition.tsv",
        legacy_source: "EMBOSS compseq application",
        legacy_locator: "https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/compseq.acd",
        legacy_invocation: "compseq -sequence nucleotide_pattern_records.fasta -stdout yes",
    },
    AcceptanceAnchorSpec {
        tool_name: "pepstats",
        autodoc_contract: "docs/autodoc/tools/pepstats.json",
        example_id: "protein_summary_statistics",
        expected_output: "crates/emboss-testkit/tests/fixtures/acceptance_anchors/pepstats_protein_summary_statistics.tsv",
        legacy_source: "EMBOSS pepstats application",
        legacy_locator: "https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/pepstats.acd",
        legacy_invocation: "pepstats -sequence protein_stats_records.fasta -stdout yes",
    },
    AcceptanceAnchorSpec {
        tool_name: "cai",
        autodoc_contract: "docs/autodoc/tools/cai.json",
        example_id: "score_query_sequences_against_reference_fixture",
        expected_output: "crates/emboss-testkit/tests/fixtures/acceptance_anchors/cai_score_query_sequences_against_reference_fixture.tsv",
        legacy_source: "EMBOSS cai application",
        legacy_locator: "https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/cai.acd",
        legacy_invocation: "cai -sequence codon_query.fasta -cfile codon_reference.fasta -outfile stdout",
    },
    AcceptanceAnchorSpec {
        tool_name: "chips",
        autodoc_contract: "docs/autodoc/tools/chips.json",
        example_id: "report_codon_usage_for_reference_fixture",
        expected_output: "crates/emboss-testkit/tests/fixtures/acceptance_anchors/chips_report_codon_usage_for_reference_fixture.tsv",
        legacy_source: "EMBOSS chips application",
        legacy_locator: "https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/chips.acd",
        legacy_invocation: "chips -sequence codon_reference.fasta -outfile stdout",
    },
    AcceptanceAnchorSpec {
        tool_name: "codcmp",
        autodoc_contract: "docs/autodoc/tools/codcmp.json",
        example_id: "compare_two_coding_fixtures",
        expected_output: "crates/emboss-testkit/tests/fixtures/acceptance_anchors/codcmp_compare_two_coding_fixtures.tsv",
        legacy_source: "EMBOSS codcmp application",
        legacy_locator: "https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/codcmp.acd",
        legacy_invocation: "codcmp -first codon_query.fasta -second codon_compare_right.fasta -outfile stdout",
    },
    AcceptanceAnchorSpec {
        tool_name: "codcopy",
        autodoc_contract: "docs/autodoc/tools/codcopy.json",
        example_id: "normalize_coding_input_into_reusable_profile",
        expected_output: "crates/emboss-testkit/tests/fixtures/acceptance_anchors/codcopy_normalize_coding_input_into_reusable_profile.tsv",
        legacy_source: "EMBOSS codcopy application",
        legacy_locator: "https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/codcopy.acd",
        legacy_invocation: "codcopy -sequence codon_reference.fasta -outfile stdout",
    },
    AcceptanceAnchorSpec {
        tool_name: "aaindexextract",
        autodoc_contract: "docs/autodoc/tools/aaindexextract.json",
        example_id: "report_hydropathy_subset",
        expected_output: "crates/emboss-testkit/tests/fixtures/acceptance_anchors/aaindexextract_report_hydropathy_subset.tsv",
        legacy_source: "EMBOSS aaindexextract application",
        legacy_locator: "https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/aaindexextract.acd",
        legacy_invocation: "aaindexextract -index hydropathy -outfile stdout",
    },
    AcceptanceAnchorSpec {
        tool_name: "infobase",
        autodoc_contract: "docs/autodoc/tools/infobase.json",
        example_id: "lookup_any_base_symbol",
        expected_output: "crates/emboss-testkit/tests/fixtures/acceptance_anchors/infobase_lookup_any_base_symbol.tsv",
        legacy_source: "EMBOSS infobase application",
        legacy_locator: "https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/infobase.acd",
        legacy_invocation: "infobase -base N -outfile stdout",
    },
    AcceptanceAnchorSpec {
        tool_name: "inforesidue",
        autodoc_contract: "docs/autodoc/tools/inforesidue.json",
        example_id: "lookup_lysine",
        expected_output: "crates/emboss-testkit/tests/fixtures/acceptance_anchors/inforesidue_lookup_lysine.tsv",
        legacy_source: "EMBOSS inforesidue application",
        legacy_locator: "https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/inforesidue.acd",
        legacy_invocation: "inforesidue -residue K -outfile stdout",
    },
    AcceptanceAnchorSpec {
        tool_name: "oddcomp",
        autodoc_contract: "docs/autodoc/tools/oddcomp.json",
        example_id: "count_exact_protein_words",
        expected_output: "crates/emboss-testkit/tests/fixtures/acceptance_anchors/oddcomp_count_exact_protein_words.tsv",
        legacy_source: "EMBOSS oddcomp application",
        legacy_locator: "https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/oddcomp.acd",
        legacy_invocation: "oddcomp -sequence oddcomp_records.fasta -word MAM -word QQQ -outfile stdout",
    },
    AcceptanceAnchorSpec {
        tool_name: "iep",
        autodoc_contract: "docs/autodoc/tools/iep.json",
        example_id: "estimate_pi_for_protein_records",
        expected_output: "crates/emboss-testkit/tests/fixtures/acceptance_anchors/iep_estimate_pi_for_protein_records.tsv",
        legacy_source: "EMBOSS iep application",
        legacy_locator: "https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/iep.acd",
        legacy_invocation: "iep -sequence iep_records.fasta -outfile stdout",
    },
    AcceptanceAnchorSpec {
        tool_name: "pepdigest",
        autodoc_contract: "docs/autodoc/tools/pepdigest.json",
        example_id: "digest_proteins_with_trypsin",
        expected_output: "crates/emboss-testkit/tests/fixtures/acceptance_anchors/pepdigest_digest_proteins_with_trypsin.tsv",
        legacy_source: "EMBOSS pepdigest application",
        legacy_locator: "https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/pepdigest.acd",
        legacy_invocation: "pepdigest -sequence pepdigest_records.fasta -menu trypsin -outfile stdout",
    },
    AcceptanceAnchorSpec {
        tool_name: "wordcount",
        autodoc_contract: "docs/autodoc/tools/wordcount.json",
        example_id: "count_overlapping_words",
        expected_output: "crates/emboss-testkit/tests/fixtures/acceptance_anchors/wordcount_count_overlapping_words.tsv",
        legacy_source: "EMBOSS wordcount application",
        legacy_locator: "https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/wordcount.acd",
        legacy_invocation: "wordcount -sequence three_records.fasta -wordsize 2 -outfile stdout",
    },
    AcceptanceAnchorSpec {
        tool_name: "dan",
        autodoc_contract: "docs/autodoc/tools/dan.json",
        example_id: "report_sliding_window_tm",
        expected_output: "crates/emboss-testkit/tests/fixtures/acceptance_anchors/dan_report_sliding_window_tm.tsv",
        legacy_source: "EMBOSS dan application",
        legacy_locator: "https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/dan.acd",
        legacy_invocation: "dan -sequence three_records.fasta -windowsize 2 -outfile stdout",
    },
    AcceptanceAnchorSpec {
        tool_name: "complex",
        autodoc_contract: "docs/autodoc/tools/complex.json",
        example_id: "report_whole_sequence_complexity",
        expected_output: "crates/emboss-testkit/tests/fixtures/acceptance_anchors/complex_report_whole_sequence_complexity.tsv",
        legacy_source: "EMBOSS complex application",
        legacy_locator: "https://github.com/kimrutherford/EMBOSS/blob/master/emboss/acd/complex.acd",
        legacy_invocation: "complex -sequence complex_records.fasta -kmin 1 -kmax 2 -outfile stdout",
    },
];

/// Returns the committed acceptance-anchor cohort.
#[must_use]
pub fn acceptance_anchor_specs() -> &'static [AcceptanceAnchorSpec] {
    ACCEPTANCE_ANCHORS
}

/// Derives an executed-and-compared validation report for one acceptance anchor.
pub fn derive_acceptance_anchor_report(
    repo_root: impl AsRef<Path>,
    tool_name: &str,
) -> Result<ToolValidationReport, PlatformError> {
    let repo_root = repo_root.as_ref();
    let spec = acceptance_anchor_specs()
        .iter()
        .find(|spec| spec.tool_name == tool_name)
        .ok_or_else(|| {
            PlatformError::new(
                ErrorCategory::Configuration,
                "requested tool is not part of the acceptance-anchor cohort",
            )
            .with_code("testkit.anchor.unknown_tool")
            .with_detail(tool_name.to_owned())
        })?;

    let document =
        load_document_from_path(repo_root.join(spec.autodoc_contract)).map_err(|error| {
            PlatformError::new(
                ErrorCategory::Validation,
                "failed to load acceptance-anchor autodoc contract",
            )
            .with_code("testkit.anchor.autodoc.load_failed")
            .with_detail(format!(
                "{}: {error}",
                repo_root.join(spec.autodoc_contract).display()
            ))
        })?;
    let mut report = derive_validation_report(&document)?;
    let actual = execute_anchor_payload(repo_root, spec)?;
    let expected = fs::read_to_string(repo_root.join(spec.expected_output)).map_err(|error| {
        PlatformError::new(
            ErrorCategory::Configuration,
            "failed to read committed acceptance-anchor expected output",
        )
        .with_code("testkit.anchor.expected_output.read_failed")
        .with_detail(format!(
            "{}: {error}",
            repo_root.join(spec.expected_output).display()
        ))
    })?;

    let actual_normalized = normalize_text(repo_root, &actual.payload);
    let expected_normalized = normalize_text(repo_root, &expected);
    if actual_normalized != expected_normalized {
        return Err(PlatformError::new(
            ErrorCategory::Validation,
            "acceptance-anchor output differed from the committed expected output",
        )
        .with_code("testkit.anchor.comparison.failed")
        .with_detail(format!(
            "tool '{}' anchor case '{}' did not match '{}'\nexpected:\n{:?}\nactual:\n{:?}",
            spec.tool_name,
            spec.example_id,
            spec.expected_output,
            expected_normalized,
            actual_normalized
        )));
    }

    if let Some(expected_plot_output) = expected_plot_output(spec.tool_name) {
        let actual_plot = actual.plot.ok_or_else(|| {
            PlatformError::new(
                ErrorCategory::Validation,
                "acceptance-anchor invocation did not attach the expected plot contract",
            )
            .with_code("testkit.anchor.plot.missing")
            .with_detail(spec.tool_name.to_owned())
        })?;
        let expected_plot =
            fs::read_to_string(repo_root.join(expected_plot_output)).map_err(|error| {
                PlatformError::new(
                    ErrorCategory::Configuration,
                    "failed to read committed acceptance-anchor expected plot contract",
                )
                .with_code("testkit.anchor.expected_plot.read_failed")
                .with_detail(format!(
                    "{}: {error}",
                    repo_root.join(expected_plot_output).display()
                ))
            })?;
        let actual_plot_normalized = normalize_text(repo_root, &actual_plot);
        let expected_plot_normalized = normalize_text(repo_root, &expected_plot);
        if actual_plot_normalized != expected_plot_normalized {
            return Err(PlatformError::new(
                ErrorCategory::Validation,
                "acceptance-anchor plot contract differed from the committed expected output",
            )
            .with_code("testkit.anchor.plot.comparison.failed")
            .with_detail(format!(
                "tool '{}' plot output did not match '{}'\nexpected:\n{:?}\nactual:\n{:?}",
                spec.tool_name,
                expected_plot_output,
                expected_plot_normalized,
                actual_plot_normalized
            )));
        }
    }

    let legacy_reference = LegacyReference {
        source: spec.legacy_source.to_owned(),
        locator: Some(spec.legacy_locator.to_owned()),
        invocation: Some(spec.legacy_invocation.to_owned()),
    };

    let case = report
        .cases
        .iter_mut()
        .find(|case| case.id == spec.example_id)
        .ok_or_else(|| {
            PlatformError::new(
                ErrorCategory::Validation,
                "acceptance-anchor contract did not contain its declared example id",
            )
            .with_code("testkit.anchor.example.missing")
            .with_detail(format!("{} in {}", spec.example_id, spec.autodoc_contract))
        })?;

    case.evidence_source = EvidenceSourceKind::ExecutedRun;
    case.declaration_status = EvidenceDeclarationStatus::Harvested;
    case.execution_status = ExecutionStatus::Executed;
    case.comparison_status = ComparisonStatus::Passed;
    if !case.provenance.contains(&legacy_reference) {
        case.provenance.push(legacy_reference.clone());
    }

    if !report.provenance.contains(&legacy_reference) {
        report.provenance.push(legacy_reference);
    }
    report.evidence_source = EvidenceSourceKind::ExecutedRun;

    Ok(ToolValidationReport::new(
        report.tool_name,
        report.document_id,
        report.source_mode,
        report.evidence_source,
        report.cases,
        report.unresolved_gaps,
        report.diagnostics,
        report.provenance,
    ))
}

/// Writes executed-and-compared reports for the acceptance-anchor cohort.
pub fn write_acceptance_anchor_reports(
    repo_root: impl AsRef<Path>,
    output_dir: impl AsRef<Path>,
) -> Result<Vec<PathBuf>, PlatformError> {
    let repo_root = repo_root.as_ref();
    let output_dir = output_dir.as_ref();
    fs::create_dir_all(output_dir).map_err(|error| {
        PlatformError::new(
            ErrorCategory::Configuration,
            "failed to create acceptance-anchor validation output directory",
        )
        .with_code("testkit.anchor.output_dir.create_failed")
        .with_detail(format!("{}: {error}", output_dir.display()))
    })?;

    let mut written = Vec::new();
    for spec in acceptance_anchor_specs() {
        let report = derive_acceptance_anchor_report(repo_root, spec.tool_name)?;
        let path = output_dir.join(format!("{}.validation.json", spec.tool_name));
        write_validation_report_json(&report, &path)?;
        written.push(path);
    }

    Ok(written)
}

fn execute_anchor_payload(
    repo_root: &Path,
    spec: &AcceptanceAnchorSpec,
) -> Result<AnchorActualResult, PlatformError> {
    if matches!(
        spec.tool_name,
        "refseqget" | "runinfo" | "runget" | "infoassembly"
    ) {
        return execute_mocked_provider_anchor_payload(repo_root, spec);
    }

    let service = implemented_service()?;
    let request = InvocationRequest::new(
        ExecutionContext::default(),
        ToolName::new(spec.tool_name).map_err(|error| {
            PlatformError::new(
                ErrorCategory::Configuration,
                "invalid acceptance-anchor tool name",
            )
            .with_code("testkit.anchor.tool.invalid")
            .with_source(error)
        })?,
    )
    .with_arguments(anchor_arguments(repo_root, spec.tool_name));

    let response = service.invoke(request).map_err(|error| {
        PlatformError::new(
            ErrorCategory::Invocation,
            "acceptance-anchor invocation failed",
        )
        .with_code("testkit.anchor.invoke_failed")
        .with_source(error)
        .with_detail(spec.tool_name.to_owned())
    })?;

    Ok(AnchorActualResult {
        payload: render_payload(&response.result.payload)?,
        plot: render_plot(response.result.plot.as_ref())?,
    })
}

#[derive(Default)]
struct AnchorMockHttpClient {
    responses: HashMap<String, HttpResponse>,
}

impl AnchorMockHttpClient {
    fn with_response(mut self, url: impl Into<String>, response: HttpResponse) -> Self {
        self.responses.insert(url.into(), response);
        self
    }
}

impl ProviderHttpClient for AnchorMockHttpClient {
    fn get_text(&self, request: &HttpRequest) -> Result<HttpResponse, PlatformError> {
        self.responses.get(&request.url).cloned().ok_or_else(|| {
            PlatformError::new(
                ErrorCategory::Configuration,
                "acceptance-anchor mock HTTP client had no registered response for the requested URL",
            )
            .with_code("testkit.anchor.http.unregistered_url")
            .with_detail(request.url.clone())
        })
    }
}

fn execute_mocked_provider_anchor_payload(
    _repo_root: &Path,
    spec: &AcceptanceAnchorSpec,
) -> Result<AnchorActualResult, PlatformError> {
    let service = implemented_service()?;
    let tool = ToolName::new(spec.tool_name).map_err(|error| {
        PlatformError::new(
            ErrorCategory::Configuration,
            "invalid mocked-provider acceptance-anchor tool name",
        )
        .with_code("testkit.anchor.tool.invalid")
        .with_source(error)
    })?;
    let descriptor = service.registry().find(&tool).copied().ok_or_else(|| {
        PlatformError::new(
            ErrorCategory::Configuration,
            "mocked-provider acceptance-anchor tool is not registered",
        )
        .with_code("testkit.anchor.tool.unregistered")
        .with_detail(spec.tool_name.to_owned())
    })?;

    let (request, client) = mocked_provider_request(spec.tool_name);
    let response = match spec.tool_name {
        "refseqget" => service.invoke_refseqget_with_client(request, descriptor, Some(&client)),
        "runinfo" => service.invoke_runinfo_with_client(request, descriptor, Some(&client)),
        "runget" => service.invoke_runget_with_client(request, descriptor, Some(&client)),
        "infoassembly" => {
            service.invoke_infoassembly_with_client(request, descriptor, Some(&client))
        }
        other => {
            return Err(PlatformError::new(
                ErrorCategory::Configuration,
                "unsupported mocked-provider acceptance-anchor tool",
            )
            .with_code("testkit.anchor.mocked_provider.unsupported")
            .with_detail(other.to_owned()));
        }
    }
    .map_err(|error| {
        PlatformError::new(
            ErrorCategory::Invocation,
            "mocked-provider acceptance-anchor invocation failed",
        )
        .with_code("testkit.anchor.invoke_failed")
        .with_source(error)
        .with_detail(spec.tool_name.to_owned())
    })?;

    Ok(AnchorActualResult {
        payload: render_payload(&response.result.payload)?,
        plot: render_plot(response.result.plot.as_ref())?,
    })
}

fn implemented_service() -> Result<EmbossService, PlatformError> {
    let mut registry = ServiceRegistry::new();
    for descriptor in governed_tool_descriptors() {
        registry.register(*descriptor).map_err(|error| {
            PlatformError::new(
                ErrorCategory::Configuration,
                "failed to register governed tool in acceptance-anchor service",
            )
            .with_code("testkit.anchor.registry.register_failed")
            .with_source(error)
            .with_detail(descriptor.name.to_owned())
        })?;
    }
    Ok(EmbossService::new(registry))
}

fn anchor_arguments(repo_root: &Path, tool_name: &str) -> Vec<String> {
    match tool_name {
        "needle" => vec![
            repo_root
                .join("crates/emboss-tools/tests/fixtures/needle_query.fasta")
                .display()
                .to_string(),
            repo_root
                .join("crates/emboss-tools/tests/fixtures/needle_target.fasta")
                .display()
                .to_string(),
        ],
        "seqret" => vec![
            repo_root
                .join("crates/emboss-tools/tests/fixtures/three_records.fasta")
                .display()
                .to_string(),
        ],
        "seqretsetall" => vec![
            repo_root
                .join("crates/emboss-tools/tests/fixtures/three_records.fasta")
                .display()
                .to_string(),
            repo_root
                .join("crates/emboss-tools/tests/fixtures/two_records.fasta")
                .display()
                .to_string(),
        ],
        "seqretsplit" => vec![
            repo_root
                .join("crates/emboss-tools/tests/fixtures/three_records.fasta")
                .display()
                .to_string(),
        ],
        "psiphi" => vec![
            repo_root
                .join("crates/emboss-tools/tests/fixtures/psiphi_backbone.txt")
                .display()
                .to_string(),
        ],
        "eprimer3" => vec![
            repo_root
                .join("crates/emboss-tools/tests/fixtures/eprimer3_targets.fasta")
                .display()
                .to_string(),
        ],
        "primersearch" => vec![
            repo_root
                .join("crates/emboss-tools/tests/fixtures/primersearch_targets.fasta")
                .display()
                .to_string(),
            repo_root
                .join("crates/emboss-tools/tests/fixtures/primersearch_pairs.tsv")
                .display()
                .to_string(),
        ],
        "sirna" => vec![
            repo_root
                .join("crates/emboss-tools/tests/fixtures/sirna_targets.fasta")
                .display()
                .to_string(),
        ],
        "newseq" => vec![
            "created".to_owned(),
            "ACGTAC".to_owned(),
            "--description".to_owned(),
            "created example".to_owned(),
            "--molecule".to_owned(),
            "dna".to_owned(),
        ],
        "makenucseq" => vec![
            "made_nuc".to_owned(),
            "6".to_owned(),
            "--count".to_owned(),
            "2".to_owned(),
            "--seed".to_owned(),
            "7".to_owned(),
            "--molecule".to_owned(),
            "rna".to_owned(),
        ],
        "makeprotseq" => vec![
            "made_prot".to_owned(),
            "5".to_owned(),
            "--seed".to_owned(),
            "9".to_owned(),
        ],
        "seqcount" => vec![
            repo_root
                .join("crates/emboss-tools/tests/fixtures/three_records.fasta")
                .display()
                .to_string(),
        ],
        "notseq" => vec![
            repo_root
                .join("crates/emboss-tools/tests/fixtures/three_records.fasta")
                .display()
                .to_string(),
            "2".to_owned(),
        ],
        "nthseq" => vec![
            repo_root
                .join("crates/emboss-tools/tests/fixtures/three_records.fasta")
                .display()
                .to_string(),
            "2".to_owned(),
        ],
        "skipseq" => vec![
            repo_root
                .join("crates/emboss-tools/tests/fixtures/three_records.fasta")
                .display()
                .to_string(),
            "1".to_owned(),
        ],
        "listor" => vec![
            repo_root
                .join("crates/emboss-tools/tests/fixtures/listor_first.fasta")
                .display()
                .to_string(),
            repo_root
                .join("crates/emboss-tools/tests/fixtures/listor_second.fasta")
                .display()
                .to_string(),
            "--operator".to_owned(),
            "xor".to_owned(),
        ],
        "skipredundant" => vec![
            repo_root
                .join("crates/emboss-tools/tests/fixtures/skipredundant_records.fasta")
                .display()
                .to_string(),
        ],
        "extractfeat" => vec![
            repo_root
                .join("crates/emboss-tools/tests/fixtures/annotated_feature.gbk")
                .display()
                .to_string(),
            "--kind".to_owned(),
            "gene".to_owned(),
        ],
        "featcopy" => vec![
            repo_root
                .join("crates/emboss-tools/tests/fixtures/annotated_feature.gbk")
                .display()
                .to_string(),
            repo_root
                .join("crates/emboss-tools/tests/fixtures/featcopy_target.fasta")
                .display()
                .to_string(),
            "--kind".to_owned(),
            "gene".to_owned(),
        ],
        "maskfeat" => vec![
            repo_root
                .join("crates/emboss-tools/tests/fixtures/annotated_feature.gbk")
                .display()
                .to_string(),
            "--kind".to_owned(),
            "gene".to_owned(),
        ],
        "coderet" => vec![
            repo_root
                .join("crates/emboss-tools/tests/fixtures/annotated_feature.gbk")
                .display()
                .to_string(),
            "--translate".to_owned(),
        ],
        "featmerge" => vec![
            repo_root
                .join("crates/emboss-tools/tests/fixtures/annotated_feature.gbk")
                .display()
                .to_string(),
            repo_root
                .join("crates/emboss-tools/tests/fixtures/annotated_merge_right.gbk")
                .display()
                .to_string(),
        ],
        "featreport" => vec![
            repo_root
                .join("crates/emboss-tools/tests/fixtures/annotated_feature.gbk")
                .display()
                .to_string(),
        ],
        "feattext" => vec![
            repo_root
                .join("crates/emboss-tools/tests/fixtures/annotated_feature.gbk")
                .display()
                .to_string(),
        ],
        "splitsource" => vec![
            repo_root
                .join("crates/emboss-tools/tests/fixtures/splitsource_annotated.gbk")
                .display()
                .to_string(),
        ],
        "twofeat" => vec![
            repo_root
                .join("crates/emboss-tools/tests/fixtures/annotated_feature.gbk")
                .display()
                .to_string(),
            "--a-kind".to_owned(),
            "gene".to_owned(),
            "--b-kind".to_owned(),
            "cds".to_owned(),
        ],
        "maskseq" => vec![
            repo_root
                .join("crates/emboss-tools/tests/fixtures/three_records.fasta")
                .display()
                .to_string(),
            "2:3".to_owned(),
        ],
        "biosed" => vec![
            repo_root
                .join("crates/emboss-tools/tests/fixtures/biosed_records.fasta")
                .display()
                .to_string(),
            "2".to_owned(),
            "3".to_owned(),
            "--replace".to_owned(),
            "NN".to_owned(),
        ],
        "degapseq" => vec![
            repo_root
                .join("crates/emboss-tools/tests/fixtures/gapped_records.fasta")
                .display()
                .to_string(),
        ],
        "revseq" => vec![
            repo_root
                .join("crates/emboss-tools/tests/fixtures/three_records.fasta")
                .display()
                .to_string(),
        ],
        "msbar" => vec![
            repo_root
                .join("crates/emboss-tools/tests/fixtures/msbar_records.fasta")
                .display()
                .to_string(),
            "2:T".to_owned(),
            "4:A".to_owned(),
        ],
        "trimest" => vec![
            repo_root
                .join("crates/emboss-tools/tests/fixtures/trimest_records.fasta")
                .display()
                .to_string(),
            "--min-tail".to_owned(),
            "4".to_owned(),
        ],
        "trimseq" => vec![
            repo_root
                .join("crates/emboss-tools/tests/fixtures/three_records.fasta")
                .display()
                .to_string(),
            "--left".to_owned(),
            "1".to_owned(),
            "--right".to_owned(),
            "1".to_owned(),
        ],
        "vectorstrip" => vec![
            repo_root
                .join("crates/emboss-tools/tests/fixtures/vectorstrip_records.fasta")
                .display()
                .to_string(),
            repo_root
                .join("crates/emboss-tools/tests/fixtures/vectorstrip_vector.fasta")
                .display()
                .to_string(),
        ],
        "maskambignuc" => vec![
            repo_root
                .join("crates/emboss-tools/tests/fixtures/ambiguous_nucleotide_records.fasta")
                .display()
                .to_string(),
        ],
        "maskambigprot" => vec![
            repo_root
                .join("crates/emboss-tools/tests/fixtures/ambiguous_protein_records.fasta")
                .display()
                .to_string(),
        ],
        "extractseq" => vec![
            repo_root
                .join("crates/emboss-tools/tests/fixtures/three_records.fasta")
                .display()
                .to_string(),
            "2".to_owned(),
            "3".to_owned(),
        ],
        "cutseq" => vec![
            repo_root
                .join("crates/emboss-tools/tests/fixtures/three_records.fasta")
                .display()
                .to_string(),
            "2".to_owned(),
        ],
        "union" => vec![
            repo_root
                .join("crates/emboss-tools/tests/fixtures/three_records.fasta")
                .display()
                .to_string(),
            repo_root
                .join("crates/emboss-tools/tests/fixtures/two_records.fasta")
                .display()
                .to_string(),
        ],
        "pasteseq" => vec![
            repo_root
                .join("crates/emboss-tools/tests/fixtures/pasteseq_main.fasta")
                .display()
                .to_string(),
            repo_root
                .join("crates/emboss-tools/tests/fixtures/pasteseq_insert.fasta")
                .display()
                .to_string(),
            "2".to_owned(),
        ],
        "splitter" => vec![
            repo_root
                .join("crates/emboss-tools/tests/fixtures/three_records.fasta")
                .display()
                .to_string(),
            "2".to_owned(),
        ],
        "merger" | "megamerger" => vec![
            repo_root
                .join("crates/emboss-tools/tests/fixtures/merger_left.fasta")
                .display()
                .to_string(),
            repo_root
                .join("crates/emboss-tools/tests/fixtures/merger_right.fasta")
                .display()
                .to_string(),
        ],
        "sizeseq" => vec![
            repo_root
                .join("crates/emboss-tools/tests/fixtures/sizeseq_records.fasta")
                .display()
                .to_string(),
        ],
        "shuffleseq" => vec![
            repo_root
                .join("crates/emboss-tools/tests/fixtures/three_records.fasta")
                .display()
                .to_string(),
            "--seed".to_owned(),
            "7".to_owned(),
        ],
        "compseq" => vec![
            repo_root
                .join("crates/emboss-tools/tests/fixtures/nucleotide_pattern_records.fasta")
                .display()
                .to_string(),
        ],
        "pepstats" => vec![
            repo_root
                .join("crates/emboss-tools/tests/fixtures/protein_stats_records.fasta")
                .display()
                .to_string(),
        ],
        "cai" => vec![
            repo_root
                .join("crates/emboss-tools/tests/fixtures/codon_query.fasta")
                .display()
                .to_string(),
            repo_root
                .join("crates/emboss-tools/tests/fixtures/codon_reference.fasta")
                .display()
                .to_string(),
        ],
        "chips" | "codcopy" => vec![
            repo_root
                .join("crates/emboss-tools/tests/fixtures/codon_reference.fasta")
                .display()
                .to_string(),
        ],
        "codcmp" => vec![
            repo_root
                .join("crates/emboss-tools/tests/fixtures/codon_query.fasta")
                .display()
                .to_string(),
            repo_root
                .join("crates/emboss-tools/tests/fixtures/codon_compare_right.fasta")
                .display()
                .to_string(),
        ],
        "aaindexextract" => vec!["hydropathy".to_owned()],
        "infobase" => vec!["N".to_owned()],
        "inforesidue" => vec!["K".to_owned()],
        "oddcomp" => vec![
            repo_root
                .join("crates/emboss-tools/tests/fixtures/oddcomp_records.fasta")
                .display()
                .to_string(),
            "--word".to_owned(),
            "MAM".to_owned(),
            "--word".to_owned(),
            "QQQ".to_owned(),
        ],
        "iep" => vec![
            repo_root
                .join("crates/emboss-tools/tests/fixtures/iep_records.fasta")
                .display()
                .to_string(),
        ],
        "pepdigest" => vec![
            repo_root
                .join("crates/emboss-tools/tests/fixtures/pepdigest_records.fasta")
                .display()
                .to_string(),
        ],
        "wordcount" => vec![
            repo_root
                .join("crates/emboss-tools/tests/fixtures/three_records.fasta")
                .display()
                .to_string(),
            "--word-size".to_owned(),
            "2".to_owned(),
        ],
        "dan" => vec![
            repo_root
                .join("crates/emboss-tools/tests/fixtures/three_records.fasta")
                .display()
                .to_string(),
            "--window".to_owned(),
            "2".to_owned(),
        ],
        "complex" => vec![
            repo_root
                .join("crates/emboss-tools/tests/fixtures/complex_records.fasta")
                .display()
                .to_string(),
            "--k-min".to_owned(),
            "1".to_owned(),
            "--k-max".to_owned(),
            "2".to_owned(),
        ],
        "recoder" => vec![
            repo_root
                .join("crates/emboss-tools/tests/fixtures/recoder_records.fasta")
                .display()
                .to_string(),
            "GAATTC".to_owned(),
        ],
        "silent" => vec![
            repo_root
                .join("crates/emboss-tools/tests/fixtures/silent_records.fasta")
                .display()
                .to_string(),
            "GAATTC".to_owned(),
        ],
        "backtranseq" | "backtranambig" => vec![
            repo_root
                .join("crates/emboss-tools/tests/fixtures/protein_stats_records.fasta")
                .display()
                .to_string(),
        ],
        "checktrans" => vec![
            repo_root
                .join("crates/emboss-tools/tests/fixtures/checktrans_nucleotide.fasta")
                .display()
                .to_string(),
            repo_root
                .join("crates/emboss-tools/tests/fixtures/checktrans_protein.fasta")
                .display()
                .to_string(),
        ],
        "transeq" => vec![
            repo_root
                .join("crates/emboss-tools/tests/fixtures/checktrans_nucleotide.fasta")
                .display()
                .to_string(),
            "--frame".to_owned(),
            "1".to_owned(),
        ],
        "getorf" => vec![
            repo_root
                .join("crates/emboss-tools/tests/fixtures/getorf_records.fasta")
                .display()
                .to_string(),
        ],
        "prettyseq" => vec![
            repo_root
                .join("crates/emboss-tools/tests/fixtures/checktrans_nucleotide.fasta")
                .display()
                .to_string(),
            "--frame".to_owned(),
            "1".to_owned(),
        ],
        "tranalign" => vec![
            repo_root
                .join("crates/emboss-tools/tests/fixtures/tranalign_protein_alignment.sto")
                .display()
                .to_string(),
            repo_root
                .join("crates/emboss-tools/tests/fixtures/checktrans_nucleotide.fasta")
                .display()
                .to_string(),
        ],
        "aligncopy" => vec![
            repo_root
                .join("crates/emboss-tools/tests/fixtures/multiple_alignment.sto")
                .display()
                .to_string(),
        ],
        "aligncopypair" => vec![
            repo_root
                .join("crates/emboss-tools/tests/fixtures/pairwise_alignment.sto")
                .display()
                .to_string(),
        ],
        "infoalign" => vec![
            repo_root
                .join("crates/emboss-tools/tests/fixtures/multiple_alignment.sto")
                .display()
                .to_string(),
        ],
        "extractalign" => vec![
            repo_root
                .join("crates/emboss-tools/tests/fixtures/multiple_alignment.sto")
                .display()
                .to_string(),
            "--row-id".to_owned(),
            "alpha".to_owned(),
            "--row".to_owned(),
            "3".to_owned(),
            "--start".to_owned(),
            "2".to_owned(),
            "--end".to_owned(),
            "4".to_owned(),
        ],
        "nthseqset" => vec![
            repo_root
                .join("crates/emboss-tools/tests/fixtures/nthseqset_alignments.sto")
                .display()
                .to_string(),
            "2".to_owned(),
        ],
        "matcher" => vec![
            repo_root
                .join("crates/emboss-tools/tests/fixtures/needle_query.fasta")
                .display()
                .to_string(),
            repo_root
                .join("crates/emboss-tools/tests/fixtures/needle_target.fasta")
                .display()
                .to_string(),
        ],
        "distmat" => vec![
            repo_root
                .join("crates/emboss-tools/tests/fixtures/three_records.fasta")
                .display()
                .to_string(),
        ],
        "cons" | "consambig" => vec![
            repo_root
                .join("crates/emboss-tools/tests/fixtures/multiple_alignment.sto")
                .display()
                .to_string(),
        ],
        "needleall" => vec![
            repo_root
                .join("crates/emboss-tools/tests/fixtures/needleall_queries.fasta")
                .display()
                .to_string(),
            repo_root
                .join("crates/emboss-tools/tests/fixtures/needleall_targets.fasta")
                .display()
                .to_string(),
        ],
        "diffseq" => vec![
            repo_root
                .join("crates/emboss-tools/tests/fixtures/diffseq_left.fasta")
                .display()
                .to_string(),
            repo_root
                .join("crates/emboss-tools/tests/fixtures/diffseq_right.fasta")
                .display()
                .to_string(),
        ],
        "edialign" => vec![
            repo_root
                .join("crates/emboss-tools/tests/fixtures/edialign_records.fasta")
                .display()
                .to_string(),
        ],
        "water" => vec![
            repo_root
                .join("crates/emboss-tools/tests/fixtures/water_query.fasta")
                .display()
                .to_string(),
            repo_root
                .join("crates/emboss-tools/tests/fixtures/water_target.fasta")
                .display()
                .to_string(),
        ],
        "charge" => vec![
            repo_root
                .join("crates/emboss-tools/tests/fixtures/charge_protein.fasta")
                .display()
                .to_string(),
            "--window".to_owned(),
            "5".to_owned(),
            "--step".to_owned(),
            "1".to_owned(),
        ],
        "pepwindow" => vec![
            repo_root
                .join("crates/emboss-tools/tests/fixtures/pepwindow_protein.fasta")
                .display()
                .to_string(),
            "--window".to_owned(),
            "5".to_owned(),
            "--step".to_owned(),
            "2".to_owned(),
        ],
        "hmoment" => vec![
            repo_root
                .join("crates/emboss-tools/tests/fixtures/hmoment_protein.fasta")
                .display()
                .to_string(),
            "--window".to_owned(),
            "4".to_owned(),
            "--step".to_owned(),
            "1".to_owned(),
            "--angle-degrees".to_owned(),
            "100".to_owned(),
        ],
        "octanol" => vec![
            repo_root
                .join("crates/emboss-tools/tests/fixtures/octanol_protein.fasta")
                .display()
                .to_string(),
            "--window".to_owned(),
            "3".to_owned(),
            "--step".to_owned(),
            "1".to_owned(),
        ],
        "pepinfo" => vec![
            repo_root
                .join("crates/emboss-tools/tests/fixtures/pepinfo_protein.fasta")
                .display()
                .to_string(),
            "--window".to_owned(),
            "3".to_owned(),
            "--step".to_owned(),
            "1".to_owned(),
        ],
        "density" => vec![
            repo_root
                .join("crates/emboss-tools/tests/fixtures/density_nucleotide.fasta")
                .display()
                .to_string(),
            "--window".to_owned(),
            "4".to_owned(),
            "--step".to_owned(),
            "1".to_owned(),
        ],
        "wobble" => vec![
            repo_root
                .join("crates/emboss-tools/tests/fixtures/wobble_coding_nucleotide.fasta")
                .display()
                .to_string(),
            "--codon-window".to_owned(),
            "3".to_owned(),
            "--codon-step".to_owned(),
            "1".to_owned(),
        ],
        "isochore" => vec![
            repo_root
                .join("crates/emboss-tools/tests/fixtures/isochore_nucleotide.fasta")
                .display()
                .to_string(),
            "--window".to_owned(),
            "4".to_owned(),
            "--step".to_owned(),
            "4".to_owned(),
        ],
        "banana" => vec![
            repo_root
                .join("crates/emboss-tools/tests/fixtures/banana_nucleotide.fasta")
                .display()
                .to_string(),
        ],
        "syco" => vec![
            repo_root
                .join("crates/emboss-tools/tests/fixtures/syco_coding_nucleotide.fasta")
                .display()
                .to_string(),
            repo_root
                .join("crates/emboss-tools/tests/fixtures/codon_reference.fasta")
                .display()
                .to_string(),
            "--codon-window".to_owned(),
            "2".to_owned(),
            "--codon-step".to_owned(),
            "1".to_owned(),
        ],
        "descseq" => vec![
            repo_root
                .join("crates/emboss-tools/tests/fixtures/annotated_feature.gbk")
                .display()
                .to_string(),
        ],
        "geecee" => vec![
            repo_root
                .join("crates/emboss-tools/tests/fixtures/nucleotide_pattern_records.fasta")
                .display()
                .to_string(),
        ],
        "infoseq" => vec![
            repo_root
                .join("crates/emboss-tools/tests/fixtures/three_records.fasta")
                .display()
                .to_string(),
        ],
        "cusp" => vec![
            repo_root
                .join("crates/emboss-tools/tests/fixtures/codon_reference.fasta")
                .display()
                .to_string(),
        ],
        "fuzznuc" => vec![
            repo_root
                .join("crates/emboss-tools/tests/fixtures/nucleotide_pattern_records.fasta")
                .display()
                .to_string(),
            "ACGN".to_owned(),
        ],
        "fuzzpro" => vec![
            repo_root
                .join("crates/emboss-tools/tests/fixtures/protein_records.fasta")
                .display()
                .to_string(),
            "MX".to_owned(),
        ],
        "fuzztran" => vec![
            repo_root
                .join("crates/emboss-tools/tests/fixtures/checktrans_nucleotide.fasta")
                .display()
                .to_string(),
            "MA".to_owned(),
        ],
        "preg" => vec![
            repo_root
                .join("crates/emboss-tools/tests/fixtures/preg_records.fasta")
                .display()
                .to_string(),
            "MAM".to_owned(),
        ],
        "patmatdb" => vec![
            repo_root
                .join("crates/emboss-tools/tests/fixtures/patmatdb_records.fasta")
                .display()
                .to_string(),
            repo_root
                .join("crates/emboss-tools/tests/fixtures/patmatdb_motifs.tsv")
                .display()
                .to_string(),
        ],
        "wordmatch" => vec![
            repo_root
                .join("crates/emboss-tools/tests/fixtures/wordmatch_query.fasta")
                .display()
                .to_string(),
            repo_root
                .join("crates/emboss-tools/tests/fixtures/wordmatch_target.fasta")
                .display()
                .to_string(),
            "--word-size".to_owned(),
            "4".to_owned(),
        ],
        "wordfinder" => vec![
            repo_root
                .join("crates/emboss-tools/tests/fixtures/wordmatch_query.fasta")
                .display()
                .to_string(),
            repo_root
                .join("crates/emboss-tools/tests/fixtures/wordfinder_targets.fasta")
                .display()
                .to_string(),
            "--word-size".to_owned(),
            "4".to_owned(),
        ],
        "wossname" => vec![
            std::fs::read_to_string(
                repo_root.join("crates/emboss-tools/tests/fixtures/wossname_query.txt"),
            )
            .expect("wossname query fixture should read")
            .trim()
            .to_owned(),
        ],
        "seealso" => vec![
            std::fs::read_to_string(
                repo_root.join("crates/emboss-tools/tests/fixtures/seealso_query_tool.txt"),
            )
            .expect("seealso query-tool fixture should read")
            .trim()
            .to_owned(),
        ],
        "dreg" => vec![
            repo_root
                .join("crates/emboss-tools/tests/fixtures/dreg_records.fasta")
                .display()
                .to_string(),
            "ATA".to_owned(),
        ],
        "palindrome" => vec![
            repo_root
                .join("crates/emboss-tools/tests/fixtures/palindrome_records.fasta")
                .display()
                .to_string(),
            "--min-length".to_owned(),
            "6".to_owned(),
            "--max-length".to_owned(),
            "6".to_owned(),
        ],
        "einverted" => vec![
            repo_root
                .join("crates/emboss-tools/tests/fixtures/einverted_records.fasta")
                .display()
                .to_string(),
            "--min-arm-length".to_owned(),
            "4".to_owned(),
            "--max-gap-length".to_owned(),
            "2".to_owned(),
        ],
        "seqmatchall" => vec![
            repo_root
                .join("crates/emboss-tools/tests/fixtures/seqmatchall_records.fasta")
                .display()
                .to_string(),
            "--word-size".to_owned(),
            "4".to_owned(),
        ],
        _ => Vec::new(),
    }
}

fn render_payload(payload: &ResultPayload) -> Result<String, PlatformError> {
    match payload {
        ResultPayload::Alignment(alignment) => write_stockholm_string(alignment).map_err(|error| {
            PlatformError::new(
                ErrorCategory::Internal,
                "failed to render acceptance-anchor alignment payload",
            )
            .with_code("testkit.anchor.render_alignment_failed")
            .with_source(error)
        }),
        ResultPayload::Sequence(record) => write_fasta_string(std::slice::from_ref(record))
            .map_err(|error| {
                PlatformError::new(
                    ErrorCategory::Internal,
                    "failed to render acceptance-anchor sequence payload",
                )
                .with_code("testkit.anchor.render_sequence_failed")
                .with_source(error)
            }),
        ResultPayload::SequenceCollection(records) => {
            write_fasta_string(records).map_err(|error| {
                PlatformError::new(
                    ErrorCategory::Internal,
                    "failed to render acceptance-anchor sequence collection",
                )
                .with_code("testkit.anchor.render_sequence_collection_failed")
                .with_source(error)
            })
        }
        ResultPayload::SequencePartitions(partitions) => render_partitions(partitions),
        ResultPayload::TableReport(table) => Ok(render_table(table)),
        ResultPayload::TextReport(report) => Ok(report.body.clone()),
        other => Err(PlatformError::new(
            ErrorCategory::Validation,
            "acceptance-anchor payload kind is not currently comparable",
        )
        .with_code("testkit.anchor.payload.unsupported")
        .with_detail(other.kind_label().to_owned())),
    }
}

fn render_plot(
    plot: Option<&emboss_plot_contract::PlotPayload>,
) -> Result<Option<String>, PlatformError> {
    plot.map(|plot| {
        plot.to_json_pretty().map_err(|error| {
            PlatformError::new(
                ErrorCategory::Validation,
                "failed to render acceptance-anchor plot payload",
            )
            .with_code("testkit.anchor.render_plot_failed")
            .with_source(error)
        })
    })
    .transpose()
}

fn expected_plot_output(tool_name: &str) -> Option<&'static str> {
    match tool_name {
        "charge" => Some("crates/emboss-tools/tests/fixtures/charge_plot_contract.json"),
        "pepwindow" => Some("crates/emboss-tools/tests/fixtures/pepwindow_plot_contract.json"),
        "hmoment" => Some("crates/emboss-tools/tests/fixtures/hmoment_plot_contract.json"),
        "octanol" => Some("crates/emboss-tools/tests/fixtures/octanol_plot_contract.json"),
        "pepinfo" => Some("crates/emboss-tools/tests/fixtures/pepinfo_plot_contract.json"),
        "density" => Some("crates/emboss-tools/tests/fixtures/density_plot_contract.json"),
        "wobble" => Some("crates/emboss-tools/tests/fixtures/wobble_plot_contract.json"),
        "isochore" => Some("crates/emboss-tools/tests/fixtures/isochore_plot_contract.json"),
        "banana" => Some("crates/emboss-tools/tests/fixtures/banana_plot_contract.json"),
        "syco" => Some("crates/emboss-tools/tests/fixtures/syco_plot_contract.json"),
        "wordcount" => Some("crates/emboss-tools/tests/fixtures/wordcount_plot_contract.json"),
        _ => None,
    }
}

fn render_table(table: &emboss_service::TableReport) -> String {
    let mut rendered = String::new();
    if !table.columns.is_empty() {
        rendered.push_str(&table.columns.join("\t"));
        rendered.push('\n');
    }
    for row in &table.rows {
        rendered.push_str(&row.join("\t"));
        rendered.push('\n');
    }
    rendered
}

fn render_partitions(partitions: &[Vec<SequenceRecord>]) -> Result<String, PlatformError> {
    let mut rendered = String::new();
    for (index, partition) in partitions.iter().enumerate() {
        if index > 0 {
            rendered.push('\n');
        }
        rendered.push_str(&format!("# Partition {}\n", index + 1));
        let fasta = write_fasta_string(partition).map_err(|error| {
            PlatformError::new(
                ErrorCategory::Internal,
                "failed to render acceptance-anchor sequence partitions",
            )
            .with_code("testkit.anchor.render_sequence_partitions_failed")
            .with_source(error)
        })?;
        rendered.push_str(&fasta);
    }
    Ok(rendered)
}

fn mocked_provider_request(tool_name: &str) -> (InvocationRequest, AnchorMockHttpClient) {
    let request = InvocationRequest::new(
        ExecutionContext::default(),
        ToolName::new(tool_name).expect("mocked-provider anchor tool name must be valid"),
    );

    match tool_name {
        "refseqget" => (
            request.with_arguments(vec!["ncbi:protein:NP_000537.3".to_owned()]),
            AnchorMockHttpClient::default().with_response(
                "https://eutils.ncbi.nlm.nih.gov/entrez/eutils/efetch.fcgi?db=protein&id=NP_000537.3&rettype=fasta&retmode=text",
                HttpResponse::new(200, ">NP_000537.3 TP53\nMEEPQSDPSV\n"),
            ),
        ),
        "runinfo" => (
            request.with_arguments(vec!["ena:ERR123456".to_owned()]),
            AnchorMockHttpClient::default().with_response(
                "https://www.ebi.ac.uk/ena/portal/api/filereport?accession=ERR123456&result=read_run&fields=run_accession%2Cstudy_accession%2Cexperiment_accession%2Csample_accession%2Cinstrument_platform%2Cinstrument_model%2Clibrary_layout%2Clibrary_strategy%2Clibrary_source%2Cfastq_ftp%2Cfastq_md5%2Cfastq_bytes%2Csubmitted_ftp%2Csubmitted_md5%2Csubmitted_bytes%2Csra_ftp%2Csra_md5%2Csra_bytes&format=tsv&download=false",
                HttpResponse::new(200, "run_accession\tstudy_accession\texperiment_accession\tsample_accession\tinstrument_platform\tinstrument_model\tlibrary_layout\tlibrary_strategy\tlibrary_source\tfastq_ftp\tfastq_md5\tfastq_bytes\tsubmitted_ftp\tsubmitted_md5\tsubmitted_bytes\tsra_ftp\tsra_md5\tsra_bytes\nERR123456\tERP000001\tERX000001\tERS000001\tILLUMINA\tNovaSeq 6000\tPAIRED\tWGS\tGENOMIC\tftp.sra.ebi.ac.uk/vol1/fastq/ERR123/ERR123456/ERR123456_1.fastq.gz;ftp.sra.ebi.ac.uk/vol1/fastq/ERR123/ERR123456/ERR123456_2.fastq.gz\tmd51;md52\t10;12\t\t\t\t\t\t\n"),
            ),
        ),
        "runget" => (
            request.with_arguments(vec!["ena:ERR123456".to_owned()]),
            AnchorMockHttpClient::default().with_response(
                "https://www.ebi.ac.uk/ena/portal/api/filereport?accession=ERR123456&result=read_run&fields=run_accession%2Cstudy_accession%2Cexperiment_accession%2Csample_accession%2Cinstrument_platform%2Cinstrument_model%2Clibrary_layout%2Clibrary_strategy%2Clibrary_source%2Cfastq_ftp%2Cfastq_md5%2Cfastq_bytes%2Csubmitted_ftp%2Csubmitted_md5%2Csubmitted_bytes%2Csra_ftp%2Csra_md5%2Csra_bytes&format=tsv&download=false",
                HttpResponse::new(200, "run_accession\tstudy_accession\texperiment_accession\tsample_accession\tinstrument_platform\tinstrument_model\tlibrary_layout\tlibrary_strategy\tlibrary_source\tfastq_ftp\tfastq_md5\tfastq_bytes\tsubmitted_ftp\tsubmitted_md5\tsubmitted_bytes\tsra_ftp\tsra_md5\tsra_bytes\nERR123456\tERP000001\tERX000001\tERS000001\tILLUMINA\tNovaSeq 6000\tPAIRED\tWGS\tGENOMIC\tftp.sra.ebi.ac.uk/vol1/fastq/ERR123/ERR123456/ERR123456_1.fastq.gz;ftp.sra.ebi.ac.uk/vol1/fastq/ERR123/ERR123456/ERR123456_2.fastq.gz\tmd51;md52\t10;12\t\t\t\t\t\t\n"),
            ),
        ),
        "infoassembly" => (
            request.with_arguments(vec!["ena:ERR123456".to_owned()]),
            AnchorMockHttpClient::default().with_response(
                "https://www.ebi.ac.uk/ena/portal/api/filereport?accession=ERR123456&result=read_run&fields=run_accession%2Cstudy_accession%2Cexperiment_accession%2Csample_accession%2Cinstrument_platform%2Cinstrument_model%2Clibrary_layout%2Clibrary_strategy%2Clibrary_source%2Cfastq_ftp%2Cfastq_md5%2Cfastq_bytes%2Csubmitted_ftp%2Csubmitted_md5%2Csubmitted_bytes%2Csra_ftp%2Csra_md5%2Csra_bytes&format=tsv&download=false",
                HttpResponse::new(200, "run_accession\tstudy_accession\texperiment_accession\tsample_accession\tinstrument_platform\tinstrument_model\tlibrary_layout\tlibrary_strategy\tlibrary_source\tfastq_ftp\tfastq_md5\tfastq_bytes\tsubmitted_ftp\tsubmitted_md5\tsubmitted_bytes\tsra_ftp\tsra_md5\tsra_bytes\nERR123456\tERP000001\tERX000001\tERS000001\tILLUMINA\tNovaSeq 6000\tPAIRED\tWGS\tGENOMIC\tftp.sra.ebi.ac.uk/vol1/fastq/ERR123/ERR123456/ERR123456_1.fastq.gz;ftp.sra.ebi.ac.uk/vol1/fastq/ERR123/ERR123456/ERR123456_2.fastq.gz\tmd51;md52\t10;12\t\t\t\t\t\t\n"),
            ),
        ),
        other => panic!("unsupported mocked-provider anchor tool: {other}"),
    }
}

fn normalize_text(repo_root: &Path, text: &str) -> String {
    let repo_root = repo_root
        .canonicalize()
        .unwrap_or_else(|_| repo_root.to_path_buf());
    text.replace("\r\n", "\n")
        .trim_end()
        .lines()
        .map(|line| {
            line.split('\t')
                .map(|cell| normalize_repo_local_path(&repo_root, cell))
                .collect::<Vec<_>>()
                .join("\t")
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn normalize_repo_local_path(repo_root: &Path, cell: &str) -> String {
    let path = Path::new(cell);
    if path.is_absolute() {
        if let Ok(relative) = path.strip_prefix(repo_root) {
            return relative.display().to_string();
        }

        let normalized = cell.replace('\\', "/");
        if let Some((_, relative)) = normalized.split_once("/emboss-rs/") {
            return relative.to_owned();
        }
    }

    cell.to_owned()
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::{acceptance_anchor_specs, derive_acceptance_anchor_report, normalize_text};
    use crate::{ComparisonStatus, ExecutionStatus};

    fn repo_root() -> std::path::PathBuf {
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .canonicalize()
            .expect("repo root should resolve")
    }

    #[test]
    fn derives_passed_reports_for_every_acceptance_anchor() {
        let repo_root = repo_root();
        for spec in acceptance_anchor_specs() {
            let report = derive_acceptance_anchor_report(&repo_root, spec.tool_name)
                .expect("acceptance anchor should derive");
            assert!(report.summary.total_case_count >= 1);
            assert!(report.summary.executed_case_count >= 1);
            assert!(report.summary.compared_case_count >= 1);
            assert!(report.summary.passed_case_count >= 1);
            let case = report
                .cases
                .iter()
                .find(|case| case.id == spec.example_id)
                .expect("acceptance anchor should preserve its selected case");
            assert_eq!(case.execution_status, ExecutionStatus::Executed);
            assert_eq!(case.comparison_status, ComparisonStatus::Passed);
        }
    }

    #[test]
    fn normalizes_repo_local_absolute_paths_to_repo_relative_form() {
        let repo_root = Path::new("/home/runner/work/emboss-rs/emboss-rs");
        let text = "input\tcount\n/Users/stephen/Projects/emboss-rs/crates/emboss-tools/tests/fixtures/three_records.fasta\t3\n/home/runner/work/emboss-rs/emboss-rs/crates/emboss-tools/tests/fixtures/three_records.fasta\t3";

        let normalized = normalize_text(repo_root, text);

        assert_eq!(
            normalized,
            "input\tcount\ncrates/emboss-tools/tests/fixtures/three_records.fasta\t3\ncrates/emboss-tools/tests/fixtures/three_records.fasta\t3"
        );
    }
}

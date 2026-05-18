# Shipped Cohort Validation Report

This page is generated from the governed EMBOSS-RS tool registry plus checked-in autodoc and validation artefacts. It reports evidence maturity and visible gaps across the shipped method cohort.

## Summary

- Registry source: `emboss_tools::governed_tool_descriptors`
- Methods in cohort: `46`
- Documentation-complete methods: `46`
- Methods with validation stubs: `46`
- Documented-only methods: `8`
- Methods with declared evidence only: `1`
- Methods with harvested legacy evidence: `0`
- Methods with executable validation: `31`
- Methods with compared evidence: `6`
- Methods with visible gaps: `40`

## Evidence Level Definitions

- `documented_only`: the tool has documentation artefacts but no declared validation cases yet.
- `declared_evidence`: the tool has declared validation cases, but no runnable or executed evidence yet.
- `harvested_evidence`: the tool has legacy-derived or legacy-backed declared evidence.
- `executable_evidence`: the tool has at least one runnable or executed validation case.
- `compared_evidence`: the tool has at least one completed comparison result.

## Cohort Table

| Tool | Family | Evidence level | Docs | Stub | Harvested | Executable | Compared | Gap count |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `aligncopy` | `alignment_tools` | `executable_evidence` | `yes` | `yes` | `no` | `yes` | `no` | `2` |
| `aligncopypair` | `alignment_tools` | `executable_evidence` | `yes` | `yes` | `no` | `yes` | `no` | `2` |
| `infoalign` | `alignment_tools` | `executable_evidence` | `yes` | `yes` | `no` | `yes` | `no` | `2` |
| `extractalign` | `alignment_tools` | `executable_evidence` | `yes` | `yes` | `no` | `yes` | `no` | `2` |
| `runinfo` | `archive_tools` | `executable_evidence` | `yes` | `yes` | `no` | `yes` | `no` | `2` |
| `runget` | `archive_tools` | `executable_evidence` | `yes` | `yes` | `no` | `yes` | `no` | `2` |
| `matcher` | `alignment_analysis` | `executable_evidence` | `yes` | `yes` | `no` | `yes` | `no` | `2` |
| `distmat` | `alignment_analysis` | `executable_evidence` | `yes` | `yes` | `no` | `yes` | `no` | `2` |
| `cons` | `alignment_analysis` | `executable_evidence` | `yes` | `yes` | `no` | `yes` | `no` | `2` |
| `consambig` | `alignment_analysis` | `executable_evidence` | `yes` | `yes` | `no` | `yes` | `no` | `2` |
| `needle` | `pairwise_alignment` | `compared_evidence` | `yes` | `yes` | `yes` | `yes` | `yes` | `0` |
| `needleall` | `pairwise_alignment` | `executable_evidence` | `yes` | `yes` | `no` | `yes` | `no` | `2` |
| `seqret` | `retrieval_tools` | `compared_evidence` | `yes` | `yes` | `yes` | `yes` | `yes` | `0` |
| `refseqget` | `retrieval_tools` | `executable_evidence` | `yes` | `yes` | `no` | `yes` | `no` | `2` |
| `newseq` | `sequence_stream` | `declared_evidence` | `yes` | `yes` | `no` | `no` | `no` | `4` |
| `seqcount` | `sequence_stream` | `executable_evidence` | `yes` | `yes` | `no` | `yes` | `no` | `2` |
| `notseq` | `sequence_stream` | `executable_evidence` | `yes` | `yes` | `no` | `yes` | `no` | `2` |
| `nthseq` | `sequence_stream` | `executable_evidence` | `yes` | `yes` | `no` | `yes` | `no` | `2` |
| `skipseq` | `sequence_stream` | `executable_evidence` | `yes` | `yes` | `no` | `yes` | `no` | `2` |
| `degapseq` | `sequence_edit` | `documented_only` | `yes` | `yes` | `no` | `no` | `no` | `5` |
| `revseq` | `sequence_edit` | `executable_evidence` | `yes` | `yes` | `no` | `yes` | `no` | `2` |
| `trimseq` | `sequence_edit` | `documented_only` | `yes` | `yes` | `no` | `no` | `no` | `5` |
| `descseq` | `sequence_edit` | `executable_evidence` | `yes` | `yes` | `no` | `yes` | `no` | `2` |
| `maskseq` | `feature_tools` | `compared_evidence` | `yes` | `yes` | `yes` | `yes` | `yes` | `0` |
| `maskfeat` | `feature_tools` | `executable_evidence` | `yes` | `yes` | `no` | `yes` | `no` | `2` |
| `extractfeat` | `feature_tools` | `compared_evidence` | `yes` | `yes` | `yes` | `yes` | `yes` | `0` |
| `featcopy` | `feature_tools` | `executable_evidence` | `yes` | `yes` | `no` | `yes` | `no` | `2` |
| `cai` | `codon_tools` | `executable_evidence` | `yes` | `yes` | `no` | `yes` | `no` | `2` |
| `chips` | `codon_tools` | `executable_evidence` | `yes` | `yes` | `no` | `yes` | `no` | `2` |
| `codcmp` | `codon_tools` | `executable_evidence` | `yes` | `yes` | `no` | `yes` | `no` | `2` |
| `codcopy` | `codon_tools` | `executable_evidence` | `yes` | `yes` | `no` | `yes` | `no` | `2` |
| `fuzznuc` | `pattern_tools` | `executable_evidence` | `yes` | `yes` | `no` | `yes` | `no` | `2` |
| `fuzzpro` | `pattern_tools` | `executable_evidence` | `yes` | `yes` | `no` | `yes` | `no` | `2` |
| `fuzztran` | `pattern_tools` | `executable_evidence` | `yes` | `yes` | `no` | `yes` | `no` | `2` |
| `charge` | `protein_plots` | `documented_only` | `yes` | `yes` | `no` | `no` | `no` | `5` |
| `complex` | `sequence_stats` | `documented_only` | `yes` | `yes` | `no` | `no` | `no` | `5` |
| `compseq` | `sequence_stats` | `compared_evidence` | `yes` | `yes` | `yes` | `yes` | `yes` | `0` |
| `geecee` | `sequence_stats` | `executable_evidence` | `yes` | `yes` | `no` | `yes` | `no` | `2` |
| `pepstats` | `sequence_stats` | `compared_evidence` | `yes` | `yes` | `yes` | `yes` | `yes` | `0` |
| `backtranseq` | `translation_tools` | `documented_only` | `yes` | `yes` | `no` | `no` | `no` | `5` |
| `backtranambig` | `translation_tools` | `documented_only` | `yes` | `yes` | `no` | `no` | `no` | `5` |
| `checktrans` | `translation_tools` | `documented_only` | `yes` | `yes` | `no` | `no` | `no` | `5` |
| `extractseq` | `sequence_transform` | `executable_evidence` | `yes` | `yes` | `no` | `yes` | `no` | `2` |
| `cutseq` | `sequence_transform` | `executable_evidence` | `yes` | `yes` | `no` | `yes` | `no` | `2` |
| `union` | `sequence_transform` | `executable_evidence` | `yes` | `yes` | `no` | `yes` | `no` | `2` |
| `splitter` | `sequence_transform` | `documented_only` | `yes` | `yes` | `no` | `no` | `no` | `5` |

## Visible Gaps

- `aligncopy`: `missing_harvested_legacy_evidence`, `missing_compared_evidence`
- `aligncopypair`: `missing_harvested_legacy_evidence`, `missing_compared_evidence`
- `infoalign`: `missing_harvested_legacy_evidence`, `missing_compared_evidence`
- `extractalign`: `missing_harvested_legacy_evidence`, `missing_compared_evidence`
- `runinfo`: `missing_harvested_legacy_evidence`, `missing_compared_evidence`
- `runget`: `missing_harvested_legacy_evidence`, `missing_compared_evidence`
- `matcher`: `missing_harvested_legacy_evidence`, `missing_compared_evidence`
- `distmat`: `missing_harvested_legacy_evidence`, `missing_compared_evidence`
- `cons`: `missing_harvested_legacy_evidence`, `missing_compared_evidence`
- `consambig`: `missing_harvested_legacy_evidence`, `missing_compared_evidence`
- `needleall`: `missing_harvested_legacy_evidence`, `missing_compared_evidence`
- `refseqget`: `missing_harvested_legacy_evidence`, `missing_compared_evidence`
- `newseq`: `missing_harvested_legacy_evidence`, `missing_executable_evidence`, `missing_compared_evidence`, `validation_report_gap`
- `seqcount`: `missing_harvested_legacy_evidence`, `missing_compared_evidence`
- `notseq`: `missing_harvested_legacy_evidence`, `missing_compared_evidence`
- `nthseq`: `missing_harvested_legacy_evidence`, `missing_compared_evidence`
- `skipseq`: `missing_harvested_legacy_evidence`, `missing_compared_evidence`
- `degapseq`: `missing_validation_cases`, `missing_harvested_legacy_evidence`, `missing_executable_evidence`, `missing_compared_evidence`, `validation_report_gap`
- `revseq`: `missing_harvested_legacy_evidence`, `missing_compared_evidence`
- `trimseq`: `missing_validation_cases`, `missing_harvested_legacy_evidence`, `missing_executable_evidence`, `missing_compared_evidence`, `validation_report_gap`
- `descseq`: `missing_harvested_legacy_evidence`, `missing_compared_evidence`
- `maskfeat`: `missing_harvested_legacy_evidence`, `missing_compared_evidence`
- `featcopy`: `missing_harvested_legacy_evidence`, `missing_compared_evidence`
- `cai`: `missing_harvested_legacy_evidence`, `missing_compared_evidence`
- `chips`: `missing_harvested_legacy_evidence`, `missing_compared_evidence`
- `codcmp`: `missing_harvested_legacy_evidence`, `missing_compared_evidence`
- `codcopy`: `missing_harvested_legacy_evidence`, `missing_compared_evidence`
- `fuzznuc`: `missing_harvested_legacy_evidence`, `missing_compared_evidence`
- `fuzzpro`: `missing_harvested_legacy_evidence`, `missing_compared_evidence`
- `fuzztran`: `missing_harvested_legacy_evidence`, `missing_compared_evidence`
- `charge`: `missing_validation_cases`, `missing_harvested_legacy_evidence`, `missing_executable_evidence`, `missing_compared_evidence`, `validation_report_gap`
- `complex`: `missing_validation_cases`, `missing_harvested_legacy_evidence`, `missing_executable_evidence`, `missing_compared_evidence`, `validation_report_gap`
- `geecee`: `missing_harvested_legacy_evidence`, `missing_compared_evidence`
- `backtranseq`: `missing_validation_cases`, `missing_harvested_legacy_evidence`, `missing_executable_evidence`, `missing_compared_evidence`, `validation_report_gap`
- `backtranambig`: `missing_validation_cases`, `missing_harvested_legacy_evidence`, `missing_executable_evidence`, `missing_compared_evidence`, `validation_report_gap`
- `checktrans`: `missing_validation_cases`, `missing_harvested_legacy_evidence`, `missing_executable_evidence`, `missing_compared_evidence`, `validation_report_gap`
- `extractseq`: `missing_harvested_legacy_evidence`, `missing_compared_evidence`
- `cutseq`: `missing_harvested_legacy_evidence`, `missing_compared_evidence`
- `union`: `missing_harvested_legacy_evidence`, `missing_compared_evidence`
- `splitter`: `missing_validation_cases`, `missing_harvested_legacy_evidence`, `missing_executable_evidence`, `missing_compared_evidence`, `validation_report_gap`

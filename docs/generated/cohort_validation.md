# Shipped Cohort Validation Report

This page is generated from the governed EMBOSS-RS tool registry plus checked-in autodoc and validation artefacts. It reports evidence maturity and visible gaps across the shipped method cohort.

## Summary

- Registry source: `emboss_tools::governed_tool_descriptors`
- Methods in cohort: `96`
- Documentation-complete methods: `96`
- Methods with validation stubs: `96`
- Documented-only methods: `0`
- Methods with declared evidence only: `0`
- Methods at harvested-evidence maturity: `0`
- Methods with harvested legacy provenance recorded: `88`
- Methods with executable validation: `44`
- Methods with compared evidence: `52`
- Methods with visible gaps: `44`

## Evidence Level Definitions

- `documented_only`: the tool has documentation artefacts but no declared validation cases yet.
- `declared_evidence`: the tool has declared validation cases, but no runnable or executed evidence yet.
- `harvested_evidence`: the tool has legacy-derived or legacy-backed declared evidence.
- `executable_evidence`: the tool has at least one runnable or executed validation case.
- `compared_evidence`: the tool has at least one completed comparison result.

## Cohort Table

| Tool | Family | Evidence level | Docs | Stub | Harvested | Executable | Compared | Gap count |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `aligncopy` | `alignment_tools` | `compared_evidence` | `yes` | `yes` | `yes` | `yes` | `yes` | `0` |
| `aligncopypair` | `alignment_tools` | `compared_evidence` | `yes` | `yes` | `yes` | `yes` | `yes` | `0` |
| `diffseq` | `alignment_tools` | `compared_evidence` | `yes` | `yes` | `yes` | `yes` | `yes` | `0` |
| `edialign` | `alignment_tools` | `compared_evidence` | `yes` | `yes` | `yes` | `yes` | `yes` | `0` |
| `infoalign` | `alignment_tools` | `compared_evidence` | `yes` | `yes` | `yes` | `yes` | `yes` | `0` |
| `extractalign` | `alignment_tools` | `compared_evidence` | `yes` | `yes` | `yes` | `yes` | `yes` | `0` |
| `nthseqset` | `alignment_tools` | `compared_evidence` | `yes` | `yes` | `yes` | `yes` | `yes` | `0` |
| `runinfo` | `archive_tools` | `compared_evidence` | `yes` | `yes` | `yes` | `yes` | `yes` | `0` |
| `runget` | `archive_tools` | `compared_evidence` | `yes` | `yes` | `yes` | `yes` | `yes` | `0` |
| `matcher` | `alignment_analysis` | `compared_evidence` | `yes` | `yes` | `yes` | `yes` | `yes` | `0` |
| `distmat` | `alignment_analysis` | `compared_evidence` | `yes` | `yes` | `yes` | `yes` | `yes` | `0` |
| `cons` | `alignment_analysis` | `compared_evidence` | `yes` | `yes` | `yes` | `yes` | `yes` | `0` |
| `consambig` | `alignment_analysis` | `compared_evidence` | `yes` | `yes` | `yes` | `yes` | `yes` | `0` |
| `needle` | `pairwise_alignment` | `compared_evidence` | `yes` | `yes` | `yes` | `yes` | `yes` | `0` |
| `needleall` | `pairwise_alignment` | `compared_evidence` | `yes` | `yes` | `yes` | `yes` | `yes` | `0` |
| `water` | `pairwise_alignment` | `compared_evidence` | `yes` | `yes` | `yes` | `yes` | `yes` | `0` |
| `seqret` | `retrieval_tools` | `compared_evidence` | `yes` | `yes` | `yes` | `yes` | `yes` | `0` |
| `refseqget` | `retrieval_tools` | `compared_evidence` | `yes` | `yes` | `yes` | `yes` | `yes` | `0` |
| `newseq` | `sequence_stream` | `compared_evidence` | `yes` | `yes` | `yes` | `yes` | `yes` | `0` |
| `makenucseq` | `sequence_stream` | `executable_evidence` | `yes` | `yes` | `yes` | `yes` | `no` | `1` |
| `makeprotseq` | `sequence_stream` | `executable_evidence` | `yes` | `yes` | `yes` | `yes` | `no` | `1` |
| `seqcount` | `sequence_stream` | `compared_evidence` | `yes` | `yes` | `yes` | `yes` | `yes` | `0` |
| `notseq` | `sequence_stream` | `compared_evidence` | `yes` | `yes` | `yes` | `yes` | `yes` | `0` |
| `nthseq` | `sequence_stream` | `compared_evidence` | `yes` | `yes` | `yes` | `yes` | `yes` | `0` |
| `skipseq` | `sequence_stream` | `compared_evidence` | `yes` | `yes` | `yes` | `yes` | `yes` | `0` |
| `listor` | `sequence_stream` | `compared_evidence` | `yes` | `yes` | `yes` | `yes` | `yes` | `0` |
| `skipredundant` | `sequence_stream` | `compared_evidence` | `yes` | `yes` | `yes` | `yes` | `yes` | `0` |
| `biosed` | `sequence_edit` | `executable_evidence` | `yes` | `yes` | `yes` | `yes` | `no` | `1` |
| `degapseq` | `sequence_edit` | `executable_evidence` | `yes` | `yes` | `no` | `yes` | `no` | `2` |
| `revseq` | `sequence_edit` | `executable_evidence` | `yes` | `yes` | `yes` | `yes` | `no` | `1` |
| `msbar` | `sequence_edit` | `executable_evidence` | `yes` | `yes` | `yes` | `yes` | `no` | `1` |
| `trimest` | `sequence_edit` | `executable_evidence` | `yes` | `yes` | `yes` | `yes` | `no` | `1` |
| `trimseq` | `sequence_edit` | `executable_evidence` | `yes` | `yes` | `yes` | `yes` | `no` | `1` |
| `descseq` | `sequence_edit` | `compared_evidence` | `yes` | `yes` | `yes` | `yes` | `yes` | `0` |
| `vectorstrip` | `sequence_edit` | `executable_evidence` | `yes` | `yes` | `yes` | `yes` | `no` | `1` |
| `maskseq` | `feature_tools` | `compared_evidence` | `yes` | `yes` | `yes` | `yes` | `yes` | `0` |
| `maskambignuc` | `feature_tools` | `executable_evidence` | `yes` | `yes` | `yes` | `yes` | `no` | `1` |
| `maskambigprot` | `feature_tools` | `executable_evidence` | `yes` | `yes` | `yes` | `yes` | `no` | `1` |
| `maskfeat` | `feature_tools` | `executable_evidence` | `yes` | `yes` | `yes` | `yes` | `no` | `1` |
| `extractfeat` | `feature_tools` | `compared_evidence` | `yes` | `yes` | `yes` | `yes` | `yes` | `0` |
| `featcopy` | `feature_tools` | `executable_evidence` | `yes` | `yes` | `no` | `yes` | `no` | `2` |
| `coderet` | `feature_tools` | `executable_evidence` | `yes` | `yes` | `yes` | `yes` | `no` | `1` |
| `featmerge` | `feature_tools` | `executable_evidence` | `yes` | `yes` | `yes` | `yes` | `no` | `1` |
| `featreport` | `feature_tools` | `executable_evidence` | `yes` | `yes` | `yes` | `yes` | `no` | `1` |
| `feattext` | `feature_tools` | `executable_evidence` | `yes` | `yes` | `yes` | `yes` | `no` | `1` |
| `splitsource` | `feature_tools` | `executable_evidence` | `yes` | `yes` | `no` | `yes` | `no` | `2` |
| `twofeat` | `feature_tools` | `executable_evidence` | `yes` | `yes` | `yes` | `yes` | `no` | `1` |
| `cai` | `codon_tools` | `executable_evidence` | `yes` | `yes` | `no` | `yes` | `no` | `2` |
| `chips` | `codon_tools` | `executable_evidence` | `yes` | `yes` | `no` | `yes` | `no` | `2` |
| `cusp` | `codon_tools` | `compared_evidence` | `yes` | `yes` | `yes` | `yes` | `yes` | `0` |
| `codcmp` | `codon_tools` | `executable_evidence` | `yes` | `yes` | `no` | `yes` | `no` | `2` |
| `codcopy` | `codon_tools` | `executable_evidence` | `yes` | `yes` | `no` | `yes` | `no` | `2` |
| `dreg` | `pattern_tools` | `executable_evidence` | `yes` | `yes` | `yes` | `yes` | `no` | `1` |
| `einverted` | `pattern_tools` | `executable_evidence` | `yes` | `yes` | `yes` | `yes` | `no` | `1` |
| `fuzznuc` | `pattern_tools` | `compared_evidence` | `yes` | `yes` | `yes` | `yes` | `yes` | `0` |
| `fuzzpro` | `pattern_tools` | `compared_evidence` | `yes` | `yes` | `yes` | `yes` | `yes` | `0` |
| `fuzztran` | `pattern_tools` | `compared_evidence` | `yes` | `yes` | `yes` | `yes` | `yes` | `0` |
| `palindrome` | `pattern_tools` | `executable_evidence` | `yes` | `yes` | `yes` | `yes` | `no` | `1` |
| `preg` | `pattern_tools` | `executable_evidence` | `yes` | `yes` | `yes` | `yes` | `no` | `1` |
| `patmatdb` | `pattern_tools` | `executable_evidence` | `yes` | `yes` | `yes` | `yes` | `no` | `1` |
| `seqmatchall` | `pattern_tools` | `executable_evidence` | `yes` | `yes` | `yes` | `yes` | `no` | `1` |
| `wordmatch` | `pattern_tools` | `executable_evidence` | `yes` | `yes` | `yes` | `yes` | `no` | `1` |
| `wordfinder` | `pattern_tools` | `executable_evidence` | `yes` | `yes` | `yes` | `yes` | `no` | `1` |
| `charge` | `protein_plots` | `executable_evidence` | `yes` | `yes` | `yes` | `yes` | `no` | `2` |
| `pepwindow` | `protein_plots` | `executable_evidence` | `yes` | `yes` | `yes` | `yes` | `no` | `2` |
| `recoder` | `restriction_tools` | `executable_evidence` | `yes` | `yes` | `yes` | `yes` | `no` | `1` |
| `silent` | `restriction_tools` | `executable_evidence` | `yes` | `yes` | `yes` | `yes` | `no` | `1` |
| `aaindexextract` | `sequence_stats` | `executable_evidence` | `yes` | `yes` | `yes` | `yes` | `no` | `1` |
| `complex` | `sequence_stats` | `executable_evidence` | `yes` | `yes` | `no` | `yes` | `no` | `2` |
| `compseq` | `sequence_stats` | `compared_evidence` | `yes` | `yes` | `yes` | `yes` | `yes` | `0` |
| `dan` | `sequence_stats` | `executable_evidence` | `yes` | `yes` | `yes` | `yes` | `no` | `1` |
| `geecee` | `sequence_stats` | `compared_evidence` | `yes` | `yes` | `yes` | `yes` | `yes` | `0` |
| `infobase` | `sequence_stats` | `executable_evidence` | `yes` | `yes` | `yes` | `yes` | `no` | `1` |
| `infoseq` | `sequence_stats` | `compared_evidence` | `yes` | `yes` | `yes` | `yes` | `yes` | `0` |
| `inforesidue` | `sequence_stats` | `executable_evidence` | `yes` | `yes` | `yes` | `yes` | `no` | `1` |
| `iep` | `sequence_stats` | `executable_evidence` | `yes` | `yes` | `yes` | `yes` | `no` | `1` |
| `oddcomp` | `sequence_stats` | `executable_evidence` | `yes` | `yes` | `yes` | `yes` | `no` | `1` |
| `pepdigest` | `sequence_stats` | `executable_evidence` | `yes` | `yes` | `yes` | `yes` | `no` | `1` |
| `pepstats` | `sequence_stats` | `compared_evidence` | `yes` | `yes` | `yes` | `yes` | `yes` | `0` |
| `wordcount` | `sequence_stats` | `executable_evidence` | `yes` | `yes` | `yes` | `yes` | `no` | `1` |
| `backtranseq` | `translation_tools` | `compared_evidence` | `yes` | `yes` | `yes` | `yes` | `yes` | `0` |
| `backtranambig` | `translation_tools` | `compared_evidence` | `yes` | `yes` | `yes` | `yes` | `yes` | `0` |
| `checktrans` | `translation_tools` | `compared_evidence` | `yes` | `yes` | `yes` | `yes` | `yes` | `0` |
| `transeq` | `translation_tools` | `compared_evidence` | `yes` | `yes` | `yes` | `yes` | `yes` | `0` |
| `getorf` | `translation_tools` | `compared_evidence` | `yes` | `yes` | `yes` | `yes` | `yes` | `0` |
| `prettyseq` | `translation_tools` | `compared_evidence` | `yes` | `yes` | `yes` | `yes` | `yes` | `0` |
| `tranalign` | `translation_tools` | `compared_evidence` | `yes` | `yes` | `yes` | `yes` | `yes` | `0` |
| `extractseq` | `sequence_transform` | `compared_evidence` | `yes` | `yes` | `yes` | `yes` | `yes` | `0` |
| `cutseq` | `sequence_transform` | `compared_evidence` | `yes` | `yes` | `yes` | `yes` | `yes` | `0` |
| `union` | `sequence_transform` | `compared_evidence` | `yes` | `yes` | `yes` | `yes` | `yes` | `0` |
| `pasteseq` | `sequence_transform` | `compared_evidence` | `yes` | `yes` | `yes` | `yes` | `yes` | `0` |
| `splitter` | `sequence_transform` | `compared_evidence` | `yes` | `yes` | `yes` | `yes` | `yes` | `0` |
| `merger` | `sequence_transform` | `compared_evidence` | `yes` | `yes` | `yes` | `yes` | `yes` | `0` |
| `megamerger` | `sequence_transform` | `compared_evidence` | `yes` | `yes` | `yes` | `yes` | `yes` | `0` |
| `sizeseq` | `sequence_transform` | `compared_evidence` | `yes` | `yes` | `yes` | `yes` | `yes` | `0` |
| `shuffleseq` | `sequence_transform` | `compared_evidence` | `yes` | `yes` | `yes` | `yes` | `yes` | `0` |

## Visible Gaps

- `makenucseq`: `missing_compared_evidence`
- `makeprotseq`: `missing_compared_evidence`
- `biosed`: `missing_compared_evidence`
- `degapseq`: `missing_harvested_legacy_evidence`, `missing_compared_evidence`
- `revseq`: `missing_compared_evidence`
- `msbar`: `missing_compared_evidence`
- `trimest`: `missing_compared_evidence`
- `trimseq`: `missing_compared_evidence`
- `vectorstrip`: `missing_compared_evidence`
- `maskambignuc`: `missing_compared_evidence`
- `maskambigprot`: `missing_compared_evidence`
- `maskfeat`: `missing_compared_evidence`
- `featcopy`: `missing_harvested_legacy_evidence`, `missing_compared_evidence`
- `coderet`: `missing_compared_evidence`
- `featmerge`: `missing_compared_evidence`
- `featreport`: `missing_compared_evidence`
- `feattext`: `missing_compared_evidence`
- `splitsource`: `missing_harvested_legacy_evidence`, `missing_compared_evidence`
- `twofeat`: `missing_compared_evidence`
- `cai`: `missing_harvested_legacy_evidence`, `missing_compared_evidence`
- `chips`: `missing_harvested_legacy_evidence`, `missing_compared_evidence`
- `codcmp`: `missing_harvested_legacy_evidence`, `missing_compared_evidence`
- `codcopy`: `missing_harvested_legacy_evidence`, `missing_compared_evidence`
- `dreg`: `missing_compared_evidence`
- `einverted`: `missing_compared_evidence`
- `palindrome`: `missing_compared_evidence`
- `preg`: `missing_compared_evidence`
- `patmatdb`: `missing_compared_evidence`
- `seqmatchall`: `missing_compared_evidence`
- `wordmatch`: `missing_compared_evidence`
- `wordfinder`: `missing_compared_evidence`
- `charge`: `missing_compared_evidence`, `validation_report_gap`
- `pepwindow`: `missing_compared_evidence`, `validation_report_gap`
- `recoder`: `missing_compared_evidence`
- `silent`: `missing_compared_evidence`
- `aaindexextract`: `missing_compared_evidence`
- `complex`: `missing_harvested_legacy_evidence`, `missing_compared_evidence`
- `dan`: `missing_compared_evidence`
- `infobase`: `missing_compared_evidence`
- `inforesidue`: `missing_compared_evidence`
- `iep`: `missing_compared_evidence`
- `oddcomp`: `missing_compared_evidence`
- `pepdigest`: `missing_compared_evidence`
- `wordcount`: `missing_compared_evidence`

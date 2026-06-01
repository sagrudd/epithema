# Shipped Cohort Validation Report

This page is generated from the governed EMBOSS-RS tool registry plus checked-in autodoc and validation artefacts. It reports evidence maturity and visible gaps across the shipped method cohort.

## Summary

- Registry source: `emboss_tools::governed_tool_descriptors`
- Methods in cohort: `103`
- Documentation-complete methods: `103`
- Methods with validation stubs: `103`
- Documented-only methods: `0`
- Methods with declared evidence only: `0`
- Methods at harvested-evidence maturity: `0`
- Methods with harvested legacy provenance recorded: `103`
- Methods with executable validation: `1`
- Methods with compared evidence: `102`
- Methods with blocking cohort gaps: `1`

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
| `makenucseq` | `sequence_stream` | `compared_evidence` | `yes` | `yes` | `yes` | `yes` | `yes` | `0` |
| `makeprotseq` | `sequence_stream` | `compared_evidence` | `yes` | `yes` | `yes` | `yes` | `yes` | `0` |
| `seqcount` | `sequence_stream` | `compared_evidence` | `yes` | `yes` | `yes` | `yes` | `yes` | `0` |
| `notseq` | `sequence_stream` | `compared_evidence` | `yes` | `yes` | `yes` | `yes` | `yes` | `0` |
| `nthseq` | `sequence_stream` | `compared_evidence` | `yes` | `yes` | `yes` | `yes` | `yes` | `0` |
| `skipseq` | `sequence_stream` | `compared_evidence` | `yes` | `yes` | `yes` | `yes` | `yes` | `0` |
| `listor` | `sequence_stream` | `compared_evidence` | `yes` | `yes` | `yes` | `yes` | `yes` | `0` |
| `skipredundant` | `sequence_stream` | `compared_evidence` | `yes` | `yes` | `yes` | `yes` | `yes` | `0` |
| `biosed` | `sequence_edit` | `compared_evidence` | `yes` | `yes` | `yes` | `yes` | `yes` | `0` |
| `degapseq` | `sequence_edit` | `compared_evidence` | `yes` | `yes` | `yes` | `yes` | `yes` | `0` |
| `revseq` | `sequence_edit` | `compared_evidence` | `yes` | `yes` | `yes` | `yes` | `yes` | `0` |
| `msbar` | `sequence_edit` | `compared_evidence` | `yes` | `yes` | `yes` | `yes` | `yes` | `0` |
| `trimest` | `sequence_edit` | `compared_evidence` | `yes` | `yes` | `yes` | `yes` | `yes` | `0` |
| `trimseq` | `sequence_edit` | `compared_evidence` | `yes` | `yes` | `yes` | `yes` | `yes` | `0` |
| `descseq` | `sequence_edit` | `compared_evidence` | `yes` | `yes` | `yes` | `yes` | `yes` | `0` |
| `vectorstrip` | `sequence_edit` | `compared_evidence` | `yes` | `yes` | `yes` | `yes` | `yes` | `0` |
| `maskseq` | `feature_tools` | `compared_evidence` | `yes` | `yes` | `yes` | `yes` | `yes` | `0` |
| `maskambignuc` | `feature_tools` | `compared_evidence` | `yes` | `yes` | `yes` | `yes` | `yes` | `0` |
| `maskambigprot` | `feature_tools` | `compared_evidence` | `yes` | `yes` | `yes` | `yes` | `yes` | `0` |
| `maskfeat` | `feature_tools` | `compared_evidence` | `yes` | `yes` | `yes` | `yes` | `yes` | `0` |
| `extractfeat` | `feature_tools` | `compared_evidence` | `yes` | `yes` | `yes` | `yes` | `yes` | `0` |
| `featcopy` | `feature_tools` | `compared_evidence` | `yes` | `yes` | `yes` | `yes` | `yes` | `0` |
| `coderet` | `feature_tools` | `compared_evidence` | `yes` | `yes` | `yes` | `yes` | `yes` | `0` |
| `featmerge` | `feature_tools` | `compared_evidence` | `yes` | `yes` | `yes` | `yes` | `yes` | `0` |
| `featreport` | `feature_tools` | `compared_evidence` | `yes` | `yes` | `yes` | `yes` | `yes` | `0` |
| `feattext` | `feature_tools` | `compared_evidence` | `yes` | `yes` | `yes` | `yes` | `yes` | `0` |
| `splitsource` | `feature_tools` | `compared_evidence` | `yes` | `yes` | `yes` | `yes` | `yes` | `0` |
| `twofeat` | `feature_tools` | `compared_evidence` | `yes` | `yes` | `yes` | `yes` | `yes` | `0` |
| `cai` | `codon_tools` | `compared_evidence` | `yes` | `yes` | `yes` | `yes` | `yes` | `0` |
| `chips` | `codon_tools` | `compared_evidence` | `yes` | `yes` | `yes` | `yes` | `yes` | `0` |
| `cusp` | `codon_tools` | `compared_evidence` | `yes` | `yes` | `yes` | `yes` | `yes` | `0` |
| `codcmp` | `codon_tools` | `compared_evidence` | `yes` | `yes` | `yes` | `yes` | `yes` | `0` |
| `codcopy` | `codon_tools` | `compared_evidence` | `yes` | `yes` | `yes` | `yes` | `yes` | `0` |
| `dreg` | `pattern_tools` | `compared_evidence` | `yes` | `yes` | `yes` | `yes` | `yes` | `0` |
| `einverted` | `pattern_tools` | `compared_evidence` | `yes` | `yes` | `yes` | `yes` | `yes` | `0` |
| `fuzznuc` | `pattern_tools` | `compared_evidence` | `yes` | `yes` | `yes` | `yes` | `yes` | `0` |
| `fuzzpro` | `pattern_tools` | `compared_evidence` | `yes` | `yes` | `yes` | `yes` | `yes` | `0` |
| `fuzztran` | `pattern_tools` | `compared_evidence` | `yes` | `yes` | `yes` | `yes` | `yes` | `0` |
| `palindrome` | `pattern_tools` | `compared_evidence` | `yes` | `yes` | `yes` | `yes` | `yes` | `0` |
| `preg` | `pattern_tools` | `compared_evidence` | `yes` | `yes` | `yes` | `yes` | `yes` | `0` |
| `patmatdb` | `pattern_tools` | `compared_evidence` | `yes` | `yes` | `yes` | `yes` | `yes` | `0` |
| `seqmatchall` | `pattern_tools` | `compared_evidence` | `yes` | `yes` | `yes` | `yes` | `yes` | `0` |
| `wordmatch` | `pattern_tools` | `compared_evidence` | `yes` | `yes` | `yes` | `yes` | `yes` | `0` |
| `wordfinder` | `pattern_tools` | `compared_evidence` | `yes` | `yes` | `yes` | `yes` | `yes` | `0` |
| `banana` | `nucleotide_plots` | `executable_evidence` | `yes` | `yes` | `yes` | `yes` | `no` | `2` |
| `density` | `nucleotide_plots` | `compared_evidence` | `yes` | `yes` | `yes` | `yes` | `yes` | `1` |
| `wobble` | `nucleotide_plots` | `compared_evidence` | `yes` | `yes` | `yes` | `yes` | `yes` | `1` |
| `isochore` | `nucleotide_plots` | `compared_evidence` | `yes` | `yes` | `yes` | `yes` | `yes` | `1` |
| `charge` | `protein_plots` | `compared_evidence` | `yes` | `yes` | `yes` | `yes` | `yes` | `1` |
| `hmoment` | `protein_plots` | `compared_evidence` | `yes` | `yes` | `yes` | `yes` | `yes` | `1` |
| `octanol` | `protein_plots` | `compared_evidence` | `yes` | `yes` | `yes` | `yes` | `yes` | `1` |
| `pepinfo` | `protein_plots` | `compared_evidence` | `yes` | `yes` | `yes` | `yes` | `yes` | `1` |
| `pepwindow` | `protein_plots` | `compared_evidence` | `yes` | `yes` | `yes` | `yes` | `yes` | `1` |
| `recoder` | `restriction_tools` | `compared_evidence` | `yes` | `yes` | `yes` | `yes` | `yes` | `0` |
| `silent` | `restriction_tools` | `compared_evidence` | `yes` | `yes` | `yes` | `yes` | `yes` | `0` |
| `aaindexextract` | `sequence_stats` | `compared_evidence` | `yes` | `yes` | `yes` | `yes` | `yes` | `0` |
| `complex` | `sequence_stats` | `compared_evidence` | `yes` | `yes` | `yes` | `yes` | `yes` | `0` |
| `compseq` | `sequence_stats` | `compared_evidence` | `yes` | `yes` | `yes` | `yes` | `yes` | `0` |
| `dan` | `sequence_stats` | `compared_evidence` | `yes` | `yes` | `yes` | `yes` | `yes` | `0` |
| `geecee` | `sequence_stats` | `compared_evidence` | `yes` | `yes` | `yes` | `yes` | `yes` | `0` |
| `infobase` | `sequence_stats` | `compared_evidence` | `yes` | `yes` | `yes` | `yes` | `yes` | `0` |
| `infoseq` | `sequence_stats` | `compared_evidence` | `yes` | `yes` | `yes` | `yes` | `yes` | `0` |
| `inforesidue` | `sequence_stats` | `compared_evidence` | `yes` | `yes` | `yes` | `yes` | `yes` | `0` |
| `iep` | `sequence_stats` | `compared_evidence` | `yes` | `yes` | `yes` | `yes` | `yes` | `0` |
| `oddcomp` | `sequence_stats` | `compared_evidence` | `yes` | `yes` | `yes` | `yes` | `yes` | `0` |
| `pepdigest` | `sequence_stats` | `compared_evidence` | `yes` | `yes` | `yes` | `yes` | `yes` | `0` |
| `pepstats` | `sequence_stats` | `compared_evidence` | `yes` | `yes` | `yes` | `yes` | `yes` | `0` |
| `wordcount` | `sequence_stats` | `compared_evidence` | `yes` | `yes` | `yes` | `yes` | `yes` | `0` |
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

Visible gaps may include non-blocking notes that do not lower the tool's current evidence maturity or contribute to the blocking cohort-gap count above. In the current zero-burden state, the remaining visible plotting notes reflect missing explicit legacy-reference artefacts rather than missing compared evidence.

- `banana`: `missing_compared_evidence`, `missing_explicit_legacy_reference`
- `density`: `missing_explicit_legacy_reference`
- `wobble`: `missing_explicit_legacy_reference`
- `isochore`: `missing_explicit_legacy_reference`
- `charge`: `missing_explicit_legacy_reference`
- `hmoment`: `missing_explicit_legacy_reference`
- `octanol`: `missing_explicit_legacy_reference`
- `pepinfo`: `missing_explicit_legacy_reference`
- `pepwindow`: `missing_explicit_legacy_reference`

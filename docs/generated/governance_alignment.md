# Governance and Registry Alignment Report

This page is generated from the maintained governance family-to-tool appendix, the shipped EMBOSS-RS Rust registry, and the cohort validation report. It exists to keep backlog truth, shipped scope, curated autodoc coverage, and evidence depth aligned.

## Summary

- Governance source: `docs/governance/appendices/family_to_tool_mapping_reference.md`
- Registry source: `emboss_tools::governed_tool_descriptors`
- Governed mapped tools: `265`
- Governed retained tools: `90`
- Shipped tools: `101`
- Shipped tools with governance mapping: `101`
- Retained backlog still unshipped: `0`
- Shipped tools with curated autodoc: `101`
- Shipped tools with executable or compared evidence: `101`
- Shipped tools with harvested legacy provenance: `101`
- Shipped tools with compared evidence: `100`
- Shipped tools still documented-only: `0`

## Shipped Decision Split

- Shipped retain methods: `90`
- Shipped rework methods: `11`
- Shipped omit methods: `0`
- Shipped add methods: `0`

## Family Reconciliation

| Governance family | Retained total | Retained shipped | Retained backlog | Shipped curated | Shipped executable+ | Shipped compared | Recommendation |
|---|---:|---:|---:|---:|---:|---:|---|
| Core Retain — Alignment read-write and post-processing | 13 | 13 | 0 | 13 | 13 | 13 | family is aligned at the current governance and evidence threshold |
| Core Retain — Basic sequence IO and conversion | 18 | 18 | 0 | 18 | 18 | 18 | family is aligned at the current governance and evidence threshold |
| Core Retain — Core sequence statistics and composition | 16 | 16 | 0 | 16 | 16 | 16 | family is aligned at the current governance and evidence threshold |
| Core Retain — ORF and translation-adjacent utilities | 4 | 4 | 0 | 4 | 4 | 4 | family is aligned at the current governance and evidence threshold |
| Core Retain — Sequence editing and manipulation | 23 | 23 | 0 | 23 | 23 | 23 | family is aligned at the current governance and evidence threshold |
| Core Retain — Simple motif, pattern, and regular-expression search | 12 | 12 | 0 | 12 | 12 | 12 | family is aligned at the current governance and evidence threshold |
| Defer — Ontology command group | 0 | 0 | 0 | 0 | 0 | 0 | family is aligned at the current governance and evidence threshold |
| Defer — Specialized metadata and semantic lookup utilities | 0 | 0 | 0 | 0 | 0 | 0 | family is aligned at the current governance and evidence threshold |
| Exclude Permanently — ACD developer tooling | 0 | 0 | 0 | 0 | 0 | 0 | family is aligned at the current governance and evidence threshold |
| Exclude Permanently — EMBOSS local database indexing administration | 0 | 0 | 0 | 0 | 0 | 0 | family is aligned at the current governance and evidence threshold |
| Exclude Permanently — EMBOSS-era server-cache-registry plumbing | 0 | 0 | 0 | 0 | 0 | 0 | family is aligned at the current governance and evidence threshold |
| Exclude Permanently — Wrapper-only compatibility commands | 0 | 0 | 0 | 0 | 0 | 0 | family is aligned at the current governance and evidence threshold |
| Modernize — Rework — Command discovery and help-navigation | 0 | 0 | 0 | 0 | 0 | 0 | family is aligned at the current governance and evidence threshold |
| Modernize — Rework — External database preparation helpers | 0 | 0 | 0 | 0 | 0 | 0 | family is aligned at the current governance and evidence threshold |
| Modernize — Rework — Legacy prediction methods with enduring scientific value | 0 | 0 | 0 | 0 | 0 | 0 | family is aligned at the current governance and evidence threshold |
| Modernize — Rework — Plotting and visualization tools | 0 | 0 | 0 | 7 | 7 | 6 | family is aligned at the current governance and evidence threshold |
| Modernize — Rework — Primer and assay-oriented search | 0 | 0 | 0 | 0 | 0 | 0 | family is aligned at the current governance and evidence threshold |
| Modernize — Rework — Protein property and structural-summary utilities | 2 | 2 | 0 | 2 | 2 | 2 | family is aligned at the current governance and evidence threshold |
| Modernize — Rework — Remote retrieval and archive acquisition | 0 | 0 | 0 | 4 | 4 | 4 | family is aligned at the current governance and evidence threshold |
| Modernize — Rework — Restriction-enzyme design and analysis | 2 | 2 | 0 | 2 | 2 | 2 | family is aligned at the current governance and evidence threshold |
| Strategic Add — HMM and probabilistic homology workflows | 0 | 0 | 0 | 0 | 0 | 0 | family is aligned at the current governance and evidence threshold |
| Strategic Add — Modern archive-scale raw data ingestion | 0 | 0 | 0 | 0 | 0 | 0 | family is aligned at the current governance and evidence threshold |

## Recommended Next Governed Sweeps


## Retained Backlog

No retained governance backlog remains outside the shipped registry.

## Shipped Methods Without Governance Mapping

All shipped methods are mapped in the governance appendix.

## Shipped Registry Surface

| Tool | Shipped family | Governance family | Governance decision | Curated autodoc | Evidence level |
|---|---|---|---|---:|---|
| `aligncopy` | `alignment_tools` | Core Retain — Alignment read-write and post-processing | retain | yes | `compared_evidence` |
| `aligncopypair` | `alignment_tools` | Core Retain — Alignment read-write and post-processing | retain | yes | `compared_evidence` |
| `diffseq` | `alignment_tools` | Core Retain — Alignment read-write and post-processing | retain | yes | `compared_evidence` |
| `edialign` | `alignment_tools` | Core Retain — Alignment read-write and post-processing | retain | yes | `compared_evidence` |
| `infoalign` | `alignment_tools` | Core Retain — Alignment read-write and post-processing | retain | yes | `compared_evidence` |
| `extractalign` | `alignment_tools` | Core Retain — Alignment read-write and post-processing | retain | yes | `compared_evidence` |
| `nthseqset` | `alignment_tools` | Core Retain — Basic sequence IO and conversion | retain | yes | `compared_evidence` |
| `runinfo` | `archive_tools` | Modernize — Rework — Remote retrieval and archive acquisition | rework | yes | `compared_evidence` |
| `runget` | `archive_tools` | Modernize — Rework — Remote retrieval and archive acquisition | rework | yes | `compared_evidence` |
| `matcher` | `alignment_analysis` | Core Retain — Alignment read-write and post-processing | retain | yes | `compared_evidence` |
| `distmat` | `alignment_analysis` | Core Retain — Alignment read-write and post-processing | retain | yes | `compared_evidence` |
| `cons` | `alignment_analysis` | Core Retain — Alignment read-write and post-processing | retain | yes | `compared_evidence` |
| `consambig` | `alignment_analysis` | Core Retain — Alignment read-write and post-processing | retain | yes | `compared_evidence` |
| `needle` | `pairwise_alignment` | Core Retain — Alignment read-write and post-processing | retain | yes | `compared_evidence` |
| `needleall` | `pairwise_alignment` | Core Retain — Alignment read-write and post-processing | retain | yes | `compared_evidence` |
| `water` | `pairwise_alignment` | Core Retain — Alignment read-write and post-processing | retain | yes | `compared_evidence` |
| `seqret` | `retrieval_tools` | Modernize — Rework — Remote retrieval and archive acquisition | rework | yes | `compared_evidence` |
| `refseqget` | `retrieval_tools` | Modernize — Rework — Remote retrieval and archive acquisition | rework | yes | `compared_evidence` |
| `newseq` | `sequence_stream` | Core Retain — Basic sequence IO and conversion | retain | yes | `compared_evidence` |
| `makenucseq` | `sequence_stream` | Core Retain — Sequence editing and manipulation | retain | yes | `compared_evidence` |
| `makeprotseq` | `sequence_stream` | Core Retain — Sequence editing and manipulation | retain | yes | `compared_evidence` |
| `seqcount` | `sequence_stream` | Core Retain — Basic sequence IO and conversion | retain | yes | `compared_evidence` |
| `notseq` | `sequence_stream` | Core Retain — Basic sequence IO and conversion | retain | yes | `compared_evidence` |
| `nthseq` | `sequence_stream` | Core Retain — Basic sequence IO and conversion | retain | yes | `compared_evidence` |
| `skipseq` | `sequence_stream` | Core Retain — Basic sequence IO and conversion | retain | yes | `compared_evidence` |
| `listor` | `sequence_stream` | Core Retain — Sequence editing and manipulation | retain | yes | `compared_evidence` |
| `skipredundant` | `sequence_stream` | Core Retain — Sequence editing and manipulation | retain | yes | `compared_evidence` |
| `biosed` | `sequence_edit` | Core Retain — Sequence editing and manipulation | retain | yes | `compared_evidence` |
| `degapseq` | `sequence_edit` | Core Retain — Basic sequence IO and conversion | retain | yes | `compared_evidence` |
| `revseq` | `sequence_edit` | Core Retain — Sequence editing and manipulation | retain | yes | `compared_evidence` |
| `msbar` | `sequence_edit` | Core Retain — Sequence editing and manipulation | retain | yes | `compared_evidence` |
| `trimest` | `sequence_edit` | Core Retain — Sequence editing and manipulation | retain | yes | `compared_evidence` |
| `trimseq` | `sequence_edit` | Core Retain — Sequence editing and manipulation | retain | yes | `compared_evidence` |
| `descseq` | `sequence_edit` | Core Retain — Sequence editing and manipulation | retain | yes | `compared_evidence` |
| `vectorstrip` | `sequence_edit` | Core Retain — Sequence editing and manipulation | retain | yes | `compared_evidence` |
| `maskseq` | `feature_tools` | Core Retain — Sequence editing and manipulation | retain | yes | `compared_evidence` |
| `maskambignuc` | `feature_tools` | Core Retain — Sequence editing and manipulation | retain | yes | `compared_evidence` |
| `maskambigprot` | `feature_tools` | Core Retain — Sequence editing and manipulation | retain | yes | `compared_evidence` |
| `maskfeat` | `feature_tools` | Core Retain — Sequence editing and manipulation | retain | yes | `compared_evidence` |
| `extractfeat` | `feature_tools` | Core Retain — Basic sequence IO and conversion | retain | yes | `compared_evidence` |
| `featcopy` | `feature_tools` | Core Retain — Basic sequence IO and conversion | retain | yes | `compared_evidence` |
| `coderet` | `feature_tools` | Core Retain — Basic sequence IO and conversion | retain | yes | `compared_evidence` |
| `featmerge` | `feature_tools` | Core Retain — Basic sequence IO and conversion | retain | yes | `compared_evidence` |
| `featreport` | `feature_tools` | Core Retain — Basic sequence IO and conversion | retain | yes | `compared_evidence` |
| `feattext` | `feature_tools` | Core Retain — Basic sequence IO and conversion | retain | yes | `compared_evidence` |
| `splitsource` | `feature_tools` | Core Retain — Basic sequence IO and conversion | retain | yes | `compared_evidence` |
| `twofeat` | `feature_tools` | Core Retain — Sequence editing and manipulation | retain | yes | `compared_evidence` |
| `cai` | `codon_tools` | Core Retain — Core sequence statistics and composition | retain | yes | `compared_evidence` |
| `chips` | `codon_tools` | Core Retain — Core sequence statistics and composition | retain | yes | `compared_evidence` |
| `cusp` | `codon_tools` | Core Retain — Core sequence statistics and composition | retain | yes | `compared_evidence` |
| `codcmp` | `codon_tools` | Core Retain — Core sequence statistics and composition | retain | yes | `compared_evidence` |
| `codcopy` | `codon_tools` | Core Retain — Core sequence statistics and composition | retain | yes | `compared_evidence` |
| `dreg` | `pattern_tools` | Core Retain — Simple motif, pattern, and regular-expression search | retain | yes | `compared_evidence` |
| `einverted` | `pattern_tools` | Core Retain — Simple motif, pattern, and regular-expression search | retain | yes | `compared_evidence` |
| `fuzznuc` | `pattern_tools` | Core Retain — Simple motif, pattern, and regular-expression search | retain | yes | `compared_evidence` |
| `fuzzpro` | `pattern_tools` | Core Retain — Simple motif, pattern, and regular-expression search | retain | yes | `compared_evidence` |
| `fuzztran` | `pattern_tools` | Core Retain — Simple motif, pattern, and regular-expression search | retain | yes | `compared_evidence` |
| `palindrome` | `pattern_tools` | Core Retain — Simple motif, pattern, and regular-expression search | retain | yes | `compared_evidence` |
| `preg` | `pattern_tools` | Core Retain — Simple motif, pattern, and regular-expression search | retain | yes | `compared_evidence` |
| `patmatdb` | `pattern_tools` | Core Retain — Simple motif, pattern, and regular-expression search | retain | yes | `compared_evidence` |
| `seqmatchall` | `pattern_tools` | Core Retain — Simple motif, pattern, and regular-expression search | retain | yes | `compared_evidence` |
| `wordmatch` | `pattern_tools` | Core Retain — Simple motif, pattern, and regular-expression search | retain | yes | `compared_evidence` |
| `wordfinder` | `pattern_tools` | Core Retain — Simple motif, pattern, and regular-expression search | retain | yes | `compared_evidence` |
| `density` | `nucleotide_plots` | Modernize — Rework — Plotting and visualization tools | rework | yes | `compared_evidence` |
| `wobble` | `nucleotide_plots` | Modernize — Rework — Plotting and visualization tools | rework | yes | `executable_evidence` |
| `charge` | `protein_plots` | Modernize — Rework — Plotting and visualization tools | rework | yes | `compared_evidence` |
| `hmoment` | `protein_plots` | Modernize — Rework — Plotting and visualization tools | rework | yes | `compared_evidence` |
| `octanol` | `protein_plots` | Modernize — Rework — Plotting and visualization tools | rework | yes | `compared_evidence` |
| `pepinfo` | `protein_plots` | Modernize — Rework — Plotting and visualization tools | rework | yes | `compared_evidence` |
| `pepwindow` | `protein_plots` | Modernize — Rework — Plotting and visualization tools | rework | yes | `compared_evidence` |
| `recoder` | `restriction_tools` | Modernize — Rework — Restriction-enzyme design and analysis | retain | yes | `compared_evidence` |
| `silent` | `restriction_tools` | Modernize — Rework — Restriction-enzyme design and analysis | retain | yes | `compared_evidence` |
| `aaindexextract` | `sequence_stats` | Core Retain — Core sequence statistics and composition | retain | yes | `compared_evidence` |
| `complex` | `sequence_stats` | Core Retain — Simple motif, pattern, and regular-expression search | retain | yes | `compared_evidence` |
| `compseq` | `sequence_stats` | Core Retain — Core sequence statistics and composition | retain | yes | `compared_evidence` |
| `dan` | `sequence_stats` | Core Retain — Core sequence statistics and composition | retain | yes | `compared_evidence` |
| `geecee` | `sequence_stats` | Core Retain — Core sequence statistics and composition | retain | yes | `compared_evidence` |
| `infobase` | `sequence_stats` | Core Retain — Core sequence statistics and composition | retain | yes | `compared_evidence` |
| `infoseq` | `sequence_stats` | Core Retain — Core sequence statistics and composition | retain | yes | `compared_evidence` |
| `inforesidue` | `sequence_stats` | Core Retain — Core sequence statistics and composition | retain | yes | `compared_evidence` |
| `iep` | `sequence_stats` | Modernize — Rework — Protein property and structural-summary utilities | retain | yes | `compared_evidence` |
| `oddcomp` | `sequence_stats` | Core Retain — Core sequence statistics and composition | retain | yes | `compared_evidence` |
| `pepdigest` | `sequence_stats` | Modernize — Rework — Protein property and structural-summary utilities | retain | yes | `compared_evidence` |
| `pepstats` | `sequence_stats` | Core Retain — Core sequence statistics and composition | retain | yes | `compared_evidence` |
| `wordcount` | `sequence_stats` | Core Retain — Core sequence statistics and composition | retain | yes | `compared_evidence` |
| `backtranseq` | `translation_tools` | Core Retain — Basic sequence IO and conversion | retain | yes | `compared_evidence` |
| `backtranambig` | `translation_tools` | Core Retain — Basic sequence IO and conversion | retain | yes | `compared_evidence` |
| `checktrans` | `translation_tools` | Core Retain — Core sequence statistics and composition | retain | yes | `compared_evidence` |
| `transeq` | `translation_tools` | Core Retain — ORF and translation-adjacent utilities | retain | yes | `compared_evidence` |
| `getorf` | `translation_tools` | Core Retain — ORF and translation-adjacent utilities | retain | yes | `compared_evidence` |
| `prettyseq` | `translation_tools` | Core Retain — ORF and translation-adjacent utilities | retain | yes | `compared_evidence` |
| `tranalign` | `translation_tools` | Core Retain — ORF and translation-adjacent utilities | retain | yes | `compared_evidence` |
| `extractseq` | `sequence_transform` | Core Retain — Basic sequence IO and conversion | retain | yes | `compared_evidence` |
| `cutseq` | `sequence_transform` | Core Retain — Sequence editing and manipulation | retain | yes | `compared_evidence` |
| `union` | `sequence_transform` | Core Retain — Basic sequence IO and conversion | retain | yes | `compared_evidence` |
| `pasteseq` | `sequence_transform` | Core Retain — Sequence editing and manipulation | retain | yes | `compared_evidence` |
| `splitter` | `sequence_transform` | Core Retain — Sequence editing and manipulation | retain | yes | `compared_evidence` |
| `merger` | `sequence_transform` | Core Retain — Sequence editing and manipulation | retain | yes | `compared_evidence` |
| `megamerger` | `sequence_transform` | Core Retain — Sequence editing and manipulation | retain | yes | `compared_evidence` |
| `sizeseq` | `sequence_transform` | Core Retain — Sequence editing and manipulation | retain | yes | `compared_evidence` |
| `shuffleseq` | `sequence_transform` | Core Retain — Sequence editing and manipulation | retain | yes | `compared_evidence` |

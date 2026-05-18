# Governance and Registry Alignment Report

This page is generated from the maintained governance family-to-tool appendix, the shipped EMBOSS-RS Rust registry, and the cohort validation report. It exists to keep backlog truth, shipped scope, curated autodoc coverage, and evidence depth aligned.

## Summary

- Governance source: `docs/governance/appendices/family_to_tool_mapping_reference.md`
- Registry source: `emboss_tools::governed_tool_descriptors`
- Governed mapped tools: `265`
- Governed retained tools: `90`
- Shipped tools: `70`
- Shipped tools with governance mapping: `70`
- Retained backlog still unshipped: `26`
- Shipped tools with curated autodoc: `70`
- Shipped tools with executable or compared evidence: `70`
- Shipped tools with compared evidence: `21`
- Shipped tools still documented-only: `0`

## Shipped Decision Split

- Shipped retain methods: `64`
- Shipped rework methods: `6`
- Shipped omit methods: `0`
- Shipped add methods: `0`

## Family Reconciliation

| Governance family | Retained total | Retained shipped | Retained backlog | Shipped curated | Shipped executable+ | Shipped compared | Recommendation |
|---|---:|---:|---:|---:|---:|---:|---|
| Core Retain — Alignment read-write and post-processing | 13 | 13 | 0 | 13 | 13 | 2 | upgrade shipped retained methods to compared evidence |
| Core Retain — Basic sequence IO and conversion | 18 | 16 | 2 | 16 | 16 | 3 | prioritise retained backlog closure (2 remaining) |
| Core Retain — Core sequence statistics and composition | 16 | 12 | 4 | 12 | 12 | 6 | prioritise retained backlog closure (4 remaining) |
| Core Retain — ORF and translation-adjacent utilities | 4 | 4 | 0 | 4 | 4 | 4 | family is aligned at the current governance and evidence threshold |
| Core Retain — Sequence editing and manipulation | 23 | 15 | 8 | 15 | 15 | 2 | prioritise retained backlog closure (8 remaining) |
| Core Retain — Simple motif, pattern, and regular-expression search | 12 | 4 | 8 | 4 | 4 | 3 | prioritise retained backlog closure (8 remaining) |
| Defer — Ontology command group | 0 | 0 | 0 | 0 | 0 | 0 | family is aligned at the current governance and evidence threshold |
| Defer — Specialized metadata and semantic lookup utilities | 0 | 0 | 0 | 0 | 0 | 0 | family is aligned at the current governance and evidence threshold |
| Exclude Permanently — ACD developer tooling | 0 | 0 | 0 | 0 | 0 | 0 | family is aligned at the current governance and evidence threshold |
| Exclude Permanently — EMBOSS local database indexing administration | 0 | 0 | 0 | 0 | 0 | 0 | family is aligned at the current governance and evidence threshold |
| Exclude Permanently — EMBOSS-era server-cache-registry plumbing | 0 | 0 | 0 | 0 | 0 | 0 | family is aligned at the current governance and evidence threshold |
| Exclude Permanently — Wrapper-only compatibility commands | 0 | 0 | 0 | 0 | 0 | 0 | family is aligned at the current governance and evidence threshold |
| Modernize — Rework — Command discovery and help-navigation | 0 | 0 | 0 | 0 | 0 | 0 | family is aligned at the current governance and evidence threshold |
| Modernize — Rework — External database preparation helpers | 0 | 0 | 0 | 0 | 0 | 0 | family is aligned at the current governance and evidence threshold |
| Modernize — Rework — Legacy prediction methods with enduring scientific value | 0 | 0 | 0 | 0 | 0 | 0 | family is aligned at the current governance and evidence threshold |
| Modernize — Rework — Plotting and visualization tools | 0 | 0 | 0 | 2 | 2 | 0 | family is aligned at the current governance and evidence threshold |
| Modernize — Rework — Primer and assay-oriented search | 0 | 0 | 0 | 0 | 0 | 0 | family is aligned at the current governance and evidence threshold |
| Modernize — Rework — Protein property and structural-summary utilities | 2 | 0 | 2 | 0 | 0 | 0 | prioritise retained backlog closure (2 remaining) |
| Modernize — Rework — Remote retrieval and archive acquisition | 0 | 0 | 0 | 4 | 4 | 1 | family is aligned at the current governance and evidence threshold |
| Modernize — Rework — Restriction-enzyme design and analysis | 2 | 0 | 2 | 0 | 0 | 0 | prioritise retained backlog closure (2 remaining) |
| Strategic Add — HMM and probabilistic homology workflows | 0 | 0 | 0 | 0 | 0 | 0 | family is aligned at the current governance and evidence threshold |
| Strategic Add — Modern archive-scale raw data ingestion | 0 | 0 | 0 | 0 | 0 | 0 | family is aligned at the current governance and evidence threshold |

## Recommended Next Governed Sweeps

- **Core Retain — Sequence editing and manipulation**: prioritise retained backlog closure (8 remaining). Retained backlog: `biosed`, `listor`, `makenucseq`, `makeprotseq`, `msbar`, `skipredundant`, `trimest`, `vectorstrip`
- **Core Retain — Simple motif, pattern, and regular-expression search**: prioritise retained backlog closure (8 remaining). Retained backlog: `dreg`, `einverted`, `palindrome`, `patmatdb`, `preg`, `seqmatchall`, `wordfinder`, `wordmatch`
- **Core Retain — Core sequence statistics and composition**: prioritise retained backlog closure (4 remaining). Retained backlog: `aaindexextract`, `infobase`, `inforesidue`, `oddcomp`
- **Core Retain — Basic sequence IO and conversion**: prioritise retained backlog closure (2 remaining). Retained backlog: `nthseqset`, `splitsource`
- **Modernize — Rework — Protein property and structural-summary utilities**: prioritise retained backlog closure (2 remaining). Retained backlog: `iep`, `pepdigest`

## Retained Backlog

- `nthseqset` — retain — Core Retain — Basic sequence IO and conversion
- `splitsource` — retain — Core Retain — Basic sequence IO and conversion
- `biosed` — retain — Core Retain — Sequence editing and manipulation
- `listor` — retain — Core Retain — Sequence editing and manipulation
- `makenucseq` — retain — Core Retain — Sequence editing and manipulation
- `makeprotseq` — retain — Core Retain — Sequence editing and manipulation
- `msbar` — retain — Core Retain — Sequence editing and manipulation
- `skipredundant` — retain — Core Retain — Sequence editing and manipulation
- `trimest` — retain — Core Retain — Sequence editing and manipulation
- `vectorstrip` — retain — Core Retain — Sequence editing and manipulation
- `aaindexextract` — retain — Core Retain — Core sequence statistics and composition
- `infobase` — retain — Core Retain — Core sequence statistics and composition
- `inforesidue` — retain — Core Retain — Core sequence statistics and composition
- `oddcomp` — retain — Core Retain — Core sequence statistics and composition
- `dreg` — retain — Core Retain — Simple motif, pattern, and regular-expression search
- `einverted` — retain — Core Retain — Simple motif, pattern, and regular-expression search
- `palindrome` — retain — Core Retain — Simple motif, pattern, and regular-expression search
- `patmatdb` — retain — Core Retain — Simple motif, pattern, and regular-expression search
- `preg` — retain — Core Retain — Simple motif, pattern, and regular-expression search
- `seqmatchall` — retain — Core Retain — Simple motif, pattern, and regular-expression search
- `wordfinder` — retain — Core Retain — Simple motif, pattern, and regular-expression search
- `wordmatch` — retain — Core Retain — Simple motif, pattern, and regular-expression search
- `recoder` — retain — Modernize — Rework — Restriction-enzyme design and analysis
- `silent` — retain — Modernize — Rework — Restriction-enzyme design and analysis
- `iep` — retain — Modernize — Rework — Protein property and structural-summary utilities
- `pepdigest` — retain — Modernize — Rework — Protein property and structural-summary utilities

## Shipped Methods Without Governance Mapping

All shipped methods are mapped in the governance appendix.

## Shipped Registry Surface

| Tool | Shipped family | Governance family | Governance decision | Curated autodoc | Evidence level |
|---|---|---|---|---:|---|
| `aligncopy` | `alignment_tools` | Core Retain — Alignment read-write and post-processing | retain | yes | `executable_evidence` |
| `aligncopypair` | `alignment_tools` | Core Retain — Alignment read-write and post-processing | retain | yes | `executable_evidence` |
| `diffseq` | `alignment_tools` | Core Retain — Alignment read-write and post-processing | retain | yes | `executable_evidence` |
| `edialign` | `alignment_tools` | Core Retain — Alignment read-write and post-processing | retain | yes | `executable_evidence` |
| `infoalign` | `alignment_tools` | Core Retain — Alignment read-write and post-processing | retain | yes | `executable_evidence` |
| `extractalign` | `alignment_tools` | Core Retain — Alignment read-write and post-processing | retain | yes | `executable_evidence` |
| `runinfo` | `archive_tools` | Modernize — Rework — Remote retrieval and archive acquisition | rework | yes | `executable_evidence` |
| `runget` | `archive_tools` | Modernize — Rework — Remote retrieval and archive acquisition | rework | yes | `executable_evidence` |
| `matcher` | `alignment_analysis` | Core Retain — Alignment read-write and post-processing | retain | yes | `executable_evidence` |
| `distmat` | `alignment_analysis` | Core Retain — Alignment read-write and post-processing | retain | yes | `executable_evidence` |
| `cons` | `alignment_analysis` | Core Retain — Alignment read-write and post-processing | retain | yes | `executable_evidence` |
| `consambig` | `alignment_analysis` | Core Retain — Alignment read-write and post-processing | retain | yes | `executable_evidence` |
| `needle` | `pairwise_alignment` | Core Retain — Alignment read-write and post-processing | retain | yes | `compared_evidence` |
| `needleall` | `pairwise_alignment` | Core Retain — Alignment read-write and post-processing | retain | yes | `executable_evidence` |
| `water` | `pairwise_alignment` | Core Retain — Alignment read-write and post-processing | retain | yes | `compared_evidence` |
| `seqret` | `retrieval_tools` | Modernize — Rework — Remote retrieval and archive acquisition | rework | yes | `compared_evidence` |
| `refseqget` | `retrieval_tools` | Modernize — Rework — Remote retrieval and archive acquisition | rework | yes | `executable_evidence` |
| `newseq` | `sequence_stream` | Core Retain — Basic sequence IO and conversion | retain | yes | `executable_evidence` |
| `seqcount` | `sequence_stream` | Core Retain — Basic sequence IO and conversion | retain | yes | `executable_evidence` |
| `notseq` | `sequence_stream` | Core Retain — Basic sequence IO and conversion | retain | yes | `executable_evidence` |
| `nthseq` | `sequence_stream` | Core Retain — Basic sequence IO and conversion | retain | yes | `executable_evidence` |
| `skipseq` | `sequence_stream` | Core Retain — Basic sequence IO and conversion | retain | yes | `executable_evidence` |
| `degapseq` | `sequence_edit` | Core Retain — Basic sequence IO and conversion | retain | yes | `executable_evidence` |
| `revseq` | `sequence_edit` | Core Retain — Sequence editing and manipulation | retain | yes | `executable_evidence` |
| `trimseq` | `sequence_edit` | Core Retain — Sequence editing and manipulation | retain | yes | `executable_evidence` |
| `descseq` | `sequence_edit` | Core Retain — Sequence editing and manipulation | retain | yes | `compared_evidence` |
| `maskseq` | `feature_tools` | Core Retain — Sequence editing and manipulation | retain | yes | `compared_evidence` |
| `maskambignuc` | `feature_tools` | Core Retain — Sequence editing and manipulation | retain | yes | `executable_evidence` |
| `maskambigprot` | `feature_tools` | Core Retain — Sequence editing and manipulation | retain | yes | `executable_evidence` |
| `maskfeat` | `feature_tools` | Core Retain — Sequence editing and manipulation | retain | yes | `executable_evidence` |
| `extractfeat` | `feature_tools` | Core Retain — Basic sequence IO and conversion | retain | yes | `compared_evidence` |
| `featcopy` | `feature_tools` | Core Retain — Basic sequence IO and conversion | retain | yes | `executable_evidence` |
| `coderet` | `feature_tools` | Core Retain — Basic sequence IO and conversion | retain | yes | `executable_evidence` |
| `featmerge` | `feature_tools` | Core Retain — Basic sequence IO and conversion | retain | yes | `executable_evidence` |
| `featreport` | `feature_tools` | Core Retain — Basic sequence IO and conversion | retain | yes | `executable_evidence` |
| `feattext` | `feature_tools` | Core Retain — Basic sequence IO and conversion | retain | yes | `executable_evidence` |
| `twofeat` | `feature_tools` | Core Retain — Sequence editing and manipulation | retain | yes | `executable_evidence` |
| `cai` | `codon_tools` | Core Retain — Core sequence statistics and composition | retain | yes | `executable_evidence` |
| `chips` | `codon_tools` | Core Retain — Core sequence statistics and composition | retain | yes | `executable_evidence` |
| `cusp` | `codon_tools` | Core Retain — Core sequence statistics and composition | retain | yes | `compared_evidence` |
| `codcmp` | `codon_tools` | Core Retain — Core sequence statistics and composition | retain | yes | `executable_evidence` |
| `codcopy` | `codon_tools` | Core Retain — Core sequence statistics and composition | retain | yes | `executable_evidence` |
| `fuzznuc` | `pattern_tools` | Core Retain — Simple motif, pattern, and regular-expression search | retain | yes | `compared_evidence` |
| `fuzzpro` | `pattern_tools` | Core Retain — Simple motif, pattern, and regular-expression search | retain | yes | `compared_evidence` |
| `fuzztran` | `pattern_tools` | Core Retain — Simple motif, pattern, and regular-expression search | retain | yes | `compared_evidence` |
| `charge` | `protein_plots` | Modernize — Rework — Plotting and visualization tools | rework | yes | `executable_evidence` |
| `pepwindow` | `protein_plots` | Modernize — Rework — Plotting and visualization tools | rework | yes | `executable_evidence` |
| `complex` | `sequence_stats` | Core Retain — Simple motif, pattern, and regular-expression search | retain | yes | `executable_evidence` |
| `compseq` | `sequence_stats` | Core Retain — Core sequence statistics and composition | retain | yes | `compared_evidence` |
| `dan` | `sequence_stats` | Core Retain — Core sequence statistics and composition | retain | yes | `executable_evidence` |
| `geecee` | `sequence_stats` | Core Retain — Core sequence statistics and composition | retain | yes | `compared_evidence` |
| `infoseq` | `sequence_stats` | Core Retain — Core sequence statistics and composition | retain | yes | `compared_evidence` |
| `pepstats` | `sequence_stats` | Core Retain — Core sequence statistics and composition | retain | yes | `compared_evidence` |
| `wordcount` | `sequence_stats` | Core Retain — Core sequence statistics and composition | retain | yes | `executable_evidence` |
| `backtranseq` | `translation_tools` | Core Retain — Basic sequence IO and conversion | retain | yes | `compared_evidence` |
| `backtranambig` | `translation_tools` | Core Retain — Basic sequence IO and conversion | retain | yes | `compared_evidence` |
| `checktrans` | `translation_tools` | Core Retain — Core sequence statistics and composition | retain | yes | `compared_evidence` |
| `transeq` | `translation_tools` | Core Retain — ORF and translation-adjacent utilities | retain | yes | `compared_evidence` |
| `getorf` | `translation_tools` | Core Retain — ORF and translation-adjacent utilities | retain | yes | `compared_evidence` |
| `prettyseq` | `translation_tools` | Core Retain — ORF and translation-adjacent utilities | retain | yes | `compared_evidence` |
| `tranalign` | `translation_tools` | Core Retain — ORF and translation-adjacent utilities | retain | yes | `compared_evidence` |
| `extractseq` | `sequence_transform` | Core Retain — Basic sequence IO and conversion | retain | yes | `executable_evidence` |
| `cutseq` | `sequence_transform` | Core Retain — Sequence editing and manipulation | retain | yes | `executable_evidence` |
| `union` | `sequence_transform` | Core Retain — Basic sequence IO and conversion | retain | yes | `executable_evidence` |
| `pasteseq` | `sequence_transform` | Core Retain — Sequence editing and manipulation | retain | yes | `executable_evidence` |
| `splitter` | `sequence_transform` | Core Retain — Sequence editing and manipulation | retain | yes | `executable_evidence` |
| `merger` | `sequence_transform` | Core Retain — Sequence editing and manipulation | retain | yes | `executable_evidence` |
| `megamerger` | `sequence_transform` | Core Retain — Sequence editing and manipulation | retain | yes | `executable_evidence` |
| `sizeseq` | `sequence_transform` | Core Retain — Sequence editing and manipulation | retain | yes | `executable_evidence` |
| `shuffleseq` | `sequence_transform` | Core Retain — Sequence editing and manipulation | retain | yes | `executable_evidence` |

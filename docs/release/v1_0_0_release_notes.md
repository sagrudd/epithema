# Draft Release Notes: EMBOSS-RS 1.0.0

## Overview

`emboss-rs` `1.0.0` is the first stable coordinated release of the EMBOSS
reboot in Rust, paired with `emboss-r` `1.0.0`.

This release establishes a governed Linux-first CLI and platform, typed shared
core and IO layers, provider-backed retrieval seams, a formal plot contract,
and a first-class sister R surface rather than a Rust-only proof of concept.

These notes remain a draft release document, not a claim that every shipped
method has full historical-example or expected-output acceptance coverage.
For the current evidence posture, see:

- [Cohort Validation Report](../generated/cohort_validation.md)
- [Governance Alignment Report](../generated/governance_alignment.md)
- [Cohort Health Gate](../generated/cohort_health.md)

## Highlights

### Rust-first CLI reboot

The shipped `emboss-rs <tool>` surface now covers a practical governed cohort
of `90` methods, including:

- sequence construction, counting, selection, extraction, partitioning,
  cleanup, and description editing
- feature-aware masking, extraction, and feature-copy operations for annotated
  EMBL and GenBank inputs
- deterministic translation, ORF, translation-alignment, and pattern-search
  tools
- composition, GC, protein statistics, codon-usage, and complexity tools
- alignment utility, global alignment, local alignment, similarity,
  distance-matrix, and consensus tools
- the retained exception tool `complex`
- the first governed plot-producing analytical tools `charge` and `pepwindow`

### Governed retrieval

The release includes the first real governed retrieval layers and user-facing
tools for:

- single-sequence retrieval via ENA and NCBI-backed acquisition seams
- modernized `seqret` and `refseqget`
- archive metadata and manifest-oriented `runinfo` and `runget`

### Documentation and validation

`1.0.0` includes:

- a Sphinx documentation site published through GitHub Pages
- typed autodoc and generated-validation artefacts
- a cohort-level validation report and governance-alignment report
- validation stubs, compared-evidence anchors, and contributor-facing
  release/check guidance
- cross-surface validation for a curated subset of the public R interface

Current evidence posture at the time of this draft:

- `37` shipped methods carry compared evidence
- `59` shipped methods carry executable evidence
- `63` shipped methods record harvested legacy provenance
- `0` shipped methods remain at declared or documented-only evidence

### First-class R story

Plot rendering remains R-owned through the sister `emboss-r` package. The Rust
release includes:

- the typed `emboss-plot-contract` crate
- the Rust-to-R bridge
- the `charge` and `pepwindow` analytical paths emitting real plot contracts
  for R rendering

## Important Deferred Areas

This release does not claim:

- full historical EMBOSS catalog parity
- full public R exposure of every shipped CLI method
- full harvested legacy-example coverage across the shipped cohort
- full expected-output comparison coverage across the shipped cohort
- protected-data retrieval or broad raw-read orchestration
- broad multi-platform native packaging beyond the Linux-first release path
- CRAN or Bioconductor publication for the R package

## Compatibility Statement

`emboss-rs` `1.0.0` is the intended coordinated stable partner for
`emboss-r` `1.0.0`.

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
- [Comparison Coverage Report](../generated/comparison_coverage.md)
- [Full Compared Cohort Gate](../generated/full_compared_cohort.md)
- [Harvest Coverage Exceptions](../generated/harvest_coverage.md)
- [Retained Backlog Closure Report](../generated/retained_backlog_closure.md)

## Highlights

### Rust-first CLI reboot

The shipped `emboss-rs <tool>` surface now covers a practical governed cohort
of `109` methods, including:

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
- the governed plot-producing analytical tools `charge`, `pepwindow`,
  `hmoment`, `octanol`, `pepinfo`, `density`, `wobble`, `isochore`, and
  `banana`
- the bounded protein-coordinate torsion-reporting method `psiphi`
- the bounded primer-pair search method `primersearch`

### Governed retrieval

The release includes the first real governed retrieval layers and user-facing
tools for:

- single-sequence retrieval via ENA and NCBI-backed acquisition seams
- modernized `seqret`, `seqretsetall`, `seqretsplit`, and `refseqget`
- archive metadata and manifest-oriented `runinfo`, `runget`, and
  `infoassembly`

### Documentation and validation

`1.0.0` includes:

- a Sphinx documentation site published through GitHub Pages
- typed autodoc and generated-validation artefacts
- a cohort-level validation report and governance-alignment report
- validation stubs, compared-evidence anchors, and contributor-facing
  release/check guidance
- cross-surface validation for a curated subset of the public R interface

Current evidence posture at the time of this draft:

- `108` shipped methods carry compared evidence
- `1` shipped methods carry executable evidence
- `109` shipped methods record harvested legacy provenance
- full compared cohort: `no`
- non-blocking plotting legacy-reference notes remain visible: `yes`
- blocking cohort gaps: `1`
- weakest evidence family: `Modernize — Rework — Primer and assay-oriented search`

The remaining visible plotting notes for `charge` and `pepwindow` are
non-blocking provenance/documentation nuances about missing explicit
legacy-reference artefacts. They do not lower evidence maturity and do not
change the release gate state above.
- `hmoment` now ships through the governed surface with canonical checked-in
  analytical and plot-contract fixtures plus compared acceptance evidence for
  both surfaces.
- `octanol` now ships through the governed surface with canonical checked-in
  analytical and plot-contract fixtures plus compared acceptance evidence for
  both surfaces.
- `pepinfo` now ships through the governed surface with canonical checked-in
  analytical and multi-series plot-contract fixtures plus compared acceptance
  evidence for both surfaces.
- `density` now ships through the governed surface with canonical checked-in
  analytical and plot-contract fixtures plus compared acceptance evidence for
  both surfaces.
- `syco` now ships through the governed surface with canonical checked-in
  analytical and plot-contract fixtures plus compared acceptance evidence for
  both surfaces.
- `primersearch` now ships through the governed surface with a checked-in
  executable validation stub and curated legacy provenance, while the
  canonical compared primer-hit fixture remains the next evidence-closing
  step.
- `wobble` now ships through the governed surface with canonical checked-in
  analytical and plot-contract fixtures plus compared acceptance evidence for
  both surfaces.
- `isochore` now ships through the governed surface with canonical checked-in
  analytical and plot-contract fixtures plus compared acceptance evidence for
  both surfaces.
- `0` retained governance methods remain unshipped
- `1` shipped methods remain below compared evidence

This means the shipped retained cohort remains fully closed, but the shipped
governed cohort is at an honest interim executable-only checkpoint rather than
the fully green release gate. The bounded `primersearch` slice is now shipped
with curated provenance and a runnable governed validation seam, and it returns
to the fully green gate only once its canonical compared primer-hit fixture
lands.

### First-class R story

Plot rendering remains R-owned through the sister `emboss-r` package. The Rust
release includes:

- the typed `emboss-plot-contract` crate
- the Rust-to-R bridge
- the `charge`, `pepwindow`, and governed `wordcount` analytical paths
  emitting real plot contracts for R rendering

## Important Deferred Areas

This release does not claim:

- full historical EMBOSS catalog parity
- full public R exposure of every shipped CLI method
- protected-data retrieval or broad raw-read orchestration
- broad multi-platform native packaging beyond the Linux-first release path
- CRAN or Bioconductor publication for the R package
- broad plotting-family rollout beyond the governed `charge`, `pepwindow`, and
  `wordcount` analytical producers
- broad provider parity beyond the governed shipped retrieval/archive slice

## Compatibility Statement

`emboss-rs` `1.0.0` is the intended coordinated stable partner for
`emboss-r` `1.0.0`.

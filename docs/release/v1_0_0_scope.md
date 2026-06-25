# v1.0.0 Scope

This page defines the intended coordinated `1.0.0` release scope for
`epithema`.

## In Scope

The `1.0.0` release includes:

- the governed `epithema <tool>` single-binary command surface
- shared biological core, IO, diagnostics, service, validation, and autodoc
  infrastructure
- the practical shipped CLI cohort documented in the repository root README:
  - sequence construction and selection
  - extraction, partitioning, cleanup, and editing
  - feature-aware masking, extraction, and copy operations
  - translation-adjacent and deterministic pattern-search operations
  - composition, GC, protein statistics, codon-usage, and linguistic
    complexity summaries
  - alignment utility, global pairwise alignment, similarity, distance-matrix,
    and consensus tools
  - governed provider-backed retrieval plus archive metadata/manifest tools
  - the retained exception tool `complex`
  - the first plot-producing tool `charge`
- the Rust plot-contract layer and the sister `epithemaR` rendering path
- the Rust-to-R bridge and the first-class R analytical package surface
- curated cross-surface validation between Rust-native and R-facing semantics
- release automation, Linux binary packaging, docs publication, and GHCR
  container automation

## Explicit Deferred Scope

The coordinated `1.0.0` release intentionally does not include:

- full parity with the historical EMBOSS catalog
- broad provider or protected-access retrieval workflows
- protected-access, dbGaP-controlled, credentialed, requester-pays, or
  object-store publication workflows for the NGS acquisition milestone
- full archive replication/orchestration and raw-read toolkit integration
- a full runtime Rust-to-R embedding model beyond the current governed bridge
- exposure of every shipped CLI method as a public R wrapper
- pixel-level visual regression testing for plots
- multi-platform native release artefacts beyond the Linux-first release path
- CRAN or Bioconductor publication for `epithemaR`

## Release Standard

The question for `1.0.0` is not “is every eventual EMBOSS reboot feature
implemented?” The release standard is:

- the governed v1 cohort is real and documented
- the shipped tools and R surface are coherent and credible
- release automation, docs, and validation are in place
- major deferred areas are stated honestly rather than implied silently

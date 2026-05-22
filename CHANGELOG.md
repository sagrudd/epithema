# Changelog

All notable changes to `emboss-rs` will be documented in this file.

The project uses a human-maintained changelog with coordinated release notes.
`v1.0.0` will be the first coordinated release with the sister `emboss-r`
package. Until that cutover, changes accumulate under `Unreleased`.

## [Unreleased]

Changes recorded here after the coordinated `1.0.0` stable cut must preserve
the same release-truth model used during the RC phase.

New shipped tools after `1.0.0` must not bypass:

- governance mapping
- autodoc presence
- validation-stub generation
- cohort-report inclusion
- at least one compared anchor per shipped family
- comparison-coverage reporting
- harvest-coverage reporting
- full-compared-cohort reporting
- retained-backlog-closure reporting
- drift-free release-facing counts and report links
- honest release-note wording

Release-facing documentation under `Unreleased` should continue to defer to the
generated cohort, governance, cohort-health, comparison-coverage,
harvest-coverage, full-compared-cohort, and retained-backlog reports rather
than implying biological acceptance from shipped-method count alone.

## [1.0.0] - Planned

### Added
- Governed single-binary `emboss-rs` CLI with the practical shipped EMBOSS-RS
  v1 tool cohort.
- First-class documentation, autodoc generation, validation stubs, provider
  seams, plot contract, R bridge support, and cross-surface validation.
- Sequence-stream, sequence-editing, feature-aware, translation, pattern,
  composition/statistics, codon-usage, alignment-utility, alignment-summary,
  retrieval, archive-metadata, complexity, and charge-profile tool families.
- Coordinated `1.0.0` release automation with Linux binary packaging, docs
  gating, and GHCR container publication.

### Changed
- Workspace and release metadata now target the coordinated stable `1.0.0`
  release rather than pre-release development versions.

### Documentation
- Added formal v1 scope, RC readiness, and draft release-note material under
  `docs/release/`.

### Infrastructure
- Added tag-driven GitHub release automation, release-note scaffolding,
  release-bundle assembly, release metadata checks, and Linux container smoke
  validation.

### Notes
- `1.0.0` is reserved for the coordinated release with `emboss-r` `1.0.0`.
- The release remains governed by the RC readiness and evidence reports under
  `docs/release/` and `docs/generated/`; shipped-method count alone is not used
  as a proxy for biological acceptance completeness.

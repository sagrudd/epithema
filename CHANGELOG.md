# Changelog

All notable changes to `emboss-rs` will be documented in this file.

The project uses a human-maintained changelog with coordinated release notes.
`v1.0.0` will be the first coordinated release with the sister `emboss-r`
package. Until that cutover, changes accumulate under `Unreleased`.

## [Unreleased]

- Post-`1.0.0` changes will be recorded here after the coordinated stable cut.

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
- Added tag-driven GitHub release automation, release-note scaffolding, and
  local release-check targets.

### Notes
- `1.0.0` is reserved for the coordinated release with `emboss-r` `1.0.0`.

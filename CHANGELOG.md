# Changelog

All notable changes to `emboss-rs` will be documented in this file.

The project uses a human-maintained changelog with coordinated release notes.
`v1.0.0` will be the first coordinated release with the sister `emboss-r`
package. Until that cutover, changes accumulate under `Unreleased`.

## [Unreleased]

### Added
- Release-process documentation, release checklist, and coordinated release
  policy for the `emboss-rs` / `emboss-r` `v1.0.0` cutover.
- Tag-driven release automation, Linux release artefact packaging, and GHCR
  container build/publish automation scaffolding.

### Changed
- Release validation is now explicitly gated on Rust tests, Sphinx docs, and
  release-oriented Make targets.

### Documentation
- Added formal cross-repo release guidance and container-release usage notes.

### Infrastructure
- Added GitHub release-note scaffolding and local release-check Make targets.

## [1.0.0] - Planned

### Added
- Governed single-binary `emboss-rs` CLI with the practical shipped EMBOSS-RS
  v1 tool cohort.
- First-class documentation, autodoc generation, validation stubs, provider
  seams, plot contract, R bridge support, and cross-surface validation.

### Notes
- `1.0.0` is reserved for the coordinated release with `emboss-r` `1.0.0`.

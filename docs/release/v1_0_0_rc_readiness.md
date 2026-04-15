# v1.0.0 Release-Candidate Readiness

Status date: 2026-04-16

## Recommendation

Ready for a coordinated `1.0.0` release decision, with explicit manual final
steps still required.

This stabilization pass found no product-scope blocker that warrants deferring
the coordinated `1.0.0` release. The remaining work is cutover-oriented:
human approval, tag creation, GitHub release publication, and environment-level
confirmation of the R package and container paths.

## Shipped Scope Summary

- `emboss-rs` ships the governed v1 CLI cohort described in the root README,
  including sequence operations, feature-aware tools, translation/pattern
  tools, composition/statistics, codon-usage tools, alignment utilities,
  global alignment, similarity/consensus tools, retrieval tools, archive
  metadata/manifests, `complex`, and `charge`.
- `emboss-r` ships the first-class R analytical surface for the practical
  in-memory cohort plus the R-owned plotting/rendering layer.
- Cross-surface validation exists for a curated subset spanning sequences,
  tables/reports, alignment summaries/consensus, and charge-profile analytics.

## Validation Summary

Local checks completed in this stabilization pass:

- `cargo build`
- `cargo test`
- `cargo fmt --check`
- docs build via `PYTHON=.venv-docs/bin/python make docs`
- local Rust release gate via `PYTHON=.venv-docs/bin/python make release-check`
- workflow YAML parse validation for both repositories

Environment-limited checks not runnable on this machine:

- `emboss-r` package tests and release checks, because `Rscript` is not
  installed here
- local container smoke build, because Docker is not installed here

These are release-process verification items, not audited product blockers, but
they must still be confirmed in CI or on a maintainer workstation before the
final tags are cut.

## Documentation Status

- release process, checklist, scope, and draft release notes are now present
  under `docs/release/`
- root docs navigation links to the release section
- the repository README states the shipped v1 scope and the role of `emboss-r`
- the package README and `RELEASE.md` in `emboss-r` state the shipped and
  deferred R scope

## Coordinated Version Status

- `emboss-rs` version metadata now targets `1.0.0`
- `emboss-r` version metadata now targets `1.0.0`
- prerelease policy was corrected so that the process no longer assumes the R
  package and Cargo can share identical prerelease version syntax

## Remaining Manual Release Steps

- run the `emboss-r` release checks in an environment with R installed
- run or observe a successful Linux container smoke build in an environment
  with Docker available
- perform final human review of changelogs and draft release notes
- create coordinated `v1.0.0` tags in the documented order
- publish the GitHub releases after the tag-triggered workflows complete

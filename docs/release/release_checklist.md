# Release Checklist

Use this checklist before cutting a coordinated stable tag.

## Coordinated gates

- [ ] `emboss-rs` and `emboss-r` versions are aligned exactly for the target
      release.
- [ ] Stable compatibility statement for the paired versions is prepared.
- [ ] Changelogs in both repos have been reviewed and updated.
- [ ] Release notes in both repos are drafted and reviewed.

## `emboss-rs`

- [ ] `cargo build` succeeds.
- [ ] `cargo test` succeeds.
- [ ] `make release-check` succeeds.
- [ ] autodoc-generated content and validation artefacts are current.
- [ ] documentation builds cleanly.
- [ ] release artefact packaging has been smoke-tested.
- [ ] Linux container build has been smoke-tested.
- [ ] GHCR publication settings remain valid.

## `emboss-r`

- [ ] package tests succeed.
- [ ] release-oriented package build/check workflow is green.
- [ ] analytical wrappers remain green.
- [ ] plotting contract tests remain green.
- [ ] cross-surface validation subset remains green.
- [ ] source package artefact builds successfully.

## Cross-surface and documentation

- [ ] cross-surface validation fixtures are current.
- [ ] charge-profile analytical path remains aligned between Rust and R.
- [ ] docs published by GitHub Pages are current for `emboss-rs`.
- [ ] release-process docs still match repository automation.

## Cutover

- [ ] final version bump commit is prepared in both repos.
- [ ] release tags are prepared in the documented order.
- [ ] GitHub Releases are ready to publish with attached artefacts.

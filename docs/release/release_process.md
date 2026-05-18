# Release Process

This page defines the coordinated release process for `emboss-rs` and the
sister `emboss-r` package.

## Versioning policy

- `emboss-rs` and `emboss-r` use coordinated semantic versions for stable
  paired releases.
- The intended first coordinated public release is:
  - `emboss-rs` `1.0.0`
  - `emboss-r` `1.0.0`
- Development versions may differ before cutover, but the coordinated stable
  release requires exact version alignment at tag time.
- For the final stabilization pass, both repositories now carry `1.0.0` as the
  intended release version. The remaining cutover step is tag-and-release, not
  another version bump.
- Manual release-candidate verification should use branch heads or
  `workflow_dispatch` against the checked-in target version. The process does
  not require prerelease tags with identical version syntax across both repos,
  because the R package and Cargo ecosystems do not express prerelease versions
  in the same way.

## Compatibility rule

For `v1.0.0`, “release compatible” means:

- the Rust CLI and service surface are built from the `emboss-rs` `1.0.0` tag
- the public R package surface is built from the `emboss-r` `1.0.0` tag
- cross-surface validation passes for the curated shared method subset
- plot-contract rendering and charge-profile flow remain aligned

## Preferred release order

1. Finalize and validate `emboss-rs`.
2. Finalize and validate `emboss-r`.
3. Tag and publish the two releases in close succession.
4. State explicitly in both release notes that `emboss-rs` `1.0.0` and
   `emboss-r` `1.0.0` are the supported coordinated pair.

## Automated paths

### `emboss-rs`

- `make release-version-check`
- `make release-generated-check`
- `make release-build`
- `make release-test`
- `make release-docs`
- `make release-artifacts`
- `make release-container`
- `make release-check`
- GitHub Actions release workflow:
  - runs on `v*` tags
  - supports manual dispatch for release-candidate verification
  - verifies checked-in release metadata before packaging
  - runs the local release gate
  - assembles the Linux/docs/validation artefact bundle through `make release-artifacts`
  - builds the container image and publishes to GHCR when configured
  - attaches artefacts to a GitHub release on tag builds

### `emboss-r`

- `make test`
- `make build-package`
- `make check`
- `make release-check`
- GitHub Actions package workflows:
  - CI workflow for package checks
  - release workflow on `v*` tags and manual dispatch
  - source package artefact upload
  - GitHub release attachment on tag builds

## Release artefacts

### `emboss-rs`

- Linux `emboss-rs` binary tarball
- SHA256 checksum for the Linux tarball
- built documentation archive
- validation-report archive containing the cohort-level evidence outputs
- release manifest JSON describing the checked-in version and artefact names
- source archive from GitHub
- GHCR container image

### `emboss-r`

- source package tarball produced by `R CMD build`
- package check outputs in CI/release logs
- source archive from GitHub

## Manual settings still required

- GitHub Pages publication is provisioned automatically by the docs workflow
  when repository policy allows it. If repository or organization policy blocks
  automatic enablement, Pages must be set manually to deploy from GitHub
  Actions for `emboss-rs`.
- GHCR publication requires GitHub Actions package write permission on the
  `emboss-rs` repository. No extra secret is required when publishing with the
  repository `GITHUB_TOKEN`.
- GitHub Releases must remain enabled in both repositories.

## Manual release steps

The workflows automate build, packaging, and release staging. Humans still own:

- final changelog editing
- final release-note review
- final tag creation
- final go/no-go decision after stabilization audit

## Post-1.0 truth model

After the stable `1.0.0` cut, the repository should carry the same release
truth rules forward into `Unreleased`.

New shipped tools must not bypass:

- governance mapping
- autodoc presence
- validation-stub generation
- cohort-report inclusion
- honest release-note wording

`Unreleased` changelog and release-facing wording should continue to defer to:

- `docs/generated/cohort_validation.md`
- `docs/generated/governance_alignment.md`
- `docs/generated/cohort_health.md`

This project treats those generated reports as the current truth surface for
scope, evidence, and roadmap pressure. Shipped-method count alone is not used
as a proxy for biological acceptance completeness.

## Local release-candidate flow

For a conservative local candidate build, run:

1. `make release-version-check`
2. `make release-truth-check`
3. `make release-generated-check`
4. `make release-check`
5. `make release-artifacts`
6. `make release-container` on a machine with Docker available

The release bundle is written under `dist/release/<version>/`. It is intended
for inspection and local smoke verification before a `v*` tag is created.

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

- `make release-build`
- `make release-test`
- `make release-docs`
- `make release-container`
- `make release-check`
- GitHub Actions release workflow:
  - runs on `v*` tags
  - supports manual dispatch for release-candidate verification
  - builds Linux release artefacts
  - builds docs
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

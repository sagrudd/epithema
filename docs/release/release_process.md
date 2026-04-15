# Release Process

This page defines the coordinated release process for `emboss-rs` and the
sister `emboss-r` package.

## Versioning policy

- `emboss-rs` and `emboss-r` use coordinated semantic versions for stable
  paired releases.
- The intended first coordinated public release is:
  - `emboss-rs` `1.0.0`
  - `emboss-r` `1.0.0`
- Development versions may differ before cutover, but a stable coordinated
  release requires exact version alignment at tag time.
- Release-candidate tags may use `v1.0.0-rc.N` while versions inside both repos
  are set to the same candidate string.

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

- GitHub Pages must remain configured to deploy from GitHub Actions for
  `emboss-rs`.
- GHCR publication requires GitHub Actions package write permission on the
  `emboss-rs` repository. No extra secret is required when publishing with the
  repository `GITHUB_TOKEN`.
- GitHub Releases must remain enabled in both repositories.

## Manual release steps

The workflows automate build, packaging, and release staging. Humans still own:

- final changelog editing
- final release-note review
- final version bump and tag creation
- final go/no-go decision after stabilization audit

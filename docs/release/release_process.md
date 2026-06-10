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
  - configured to run on `v*` tags
  - configured to support manual dispatch for release-candidate verification
  - verifies checked-in release metadata before packaging
  - runs the local release gate
  - assembles the target-platform/docs/validation artefact bundle through `make release-artifacts`
  - builds the container image and publishes to GHCR when configured
  - attaches artefacts to a GitHub release on tag builds
  - intentionally disabled manually as of 2026-06-10; until re-enabled, these
    hosted release checks must be replaced by local execution of the matching
    `make` targets

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

- target-platform `emboss-rs` binary tarball
- SHA256 checksum for the target-platform tarball
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
  when repository policy allows it and the workflow is enabled. The
  `emboss-rs` workflows are intentionally disabled manually as of 2026-06-10,
  so Pages publication is suspended until the docs workflow is re-enabled. If
  repository or organization policy blocks automatic enablement after that,
  Pages must be set manually to deploy from GitHub Actions for `emboss-rs`.
- GHCR publication requires GitHub Actions package write permission on the
  `emboss-rs` repository and an enabled release workflow. No extra secret is
  required when publishing with the repository `GITHUB_TOKEN`.
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
- at least one compared anchor per shipped family
- comparison-coverage reporting
- harvest-coverage reporting
- full-compared-cohort reporting
- retained-backlog-closure reporting
- explicit full-compared-cohort release status once achieved
- stable post-closure summary semantics once the evidence backlog is closed
- drift-free release-facing counts and report links
- honest release-note wording

`Unreleased` changelog and release-facing wording should continue to defer to:

- `docs/generated/cohort_validation.md`
- `docs/generated/governance_alignment.md`
- `docs/generated/cohort_health.md`
- `docs/generated/comparison_coverage.md`
- `docs/generated/harvest_coverage.md`
- `docs/generated/full_compared_cohort.md`
- `docs/generated/retained_backlog_closure.md`

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

By default, `make release-artifacts` targets `linux-x86_64` and now fails fast
unless the local host reports `linux-x86_64`. This prevents a macOS or ARM
binary from being packaged under a Linux x86_64 archive name. If a different
platform bundle is intentionally being produced, set both
`RELEASE_TARGET_OS=<os>` and `RELEASE_TARGET_ARCH=<arch>` explicitly and describe
the resulting artefact platform honestly in the release evidence. The first
public `1.0.0` release still requires a validated Linux x86_64 bundle before
cutover.

## Local CI-parity while GitHub Actions are suspended

The `emboss-rs` GitHub Actions workflows are intentionally disabled manually as
of 2026-06-10. While they remain disabled, do not claim hosted CI, hosted Pages,
hosted release, or hosted container coverage. Use this local command set as the
minimum replacement for the repository-owned parts of those checks:

1. `cargo fmt --check`
2. `make lint-repo`
3. `make check-sister-repo`
4. `make lint-docs`
5. `make docs`
6. `cargo test --workspace --all-features`
7. `make release-version-check`
8. `make release-truth-check`
9. `make release-generated-check`
10. `make release-check`
11. `make release-artifacts` on a host matching the intended
    `RELEASE_TARGET_OS` and `RELEASE_TARGET_ARCH`

The broad `make ci` target remains useful for day-to-day local parity, but the
explicit sequence above is preferred for release-candidate evidence while
hosted workflows are suspended because it also captures release-truth,
generated-artifact, release-gate, and bundle freshness checks.

Environment prerequisites:

- Rust toolchain and Cargo must be available for formatting, builds, Clippy,
  and workspace tests.
- The selected Python/Sphinx environment must provide the docs requirements.
- `../emboss-r` should be present when cross-repository awareness is part of
  the local validation claim; otherwise `make check-sister-repo` can only
  report that the sibling repository is unavailable.
- Docker must be available before claiming Linux container smoke validation
  with `make release-container`.
- The default `make release-artifacts` target requires a Linux x86_64 host.
  On other hosts it is valid to run the earlier release checks and container
  smoke checks, but do not claim a Linux x86_64 binary archive unless the
  binary was built and packaged on that target platform.
- GitHub Pages deployment, GHCR publication, release attachment, and
  repository-token permission behavior cannot be validated locally while the
  hosted workflows are disabled.

With the shipped cohort now fully compared and fully harvested, `make
release-truth-check` also treats these as hard release conditions:

- `docs/generated/validation/full_compared_cohort.json` must report
  `full_compared_cohort: true`
- `docs/generated/validation/harvest_coverage.json` must report
  `harvest_coverage_complete: true`
- release-facing documents must state `Full compared cohort: yes` explicitly
  rather than leaving the status to be inferred from counts alone
- every shipped family row in `docs/generated/validation/comparison_coverage.json`
  must remain fully compared once the full-compared milestone has been reached
- `docs/generated/validation/shipped_cohort.validation.json` must keep
  `gapped_method_count: 0` in the fully closed post-closure state
- `docs/generated/validation/cohort_health.json` must keep
  `weakest_evidence_family: null` when `weak_evidence_method_count: 0`

If either condition regresses, the release gate must fail until the generated
reports and underlying evidence state are brought back into alignment.

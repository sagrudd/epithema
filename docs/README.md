# Documentation

Project-governing documentation is maintained under [governance/index.md](./governance/index.md).
Contributor and Codex workflow guidance is maintained under
[development/index.md](./development/index.md).

The canonical governance document is
[governance/emboss_rs_governance_manual.md](./governance/emboss_rs_governance_manual.md).
Supporting policy modules and reference appendices are organized beneath the same
tree for long-term maintenance and future Sphinx ingestion.

## Local Build

Install the documentation dependencies with:

```bash
python3 -m pip install -r docs/requirements.txt
```

Build the documentation site from the repository root with:

```bash
make docs
```

The generated HTML is written to `docs/_build/html/`.

Autodoc-generated Markdown source pages live under `docs/generated/`. They are
intended to be committed as deterministic Sphinx source artefacts and refreshed
through the governed CLI path:

```bash
cargo run -p emboss-cli -- autodoc <path-to-autodoc.json> --emit-docs
```

Structured validation-evidence stubs for tool examples can also be derived from
the same autodoc inputs:

```bash
cargo run -p emboss-cli -- autodoc <path-to-autodoc.json> --emit-validation-stub
```

The baseline validation stubs can then be upgraded for the committed
acceptance-anchor cohort through:

```bash
make anchor-validation
```

The committed autodoc input contracts that drive those generated pages live
under [`docs/autodoc/tools/`](./autodoc/README.md). Refresh the committed
registry-backed stubs with `make autodoc-stubs`, then rebuild generated pages
through the canonical CLI path with `make autodoc-refresh`. Refresh the shipped
cohort evidence roll-up with `make cohort-report`, which now also refreshes the
executed-and-compared anchor reports before aggregating the cohort summary.

By default these reports are written under `docs/generated/validation/` as
machine-readable JSON. They represent declared and harvested evidence only; they
do not imply that the corresponding cases were executed or compared yet.

A cohort-level report across the shipped governed registry can also be derived:

```bash
make cohort-report
```

This refreshes both:

- `docs/generated/validation/shipped_cohort.validation.json`
- `docs/generated/cohort_validation.md`

The cohort report uses the actual shipped tool registry as its source of truth
and records documentation completeness, validation-stub presence, evidence
maturity, and visible gaps per tool.

A governance-alignment report can also be derived to reconcile the maintained
family-to-tool appendix against the shipped Rust registry plus current curated
autodoc and evidence state:

```bash
make governance-report
```

This refreshes both:

- `docs/generated/validation/governance_alignment.json`
- `docs/generated/governance_alignment.md`

```bash
make cohort-health-report
```

This refreshes both:

- `docs/generated/validation/cohort_health.json`
- `docs/generated/cohort_health.md`

The cohort-health gate turns the cohort and governance reports into a standing
reprioritization check. It makes three things explicit:

- which family currently carries the largest retained backlog
- which shipped family currently carries the largest below-compared evidence burden
- whether the release-candidate readiness report has fallen behind the generated cohort state

The governance-alignment report uses the governance appendix as the backlog
source of truth and makes three things explicit:

- which shipped methods are governed `retain` versus `rework`
- which retained governance methods remain unshipped backlog
- which shipped methods have curated autodoc coverage and executable or compared
  evidence

At this stage generated pages include validated narrative content, declared
artefacts, declared example stubs, provenance, and transformation notes when
available. Tool execution, acquisition, and acceptance reporting remain
deferred.

Provider-backed documentation artefacts are enforced through the governed
EMBOSS-RS acquisition seam. Until a real provider implementation exists,
`emboss-rs autodoc` will reject such inputs rather than allowing ad hoc direct
downloads inside docgen.

The root `Makefile` is the canonical entry point for common repository tasks.
Run `make help` from the repository root to see the current task surface.

Additional documentation-oriented targets currently available are:

- `make build` for a full Rust workspace build
- `make fmt` for Rust formatting checks
- `make lint` for workspace-wide Clippy validation
- `make test` for workspace-wide Rust tests
- `make lint-docs` for strict Sphinx structure and cross-reference checks
- `make lint-repo` for lightweight repository-structure and governance-entry checks
- `make check-sister-repo` for a read-only compatibility-awareness check against
  `../emboss-r` when that sibling repository is present locally
- `make ci` to run the current local CI-equivalent validation set
- `make docs-clean` to remove generated documentation output
- `make docs-live` for a live-reloading preview when `sphinx-autobuild` is
  installed in the selected Python environment
- `make release-version-check` to verify the checked-in Cargo and Sphinx
  release metadata are aligned
- `make release-generated-check` to refresh governed generated artefacts and
  require a clean diff
- `make release-artifacts` to assemble the reproducible local release bundle
  under `dist/release/<version>/`

## Baseline CI Validation

The baseline CI workflow validates the current repository state without assuming
future features such as `emboss-rs autodoc`.

At present CI enforces:

- Rust formatting checks
- Rust Clippy validation
- Rust tests
- release-mode Rust build verification
- release-generated docs and validation artefact freshness verification
- repository-structure and governance entry-point checks
- read-only awareness of the sister `emboss-r` repository when available
- strict Sphinx validation
- full Sphinx HTML build
- Linux-first container smoke build verification

## GitHub Pages Publication

GitHub Pages is the formal public publication path for the EMBOSS-RS
documentation site from project start.

Publication is handled by the GitHub Actions workflow at
[`../.github/workflows/docs-pages.yml`](../.github/workflows/docs-pages.yml).
The workflow:

- runs automatically on pushes to `main`
- supports manual publication through `workflow_dispatch`
- provisions the repository Pages site automatically when the repository token
  is allowed to manage Pages configuration
- rebuilds the Sphinx site from `docs/requirements.txt`
- runs the release-oriented docs target so the published site stays aligned
  with release metadata
- uploads `docs/_build/html/` as the Pages artifact and deploys it through the
  standard GitHub Pages deployment actions

## Repository Setting Note

The publication workflow now requests GitHub Pages enablement automatically via
`actions/configure-pages`. If an organization policy or repository setting
still blocks that change, the equivalent manual configuration is:

- **Source:** `GitHub Actions`

This setting lives under **Settings -> Pages** in the GitHub repository UI.

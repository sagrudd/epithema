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

By default these reports are written under `docs/generated/validation/` as
machine-readable JSON. They represent declared and harvested evidence only; they
do not imply that the corresponding cases were executed or compared yet.

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

## Baseline CI Validation

The baseline CI workflow validates the current repository state without assuming
future features such as `emboss-rs autodoc`.

At present CI enforces:

- Rust formatting checks
- Rust Clippy validation
- Rust tests
- repository-structure and governance entry-point checks
- read-only awareness of the sister `emboss-r` repository when available
- strict Sphinx validation
- full Sphinx HTML build

## GitHub Pages Publication

GitHub Pages is the formal public publication path for the EMBOSS-RS
documentation site from project start.

Publication is handled by the GitHub Actions workflow at
[`../.github/workflows/docs-pages.yml`](../.github/workflows/docs-pages.yml).
The workflow:

- runs automatically on pushes to `main`
- supports manual publication through `workflow_dispatch`
- rebuilds the Sphinx site from `docs/requirements.txt`
- uploads `docs/_build/html/` as the Pages artifact and deploys it through the
  standard GitHub Pages deployment actions

## Required Repository Setting

A repository administrator must enable GitHub Pages with:

- **Source:** `GitHub Actions`

This setting lives under **Settings -> Pages** in the GitHub repository UI.

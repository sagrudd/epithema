# Documentation

Project-governing documentation is maintained under [governance/index.md](./governance/index.md).

The canonical governance document is
[governance/emboss_rs_governance_manual.md](./governance/emboss_rs_governance_manual.md).
Supporting policy modules and reference appendices are organized beneath the same
directory for long-term maintenance and future Sphinx ingestion.

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

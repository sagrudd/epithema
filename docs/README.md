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

# Autodoc Inputs

The committed autodoc contracts for the exposed EMBOSS-RS tool surface live
under `docs/autodoc/tools/`.

These JSON files are the canonical documentation-preparation inputs for
generated tool pages. Refresh them from the current governed tool registry with:

```bash
make autodoc-stubs
```

Rebuild the generated Markdown pages through the governed CLI path with:

```bash
make autodoc-refresh
```

This intentionally routes page generation through `emboss-rs autodoc` rather
than a parallel ad hoc script. CI and repository tests treat these contracts as
required coverage for every exposed tool.

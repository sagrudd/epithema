# emboss-rs

A reboot of the EMBOSS package of bioinformatics tools in Rust.

## Workspace

The Rust workspace is organized under `crates/` with a single CLI binary named
`emboss-rs` and domain-oriented library crates for core primitives, IO, tools,
service/runtime, diagnostics/provenance, plot contracts, R bridging, fixtures,
validation, doc generation, configuration, and provider abstraction.

The current top-level command surface is intentionally small: `emboss-rs list`
provides service-backed discovery, `emboss-rs autodoc` reserves the governed
documentation command path, and future governed tools will execute as
`emboss-rs <tool> ...`.

The Rust workspace also includes `emboss-r-bridge`, which provides the typed
Rust-side contract seam for the first-class sister package `emboss-r`.

The first shipped tool cohort now covers sequence-stream and sequence-selection
operations through the governed single-binary surface:

- `emboss-rs seqcount <input>` counts sequence records in a local FASTA, FASTQ,
  EMBL, or GenBank file.
- `emboss-rs nthseq <input> <index>` selects the 1-based Nth sequence record
  and emits FASTA.
- `emboss-rs skipseq <input> <count>` skips the first N records and emits the
  remaining records as FASTA.
- `emboss-rs notseq <input> <index>` emits all records except the 1-based
  excluded record as FASTA.
- `emboss-rs newseq <identifier> <sequence>` creates a new sequence record from
  inline residues, with optional `--description` and `--molecule` hints, and
  emits FASTA.

The `emboss-docgen` crate owns the versioned JSON contract that future
`emboss-rs autodoc` runs will consume for reproducible documentation inputs.
The current `emboss-rs autodoc <path>` command validates that contract and
prints a normalized summary. With `--emit-docs`, it also writes deterministic
generated Markdown pages under `docs/generated/`. With
`--emit-validation-stub`, it also derives a structured tool-evidence JSON stub
under `docs/generated/validation/`. The same crate also contains a legacy
EMBOSS artefact discovery layer for tool-focused harvesting from a local
historical source tree, plus a typed legacy-to-autodoc transformation layer
that emits provenance-rich autodoc JSON. Provider-backed documentation artefacts
must now pass through the formal EMBOSS-RS acquisition seam; docgen will reject
ad hoc downloader-style references and still reports provider acquisition as
not implemented until a real governed provider path exists.

## Documentation

Project-governing documentation is maintained under [docs/](./docs/README.md).
The canonical governance entry point is
[docs/governance/index.md](./docs/governance/index.md).

GitHub Pages is the formal public publication path for the documentation site.
Repository administrators must set Pages to deploy from **GitHub Actions** so
the workflow in `.github/workflows/docs-pages.yml` can publish the built Sphinx
site.

## Contributor Workflow

Contributor guidance is provided in [CONTRIBUTING.md](./CONTRIBUTING.md) and in
the development workflow section of the docs site at
[docs/development/index.md](./docs/development/index.md).

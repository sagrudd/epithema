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
Rust-side contract seam for the first-class sister package `emboss-r`. The
R-facing bridge now grows through narrow typed methods rather than CLI
emulation and now covers the shipped first-class R cohort for sequence
construction/editing, feature operations, translation, pattern scanning,
statistics, alignment summaries, and the plotted `charge` profile path.

Cross-surface validation fixtures for the first-class R interface now live
under
`crates/emboss-testkit/tests/fixtures/cross_surface/curated_methods.json`.
`emboss-rs` owns that canonical semantic fixture catalogue, and `emboss-r`
consumes it to verify that its public wrappers agree with Rust-native method
behavior for a curated subset of sequence, table/report, alignment-summary,
and charge-profile outputs. This compares normalized structured semantics, not
CLI formatting or pixel output.

Plot-ready analytical outputs should target the typed JSON-serializable
`emboss-plot-contract` crate. Rendering remains owned by the sister `emboss-r`
package, which consumes that contract for governed plot families such as line,
scatter, and bar plots. The first production end-to-end plotting slice is
`charge`: Rust computes the analytical profile and emits the contract, while
`emboss-r` renders the plot. The next governed plot-family method is
`pepwindow`, which now emits the same kind of typed line contract from Rust and
remains ready for its sibling R renderer.

Release engineering guidance now lives under
[docs/release/](./docs/release/index.md). The coordinated first stable release
target is `emboss-rs` `1.0.0` paired with `emboss-r` `1.0.0`, with checked-in
`1.0.0` version metadata, tag-driven release automation, Linux binary
packaging, Sphinx docs gating, release-bundle assembly under
`dist/release/<version>/`, and a GHCR container image path in place for the
final cutover. The local pre-release path is intentionally explicit:

- `make release-version-check`
- `make release-generated-check`
- `make release-check`
- `make release-artifacts`
- `make release-container`

`make release-artifacts` produces the target-platform tarball, checksum, docs
archive, validation archive, and a release manifest JSON that records the exact
checked-in version, platform label, and artefact names for the candidate build.
The default release target is `linux-x86_64`, and the Makefile fails fast if the
local host does not match that target so a macOS or ARM binary is not mislabeled
as a Linux x86_64 release artefact.

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

The next shipped cohort now covers extraction and partitioning operations:

- `emboss-rs extractseq <input> <start> <end>` extracts the same 1-based
  inclusive region from each input record and emits FASTA.
- `emboss-rs cutseq <input> <position>` cuts each input record after the
  supplied 1-based interior position and emits left/right FASTA fragments.
- `emboss-rs union <input-a> <input-b> [input-c ...]` concatenates multiple
  sequence inputs in deterministic input order and emits FASTA.
- `emboss-rs splitter <input> <chunk-size>` partitions a sequence stream into
  fixed-size record chunks and emits deterministic FASTA partitions.

The next shipped cohort now covers simple cleanup and editing operations:

- `emboss-rs degapseq <input>` removes `-` and `.` gap characters from each
  input record and emits FASTA.
- `emboss-rs revseq <input> [--reverse-only | --complement]` reverses each
  input record and defaults to molecule-aware reverse-complement behavior for
  nucleotide records in auto mode.
- `emboss-rs trimseq <input> [--left <count>] [--right <count>]` trims explicit
  residue counts from the ends of each record and emits FASTA.
- `emboss-rs descseq <input>` reports stable per-record description and
  metadata summary rows for plain or annotated sequence inputs.

The next shipped cohort now covers feature-driven masking, extraction, and
annotation-copy operations:

- `emboss-rs maskseq <input> <start:end> [start:end ...] [--mask-char <char>]`
  masks explicit 1-based inclusive coordinate intervals in each input record.
- `emboss-rs maskfeat <input> [--kind <kind>] [--name <name>] [--qualifier <key[=value]>]`
  masks selected simple feature spans from annotated EMBL or GenBank inputs.
- `emboss-rs extractfeat <input> [--kind <kind>] [--name <name>] [--qualifier <key[=value]>]`
  extracts one rebased output record per selected simple feature.
- `emboss-rs featcopy <source> <target> [--kind <kind>] [--name <name>] [--qualifier <key[=value]>]`
  copies selected features from annotated source records onto identifier-matched,
  equal-length target records.

The next shipped cohort now covers translation-adjacent operations:

- `emboss-rs backtranseq <protein-input>` back-translates protein records to
  deterministic representative DNA codons using the standard genetic code.
- `emboss-rs backtranambig <protein-input>` back-translates protein records to
  deterministic ambiguous DNA codons using IUPAC nucleotide ambiguity.
- `emboss-rs checktrans <nucleotide-input> <protein-input>` strictly translates
  frame-1 DNA coding sequences with the standard genetic code and compares them
  against expected protein records paired by input order.

The next shipped cohort now covers simple deterministic pattern-search
operations:

- `emboss-rs fuzznuc <nucleotide-input> <pattern>` scans forward nucleotide
  sequences for exact or IUPAC-ambiguous patterns and reports 1-based inclusive
  hit coordinates.
- `emboss-rs fuzzpro <protein-input> <pattern>` scans protein sequences for
  exact patterns with `X` wildcard support and reports 1-based inclusive hit
  coordinates.
- `emboss-rs fuzztran <nucleotide-input> <protein-pattern>` translates all
  three forward frames and reports translated-protein hits mapped back to
  1-based inclusive nucleotide coordinates.

The next shipped cohort now covers simple composition and summary-statistics
operations:

- `emboss-rs compseq <input>` reports per-record and aggregate residue counts
  and frequencies for nucleotide or protein sequence inputs.
- `emboss-rs geecee <nucleotide-input>` reports per-record and aggregate GC
  counts and GC percentages using canonical bases in the denominator.
- `emboss-rs pepstats <protein-input>` reports per-record protein composition,
  residue length, and molecular-weight estimates. pI estimation is deferred in
  v1.

The next shipped cohort now covers codon-usage and coding-bias operations:

- `emboss-rs chips <coding-input>` reports per-record and aggregate codon counts
  and frequencies for strict in-frame coding-sequence inputs.
- `emboss-rs codcopy <coding-or-profile-input> [--profile-out <path>]`
  normalizes codon usage into a reusable tab-separated profile.
- `emboss-rs cai <coding-input> <reference-input>` reports deterministic
  CAI-like values against a coding-sequence or normalized-profile reference.
- `emboss-rs codcmp <left-input> <right-input>` compares codon counts and
  frequencies across two coding-sequence or normalized-profile sources.

The next shipped cohort now covers alignment-utility operations:

- `emboss-rs aligncopy <input>` copies a single aligned FASTA or Stockholm
  alignment unchanged and emits Stockholm by default.
- `emboss-rs aligncopypair <input>` copies a pairwise alignment unchanged and
  rejects inputs that do not contain exactly two rows.
- `emboss-rs infoalign <input>` reports alignment-level and per-row summary
  statistics including row count, column count, ungapped length, and gap count.
- `emboss-rs extractalign <input> [--row <ordinal>] [--row-id <identifier>] [--start <column>] [--end <column>]`
  extracts selected rows plus an optional 1-based inclusive column slice and
  emits the resulting sub-alignment as Stockholm.

The next shipped cohort now covers global pairwise alignment operations:

- `emboss-rs needle <query-input> <target-input> [--gap-open <penalty>] [--gap-extend <penalty>]`
  performs deterministic global pairwise alignment between exactly one query
  sequence and one target sequence and emits Stockholm by default.
- `emboss-rs needleall <query-input> <target-input> [--gap-open <penalty>] [--gap-extend <penalty>]`
  performs deterministic many-vs-many global pairwise alignments in query-major
  order and reports a structured comparison table.

The next shipped cohort now covers modernized sequence retrieval operations:

- `emboss-rs seqret <input>` normalizes a local sequence file or resolves a
  provider-qualified accession through the governed acquisition seam and emits
  normalized FASTA.
- `emboss-rs refseqget <provider-qualified-accession>` retrieves a single
  provider-backed accession-addressable sequence through the governed
  acquisition seam and emits normalized FASTA.

The next shipped cohort now covers modern archive metadata and run acquisition:

- `emboss-rs runinfo <archive-accession>` normalizes one ENA or SRA
  accession-backed archive metadata record into a structured report.
- `emboss-rs runget <run-accession>` discovers a normalized public-run manifest
  through the governed archive acquisition seam and reports provider file URLs,
  checksums, and byte counts when available.

The next shipped cohort now covers alignment-summary and similarity operations:

- `emboss-rs matcher <query-input> <target-input>` compares exactly one query
  and one target record over their shared ungapped overlap and reports
  identities, mismatches, and overlap-based identity percentage.
- `emboss-rs distmat <input>` computes a deterministic equal-length p-distance
  matrix for a sequence set and renders it as a stable table.
- `emboss-rs cons <input>` derives a simple majority non-gap consensus from one
  alignment and emits FASTA.
- `emboss-rs consambig <input>` derives an ambiguity-aware consensus from one
  alignment, using nucleotide IUPAC ambiguity where possible, and emits FASTA.

The explicit retained-exception tool now available in v1 is:

- `emboss-rs complex <input> --k-min <k> --k-max <k> [--window <length> --step <length>]`
  computes canonical-nucleotide linguistic complexity for whole sequences and,
  when requested, deterministic sliding windows.

The first end-to-end plot-producing tool now available is:

- `emboss-rs charge <protein-input> [--window <length>] [--step <length>] [--plot-contract-out <path>]`
  computes a sliding-window mean protein charge profile, emits a structured
  report, and can write the canonical typed line-plot contract JSON for
  rendering through the sister `emboss-r` package.

The next governed Rust-side plot-contract method now available is:

- `emboss-rs pepwindow <protein-input> [--window <length>] [--step <length>] [--plot-contract-out <path>]`
  computes a sliding-window Kyte-Doolittle hydropathy profile, emits a
  structured report, and can write the canonical typed line-plot contract JSON.
  Rendering for this method remains intentionally R-owned and is a follow-on
  task for the sister `emboss-r` package.

The provider layer now also supports formal library/service-backed single
sequence retrieval for provider-qualified accession inputs. The initial
implemented routes are:

- `ena:<accession>` for ENA browser FASTA retrieval of one nucleotide record.
- `ncbi:nuccore:<accession>` or `ncbi:protein:<accession>` for NCBI E-utilities
  FASTA retrieval of one nucleotide or protein record.
- `ncbi:<refseq-accession>` for a conservative subset of safe RefSeq prefixes
  where the database can be inferred without broad guessing.

Bare accessions remain conservative in v1: they flow through the shared
accession-resolution seam but are not automatically fetched unless the caller
provides an explicit provider route.

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
The workflow in `.github/workflows/docs-pages.yml` now provisions Pages through
the standard GitHub Pages actions and publishes the built Sphinx site from
`main`. The Pages build now runs the release-oriented docs task so that the
published documentation stays aligned with the checked-in release metadata.

## Contributor Workflow

Contributor guidance is provided in [CONTRIBUTING.md](./CONTRIBUTING.md) and in
the development workflow section of the docs site at
[docs/development/index.md](./docs/development/index.md).

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
- `emboss-rs revseq <input>` reverses each input record and emits FASTA. This
  v1 implementation performs plain reversal only.
- `emboss-rs trimseq <input> [--left <count>] [--right <count>]` trims explicit
  residue counts from the ends of each record and emits FASTA.
- `emboss-rs descseq <input> --description <text>` or
  `emboss-rs descseq <input> --clear` replaces or clears sequence descriptions
  while preserving identifiers and sequence content.

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

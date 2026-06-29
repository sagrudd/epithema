# ngsget

> Generated from validated autodoc input. Edit the source autodoc document rather than this page.

## Summary

Plan and materialize public ENA or SRA NGS assets for one study, sample, experiment, or run accession

## Document Metadata

- Document ID: `ngsget-v1`
- Schema version: `epithema.autodoc/v1`
- Source mode: `curated`
- Tool family: `archive_tools`

## Evidence Status

- Declared evidence baseline: `declared_evidence`
- Machine-readable validation report: [`../validation/ngsget.validation.json`](../validation/ngsget.validation.json)
- This page records declared documentation and evidence intent only. Runnable, executed, or compared validation evidence is tracked through the machine-readable validation report and the shipped cohort validation report.

## Overview

`ngsget` is the acquisition companion to `ngslist`. It resolves a public ENA or SRA NGS study, sample, experiment, or run accession to a normalized run-level manifest, selects generated FASTQ assets by default, optionally adds raw or submitted assets alongside any available FASTQ when `--raw` is present, materializes files under the documented run layout, and writes NGS provenance JSON.

## Inputs

The command interface is `epithema ngsget <accession> [--provider auto|ena|sra] [--out <dir>] [--raw] [--threads <n>] [--transport https|auto|aspera] [--ascp <path>] [--aspera-key <path>] [--aspera-rate <rate>] [--check-downloads <path>]`. Provider-qualified accessions such as `ena:PRJNA1011899` and `sra:SRR123456` are part of the contract. The optional `--threads <n>` value allows 1 to 20 concurrent direct downloads and defaults to 1. The optional `--transport aspera` mode uses IBM Aspera `ascp` for ENA public file URLs that map to ENA's public FASP endpoints; `--transport auto` uses Aspera when `ascp` and an auth key are discoverable and otherwise uses HTTPS. When `--ascp` is omitted, `ngsget` resolves `ascp` from `PATH`; `--transport aspera` fails early if `ascp` cannot be found. `--ascp`, `--aspera-key`, and `--aspera-rate` configure the external Aspera command. When `--aspera-key` is omitted, `ngsget` creates and uses an epithema-managed copy of a discovered Aspera package key; it does not synthesize a fresh SSH key because ENA's public FASP endpoints only accept trusted public-data keys. The optional `--check-downloads` path recursively searches an existing download tree for same-name files before network retrieval, copies verified matches into the output tree, leaves originals intact, and reports failed materialization records for same-name candidates with unexpected checksums. Custom container selection is not exposed; SRA FASTQ conversion uses the pinned default SRA Toolkit container recorded in provenance.

## Conda-Based Aspera Setup

Epithema supports Aspera acceleration through conda-compatible user environments. Install `ascli` with `micromamba create -n aspera -c conda-forge -c bioconda aspera-cli`, activate the environment, run `ascli config transferd install` to install IBM's Aspera transfer runtime under `$HOME/.aspera/sdk`, and then run `ascli config ascp info >/dev/null` to force `ascli` to materialize its lazily generated SDK bypass keys. Add `$HOME/.aspera/sdk` to the environment `PATH` through `$CONDA_PREFIX/etc/conda/activate.d/aspera-sdk.sh`, then verify `command -v ascp`, `ascli config ascp show`, and the presence of `aspera_bypass_rsa.pem` or `aspera_bypass_dsa.pem` under `$HOME/.aspera/sdk`. The Bioconda `aspera-cli` package alone does not place `ascp` on `PATH`; the transfer runtime install is required. Epithema searches next to the resolved `ascp` executable, `$CONDA_PREFIX/etc`, `$HOME/.aspera/connect/etc`, and `$HOME/.aspera/sdk` for `aspera_bypass_rsa.pem`, `aspera_bypass_dsa.pem`, `aspera_tokenauth_id_rsa`, `aspera_tokenauth_id_dsa`, or `asperaweb_id_dsa.openssh`. If no key is found and `ascli` is available on `PATH`, epithema silently runs `ascli config ascp info` once and repeats key discovery. For SDK bypass keys that require a passphrase, epithema obtains the UUID by preferring `ascli --format=text config echo @ruby:Aspera::Ascp::Installation.instance.ssh_cert_uuid`, which avoids version-specific `--show-secrets` parsing, and then falls back across older `config ascp info` forms plus plain info output. It passes the UUID only to the spawned `ascp` process through `ASPERA_SCP_PASS`; the secret is not recorded in provenance, summaries, or logs. The `aspera_bypass_*.pem` files are SDK bypass keys created by the Aspera transfer runtime and are accepted by epithema's ENA Aspera path; epithema prefers `aspera_bypass_rsa.pem` when available and refreshes its managed cache from the current SDK before falling back to older cached keys. If discovery still fails, pass `--ascp $HOME/.aspera/sdk/ascp --aspera-key $HOME/.aspera/sdk/aspera_bypass_rsa.pem` explicitly.

## Outputs

The documented output layout is `<out>/manifest.tsv`, `<out>/provenance.json`, and `runs/<run_accession>/` subdirectories for `fastq`, `raw`, `sra`, and logs. Implemented service records capture selected and skipped assets, expected and observed byte counts and MD5 checksums, direct-download verification state, Aspera `ascp` command details when that transport is used, SRA Toolkit conversion command details, container image and tool version metadata, exit status, and generated FASTQ output paths. Direct provider downloads stream to `.partial` files, resume existing partials when the provider honors byte-range requests, verify from disk, and emit CLI progress events with per-download transfer-speed estimates plus roughly two-second active-transfer heartbeat repaints so raw archives in the tens or hundreds of gigabytes are not buffered in memory or left looking idle while an external transfer is still running. Aspera progress is measured from live filesystem artifacts in the target run directory rather than from `ascp` terminal output alone, so sibling final-name artifacts created by the external transfer can drive the byte counter. The service can write a stable handoff `manifest.tsv` for later object-store importers without uploading or publishing objects.

## Current Status

`ngsget` is exposed through the governed command route. The implementation supports download planning, direct ENA-style materialization with streamed partial-file handling, byte-range resume, optional ENA Aspera transfer through `--transport aspera`, bounded concurrent direct downloads through `--threads`, disk-based verification, speed-aware CLI progress reporting with active-transfer spinner heartbeats and live filesystem artifact measurement for Aspera transfers, recursive existing-download lookup through `--check-downloads`, copy-then-verify reuse of matching files without modifying the originals, SRA archive download plus pinned-container FASTQ conversion through an injectable runner, deterministic failure/resume semantics, provenance JSON writing, stable handoff manifest TSV writing, and mocked ENA/SRA validation fixtures. Remaining follow-up work includes per-run log files, custom container selection, and opt-in live-provider or Docker/SRA Toolkit validation.

## Caveats

The first `ngsget` implementation is limited to public ENA/SRA datasets. Protected-access, dbGaP-controlled, credentialed, requester-pays, and object-store publication workflows are not implemented and must remain explicit future work. Routine validation is mocked and deterministic; live-provider or Docker/SRA Toolkit checks should stay opt-in gated checks.

## Declared Artifacts

### Mocked service NGS ingestion case fixture

- Artifact ID: `ngsget_service_ingestion_case`
- Origin: fixture asset
- Acquisition: fixture
- Reference: managed asset `crates/epithema-testkit/tests/fixtures/autodoc/ngsget_service_ingestion_case.md`
- Notes: Repository-managed case note covering the mocked service-layer NGS planning, materialization, SRA conversion, and provenance serialization tests.

## Declared Examples

### Materialize generated FASTQ assets for a public NGS accession

- Example ID: `materialize_public_ngs_fastq_assets`
- Description: Plans generated FASTQ acquisition for a public ENA or SRA accession and records materialized-file provenance through mocked service seams.
- Referenced artifacts: `ngsget_service_ingestion_case`
- Expected outputs:
  - `ngs_provenance_json`: NGS provenance JSON (A stable `epithema.ngs-provenance/v1` JSON document describing query metadata, selected and skipped assets, materialization records, verification evidence, and generated FASTQ paths.)

## Provenance

- Curated by: epithema maintainers
- Source references: none declared

## Declared Validation Intent

This section describes what future governed validation should execute or compare. It is not evidence that those runs have already happened.

- Declared required examples for future validation: `materialize_public_ngs_fastq_assets`
- Future legacy comparison requested: no
- Future execution must capture provenance: yes

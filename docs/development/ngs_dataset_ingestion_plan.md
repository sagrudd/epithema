# NGS Dataset Ingestion Implementation Plan

This page defines the implementation tasks for the governed NGS dataset
acquisition milestone described in the
[Scope and Tool-Family Policy](../governance/policies/scope_and_tool_family_policy.md).

The milestone is a Strategic Add outside the coordinated `1.0.0` release
scope. The governed `ngslist` and `ngsget` surfaces are now implemented for the
public ENA/SRA acquisition workflow described below; protected-access,
credentialed, requester-pays, object-store publication, and live provider
validation workflows remain explicit future work.

## Target User Surface

The planned commands are:

```text
epithema ngslist <accession> [--provider auto|ena|sra] [--format table|json]
epithema ngsget <accession> [--provider auto|ena|sra] [--out <dir>] [--raw] [--threads <n>] [--transport https|auto|aspera] [--ascp <path>] [--aspera-key <path>] [--aspera-rate <rate>] [--check-downloads <path>]
```

`ngslist` reports the assets associated with a study, sample, experiment, or
run accession. `ngsget` downloads generated FASTQ by default. With `--raw`,
`ngsget` downloads any available generated FASTQ and also includes raw or
submitted assets such as BAM, CRAM, FAST5, POD5, tar archives, and
provider-native SRA files when the provider exposes them. With
`--check-downloads <path>`, `ngsget` should recursively search an existing
download root for same-name files, copy verified matches into the output tree
without modifying the original files, and fail the selected asset if a
same-name candidate has an unexpected checksum. With `--threads <n>`, `ngsget`
allows concurrent direct downloads for ENA-style file URLs; the default is `1`
and the maximum accepted value is `20`. With `--transport aspera`, `ngsget`
uses IBM Aspera `ascp` for ENA public file URLs that can be mapped to ENA's
public FASP endpoints. `--transport auto` uses Aspera when `ascp` and an auth
key are discoverable and otherwise falls back to HTTPS. `--ascp`,
`--aspera-key`, and `--aspera-rate` configure the external `ascp` command. When
`--aspera-key` is omitted, `ngsget` creates and uses an epithema-managed copy of
a discovered Aspera package key; it does not synthesize a fresh SSH key because
ENA's public FASP endpoints only accept trusted public-data keys.

Custom container selection is not part of the current command surface. SRA FASTQ
conversion uses the pinned default SRA Toolkit container recorded by the service
provenance model.

## Functional Scope

The implementation must support these accession levels:

- study or project accessions such as `PRJNA1011899`, `PRJEB...`, `SRP...`, and
  `ERP...`
- sample accessions such as `SAMN...`, `SAMEA...`, `SRS...`, and `ERS...`
- experiment accessions such as `SRX...` and `ERX...`
- run accessions such as `SRR...` and `ERR...`

All accepted queries should resolve to a normalized run-level manifest before
download selection occurs.

## Output Contracts

`ngslist` should emit a stable tabular report with one row per asset. Required
columns are:

- `provider`
- `query_accession`
- `query_object_class`
- `study_accession`
- `study_title`
- `sample_accession`
- `sample_title`
- `experiment_accession`
- `run_accession`
- `instrument_platform`
- `instrument_model`
- `library_strategy`
- `library_source`
- `library_selection`
- `library_layout`
- `asset_role`
- `asset_format`
- `source_url`
- `size_bytes`
- `checksum_md5`

Stable asset roles are:

- `generated_fastq`
- `submitted_raw`
- `submitted_alignment`
- `sra_archive`
- `index`
- `unknown_submitted`

`ngsget` must write:

- downloaded or generated data files
- `manifest.tsv`
- `provenance.json`
- per-run logs for downloads, extraction, and verification

The default output layout is:

```text
<out>/
  manifest.tsv
  provenance.json
  runs/
    <run_accession>/
      fastq/
      raw/
      sra/
      logs/
```

## Provenance JSON Contract

The provenance document should use schema label
`epithema.ngs-provenance/v1`. It must include:

- the original query accession, selected provider, resolved object class, and
  query timestamp
- normalized study metadata, including title and secondary accessions when
  available
- normalized sample, experiment, and run metadata
- every considered asset, whether selected or skipped
- every downloaded or generated local file
- expected and observed byte counts when available
- expected and observed MD5 checksums when available
- verification status for every materialized asset
- SRA Toolkit command details when SRA archives are converted to FASTQ
- container image, digest if available, and tool versions when containerized
  extraction is used
- Epithema version and provider route metadata

## Implementation Tasks

1. Add NGS archive domain models.

   Extend `crates/epithema-providers` with provider-neutral models for
   `NgsQuery`, `NgsObjectClass`, `NgsRunMetadata`, `NgsAsset`,
   `NgsManifest`, `NgsDownloadPlan`, `NgsDownloadRecord`, and
   `NgsProvenance`.

   Status: implemented in `crates/epithema-providers/src/ngs.rs` as
   provider-neutral Rust models. Serialization and provider-specific filling of
   these models remain later tasks in this plan.

2. Add accession classification for NGS queries.

   Implement conservative parsing for study, sample, experiment, and run
   accession prefixes. Provider-qualified forms such as `ena:PRJNA1011899` and
   `sra:SRR...` should be accepted. Ambiguous bare prefixes should route through
   `auto` only when a deterministic provider strategy is documented.

   Status: implemented as `NgsQuery::classify` in
   `crates/epithema-providers/src/ngs.rs`. The classifier accepts bare
   accessions plus `auto:`, `ena:`, and `sra:` qualified forms, resolves the
   NGS object class, and leaves bare or `auto:` queries provider-neutral for a
   later service routing decision.

3. Implement ENA manifest expansion.

   Add an ENA NGS adapter that expands study, sample, experiment, and run
   queries to `read_run` rows through the ENA Portal API. Normalize generated
   FASTQ fields, submitted file fields, checksums, byte counts, study titles,
   sample titles, sequencing metadata, and provider route metadata.

   Status: implemented in `crates/epithema-providers/src/ena_ngs.rs` as an
   ENA provider adapter that builds `read_run` file-report requests and
   normalizes mocked multi-run TSV responses into `NgsManifest`.

4. Implement SRA manifest expansion.

   Add an SRA NGS adapter that normalizes SRA RunInfo responses into the same
   run-level manifest contract. Where SRA does not expose generated FASTQ URLs,
   represent the provider-native SRA archive as the source asset and mark FASTQ
   as generated through a conversion step rather than direct download.

   Status: implemented in `crates/epithema-providers/src/sra_ngs.rs` as an
   SRA provider adapter that builds RunInfo requests, normalizes mocked
   multi-run CSV responses into `NgsManifest`, records provider-native SRA
   archive assets, and represents FASTQ materialization through
   `sra-convert://.../fastq` conversion locators.

5. Add service-level NGS acquisition gateways.

   Add `ServiceNgsRetrieval` under `crates/epithema-service` with methods for
   manifest listing, download planning, materialization, verification, and
   provenance writing. This service must enforce existing provider enablement
   and remote-acquisition policy checks.

   Status: implemented in `crates/epithema-service/src/ngs_retrieval.rs` as a
   service-owned NGS gateway for ENA/SRA manifest listing, download planning,
   materialization, verification during materialization, provenance writing,
   and stable handoff manifest writing. The gateway enforces remote-acquisition
   policy, provider registry membership, archive-acquisition capability, and
   per-provider enablement before routing to the provider adapters.

6. Implement `ngslist`.

   Add the governed tool descriptor, CLI routing, text help, tabular rendering,
   JSON rendering, service tests, and generated autodoc input for `ngslist`.
   The first acceptance fixtures should cover one ENA study query that expands
   to multiple runs and one run-level query.

   Status: implemented across `crates/epithema-tools`,
   `crates/epithema-service`, `crates/epithema-cli`, and
   `docs/autodoc/tools/ngslist.json`. The service path supports
   `epithema ngslist <accession> [--provider auto|ena|sra] [--format
   table|json]`, emits the documented one-row-per-asset table by default, and
   provides deterministic JSON text rendering. Mocked service coverage includes
   an ENA study query that expands to multiple runs and an ENA run-level JSON
   rendering case. The command remains a manifest-listing surface only; download
   planning and materialization are later tasks.

7. Implement download planning for `ngsget`.

   Build a deterministic selector that chooses `generated_fastq` assets by
   default and adds raw/submitted assets only when `--raw` is present. The plan
   must be previewable in tests without network downloads.

   Status: implemented in `crates/epithema-service/src/ngs_retrieval.rs` as
   the service-owned `ServiceNgsRetrieval::plan_downloads` selector. The
   planner preserves manifest order, selects only `generated_fastq` assets by
   default, and includes submitted raw, submitted alignment, index,
   provider-native SRA archive, and unknown submitted assets when raw inclusion
   is requested. Coverage uses in-memory manifests and does not perform network
   downloads or file materialization.

8. Implement direct ENA downloads.

   Add resumable or idempotent file materialization with `.partial` files,
   atomic rename after verification, byte-count checks, MD5 verification, and
   skip-on-verified behavior.

   Status: implemented for direct ENA-style file URLs in
   `crates/epithema-service/src/ngs_retrieval.rs`. The materializer streams
   provider response bodies directly to `.partial` files, emits CLI progress
   events with transfer-speed estimates for long downloads, verifies size and
   MD5 evidence from disk, and promotes only after available byte-count and
   checksum evidence passes. It writes selected assets to the documented
   `runs/<run>/fastq`,
   `runs/<run>/raw`, or `runs/<run>/sra` layout, leaves failed verification as
   a partial file, resumes existing `.partial` files when the provider honors
   byte-range requests, skips already verified local files, supports optional
   ENA Aspera `ascp` downloads through `ngsget --transport aspera`, supports
   bounded concurrent direct downloads through `ngsget --threads <n>`, and supports
   service-layer `ngsget --check-downloads <path>` behavior: recursive
   same-name lookup, copy-then-verify into the output tree, original source
   preservation, and failed materialization records for same-name candidates
   with unexpected checksum evidence. SRA provider materialization and
   `sra-convert://` FASTQ extraction remain task 9.

9. Implement SRA FASTQ extraction.

   Support SRA archive acquisition followed by FASTQ extraction with
   `prefetch` plus `fasterq-dump`, preferably through a pinned container image
   first. Record the SRA archive, extraction outputs, command lines, exit
   statuses, tool versions, and container metadata in provenance.

   Status: implemented in `crates/epithema-service/src/ngs_retrieval.rs`.
   SRA materialization downloads direct provider-native SRA archive assets when
   a direct URL is available, runs `sra-convert://.../fastq` assets through a
   pinned-container conversion runner using
   `docker.io/ncbi/sra-tools:3.1.1`, and records the `prefetch` plus
   `fasterq-dump` command line, container image, tool version, exit status, and
   discovered FASTQ output paths. Unit coverage injects a fake runner for
   deterministic successful and failed conversion cases; live Docker/SRA
   Toolkit validation remains an explicit future gated check rather than a
   routine local test.

10. Add provenance serialization.

    Write `provenance.json` from the provider-neutral provenance model. Include
    selected, skipped, downloaded, converted, verified, and failed assets.

    Status: implemented in `crates/epithema-service/src/ngs_retrieval.rs`.
    The service gateway writes stable pretty-printed `provenance.json` from
    `NgsProvenance`, including the schema label, Epithema version, acquisition
    timestamp, query/provider/route metadata, normalized run metadata, all
    considered assets marked as selected or skipped, materialization records
    with expected and observed verification evidence, SRA command/container
    fields when present, and a flattened local-file list for downloaded and
    generated outputs. Deterministic service coverage verifies direct-download
    provenance serialization. Stable failure and resume behavior remains task
    11.

11. Add failure and resume semantics.

    Define stable behavior for missing checksums, missing byte counts, partial
    downloads, checksum mismatch, provider 404, unsupported object class,
    unsupported provider, and interrupted conversion.

    Status: implemented in `crates/epithema-service/src/ngs_retrieval.rs`.
    The service semantics are covered with deterministic tests: size-only or
    checksum-only direct downloads verify when the available evidence matches;
    downloads with no checksum or byte count are promoted but marked
    `unverified`; checksum or byte-count mismatches leave `.partial` files and
    record failed verification; retry behavior resumes incomplete `.partial`
    files when the provider accepts byte-range requests and otherwise restarts
    the direct download; provider 404 responses produce failed records
    without creating partial files; unclassified queries and unsupported
    provider routes return stable service error codes; interrupted SRA
    conversion is recorded as a failed materialization with exit status `-1`.

12. Add validation fixtures.

    Add mocked ENA and SRA provider responses for study, sample, experiment,
    and run queries. Include fixture cases for paired FASTQ, raw nanopore
    FAST5/POD5-style submitted assets, BAM/CRAM-style submitted assets,
    checksum mismatch, and SRA conversion planning.

    Status: implemented with fixture files under
    `crates/epithema-providers/tests/fixtures/ngs/` and
    `crates/epithema-service/tests/fixtures/ngs/`. Provider tests now load
    ENA `read_run` TSV fixtures and SRA RunInfo CSV fixtures for study, sample,
    experiment, and run-level queries. The ENA study fixture covers paired
    FASTQ, FAST5/POD5 raw submitted assets, BAM/CRAM submitted alignments, and
    BAI/CRAI indexes. SRA fixtures verify provider-native archive assets plus
    generated FASTQ conversion locators. The service checksum-mismatch test now
    uses a FASTQ fixture body. Live-provider fixture refresh remains outside
    routine CI by design.

13. Add generated documentation and release-facing caveats.

    Add autodoc JSON contracts for `ngslist` and `ngsget`, generate tool pages,
    and document that the first implementation supports public ENA/SRA datasets
    only. Protected-access, dbGaP-controlled, and object-store publication
    workflows must remain explicit future work unless implemented.

    Status: implemented with curated autodoc contracts for `ngslist` and
    `ngsget`, generated tool pages, and generated validation stubs. The
    `ngslist` page documents the shipped manifest-listing boundary. The
    `ngsget` page documents the governed command route for public ENA/SRA
    acquisition, generated FASTQ default selection, optional raw/submitted
    assets alongside any available FASTQ, recursive verified reuse through
    `--check-downloads`, bounded concurrent direct downloads through
    `--threads`, optional ENA Aspera transfer through `--transport aspera`,
    streamed large-file direct downloads with speed-aware CLI progress and
    resumable `.partial` files, SRA conversion through the pinned default
    container, provenance JSON, and stable handoff manifest behavior.
    Release-facing scope and notes now explicitly keep protected-access,
    dbGaP-controlled, credentialed,
    requester-pays, object-store publication, custom container selection, and
    live-provider validation workflows outside the current NGS milestone.

14. Add object-store handoff readiness.

    Keep object-store upload out of the first `ngsget` implementation, but make
    provenance and manifest paths stable enough for a later importer to load
    files, metadata, checksums, and run/sample/study relationships without
    reparsing provider-specific reports.

    Status: implemented in `crates/epithema-service/src/ngs_retrieval.rs` as
    `ServiceNgsRetrieval::write_manifest`. The service writes a stable
    `manifest.tsv` handoff file from the normalized `NgsProvenance` bundle,
    including provider/query metadata, study/sample/experiment/run
    relationships, asset roles and formats, expected checksums and byte counts,
    selection status, local materialized paths, generated FASTQ paths, observed
    verification evidence, verification status, materialization method, and
    failure reason. This remains an importer-ready local handoff only; object
    upload, publication, credentials, and requester-pays workflows are still
    explicitly outside the implemented scope.

## Validation Expectations

Initial validation should be mocked and deterministic. Live-provider validation
may be added later as an explicit gated check, but the first implementation
should not make routine CI depend on ENA or SRA availability.

The minimum local checks before marking the milestone complete are:

- provider-unit tests for ENA and SRA normalization
- service tests for policy enforcement and manifest expansion
- tool tests for `ngslist` table and JSON output
- download-planning tests for default FASTQ and `--raw` selection
- provenance serialization tests
- checksum and byte-count verification tests
- SRA conversion command-planning tests
- generated docs and validation-report freshness checks

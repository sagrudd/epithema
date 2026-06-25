Service-backed NGS materialization case for the planned `ngsget` surface.

- Service coverage: `materializes_direct_ena_downloads_with_verification`
- Service coverage: `materializes_sra_archive_and_records_fastq_conversion_plan`
- Service coverage: `writes_ngs_provenance_json_with_selected_skipped_and_records`
- Service coverage: `writes_ngs_handoff_manifest_for_object_store_importers`
- Evidence boundary: mocked provider and runner seams in Rust service tests, not live ENA/SRA acquisition
- Current exposure boundary: service-layer acquisition pieces exist, but the governed `epithema ngsget` CLI route is not yet shipped

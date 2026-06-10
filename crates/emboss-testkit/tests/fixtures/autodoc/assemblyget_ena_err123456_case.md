# assemblyget mocked ENA manifest-intent case

This fixture records the governed `assemblyget` example that resolves
`ena:ERR123456` through the mocked ENA archive metadata route and emits the
bounded manifest-intent table.

The mocked ENA response contains two FASTQ file records for run `ERR123456`
under study/project `ERP000001`, with known byte sizes `10` and `12`. The
expected table therefore records:

- provider `ena`
- requested accession `ERR123456`
- object class `run`
- selected assembly/study accession `ERP000001`
- linked run accession `ERR123456`
- route endpoint `ena.portal.filereport`
- manifest mode `manifest_intent_only`
- file count `2`
- total known bytes `22`
- materialization status `not_materialized`

This fixture is intentionally not a live-provider evidence claim. It proves the
checked-in governed service seam, table shape, and explicit no-materialization
policy for the shipped v1 surface.

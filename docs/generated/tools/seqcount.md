# seqcount

> Generated from validated autodoc input. Edit the source autodoc document rather than this page.

## Summary

Count sequence records in an input stream

## Document Metadata

- Document ID: `seqcount-stub-v1`
- Schema version: `emboss-rs.autodoc/v1`
- Source mode: `curated`
- Tool family: `sequence_stream`
- Legacy names: `seqcount`

## Overview

`seqcount` is part of the exposed EMBOSS-RS `sequence_stream` cohort. This page is a generated baseline documentation stub produced through the governed autodoc path so the shipped tool surface remains fully documented even where richer harvested narrative or executable examples are still pending.

## Inputs

This tool accepts local sequence records or in-memory sequence payloads through the shared sequence IO abstraction. Record ordering is deterministic and preserved unless the method explicitly transforms it.

## Outputs

The current implementation emits normalized sequence records or simple structured counts for stream-oriented sequence selection methods.

## Current Status

This method is implemented and exposed through `emboss-rs seqcount`. The generated tool page and the machine-readable validation stub at [`../validation/seqcount.validation.json`](../validation/seqcount.validation.json) are current. No richer autodoc examples are declared in this contract yet; future prompts should replace or extend this stub with harvested or executable evidence rather than hand-maintaining the generated page directly.

## Caveats

Baseline stub coverage documents the exposed command surface and links to available validation evidence, but it does not imply that all historical EMBOSS examples, rendered screenshots, or legacy comparisons have been captured yet.

## Declared Artifacts

No artifacts are declared for this autodoc document.

## Declared Examples

No examples are declared for this autodoc document.

## Provenance

- Curated by: emboss-rs autodoc stub generator
- Source references: none declared


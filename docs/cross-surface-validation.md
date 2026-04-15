# Cross-Surface Validation

`emboss-rs` is the computational source of truth for the first-class sister
package `emboss-r`. Cross-surface validation exists to prove that the public R
wrappers are a peer client of the same analytical core rather than a divergent
reimplementation.

## Current strategy

- `emboss-rs` owns the canonical fixture catalogue.
- The catalogue is stored at
  `crates/emboss-testkit/tests/fixtures/cross_surface/curated_methods.json`.
- Each fixture case contains:
  - a typed bridge request payload
  - a semantic expected output at the nearest useful comparison layer
- `emboss-r` reads that catalogue, runs its public wrappers from the same
  inputs, normalizes its returned objects, and compares them to the canonical
  expected outputs.

## Equality rules

- Sequence records compare identifier, sequence, description, molecule,
  alphabet, and length.
- Sequence collections compare ordered lists of normalized sequence records.
- Report-style outputs compare normalized row objects in deterministic row
  order.
- Distance matrices compare identifiers, mode, sequence length, and numeric
  cell values.
- Charge profiles compare window positions and mean-charge values. Plot objects
  and rendered image output are intentionally out of scope here.

Floating-point comparisons use the catalogue's documented default tolerance of
`1e-9`.

## Current curated coverage

The current fixture-driven coverage spans:

- `newseq`
- `seqcount`
- `nthseq`
- `skipseq`
- `notseq`
- `extractseq`
- `degapseq`
- `trimseq`
- `descseq`
- `compseq`
- `geecee`
- `pepstats`
- `matcher`
- `distmat`
- `cons`
- `consambig`
- `charge_profile`

## What this does not prove

This harness does not yet validate the entire method catalog, live retrieval,
or graphical pixel output. It is a curated semantic-equivalence layer for the
early R analytical surface.

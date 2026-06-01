# psiphi

> Generated from validated autodoc input. Edit the source autodoc document rather than this page.

## Summary

Report deterministic per-residue phi and psi torsion angles from bounded protein backbone coordinates

## Document Metadata

- Document ID: `psiphi-v1`
- Schema version: `emboss-rs.autodoc/v1`
- Source mode: `curated`
- Tool family: `protein_coordinates`
- Legacy names: `psiphi`

## Evidence Status

- Declared evidence baseline: `declared_evidence`
- Machine-readable validation report: [`../validation/psiphi.validation.json`](../validation/psiphi.validation.json)
- This page records declared documentation and evidence intent only. Runnable, executed, or compared validation evidence is tracked through the machine-readable validation report and the shipped cohort validation report.

## Overview

`psiphi` is the bounded protein-coordinate member of the active protein-property rework program. The EMBOSS-RS v1 surface computes deterministic per-residue phi and psi torsion-angle rows from one local PDB-like coordinate input and returns them as a stable table-first report.

## Inputs

The current interface accepts exactly one local coordinate file. The bounded v1 seam retains only PDB `ATOM` backbone records for `N`, `CA`, and `C`, accepts only blank or `A` alternate locations, and rejects provider-backed acquisition, inline literals, and coordinate inputs that do not retain any eligible backbone atoms.

## Outputs

The result is a stable per-residue table with chain identifier, residue identity, insertion code, backbone-atom presence flags, previous/next continuity flags, and bounded `phi_degrees` and `psi_degrees` values. Torsions remain absent rather than inferred when required backbone atoms or same-chain sequential continuity are missing.

## Current Status

This method is implemented and exposed through `emboss-rs psiphi`. The shipped interim surface is executable-only at this checkpoint: Rust coverage exercises the bounded local coordinate path plus rejection of provider-backed and backbone-free inputs through the same governed computation path, but compared acceptance evidence and harvested legacy provenance are not yet closed in this task.

## Caveats

The v1 `psiphi` seam is intentionally narrow. It does not render Ramachandran plots, does not generalize into a broader structural-analysis framework, and does not impute missing atoms, normalize alternate conformers beyond the bounded `A` policy, or claim comparative coordinate-analysis behavior.

## Declared Artifacts

### Backbone coordinate fixture for governed psiphi validation

- Artifact ID: `psiphi_fixture`
- Origin: fixture asset
- Acquisition: fixture
- Reference: managed asset `crates/emboss-tools/tests/fixtures/psiphi_backbone.txt`
- Notes: Repository-managed backbone coordinate fixture used for deterministic bounded psiphi validation.

## Declared Examples

### Compute a deterministic bounded torsion-angle profile

- Example ID: `psiphi_profile_example`
- Description: Reports stable per-residue phi and psi rows from the committed local backbone coordinate fixture through the governed protein-coordinate seam.
- Referenced artifacts: `psiphi_fixture`
- Expected outputs:
  - `psiphi_table`: Bounded psiphi analytical table (Stable per-residue torsion-angle rows with explicit continuity and backbone-presence fields.)

## Provenance

- Curated by: emboss-rs maintainers
- Source references: none declared

## Declared Validation Intent

This section describes what future governed validation should execute or compare. It is not evidence that those runs have already happened.

- Declared required examples for future validation: `psiphi_profile_example`
- Future legacy comparison requested: no
- Future execution must capture provenance: no

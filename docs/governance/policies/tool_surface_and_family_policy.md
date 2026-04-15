# Tool Surface and Family Policy

## 1. Scope

This policy defines the governed public execution surface for `emboss-rs` and
the expectations for maintaining a coherent EMBOSS-RS tool family.

## 2. Normative Rules

- The project exposes a single binary surface: `emboss-rs <tool> ...`.
- New user-facing tools are introduced as governed subcommands, not as a growing
  set of unrelated binaries.
- Tool additions must fit an explicit family structure and should be documented
  in the maintained family-to-tool reference.
- Public naming should remain consistent with the broader EMBOSS reboot and with
  the project's documentation terminology.
- R-facing plotting work remains the responsibility of `emboss-r`.

## 3. Reference Linkage

The canonical governance context for this policy is
[EMBOSS-RS Governance Manual](../emboss_rs_governance_manual.md).

The maintained registry location for family-to-tool tracking is
[Family-to-Tool Mapping Reference](../appendices/family_to_tool_mapping_reference.md).

#!/usr/bin/env python3
"""Normalize the generated docs index against the current tool page set."""

from __future__ import annotations

from pathlib import Path


HEADER = """# Generated Tool Documentation

This section contains Markdown pages generated from validated `emboss-rs autodoc` inputs. These files are deterministic documentation source artefacts intended for Sphinx ingestion.

## Contents

```{toctree}
:maxdepth: 1
:caption: Generated Tools

cohort_validation
governance_alignment
cohort_health
comparison_coverage
"""

FOOTER = "```\n"


def main() -> int:
    root = Path("docs/generated")
    tools_root = root / "tools"
    if not tools_root.exists():
        raise SystemExit("docs/generated/tools does not exist")

    pages = sorted(
        path.stem
        for path in tools_root.glob("*.md")
        if path.is_file() and path.stem != "index"
    )
    if not pages:
        raise SystemExit("docs/generated/tools does not contain any tool pages")

    content = HEADER + "".join(f"tools/{page}\n" for page in pages) + FOOTER
    (root / "index.md").write_text(content)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())

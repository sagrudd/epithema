#!/usr/bin/env python3
"""Ensure the generated docs index includes the cohort validation page."""

from __future__ import annotations

from pathlib import Path


def main() -> int:
    path = Path("docs/generated/index.md")
    lines = path.read_text().splitlines()
    if "cohort_validation" in lines:
        return 0

    for index, line in enumerate(lines):
        if line.startswith("tools/"):
            lines.insert(index, "cohort_validation")
            path.write_text("\n".join(lines) + "\n")
            return 0

    raise SystemExit("docs/generated/index.md does not contain any tool entries")


if __name__ == "__main__":
    raise SystemExit(main())

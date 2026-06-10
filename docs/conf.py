from __future__ import annotations

project = "Epithema"
author = "Mnemosyne Biosciences Ltd"
copyright = "2026, Mnemosyne Biosciences Ltd"
release = "1.0.0"

extensions = [
    "myst_parser",
    "sphinx.ext.githubpages",
]

source_suffix = {
    ".md": "markdown",
}

root_doc = "index"

exclude_patterns = [
    "_build",
    "Thumbs.db",
    ".DS_Store",
]

nitpicky = True
show_warning_types = True

myst_heading_anchors = 3

html_theme = "sphinx_rtd_theme"
html_title = "Epithema Documentation"
html_static_path = []

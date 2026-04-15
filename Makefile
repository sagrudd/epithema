PYTHON ?= python3
SPHINXBUILD ?= $(PYTHON) -m sphinx
SPHINXOPTS ?= -n -W --keep-going
DOCS_DIR := docs
DOCS_BUILD_DIR := $(DOCS_DIR)/_build
DOCS_HTML_DIR := $(DOCS_BUILD_DIR)/html
DOCS_LIVE_PORT ?= 8000

.DEFAULT_GOAL := help

.PHONY: help docs docs-clean docs-live lint-docs lint-repo check-sister-repo ci clean

help:
	@printf "%s\n" \
		"EMBOSS-RS project tasks" \
		"" \
		"Documentation:" \
		"  make docs        Build the Sphinx documentation site" \
		"  make lint-docs   Run strict Sphinx structure and reference checks" \
		"  make lint-repo   Validate required repository entry points and docs wiring" \
		"  make check-sister-repo  Inspect ../emboss-r read-only when present" \
		"  make ci          Run the current local CI-equivalent checks" \
		"  make docs-live   Start a live-reloading docs preview (requires sphinx-autobuild)" \
		"  make docs-clean  Remove built documentation output" \
		"" \
		"Housekeeping:" \
		"  make clean       Remove generated repository artefacts tracked by this Makefile" \
		"" \
		"Reserved for future extension:" \
		"  autodoc, validate, release, and container targets will be added as implemented"

# Documentation
docs:
	$(SPHINXBUILD) $(SPHINXOPTS) -b html $(DOCS_DIR) $(DOCS_HTML_DIR)

lint-docs:
	$(SPHINXBUILD) $(SPHINXOPTS) -b dummy $(DOCS_DIR) $(DOCS_BUILD_DIR)/lint

lint-repo:
	test -f README.md
	test -f Makefile
	test -f docs/index.md
	test -f docs/README.md
	test -f docs/governance/index.md
	test -f docs/governance/emboss_rs_governance_manual.md
	test -f .github/workflows/docs-pages.yml
	grep -n "^governance/index$$" docs/index.md
	grep -n "emboss_rs_governance_manual" docs/governance/index.md docs/README.md README.md
	grep -n "emboss-r" README.md docs/governance/emboss_rs_governance_manual.md docs/governance/appendices/foundational_architecture_brief.md

check-sister-repo:
	@if [ -d ../emboss-r ]; then \
		printf "%s\n" "Found sibling repository: ../emboss-r"; \
		test -f ../emboss-r/README.md; \
		grep -n "^# emboss-r$$" ../emboss-r/README.md; \
		grep -n "plots\|methods available in R\|R" ../emboss-r/README.md; \
	else \
		printf "%s\n" "../emboss-r is not present in this environment; skipping read-only compatibility awareness check."; \
	fi

docs-live:
	@$(PYTHON) -m sphinx_autobuild --version >/dev/null 2>&1 || { \
		printf "%s\n" \
			"sphinx-autobuild is not installed in the selected Python environment." \
			"Install it with: $(PYTHON) -m pip install sphinx-autobuild"; \
		exit 1; \
	}
	$(PYTHON) -m sphinx_autobuild \
		--port $(DOCS_LIVE_PORT) \
		$(DOCS_DIR) \
		$(DOCS_HTML_DIR)

docs-clean:
	rm -rf $(DOCS_BUILD_DIR)

# Housekeeping
clean: docs-clean

ci: lint-repo check-sister-repo lint-docs docs

# Future extension points:
# - autodoc
# - validate
# - release
# - container

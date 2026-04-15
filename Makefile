PYTHON ?= python3
SPHINXBUILD ?= $(PYTHON) -m sphinx
SPHINXOPTS ?= -n -W --keep-going
DOCS_DIR := docs
DOCS_BUILD_DIR := $(DOCS_DIR)/_build
DOCS_HTML_DIR := $(DOCS_BUILD_DIR)/html
DOCS_LIVE_PORT ?= 8000

.DEFAULT_GOAL := help

.PHONY: help docs docs-clean docs-live lint-docs clean

help:
	@printf "%s\n" \
		"EMBOSS-RS project tasks" \
		"" \
		"Documentation:" \
		"  make docs        Build the Sphinx documentation site" \
		"  make lint-docs   Run strict Sphinx structure and reference checks" \
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

# Future extension points:
# - autodoc
# - validate
# - release
# - container

.DEFAULT_GOAL := help

PYTHONPATH=
SHELL=/bin/sh
VENV=.venv
PYENV_VERSION=3.11

ifeq ($(OS),Windows_NT)
	VENV_BIN=$(VENV)/Scripts
else
	VENV_BIN=$(VENV)/bin
endif

.PHONY: set_python
set_python: ## Configure Python's version con pyenv
	export PYENV_VERSION=$(PYENV_VERSION) && $(MAKE) .venv

.PHONY: .venv
.venv: ## Set up Python virtual environment and install requirements
	python3 -m venv $(VENV)
	$(MAKE) requirements

.PHONY: requirements
requirements: ## Install/refresh Python project requirements
	$(VENV_BIN)/python -m pip install --upgrade pip
	$(VENV_BIN)/pip install --upgrade -r py-dars/requirements.txt
	$(VENV_BIN)/nbstripout --install --global


nbstrip_uninstall:
	nbstripout --uninstall --global


.PHONY: build-python
build-python: .venv  ## Compile and install Python Dars
	@$(MAKE) -s -C py-dars build

.PHONY: clean
clean:  ## Clean up caches and build artifacts
	@rm -rf .venv/
	@rm -rf target/
	@rm -f Cargo.lock
	@cargo clean
	@$(MAKE) -s -C py-dars/ $@

.PHONY: help
help:  ## Display this help screen
	@echo -e "\033[1mAvailable commands:\033[0m"
	@grep -E '^[a-z.A-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | awk 'BEGIN {FS = ":.*?## "}; {printf "  \033[36m%-22s\033[0m %s\n", $$1, $$2}' | sort

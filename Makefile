all: enigma

enigma: src/main.rs  ## Compile the enigma interpreter
	rustc src/main.rs -o enigma

.PHONY:
.SILENT:

VENV_DIR ?= .venv
VENV_ACTIVATE = $(VENV_DIR)/bin/activate
VENV_RUN = . $(VENV_ACTIVATE);

lint: $(VENV_DIR) ## Run linters
	$(VENV_RUN) pre-commit run --all-files
	cargo clippy

help: ## Show this help message
	grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

all: target/debug/enigma

target/debug/enigma: src/* ## Compile the enigma interpreter
	cargo build

.PHONY:
.SILENT:

init: ## Initialize pre-commit hooks
	uv venv
	uv pip install pre-commit
	uv run pre-commit install

run: target/debug/enigma ## Run the enigma interpreter
	cargo run examples/unary-addition.en

lint: ## Run linters
	uv run pre-commit run --all-files
	cargo clippy

help: ## Show this help message
	grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

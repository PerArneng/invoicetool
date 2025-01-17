
.PHONY: cargo-build
cargo-build: ## Build the project.
	cargo build

.PHONY: help
help: ## Display this help.
	@sh .make/makefile-help.sh $(MAKEFILE_LIST)
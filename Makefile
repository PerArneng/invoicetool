
.PHONY: cargo-build
cargo-build: ## Build the project.
	cargo build


.PHONY: test-run
test-run: ## Run the tests.
	cargo run -- test

.PHONY: help
help: ## Display this help.
	@sh .make/makefile-help.sh $(MAKEFILE_LIST)
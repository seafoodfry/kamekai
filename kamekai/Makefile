.DEFAULT_GOAL := help

.PHONY: help
help:  ## Display this help message
	@awk 'BEGIN {FS = ":.*##"; printf "\nUsage:\n  make \033[36m<target>\033[0m\n"} /^[a-zA-Z_-]+:.*?##/ { printf "  \033[36m%-20s\033[0m %s\n", $$1, $$2 } /^##@/ { printf "\n\033[1m%s\033[0m\n", substr($$0, 5) } ' $(MAKEFILE_LIST)


##@ Release Commands
.PHONY: release
release: qa
	pnpm tauri build --target aarch64-apple-darwin

##@ Local Development
.PHONY: run
run: qa  ## Run the tauri app
	pnpm tauri dev


##@ Source Code Management
.PHONY: fmt
fmt:  ## Format code
	pnpm format
	cd ./src-tauri && cargo fmt

.PHONY: lint
lint:  ## Lint code
	pnpm lint


##@ Quality Assurance.
.PHONY: qa-test
qa-test: lint  ## Run all quality assurance checks except for fmt (mean for CI)

.PHONY: qa
qa: fmt qa-test  ## Run all quality assurance checks
	@echo "QA checks passed"

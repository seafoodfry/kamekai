IMG := kamekai-backend
CONTAINER_OPTS := --rm \
	-e OTEL_SERVICE_NAME="kamekai-backend" \
	-e OTEL_EXPORTER_OTLP_ENDPOINT="https://api.honeycomb.io" \
	-e OTEL_EXPORTER_OTLP_PROTOCOL="http/protobuf" \
	-e OTEL_EXPORTER_OTLP_HEADERS="x-honeycomb-team=$$(op read 'op://eng-vault/honeycomb-api-key/password')" \
	-e AWS_ACCESS_KEY_ID \
	-e AWS_SECRET_ACCESS_KEY \
	-e AWS_SESSION_TOKEN


.DEFAULT_GOAL := help

help:  ## Display this help message
	@awk 'BEGIN {FS = ":.*##"; printf "\nUsage:\n  make \033[36m<target>\033[0m\n"} /^[a-zA-Z_-]+:.*?##/ { printf "  \033[36m%-20s\033[0m %s\n", $$1, $$2 } /^##@/ { printf "\n\033[1m%s\033[0m\n", substr($$0, 5) } ' $(MAKEFILE_LIST)


##@ Local Development.
.PHONY: build-dev
build-dev:  ## Build the dev container image
	docker build --target builder -t $(IMG)-dev .

.PHONY: debug
debug: build-dev  ## Run the dev container
	./run-cmd-in-shell.sh docker run -it $(CONTAINER_OPTS) $(IMG)-dev bash

.PHONY: build
build:  ## Build the prod container image
	docker build -t $(IMG) .

.PHONY: run
run: build  ## Run the prod container
	./run-cmd-in-shell.sh docker run -it $(CONTAINER_OPTS) $(IMG)



##@ Source Code Management.
.PHONY: update
update:  ## Update project dependencies
	cargo update

.PHONY: fmt
fmt:  ## Format code
	rustfmt -l src/*.rs

.PHONY: test
test:  ## Run tests
	cargo test

.PHONY: security
security:  ## Run security checks
	cargo audit


##@ Quality Assurance.
.PHONY: qa
qa: fmt test security  ## Run all quality assurance checks


.PHONY: qa-test
qa-test: test security  ## Run all quality assurance checks except for fmt (mean for CI)



##@ Development Cleanup.
.PHONY: clean
clean:  ## Delete build artifacts
	cargo clean
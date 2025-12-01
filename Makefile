.PHONY: sync build test build-rust test-rust test-parsing sync-python build-python test-python sync-node build-node test-node


##@ ----------------------------- Primary workflow -----------------------------

sync: sync-python sync-node ## Installs deps for the two bindings
 
build: build-rust build-python build-node ## Builds everything, including the bindings

test: test-rust test-python test-node ## Tests everything


##@ ---------------------------- The project itself ----------------------------

build-rust: ## Builds the project as release
	@echo "\n\n🧱 Running release build process for md2data\n\n"
	cargo build --release

test-rust: test-parsing test-formatting ## Tests all components of the rust project

test-parsing: ## Tests the parsers
	@echo "\n\n🧪 Testing structured parser\n\n"
	cargo test --test parser
	@echo "\n\n🧪 Testing minified parser (default)\n\n"
	cargo test --test parser_minified

test-formatting: ## Tests the formatting
	@echo "\n\nTried to test formatting, not implemented yet\n\n"


##@ ------------------------------ Python binding ------------------------------

sync-python: ## Installs deps
	@echo "\n\n📥 Installing python binding's dependencies...\n\n"
	cd bindings/python && uv sync --all-extras

build-python: ## Builds the binding
	@echo "\n\n🧱 Building python binding...\n\n"
	cd bindings/python && uv build

test-python: ## Runs integration tests
	@echo "\n\n🧪 Testing python binding...\n\n"
	cd bindings/python && uv run pytest


##@ ------------------------------- Node binding -------------------------------

sync-node: ## Installs deps
	@echo "\n\n📥 Installing node binding's dependencies...\n\n"
	cd bindings/node && npm install

build-node: ## Builds the binding
	@echo "\n\n🧱 Building node binding...\n\n"
	@# Detect platform and architecture
	@PLATFORM=$$(uname -s | tr '[:upper:]' '[:lower:]'); \
	ARCH=$$(uname -m); \
	if [ "$$PLATFORM" = "linux" ]; then \
		TARGET="x86_64-unknown-linux-gnu"; \
	elif [ "$$PLATFORM" = "darwin" ]; then \
		if [ "$$ARCH" = "arm64" ]; then \
			TARGET="aarch64-apple-darwin"; \
		else \
			TARGET="x86_64-apple-darwin"; \
		fi; \
	else \
		echo "Unsupported platform: $$PLATFORM"; \
		exit 1; \
	fi; \
	echo "Building for target: $$TARGET"; \
	cd bindings/node && npm run build -- --target $$TARGET && \
	if [ -f "index.node" ]; then \
		mv index.node "index.$$TARGET.node"; \
		echo "✅ Created index.$$TARGET.node"; \
	else \
		echo "⚠️  index.node not found, checking for platform-specific binary..."; \
		ls -la index.*.node 2>/dev/null || echo "No .node files found"; \
	fi

test-node: ## Runs integration tests
	@echo "\n\n🧪 Testing node binding...\n\n"
	cd bindings/node && npm test


##@ ----------------------------------- Help -----------------------------------

help: ## Display this help message
	@awk 'BEGIN {FS = ":.*##"; printf "\nUsage:$(NC)\n  make <target>$(NC)\n"} /^[a-zA-Z_-]+:.*?##/ { printf "  %-20s$(NC) %s\n", $$1, $$2 } /^##@/ { printf "\n%s$(NC)\n", substr($$0, 5) } ' $(MAKEFILE_LIST)

.DEFAULT_GOAL := help
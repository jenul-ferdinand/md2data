# ----------------------------- Primary workflow ----------------------------- #

# Installs deps for the two bindings
sync: sync-python sync-node
# Builds everything, including the bindings
build: build-rust build-python build-node
# Tests everything
test: test-rust test-python test-node

# ---------------------------- The project itself ---------------------------- #

# Builds the project as release
build-rust:
	@echo "\n\n🧱 Running release build process for md2data\n\n"
	cargo build --release
# Tests all components of the rust project
test-rust: test-parsing
# Tests the parsers
test-parsing:
	@echo "\n\n🧪 Testing structured parser\n\n"
	cargo test --test parser
	@echo "\n\n🧪 Testing minified parser\n\n"
	cargo test --test parser_minified

# ------------------------------ Python binding ------------------------------ #

# Installs deps
sync-python:
	@echo "\n\n📥 Installing python binding's dependencies...\n\n"
	cd bindings/python && uv sync --all-extras
# Builds the binding
build-python:
	@echo "\n\n🧱 Building python binding...\n\n"
	cd bindings/python && uv build
# Runs integration tests
test-python:
	@echo "\n\n🧪 Testing python binding...\n\n"
	cd bindings/python && uv run pytest

# ------------------------------- Node binding ------------------------------- #

# Installs deps
sync-node:
	@echo "\n\n📥 Installing node binding's dependencies...\n\n"
	cd bindings/node && npm install
# Builds the binding
build-node:
	@echo "\n\n🧱 Building node binding...\n\n"
	cd bindings/node && npm run build
# Runs integration tests
test-node:
	@echo "\n\n🧪 Testing node binding...\n\n"
	cd bindings/node && npm test

set shell := ['uv', 'run', '--frozen', 'bash', '-euxo', 'pipefail', '-c']
set unstable
set positional-arguments

project := "jsonlt"
package := "jsonlt"
pnpm := "pnpm exec"

# Run security audit
audit:
  cargo deny check
  cargo audit

# Clean build artifacts
clean:
  #!/usr/bin/env bash
  cargo clean
  rm -rf node_modules
  rm -rf .vale

# List available recipes
default:
  @just --list

# Generate documentation
doc *args:
  cargo doc --all-features --no-deps "$@"

# Run benchmarks
benchmark *args:
  cargo bench "$@"

# Build the library
build *args:
  cargo build "$@"

# Build for WASM target
build-wasm *args:
  cargo build --target wasm32-unknown-unknown --features wasm "$@"

# Format code
format:
  codespell -w
  cargo fmt
  {{pnpm}} biome format --write .

# Fix code issues
fix:
  cargo fmt
  cargo clippy --fix --allow-dirty --allow-staged
  {{pnpm}} biome format --write .
  {{pnpm}} biome check --write .

# Fix code issues including unsafe fixes
fix-unsafe:
  cargo fmt
  cargo clippy --fix --allow-dirty --allow-staged
  {{pnpm}} biome check --write --unsafe .

# Run all linters
lint: lint-rust lint-spelling lint-markdown lint-web
  yamllint --strict .

# Lint Markdown files
lint-markdown:
  {{pnpm}} markdownlint-cli2 "**/*.md"

# Lint prose in Markdown files
lint-prose:
  vale CODE_OF_CONDUCT.md CONTRIBUTING.md README.md SECURITY.md

# Lint Rust code
lint-rust:
  cargo fmt --check
  cargo clippy --all-targets --all-features -- -D warnings

# Check spelling
lint-spelling:
  codespell

# Lint web files (CSS, HTML, JS, JSON)
lint-web:
  {{pnpm}} biome check .

# Install all dependencies (Python + Node.js + Rust)
install: install-node install-python install-rust

# Install only Node.js dependencies
install-node:
  #!/usr/bin/env bash
  pnpm install --frozen-lockfile

# Install pre-commit hooks
install-prek:
  prek install

# Install only Python dependencies (for linting tools)
install-python:
  #!/usr/bin/env bash
  uv sync --frozen

# Install Rust toolchain and dependencies
install-rust:
  #!/usr/bin/env bash
  rustup show
  cargo fetch

# Run pre-commit hooks on changed files
prek:
  prek

# Run pre-commit hooks on all files
prek-all:
  prek run --all-files

# Generate SBOM for current environment
sbom output="sbom.cdx.json":
  uv run --isolated --group release cyclonedx-py environment --of json -o {{output}}

# Run tests (excludes slow tests by default)
test *args:
  cargo test "$@"

# Run all tests including ignored
test-all *args:
  cargo test --all-features -- --include-ignored "$@"

# Run conformance tests
test-conformance *args:
  cargo test --test conformance "$@"

# Run tests with coverage
test-coverage *args:
  cargo llvm-cov --all-features --lcov --output-path lcov.info "$@"

# Run integration tests
test-integration *args:
  cargo test --test '*' "$@"

# Run property-based tests
test-property *args:
  cargo test proptest "$@"

# Run WASM tests
test-wasm *args:
  wasm-pack test --headless --chrome --features wasm "$@"

# Sync Vale styles and dictionaries
vale-sync:
  vale sync

# Justfile for cargo-quickstart
set shell := ["bash", "-cu"]

default:
  @just --summary

# Dev-only tools (not required for CI)
ensure-dev-tools:
  which cargo-udeps         || cargo install cargo-udeps
  which cargo-outdated      || cargo install cargo-outdated
  which cargo-checkmate     || cargo install cargo-checkmate
  which cargo-readme        || cargo install cargo-readme
  which cargo-llvm-lines    || cargo install cargo-llvm-lines
  which sccache             || cargo install sccache
  which cargo-shear         || cargo install cargo-shear
  which cargo-msrv          || cargo install cargo-msrv

# CI-related tools
ensure-ci-tools:
  which cargo-nextest       || cargo install nextest
  which cargo-tarpaulin     || cargo install cargo-tarpaulin
  which cargo-release       || cargo install cargo-release
  which cargo-smart-release || cargo install cargo-smart-release

ensure-tools: ensure-dev-tools ensure-ci-tools

fmt:
  cargo +nightly fmt --all

clippy:
  cargo +nightly clippy --all-features -- -D warnings

test:
  cargo test --all

nextest:
  cargo nextest run --all

build:
  cargo build --workspace --all-features

release:
  cargo +stable build --release

cover:
  cargo tarpaulin --workspace --all-features --out Lcov

# LLVM coverage (HTML report)
cover-llvm:
  cargo llvm-cov clean --workspace
  cargo llvm-cov --workspace --all-features --html

# LLVM coverage (lcov.info for VSCode)
cover-lcov:
  cargo llvm-cov --workspace --all-features --lcov --output-path lcov.info

# LLVM coverage (table terminal output)
cover-summary:
  cargo llvm-cov --workspace --all-features --summary-only

# Full coverage generation (HTML + LCOV)
cover-full:
  cargo llvm-cov clean --workspace
  cargo llvm-cov --workspace --all-features --html
  cargo llvm-cov --workspace --all-features --lcov --output-path lcov.info

update:
  cargo update && cargo outdated || true

  # Just run quick checks
check:
    cargo check --all-features --workspace

clean:
  cargo clean

# Combined alias for dev-time tasks
dev: lint nextest

# Alias for clippy only
lint: fmt clippy

# Clean and rebuild
rebuild: clean build

ci: fmt clippy test

help:
  @echo "Available tasks:"
  @echo "  fmt       - Format code using rustfmt"
  @echo "  clippy    - Lint code using clippy"
  @echo "  test      - Run tests using cargo"
  @echo "  nextest   - Run tests using cargo-nextest"
  @echo "  build     - Debug build (beta)"
  @echo "  release   - Release build (stable)"
  @echo "  cover     - Coverage report with tarpaulin"
  @echo "  update    - Update + list outdated dependencies"
  @echo "  clean     - Clean artifacts"
  @echo "  ci        - Run full CI checks (fmt, clippy, test)"
  @echo "  ensure-tools - Install required cargo tools"
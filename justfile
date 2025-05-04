# Justfile for cargo-quickstart
set shell := ["bash", "-cu"]
set positional-arguments
set export

default:
  @just --summary

# Install all required cargo tools
setup: ensure-tools

# Dev-only tools (not required for CI)
ensure-dev-tools:
  which cargo-udeps         || cargo install cargo-udeps
  which cargo-outdated      || cargo install cargo-outdated
  which cargo-checkmate     || cargo install cargo-checkmate
  which cargo-shear         || cargo install cargo-shear
  which cargo-msrv          || cargo install cargo-msrv
  which sccache             || cargo install sccache
  which cargo-release       || cargo install cargo-release
  which cargo-smart-release || cargo install cargo-smart-release

# CI-related tools
ensure-ci-tools:
  which cargo-nextest       || cargo install cargo-nextest
  which cargo-llvm-cov      || cargo install cargo-llvm-cov

ensure-tools: ensure-dev-tools ensure-ci-tools

fmt:
  cargo fmt --all

check:
  cargo check --all-features --workspace

check-workspace:
  cargo udeps --all-targets --all-features --workspace

check-msrv:
  MSRV=$(grep 'rust-version' Cargo.toml | head -1 | cut -d '"' -f 2) && \
  echo "Testing MSRV: $MSRV" && \
  cargo msrv find --min 1.70 -- cargo check --workspace

clean:
  cargo clean

update:
  cargo update && cargo outdated || true

clippy:
  cargo clippy --all-features --workspace --all-targets -- -D warnings

lint: fmt clippy

lint-deps:
  just check-workspace
  cargo shear check

build:
  cargo build --workspace --all-features

rebuild: clean build

release:
  cargo build --release --workspace --all-features --all-targets

test:
  RUST_BACKTRACE=1 cargo test --all-features --workspace

nextest:
  RUST_BACKTRACE=1 cargo nextest run --all-features --workspace

nextest-fast:
  RUST_BACKTRACE=1 cargo nextest run --all-features --workspace --run-ignored=default

nextest-ignored:
  RUST_BACKTRACE=1 cargo nextest run --all-features --workspace --run-ignored=only

test-all:
  just test
  just nextest

# CI workflow tasks combined
ci: lint nextest-fast

bench:
  cargo bench --all-features --workspace

dev: lint nextest

validate:
  just fmt
  just clippy
  just lint-deps
  just nextest

docs:
  cargo doc --no-deps --all-features --workspace

docs-open:
  cargo doc --no-deps --all-features --workspace --open

run:
  cargo run --bin cargo-quickstart -- my-app --bin --yes

cover *FLAGS:
  cargo llvm-cov --workspace --all-features --lcov --output-path lcov.info {{FLAGS}}

cover-nextest *FLAGS:
  cargo llvm-cov nextest --workspace --all-features --lcov --output-path lcov.info {{FLAGS}}

watch TEST="":
  cargo watch -c -x "nextest run {{TEST}}"

watch-lint:
  cargo watch -c -x "fmt --all" -x "clippy --all-features -- -D warnings"

watch-cover:
  cargo watch -c -s "just cover --summary-only"

watch-cmd CMD:
  cargo watch -c -x "{{CMD}}"

help:
  @just --summary

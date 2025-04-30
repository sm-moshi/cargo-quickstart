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

# CI-related tools
ensure-ci-tools:
  which cargo-nextest       || cargo install cargo-nextest
  which cargo-release       || cargo install cargo-release
  which cargo-smart-release || cargo install cargo-smart-release
  which cargo-llvm-cov      || cargo install cargo-llvm-cov

ensure-tools: ensure-dev-tools ensure-ci-tools

fmt:
  cargo +nightly fmt --all

check:
  cargo check --all-features --workspace

check-workspace:
  cargo +nightly udeps --all-targets --workspace

check-msrv:
  MSRV=$(grep 'rust-version' Cargo.toml | head -1 | cut -d '"' -f 2) && \
  echo "Testing MSRV: $MSRV" && \
  cargo msrv find --min 1.70 -- cargo check --workspace

clean:
  cargo clean

update:
  cargo update && cargo outdated || true

clippy:
  cargo +nightly clippy --all-features --workspace -- -D warnings

lint: fmt clippy

lint-deps:
  just check-workspace
  cargo shear check

build:
  cargo build --workspace --all-features

rebuild: clean build

release:
  cargo +stable build --release --workspace --all-features

test:
  cargo test --all-features --workspace

nextest:
  cargo nextest run --all-features --workspace

nextest-fast:
  cargo nextest run --all-features --workspace --no-run-ignored

nextest-ignored:
  cargo nextest run --all-features --workspace --run-ignored=only

test-all:
  just test
  just nextest

bench:
  cargo bench --all-features --workspace

dev: lint nextest

validate:
  just fmt
  just clippy
  just lint-deps
  just test

docs:
  cargo doc --no-deps --all-features --workspace

docs-open:
  cargo doc --no-deps --all-features --workspace --open

run:
  cargo run --bin cargo-quickstart -- my-app --bin --yes

cover *FLAGS:
  cargo llvm-cov clean --workspace
  cargo llvm-cov --workspace --all-features --lcov --output-path lcov.info {{FLAGS}}

cover-nextest *FLAGS:
  cargo llvm-cov clean --workspace
  cargo llvm-cov nextest --workspace --all-features --lcov --output-path lcov.info {{FLAGS}}

watch TEST="":
  cargo watch -c -x "nextest run {{TEST}}"

watch-lint:
  cargo watch -c -x "+nightly fmt --all" -x "+nightly clippy --all-features -- -D warnings"

watch-cover:
  cargo watch -c -s "just cover --summary-only"

watch-cmd CMD:
  cargo watch -c -x "{{CMD}}"

help:
  @just --summary

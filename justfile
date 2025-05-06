# Justfile for cargo-quickstart
set shell := ["bash", "-cu"]
set positional-arguments
set export

default:
  @just --summary

# ────── TOOLING ──────────────────────────────────────────────
ensure-tool TOOL:
    if ! command -v {{TOOL}} > /dev/null; then \
        echo "Installing {{TOOL}}..."; \
        cargo install --locked {{TOOL}}; \
    fi

ensure-dev-tools:
    just ensure-tool cargo-udeps
    just ensure-tool cargo-outdated
    just ensure-tool cargo-checkmate
    just ensure-tool cargo-shear
    just ensure-tool cargo-msrv
    just ensure-tool sccache
    just ensure-tool cargo-release
    just ensure-tool cargo-smart-release

ensure-ci-tools:
    just ensure-tool cargo-nextest
    just ensure-tool cargo-llvm-cov
    just ensure-tool cargo-audit

ensure-tools: ensure-dev-tools ensure-ci-tools

# ────── BUILD ────────────────────────────────────────────────
build:
  cargo build --workspace

dev:
  just lint
  just nextest

rebuild: clean build

release:
  RUSTFLAGS="" cargo +stable build --release --workspace --all-features --all-targets

perf:
  cargo build --profile perf

clean:
  cargo clean

# ────── TEST ─────────────────────────────────────────────────
test:
  cargo test --all-features --workspace

nextest:
  cargo nextest run --all-features --workspace

nextest-fast:
  cargo nextest run --all-features --workspace --run-ignored=default

nextest-ignored:
  cargo nextest run --all-features --workspace --run-ignored=only

test-all:
  just test
  just nextest

miri:
  cargo +nightly miri test --profile miri --all-features --workspace

miri-nextest:
  MIRIFLAGS="-Zmiri-isolation-error=warn -Zalways-encode-mir" \
  cargo +nightly miri nextest run --all-features --workspace

# ────── BENCHMARKING ─────────────────────────────────────────
bench:
  cargo bench --all-features --workspace

bench-filter FILTER:
  cargo bench --all-features -- {{FILTER}}

bench-check:
  cargo check --benches --all-features --workspace

perf-bench:
  cargo bench --all-features --profile perf --bench template_benchmarks_pprof

perf-cmd:
  cargo bench --all-features --profile perf --bench command_benchmarks_pprof

open-flamegraph NAME:
  open target/criterion/{{NAME}}/profile/flamegraph.svg

# ────── LINT / VERIFY / DOCS ─────────────────────────────────
fmt:
  cargo fmt --all

clippy:
  cargo clippy --workspace --all-targets --all-features -- -D warnings

check:
  cargo check --workspace

lint: fmt clippy

lint-deps:
  cargo +nightly udeps --all-targets --all-features --workspace
  cargo shear

check-msrv:
  MSRV=$(grep 'rust-version' Cargo.toml | head -1 | cut -d '"' -f 2) && \
  echo "Testing MSRV: $MSRV" && \
  cargo msrv verify -- cargo check --workspace --all-features

audit:
  just ensure-tool cargo-audit
  cargo audit

update:
  cargo update && cargo outdated || true

docs:
  cargo doc --no-deps --all-features --workspace

docs-open:
  cargo doc --no-deps --all-features --workspace --open

# ────── COVERAGE ─────────────────────────────────────────────
cover *FLAGS:
  cargo llvm-cov clean --workspace
  cargo llvm-cov --workspace --all-features --lcov --output-path lcov.info {{FLAGS}}

cover-nextest *FLAGS:
  cargo llvm-cov clean --workspace
  cargo llvm-cov nextest --workspace --all-features --lcov --output-path lcov.info {{FLAGS}}

# ────── WATCH & RUN ──────────────────────────────────────────
watch TEST="":
  cargo watch -c -x "nextest run {{TEST}}"

watch-lint:
  cargo watch -c -x "fmt --all" -x "clippy --all-features -- -D warnings"

watch-cover:
  cargo watch -c -s "just cover --summary-only"

watch-cmd CMD:
  cargo watch -c -x "{{CMD}}"

run:
  cargo run --bin cargo-quickstart -- my-app --bin --yes

ci:
  just fmt
  just clippy
  just check
  just lint-deps
  just nextest-fast
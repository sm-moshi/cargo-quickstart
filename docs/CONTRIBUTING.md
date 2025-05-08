# Contributing to cargo-quickstart 🚀

Thank you for your interest in contributing to cargo-quickstart! This document provides guidelines and instructions for contributing.

## Development Setup 🛠️

1. **Rust Setup**
   - Minimum Rust version: 1.82.0
   - Use `rustup` to install the toolchain:
     ```bash
     rustup install 1.82.0
     ```

2. **Development Tools**
   Required tools for development:
   ```bash
   cargo install cargo-msrv cargo-audit cargo-udeps
   ```

## Feature Overview 📦

The project is split into two crates:

### quickstart-cli
- **doctor**: System health checks (`--features doctor`)
- **completions**: Shell completion generation (`--features completions`)
- **test-utils**: Testing utilities (dev only)

### quickstart-lib
- **test-utils**: Testing utilities for integration tests

## Development Workflow 🔄

1. **Branch Naming**
   - Features: `feature/name`
   - Fixes: `fix/issue-number`
   - Docs: `docs/topic`

2. **Commit Messages**
   - Use emoji prefixes
   - Keep first line under 50 chars
   - Add detailed description if needed

3. **Testing**
   ```bash
   cargo test --workspace
   cargo test --workspace --features test-utils
   ```

4. **Code Style**
   - Run before committing:
     ```bash
     cargo fmt
     cargo clippy --workspace --all-features
     ```

## Release Process 📦

1. Update CHANGELOG.md
2. Update version in Cargo.toml
3. Create git tag
4. Push to crates.io

## Need Help? 🤝

- Open an issue for bugs
- Start a discussion for features
- Join our Discord community

## Documentation and Cross-Referencing Policy

- The memory bank files provide concise summaries and always point to the canonical, detailed documentation in the `docs/` folder.
- Milestone plans are found in [docs/v0.2.x.md](v0.2.x.md), [docs/v0.3.x.md](v0.3.x.md), and [docs/v0.4.x.md](v0.4.x.md).
- The canonical project status and task list are in [docs/ROADMAP.md](ROADMAP.md) and [docs/TODO.md](TODO.md).
- When updating features, tasks, or technical plans, always update both the memory bank and the relevant canonical doc, and ensure all cross-references are correct and up to date.

Thank you for contributing! 🎉
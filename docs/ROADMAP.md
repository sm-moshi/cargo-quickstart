# Roadmap – cargo-quickstart

This roadmap outlines the phased evolution of `cargo-quickstart` from a foundational CLI tool into a fully-featured, interactive, and extensible Rust project generator.

⸻

## ✅ MVP Goals (v0.1.x) — Complete

**Focus:** Build the foundation of the CLI-based scaffolding tool.

-   [x] Implement core CLI with `clap`:
    -   [x] Root command setup and versioning.
    -   [x] Support flags: `--bin`, `--lib`, `--edition`, `--license`, `--git`, `--path`, `--yes`, `--interactive`.
-   [x] Implement interactive prompts with `dialoguer`/`inquire`:
    -   [x] Project name prompt with validation
    -   [x] Project type selection
    -   [x] Configuration confirmation
-   [x] Build minimal and extended project template structures:
    -   [x] Create `.vscode/`, `.cargo/`, `.editorconfig`, `.gitignore` templates.
    -   [x] Setup documentation stubs (README.md, CHANGELOG.md, CONTRIBUTING.md, etc.) for all generated projects.
-   [x] Create template interpolation system:
    -   [x] Support dynamic values: `{crate_name}`, `{author}`, `{year}`, `{license}`, `{edition}`.
-   [x] Implement shell completions for all major shells
-   [x] Polish CLI output with console for a user-friendly UX
-   [x] All lints and tests pass
-   [x] Add Miri compatibility for all tests
    -   [x] Modify filesystem operations with `if cfg!(miri)` conditionals
    -   [x] Fix error handling in test helpers
    -   [x] Adjust time-based operations in template handling
-   [x] Establish benchmarking infrastructure
    -   [x] Add criterion-based benchmarks with pprof integration
    -   [x] Create performance benchmark commands
    -   [x] Identify and begin optimizing template rendering bottlenecks

⸻

**Template system MVP is complete, robust, and fully tested. Project generation logic is template-driven and production-ready. Shell completions and CLI polish are complete.**

⸻

## 🔄 Near-Term Goals (v0.2.x) — In Progress

**Focus:** Improve interactivity, add structured configuration, feature flags, and multiple UX modes.

### 🎛️ Interactive Mode Enhancements

**Directory: `crates/quickstart-cli/src/mode/`**

- [ ] Add `wizard.rs` for prompt-based UX (default mode, uses `inquire`)
- [ ] Add `manual.rs` to parse all configuration via CLI args (`--manual`)
- [ ] Add `tui.rs` for fullscreen interactive mode (`--interactive`, uses `ratatui`)

**CLI Command Routing: `main.rs`**

```rust
match args {
    Args { manual: true, .. } => mode::manual::run(args),
    Args { interactive: true, .. } => mode::tui::run(args),
    _ => mode::wizard::run(args),
}
```

### 🧠 Shared Configuration Model

- Create `QuickstartConfig` struct in `crates/quickstart-lib/src/config.rs`
- Used by all UX modes to drive scaffolding
- Enables switching between wizard/manual/TUI without logic duplication

### 🧩 Feature Flag Support

**Cargo.toml (root-level):**

```toml
[features]
default = ["wizard"]
wizard = ["inquire"]
tui = ["ratatui", "crossterm", "tui-input"]
manual = []

with-ui = ["console", "indicatif"]
with-color-eyre = ["color-eyre"]
with-eyre = ["eyre"]
with-time = ["chrono"]
with-test-utils = ["mockall", "assert_cmd", "tempfile"]
with-benchmarks = ["criterion", "pprof"]
```

- Use `#[cfg(feature = "...")]` inside `ui.rs`, `main.rs`, and `mode/*.rs`

### 🧰 Macros

**File: `src/macros.rs`**

- `cli_try!`: Simplify error mapping to `CliError`
- `prompt_text!`: Wraps `inquire::Text` creation
- `wrap_suggest!`: Adds `.suggest([...])` and `.wrap_err(...)` for rich error context

### Already Completed in v0.1.x

-   [x] Doctor command (project/environment diagnostics)
-   [x] Path validation for safe directory operations
-   [x] CI/CD improvements:
    -   [x] Specialized Rust caching with Leafwing-Studios/cargo-cache
    -   [x] Modern toolchain installation with crusty-pie/toolchain
-   [x] Testing and validation improvements:
    -   [x] Make tests compatible with Miri for memory safety validation
    -   [x] Establish benchmarking infrastructure for performance profiling
        -   [x] Criterion-based benchmarks with pprof integration
        -   [x] Performance testing commands
        -   [x] Template rendering bottleneck identification
-   [x] Performance optimization foundation:
    -   [x] Add criterion-based benchmarks with pprof integration
    -   [x] Begin optimizing template engine operations

### Currently in Progress

-   [ ] Template engine optimizations (caching, helpers, etc.)
-   [ ] Filesystem operations optimization for better efficiency
-   [ ] Template info/discovery expansion
-   [ ] Remote/custom template support

⸻

## 🏗 Mid-Term Goals (v0.3.x) — Architecture Refinement

**Focus:** Architecture clean-up, plugin scaffolding, and performance measurement.

### 📦 Internal Refactoring

- Fully adopt `QuickstartConfig` in `quickstart-cli/src/mode/wizard.rs`, `quickstart-cli/src/mode/manual.rs`, and `quickstart-cli/src/mode/tui.rs`
- Move error definitions to `crates/quickstart-lib/src/errors.rs`
- Isolate CLI code in `cli/`, test helpers in `tests/utils.rs`

### 🔌 Plugin System

- Define trait `QuickstartPlugin` in `quickstart-lib/src/plugin/plugin.rs` or `quickstart-lib/src/plugin/mod.rs`
- Allow user-defined plugins to extend config generation or post-processing
- Add feature flags to register built-in plugin types (e.g., `rustfmt`, `prettier`, etc.)

### 📈 Benchmarking and Flamegraph Support

- Add `benches/init_bench.rs` using `criterion`
- Enable `pprof` flamegraph generation via:

```rust
Criterion::default().with_profiler(PProfProfiler::new(100, Output::Flamegraph))
```

- Add instructions to view results in `docs/benchmarks.md`

### ⚙️ CI Improvements

- GitHub Actions workflows: `.github/workflows/ci.yml`
- Add matrix for:
    - MSRV + stable + nightly
    - `--all-features` + `--no-default-features`
- Automate checks: `just ci`

### Additional Improvements

- [ ] Introduce `[features]` gating to reduce release binary size
- [ ] Benchmark CLI start time and profile TUI + prompt UX for speed
- [ ] Add CI/CD configuration scaffolding:
    - [ ] GitHub Actions workflows for formatting, linting, testing
    - [ ] GitLab CI support
- [ ] Generate dynamic VSCode tasks and launch files based on scaffolded project
- [ ] Automatically detect MSRV and populate `Cargo.toml` accordingly

⸻

## 🚀 Future Goals (v0.4.x) — Extensibility & Developer Experience

**Focus:** Developer experience, extensibility, template system growth.

### 🧩 Template Presets

- Add CLI arg: `--preset cli|lib|web|wasm`
- In `preset.rs`, map each preset to config stubs
- Generate specialized templates: `templates/preset/cli/`, etc.

### 🌐 Remote Template Support

- Allow `--template <URL>` pointing to Git repos or registries
- Fetch into temp dir, validate layout, and use as local template

### 🔌 Plugin System Finalization

- Support dynamic plugins via JSON manifest or Rust trait implementations
- Expose `register_plugin()` function in `lib.rs`

### 📈 Optional Telemetry

- Add feature `with-telemetry`
- Prompt for opt-in on first run
- Collect mode usage, flags, and presets (anonymized)

⸻

## 🎯 Long-Term Vision (v1.0+)

**Focus:** Provide a complete, stable, and extensible Rust project generator for individuals and teams.

- [ ] Publish stable `v1.0.0` release adhering to SemVer
- [ ] Ensure full cross-platform compatibility:
    - [ ] Linux, macOS, Windows, WSL verified
- [ ] Add WASM project scaffolding:
    - [ ] Integrate with `wasm-pack` and `wasm-bindgen` ecosystems
- [ ] Build a community-contributed public template registry:
    - [ ] Allow custom templates for CLI/Web/WASM/Library projects
- [ ] Integrate advanced workflows:
    - [ ] `cargo xtask` templates
    - [ ] `cargo-smart-release` setup for automated publishing
- [ ] Metadata-driven template generation:
    - [ ] Leverage `[workspace.metadata]` for dynamic project configuration
- [ ] Remote template sources and auto-updating
- [ ] TUI mode parity with `crates-tui` UX quality
- [ ] Dynamic README / LICENSE generation via templates
- [ ] `--headless` + `--config` JSON/YAML support

⸻

## 🧪 Development & CI

**Focus:** Build and maintain high-quality development processes.

-   [x] Nextest for fast, parallel test runs
-   [ ] GitHub Actions CI with MSRV + all-features matrix
-   [ ] Benchmarks tracked over time (HTML + flamegraphs)
-   [ ] Format, lint, Clippy, and safety gates

⸻

## 🔍 Implementation Timeline

### v0.1.x — Foundation (Complete)
- Core CLI and template system
- Shell completions and colourized output
- Project scaffolding with binary and library support
- Path validation and error handling
- Doctor command for project diagnostics

### v0.2.x — Interactive Polish (In Progress)
- Three distinct UX modes
- Shared configuration structure
- Feature flags for conditional compilation
- CLI macros for error handling and prompts
- Performance optimizations

### v0.3.x — Architecture Refinement
- Internal refactoring
- Plugin system scaffolding
- Enhanced benchmarking
- Compile-time optimizations
- CI/CD improvements

### v0.4.x — Extensibility & DX
- Project presets
- Plugins and extensions
- Remote templates
- Opt-in telemetry
- Advanced project customization

### v1.0+ — Long-term Vision
- Full platform support
- Community template registry
- Workflow integration
- CI/CD templates
- Dynamic configuration

⸻

Let's create the most ergonomic, robust, and extensible way to bootstrap modern Rust projects! 🐹

# Roadmap – cargo-quickstart

This roadmap outlines the phased evolution of `cargo-quickstart` from a foundational CLI tool into a fully-featured, interactive, and extensible Rust project generator.

⸻

## ✅ MVP Goals (v0.1.x) — Complete

**Focus:** Build the foundation of the CLI-based scaffolding tool.

- [x] Implement core CLI with `clap`:
  - [x] Root command setup and versioning.
  - [x] Support flags: `--bin`, `--lib`, `--edition`, `--license`, `--git`, `--path`, `--yes`, `--interactive`.
- [x] Implement interactive prompts with `dialoguer`/`inquire`:
  - [x] Project name prompt with validation
  - [x] Project type selection
  - [x] Configuration confirmation
- [x] Build minimal and extended project template structures:
  - [x] Create `.vscode/`, `.cargo/`, `.editorconfig`, `.gitignore` templates.
  - [x] Setup documentation stubs (README.md, CHANGELOG.md, CONTRIBUTING.md, etc.) for all generated projects.
- [x] Create template interpolation system:
  - [x] Support dynamic values: `{crate_name}`, `{author}`, `{year}`, `{license}`, `{edition}`.
- [x] Implement shell completions for all major shells
- [x] Polish CLI output with console for a user-friendly UX
- [x] All lints and tests pass
- [x] Add Miri compatibility for all tests
  - [x] Modify filesystem operations with `if cfg!(miri)` conditionals
  - [x] Fix error handling in test helpers
  - [x] Adjust time-based operations in template handling
- [x] Establish benchmarking infrastructure
  - [x] Add criterion-based benchmarks with pprof integration
  - [x] Create performance benchmark commands
  - [x] Identify and begin optimizing template rendering bottlenecks

⸻

**Template system MVP is complete, robust, and fully tested. Project generation logic is template-driven and production-ready. Shell completions and CLI polish are complete.**

⸻

## 🔄 Near-Term Goals (v0.2.x) — In Progress

**Focus:** Improve interactivity, add structured configuration, feature flags, and multiple UX modes.

### 🎛️ Interactive Mode Enhancements

**Directory: `crates/quickstart-cli/src/mode/`**

- [x] Add `crates/quickstart-cli/src/mode/wizard.rs` for prompt-based UX (default mode, uses `inquire`) (stubbed)
- [x] Add `crates/quickstart-cli/src/mode/manual.rs` to parse all configuration via CLI args (`--manual`) (stubbed)
- [x] Use `crates/quickstart-tui/src/lib.rs` for fullscreen interactive mode (`--interactive`, uses `ratatui`) (stubbed)

**CLI Command Routing: `crates/quickstart-cli/src/main.rs`**

```rust
match args {
    Args { manual: true, .. } => mode::manual::run(args),
    Args { interactive: true, .. } => mode::tui::run(args),
    _ => mode::wizard::run(args),
}
```

### 🧠 Shared Configuration Model

- [x] `QuickstartConfig` is now the canonical configuration struct in `crates/quickstart-lib/src/config.rs` (as of 2025-05-08)
- [x] Used by all UX modes (wizard/manual/TUI) to drive scaffolding
- [x] Enables switching between wizard/manual/TUI without logic duplication
- [x] Fully replaces the legacy `ProjectConfig` (now removed)

> **Note:** As of 2025-05-08, all configuration handling is unified under `QuickstartConfig`, which is extensible and future-proof. This enables consistent, DRY, and maintainable config handling across all modes and features.

### 🧩 Feature Flag Support

**crates/quickstart-cli/`Cargo.toml` at project root:**

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

- Use `#[cfg(feature = "...")]` inside `ui.rs`, `crates/quickstart-cli/src/main.rs`, and `mode/*.rs`

### 🧰 Macros

**File: `src/macros.rs`**

- `cli_try!`: Simplify error mapping to `CliError`
- `prompt_text!`: Wraps `inquire::Text` creation
- `wrap_suggest!`: Adds `.suggest([...])` and `.wrap_err(...)` for rich error context

### Already Completed in v0.1.x

- [x] Doctor command (project/environment diagnostics)
- [x] Path validation for safe directory operations
- [x] CI/CD improvements:
  - [x] Specialized Rust caching with Leafwing-Studios/cargo-cache
  - [x] Modern toolchain installation with crusty-pie/toolchain
- [x] Testing and validation improvements:
  - [x] Make tests compatible with Miri for memory safety validation
  - [x] Establish benchmarking infrastructure for performance profiling
    - [x] Criterion-based benchmarks with pprof integration
    - [x] Performance testing commands
    - [x] Template rendering bottleneck identification
- [x] Performance optimization foundation:
  - [x] Add criterion-based benchmarks with pprof integration
  - [x] Begin optimizing template engine operations

### Currently in Progress

- [ ] Template engine optimizations (caching, helpers, etc.)
- [ ] Filesystem operations optimization for better efficiency
- [ ] Template info/discovery expansion
- [ ] Remote/custom template support
- [ ] Implement full logic for wizard/manual/TUI modes

> **Current Status:** Mode modules and config groundwork are now committed. Next: implement full mode logic and TUI integration.

### 🖥️ TUI Mode (ratatui + crossterm)

- Create new crate: `crates/quickstart-tui`
  - [ ] Type: Library crate with `crates/quickstart-tui/src/lib.rs`
  - [ ] Controlled via `with-tui` feature flag in `quickstart-cli`
  - [ ] Uses `ratatui` and `crossterm` with `default-features = false`
  - [ ] Depends on `quickstart-lib`
  - [ ] Added to `[workspace.members]` in `Cargo.toml` at project root

- Expose public entrypoint:
  - [ ] `pub fn launch_tui(args: &Args) -> Result<()>` in `crates/quickstart-tui/src/lib.rs`
  - [ ] Called directly from `crates/quickstart-cli/src/main.rs` if compiled with `--features tui` and `--interactive` is passed

- Internal file structure:
  - [ ] `crates/quickstart-tui/src/layout.rs` — UI rendering and layout components
  - [ ] `crates/quickstart-tui/src/events.rs` — Terminal input, event loop, exit signals
  - [ ] `crates/quickstart-tui/src/app_state.rs` — State machine for user choices and configuration
  - [ ] `crates/quickstart-tui/src/lib.rs` — Public entrypoint and coordination

- CLI integration (in `crates/quickstart-cli/src/main.rs`):
  - [ ] Add feature-gated conditional:

        ```rust
        #[cfg(feature = "tui")]
        Args { interactive: true, .. } => quickstart_tui::launch_tui(&args),
        ```

  - [ ] No `crates/quickstart-cli/src/mode/tui.rs` required — all logic is delegated

- Compilation strategy:
  - [ ] Compile only with `--features tui`

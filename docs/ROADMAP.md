# Roadmap – cargo-quickstart

This roadmap outlines evolving goals from foundational scaffolding to a full-featured project generator.

⸻

## MVP Goals (v0.1.x) ✅ (Complete)

**Focus:** Build a functional CLI-based Rust project scaffolder.

-   [x] Implement core CLI with `clap`:
    -   [x] Root command setup and versioning.
    -   [x] Support flags: `--bin`, `--lib`, `--edition`, `--license`, `--git`, `--path`, `--yes`, `--interactive`.
-   [x] Implement interactive prompts with `dialoguer`:
    -   [x] Project name prompt with validation
    -   [x] Project type selection
    -   [x] Configuration confirmation
-   [x] Build minimal and extended project template structures:
    -   [~] Create `.vscode/`, `.cargo/`, `.editorconfig`, `.gitignore` templates.
    -   [x] Setup documentation stubs (README.md, CHANGELOG.md, CONTRIBUTING.md, CODE_OF_CONDUCT.md, etc.) for all generated projects.
-   [x] Create template interpolation system:
    -   [x] Support dynamic values: `{crate_name}`, `{author}`, `{year}`, `{license}`, `{edition}`.
-   [x] Implement shell completions for all major shells
-   [x] Polish CLI output with owo-colors for a user-friendly UX
-   [x] All lints and tests pass (only known cargo-udeps false positives in the library crate)

---

**Template system MVP is complete, robust, and fully tested. Project generation logic is template-driven and production-ready. Shell completions and CLI polish are complete.**

⸻

## Near-Term Goals (v0.2.x) 🔄 (In Progress)

**Focus:** Improve flexibility, resilience, and interactivity.

-   [x] Doctor command (project/environment diagnostics)
-   [ ] Template info/discovery expansion
-   [ ] Remote/custom template support
-   [ ] Interactive mode
-   [ ] Config file support
-   [ ] Fix remaining linting issues:
    -   [ ] Add missing test dependencies
    -   [ ] Clean up unused imports in init module

⸻

## Mid-Term Goals (v0.3.x – v0.4.x)

**Focus:** Expand project scalability, CI/CD integration, and template variety.

-   [ ] Introduce plugin system:
    -   [ ] Allow extendable project templates or workflows.
-   [ ] Add CI/CD configuration scaffolding:
    -   [ ] GitHub Actions workflows for formatting, linting, testing.
    -   [ ] GitLab CI support.
-   [ ] Add official project presets:
    -   [ ] `--preset cli` for command-line tools.
    -   [ ] `--preset web` for Axum/Rocket web projects.
    -   [ ] `--preset lib` for API/library crates.
-   [ ] Generate dynamic VSCode tasks and launch files based on scaffolded project.
-   [ ] Automatically detect MSRV and populate `Cargo.toml` accordingly.

⸻

## Long-Term Vision (v1.0+)

**Focus:** Provide a complete, stable, and extensible Rust project generator for individuals and teams.

-   [ ] Publish stable `v1.0.0` release adhering to SemVer.
-   [ ] Ensure full cross-platform compatibility:
    -   [ ] Linux, macOS, Windows, WSL verified.
-   [ ] Add WASM project scaffolding:
    -   [ ] Integrate with `wasm-pack` and `wasm-bindgen` ecosystems.
-   [ ] Build a community-contributed public template registry:
    -   [ ] Allow custom templates for CLI/Web/WASM/Library projects.
-   [ ] Integrate advanced workflows:
    -   [ ] `cargo xtask` templates.
    -   [ ] `cargo-smart-release` setup for automated publishing.
-   [ ] Metadata-driven template generation:
    -   [ ] Leverage `[workspace.metadata]` for dynamic project configuration.

⸻

Let's create the fastest, cleanest, and most powerful way to bootstrap modern Rust projects! 🐹

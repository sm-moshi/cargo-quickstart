# Roadmap – cargo-quickstart

This roadmap outlines evolving goals from foundational scaffolding to a full-featured project generator.

⸻

## MVP Goals (v0.1.x)

**Focus:** Build a functional CLI-based Rust project scaffolder.

-   [ ] Implement core CLI with `clap`:
    -   [ ] Root command setup and versioning.
    -   [ ] Support flags: `--bin`, `--lib`, `--edition`, `--license`, `--git`, `--path`, `--yes`, `--interactive`.
-   [ ] Build minimal and extended project template structures:
    -   [ ] Create `.vscode/`, `.cargo/`, `.editorconfig`, `.gitignore` templates.
    -   [ ] Setup documentation stubs (README.md, CHANGELOG.md, etc.).
-   [ ] Create template interpolation system:
    -   [ ] Support dynamic values: `{crate_name}`, `{author}`, `{year}`, `{license}`, `{edition}`.
-   [ ] Implement Git integration:
    -   [ ] `git init` on project creation.
    -   [ ] Optionally create an initial commit.
-   [ ] Generate Justfile for automation tasks.
-   [ ] Provide developer tooling installer (`cargo install` helpers).

⸻

## Near-Term Goals (v0.2.x)

**Focus:** Improve flexibility, resilience, and interactivity.

-   [ ] Integrate templating engine (Handlebars or Tera):
    -   [ ] Enable dynamic template creation based on user input.
-   [ ] Offer choice of Minimal vs Extended templates during creation.
-   [ ] Implement `--interactive` wizard mode:
    -   [ ] Prompt for project name, type, license, edition.
-   [ ] Add remote template support:
    -   [ ] Allow `--from github:user/repo` or local template paths.
-   [ ] Improve project safety:
    -   [ ] Detect non-empty directories and confirm overwrites.
-   [ ] Implement self-update support via `cargo install-update`.

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

Let's create the fastest, cleanest, and most powerful way to bootstrap modern Rust projects!

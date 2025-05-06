# cargo-quickstart

A blazing fast and opinionated `cargo` subcommand to bootstrap modern Rust projects with confidence and speed.

⸻

## ✨ Features

-   🚀 Quickly scaffold `bin`, `lib`, or hybrid projects.
-   🛠️ Preconfigured best-practice templates: `.vscode/`, `.cargo/config.toml`, `.gitignore`.
-   📚 Full documentation scaffolding: README, CHANGELOG, CONTRIBUTING, CODE_OF_CONDUCT.
-   🧩 Flexible template system with variants (minimal/extended).
-   🛡️ Built-in Git integration and license generation (MIT, Apache-2.0).
-   🎯 Fast CI/CD-ready projects out of the box.
-   📋 Shell completions for Bash, Zsh, Fish, PowerShell, and Elvish.
-   🧰 Project health diagnostics with the `doctor` command.
-   📊 Comprehensive benchmarking infrastructure for performance optimization.

⸻

## 🚀 Quickstart

```bash
cargo quickstart my-awesome-project --bin --edition 2021 --git --license MIT
```

This will scaffold a full project with Git initialized, best practices set up, and documentation templates ready to go.

⸻

## 📚 Why cargo-quickstart?

While `cargo new` and `cargo-generate` are powerful, `cargo-quickstart` offers **fast, reproducible, and opinionated** project setups without manual boilerplate, ideal for teams and solo developers alike.

⸻

## 📦 Installation

Until published on crates.io:

```bash
git clone https://github.com/sm-moshi/cargo-quickstart
cd cargo-quickstart
cargo install --path crates/quickstart-cli
```

⸻

## 🛤 Roadmap

-   ✅ Working CLI, project generator, template system
-   ✅ VS Code integration with optimal settings
-   ✅ Documentation stubs: CHANGELOG, CONTRIBUTING, CODE_OF_CONDUCT
-   ✅ Doctor command for project health diagnostics
-   ✅ Shell completions for all major shells
-   🔜 Interactive mode, remote templates, template variants
-   🛠 Mid-term: Config file support, template info/discovery
-   🚀 Future: Remote/custom template support, smart-release

(See [ROADMAP.md](docs/ROADMAP.md) for full details.)

⸻

## 🤝 Contributing

Contributions, feature ideas, bug reports are warmly welcome! Have fun!
See [CONTRIBUTING.md](docs/CONTRIBUTING.md).

⸻

## 📄 License

Dual-licensed under MIT OR Apache-2.0.

## Template System Directory Structure

The template system uses a flexible, file-based approach for project scaffolding. Templates are stored in a `templates/` directory at the project root, with the following structure:

```text
/templates
  /base                # Common files for all projects (placed at project root)
    README.md.hbs
    CHANGELOG.md.hbs
    CONTRIBUTING.md.hbs
    CODE_OF_CONDUCT.md.hbs
    .gitignore.hbs
    Cargo.toml.hbs
    /.vscode/         # VS Code configuration templates
      settings.json.hbs
      extensions.json.hbs
      launch.json.hbs
      tasks.json.hbs
    /.cargo/
      config.toml.hbs
  /binary
    /minimal/src/main.rs.hbs
    /extended/src/main.rs.hbs
  /library
    /minimal/src/lib.rs.hbs
    /extended/src/lib.rs.hbs
```

- **Base templates**: Files in `base/` are always placed at the root of the generated project (e.g., `README.md`, not `base/README.md`).
- **Type/variant templates**: Files in `binary/` and `library/` are placed according to their subdirectory structure.
- **File extension**: Only `.hbs` files are treated as templates using Handlebars syntax for variable substitution.

### Template Variables

Templates use a consistent set of variables:
- `name`: Project name (used throughout all templates)
- `project.is_binary` / `project.is_library`: Conditional sections for project type
- `date.year`: Dynamic year generation for documentation

## Shell Completions 🐚

cargo-quickstart supports shell completions for Bash, Zsh, Fish, PowerShell, and Elvish. To generate completions, use the `completions` subcommand:

```
cargo quickstart completions <shell> [--output <path>]
```

- If `--output` is omitted, completions are printed to stdout.
- Example (Bash):
  ```sh
  cargo quickstart completions bash > /usr/local/etc/bash_completion.d/cargo-quickstart
  ```
- Example (Zsh):
  ```sh
  cargo quickstart completions zsh > "${fpath[1]}/_cargo-quickstart"
  ```
- Example (Fish):
  ```sh
  cargo quickstart completions fish | source
  ```

## VS Code Integration ⚙️

Generated projects include comprehensive VS Code configuration:

- **settings.json**: Rust-analyzer settings optimized for development
- **extensions.json**: Recommended extensions for Rust development
- **launch.json**: Debug configurations for binary applications and library tests
- **tasks.json**: Common cargo commands with problem matcher configurations

These configurations provide an optimal developer experience right out of the box.

## Doctor Command 🩺

Use the `doctor` command to check the health of your project:

```
cargo quickstart doctor
```

This analyzes your project structure, dependencies, and configuration to identify potential issues and provide recommendations for improvements.

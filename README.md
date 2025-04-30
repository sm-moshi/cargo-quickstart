# cargo-quickstart

A blazing fast and opinionated `cargo` subcommand to bootstrap modern Rust projects with confidence and speed.

⸻

## ✨ Features

-   🚀 Quickly scaffold `bin`, `lib`, or hybrid projects.
-   🛠️ Preconfigured best-practice templates: `.vscode/`, `.cargo/`, `.editorconfig`, `.gitignore`.
-   📚 Full documentation scaffolding: README, CHANGELOG, CONTRIBUTING, ROADMAP, TODO.
-   🔧 Dev-tools installer: cargo-nextest, cargo-tarpaulin, cargo-release, sccache, etc.
-   🧩 Extendable templates, presets (CLI/Web/Library), and future plugin system.
-   🛡️ Built-in Git integration and license generation (MIT, Apache-2.0).
-   🎯 Fast CI/CD-ready projects out of the box.

⸻

## 🚀 Quickstart

```bash
cargo quickstart my-awesome-project --bin --edition 2021 --git --license MIT
```

This will scaffold a full project with Git initialized, best practices set up, and developer tools ready to install.

⸻

## 📚 Why cargo-quickstart?

While `cargo new` and `cargo-generate` are powerful, `cargo-quickstart` aims to offer **fast, reproducible, and opinionated** project setups without manual boilerplate, ideal for teams and solo developers alike.

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

-   ✅ MVP: Working CLI, project generator, dev tools installer
-   🔜 Interactive mode, remote templates, template variants
-   🛠 Mid-term: Plugin system, CI/CD generation, Presets
-   🚀 Future: WASM support, smart-release, public template registry

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
    Cargo.toml.hbs
    .gitignore.hbs
  /binary
    /minimal/src/main.rs.hbs
    /extended/src/main.rs.hbs
  /library
    /minimal/src/lib.rs.hbs
    /extended/src/lib.rs.hbs
```

- **Base templates**: Files in `base/` are always placed at the root of the generated project (e.g., `README.md`, not `base/README.md`).
- **Type/variant templates**: Files in `binary/` and `library/` are placed according to their subdirectory structure.
- **File extension**: Only `.hbs` files are treated as templates.

### Template Loader Path Resolution
- The loader searches upwards from the current directory for the nearest `templates/` directory.
- This ensures both CLI and tests can always find the correct templates, regardless of working directory.

### Contributing Templates
- Add new templates to the appropriate subdirectory.
- Use Handlebars syntax (`{{variable}}`) for variable substitution.
- See `crates/quickstart-lib/src/template/variables.rs` for available variables.

## Shell Completions 🐚

cargo-quickstart supports shell completions for Bash, Zsh, Fish, Powershell, and Elvish. To generate completions, use the `completions` subcommand:

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

Completions are also available for Powershell and Elvish. See `cargo quickstart completions --help` for details.

## Colourful, User-Friendly Output 🎨

All CLI output is colourised and formatted for clarity using [owo-colors](https://docs.rs/owo-colors) and consistent output patterns. Success, info, warning, and error messages are visually distinct for a polished UX.

- Success: green ✓
- Info: blue ℹ
- Warning: yellow ⚠
- Error: red ✗

Project configuration and template listings are also formatted for readability.

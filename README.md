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

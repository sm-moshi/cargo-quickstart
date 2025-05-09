# 🧰 Custom Macros Guide for `cargo-quickstart`

This guide documents the internal macros used to reduce boilerplate in error handling, user prompts, and diagnostics.

---

## 📋 Implementation Recommendation

These UI and error handling macros should be implemented in the **quickstart-cli** crate rather than in a separate crate or in the library:

### ✅ Benefits of CLI-focused approach

- Keeps UI/prompt dependencies (inquire, color-eyre) only in the CLI crate
- Maintains cleaner separation of concerns (lib for core logic, CLI for UI/UX)
- Better aligns with the nature of these macros which serve CLI-specific needs
- Avoids adding unnecessary dependencies to the library crate

### 🔄 When to reconsider

Consider extracting to a separate `quickstart-macros` crate only if:

- The macro collection grows significantly (>5-6 macros)
- You develop macros that serve core functionality needed by both lib and CLI
- The macros could be independently useful to other projects

---

## 📦 File Location

Place all macros in `src/macros.rs` or a dedicated module like `crate::util::macros`.

Use the macros by either:

- Using the 2015-style: `#[macro_use] mod macros;`
- Or importing directly: `use crate::macros::*;`

---

## 🔧 1. `cli_try!` Macro

### ✅ Purpose

Simplifies mapping errors into custom `CliError` variants.

### 🧪 Example

```rust
cli_try!(get_project_name(&args), ProjectConfig)?;
```

### 🔧 Definition

```rust
#[macro_export]
macro_rules! cli_try {
    ($expr:expr, $variant:ident) => {
        $expr.map_err(|e| CliError::$variant(e.to_string()))
    };
}
```

---

## 🎤 2. `prompt_text!` Macro

### ✅ Purpose

Simplifies prompt creation using `inquire::Text` with a label and help message.

### 🧪 Example

```rust
let prompt = prompt_text!("Project name", "Enter your project name");
```

### 🔧 Definition

```rust
#[macro_export]
macro_rules! prompt_text {
    ($label:expr, $help:expr) => {
        inquire::Text::new($label).with_help_message($help)
    };
}
```

---

## 🧠 3. `wrap_suggest!` Macro

### ✅ Purpose

Chains an error context and CLI suggestion for `color-eyre` Reports.

### 🧪 Example

```rust
wrap_suggest!(
    std::fs::read_to_string("Cargo.toml"),
    "failed to read manifest",
    ["Check if the file exists", "Verify permissions"]
)?;
```

### 🔧 Definition

```rust
#[macro_export]
macro_rules! wrap_suggest {
    ($res:expr, $msg:expr, [$($sugg:expr),*]) => {
        $res.wrap_err($msg).suggest(&[$($sugg),*])
    };
}
```

> 💡 Use this only when `color-eyre` is enabled via `#[cfg(feature = "with-color-eyre")]`

---

## 📚 Usage Notes

- Always import your macros where used or use `#[macro_use]`
- Consider conditional compilation with `#[cfg(feature = "...")]` if the macros depend on optional crates

---

**Maintained by:** `sm-moshi`
**License:** MIT OR Apache-2.0
---

## 📐 Macro Scope and the TUI Crate

All currently defined macros (`cli_try!`, `prompt_text!`, `wrap_suggest!`, `suggest_diag!`) live in the CLI crate:

- Path: `crates/quickstart-cli/src/macros.rs`

These macros are tightly integrated with prompt-driven or diagnostic workflows inside `crates/quickstart-cli`.

### TUI-Specific Consideration

The TUI crate (`crates/quickstart-tui`) does **not** use these CLI macros, and they are not imported there. If future TUI interaction patterns emerge (e.g., widget templates, layout helpers, color bindings), they should be introduced via a dedicated:

- `crates/quickstart-tui/src/tui_macros.rs` module (to be created only if needed)

This preserves a clean separation of concerns and keeps each crate’s macro layer focused on its domain (CLI ergonomics vs. TUI composition).

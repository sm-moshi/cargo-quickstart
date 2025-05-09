# 📦 Crate Usage Guide – `cargo-quickstart`

This guide documents all crates used (or considered) in the `cargo-quickstart` project. It covers **why each crate is used**, **which features are enabled**, **where it's used (CLI vs. lib)**, **what alternatives exist**, and whether `default-features = false` should be applied to optimize for performance and binary size.

It is intended for maintainers, contributors, and advanced users reviewing dependency choices and optimizing build performance.

---

## 🧑‍💻 CLI & UX Crates (CLI only)

### 1. `clap`

- **Purpose:** Used for command-line argument parsing, subcommand dispatch, and help output.
- **Features:** `"derive"`, `"suggestions"`, `"wrap_help"`
- **Location:** `quickstart-cli`
- **Alternatives:** `argh` (minimal), `bpaf` (declarative), `pico-args` (micro CLI)
- **Performance:** Medium compile time due to proc-macro derivations
- **Recommendation:** ✅ Use `default-features = false` to avoid heavy unused defaults.

### 2. `clap_complete`

- **Purpose:** Enables shell completions for bash, zsh, fish, etc.
- **Location:** CLI only
- **Alternatives:** None necessary (fits well with `clap`)
- **Performance:** Tiny; only used at CLI generation time
- **Recommendation:** ✅ OK as-is. Defaults are minimal.

### 3. `inquire`

- **Purpose:** Powers interactive prompts in Wizard and TUI modes (e.g., `Text`, `Select`, `Confirm`)
- **Features:** `"crossterm"` for terminal backend support
- **Location:** CLI only
- **Alternatives:** `dialoguer` (heavier), `promptly` (basic)
- **Performance:** Light and ergonomic
- **Recommendation:** ✅ Use `default-features = false` with `"crossterm"` explicitly enabled.

### 4. `console`

- **Purpose:** Terminal styling (colors, bold text, hiding cursor)
- **Location:** CLI only
- **Alternatives:** `termcolor`, `yansi`
- **Performance:** Small but could transitively enable more features
- **Recommendation:** ✅ Consider setting `default-features = false` to minimize footprint.

### 5. `indicatif`

- **Purpose:** Progress bars and spinners for visual feedback during project creation.
- **Features:** `"improved_unicode"`
- **Location:** CLI only
- **Alternatives:** `tinybar`, `console`
- **Recommendation:** ✅ Already optimized with `default-features = false`.

---

## 🧰 Error Handling & Reporting

### 6. `color-eyre`

- **Purpose:** Developer-friendly error reports with backtraces and contextual suggestions.
- **Features:** `"track-caller"`
- **Location:** CLI & Library when `with-color-eyre` is enabled
- **Alternatives:** `eyre`, `anyhow`, `miette`
- **Performance:** Compile time ~medium; runtime cost is negligible
- **Recommendation:** ✅ Use `default-features = false` and gate behind a feature flag.

### 7. `eyre`

- **Purpose:** Lightweight error handling fallback for minimal builds.
- **Location:** Library or CLI (via `with-eyre`)
- **Recommendation:** ✅ Use `default-features = false` and conditionally compile with feature.

---

## 🧬 Template Rendering & Serialization

### 8. `handlebars`

- **Purpose:** Used for rendering template files with user/project config context.
- **Location:** `quickstart-lib`
- **Alternatives:** `tera` (similar API), `minijinja` (fast), `askama` (compile-time)
- **Performance:** Medium–high for dynamic template rendering
- **Recommendation:** ✅ Use `default-features = false` for tight control over features.

### 9. `serde`

- **Purpose:** Serialize structs into `serde_json::Value` for use in templates and config generation.
- **Features:** `"derive"`
- **Location:** Library
- **Alternatives:** `miniserde` (only for ultra-small needs)
- **Recommendation:** ✅ `features = ["derive"]` only — ideal config.

---

## 🕒 Time Handling

### 10. `chrono`

- **Purpose:** Add timestamps to README or metadata headers.
- **Location:** Library
- **Alternatives:** `time`, `humantime`, `chrono-tz`
- **Performance:** Medium (due to `time` and `num-traits` transitive deps)
- **Recommendation:** ✅ Use `default-features = false`; offer switch to `time` via `with-time`.

### 11. `time` (optional)

- **Purpose:** Modern, faster alternative to `chrono`.
- **Recommendation:** Consider as an opt-in behind `with-time`.

---

## 🧩 System Utilities

### 12. `pathdiff`

- **Purpose:** Compute relative paths for template output paths.
- **Location:** Library
- **Performance:** Negligible
- **Recommendation:** ✅ No action needed.

### 13. `which`

- **Purpose:** Detect `cargo`, `git`, and other CLI tool presence.
- **Features:** `"tracing"`
- **Location:** Library (used by doctor checks)
- **Recommendation:** ✅ Use `default-features = false`.

---

## 🆔 Meta & IDs

### 14. `uuid`

- **Purpose:** Generates unique IDs (e.g. for benchmarking runs)
- **Features:** `"v4"`
- **Location:** Benchmarks, maybe diagnostics
- **Recommendation:** ✅ Use `default-features = false`.

---

## 🧪 Test Support (Dev-only)

### 15. `mockall`

- **Purpose:** Mock user prompts and commands in tests.
- **Location:** CLI test helpers
- **Alternatives:** `double`, `mockito`, `mockers`
- **Recommendation:** ✅ Add `default-features = false`.

### 16. `assert_cmd`

- **Purpose:** Assert CLI success/failure in integration tests.
- **Features:** `"color"`
- **Recommendation:** ✅ Use `default-features = false`.

### 17. `pretty_assertions`

- **Purpose:** Better diffs in `assert_eq!` during testing.
- **Recommendation:** ✅ Dev-only; fine as-is.

### 18. `tempfile`

- **Purpose:** Create temp project dirs for testing CLI output.
- **Alternatives:** `tempdir`
- **Recommendation:** ✅ Add `default-features = false`.

---

## 📊 Benchmarking (Dev-only)

### 19. `criterion`

- **Purpose:** Benchmark key stages of config/template rendering.
- **Features:** `"html_reports"`
- **Location:** Dev/benches only
- **Recommendation:** ✅ Use `default-features = false`.

### 20. `pprof`

- **Purpose:** Flamegraph generation during profiling
- **Features:** `"criterion"`, `"flamegraph"`
- **Recommendation:** ✅ Use `default-features = false`.

---

## ✅ Summary

### Crates to Update

Add `default-features = false` to:

- `mockall` ✅
- `console` ✅
- `tempfile` ✅

### Already Optimized

You already use `default-features = false` for:

- `clap`, `chrono`, `indicatif`, `uuid`, `pprof`, `criterion`, `color-eyre`, `handlebars`, `eyre`, `inquire`, `serde`

This guide should be placed at `docs/crate-usage.md` and linked from `README.md`, `ROADMAP.md`, and contributor onboarding docs
---

## 📦 Internal Crate — `crates/quickstart-tui`

The `quickstart-tui` crate encapsulates all logic for fullscreen TUI rendering. It is designed to be modular, feature-gated, and compile-isolated from the CLI unless `--features tui` is explicitly passed.

- **Purpose**: Implements the interactive `--interactive` UX mode using `ratatui` and `crossterm`
- **Path**: `crates/quickstart-tui`
- **Entrypoint**: `crates/quickstart-tui/src/lib.rs`
- **Dispatch**: Called from `crates/quickstart-cli/src/main.rs` via:

  ```rust
  #[cfg(feature = "tui")]
  Args { interactive: true, .. } => quickstart_tui::launch_tui(&args),
  ```

- **Dependencies**:
  - [`ratatui`](https://github.com/ratatui-org/ratatui) — for layout/rendering (`default-features = false`)
  - [`crossterm`](https://github.com/crossterm-rs/crossterm) — for terminal backend (`default-features = false`, uses `event-stream`, `cursor`, `terminal`, `style`)
  - [`color-eyre`] (optional) — gated by `with-color-eyre` feature
- **Feature Gate**: Enabled only when `tui` is passed to `crates/quickstart-cli/Cargo.toml`:

  ```toml
  [features]
  tui = ["quickstart-tui"]
  ```

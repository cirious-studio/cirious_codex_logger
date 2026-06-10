<div align="center">

# 📜 Cirious Codex Logger

**Structured Logging & Tracing for the Cirious Ecosystem**

[![CI](https://github.com/cirious-studio/cirious_codex_logger/actions/workflows/ci.yml/badge.svg)](https://github.com/cirious-studio/cirious_codex_logger/actions/workflows/ci.yml) [![Crates.io](https://img.shields.io/crates/v/cirious_codex_logger.svg)](https://crates.io/crates/cirious_codex_logger) [![Docs.rs](https://docs.rs/cirious_codex_logger/badge.svg)](https://docs.rs/cirious_codex_logger) [![Language](https://img.shields.io/badge/Language-Rust-black?logo=rust)](https://www.rust-lang.org/) [![License](https://img.shields.io/badge/License-MIT%2FApache-blue.svg)](#-license)

</div>

---

## 📖 Overview

**Cirious Codex Logger** is a highly optimized, foundational library for structured logging and event tracing. It provides efficient mechanisms for capturing, formatting, and dispatching application events with rich metadata. 

Designed to be the ultimate observability bedrock for tools and applications within the Cirious ecosystem, prioritizing performance, structural integrity, and seamless terminal integration.

## 🚀 Quick Start
 
Add the following to your `Cargo.toml`:

```toml
[dependencies]
cirious_codex_logger = "0.2"
```

And then in your code:

```rust
use cirious_codex_logger::{info, init, StdoutDispatcher, StyledTerminalFormatter};

fn main() {
  let formatter = StyledTerminalFormatter;
  let dispatcher = Box::new(StdoutDispatcher::new(formatter));
  init(dispatcher).unwrap();
  info!("The logger is officially powered by Cirious Codex Term!");
  info!("Data: {}, Status: {}", 123, "Active");
}
```
---

## 🚧 Current Status & Roadmap

### ✅ v0.1.0 — Completed

- [x] **Core logging macros:** (`trace`, `debug`, `info`, `warn`, `error`).
- [x] **Extensible output formatters:** (JSON, human-readable terminal).
- [x] **Pluggable dispatchers:** (stdout, stderr, rolling files).
- [x] **Integration:** with `cirious_codex_term` for rich, native terminal styling.

### 🔭 v0.2.0 — Planned

- [x] Global Logger Registration (`OnceLock` integration for macros).
- [x] Context & Metadata Enrichment (`timestamp`, `module_path`, `file`, `line`).
- [x] Log Filtering (Level & Module-based filtering capabilities).
- [ ] Non-blocking / Async Dispatching (Background thread queueing).
- [ ] Rolling File Dispatcher (Size or Date-based file rotation).

---

## 📜 License

Licensed under either of the following, at your option:

* **[MIT License](LICENSE-MIT)**
* **[Apache License 2.0](LICENSE-APACHE)**

---

<div align="center">
  <i>Minimalist by design. Consistent in execution.</i><br>
  <sub>Engineered by Cirious Studio</sub>
</div>

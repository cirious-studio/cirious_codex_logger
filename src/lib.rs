//! ## Initialization
//!
//! To begin logging, initialize the system with your preferred dispatcher:
//!
//! ```rust
//! use cirious_codex_logger::{init, StdoutDispatcher, StyledTerminalFormatter};
//!
//! let formatter = StyledTerminalFormatter;
//! let dispatcher = Box::new(StdoutDispatcher::new(formatter));
//!
//! init(dispatcher).expect("Logger already initialized");
//! ```

// Enables docs.rs features to show tags like "Only on Windows"
#![cfg_attr(docsrs, feature(doc_cfg))]
// Ensures all public items are documented (Essential for crates.io)
#![warn(missing_docs)]
// Prevents accidental unsafe code in the entire crate
#![deny(unsafe_code)]

/// Routing and dispatching mechanisms for log records.
///
/// Includes implementations for standard out and standard error streams.
pub mod dispatcher;

/// Utilities for structuring and serializing log events.
///
/// Provides formatters for both human-readable terminal output and structured JSON.
pub mod format;

/// Definitions for logging verbosity levels.
///
/// Controls the severity and filtering of the emitted log records.
pub mod level;

/// The core logging macros and global logging entry points.
///
/// This module provides the essential `info!`, `error!`, `warn!`, `debug!`, and `trace!` macros.
pub mod log;

/// Terminal styling and colorization utilities.
///
/// Integrates with the Cirious ecosystem to provide visually distinct and rich terminal outputs.
pub mod style;

pub use dispatcher::{AsyncDispatcher, Dispatcher, RollingFileDispatcher, StderrDispatcher, StdoutDispatcher};
pub use format::{Formatter, HumanReadableFormatter, JsonFormatter, Record};
pub use level::Level;
pub use log::*;
pub use style::{level_color, StyledTerminalFormatter};

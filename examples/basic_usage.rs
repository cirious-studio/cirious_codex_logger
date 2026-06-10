//! # Cirious Codex Logger — Basic Usage Example
//!
//! This example demonstrates how to initialize the logger and dispatch
//! styled logs to the standard output.

use cirious_codex_logger::{info, init, StdoutDispatcher, StyledTerminalFormatter};

fn main() {
  // 1. Instantiate the formatter that leverages cirious_codex_term for ANSI styling.
  let formatter = StyledTerminalFormatter;

  // 2. Create the Dispatcher.
  // The Dispatcher is responsible for routing the formatted records to the destination.
  let dispatcher = Box::new(StdoutDispatcher::new(formatter));

  // 3. Initialize the global logger.
  // This call automatically triggers terminal setup (ANSI support on Windows,
  // NO_COLOR check, and TTY detection).
  init(dispatcher).expect("Failed to initialize the global logger");

  // 4. Log events.
  // The macro captures module path, file, and line metadata automatically.
  info!("The logger is officially powered by Cirious Codex Term!");
  info!("Data: {}, Status: {}", 123, "Active");
}

//! Example demonstrating basic synchronous logging to standard output.
//!
//! This example shows the simplest way to initialize the logger and
//! emit events using the standard `StdoutDispatcher`.

use cirious_codex_logger::{error, info, init, StdoutDispatcher, StyledTerminalFormatter};

fn main() {
  let formatter = StyledTerminalFormatter;
  let dispatcher = Box::new(StdoutDispatcher::new(formatter));

  init(dispatcher).ok();

  info!("Starting the application...");
  error!("An example error occurred.");
}

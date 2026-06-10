//! Example demonstrating non-blocking asynchronous logging.
//!
//! This example wraps a standard dispatcher in an `AsyncDispatcher`,
//! ensuring that logging operations do not block the execution of your
//! main application threads.

use cirious_codex_logger::{init, warn, AsyncDispatcher, StdoutDispatcher, StyledTerminalFormatter};

fn main() {
  let stdout = Box::new(StdoutDispatcher::new(StyledTerminalFormatter));
  let async_dispatcher = Box::new(AsyncDispatcher::new(stdout, 100));

  init(async_dispatcher).ok();

  warn!("This log is processed in a background thread without blocking.");
}

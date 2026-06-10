//! Example demonstrating file-based logging with automatic rotation.
//!
//! This example configures a `RollingFileDispatcher` that will rotate the
//! log file once it exceeds 1MB in size, keeping the system clean.

use cirious_codex_logger::{info, init, HumanReadableFormatter, RollingFileDispatcher};

fn main() {
  let formatter = HumanReadableFormatter;
  let path = "app.log";
  let max_size = 1024 * 1024;

  let dispatcher = RollingFileDispatcher::new(path, max_size, formatter).ok();

  if let Some(dispatcher) = dispatcher {
    let dispatcher = Box::new(dispatcher);
    init(dispatcher).ok();
  }

  for i in 1..=5 {
    info!("Logging message number: {}", i);
  }

  println!("Check your project root for 'app.log' and 'app.log.old'");
}

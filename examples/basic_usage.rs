#![allow(missing_docs)]

use cirious_codex_logger::{debug, error, info, trace, warn};

fn main() {
  info!("Application started successfully.");
  debug!("Debugging an internal variable: {}", 42);
  warn!("Memory usage is high.");
  error!("Failed to connect to the database.");
  trace!("Tracing execution flow.");
}

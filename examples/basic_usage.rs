#![allow(missing_docs)]

use cirious_codex_logger::{info, init, StdoutDispatcher, StyledTerminalFormatter};

fn main() {
  let formatter = StyledTerminalFormatter;
  let dispatcher = Box::new(StdoutDispatcher::new(formatter));
  init(dispatcher).unwrap();
  info!("The logger is officially powered by Cirious Codex Term!");
  info!("Data: {}, Status: {}", 123, "Active");
}

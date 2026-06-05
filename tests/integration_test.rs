#![allow(missing_docs)]

use cirious_codex_logger::{Formatter, JsonFormatter, Level, Record};

#[test]
fn test_json_formatter() {
  let args = format_args!("Hello World");
  let record = Record {
    level: Level::Info,
    args,
  };

  let formatter = JsonFormatter;
  let result = formatter.format(&record);

  assert_eq!(result, r#"{"level":"Info","message":"Hello World"}"#);
}

use cirious_codex_logger::{debug, error, info, trace, warn};

#[test]
fn test_macros_compilation_and_execution() {
  // This test ensures that invoking the macros doesn't cause panics
  // and that formatting variables works properly.
  info!("Testing info without args");
  debug!("Testing debug with an arg: {}", 42);
  warn!("Testing warn...");
  error!("Testing error with multiple args: {}, {}", "foo", "bar");
  trace!("Testing trace");
}

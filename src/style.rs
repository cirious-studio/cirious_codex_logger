// Adjust the import path according to your actual `cirious_codex_term` crate structure
use crate::{
  format::{Formatter, Record},
  Level,
};
use cirious_codex_term::Color;

/// Maps a `Level` to its corresponding terminal `Color`.
///
/// This helper ensures a consistent and recognizable color scheme across
/// the Cirious ecosystem:
/// - Error: Red
/// - Warn: Yellow
/// - Info: Blue
/// - Debug: Magenta
/// - Trace: Bright Black (Gray)
pub fn level_color(level: Level) -> Color {
  match level {
    Level::Error => Color::Red,
    Level::Warn => Color::Yellow,
    Level::Info => Color::Blue,
    Level::Debug => Color::Magenta,
    Level::Trace => Color::BrightBlack,
  }
}

/// A formatter that leverages `cirious_codex_term` for rich terminal styling.
///
/// This formatter wraps the log level in brackets, colors it according to the
/// severity, and applies bold text to make tags pop in standard terminal
/// environments.
#[derive(Debug)]
pub struct StyledTerminalFormatter;

/// Formats a `std::time::SystemTime` into a human-readable string.
fn format_timestamp(time: std::time::SystemTime) -> String {
  use std::time::UNIX_EPOCH;

  let duration = time.duration_since(UNIX_EPOCH).unwrap_or_default();
  let secs = duration.as_secs();

  let h = (secs / 3600) % 24;
  let m = (secs / 60) % 60;
  let s = secs % 60;

  format!("{:02}:{:02}:{:02}", h, m, s)
}

impl Formatter for StyledTerminalFormatter {
  fn format(&self, record: &Record) -> String {
    use cirious_codex_term::StyleExt;

    // 1. Prepare styling
    let color = level_color(record.level);
    let level_tag = format!("{:?}", record.level).to_uppercase();
    let styled_level = format!("[{}]", level_tag).bold().color(color);

    // 2. Prepare Timestamp (Simple format using system time)
    let now = std::time::SystemTime::now();

    let timestamp = format!("[{}]", format_timestamp(now).rgb(68, 68, 68).bold());

    // 3. Conditional Layout based on Level
    let base = format!("{} {}", timestamp, styled_level);

    if record.level == crate::Level::Trace {
      // Verbose layout: [Time] [Level] [File:Module:Line] Message
      let trace_location = format!("[{}:{}:{}]", record.file, record.module_path, record.line)
        .bold()
        .bright_black();

      format!("{} {} {}", base, trace_location, record.args)
    } else {
      // Standard layout: [Time] [Level] Message
      format!("{} {}", base, record.args)
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use cirious_codex_term::Color;

  #[test]
  fn test_level_color_mapping() {
    assert_eq!(level_color(Level::Error), Color::Red);
    assert_eq!(level_color(Level::Info), Color::Blue);
  }

  #[test]
  fn test_styled_terminal_formatter() {
    let args = format_args!("Styled message");
    let record = Record {
      level: Level::Warn,
      args: args.to_string(),
      file: "test",
      line: 1,
      module_path: "test",
      timestamp: std::time::SystemTime::now(),
    };

    let formatter = StyledTerminalFormatter;
    let result = formatter.format(&record);

    // Validating the presence of the basic strings.
    // For a more exact test, you would assert the exact ANSI string output.
    assert!(result.contains("Styled message"));
    assert!(result.contains("WARN"));
  }
}

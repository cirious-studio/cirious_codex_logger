// Adjust the import path according to your actual `cirious_codex_term` crate structure
use crate::{
  format::{Formatter, Record},
  Level,
};
use cirious_codex_term::{Color, Style};

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

impl Formatter for StyledTerminalFormatter {
  fn format(&self, record: &Record) -> String {
    let color = level_color(record.level);
    let bold_code = Style::Bold.to_str();
    let reset_code = Style::Reset.to_str();

    // Adjust `.to_str()` below to whatever method your `Color` enum uses
    // to return the ANSI foreground string.
    let color_code = color.to_fg_str();

    // Concatenate ANSI codes -> text -> Reset ANSI code
    let styled_tag = format!("{}{}[{:?}]{}", bold_code, color_code, record.level, reset_code);

    format!("{} {}", styled_tag, record.args)
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
      args,
    };

    let formatter = StyledTerminalFormatter;
    let result = formatter.format(&record);

    // Validating the presence of the basic strings.
    // For a more exact test, you would assert the exact ANSI string output.
    assert!(result.contains("Styled message"));
    assert!(result.contains("Warn"));
  }
}

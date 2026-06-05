// Adjust the import path according to your actual `cirious-codex-term` crate structure
use crate::{
  format::{Formatter, Record},
  Level,
};
use cirious_codex_term::{Color, Style};
// 1. Helper to map log levels to terminal colors
pub fn level_color(level: Level) -> Color {
  match level {
    Level::Error => Color::Red,
    Level::Warn => Color::Yellow,
    Level::Info => Color::Blue,
    Level::Debug => Color::Magenta,
    Level::Trace => Color::BrightBlack,
  }
}
// 2. Formatter that uses `cirious-codex-term` for rich styling
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

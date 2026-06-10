use crate::{
  format::{Formatter, Record},
  Level,
};
use cirious_codex_term::{Color, StyleExt};

/// Maps a `Level` to its corresponding terminal `Color`.
///
/// This provides the visual hierarchy for the terminal output:
/// - Error: Red
/// - Warn: Yellow
/// - Info: Blue
/// - Debug: Magenta
/// - Trace: Bright Black (Gray)
#[must_use]
pub const fn level_color(level: Level) -> Color {
  match level {
    Level::Error => Color::Red,
    Level::Warn => Color::Yellow,
    Level::Info => Color::Blue,
    Level::Debug => Color::Magenta,
    Level::Trace => Color::BrightBlack,
  }
}

/// A formatter that leverages `cirious_codex_term` for rich terminal styling.
#[derive(Debug, Default)]
pub struct StyledTerminalFormatter;

impl Formatter for StyledTerminalFormatter {
  /// Formats the record into an ANSI-styled string.
  ///
  /// The output includes a timestamp, a bold/colorized log level,
  /// and the message. For `Trace` logs, it includes additional
  /// source metadata (file, module, and line).
  fn format(&self, record: &Record) -> String {
    // 1. Level Styling
    let level_tag = format!("{:?}", record.level).to_uppercase();
    let styled_level = format!("[{level_tag}]").bold().color(level_color(record.level));

    // 2. Timestamp Styling
    let timestamp = format!("[{}]", format_timestamp(record.timestamp))
      .rgb(68, 68, 68)
      .bold();

    // 3. Layout Composition
    let base = format!("{timestamp} {styled_level}");

    if record.level == Level::Trace {
      let trace_loc = format!("[{}:{}:{}]", record.file, record.module_path, record.line)
        .bold()
        .bright_black();
      format!("{} {} {}", base, trace_loc, record.args)
    } else {
      format!("{} {}", base, record.args)
    }
  }
}

/// Formats `SystemTime` into a human-readable HH:MM:SS string.
fn format_timestamp(time: std::time::SystemTime) -> String {
  use std::time::UNIX_EPOCH;
  let duration = time.duration_since(UNIX_EPOCH).unwrap_or_default();
  let secs = duration.as_secs();
  format!("{:02}:{:02}:{:02}", (secs / 3600) % 24, (secs / 60) % 60, secs % 60)
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::Level;
  use std::time::SystemTime;

  #[test]
  fn test_level_color_mapping() {
    assert_eq!(level_color(Level::Error), Color::Red);
    assert_eq!(level_color(Level::Warn), Color::Yellow);
    assert_eq!(level_color(Level::Info), Color::Blue);
    assert_eq!(level_color(Level::Debug), Color::Magenta);
    assert_eq!(level_color(Level::Trace), Color::BrightBlack);
  }

  #[test]
  fn test_format_timestamp() {
    // Testa um timestamp fixo (UNIX_EPOCH é 00:00:00)
    let time = SystemTime::UNIX_EPOCH;
    assert_eq!(format_timestamp(time), "00:00:00");
  }

  #[test]
  fn test_styled_terminal_formatter_output() {
    let formatter = StyledTerminalFormatter;
    let record = Record {
      level: Level::Info,
      args: "Operação iniciada".to_string(),
      file: "main.rs",
      line: 10,
      module_path: "app::core",
      timestamp: SystemTime::UNIX_EPOCH,
    };

    let result = formatter.format(&record);

    // Verifica os elementos essenciais da formatação padrão
    assert!(result.contains("INFO")); // Nível
    assert!(result.contains("Operação iniciada")); // Mensagem
    assert!(result.contains("00:00:00")); // Timestamp
  }

  #[test]
  fn test_styled_terminal_formatter_trace_layout() {
    let formatter = StyledTerminalFormatter;
    let record = Record {
      level: Level::Trace,
      args: "Loop executando".to_string(),
      file: "loop.rs",
      line: 55,
      module_path: "app::util",
      timestamp: SystemTime::UNIX_EPOCH,
    };

    let result = formatter.format(&record);

    // Verifica se a formatação verbose (detalhes extras) está presente no TRACE
    assert!(result.contains("TRACE"));
    assert!(result.contains("loop.rs:app::util:55"));
  }
}

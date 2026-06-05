use crate::Level;

// 1. Event data structure
pub struct Record<'a> {
  pub level: Level,
  pub args: std::fmt::Arguments<'a>,
  // Future: pub target: &'a str, pub timestamp: SystemTime
}

// 2. Trait base for formatters
pub trait Formatter {
  fn format(&self, record: &Record) -> String;
}

// 3. Human-readable formatter
pub struct HumanReadableFormatter;
impl Formatter for HumanReadableFormatter {
  fn format(&self, record: &Record) -> String {
    format!("[{:?}] {}", record.level, record.args)
  }
}

// 4. JSON Formatter
pub struct JsonFormatter;
impl Formatter for JsonFormatter {
  fn format(&self, record: &Record) -> String {
    // Simple manual setup to avoid dependencies in the initial setup
    // In an advanced scenario, you can integrate `serde` or `serde_json`
    format!(r#"{{"level":"{:?}","message":"{}"}}"#, record.level, record.args)
  }
}

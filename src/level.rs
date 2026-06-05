#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Level {
  Error = 1,
  Warn,
  Info,
  Debug,
  Trace,
}

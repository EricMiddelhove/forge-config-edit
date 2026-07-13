use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum Error {
  LineNotParseable,
  UnknownLineType,
  MissingSubtreeStart(String),
  MalformedArrayHeader(String),
}

impl Display for Error {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      Error::LineNotParseable => write!(f, "line could not be parsed"),
      Error::UnknownLineType => write!(f, "unknown line type"),
      Error::MissingSubtreeStart(line) => write!(f, "expected '{{' to start subtree in line: {line}"),
      Error::MalformedArrayHeader(line) => write!(f, "malformed array header, expected format 'D:name <' in line: {line}"),
    }
  }
}

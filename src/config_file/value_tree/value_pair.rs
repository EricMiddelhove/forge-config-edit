use std::fmt::{Debug, Display, Formatter};
use crate::config_file::value_tree::error::Error;

static DATATYPE_NAME_SEPARATOR: char = ':';
static NAME_VALUE_SEPARATOR: char = '=';

#[derive(Debug, PartialEq)]
pub(crate) struct ValuePair {
  pub(crate) datatype: String,
  pub(crate) name: String,
  pub(crate) value: String,
}

impl ValuePair {
  pub(crate) fn try_new(line: &str) -> Result<Self, Error> {
    let datatype_separator = match line.chars().position(|c| c == DATATYPE_NAME_SEPARATOR) {
      None => return Err(Error::LineNotParseable),
      Some(pos) => pos
    };

    let name_value_separator = match line.chars().position(|c| c == NAME_VALUE_SEPARATOR) {
      None => return Err(Error::LineNotParseable),
      Some(pos) => pos
    };

    let datatype = line[0..datatype_separator].trim().to_string();
    let name = line[datatype_separator+1 .. name_value_separator].to_string();
    let value = line[name_value_separator+1 ..].to_string();

    Ok(ValuePair { datatype, name, value })
  }

  pub(crate) fn export(&self, s: &mut String, indent: usize) {
    let indent_string = " ".repeat(indent * 4);
    s.push_str(indent_string.as_str());
    s.push_str(format!("{}:{}={}", self.datatype, self.name, self.value).as_str());
    s.push('\n');
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_value_pair_returns_error_on_malformed_line() {
    let result = ValuePair::try_new("this is not a valid key-value pair");
    assert!(result.is_err());
  }
}

impl Display for ValuePair {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}:{}={}", self.datatype, self.name, self.value)
  }
}

use std::fmt::{Debug, Formatter};
use crate::forge_config::node::{Node};
use crate::forge_config::error::Error;
use std::str::Lines;

static DATATYPE_NAME_SEPARATOR: char = ':';
static NAME_VALUE_SEPARATOR: char = '=';

#[derive(Debug)]
pub(crate) struct ValuePair {
  datatype: String,
  name: String,
  comments: Vec<String>,
  value: String
}

impl ValuePair {
  pub(crate) fn try_new(line: &str, comments: Vec<String>) -> Result<Self, Error> {

    let datatype_separator = match line.chars().position(|c| c == DATATYPE_NAME_SEPARATOR) {
      None => return Err(Error::LineNotParseable),
      Some(pos) => pos
    };

    let name_value_separator = match line.chars().position(|c| c == NAME_VALUE_SEPARATOR) {
      None => return Err(Error::LineNotParseable),
      Some(pos) => pos
    };

    let datatype = line[0..datatype_separator].trim().to_string();

    let name = &line[datatype_separator+1 .. name_value_separator];
    let value = &line[name_value_separator +1 ..];

    Ok(ValuePair{
      datatype: datatype,
      name: name.into(),
      comments,
      value: value.into()
    })

  }
}

impl Node for ValuePair {
  fn name(&self) -> &str {
    &self.name
  }

  fn comments(&self) -> &[String] {
    &self.comments
  }

  fn export(&self, s: &mut String, indent: usize, skip_root: bool) {
    let indent_string = " ".repeat(indent);

    for comment in &self.comments {
      s.push_str(indent_string.as_str());
      s.push_str(comment.as_str());
      s.push('\n');
    }
    
    s.push_str(indent_string.as_str());

    s.push_str(format!("{}:{}={}", self.datatype, self.name, self.value).as_str());
    s.push('\n');
  }
}

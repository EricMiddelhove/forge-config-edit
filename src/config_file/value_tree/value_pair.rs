use std::fmt::{Debug, Display, Formatter};
use crate::config_file::value_tree::error::Error;
use crate::config_file::value_tree::node::Node;

static DATATYPE_NAME_SEPARATOR: char = ':';
static NAME_VALUE_SEPARATOR: char = '=';

#[derive(Debug, PartialEq)]
pub(crate) struct ValuePair {
  pub(crate) datatype: String,
  pub(crate) name: String,
  pub(crate) comments: Vec<String>,
  pub(crate) value: String
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

impl Display for ValuePair {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{}:{}={}", self.datatype, self.name,self.value)
  }
}
impl Node for ValuePair {
  fn name(&self) -> &str {
    &self.name
  }

  fn comments(&self) -> &[String] {
    self.comments.as_slice()
  }

  fn export(&self, s: &mut String, indent: usize, _skip_root: bool) {
    let indent_string = " ".repeat(indent);

    for comment in &self.comments {
      s.push_str(indent_string.as_str());
      s.push_str(comment.as_str());
      s.push('\n');
    }
    
    s.push_str(indent_string.as_str());

    s.push_str(format!("{}:{}={}", self.datatype, self.name, self.value).as_str());
    s.push('\n');
    s.push('\n');
  }
}

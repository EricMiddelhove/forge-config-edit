use crate::forge_config::error::Error;

pub enum LineTypes {
  KeyValuePair,
  TreeStart,
  TreeEnd,
  WhiteSpace,
  Comment,
}

impl TryFrom<&str> for LineTypes {
  type Error = Error;
  fn try_from(line: &str) -> Result<Self, Error> {
    let first_char = line.trim().chars().nth(0);

    let mut first_char = match first_char {
      Some(first_char) => first_char,
      None => return Ok(LineTypes::WhiteSpace)
    };
    
    if first_char == '"' {
      let next_char = line.trim().chars().nth(1);
      let next_char = match next_char {
        Some(first_char) => first_char,
        None => return Ok(LineTypes::WhiteSpace)
      };
      
      first_char = next_char
    }


    let line_type = if first_char.is_ascii_lowercase() {
      LineTypes::TreeStart
    } else if first_char.is_ascii_uppercase() {
      LineTypes::KeyValuePair
    } else if first_char == '#' {
      LineTypes::Comment
    } else if first_char == '}' {
      LineTypes::TreeEnd
    } else if first_char.is_whitespace() {
      LineTypes::WhiteSpace
    } else {
      return Err(Error::UnkownLineType)
    };

    Ok(line_type)
  }
}
#[derive(Debug)]
#[derive(PartialEq)]
pub enum LineTypes {
  KeyValuePair,
  TreeStart,
  TreeEnd,
  WhiteSpace,
  ArrayStart,
  ArrayEnd,
  Comment,
  Unknown,
}

impl From<&str> for LineTypes {

  fn from(line: &str) -> Self {
    let first_char = line.trim().chars().nth(0);

    let mut first_char = match first_char {
      Some(first_char) => first_char,
      None => return LineTypes::WhiteSpace
    };

    if first_char == '"' {
      let next_char = line.trim().chars().nth(1);
      let next_char = match next_char {
        Some(first_char) => first_char,
        None => return LineTypes::WhiteSpace
      };

      first_char = next_char
    }

    let line_type = if first_char.is_ascii_lowercase() {
      LineTypes::TreeStart
    } else if first_char.is_ascii_uppercase() {
      let last_char = line.trim().chars().last();
      let last_char = match last_char {
        Some(last_char) => last_char,
        _ => { ' ' }
      };

      if last_char == '<' {
        LineTypes::ArrayStart
      } else {
        LineTypes::KeyValuePair
      }
    } else if first_char == '#' {
      LineTypes::Comment
    } else if first_char == '}' {
      LineTypes::TreeEnd
    } else if first_char.is_whitespace() {
      LineTypes::WhiteSpace
    } else if first_char == '>' {
      LineTypes::ArrayEnd
    }else {
      LineTypes::Unknown
    };

    line_type
  }
}

#[cfg(test)]
mod tests {
  use crate::config_file::value_tree::line_type::LineTypes;

  #[test]
  fn test_try_from_for_comment_line() {
    let line_str = "# This is a comment\n";
    let expected = LineTypes::Comment;

    let line_type = LineTypes::try_from(line_str).unwrap();

    assert_eq!(expected, line_type);
  }

  #[test]
  fn test_try_from_for_tree_start_line() {
    let line_str = "sometree {";
    let expected = LineTypes::TreeStart;

    let line_type = LineTypes::try_from(line_str).unwrap();

    assert_eq!(expected, line_type);
  }
  #[test]
  fn test_try_from_for_tree_end_line() {
    let line_str = "}";
    let expected = LineTypes::TreeEnd;

    let line_type = LineTypes::try_from(line_str).unwrap();

    assert_eq!(expected, line_type);
  }

  #[test]
  fn test_try_from_for_whitespace_line() {
    let line_str = "   ";
    let expected = LineTypes::WhiteSpace;

    let line_type = LineTypes::try_from(line_str).unwrap();

    assert_eq!(expected, line_type);
  }

  #[test]
  fn test_try_from_for_whitespace_line_variant_2() {
    let line_str = "";
    let expected = LineTypes::WhiteSpace;

    let line_type = LineTypes::try_from(line_str).unwrap();

    assert_eq!(expected, line_type);
  }

  #[test]
  fn test_try_from_for_keyvalue_pair_line() {
    let line_str = "S:key=value";
    let expected = LineTypes::KeyValuePair;

    let line_type = LineTypes::try_from(line_str).unwrap();

    assert_eq!(expected, line_type);
  }

  #[test]
  fn test_try_from_for_line_beginning_with_quotation() {
    let line_str = "\"some tree\" {";
    let expected = LineTypes::TreeStart;

    let line_type = LineTypes::try_from(line_str).unwrap();
    assert_eq!(expected, line_type);

  }

  #[test]
  fn test_try_from_for_line_for_array_start() {
    let line_str = "S:times <";
    let expected = LineTypes::ArrayStart;

    let line_type = LineTypes::try_from(line_str).unwrap();
    assert_eq!(expected, line_type);
  }

  #[test]
  fn test_try_from_for_line_for_array_end() {
    let line_str = ">";
    let expected = LineTypes::ArrayEnd;

    let line_type = LineTypes::try_from(line_str).unwrap();
    assert_eq!(expected, line_type);
  }
}
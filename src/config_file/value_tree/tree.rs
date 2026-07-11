use std::fmt::{Debug, Display, Formatter};
use crate::config_file::value_tree::array::Array;
use crate::config_file::value_tree::blank_line::BlankLine;
use crate::config_file::value_tree::comment::Comment;
use crate::config_file::value_tree::error::Error;
use crate::config_file::value_tree::line_type::LineTypes;
use crate::config_file::value_tree::node::Node;
use crate::config_file::value_tree::value_pair::ValuePair;

static SUBTREE_START_MARKER: char = '{';

#[derive(Debug)]
pub(crate) struct Tree {
  name: String,
  tree_map: Vec<Box<dyn Node>>,
}

impl Tree {

  pub fn new(name: String, lines: &mut dyn Iterator<Item=String>) -> Result<Tree, Error> {
    let name = name.trim().to_string();
    let mut tree = Tree { name, tree_map: vec![] };

    let mut tree_has_ended_flag = false;

    while !tree_has_ended_flag {
      let next = lines.next();
      let raw = match &next {
        None => return Ok(tree),
        Some(line) => line.as_str()
      };
      let line = raw.trim();

      match LineTypes::from(line) {
        LineTypes::KeyValuePair => {
          tree.tree_map.push(Box::new(ValuePair::try_new(raw.trim_start())?));
        },
        LineTypes::TreeStart => {
          let name_end = line.chars().position(|c| c == SUBTREE_START_MARKER).unwrap();
          let name = line[..name_end].trim();
          tree.tree_map.push(Box::new(Tree::new(name.to_string(), lines)?));
        },
        LineTypes::TreeEnd => {
          tree_has_ended_flag = true;
        },
        LineTypes::WhiteSpace => {
          tree.tree_map.push(Box::new(BlankLine));
        },
        LineTypes::Comment => {
          tree.tree_map.push(Box::new(Comment::new(raw.trim_start())));
        },
        LineTypes::ArrayStart => {
          tree.tree_map.push(Box::new(Array::new(line.to_string(), lines)));
        },
        LineTypes::ArrayEnd => {},
        LineTypes::Unknown => {
          return Err(Error::UnknownLineType);
        }
      }
    }

    Ok(tree)
  }

}

impl Display for Tree {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{} : {:?}", self.name, self.tree_map)
  }
}

impl PartialEq for Tree {
  fn eq(&self, other: &Self) -> bool {
    let mut self_string = String::new();
    self.export(&mut self_string, 0, false);

    let mut other_string = String::new();
    other.export(&mut other_string, 0, false);

    self_string == other_string
  }
}

impl Node for Tree {
  fn name(&self) -> &str {
    &self.name
  }

  fn export(&self, s: &mut String, indent: usize, skip_root: bool) {
    let indent_string = " ".repeat(indent * 4);
    let child_indent = if skip_root { indent } else { indent + 1 };

    if !skip_root {
      s.push_str(&indent_string);
      s.push_str(&self.name);
      s.push(' ');
      s.push('{');
      s.push('\n');
    }
    for node in &self.tree_map {
      node.export(s, child_indent, false);
    }
    if !skip_root {
      s.push_str(&indent_string);
      s.push('}');
      s.push('\n');
    }
  }
}

#[cfg(test)]
mod tests {
  use crate::config_file::value_tree::array::Array;
  use crate::config_file::value_tree::node::Node;
  use crate::config_file::value_tree::tree::Tree;
  use crate::config_file::value_tree::value_pair::ValuePair;

  #[test]
  fn test_that_correct_name_is_retrieved() {
    let name = "test tree";
    let test_content = "# Hallo \n # Welt".to_string();
    let mut lines = test_content.lines().map(|s| s.to_string());

    let result = Tree::new(name.to_string(), &mut lines).unwrap();
    assert_eq!(result.name, name);
  }

  #[test]
  fn test_that_comments_are_standalone_nodes() {
    let test_content = "# Hallo\n# Welt\ntest {\n  S:key=value\n}".to_string();

    let mut lines = test_content.lines().map(|s| s.to_string());
    let result = Tree::new("tests".to_string(), &mut lines).unwrap();

    let mut output = String::new();
    result.export(&mut output, 0, true);

    assert!(output.contains("# Hallo"));
    assert!(output.contains("# Welt"));
    assert!(output.find("# Hallo").unwrap() < output.find("# Welt").unwrap());
    assert!(output.find("# Welt").unwrap() < output.find("test {").unwrap());
  }

  #[test]
  fn test_that_tree_with_key_value_pairs_is_parsed_correctly() {
    let test_content = "test {\n  S:key=value\n}".to_string();
    let mut lines = test_content.lines().map(|s| s.to_string());

    let expected = Tree {
      name: "root".to_string(),
      tree_map: vec![Box::new(Tree {
        name: "test".to_string(),
        tree_map: vec![Box::new(ValuePair {
          datatype: "S".to_string(),
          name: "key".to_string(),
          value: "value".to_string(),
        })],
      })],
    };

    let result = Tree::new("root".to_string(), &mut lines).unwrap();
    assert_eq!(expected, result);
  }

  #[test]
  fn test_that_tree_with_array_is_parsed_correctly() {
    let test_content = "test {\n  S:array <\n    v1\n    v2\n  >\n}".to_string();
    let mut lines = test_content.lines().map(|s| s.to_string());

    let expected = Tree {
      name: "root".to_string(),
      tree_map: vec![Box::new(Tree {
        name: "test".to_string(),
        tree_map: vec![Box::new(Array {
          name: "array".to_string(),
          datatype: "S".to_string(),
          values: vec!["v1".to_string(), "v2".to_string()],
        })],
      })],
    };

    let result = Tree::new("root".to_string(), &mut lines).unwrap();
    assert_eq!(expected, result);
  }

  #[test]
  fn test_that_comments_between_blank_lines_are_preserved_in_order() {
    let test_content = "
      B:key1=value1

      # standalone comment

      B:key2=value2
    ".to_string();

    let mut lines = test_content.lines().map(|s| s.to_string());
    let result = Tree::new("root".to_string(), &mut lines).unwrap();

    let mut output = String::new();
    result.export(&mut output, 0, true);

    assert!(output.contains("# standalone comment"));
    let comment_pos = output.find("# standalone comment").unwrap();
    let key1_pos = output.find("B:key1=value1").unwrap();
    let key2_pos = output.find("B:key2=value2").unwrap();
    assert!(key1_pos < comment_pos && comment_pos < key2_pos);
  }
}

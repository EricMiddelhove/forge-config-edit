use std::fmt::{Debug, Display, Formatter};
use crate::config_file::value_tree::array::Array;
use crate::config_file::value_tree::error::Error;
use crate::config_file::value_tree::line_type::LineTypes;
use crate::config_file::value_tree::node::Node;
use crate::config_file::value_tree::value_pair::ValuePair;

static SUBTREE_START_MARKER: char = '{';

#[derive(Debug)]
pub(crate) struct Tree {
  name: String,
  comments: Vec<String>,
  tree_map: Vec<Box<dyn Node>>,
}

impl Tree {

  pub fn new(name: String, lines: &mut dyn Iterator<Item=String>) -> Result<Tree, Error> {
    let name = name.trim().to_string();
    let mut tree = Tree {
      name,
      comments: vec![],
      tree_map: vec![],
    };

    let mut comment_buffer = Vec::<String>::new();
    let mut tree_has_ended_flag = false;

    while !tree_has_ended_flag {
      let line = &lines.next();

      let line = match line {
        None => return Ok(tree),
        Some(line) => line
      }.trim();

      let line_type = LineTypes::from(line);
      match line_type {
        LineTypes::KeyValuePair => {
          let item = ValuePair::try_new(line, comment_buffer.clone())?;
          comment_buffer = Vec::<String>::new();
          let boxed = Box::new(item);
          tree.tree_map.push(boxed);
        },
        LineTypes::TreeStart => {
          let name_start = line.chars().position(|c| c.is_alphabetic()).unwrap();
          let name_end = line.chars().position(|c| c == SUBTREE_START_MARKER).unwrap();
          let name = &line[name_start..name_end];

          let mut sub_tree = Tree::new(name.to_string(), lines)?;
          sub_tree.comments = comment_buffer.clone();
          comment_buffer = Vec::<String>::new();

          let boxed = Box::new(sub_tree);
          tree.tree_map.push(boxed);
        },
        LineTypes::TreeEnd => {
          tree.comments = comment_buffer.clone();
          comment_buffer = Vec::<String>::new();

          tree_has_ended_flag = true;
        },
        LineTypes::WhiteSpace => {},
        LineTypes::Comment => {
          comment_buffer.push(line.to_string());
        },
        LineTypes::ArrayStart => {
          let array = Array::new(line.to_string(), comment_buffer.clone(), lines);
          comment_buffer = Vec::<String>::new();
          tree.tree_map.push(Box::new(array));
        },
        LineTypes::ArrayEnd => {},
        LineTypes::Unknown => {
          return Err(Error::UnknownLineType)
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
  fn comments(&self) -> &[String] {
    self.comments.as_slice()
  }

  fn export(&self, s: &mut String, indent: usize, skip_root: bool) {
    let indent_string = " ".repeat(indent.saturating_sub(1));

    for comment in &self.comments {
      s.push_str(&indent_string);
      s.push_str(&comment);
      s.push('\n');
    }

    if !skip_root {
      s.push_str(&indent_string);
      s.push_str(format!("{} ", &self.name).as_str());
      s.push(' ');
      s.push('{');
      s.push('\n');
    }
    for node in &self.tree_map {
      node.export(s, indent+1, false);
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
  use crate::config_file::value_tree::tree::Tree;
  use crate::config_file::value_tree::value_pair::ValuePair;

  #[test]
  fn test_that_correct_name_is_retrieved(){
    let name = "test tree";
    let test_content = "# Hallo \n # Welt".to_string();
    let mut lines = test_content.lines().map(|s| s.to_string());

    let result = Tree::new(name.to_string(), &mut lines);
    let name = result.unwrap().name;
    assert_eq!(name, name);
  }

  #[test] // skip test for now, since this behavior is not implemented yet https://github.com/EricMiddelhove/forge-config-edit/issues/1
  fn test_that_correct_comments_are_retrieved(){

    let test_content = "
      # Hallo
      # Welt
      test {
        S:key=value
      }
    ".to_string();

    let mut lines = test_content.lines().map(|s| s.to_string());
    let expected_comments = vec!["# Hallo".to_string(), "# Welt".to_string()];

    let result = Tree::new("tests".to_string(), &mut lines).unwrap();

    let tree = result.tree_map.iter().find(|t| t.name() == "test" ).unwrap();
    let comments = tree.comments();

    assert_eq!(comments, expected_comments);
  }

  #[test]
  fn test_that_tree_with_key_value_pairs_is_parsed_correctly() {
    let name = "root";
    let test_content = "
      test {
        S:key=value
      }
    ".to_string();

    let mut lines = test_content.lines().map(|s| s.to_string());

    let exptected_subtree = Tree {
      name: "root".to_string(),
      comments: vec![],
      tree_map: vec![Box::new(Tree{
        name: "test".to_string(),
        comments: vec![],
        tree_map: vec![Box::new(ValuePair{
          datatype: "S".to_string(),
          value: "value".to_string(),
          name: "key".to_string(),
          comments: vec![]
        })],
      })],
    };

    let result = Tree::new(name.to_string(), &mut lines).unwrap();

    assert_eq!(exptected_subtree, result);
  }

  #[test]
  fn test_that_tree_with_array_is_parsed_correctly() {
    let name = "root";
    let test_content = "
      test {
        S:array <
          v1
          v2
        >
      }
    ".to_string();

    let mut lines = test_content.lines().map(|s| s.to_string());

    let exptected_subtree = Tree {
      name: "root".to_string(),
      comments: vec![],
      tree_map: vec![Box::new(Tree{
        name: "test".to_string(),
        comments: vec![],
        tree_map: vec![Box::new(Array{
          name: "array".to_string(),
          datatype: "S".to_string(),
          comments: vec![],
          values: vec!["v1".to_string(), "v2".to_string()]
        })],
      })],
    };

    let result = Tree::new(name.to_string(), &mut lines).unwrap();

    assert_eq!(exptected_subtree, result);
  }
}
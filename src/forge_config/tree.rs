use std::fmt::{Debug};
use std::str::Lines;
use crate::forge_config::node::{Node};
use crate::forge_config::error::Error;
use crate::forge_config::line_type::LineTypes;
use crate::value_pair::ValuePair;

static SUBTREE_START_MARKER: char = '{';
pub(crate) static SUBTREE_END_MARKER: char = '}';


#[derive(Debug)]
pub(crate) struct Tree {
  name: String,
  comments: Vec<String>,
  tree_map: Vec<Box<dyn Node>>,
  is_root: bool,
}

impl Tree {

  pub fn new(name: String, lines: &mut Lines) -> Result<Tree, Error> {
    let name = name.trim().to_string();
    let mut tree = Tree {
      name,
      comments: vec![],
      tree_map: vec![],
      is_root: false,
    };

    let mut comment_buffer = Vec::<String>::new();

    let mut tree_has_ended_flag = false;
    while !tree_has_ended_flag {
      let line = &lines.next();

      let line = match line {
        None => return Ok(tree),
        Some(line) => line
      }.trim();

      let line_type = LineTypes::try_from(line)?;
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

          let sub_tree = Tree::new(name.to_string(), lines)?;
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
      }
    }

    Ok(tree)
  }

}

impl Node for Tree {
  fn name(&self) -> &str {
    &self.name.trim()
  }

  fn comments(&self) -> &[String] {
    &self.comments
  }

  fn export(&self, s: &mut String, indent: usize, skip_root: bool) {
    let indent_string = " ".repeat(indent);

    for comment in &self.comments {
      s.push_str(&indent_string);
      s.push_str(&comment);
      s.push('\n');
    }

    if !skip_root {
      s.push_str(format!("{} ", &self.name).as_str());
      s.push(' ');
      s.push('{');
      s.push('\n');
    }
    for node in &self.tree_map {
      node.export(s, indent+1, false);
    }
    if !skip_root {
      s.push('\n');
      s.push('}');
      s.push('\n');
    }
  }
}

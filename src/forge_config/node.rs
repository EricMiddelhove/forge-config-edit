use std::fmt::{write, Debug, Formatter};
use crate::forge_config::tree::Tree;

pub trait Node{
  fn name(&self) -> &str;

  fn comments(&self) -> &[String];

  fn export(&self, s: &mut String, indent: usize, skip_root: bool);
}

impl Debug for dyn Node {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "{:?} - comments: {:?}", &self.name(), &self.comments().iter().count())
  }
}
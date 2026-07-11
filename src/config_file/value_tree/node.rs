use std::fmt::{Debug, Display};

pub trait Node: Debug + Display {

  fn name(&self) -> &str;

  fn export(&self, s: &mut String, indent: usize, skip_root: bool);
}

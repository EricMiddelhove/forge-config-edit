use std::fmt::{Debug, Display, Formatter};
use crate::config_file::value_tree::node::Node;

#[derive(Debug, PartialEq)]
pub(crate) struct Comment(String);

impl Comment {
    pub(crate) fn new(line: &str) -> Self {
        Comment(line.to_string())
    }
}

impl Display for Comment {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Node for Comment {
    fn name(&self) -> &str {
        ""
    }

    fn export(&self, s: &mut String, indent: usize, _skip_root: bool) {
        let indent_string = " ".repeat(indent * 4);
        s.push_str(&indent_string);
        s.push_str(&self.0);
        s.push('\n');
    }
}

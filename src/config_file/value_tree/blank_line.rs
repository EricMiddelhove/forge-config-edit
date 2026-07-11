use std::fmt::{Debug, Display, Formatter};
use crate::config_file::value_tree::node::Node;

#[derive(Debug, PartialEq)]
pub(crate) struct BlankLine;

impl Display for BlankLine {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "")
    }
}

impl Node for BlankLine {
    fn name(&self) -> &str {
        ""
    }

    fn export(&self, s: &mut String, _indent: usize, _skip_root: bool) {
        s.push('\n');
    }
}

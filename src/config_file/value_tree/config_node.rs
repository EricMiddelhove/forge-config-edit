use crate::config_file::value_tree::array::Array;
use crate::config_file::value_tree::comment::Comment;
use crate::config_file::value_tree::tree::Tree;
use crate::config_file::value_tree::value_pair::ValuePair;

#[derive(Debug, PartialEq)]
pub(crate) enum ConfigNode {
    Tree(Box<Tree>),
    ValuePair(ValuePair),
    Array(Array),
    Comment(Comment),
    BlankLine,
}

impl ConfigNode {
    pub(crate) fn name(&self) -> &str {
        match self {
            ConfigNode::Tree(t) => t.name(),
            ConfigNode::ValuePair(vp) => &vp.name,
            ConfigNode::Array(a) => &a.name,
            ConfigNode::Comment(_) | ConfigNode::BlankLine => "",
        }
    }

    pub(crate) fn export(&self, s: &mut String, indent: usize, skip_root: bool) {
        match self {
            ConfigNode::Tree(t) => t.export(s, indent, skip_root),
            ConfigNode::ValuePair(vp) => vp.export(s, indent),
            ConfigNode::Array(a) => a.export(s, indent),
            ConfigNode::Comment(c) => c.export(s, indent),
            ConfigNode::BlankLine => s.push('\n'),
        }
    }
}

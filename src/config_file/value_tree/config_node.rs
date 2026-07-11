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

    // Returns the raw value string for a ValuePair, None for everything else.
    pub(crate) fn value(&self) -> Option<&str> {
        match self {
            ConfigNode::ValuePair(vp) => Some(&vp.value),
            _ => None,
        }
    }

    // Returns a human-readable type name for --type output.
    pub(crate) fn type_name(&self) -> &str {
        match self {
            ConfigNode::Tree(_) => "subtree",
            ConfigNode::ValuePair(vp) => Self::map_datatype(&vp.datatype),
            ConfigNode::Array(a) => Self::map_array_datatype(&a.datatype),
            ConfigNode::Comment(_) | ConfigNode::BlankLine => "",
        }
    }

    fn map_datatype(code: &str) -> &'static str {
        match code {
            "B" => "bool",
            "S" => "string",
            "I" => "int",
            "D" => "double",
            "L" => "long",
            "C" => "char",
            _ => "unknown",
        }
    }

    fn map_array_datatype(code: &str) -> &'static str {
        match code {
            "B" => "bool[]",
            "S" => "string[]",
            "I" => "int[]",
            "D" => "double[]",
            "L" => "long[]",
            "C" => "char[]",
            _ => "unknown[]",
        }
    }
}

use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq)]
pub(crate) struct Comment(String);

impl Comment {
    pub(crate) fn new(line: &str) -> Self {
        Comment(line.to_string())
    }

    pub(crate) fn export(&self, s: &mut String, indent: usize) {
        let indent_string = " ".repeat(indent * 4);
        s.push_str(&indent_string);
        s.push_str(&self.0);
        s.push('\n');
    }
}

impl Display for Comment {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

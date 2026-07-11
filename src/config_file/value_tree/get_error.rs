use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum GetError {
    EmptyPath,
    NotFound(String),
    NotASection(String),
}

impl Display for GetError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            GetError::EmptyPath => write!(f, "path must not be empty"),
            GetError::NotFound(k) => write!(f, "key not found: {k}"),
            GetError::NotASection(k) => write!(f, "'{k}' is a value, not a section"),
        }
    }
}

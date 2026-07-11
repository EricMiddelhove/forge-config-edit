use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum SetError {
    EmptyPath,
    NotFound(String),
    IsSection(String),
    IsArray(String),
    NotASection(String),
}

impl Display for SetError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SetError::EmptyPath => write!(f, "path must not be empty"),
            SetError::NotFound(k) => write!(f, "key not found: {k}"),
            SetError::IsSection(k) => write!(f, "'{k}' is a section, not a value"),
            SetError::IsArray(k) => write!(f, "'{k}' is an array; array mutation is not supported"),
            SetError::NotASection(k) => write!(f, "'{k}' is a value, not a section"),
        }
    }
}

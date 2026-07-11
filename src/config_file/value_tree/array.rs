use std::fmt::{Display, Formatter};
use crate::config_file::value_tree::line_type::LineTypes;
use crate::config_file::value_tree::node::Node;

static DATATYPE_NAME_SEPARATOR: char = ':';
static NAME_VALUE_SEPARATOR: char = '<';
static VALUE_END_SEPARATOR: char = '>';

#[derive(Debug, PartialEq)]
pub(crate) struct Array {
    pub(crate) name: String,
    pub(crate) datatype: String,
    pub(crate) values: Vec<String>,
}

impl Display for Array {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}:{} < {} >", self.datatype, self.name, self.values.join(";"))
    }
}

impl Node for Array {
    fn name(&self) -> &str {
        &self.name
    }

    fn export(&self, s: &mut String, indent: usize, _skip_root: bool) {
        let indent_string = " ".repeat(indent * 4);
        s.push_str(&indent_string);
        s.push_str(&format!("{}:{} {}", self.datatype, self.name, NAME_VALUE_SEPARATOR));
        s.push('\n');

        let internal_indent = " ".repeat((indent + 1) * 4);
        for value in &self.values {
            s.push_str(&internal_indent);
            s.push_str(value);
            s.push('\n');
        }

        s.push_str(&" ".repeat(indent * 4 + 1));
        s.push(VALUE_END_SEPARATOR);
        s.push('\n');
    }
}

impl Array {
    pub fn new(headline: String, lines: &mut dyn Iterator<Item=String>) -> Array {
        let mut values = Vec::<String>::new();

        let mut array_has_ended_flag = false;
        while !array_has_ended_flag {
            let line = &lines.next();
            let line = match line {
                Some(line) => line,
                None => break
            };

            let line_type = LineTypes::from(line.trim());

            if line_type == LineTypes::ArrayEnd {
                array_has_ended_flag = true;
            } else {
                values.push(line.trim_start().to_string());
            }
        }

        let name_start = headline.chars().position(|c| c == DATATYPE_NAME_SEPARATOR).unwrap() + 1;
        let name_end = headline.chars().position(|c| c == NAME_VALUE_SEPARATOR).unwrap();
        let name = headline[name_start..name_end].trim().to_string();
        let datatype = headline.chars().nth(0).unwrap().to_string();

        Array { name, datatype, values }
    }
}

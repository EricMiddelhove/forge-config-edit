use crate::config_file::value_tree::config_node::ConfigNode;
use crate::config_file::value_tree::get_error::GetError;
use crate::config_file::value_tree::set_error::SetError;
use crate::config_file::value_tree::tree::Tree;

pub(crate) struct ConfigFile {
    tree: Tree,
}

impl ConfigFile {
    pub(crate) fn new(data: String) -> ConfigFile {
        let mut buffer_lines = data.lines().map(|l| l.to_string());
        let tree = Tree::new("root".to_string(), &mut buffer_lines).unwrap();
        ConfigFile { tree }
    }

    pub(crate) fn find<'a>(&'a self, path: &[&str]) -> Result<&'a ConfigNode, GetError> {
        self.tree.find(path)
    }

    pub(crate) fn set(&mut self, path: &[&str], value: &str) -> Result<(), SetError> {
        self.tree.set(path, value)
    }

    pub(crate) fn export(&self) {
        let mut s = String::from("# Modified with EricMiddelhove/forge-config-edit\n");
        self.tree.export(&mut s, 0, true);
        print!("{}", s);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_file_find_returns_correct_value() {
        let input = "server {\n    B:enabled=true\n}".to_string();
        let file = ConfigFile::new(input);

        let node = file.find(&["server", "enabled"]).unwrap();
        assert_eq!(node.value(), Some("true"));
    }

    #[test]
    fn test_config_file_set_mutates_correctly() {
        let input = "server {\n    B:enabled=true\n}".to_string();
        let mut file = ConfigFile::new(input);

        file.set(&["server", "enabled"], "false").unwrap();

        let node = file.find(&["server", "enabled"]).unwrap();
        assert_eq!(node.value(), Some("false"));
    }
}

impl From<String> for ConfigFile {
    fn from(data: String) -> ConfigFile {
        ConfigFile::new(String::from(data))
    }
}

impl From<Box<dyn Iterator<Item=String>>> for ConfigFile {
    fn from(value: Box<dyn Iterator<Item=String>>) -> Self {
        let mut value = value;
        let tree = Tree::new("root".to_string(), &mut value).unwrap();
        ConfigFile { tree }
    }
}

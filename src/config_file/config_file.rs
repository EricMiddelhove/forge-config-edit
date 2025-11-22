use std::io::{BufRead, StdinLock};
use crate::config_file::value_tree::node::Node;
use crate::config_file::value_tree::tree::Tree;

pub(crate) struct ConfigFile {
    tree: Tree,
}

impl ConfigFile {
    pub (crate) fn new(data: String) -> ConfigFile {

        let mut buffer_lines = data.lines().map(|l| l.to_string());

        let tree = Tree::new("root".to_string(), &mut buffer_lines);

        let tree = tree.unwrap();

        ConfigFile { tree }

    }

    pub(crate) fn export(&self) {
        let mut s = String::from("# Modified with EricMiddelhove/forge-config-edit\n");
        self.tree.export(&mut s, 0, true);
        println!("{}", s);
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

        let tree = Tree::new("root".to_string(), &mut value);

        let tree = tree.unwrap();

        ConfigFile { tree }
    }
}
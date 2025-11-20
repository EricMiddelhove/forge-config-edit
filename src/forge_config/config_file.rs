use std::ops::ControlFlow;
use crate::forge_config::node::Node;
use crate::forge_config::tree::Tree;

pub(crate) struct ConfigFile {
    tree: Tree,
}

impl ConfigFile {
    pub (crate) fn new(data: String) -> ConfigFile {

        let buffer_string: String = String::from_utf8(data).unwrap();

        let mut buffer_lines = buffer_string.lines();

        let tree = Tree::new("root".to_string(), &mut buffer_lines);

        let tree = tree.unwrap();

        ConfigFile { tree }

    }

    pub(crate) fn export(&self) {
        let mut s = String::new();
        self.tree.export(&mut s, 0, true);
        println!("{}", s);
    }
}

impl From<String> for ConfigFile {
    fn from(data: String) -> ConfigFile {
        ConfigFile::new(data)
    }
}
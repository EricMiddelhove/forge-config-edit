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
        let mut s = String::new();
        self.tree.export(&mut s, 0, true);
        println!("{}", s);
    }
}

impl From<String> for ConfigFile {
    fn from(data: String) -> ConfigFile {
        ConfigFile::new(String::from(data))
    }
}

impl From<std::io::Lines<StdinLock<'_>>> for ConfigFile {
    fn from(value: std::io::Lines<StdinLock<'_>>) -> Self {

        let mut value = value;

        let str_vec = value
          .map( |v| v
            .unwrap()
          );

        // this is so goddamn stupid there must be a better way

        // let mut str_lines = str_vec.lines();

        let tree = Tree::new("root".to_string(), &mut str_vec.into_iter());

        let tree = tree.unwrap();

        ConfigFile { tree }
    }
}
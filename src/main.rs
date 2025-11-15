mod value_pair;
mod forge_config;

use std::fs::File;
use std::io::{BufRead, Read};
use forge_config::tree::Tree;
use forge_config::node::Node;


fn main(){

    let mut file = File::open("/Users/ericmiddelhove/Development/forge-config-edit/examples/Avaritia.cfg").unwrap();

    let mut buffer: Vec<u8> = Vec::new();
    let res = file.read_to_end(&mut buffer);

    let buffer_string: String = String::from_utf8(buffer).unwrap();

    let mut buffer_lines = buffer_string.lines();

    let tree = Tree::new("root".to_string(), &mut buffer_lines);

    let tree = tree.unwrap();

    let mut s = String::new();
    tree.export(&mut s, 0, true);
    println!("{}", s);
}

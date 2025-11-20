mod forge_config;

use std::fs::File;
use std::io::{Read};

use forge_config::config_file::ConfigFile;


fn main(){

    let mut file = File::open("examples/serverutilities.cfg").unwrap();

    let mut buffer: Vec<u8> = Vec::new();
    let _ = file.read_to_end(&mut buffer);

    let string_buf = String::from_utf8(buffer).unwrap();

    let file = ConfigFile::from(string_buf);

    file.export();
}

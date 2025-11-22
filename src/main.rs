mod config_file;
mod data_provider;

use std::env;
use std::fs::File;
use std::io::{Read};

use config_file::config_file::ConfigFile;
use crate::data_provider::data_provider::DataProvider;
use crate::data_provider::implementations::file_provider::FileProvider;

fn main(){
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let provider = match_data_provider();
    let mut buffer = provider.read();
    
    let string_buf = String::from_utf8(Vec::from(buffer)).unwrap(); // cloning data

    let file = ConfigFile::from(string_buf);

    file.export();
}



fn match_data_provider() -> impl DataProvider {

    let mut file = File::open("examples/serverutilities.cfg").unwrap();
    FileProvider::new(file)

}
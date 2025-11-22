mod config_file;
mod data_provider;

use std::env;
use std::fs::File;

use config_file::config_file::ConfigFile;
use crate::data_provider::data_provider::DataProvider;
use crate::data_provider::implementations::file_provider::FileProvider;
use crate::data_provider::implementations::stdin_provider::StdinProvider;

fn main(){
    let args: Vec<String> = env::args().collect();

    let provider = match_data_provider(args);
    let buffer = provider.read();

    let string_buf = String::from_utf8(Vec::from(buffer)).unwrap(); // cloning data

    let file = ConfigFile::from(string_buf);

    file.export();
}



fn match_data_provider(args: Vec<String>) -> Box<dyn DataProvider> {

    let mut iter = args.iter();

    if iter.find(|c| c.as_str() == "-p").is_some() {

        let path_str = iter.next().unwrap();
        let file = File::open(path_str).unwrap();
        Box::new(FileProvider::new(file))

    } else {
        println!("Using Stdin");
        Box::new(StdinProvider::new())
    }

}
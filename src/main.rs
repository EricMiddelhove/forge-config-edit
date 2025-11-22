mod config_file;
mod data_provider;

use std::env;
use std::env::Args;
use std::fs::File;
use std::time::Instant;
use config_file::config_file::ConfigFile;
use crate::data_provider::data_provider::DataProvider;
use crate::data_provider::implementations::file_provider::FileProvider;
use crate::data_provider::implementations::stdin_provider::StdinProvider;

fn main(){

    let time = Instant::now();

    let args = env::args();

    let provider = match_data_provider(args);

    let buffer = provider.read();

    let file = ConfigFile::from(buffer);

    file.export();

    println!("Took {:?}", time.elapsed());
}

fn match_data_provider(args: Args) -> Box<dyn DataProvider> {

    let mut args = args;

    if args.find(|c| c.as_str() == "-p").is_some() {

        let path_str = args.next().unwrap();
        let file = File::open(path_str).unwrap();
        Box::new(FileProvider::new(file))
    } else {
        println!("Using Stdin");
        Box::new(StdinProvider::new())
    }

}
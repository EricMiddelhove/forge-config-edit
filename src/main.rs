mod config_file;
mod data_provider;

use std::env;
use std::fs::File;
use std::time::Instant;
use config_file::config_file::ConfigFile;
use crate::data_provider::data_provider::DataProvider;
use crate::data_provider::implementations::file_provider::FileProvider;
use crate::data_provider::implementations::stdin_provider::StdinProvider;

const HELP: &str = "\
Usage: forge-config-edit [OPTIONS]

Parses and re-exports Forge/FML configuration files with full round-trip
fidelity, preserving comments, blank lines, and formatting.

Options:
  --input-file-path <PATH>   Path to the input config file.
                             If omitted, reads from stdin.
  -h, --help                 Show this help message and exit.

Examples:
  forge-config-edit --input-file-path config.cfg
  cat config.cfg | forge-config-edit
";

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.iter().any(|a| a == "-h" || a == "--help") {
        print!("{}", HELP);
        return;
    }

    let time = Instant::now();
    let provider = match_data_provider(&args);
    let buffer = provider.read();
    let file = ConfigFile::from(buffer);
    file.export();
    eprintln!("Took {:?}", time.elapsed());
}

fn match_data_provider(args: &[String]) -> Box<dyn DataProvider> {
    if let Some(pos) = args.iter().position(|a| a == "--input-file-path") {
        let path = args.get(pos + 1).expect("--input-file-path requires a value");
        let file = File::open(path).unwrap();
        Box::new(FileProvider::new(file))
    } else {
        Box::new(StdinProvider::new())
    }
}

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
  --set <PATH>=<VALUE>       Set a config value. PATH is dot-separated:
                             section.subsection.key. May be repeated.
  -h, --help                 Show this help message and exit.

Examples:
  forge-config-edit --input-file-path config.cfg
  cat config.cfg | forge-config-edit
  forge-config-edit --input-file-path config.cfg --set balance!.someKey=false
  forge-config-edit --input-file-path config.cfg --set a.b.key=1 --set c.key=2
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
    let mut file = ConfigFile::from(buffer);

    for set_arg in collect_set_args(&args) {
        let eq_pos = match set_arg.find('=') {
            Some(p) => p,
            None => {
                eprintln!("Error: --set value must be in the format path.key=value (got: {set_arg})");
                std::process::exit(1);
            }
        };
        let path_str = &set_arg[..eq_pos];
        let value = &set_arg[eq_pos + 1..];
        let path: Vec<&str> = path_str.split('.').collect();

        if let Err(e) = file.set(&path, value) {
            eprintln!("Error: {e}");
            std::process::exit(1);
        }
    }

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

fn collect_set_args(args: &[String]) -> Vec<&str> {
    let mut result = Vec::new();
    let mut i = 0;
    while i < args.len() {
        if args[i] == "--set" {
            if let Some(val) = args.get(i + 1) {
                result.push(val.as_str());
                i += 2;
                continue;
            } else {
                eprintln!("Error: --set requires a value");
                std::process::exit(1);
            }
        }
        i += 1;
    }
    result
}

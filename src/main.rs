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
Usage: forge-config-edit [OPTIONS] [FILE]

Parses and re-exports Forge/FML configuration files with full round-trip
fidelity, preserving comments, blank lines, and formatting. FILE can be
given as a positional argument or via --input-file-path; omit both to read
from stdin.

Paths use dot notation: section.subsection.key

Options:
  FILE                         Input config file.
  --input-file-path <FILE>     Input config file (alternative to positional).
  --set <PATH>=<VALUE>         Set a value and write the modified config to
                               stdout. May be repeated for multiple keys.
  --get=<PATH>                 Print the value at PATH. When PATH is a section,
                               its full contents are printed.
  --type=<PATH>                Print the type of the key at PATH.
                               Values:  bool | string | int | double | long | char
                               Arrays:  bool[] | string[] | ...
                               Section: subtree
  -h, --help                   Show this help message and exit.

Examples:
  # Re-export a config unchanged (round-trip check)
  forge-config-edit config.cfg

  # Read from stdin
  cat config.cfg | forge-config-edit

  # Modify a value and write result to a new file
  forge-config-edit --set backups.enable_backups=false config.cfg > out.cfg

  # Modify multiple values at once
  forge-config-edit --set afk.enabled=false --set backups.compression_level=9 config.cfg

  # Read a single value
  forge-config-edit --get=backups.enable_backups config.cfg

  # Read an entire section
  forge-config-edit --get=backups config.cfg

  # Check the type of a key
  forge-config-edit --type=backups.compression_level config.cfg
";

struct ParsedArgs {
    file_path: Option<String>,
    set_args: Vec<String>,
    get_path: Option<String>,
    type_path: Option<String>,
    help: bool,
}

fn parse_args(args: &[String]) -> ParsedArgs {
    let mut result = ParsedArgs {
        file_path: None,
        set_args: vec![],
        get_path: None,
        type_path: None,
        help: false,
    };

    let mut i = 1;
    while i < args.len() {
        let arg = &args[i];
        if arg == "-h" || arg == "--help" {
            result.help = true;
        } else if arg == "--input-file-path" {
            i += 1;
            result.file_path = Some(args.get(i).expect("--input-file-path requires a value").clone());
        } else if let Some(v) = arg.strip_prefix("--input-file-path=") {
            result.file_path = Some(v.to_string());
        } else if arg == "--set" {
            i += 1;
            result.set_args.push(args.get(i).expect("--set requires a value").clone());
        } else if let Some(v) = arg.strip_prefix("--set=") {
            result.set_args.push(v.to_string());
        } else if let Some(v) = arg.strip_prefix("--get=") {
            result.get_path = Some(v.to_string());
        } else if arg == "--get" {
            i += 1;
            result.get_path = Some(args.get(i).expect("--get requires a value").clone());
        } else if let Some(v) = arg.strip_prefix("--type=") {
            result.type_path = Some(v.to_string());
        } else if arg == "--type" {
            i += 1;
            result.type_path = Some(args.get(i).expect("--type requires a value").clone());
        } else if !arg.starts_with('-') {
            result.file_path = Some(arg.clone());
        } else {
            eprintln!("Error: unknown option '{arg}'");
            std::process::exit(1);
        }
        i += 1;
    }

    result
}

fn parse_config_path(raw: &str) -> Vec<&str> {
    raw.split('.').filter(|s| !s.is_empty()).collect()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let parsed = parse_args(&args);

    if parsed.help {
        print!("{}", HELP);
        return;
    }

    if parsed.get_path.is_some() && parsed.type_path.is_some() {
        eprintln!("Error: --get and --type cannot be used together");
        std::process::exit(1);
    }

    let query_mode = parsed.get_path.is_some() || parsed.type_path.is_some();
    if query_mode && !parsed.set_args.is_empty() {
        eprintln!("Error: --set cannot be combined with --get or --type");
        std::process::exit(1);
    }

    let time = Instant::now();
    let provider = open_provider(parsed.file_path.as_deref());
    let buffer = provider.read();
    let mut file = ConfigFile::from(buffer);

    if let Some(path_str) = parsed.get_path {
        let path = parse_config_path(&path_str);
        if path.is_empty() {
            eprintln!("Error: path must not be empty");
            std::process::exit(1);
        }
        match file.find(&path) {
            Ok(node) => {
                if let Some(v) = node.value() {
                    println!("{v}");
                } else {
                    let mut s = String::new();
                    node.export(&mut s, 0, false);
                    print!("{s}");
                }
            },
            Err(e) => {
                eprintln!("Error: {e}");
                std::process::exit(1);
            }
        }
        return;
    }

    if let Some(path_str) = parsed.type_path {
        let path = parse_config_path(&path_str);
        if path.is_empty() {
            eprintln!("Error: path must not be empty");
            std::process::exit(1);
        }
        match file.find(&path) {
            Ok(node) => println!("{}", node.type_name()),
            Err(e) => {
                eprintln!("Error: {e}");
                std::process::exit(1);
            }
        }
        return;
    }

    for set_arg in &parsed.set_args {
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

fn open_provider(file_path: Option<&str>) -> Box<dyn DataProvider> {
    match file_path {
        Some(path) => {
            let file = File::open(path).unwrap_or_else(|e| {
                eprintln!("Error: cannot open '{path}': {e}");
                std::process::exit(1);
            });
            Box::new(FileProvider::new(file))
        },
        None => Box::new(StdinProvider::new()),
    }
}

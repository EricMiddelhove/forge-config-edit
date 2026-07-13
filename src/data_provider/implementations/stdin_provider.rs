use std::io;
use crate::data_provider::data_provider::DataProvider;

pub struct StdinProvider;
impl DataProvider for StdinProvider {
  fn read(&self) -> Box<dyn Iterator<Item=String>> {
    Box::new(io::stdin().lines().map(|line| {
      line.unwrap_or_else(|e| {
        eprintln!("Error: failed to read from stdin: {e}");
        std::process::exit(1);
      })
    }))
  }
}

impl StdinProvider {
  pub fn new() -> StdinProvider {
    StdinProvider{}
  }
}
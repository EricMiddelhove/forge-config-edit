use std::io;
use std::io::{Lines, Read, Stdin, StdinLock};
use crate::data_provider::data_provider::DataProvider;

pub struct StdinProvider;
impl DataProvider for StdinProvider {
  fn read(&self) -> Box<dyn Iterator<Item=String>> {
    Box::new(io::stdin().lines().map(|s| s.unwrap()))
  }
}

impl StdinProvider {
  pub fn new() -> StdinProvider {
    StdinProvider{}
  }
}
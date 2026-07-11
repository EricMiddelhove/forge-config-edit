use crate::data_provider::data_provider::DataProvider;
use std::fs::File;
use std::io::{BufRead, Read};

pub(crate) struct FileProvider {
  buffer: String
}
impl DataProvider for FileProvider {
  fn read(&self) -> Box<dyn Iterator<Item=String>> {
    let strings = Box::new(
      self.buffer
        .lines()
        .map(|line| {String::from(line)})
        .collect::<Vec<String>>()
        .into_iter()
    );

    strings
  }
}

impl FileProvider {
  pub fn new<'a>(mut file: File) -> FileProvider {
    let mut buffer: Vec<u8> = Vec::new();
    let _ = file.read_to_end(&mut buffer);

    let buffer = String::from_utf8(buffer).unwrap();

    FileProvider{
      buffer
    }

  }
}
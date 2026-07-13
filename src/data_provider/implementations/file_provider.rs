use crate::data_provider::data_provider::DataProvider;
use std::fs::File;
use std::io;
use std::io::Read;

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
  pub fn new(mut file: File) -> io::Result<FileProvider> {
    let mut buffer: Vec<u8> = Vec::new();
    file.read_to_end(&mut buffer)?;

    let buffer = String::from_utf8(buffer)
      .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

    Ok(FileProvider { buffer })
  }
}
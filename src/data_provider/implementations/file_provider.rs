use crate::data_provider::data_provider::DataProvider;
use std::fs::File;
use std::io::Read;

pub(crate) struct FileProvider {
  data: Box<[u8]>
}
impl DataProvider for FileProvider {
  fn read(&self) -> &[u8] {
    self.data.as_ref()
  }
}

impl FileProvider {
  pub fn new<'a>(mut file: File) -> FileProvider {
    let mut buffer: Vec<u8> = Vec::new();
    let _ = file.read_to_end(&mut buffer);

    let string_buf = String::from_utf8(buffer).unwrap();

    let vecna = string_buf.into_bytes();

    let buff = vecna.into_boxed_slice();

    FileProvider{
      data: buff,
    }
  }
}
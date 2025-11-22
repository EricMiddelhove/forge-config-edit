use std::io;
use crate::data_provider::data_provider::DataProvider;

pub struct StdinProvider{
  data: Box<[u8]>
}
impl DataProvider for StdinProvider {
  fn read(&self) -> &[u8] {
    self.data.as_ref()
  }
}

impl StdinProvider {
  fn new<'a>() -> StdinProvider {
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();

    let buffer = buffer.clone();
    let buffer = buffer.into_bytes().into_boxed_slice();

    StdinProvider {
      data: buffer,
    }
  }
}
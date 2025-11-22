use std::io;
use std::io::Read;
use crate::data_provider::data_provider::DataProvider;

static PREALLOC_BUFFER_SIZE: usize = 1024;

pub struct StdinProvider{
  data: [u8; PREALLOC_BUFFER_SIZE],
}
impl DataProvider for StdinProvider {
  fn read(&self) -> &[u8] {
    self.data.as_ref()
  }
}

impl StdinProvider {
  pub(crate) fn new() -> StdinProvider {
    let mut buffer = [0; PREALLOC_BUFFER_SIZE];
    let res = io::stdin().read(&mut buffer).unwrap();

    let lines = io::stdin().lines();

    println!("{:?}", res);

    StdinProvider {
      data: buffer,
    }
  }
}
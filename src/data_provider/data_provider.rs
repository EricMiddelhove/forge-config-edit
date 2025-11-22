pub trait DataProvider {
  fn read(&self) -> &[u8];
}
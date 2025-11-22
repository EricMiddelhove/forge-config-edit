pub trait DataProvider {
  fn read(&self) -> Box<dyn Iterator<Item=String>>;
}
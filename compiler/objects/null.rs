use super::Object;

#[derive(Debug, Clone, PartialEq)]
pub struct Null;

impl Object for Null {
  fn string(self) -> String {
    String::from("null")
  }
}

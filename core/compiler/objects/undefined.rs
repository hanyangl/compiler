use super::Object;

#[derive(Debug, Clone, PartialEq)]
pub struct Undefined;

impl Object for Undefined {
  fn string(self) -> String {
    String::from("undefined")
  }
}

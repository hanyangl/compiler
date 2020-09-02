mod arguments;
mod package;
mod store;

pub use package::Package;
pub use store::Store;

use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct Environment {
  pub arguments: arguments::Arguments,

  pub files: Vec<sflyn_parser::File>,
  pub packages: HashMap<String, Package>,         // Package name + Package data

  pub store: Store,
}

impl Environment {
  pub fn new() -> Environment {
    Environment {
      arguments: arguments::Arguments::from_console(),

      files: Vec::new(),
      packages: HashMap::new(),

      store: Store::new(),
    }
  }
}

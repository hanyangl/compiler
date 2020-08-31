mod arguments;
mod package;

pub use package::Package;

use std::collections::HashMap;

pub struct Environment {
  pub arguments: arguments::Arguments,

  pub files: Vec<sflyn_parser::File>,
  pub packages: HashMap<String, Package>,
}

impl Environment {
  pub fn new() -> Environment {
    Environment {
      arguments: arguments::Arguments::from_console(),

      files: Vec::new(),
      packages: HashMap::new(),
    }
  }
}

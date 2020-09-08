mod arguments;
mod package;
mod store;

pub use package::Package;
pub use store::Store;

use sflyn_parser::{run as run_parser, File};

use std::collections::HashMap;

use super::error::show_error;

#[derive(Debug, Clone, PartialEq)]
pub struct Environment {
  pub arguments: arguments::Arguments,

  pub files: Vec<File>,
  pub packages: HashMap<String, Package>, // Package name + Package data

  pub stdlibs: HashMap<String, File>, // Lib name + Lib file

  pub store: Store,
}

impl Environment {
  pub fn new() -> Self {
    Self {
      arguments: arguments::Arguments::from_console(),

      files: Vec::new(),
      packages: HashMap::new(),

      stdlibs: HashMap::new(),

      store: Store::new(),
    }
  }

  fn load_stdlib(&mut self, path: String, name: &str) -> i32 {
    if self.stdlibs.get(&name.to_string()).is_some() {
      println!("The `{}` library was loaded.", name);
      return 1;
    }

    match run_parser(path.clone()) {
      Ok(stdlib_file) => {
        self.stdlibs.insert(name.to_string(), stdlib_file);
        return 0;
      }
      Err((error, file)) => {
        if let Some(file) = file {
          show_error(file, error);
        } else {
          println!("{}", error.message);
        }

        return 1;
      }
    }
  }

  pub fn load_stdlibs(&mut self, sflyn_path: String) -> i32 {
    let mut sflyn_path = sflyn_path;

    if !sflyn_path.ends_with("/") {
      sflyn_path = format!("{}/", sflyn_path);
    }

    self.load_stdlib(format!("{}std/main.sf", sflyn_path), "main")
  }
}

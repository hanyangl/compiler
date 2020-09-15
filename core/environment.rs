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

  pub current_file: Option<File>,
  pub files: Vec<File>,
  pub packages: HashMap<String, Package>, // Package name + Package data

  pub stdlibs: HashMap<String, File>, // Lib name + Lib file

  pub store: Store,
}

impl Environment {
  pub fn new() -> Self {
    Self {
      arguments: arguments::Arguments::from_console(),

      current_file: None,
      files: Vec::new(),
      packages: HashMap::new(),

      stdlibs: HashMap::new(),

      store: Store::new(),
    }
  }

  pub fn get_file(&self, file_name: String) -> Option<File> {
    for file in self.files.iter() {
      if file.name == file_name {
        return Some(file.clone());
      }
    }

    None
  }

  fn load_stdlib(&mut self, path: String, name: &str) -> i32 {
    if self.stdlibs.get(&name.to_string()).is_some() {
      println!("The `{}` library was loaded.", name);
      return 1;
    }

    match run_parser(path) {
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

    let main_lib = self.load_stdlib(format!("{}std/builtins.sf", sflyn_path), "builtins");
    if main_lib != 0 {
      return main_lib;
    }

    let null_lib = self.load_stdlib(format!("{}std/Null.sf", sflyn_path), "Null");
    if null_lib != 0 {
      return null_lib;
    }

    let number_lib = self.load_stdlib(format!("{}std/Number.sf", sflyn_path), "Number");
    if number_lib != 0 {
      return number_lib;
    }

    let boolean_lib = self.load_stdlib(format!("{}std/Boolean.sf", sflyn_path), "Boolean");
    if boolean_lib != 0 {
      return boolean_lib;
    }

    self.load_stdlib(format!("{}std/Array.sf", sflyn_path), "Array")
  }
}

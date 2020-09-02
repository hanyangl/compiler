use crate::{
  Export,
  Statements,
};

#[derive(Debug, Clone, PartialEq)]
pub struct File {
  pub name: String,
  pub content: String,

  pub statements: Vec<Box<Statements>>,
  pub exports: Vec<Export>,
}

impl File {
  pub fn new(name: String, content: String) -> File {
    File {
      name,
      content,

      statements: Vec::new(),
      exports: Vec::new(),
    }
  }

  pub fn get_lines(self) -> Vec<String> {
    self.content.split("\n")
      .map(|x| x.to_string())
      .collect::<Vec<String>>()
  }

  pub fn get_full_rute(&self) -> String {
    std::fs::canonicalize(&self.name).unwrap().display().to_string()
  }
}

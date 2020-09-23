use crate::utils::repeat_character;

use sflyn_parser::{
  File,
  tokens::Token,
};

use super::Objects;

#[derive(Debug, Clone, PartialEq)]
pub struct Error {
  message: String,
  token: Token,
}

impl Error {
  pub fn new(message: String, token: Token) -> Box<Objects> {
    Box::new(Objects::ERROR(Error { message, token }))
  }

  pub fn string(&self, file: File) -> String {
    if self.token.line < 1 || self.token.line > file.get_lines().len() {
      return self.message.clone();
    }

    let line = file.get_lines()[self.token.line - 1].clone();

    format!(
      "{} | {}\n{} | {}{} {}",
      self.token.line,
      line,
      repeat_character(self.token.line.to_string().len(), " "),
      repeat_character(self.token.position - 1, " "),
      repeat_character(self.token.value.len(), "^"),
      self.message,
    )
  }
}

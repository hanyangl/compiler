use super::tokens::Token;

#[derive(Debug)]
pub struct Error {
  pub message: String,
  pub line: usize,
  pub start_position: usize,
  pub end_position: usize,
}

impl Error {
  pub fn new(message: String, line: usize, start_position: usize, end_position: usize) -> Error {
    Error { message, line, start_position, end_position }
  }

  pub fn new_empty() -> Error {
    Error {
      message: String::new(),
      line: 0,
      start_position: 0,
      end_position: 0,
    }
  }

  pub fn from_token(message: String, token: Token) -> Error {
    Error {
      message,
      line: token.line,
      start_position: token.position,
      end_position: token.position + token.value.len(),
    }
  }
}

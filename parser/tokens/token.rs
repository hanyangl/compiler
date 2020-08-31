use super::*;

#[derive(Debug, Clone, PartialEq)]
pub struct Token {
  pub token: Box<Tokens>,
  pub value: String,

  pub line: usize,
  pub position: usize,
}

impl Token {
  pub fn new(token: Box<Tokens>, value: String, line: usize, position: usize) -> Token {
    Token { token, value, line, position }
  }

  pub fn new_empty() -> Token {
    Token::new(Box::new(Tokens::ILLEGAL), String::new(), 0, 0)
  }

  pub fn from_value(value: &str, line: usize, position: usize) -> Token {
    let mut token = Box::new(Tokens::ILLEGAL);

    // Get keyword
    if let Ok(keyword) = Keywords::from_value(value) {
      token = Box::new(Tokens::KEYWORD(keyword));
    }
    // Get sign
    else if let Ok(sign) = Signs::from_value(value) {
      token = Box::new(Tokens::SIGN(sign));
    }
    // Get data type
    else if let Ok(data_type) = Types::from_value(value) {
      token = Box::new(Tokens::TYPE(data_type));
    }

    Token::new(token, value.to_string(), line, position)
  }
}

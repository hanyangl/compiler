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
    Token::new(Box::new(Tokens::ILLEGAL), String::new(), 1, 1)
  }

  pub fn from_value(value: String, line: usize, position: usize) -> Token {
    let illegal = Box::new(Tokens::ILLEGAL);
    let mut token = illegal.clone();

    // Get keyword
    if token == illegal {
      match Keywords::from_value(value.clone()) {
        Some(keyword) => {
          token = keyword;
        },
        None => {},
      }
    }

    // Get sign
    if token == illegal {
      match Signs::from_value(value.clone()) {
        Some(sign) => {
          token = sign;
        },
        None => {},
      }
    }
    
    // Get data type
    if token == illegal {
      match Types::from_value(value.clone()) {
        Some(data_type) => {
          token = data_type;
        },
        None => {},
      }
    }

    // Get the new token and return it
    Token::new(token, value, line, position)
  }
}

mod keywords;
mod signs;
mod token;
mod types;

pub use keywords::Keywords;
pub use signs::Signs;
pub use token::Token;
pub use types::Types;

#[derive(Debug, Clone, PartialEq)]
pub enum Tokens {
  ILLEGAL,

  STRING,
  IDENTIFIER,
  NUMBER,

  KEYWORD(keywords::Keywords),
  SIGN(signs::Signs),
  TYPE(types::Types),

  EOL,
  EOF,
}

impl Tokens {
  /// Check if the token is an illegal token.
  pub fn is_illegal(self) -> bool {
    match self {
      Tokens::ILLEGAL => true,
      _ => false,
    }
  }

  /// Check if the token is a string token.
  pub fn is_string(self) -> bool {
    match self {
      Tokens::STRING => true,
      _ => false,
    }
  }

  /// Check if the token is an identifier token.
  pub fn is_identifier(self) -> bool {
    match self {
      Tokens::IDENTIFIER => true,
      _ => false,
    }
  }

  /// Check if the token is a number token.
  pub fn is_number(self) -> bool {
    match self {
      Tokens::NUMBER => true,
      _ => false,
    }
  }

  /// Get the keyword token.
  pub fn get_keyword(self) -> Option<keywords::Keywords> {
    match self {
      Tokens::KEYWORD(keyword) => Some(keyword),
      _ => None,
    }
  }

  /// Get the sign token.
  pub fn get_sign(self) -> Option<signs::Signs> {
    match self {
      Tokens::SIGN(sign) => Some(sign),
      _ => None,
    }
  }

  /// Get the type token.
  pub fn get_type(self) -> Option<types::Types> {
    match self {
      Tokens::TYPE(data_type) => Some(data_type),
      _ => None,
    }
  }

  /// Check if the token is the end of the line.
  pub fn is_end_of_line(self) -> bool {
    match self {
      Tokens::EOL => true,
      _ => false,
    }
  }
}

pub trait TokenType {
  /// Get a new token.
  fn new(token: Self) -> Box<Tokens>;

  /// Get a new token from a string value.
  fn from_value(value: String) -> Option<Box<Tokens>>;
}

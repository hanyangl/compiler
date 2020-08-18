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
  NUMBER,

  KEYWORD(keywords::Keywords),
  SIGN(signs::Signs),
  TYPE(types::Types),

  EOL,
  EOF,
}

impl Tokens {
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
}

pub trait TokenType {
  /// Get a new token.
  fn new(token: Self) -> Box<Tokens>;

  /// Get a new token from a string value.
  fn from_value(value: String) -> Option<Box<Tokens>>;
}

mod keywords;
mod signs;
mod token;
mod types;

pub use keywords::*;
pub use signs::*;
pub use token::*;
pub use types::*;

#[derive(Debug, Clone, PartialEq)]
pub enum Tokens {
  ILLEGAL,

  STRING,
  IDENTIFIER,
  NUMBER,

  KEYWORD(Keywords),
  SIGN(Signs),
  TYPE(Types),

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
  pub fn get_keyword(self) -> Option<Keywords> {
    match self {
      Tokens::KEYWORD(keyword) => Some(keyword),
      _ => None,
    }
  }

  /// Check if the token is a specific keyword token.
  pub fn expect_keyword(self, expect: Keywords) -> bool {
    match self {
      Tokens::KEYWORD(keyword) => keyword == expect,
      _ => false,
    }
  }

  /// Get the sign token.
  pub fn get_sign(self) -> Option<Signs> {
    match self {
      Tokens::SIGN(sign) => Some(sign),
      _ => None,
    }
  }

  /// Check if the token is a specific sign token.
  pub fn expect_sign(self, expect: Signs) -> bool {
    match self {
      Tokens::SIGN(sign) => sign == expect,
      _ => false,
    }
  }

  /// Get the type token.
  pub fn get_type(self) -> Option<Types> {
    match self {
      Tokens::TYPE(data_type) => Some(data_type),
      _ => None,
    }
  }

  /// Check if the token is a specific type token.
  pub fn expect_type(self, expect: Types) -> bool {
    match self {
      Tokens::TYPE(data_type) => data_type == expect,
      _ => false,
    }
  }
}

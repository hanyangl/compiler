mod keywords;
pub use keywords::Keywords;
pub use keywords::get_keyword;

mod signs;
pub use signs::Signs;
pub use signs::get_sign;

mod tokens;
pub use tokens::Tokens;

mod types;
pub use types::Types;
pub use types::get_type;

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
  pub token: tokens::Tokens,
  pub keyword: keywords::Keywords,
  pub sign: signs::Signs,
  pub data_type: types::Types,
  pub value: String,

  pub position: usize,
  pub line: usize,
}

impl Token {
  pub fn empty() -> Token {
    Token {
      token: tokens::Tokens::ILLEGAL,
      keyword: keywords::Keywords::NONE,
      sign: signs::Signs::NONE,
      data_type: types::Types::NONE,
      value: String::new(),
      position: 1,
      line: 1,
    }
  }

  /// Create a new token data using the token type and the string value.
  /// Example: `new(crate::data::Tokens::STRING, String::from("'Sflyn'"), 1, 1)`
  pub fn new(
    token: tokens::Tokens,
    value: String,
    position: usize,
    line: usize,
  ) -> Token {
    let mut return_token = Token::empty();

    return_token.token = token;
    return_token.value = value;
    return_token.position = position;
    return_token.line = line;

    return_token
  }

  /// Get the Token data from a string value.
  /// Example: `from_value(String::from("let"), 1, 1)`
  pub fn from_value(
    value: String,
    position: usize,
    line: usize,
  ) -> Token {
    let mut return_token = Token::empty();

    return_token.value = value;
    return_token.position = position;
    return_token.line = line;

    return_token.fetch();

    return_token
  }

  /// Fetch the string to token type.
  fn fetch(&mut self) {
    self.keyword = keywords::get_keyword(&self.value);
    if self.keyword != keywords::Keywords::NONE {
      self.token = tokens::Tokens::KEYWORD;
      return;
    }

    self.sign = signs::get_sign(&self.value);
    if self.sign != signs::Signs::NONE {
      self.token = tokens::Tokens::SIGN;
      return;
    }

    self.data_type = types::get_type(&self.value);
    if self.data_type != types::Types::NONE {
      self.token = tokens::Tokens::TYPE;
      return;
    }

    self.token = match self.value.as_str() {
      "\n" => tokens::Tokens::EOL,
      _ => tokens::Tokens::ILLEGAL,
    }
  }
}

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

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Token {
  pub token: tokens::Tokens,
  pub keyword: keywords::Keywords,
  pub sign: signs::Signs,
  pub data_type: types::Types,
  pub value: String,
}

impl Token {
  pub fn new(token: tokens::Tokens, value: String) -> Token {
    Token {
      token,
      keyword: keywords::Keywords::NONE,
      sign: signs::Signs::NONE,
      data_type: types::Types::NONE,
      value,
    }
  }

  pub fn from_value(value: String) -> Token {
    let mut token = Token {
      token: tokens::Tokens::ILLEGAL,
      keyword: keywords::Keywords::NONE,
      sign: signs::Signs::NONE,
      data_type: types::Types::NONE,
      value,
    };

    token.fetch();

    token
  }

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

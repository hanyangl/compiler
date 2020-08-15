use crate::data::{Token, Tokens, Types};
use crate::expressions::{Expression, Object, Hashable, HashKey, ObjectType};
use crate::parser::Parser;

// EXPRESSION //
#[derive(Debug, Clone)]
pub struct Boolean {
  pub token: Token,
  value: bool,
}

impl Expression for Boolean {
  fn new() -> Boolean {
    Boolean {
      token: Token::empty(),
      value: false,
    }
  }

  fn from_token(token: &Token) -> Boolean {
    Boolean {
      token: token.clone(),
      value: token.token == Tokens::TYPE && token.data_type == Types::TRUE,
    }
  }

  fn string(self) -> String {
    self.token.value
  }
}
// END EXPRESSION //


// PARSE //
pub fn parse<'a>(parser: &'a mut Parser) -> Boolean {
  Expression::from_token(&parser.current_token.clone())
}
// END PARSE //


// OBJECT //
#[derive(Debug, Clone)]
pub struct BooleanObject {
  value: bool,
}

impl Object for BooleanObject {
  fn object_type(&self) -> ObjectType {
    ObjectType::BOOLEAN
  }

  fn string(self) -> String {
    self.value.to_string()
  }
}

impl Hashable for BooleanObject {
  fn hashkey(self) -> HashKey {
    let value: u64;

    if self.value == true {
      value = 1;
    } else {
      value = 0;
    }

    HashKey {
      object_type: self.object_type(),
      value,
    }
  }
}
// END OBJECT //

use crate::data::Token;
use crate::expressions::{Expression, Object, ObjectType, Hashable, HashKey};
use crate::parser::Parser;

// EXPRESSION //
#[derive(Debug, Clone)]
pub struct Integer {
  pub token: Token,
  value: i64,
}

impl Expression for Integer {
  fn new() -> Integer {
    Integer {
      token: Token::empty(),
      value: 0,
    }
  }

  fn from_token(token: &Token) -> Integer {
    Integer {
      token: token.clone(),
      value: 0,
    }
  }

  fn string(self) -> String {
    self.token.value
  }
}
// END EXPRESSION //


// PARSER //
pub fn parse<'a>(parser: &'a mut Parser) -> Option<Integer> {
  let token: &Token = &parser.current_token.clone();
  let mut statement: Integer = Expression::from_token(&token.clone());

  match token.value.parse::<i64>() {
    Ok(value) => {
      statement.value = value;
      Some(statement)
    },
    Err(_) => {
      parser.errors.push(
        format!(
          "[{}:{}] Could not parse {} as integer",
          token.line,
          token.position,
          token.value,
        ),
      );

      None
    }
  }
}
// END PARSER //


// OBJECT //
#[derive(Debug, Clone)]
pub struct IntegerObject {
  value: i64,
}

impl Object for IntegerObject {
  fn object_type(&self) -> ObjectType {
    ObjectType::INTEGER
  }

  fn string(self) -> String {
    self.value.to_string()
  }
}

impl Hashable for IntegerObject {
  fn hashkey(self) -> HashKey {
    HashKey {
      object_type: self.object_type(),
      value: self.string().parse().unwrap(),
    }
  }
}
// END OBJECT //

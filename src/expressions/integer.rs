use crate::data::Token;
use crate::parser::Parser;

use super::Expression;

// EXPRESSION //
#[derive(Debug, Clone, PartialEq)]
pub struct Integer {
  pub token: Token,
  pub value: i64,
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

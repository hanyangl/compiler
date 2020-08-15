use crate::data::Token;
use crate::expressions::Expression;
use crate::parser::Parser;

#[derive(Debug, Clone)]
pub struct Integer {
  token: Token,
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

pub fn parse<'a>(parser: &'a mut Parser) -> (Integer, bool) {
  let token: &Token = &parser.current_token.clone();
  let mut statement: Integer = Expression::from_token(&token.clone());

  match token.value.parse::<i64>() {
    Ok(value) => {
      statement.value = value;
      (statement, true)
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

      (Expression::new(), false)
    }
  }
}

use crate::{
  Error,
  Parser,
  tokens::Token,
};

use super::{
  Expression,
  Expressions,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Number {
  pub token: Token,
  pub value: f64,
}

impl Expression for Number {
  fn new() -> Number {
    Number {
      token: Token::new_empty(),
      value: 0.0,
    }
  }

  fn from_token(token: Token) -> Number {
    Number {
      token,
      value: 0.0,
    }
  }

  fn string(self) -> String {
    self.token.value
  }
}

impl Number {
  pub fn new_box() -> Box<Expressions> {
    Box::new(Expressions::NUMBER(Expression::new()))
  }

  pub fn new_box_from_token(token: Token) -> Box<Expressions> {
    let mut number: Number = Expression::from_token(token.clone());

    if let Ok(value) = token.value.parse::<f64>() {
      number.value = value;
    }

    Box::new(Expressions::NUMBER(number))
  }

  pub fn parse<'a>(parser: &'a mut Parser) -> Result<Box<Expressions>, Error> {
    let mut number: Number = Expression::from_token(parser.current_token.clone());

    match parser.current_token.value.clone().parse::<f64>() {
      Ok(value) => {
        number.value = value;
        Ok(Box::new(Expressions::NUMBER(number)))
      },
      Err(_) => Err(Error::from_token(
        format!("could not parse `{}` as integer.", parser.current_token.value.clone()),
        parser.current_token.clone(),
      )),
    }
  }
}

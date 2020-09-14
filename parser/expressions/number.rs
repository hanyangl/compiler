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
  token: Token,
  value: f64,
}

impl Expression for Number {
  fn new() -> Self {
    Self {
      token: Token::new_empty(),
      value: 0.0,
    }
  }

  fn from_token(token: Token) -> Self {
    Self {
      token,
      value: 0.0,
    }
  }

  fn get_token(&self) -> Token {
    self.token.clone()
  }

  fn string(&self) -> String {
    self.get_token().value
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

  pub fn get_value(&self) -> f64 {
    self.value.clone()
  }

  pub fn parse<'a>(parser: &'a mut Parser) -> Result<Box<Expressions>, Error> {
    let mut number: Number = Expression::from_token(parser.get_current_token());

    match parser.get_current_token().value.parse::<f64>() {
      Ok(value) => {
        number.value = value;
        Ok(Box::new(Expressions::NUMBER(number)))
      },
      Err(_) => Err(Error::from_token(
        format!("could not parse `{}` as integer.", parser.get_current_token().value),
        parser.get_current_token(),
      )),
    }
  }
}

use crate::{
  Error,
  Parser,
  Precedence,
  tokens::Token,
};

use super::{
  Expression,
  Expressions,
  Identifier,
  parse_expression,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Prefix {
  token: Token,
  right: Box<Expressions>,
}

impl Expression for Prefix {
  fn new() -> Self {
    Self {
      token: Token::new_empty(),
      right: Identifier::new_box(),
    }
  }

  fn from_token(token: Token) -> Self {
    Self {
      token,
      right: Identifier::new_box(),
    }
  }

  fn get_token(&self) -> Token {
    self.token.clone()
  }

  fn string(&self) -> String {
    format!(
      "{}{}",
      self.get_token().value,
      self.get_right().string(),
    )
  }
}

impl Prefix {
  pub fn get_right(&self) -> Box<Expressions> {
    self.right.clone()
  }

  pub fn parse<'a>(
    parser: &'a mut Parser,
    standard_library: bool,
    with_this: bool,
  ) -> Result<Box<Expressions>, Error> {
    let mut prefix: Prefix = Expression::from_token(parser.get_current_token());

    // Get the next token.
    parser.next_token();

    // Parse the right expression.
    match parse_expression(parser, Precedence::PREFIX, standard_library, with_this) {
      Ok(right) => {
        prefix.right = right;
      },
      Err(error) => {
        return Err(error);
      },
    }

    // Return the prefix expression.
    Ok(Box::new(Expressions::PREFIX(prefix)))
  }
}

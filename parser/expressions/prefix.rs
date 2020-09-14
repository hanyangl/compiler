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
  pub token: Token,
  pub operator: String,
  pub right: Box<Expressions>,
}

impl Expression for Prefix {
  fn new() -> Prefix {
    Prefix {
      token: Token::new_empty(),
      operator: String::new(),
      right: Identifier::new_box(),
    }
  }

  fn from_token(token: Token) -> Prefix {
    Prefix {
      token: token.clone(),
      operator: token.value,
      right: Identifier::new_box(),
    }
  }

  fn string(&self) -> String {
    format!(
      "{}{}",
      self.operator,
      self.right.string(),
    )
  }
}

impl Prefix {
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

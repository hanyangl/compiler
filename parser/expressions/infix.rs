use crate::{
  Error,
  Parser,
  tokens::{
    Keywords,
    Signs,
    Token,
  },
};

use super::{
  Expression,
  Expressions,
  Identifier,
  parse_expression,
};

#[derive(Debug, Clone, PartialEq)]
pub enum InfixType {
  INFIX,
  ALIAS,
  METHOD,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Infix {
  pub token: Token,
  pub itype: InfixType,
  pub left: Box<Expressions>,
  pub operator: String,
  pub right: Box<Expressions>,
}

impl Expression for Infix {
  fn new() -> Infix {
    Infix {
      token: Token::new_empty(),
      itype: InfixType::INFIX,
      left: Identifier::new_box(),
      operator: String::new(),
      right: Identifier::new_box(),
    }
  }

  fn from_token(token: Token) -> Infix {
    Infix {
      token: token.clone(),
      itype: InfixType::INFIX,
      left: Identifier::new_box(),
      operator: token.value,
      right: Identifier::new_box(),
    }
  }

  fn string(self) -> String {
    let whitespace = if self.clone().is_method() { "" } else { " " };

    format!(
      "{}{}{}{}{}",
      self.left.string(),
      whitespace,
      self.operator,
      whitespace,
      self.right.string(),
    )
  }
}

impl Infix {
  pub fn is_infix(self) -> bool {
    self.itype == InfixType::INFIX
  }

  pub fn is_alias(self) -> bool {
    self.itype == InfixType::ALIAS
  }

  pub fn is_method(self) -> bool {
    self.itype == InfixType::METHOD
  }

  pub fn new_box() -> Box<Expressions> {
    Box::new(Expressions::INFIX(Expression::new()))
  }

  pub fn new_box_from_token(token: Token) -> Box<Expressions> {
    Box::new(Expressions::INFIX(Expression::from_token(token)))
  }

  pub fn parse<'a>(
    parser: &'a mut Parser,
    left_expression: Box<Expressions>,
    standard_library: bool,
    with_this: bool,
  ) -> Result<Box<Expressions>, Error> {
    let mut infix: Infix = Expression::from_token(parser.current_token.clone());

    // Check if it is an alias expression.
    if parser.current_token_is(Keywords::new(Keywords::AS)) {
      infix.itype = InfixType::ALIAS;
    }
    // Check if it is a method expression.
    else if parser.current_token_is(Signs::new(Signs::ARROW)) {
      infix.itype = InfixType::METHOD;
    }

    // Set the left expression.
    infix.left = left_expression;

    // Get the current precedence.
    let precedence = parser.current_precedence();

    // Get the next token.
    parser.next_token();

    // Set the right expression.
    match parse_expression(parser, precedence, standard_library, with_this) {
      Ok(right) => {
        infix.right = right;
      },
      Err(error) => {
        return Err(error);
      },
    }

    // Return the infix expression.
    Ok(Box::new(Expressions::INFIX(infix)))
  }
}

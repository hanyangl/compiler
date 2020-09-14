use crate::{
  Error,
  Expressions,
  Identifier,
  parse_expression,
  Parser,
  Precedence,
  tokens::Token,
};

use super::{
  Statement,
  Statements,
};

#[derive(Debug, Clone, PartialEq)]
pub struct ExpressionStatement {
  token: Token,
  expression: Box<Expressions>,
}

impl Statement for ExpressionStatement {
  fn new() -> ExpressionStatement {
    ExpressionStatement {
      token: Token::new_empty(),
      expression: Identifier::new_box(),
    }
  }

  fn from_token(token: Token) -> ExpressionStatement {
    ExpressionStatement {
      token,
      expression: Identifier::new_box(),
    }
  }

  fn get_token(&self) -> Token {
    self.token.clone()
  }

  fn string(&self) -> String {
    self.get_expression().string()
  }
}

impl ExpressionStatement {
  pub fn new_box() -> Box<Statements> {
    Box::new(Statements::EXPRESSION(Statement::new()))
  }

  pub fn get_expression(&self) -> Box<Expressions> {
    self.expression.clone()
  }

  pub fn parse<'a>(
    parser: &'a mut Parser,
    standard_library: bool,
    with_this: bool,
  ) -> Result<Box<Statements>, Error> {
    let mut statement: ExpressionStatement = Statement::from_token(parser.get_current_token());

    // Parse expression.
    match parse_expression(parser, Precedence::LOWEST, standard_library, with_this) {
      Ok(expression) => {
        statement.expression = expression;
      },
      Err(error) => {
        return Err(error);
      },
    }

    // Return statement.
    Ok(Box::new(Statements::EXPRESSION(statement)))
  }
}

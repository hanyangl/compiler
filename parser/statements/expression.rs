use crate::Environment;
use crate::expressions::{Expressions, parse as parse_expression};
use crate::{Parser, Precedence};
use crate::tokens::{Token, Signs};

use super::{Statement, Statements};

#[derive(Debug, Clone, PartialEq)]
pub struct ExpressionStatement {
  pub token: Token,
  pub expression: Option<Box<Expressions>>,
}

impl Statement for ExpressionStatement {
  fn new() -> ExpressionStatement {
    ExpressionStatement {
      token: Token::new_empty(),
      expression: None,
    }
  }

  fn from_token(token: Token) -> ExpressionStatement {
    ExpressionStatement {
      token,
      expression: None,
    }
  }

  fn string(self) -> String {
    match self.expression {
      Some(exp) => exp.string(),
      None => String::new(),
    }
  }
}

impl ExpressionStatement {
  pub fn new_box() -> Box<Statements> {
    Box::new(Statements::EXPRESSION(Statement::new()))
  }

  pub fn parse<'a>(parser: &'a mut Parser, environment: &mut Environment) -> Box<Statements> {
    let mut statement: ExpressionStatement = Statement::from_token(parser.current_token.clone());

    // Parse expression.
    statement.expression = parse_expression(parser, Precedence::LOWEST, environment);

    // Check if the next token is a semicolon.
    if parser.next_token.token.clone().is_sign(Signs::SEMICOLON) {
      // Get the next token.
      parser.next_token();
    }

    // Return statement.
    Box::new(Statements::EXPRESSION(statement))
  }
}

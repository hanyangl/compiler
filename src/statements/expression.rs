use crate::compiler::environment::Environment;
use crate::data::{Token, Signs};
use crate::expressions::{Expressions, parse as expression_parse};
use crate::parser::{Parser, precedence::Precedence};

use super::Statement;

// EXPRESSION //
#[derive(Debug, Clone, PartialEq)]
pub struct ExpressionStatement {
  pub token: Token,
  pub expression: Option<Box<Expressions>>,
}

impl Statement for ExpressionStatement {
  fn new() -> ExpressionStatement {
    ExpressionStatement {
      token: Token::empty(),
      expression: None,
    }
  }

  fn from_token(token: &Token) -> ExpressionStatement {
    let mut expression: ExpressionStatement = Statement::new();

    expression.token = token.clone();

    expression
  }

  fn string(self) -> String {
    match self.expression {
      Some(x) => x.string(),
      None => "".to_string(),
    }
  }
}
// END EXPRESSION //


// PARSER //
pub fn parse<'a>(parser: &'a mut Parser, env: &mut Environment) -> ExpressionStatement {
  let mut statement: ExpressionStatement = Statement::from_token(&parser.current_token.clone());

  statement.expression = expression_parse(parser, Precedence::LOWEST, env);

  if parser.peek_token_is_sign(&Signs::SEMICOLON) == true {
    parser.next_token();
  }

  statement
}

pub fn parse_list<'a>(parser: &'a mut Parser, end: Signs, env: &mut Environment) -> Vec<Box<Expressions>> {
  let mut list: Vec<Box<Expressions>> = Vec::new();

  while !parser.peek_token_is_sign(&end) {
    if parser.current_token_is_sign(&Signs::COMMA) || parser.current_token_is_sign(&Signs::LEFTPARENTHESES) {
      parser.next_token();
    }

    match expression_parse(parser, Precedence::LOWEST, env) {
      Some(exp) => list.push(exp),
      None => {},
    }

    if !parser.peek_token_is_sign(&end) {
      parser.next_token();
    }
  }

  list
}
// END PARSER //

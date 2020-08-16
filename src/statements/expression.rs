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
pub fn parse<'a>(parser: &'a mut Parser) -> ExpressionStatement {
  let mut statement: ExpressionStatement = Statement::from_token(&parser.current_token.clone());

  statement.expression = expression_parse(parser, Precedence::LOWEST);

  if parser.peek_token_is_sign(&Signs::SEMICOLON) == true {
    parser.next_token();
  }

  statement
}

pub fn parse_list<'a>(parser: &'a mut Parser, end: Signs) -> Vec<Box<Expressions>> {
  let mut list: Vec<Box<Expressions>> = Vec::new();

  while parser.current_token_is_sign(&end) == false {
    if parser.peek_token_is_sign(&Signs::COMMA) == true {
      parser.next_token();
    }

    match expression_parse(parser, Precedence::LOWEST) {
      Some(exp) => list.push(exp),
      None => {},
    }

    parser.next_token();
  }

  list
}
// END PARSER //

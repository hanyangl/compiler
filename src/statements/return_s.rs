use crate::data::{Token, Signs};
use crate::expressions::{Expressions, parse as expression_parse};
use crate::parser::{Parser, precedence::Precedence};
use crate::statements::Statement;

// EXPRESSION //
#[derive(Debug, Clone)]
pub struct Return {
  token: Token,
  pub value: Option<Box<Expressions>>,
}

impl Statement for Return {
  fn new() -> Return {
    Return {
      token: Token::empty(),
      value: None,
    }
  }

  fn from_token(token: &Token) -> Return {
    let mut statement: Return = Statement::new();

    statement.token = token.clone();

    statement
  }

  fn string(self) -> String {
    format!(
      "{}{};",
      self.token.value,
      match self.value {
        Some(x) => format!(" {}", x.string()),
        None => "".to_string(),
      },
    )
  }
}
// END EXPRESSION //


// PARSER //
pub fn parse<'a>(parser: &'a mut Parser) -> Return {
  let mut statement: Return = Statement::from_token(&parser.current_token.clone());

  parser.next_token();

  statement.value = expression_parse(parser, Precedence::LOWEST);

  if parser.peek_token_is_sign(&Signs::SEMICOLON) == true {
    parser.next_token();
  }

  statement
}
// END PARSER //

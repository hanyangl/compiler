use crate::data::{Token, Signs, Tokens};
use crate::parser::Parser;
use crate::statements::{Statement, Statements};

// STATEMENT //
#[derive(Debug, Clone)]
pub struct Block {
  token: Token,
  pub statements: Vec<Box<Statements>>,
}

impl Statement for Block {
  fn new() -> Block {
    Block {
      token: Token::empty(),
      statements: Vec::new(),
    }
  }

  fn from_token(token: &Token) -> Block {
    let mut statement: Block = Statement::new();

    statement.token = token.clone();

    statement
  }

  fn string(self) -> String {
    let mut string = String::new();

    for statement in self.statements {
      string.push_str(statement.string().as_str());
    }

    string
  }
}
// END STATEMENT //


// PARSER //
pub fn parse<'a>(parser: &'a mut Parser) -> Block {
  let mut statement: Block = Statement::from_token(&parser.current_token.clone());

  parser.next_token();

  while parser.current_token_is_sign(&Signs::RIGHTBRACE) == false && parser.current_token_is(&Tokens::EOF) == false {
    match parser.parse_statement() {
      Some(x) => statement.statements.push(x),
      None => {},
    }

    parser.next_token();
  }

  statement
}
// END PARSER //

use crate::{
  Error,
  parse_statement,
  Parser,
  tokens::{
    Signs,
    Token,
  },
};

use super::{
  ExpressionStatement,
  Statement,
  Statements,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Export {
  pub token: Token,
  pub value: Box<Statements>,
}

impl Statement for Export {
  fn new() -> Export {
    Export {
      token: Token::new_empty(),
      value: ExpressionStatement::new_box(),
    }
  }

  fn from_token(token: Token) -> Export {
    let mut export: Export = Export::new();

    export.token = token;

    export
  }

  fn string(self) -> String {
    format!(
      "{} {};",
      self.token.value,
      self.value.string(),
    )
  }
}

impl Export {
  pub fn parse<'a>(
    parser: &'a mut Parser,
    standard_library: bool,
  ) -> Result<Box<Statements>, Error> {
    let mut export: Export = Statement::from_token(parser.current_token.clone());

    // Get the next token.
    parser.next_token();

    // Parse statement.
    match parse_statement(parser, standard_library, false, false) {
      Ok(value) => {
        export.value = value;
      },
      Err(error) => {
        return Err(error);
      },
    }

    // Check if the next token is a semicolon.
    if parser.next_token_is(Signs::new(Signs::SEMICOLON)) {
      // Get the next token.
      parser.next_token();
    }

    Ok(Box::new(Statements::EXPORT(export)))
  }
}

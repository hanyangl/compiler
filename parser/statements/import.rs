use crate::{
  Error,
  Expressions,
  parse_expression,
  Parser,
  Precedence,
  StringE,
  tokens::{
    Keywords,
    Signs,
    Token,
    Tokens,
  },
};

use super::{
  Statement,
  Statements,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Import {
  pub token: Token,
  pub modules: Vec<Box<Expressions>>,
  pub path: Box<Expressions>,
}

impl Statement for Import {
  fn new() -> Import {
    Import {
      token: Token::new_empty(),
      modules: Vec::new(),
      path: StringE::new_box(),
    }
  }

  fn from_token(token: Token) -> Import {
    let mut import: Import = Statement::new();

    import.token = token;

    import
  }

  fn string(self) -> String {
    let mut modules: Vec<String> = Vec::new();

    for module in self.modules {
      modules.push(module.string());
    }

    if modules.len() == 0 {
      format!(
        "{} {};",
        self.token.value,
        self.path.string(),
      )
    } else if modules.len() == 1 {
      format!(
        "{} {} from {};",
        self.token.value,
        modules[0].clone(),
        self.path.string(),
      )
    } else {
      format!(
        "{} {{\n\t{}\n}} from {};",
        self.token.value,
        modules.join(", "),
        self.path.string(),
      )
    }
  }
}

impl Import {
  pub fn parse<'a>(
    parser: &'a mut Parser,
    standard_library: bool,
    with_this: bool,
  ) -> Result<Box<Statements>, Error> {
    let mut import: Import = Statement::from_token(parser.current_token.clone());
    let mut required_from = false;

    // Check if the next token is a left brace.
    if parser.expect_token(Signs::new(Signs::LEFTBRACE)) {
      // Get the next token.
      parser.next_token();

      while !parser.current_token_is(Signs::new(Signs::RIGHTBRACE)) {
        // Parse expression.
        match parse_expression(parser, Precedence::LOWEST, standard_library, with_this) {
          Ok(expression) => {
            import.modules.push(expression);
          },
          Err(error) => {
            return Err(error);
          }
        }

        // Check if the next token is a comma.
        if parser.next_token_is(Signs::new(Signs::COMMA)) {
          // Get the next token.
          parser.next_token();
        }

        // Get the next token.
        parser.next_token();
      }

      required_from = true;
    }
    // Check if the next token is not a string.
    else if !parser.next_token_is(Box::new(Tokens::STRING)) {
      // Get the next token.
      parser.next_token();

      match parse_expression(parser, Precedence::LOWEST, standard_library, with_this) {
        Ok(expression) => {
          import.modules.push(expression);
        },
        Err(error) => {
          return Err(error);
        },
      }

      required_from = true;
    }

    // Check if the next token is `from` when it's required.
    if required_from && !parser.expect_token(Keywords::new(Keywords::FROM)) {
      return Err(Error::from_token(
        format!("expect `from`, got `{}` instead.", parser.next_token.value.clone()),
        parser.next_token.clone(),
      ));
    }

    // Check if the next token is a string.
    if !parser.expect_token(Box::new(Tokens::STRING)) {
      return Err(Error::from_token(
        format!("`{}` is not a valid string.", parser.next_token.value.clone()),
        parser.next_token.clone(),
      ));
    }

    import.path = StringE::new_box_from_token(parser.current_token.clone());

    // Check if the next token is a semicolon.
    if parser.next_token_is(Signs::new(Signs::SEMICOLON)) {
      // Get the next token.
      parser.next_token();
    }

    Ok(Box::new(Statements::IMPORT(import)))
  }
}

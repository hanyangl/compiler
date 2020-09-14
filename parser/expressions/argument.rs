use crate::{
  Error,
  Parser,
  Precedence,
  tokens::{
    Signs,
    Token,
    Tokens,
  },
};

use super::{
  Expression,
  Expressions,
  parse_expression,
  parse_type,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Argument {
  pub token: Token,
  pub data_type: Token,
  pub value: Option<Box<Expressions>>
}

impl Expression for Argument {
  fn new() -> Argument {
    Argument {
      token: Token::new_empty(),
      data_type: Token::new_empty(),
      value: None,
    }
  }

  fn from_token(token: Token) -> Argument {
    Argument {
      token,
      data_type: Token::new_empty(),
      value: None,
    }
  }

  fn string(&self) -> String {
    let argument = format!(
      "{}: {}",
      self.token.value,
      self.data_type.value,
    );

    match self.value.clone() {
      Some(value) => format!("{} = {}", argument, value.string()),
      None => argument,
    }
  }
}

impl Argument {
  pub fn new_box() -> Box<Expressions> {
    Box::new(Expressions::ARGUMENT(Expression::new()))
  }

  pub fn new_box_from_token(token: Token) -> Box<Expressions> {
    Box::new(Expressions::ARGUMENT(Expression::from_token(token)))
  }

  pub fn has_default_value(&self) -> bool {
    self.value.is_some()
  }

  pub fn parse<'a>(
    parser: &'a mut Parser,
    standard_library: bool,
    with_this: bool,
  ) -> Result<Vec<Box<Expressions>>, Error> {
    let mut arguments: Vec<Box<Expressions>> = Vec::new();

    // Check if the next token is a right parentheses.
    if parser.next_token_is(Signs::new(Signs::RIGHTPARENTHESES)) {
      // Get the next token.
      parser.next_token();
    }

    let mut has_default = false;
    while !parser.current_token_is(Signs::new(Signs::RIGHTPARENTHESES)) {
      // Check if the next token is an identifier.
      if !parser.expect_token(Box::new(Tokens::IDENTIFIER)) {
        return Err(Error::from_token(
          String::from("is not a valid identifier."),
          parser.get_next_token(),
        ));
      }

      let mut argument: Argument = Expression::from_token(parser.get_current_token());

      // Check if the next token is a colon.
      if !parser.expect_token(Signs::new(Signs::COLON)) {
        return Err(Error::from_token(
          format!("expect `:`, got `{}` instead.", parser.get_next_token().value),
          parser.get_next_token(),
        ));
      }

      // Get the next token.
      parser.next_token();

      // Parse argument data type.
      match parse_type(parser) {
        Ok(data_type) => {
          // Set the argument data type.
          argument.data_type = data_type;
        },
        Err(_) => {
          return Err(Error::from_token(
            String::from("is not a valid data type."),
            parser.get_current_token(),
          ));
        },
      }

      // Check if the next token is an assign sign.
      if parser.expect_token(Signs::new(Signs::ASSIGN)) {
        has_default = true;

        // Get the next token.
        parser.next_token();

        // Parse default value expression.
        match parse_expression(parser, Precedence::LOWEST, standard_library, with_this) {
          Ok(value) => {
            argument.value = Some(value);
          },
          Err(error) => {
            return Err(error);
          },
        }
      } else if has_default {
        return Err(Error::from_token(
          String::from("the argument must has a default value."),
          parser.get_next_token(),
        ));
      }

      // Check if the next token is a comma.
      if parser.next_token_is(Signs::new(Signs::COMMA)) {
        // Get the next token.
        parser.next_token();
      }

      // Check if the next token is a right parentheses.
      if parser.next_token_is(Signs::new(Signs::RIGHTPARENTHESES)) {
        // Get the next token.
        parser.next_token();
      }

      // Add the argument to the list.
      arguments.push(Box::new(Expressions::ARGUMENT(argument.clone())));
    }

    // Return arguments.
    Ok(arguments)
  }
}

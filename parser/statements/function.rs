use crate::{
  Argument,
  Error,
  Expressions,
  Parser,
  tokens::*,
};

use super::{
  Block,
  Statement,
  Statements,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Function {
  pub token: Token,
  pub name: Token,
  pub arguments: Vec<Box<Expressions>>,
  pub data_type: Token,
  pub body: Box<Statements>,
}

impl Statement for Function {
  fn new() -> Function {
    Function {
      token: Token::new_empty(),
      name: Token::new_empty(),
      arguments: Vec::new(),
      data_type: Token::from_value("void", 0, 0),
      body: Block::new_box(),
    }
  }

  fn from_token(token: Token) -> Function {
    let mut function: Function = Statement::new();

    function.token = token;

    function
  }

  fn string(self) -> String {
    let mut arguments: Vec<String> = Vec::new();

    for argument in self.arguments {
      arguments.push(argument.string());
    }

    format!(
      "function {}({}): {} {}",
      self.name.value,
      arguments.join(", "),
      self.data_type.value,
      self.body.string(),
    )
  }
}

impl Function {
  pub fn parse<'a>(
    parser: &'a mut Parser,
    standard_library: bool,
    with_this: bool,
  ) -> Result<Box<Statements>, Error> {
    let mut function: Function = Statement::from_token(parser.current_token.clone());

    // Check if the next token is a valid identifier.
    if !parser.expect_token(Box::new(Tokens::IDENTIFIER)) {
      let mut message = format!("`{}` is not a valid function name.", parser.next_token.value.clone());

      if parser.next_token_is(Signs::new(Signs::LEFTPARENTHESES)) {
        message = String::from("you must enter the function name.");
      }

      return Err(Error::from_token(
        message,
        parser.next_token.clone(),
      ));
    }

    // Set the function name.
    function.name = parser.current_token.clone();

    // Check if the next token is a left parentheses.
    if !parser.expect_token(Signs::new(Signs::LEFTPARENTHESES)) {
      return Err(Error::from_token(
        format!("expect `(`, got `{}` instead.", parser.next_token.value.clone()),
        parser.next_token.clone(),
      ));
    }

    // Parse arguments.
    match Argument::parse(parser, standard_library, with_this) {
      Ok(arguments) => {
        function.arguments = arguments;
      },
      Err(error) => {
        return Err(error);
      },
    }

    // Check if the current token is a right parentheses.
    if parser.current_token_is(Signs::new(Signs::RIGHTPARENTHESES)) {
      // Get the next token.
      parser.next_token();
    }

    // Check if the current token is a colon.
    if parser.current_token_is(Signs::new(Signs::COLON)) {
      // Get the next token.
      parser.next_token();

      // Get the return data type.
      match parser.current_token.token.clone().get_type() {
        Some(_) => {
          // Set the function return data type.
          function.data_type = parser.current_token.clone();
        },
        None => {
          return Err(Error::from_token(
            format!("`{}` is not a valid type.", parser.current_token.value.clone()),
            parser.current_token.clone(),
          ));
        },
      }

      // Get the next token.
      parser.next_token();
    }

    // Check if the next token is a left brace.
    if !parser.current_token_is(Signs::new(Signs::LEFTBRACE)) {
      return Err(Error::from_token(
        format!("expect `{{`, got `{}` instead.", parser.current_token.value.clone()),
        parser.current_token.clone(),
      ));
    }

    // Parse body.
    match Block::parse(parser, standard_library, false, with_this) {
      Ok(body) => {
        function.body = body;
      },
      Err(error) => {
        return Err(error);
      },
    }

    // Return function statement.
    Ok(Box::new(Statements::FUNCTION(function.clone())))
  }
}

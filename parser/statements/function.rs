use crate::{
  Argument,
  Error,
  Expressions,
  parse_type,
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
  token: Token,
  name: Token,
  arguments: Vec<Box<Expressions>>,
  data_type: Token,
  body: Box<Statements>,
}

impl Statement for Function {
  fn new() -> Self {
    Self {
      token: Token::new_empty(),
      name: Token::new_empty(),
      arguments: Vec::new(),
      data_type: Token::from_value("void", 0, 0),
      body: Block::new_box(),
    }
  }

  fn from_token(token: Token) -> Self {
    let mut function: Self = Statement::new();

    function.token = token;

    function
  }

  fn get_token(&self) -> Token {
    self.token.clone()
  }

  fn string(&self) -> String {
    let mut arguments: Vec<String> = Vec::new();

    for argument in self.get_arguments().iter() {
      arguments.push(argument.string());
    }

    format!(
      "function {}({}): {} {}",
      self.get_name().value,
      arguments.join(", "),
      self.get_type().value,
      self.get_body().string(),
    )
  }
}

impl Function {
  pub fn get_name(&self) -> Token {
    self.name.clone()
  }

  pub fn get_type(&self) -> Token {
    self.data_type.clone()
  }

  pub fn get_arguments(&self) -> Vec<Box<Expressions>> {
    self.arguments.clone()
  }

  pub fn get_body(&self) -> Box<Statements> {
    self.body.clone()
  }

  pub fn parse<'a>(
    parser: &'a mut Parser,
    standard_library: bool,
    with_this: bool,
  ) -> Result<Box<Statements>, Error> {
    let mut function: Function = Statement::from_token(parser.get_current_token());

    // Check if the next token is a valid identifier.
    if !parser.expect_token(Box::new(Tokens::IDENTIFIER)) {
      let mut message = format!("`{}` is not a valid function name.", parser.get_next_token().value);

      if parser.next_token_is(Signs::new(Signs::LEFTPARENTHESES)) {
        message = String::from("you must enter the function name.");
      }

      return Err(Error::from_token(
        message,
        parser.get_next_token(),
      ));
    }

    // Set the function name.
    function.name = parser.get_current_token();

    // Check if the next token is a left parentheses.
    if !parser.expect_token(Signs::new(Signs::LEFTPARENTHESES)) {
      return Err(Error::from_token(
        format!("expect `(`, got `{}` instead.", parser.get_next_token().value),
        parser.get_next_token(),
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

    // Check if the current token is a colon.
    if parser.expect_token(Signs::new(Signs::COLON)) {
      // Get the next token.
      parser.next_token();

      // Get the return data type.
      match parse_type(parser) {
        Ok(data_type) => {
          // Set the function return data type.
          function.data_type = data_type;
        },
        Err(_) => {
          return Err(Error::from_token(
            format!("`{}` is not a valid type.", parser.get_current_token().value),
            parser.get_current_token(),
          ));
        },
      }
    }

    // Check if the next token is a left brace.
    if !parser.expect_token(Signs::new(Signs::LEFTBRACE)) {
      return Err(Error::from_token(
        format!("expect `{{`, got `{}` instead.", parser.get_next_token().value),
        parser.get_next_token(),
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

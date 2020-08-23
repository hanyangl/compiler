use crate::Environment;
use crate::expressions::{Expressions, Identifier, Argument};
use crate::Parser;
use crate::tokens::*;

use super::{Statements, Statement, Block};

#[derive(Debug, Clone, PartialEq)]
pub struct Function {
  pub token: Token,
  pub name: Box<Expressions>,
  pub arguments: Vec<Box<Expressions>>,
  pub data_type: Token,
  pub body: Box<Statements>,
}

impl Statement for Function {
  fn new() -> Function {
    Function {
      token: Token::new_empty(),
      name: Identifier::new_box(),
      arguments: Vec::new(),
      data_type: Token::from_value(String::from("void"), 0, 0),
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
      self.name.string(),
      arguments.join(", "),
      self.data_type.value,
      self.body.string(),
    )
  }
}

impl Function {
  pub fn parse<'a>(parser: &'a mut Parser, environment: &mut Environment) -> Option<Box<Statements>> {
    let mut function: Function = Statement::from_token(parser.current_token.clone());

    // Check if the next token is a valid identifier.
    if !parser.expect_token(Box::new(Tokens::IDENTIFIER)) {
      let line = parser.get_error_line_next_token();
      let mut message = format!("{} `{}` is not a valid function name.", line, parser.next_token.value.clone());

      if parser.next_token_is(Signs::new(Signs::LEFTPARENTHESES)) {
        message = format!("{} you must enter the function name.", line);
      }

      parser.errors.push(message);

      return None;
    }

    // Set the function name.
    function.name = Identifier::new_box_from_token(parser.current_token.clone());

    // Check if the name is used.
    if environment.has_expression(function.name.clone().string()) ||
      environment.has_statement(function.name.clone().string()) {
      let line = parser.get_error_line_current_token();

      parser.errors.push(format!("{} `{}` is already in use.", line, function.name.clone().string()));

      return None;
    }

    // Check if the next token is a left parentheses.
    if !parser.expect_token(Signs::new(Signs::LEFTPARENTHESES)) {
      let line = parser.get_error_line_next_token();

      parser.errors.push(format!("{} expect `(`, got `{}` instead.", line, parser.next_token.value));

      return None;
    }

    // Set a new environment for the function.
    let mut function_environment = Environment::from_environment(environment.clone());

    // Parse arguments.
    match Argument::parse(parser, &mut function_environment) {
      Some(arguments) => {
        function.arguments = arguments;
      },
      None => {
        return None;
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
          let line = parser.get_error_line_current_token();

          parser.errors.push(format!("{} `{}` is not a valid type.", line, parser.current_token.value));

          return None;
        },
      }

      // Get the next token.
      parser.next_token();
    }

    // Check if the next token is a left brace.
    if !parser.current_token_is(Signs::new(Signs::LEFTBRACE)) {
      let line = parser.get_error_line_current_token();

      parser.errors.push(format!("{} expect `{{`, got `{}` instead.", line, parser.current_token.value));

      return None;
    }

    // Parse body.
    function.body = Block::parse(parser, &mut function_environment);

    // Check if the current token is the end of line.
    if parser.current_token_is(Box::new(Tokens::EOL)) {
      // Get the next token.
      parser.next_token();
    }

    // Get the function box statement.
    let function_box = Box::new(Statements::FUNCTION(function.clone()));

    // Set the function to the enviroment.
    environment.set_statement(function.name.string(), function_box.clone());

    // Return function statement.
    Some(function_box)
  }
}

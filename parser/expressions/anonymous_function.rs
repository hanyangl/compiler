use crate::{Environment, Parser};
use crate::statements::{Statements, Block};
use crate::tokens::{Token, Keywords, Signs, Tokens, TokenType};

use super::{Expressions, Expression, Argument};

#[derive(Debug, Clone, PartialEq)]
pub struct AnonymousFunction {
  pub token: Token,
  pub arguments: Vec<Box<Expressions>>,
  pub data_type: Token,
  pub body: Box<Statements>,
}

impl Expression for AnonymousFunction {
  fn new() -> AnonymousFunction {
    AnonymousFunction {
      token: Token::new_empty(),
      arguments: Vec::new(),
      data_type: Token::from_value(String::from("void"), 0, 0),
      body: Block::new_box(),
    }
  }

  fn from_token(token: Token) -> AnonymousFunction {
    let mut function: AnonymousFunction = Expression::new();

    function.token = token;

    function
  }

  fn string(self) -> String {
    let mut arguments: Vec<String> = Vec::new();

    for argument in self.arguments {
      arguments.push(argument.string());
    }

    let function = format!(
      "({}): {} {}",
      arguments.join(", "),
      self.data_type.value,
      self.body.string(),
    );

    if self.token.token.clone().is_keyword(Keywords::FUNCTION) {
      return format!("{} {}", self.token.value, function);
    }

    function
  }
}

impl AnonymousFunction {
  pub fn parse<'a>(parser: &'a mut Parser, environment: &mut Environment) -> Option<Box<Expressions>> {
    let mut function: AnonymousFunction = Expression::from_token(parser.current_token.clone());

    // Check if the current token is a left parentheses.
    if !parser.current_token_is(Signs::new(Signs::LEFTPARENTHESES)) {
      // Get the next token.
      parser.next_token();
    }

    // Set a new environment for the function.
    let mut function_environment = Environment::from_environment(environment.clone());

    // Parse arguments.
    match Argument::parse(parser, &mut function_environment) {
      Some(arguments) => {
        function.arguments = arguments;
      },
      None => {},
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
    }

    // Check if the next token is a left brace.
    if !parser.expect_token(Signs::new(Signs::LEFTBRACE)) {
      let line = parser.get_error_line_next_token();

      parser.errors.push(format!("{} expect `{{`, got `{}` instead.", line, parser.next_token.value));

      return None;
    }

    // Parse body.
    function.body = Block::parse(parser, &mut function_environment);

    // Check if the current token is the end of line.
    if parser.current_token_is(Box::new(Tokens::EOL)) {
      // Get the next token.
      parser.next_token();
    }

    Some(Box::new(Expressions::ANONYMOUSFUNCTION(function)))
  }
}

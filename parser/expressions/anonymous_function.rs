use crate::{Environment, Parser};
use crate::statements::{Statements, Block};
use crate::tokens::{Token, Keywords, Signs, TokenType};

use super::{Expressions, Expression, Argument, Call};

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
      "({}): {}",
      arguments.join(", "),
      self.data_type.value,
    );

    if self.token.token.clone().expect_keyword(Keywords::FUNCTION) {
      return format!("{} {} {}", self.token.value, function, self.body.string());
    }

    format!("{} => {}", function, self.body.string())
  }
}

impl AnonymousFunction {
  pub fn parse<'a>(
    parser: &'a mut Parser,
    environment: &mut Environment,
    standard_library: bool,
  ) -> Option<Box<Expressions>> {
    let mut function: AnonymousFunction = Expression::from_token(parser.current_token.clone());

    // Check if the current token is a left parentheses.
    if !parser.current_token_is(Signs::new(Signs::LEFTPARENTHESES)) {
      // Get the next token.
      parser.next_token();
    }

    // Set a new environment for the function.
    let mut function_environment = Environment::from_environment(environment.clone());

    // Parse arguments.
    match Argument::parse(parser, &mut function_environment, standard_library) {
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

    // Check if the function token is a left parentheses.
    if function.token.token.clone().expect_sign(Signs::LEFTPARENTHESES) {
      // Check if the next token is an assign arrow sign.
      if !parser.current_token_is(Signs::new(Signs::ASSIGNARROW)) {
        let line = parser.get_error_line_current_token();
        parser.errors.push(format!("{} expect `=>`, got `{}` instead.", line, parser.current_token.value));
        return None;
      }

      // Get the next token.
      parser.next_token();
    }

    // Check if the next token is a left brace.
    if !parser.current_token_is(Signs::new(Signs::LEFTBRACE)) {
      let line = parser.get_error_line_current_token();

      parser.errors.push(format!("{} expect `{{`, got `{}` instead.", line, parser.current_token.value));

      parser.next_token();

      return None;
    }

    // Parse body.
    match Block::parse(parser, function.data_type.clone(), &mut function_environment, standard_library) {
      Some(block) => {
        function.body = block;
      },
      None => {
        return None;
      },
    }

    Some(Box::new(Expressions::ANONYMOUSFUNCTION(function)))
  }

  pub fn get_arguments<'a>(
    parser: &'a mut Parser,
    value: Box<Expressions>,
    call: Call,
  ) -> Option<(usize, usize, Vec<Token>, Token)> {
    let mut min_arguments: usize = 0;
    let mut max_arguments: usize = 0;
    let mut data_types: Vec<Token> = Vec::new();

    // Get the anonymous function.
    match value.clone().get_anonymous_function() {
      // Is an anonymous function.
      Some(anonymous_function) => {
        for argument_exp in anonymous_function.arguments.clone() {
          // Get argument expression.
          match argument_exp.get_argument() {
            Some(argument) => {
              // Add argument data type to the data types list.
              data_types.push(argument.data_type);

              // Check if the argument has a default value.
              match argument.value {
                // With default value.
                Some(_) => {
                  max_arguments += 1;
                },
                // Without default value.
                None => {
                  min_arguments += 1;
                  max_arguments += 1;
                },
              }
            },
            None => {
              println!("TODO(anonymous_function): Without arguments");
              return None;
            },
          }
        }

        return Some((min_arguments, max_arguments, data_types, anonymous_function.data_type));
      },

      // Is not an anonymous function.
      None => {
        let line = parser.get_error_line(
          call.token.line - 1,
          call.token.position - 1,
          call.token.value.len(),
        );

        parser.errors.push(format!("{} the identifier is not a function.", line));

        None
      },
    }
  }
}

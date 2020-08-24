use crate::{Environment, Parser, Precedence};
use crate::tokens::{Token, Signs, TokenType, Tokens, Types};

use super::{Expressions, Expression, parse as parse_expression};

#[derive(Debug, Clone, PartialEq)]
pub struct HashMapItem {
  pub key: String,
  pub data_type: Token,
  pub value: Box<Expressions>,
}

impl HashMapItem {
  pub fn string(self) -> String {
    format!("{}: {}", self.key, self.value.string())
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct HashMap {
  pub token: Token,
  pub data: Vec<HashMapItem>,
}

impl Expression for HashMap {
  fn new() -> HashMap {
    HashMap {
      token: Token::new_empty(),
      data: Vec::new(),
    }
  }

  fn from_token(token: Token) -> HashMap {
    let mut hashmap: HashMap = Expression::new();

    hashmap.token = token;

    hashmap
  }

  fn string(self) -> String {
    let mut values: Vec<String> = Vec::new();

    for data in self.data {
      values.push(data.string());
    }

    format!("{{\n{}\n}}", values.join(",\n"))
  }
}

impl HashMap {
  pub fn has_key(self, name: String) -> bool {
    let mut has = false;

    for data in self.data {
      if data.key == name {
        has = true;
        break;
      }
    }

    has
  }

  pub fn get_by_key(self, name: String) -> Option<HashMapItem> {
    let mut item: Option<HashMapItem> = None;

    for data in self.data {
      if data.key == name {
        item = Some(data);
        break;
      }
    }

    item
  }

  pub fn parse<'a>(parser: &'a mut Parser, environment: &mut Environment) -> Option<Box<Expressions>> {
    let mut hashmap: HashMap = Expression::from_token(parser.current_token.clone());

    while !parser.current_token_is(Signs::new(Signs::RIGHTBRACE)) {
      // Check if the next token is an identifier or a string.
      if !parser.expect_token(Box::new(Tokens::IDENTIFIER)) {
        let line = parser.get_error_line_next_token();

        parser.errors.push(format!("{} is not a valid hashmap key.", line));

        return None;
      }

      let mut key = parser.current_token.value.clone();

      // Remove the quotes if the current token is a string.
      if parser.current_token_is(Box::new(Tokens::STRING)) {
        key = key.clone()[1..key.clone().len() - 1].to_string();
      }

      // Check if the key already exists in the HashMap.
      if hashmap.clone().has_key(key.clone()) {
        let line = parser.get_error_line_current_token();

        parser.errors.push(format!("{} the hashmap key is already in use.", line));

        return None;
      }

      // Check if the next token is a colon.
      if !parser.expect_token(Signs::new(Signs::COLON)) {
        let line = parser.get_error_line_next_token();

        parser.errors.push(format!("{} expect `:`, got `{}` instead.", line, parser.next_token.value));

        return None;
      }

      // Get the next token.
      parser.next_token();

      // Parse expression.
      match parse_expression(parser, Precedence::LOWEST, environment) {
        Some(expression) => {
          hashmap.data.push(HashMapItem {
            key,
            data_type: Types::from_expression(expression.clone(), environment),
            value: expression.clone(),
          });
        },
        None => {},
      }

      // Check if the next token is a comma.
      if parser.next_token_is(Signs::new(Signs::COMMA)) {
        // Get the next token.
        parser.next_token();
      }

      // Check if the next token is a right brace.
      if parser.next_token_is(Signs::new(Signs::RIGHTBRACE)) {
        // Get the next token.
        parser.next_token();
      }
    }

    // Return the hashmap expression.
    Some(Box::new(Expressions::HASHMAP(hashmap)))
  }

  pub fn get_from_environment(name: String, environment: &mut Environment) -> Option<HashMap> {
    match environment.get_statement(name) {
      Some(statement) => match statement.get_variable() {
        Some(variable) => match variable.value {
          Some(value) => value.get_hashmap(),
          None => None,
        },
        None => None,
      },
      None => None,
    }
  }
}

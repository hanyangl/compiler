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

use std::collections::HashMap as HashMapSTD;

use super::{
  Expression,
  Expressions,
  parse_expression,
};

#[derive(Debug, Clone, PartialEq)]
pub struct HashMap {
  token: Token,
  items: HashMapSTD<String, Box<Expressions>>,
}

impl Expression for HashMap {
  fn new() -> Self {
    Self {
      token: Token::new_empty(),
      items: HashMapSTD::new(),
    }
  }

  fn from_token(token: Token) -> Self {
    let mut hashmap: Self = Expression::new();

    hashmap.token = token;

    hashmap
  }

  fn get_token(&self) -> Token {
    self.token.clone()
  }

  fn string(&self) -> String {
    let mut values: Vec<String> = Vec::new();

    for (key, value) in self.get_items() {
      values.push(format!("{}: {}", key, value.string()));
    }

    format!("{{\n{}\n}}", values.join(",\n"))
  }
}

impl HashMap {
  pub fn get_items(&self) -> HashMapSTD<String, Box<Expressions>> {
    self.items.clone()
  }

  pub fn parse<'a>(
    parser: &'a mut Parser,
    standard_library: bool,
    with_this: bool,
  ) -> Result<Box<Expressions>, Error> {
    let mut hashmap: HashMap = Expression::from_token(parser.get_current_token());

    // Check if the next token is a right brace.
    if parser.next_token_is(Signs::new(Signs::RIGHTBRACE)) {
      // Get the next token.
      parser.next_token();
    }

    while !parser.current_token_is(Signs::new(Signs::RIGHTBRACE)) {
      // Check if the next token is an identifier or a string.
      if !parser.expect_token(Box::new(Tokens::IDENTIFIER)) {
        return Err(Error::from_token(
          String::from("is not a valid hashmap key."),
          parser.get_next_token(),
        ));
      }

      let mut key = parser.get_current_token().value;

      // Remove the quotes if the current token is a string.
      if parser.current_token_is(Box::new(Tokens::STRING)) {
        key = key.clone()[1..key.clone().len() - 1].to_string();
      }

      // Check if the key already exists in the HashMap.
      if hashmap.items.clone().contains_key(&key.clone()) {
        return Err(Error::from_token(
          String::from("the hashmap key is already in use."),
          parser.get_current_token(),
        ));
      }

      // Check if the next token is a colon.
      if !parser.expect_token(Signs::new(Signs::COLON)) {
        return Err(Error::from_token(
          format!("expect `:`, got `{}` instead.", parser.get_next_token().value),
          parser.get_next_token(),
        ));
      }

      // Get the next token.
      parser.next_token();

      // Parse expression.
      match parse_expression(parser, Precedence::LOWEST, standard_library, with_this) {
        Ok(expression) => {
          hashmap.items.insert(key, expression);
        },
        Err(error) => {
          return Err(error);
        },
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
    Ok(Box::new(Expressions::HASHMAP(hashmap)))
  }
}

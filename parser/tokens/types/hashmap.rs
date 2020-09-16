use crate::{
  parse_type,
  Parser,
  tokens::{
    Signs,
    Token,
    Tokens,
    Types,
  },
};

use std::collections::HashMap as HashMapSTD;

#[derive(Debug, Clone, PartialEq)]
pub struct HashMap {
  items: HashMapSTD<String, Token>,
  value: String,
}

impl HashMap {
  pub fn new(value: String) -> HashMap {
    HashMap {
      items: HashMapSTD::new(),
      value,
    }
  }

  pub fn from_value(value: &str) -> Result<HashMap, ()> {
    if !value.starts_with("{") && !value.ends_with("}") {
      return Err(());
    }

    let mut hashmap = HashMap::new(value.to_string());
    let new_value: &str = &value[1..value.len() - 1];

    for item in new_value.split(",") {
      let item: Vec<&str> = item.split(":").collect();

      if item.len() < 2 {
        return Err(());
      }

      hashmap.items.insert(
        item[0].trim().to_string(),
        Token::from_value(item[1..].join(":").trim(), 0, 0),
      );
    }

    Ok(hashmap)
  }

  pub fn get_items(&self) -> HashMapSTD<String, Token> {
    self.items.clone()
  }

  pub fn get_value(&self) -> String {
    self.value.clone()
  }

  pub fn parse<'a>(parser: &'a mut Parser) -> Result<Token, i32> {
    if !parser.current_token_is(Signs::new(Signs::LEFTBRACE)) {
      return Err(0);
    }

    let current_token = parser.get_current_token();

    // Get the next token.
    parser.next_token();

    let mut hashmap = HashMap::new(String::from("{"));

    while !parser.current_token_is(Signs::new(Signs::RIGHTBRACE)) {
      // Check if the next token is an identifier.
      if !parser.current_token_is(Box::new(Tokens::IDENTIFIER)) {
        return Err(1);
      }

      let key = parser.get_current_token().value;

      hashmap.value.push_str(" ");
      hashmap.value.push_str(key.as_str());

      // Check if the next token is a colon.
      if !parser.expect_token(Signs::new(Signs::COLON)) {
        return Err(1);
      }

      hashmap.value.push_str(": ");

      // Get the next token.
      parser.next_token();

      // Parse type.
      match parse_type(parser) {
        Ok(data_type) => {
          hashmap.items.insert(key, data_type.clone());
          hashmap.value.push_str(data_type.value.as_str());
        },
        Err(_) => {
          return Err(1);
        },
      }

      // Check if the next token is a comma.
      if parser.next_token_is(Signs::new(Signs::COMMA)) {
        // Get the next token.
        parser.next_token();

        hashmap.value.push_str(",");
      }

      // Get the next token.
      parser.next_token();
    }

    hashmap.value.push_str(" }");

    Ok(Token::new(
      Box::new(Tokens::TYPE(Types::HASHMAP(hashmap.clone()))),
      hashmap.value,
      current_token.line,
      current_token.position,
    ))
  }
}

#[test]
fn hashmap_from_value() {
  let mut hashmap = HashMap::new(String::from("{ lang: string, year: number }"));

  hashmap.items.insert(String::from("lang"), Token::from_value("string", 0, 0));
  hashmap.items.insert(String::from("year"), Token::from_value("number", 0, 0));

  let hashmap_2 = HashMap::from_value("{ lang: string, year: number }");

  assert_eq!(hashmap_2.is_ok(), true);
  assert_eq!(hashmap_2.unwrap(), hashmap);
}

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

use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct Function {
  arguments: HashMap<String, Token>,
  data_type: Token,
  value: String,
}

impl Function {
  pub fn new(data_type: Token, value: String) -> Function {
    Function {
      arguments: HashMap::new(),
      data_type,
      value,
    }
  }

  pub fn from_value(value: &str) -> Result<Function, ()> {
    let parts: Vec<String> = value.split("=>").map(|x| x.to_string()).collect();

    if !value.starts_with("(") && parts.len() < 2 {
      return Err(());
    }

    let mut function = Function::new(
      Token::from_value(parts.last().unwrap().trim(), 0, 0),
      value.to_string(),
    );

    let arguments = parts[0..parts.len() - 1].join("=>");
    let arguments = arguments.trim();

    if arguments != "()" {
      // Parse function arguments.
      for argument in arguments[1..].split(",") {
        let argument: Vec<&str> = argument.split(":").collect();

        if argument.len() < 2 {
          return Err(());
        }

        let data_type = argument[1..].join(":");
        let mut data_type = data_type.trim();

        if !data_type.starts_with("(") && data_type.ends_with(")") {
          data_type = &data_type[..data_type.len() - 1];
        }

        function.arguments.insert(
          argument[0].trim().to_string(),
          Token::from_value(data_type, 0, 0),
        );
      }
    }

    Ok(function)
  }

  pub fn get_arguments(&self) -> HashMap<String, Token> {
    self.arguments.clone()
  }

  pub fn get_type(&self) -> Token {
    self.data_type.clone()
  }

  pub fn get_value(&self) -> String {
    self.value.clone()
  }

  pub fn parse<'a>(parser: &'a mut Parser) -> Result<Token, i32> {
    if !parser.current_token_is(Signs::new(Signs::LEFTPARENTHESES)) || (
      !parser.next_token_is(Box::new(Tokens::IDENTIFIER)) &&
      !parser.next_token_is(Signs::new(Signs::RIGHTPARENTHESES))
    ) {
      return Err(0);
    }

    let current_token = parser.get_current_token();

    // Get the next token.
    parser.next_token();

    let mut function = Function::new(Token::new_empty(), String::from("("));

    while !parser.current_token_is(Signs::new(Signs::RIGHTPARENTHESES)) {
      // Check if the next token is an identifier.
      if !parser.current_token_is(Box::new(Tokens::IDENTIFIER)) {
        return Err(1);
      }

      let key = parser.get_current_token().value;

      if function.arguments.len() > 0 {
        function.value.push_str(" ");
      }

      function.value.push_str(key.as_str());

      // Check if the next token is a colon.
      if !parser.expect_token(Signs::new(Signs::COLON)) {
        return Err(1);
      }

      function.value.push_str(": ");

      // Get the next token.
      parser.next_token();

      // Parse type.
      match parse_type(parser) {
        Ok(data_type) => {
          function.arguments.insert(key, data_type.clone());
          function.value.push_str(data_type.value.as_str());
        },
        Err(_) => {
          return Err(1);
        },
      }

      // Check if the next token is a comma.
      if parser.next_token_is(Signs::new(Signs::COMMA)) {
        // Get the next token.
        parser.next_token();

        function.value.push_str(",");
      }

      // Get the next token.
      parser.next_token();
    }

    if function.value.ends_with(",") {
      function.value = function.value[..function.value.len() - 1].to_string();
    }

    function.value.push_str(")");

    if !parser.expect_token(Signs::new(Signs::ASSIGNARROW)) {
      return Err(1);
    }

    function.value.push_str(" => ");

    // Get the next token.
    parser.next_token();

    match parse_type(parser) {
      Ok(data_type) => {
        function.data_type = data_type.clone();
        function.value.push_str(data_type.value.as_str());
      },
      Err(_) => {
        return Err(1);
      },
    }

    Ok(Token::new(
      Box::new(Tokens::TYPE(Types::FUNCTION(function.clone()))),
      function.value,
      current_token.line,
      current_token.position,
    ))
  }
}

#[test]
fn function_from_value() {
  let mut function = Function::new(
    Token::from_value("string", 0, 0),
    String::from("(name: string) => string"),
  );

  function.arguments.insert(String::from("name"), Token::from_value("string", 0, 0));

  let function_2 = Function::from_value("(name: string) => string");

  assert_eq!(function_2.is_ok(), true);
  assert_eq!(function_2.unwrap(), function);
}

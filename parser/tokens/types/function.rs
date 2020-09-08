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
  pub arguments: HashMap<String, Token>,
  pub data_type: Token,
  pub value: String,
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
    if !value.starts_with("(") && !value.contains("=>") {
      return Err(());
    }

    let mut function = Function::new(Token::from_value("any", 0, 0), value.to_string());
    let mut arguments: String = String::new();
    let mut current_character: &str = "(";
    let mut index: usize = 1;

    while current_character != "=" {
      if value.len() - 1 == index {
        return Err(());
      }

      arguments.push_str(current_character);

      // Set the current character.
      current_character = &value[index..index + 1];

      // Get the next index.
      index += 1;
    }

    // Parse function arguments.
    for argument in arguments[1..].split(",") {
      let argument: Vec<&str> = argument.split(":").collect();

      if argument.len() != 2 {
        return Err(());
      }

      let mut data_type = argument[1].trim();

      if !data_type.starts_with("(") && data_type.ends_with(")") {
        data_type = &data_type[..data_type.len() - 1];
      }

      function.arguments.insert(
        argument[0].trim().to_string(),
        Token::from_value(data_type, 0, 0),
      );
    }

    let new_value: &str = &value[index - 1..].trim();

    if !new_value.starts_with("=>") {
      return Err(());
    }

    function.data_type = Token::from_value(&new_value[2..].trim(), 0, 0);

    Ok(function)
  }

  pub fn parse<'a>(parser: &'a mut Parser) -> Result<Token, i32> {
    if !parser.current_token_is(Signs::new(Signs::LEFTPARENTHESES)) || (
      !parser.next_token_is(Box::new(Tokens::IDENTIFIER)) &&
      !parser.next_token_is(Signs::new(Signs::RIGHTPARENTHESES))
    ) {
      return Err(0);
    }

    let current_token = parser.current_token.clone();

    // Get the next token.
    parser.next_token();

    let mut function = Function::new(Token::new_empty(), String::from("("));

    while !parser.current_token_is(Signs::new(Signs::RIGHTPARENTHESES)) {
      // Check if the next token is an identifier.
      if !parser.current_token_is(Box::new(Tokens::IDENTIFIER)) {
        return Err(1);
      }

      let key = parser.current_token.value.clone();

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
      match parse_type(parser, false) {
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

    match parse_type(parser, false) {
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

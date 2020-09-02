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

#[derive(Debug, Clone, PartialEq)]
pub struct Group {
  pub types: Vec<Token>,
  pub value: String,
}

impl Group {
  pub fn new(value: String) -> Group {
    Group { 
      types: Vec::new(),
      value,
    }
  }

  pub fn from_value(value: &str) -> Result<Group, ()> {
    let mut new_value: &str = value;

    if value.starts_with("(") && value.ends_with(")") {
      new_value = &value[1..value.len() - 1];
    } else if !value.contains("|") {
      return Err(());
    }

    let mut group = Group::new(value.to_string());

    for data_type in new_value.split("|") {
      group.types.push(Token::from_value(data_type.trim(), 0, 0));
    }

    Ok(group)
  }

  pub fn get_types_strings(self) -> Vec<String> {
    let mut types: Vec<String> = Vec::new();

    for data_type in self.types {
      types.push(data_type.value);
    }

    types
  }

  pub fn has_type(self, data_type: String) -> bool {
    self.get_types_strings().contains(&data_type)
  }

  pub fn parse<'a>(parser: &'a mut Parser) -> Result<Token, i32> {
    if !parser.current_token_is(Signs::new(Signs::LEFTPARENTHESES)) ||
      parser.next_token_is(Box::new(Tokens::IDENTIFIER)) ||
      parser.next_token_is(Signs::new(Signs::RIGHTPARENTHESES)) {
      return Err(0);
    }

    let current_token = parser.current_token.clone();

    // Get the next token.
    parser.next_token();

    // Create a new group type.
    let mut group_types = Group::new(String::from("("));

    while !parser.current_token_is(Signs::new(Signs::RIGHTPARENTHESES)) {
      let current_token = parser.current_token.clone();

      match parse_type(parser, true) {
        Ok(data_type) => {
          if group_types.types.len() > 0 {
            group_types.value.push_str(" | ");
          }

          group_types.types.push(data_type);
          group_types.value.push_str(current_token.value.as_str());
        },
        Err(_) => {
          return Err(1);
        },
      }

      // Check if the next token is a bit or.
      if parser.next_token_is(Signs::new(Signs::BITOR)) {
        // Get the next token.
        parser.next_token();
      }

      // Get the next token.
      parser.next_token();
    }

    group_types.value.push_str(")");

    Ok(Token::new(
      Box::new(Tokens::TYPE(Types::GROUP(group_types.clone()))),
      group_types.value,
      current_token.line,
      current_token.position,
    ))
  }

  pub fn parse_without_parentheses<'a>(parser: &'a mut Parser, token: Token) -> Result<Token, i32> {
    if !parser.expect_token(Signs::new(Signs::BITOR)) {
      return Err(0);
    }

    let mut group_types = Group::new(token.value.clone());

    group_types.types.push(token.clone());
    group_types.value.push_str(" |");

    // Get the next token.
    parser.next_token();

    loop {
      match parse_type(parser, true) {
        Ok(data_type) => {
          group_types.types.push(data_type.clone());
          group_types.value.push_str(" ");
          group_types.value.push_str(data_type.value.as_str());
        },
        Err(_) => {
          break;
        },
      }

      // Check if the next token is a bit or.
      if parser.next_token_is(Signs::new(Signs::BITOR)) {
        // Get the next token.
        parser.next_token();

        group_types.value.push_str(" |");
      }

      // Get the next token.
      parser.next_token();
    }

    Ok(Token::new(
      Box::new(Tokens::TYPE(Types::GROUP(group_types.clone()))),
      group_types.value,
      token.line.clone(),
      token.position.clone(),
    ))
  }
}

#[test]
fn group_from_value() {
  let mut group = Group::new(String::from("(number | string)"));

  group.types.push(Token::from_value("number", 0, 0));
  group.types.push(Token::from_value("string", 0, 0));

  let group_2 = Group::from_value("(number | string)");

  assert_eq!(group_2.is_ok(), true);
  assert_eq!(group_2.unwrap(), group);
}

#[test]
fn group_from_value_2() {
  let mut group = Group::new(String::from("number | string | undefined"));

  group.types.push(Token::from_value("number", 0, 0));
  group.types.push(Token::from_value("string", 0, 0));
  group.types.push(Token::from_value("undefined", 0, 0));

  let group_2 = Group::from_value("number | string | undefined");

  assert_eq!(group_2.is_ok(), true);
  assert_eq!(group_2.unwrap(), group);
}

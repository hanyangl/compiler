use crate::{
  Parser,
  tokens::{
    Signs,
    Token,
    Tokens,
    Types,
  },
};

#[derive(Debug, Clone, PartialEq)]
pub struct Array {
  pub data_type: Token,
  pub value: String,
}

impl Array {
  pub fn new(data_type: Token, value: String) -> Array {
    Array { data_type, value }
  }

  pub fn from_value(value: &str) -> Result<Array, ()> {
    if !value.ends_with("[]") {
      return Err(());
    }

    Ok(Array::new(Token::from_value(&value[0..value.len() - 2], 0, 0), value.to_string()))
  }

  pub fn parse<'a>(parser: &'a mut Parser, token: Token) -> Result<Token, ()> {
    if !parser.expect_token(Signs::new(Signs::LEFTBRACKET)) ||
      !parser.expect_token(Signs::new(Signs::RIGHTBRACKET)) {
      return Err(());
    }

    let value = format!("{}[]", token.value.clone());

    Ok(Token::new(
      Box::new(Tokens::TYPE(Types::ARRAY(
        Array::new(token.clone(), value.clone()),
      ))),
      value,
      token.line.clone(),
      token.position.clone(),
    ))
  }
}

#[test]
fn array_from_value_1() {
  let array = Array::new(Token::from_value("number", 0, 0), String::from("number[]"));
  let array_2 = Array::from_value("number[]");

  assert_eq!(array_2.is_ok(), true);
  assert_eq!(array_2.unwrap(), array);
}

#[test]
fn array_from_value_2() {
  let array = Array::new(
    Token::from_value("(number | undefined | string[])", 0, 0),
    String::from("(number | undefined | string[])[]"),
  );

  let array_2 = Array::from_value("(number | undefined | string[])[]");

  assert_eq!(array_2.is_ok(), true);
  assert_eq!(array_2.unwrap(), array);
}

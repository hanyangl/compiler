#[cfg(test)]
use sflyn_compiler::parser::tokens::*;

#[cfg(test)]
fn test_keyword(value: &str, expect: Keywords) {
  let value = value.to_string();
  let token = Token::from_value(value.clone(), 1, 1);

  match token.token.clone().get_keyword() {
    Some(keyword) => {
      assert_eq!(keyword, expect);
    },
    None => {
      println!("Expect {:?}, got {:?} instead", expect, token.token);
    },
  }

  assert_eq!(token.value, value.clone());
}

#[test]
fn parser_keywords() {
  // Variables
  test_keyword("let", Keywords::LET);
  test_keyword("const", Keywords::CONST);

  // Function
  test_keyword("function", Keywords::FUNCTION);
  test_keyword("return", Keywords::RETURN);

  // Boolean
  test_keyword("true", Keywords::TRUE);
  test_keyword("false", Keywords::FALSE);
}

#[cfg(test)]
fn test_sign(value: &str, expect: Signs) {
  let value = value.to_string();
  let token = Token::from_value(value.clone(), 1, 1);

  match token.token.clone().get_sign() {
    Some(sign) => {
      assert_eq!(sign, expect);
    },
    None => {
      println!("Expect {:?}, got {:?} instead.", expect, token.token);
    },
  }

  assert_eq!(token.value, value.clone());
}

#[test]
fn parser_signs() {
  // Delimeters
  test_sign(",", Signs::COMMA); // COMMA
  test_sign(":", Signs::COLON); // COLON
  test_sign(";", Signs::SEMICOLON); // SEMICOLON

  test_sign("(", Signs::LEFTPARENTHESES); // LEFTPARENTHESES
  test_sign(")", Signs::RIGHTPARENTHESES); // RIGHTPARENTHESES

  test_sign("[", Signs::LEFTBRACKET); // LEFTBRACKET
  test_sign("]", Signs::RIGHTBRACKET); // RIGHTBRACKET

  test_sign("{", Signs::LEFTBRACE); // LEFTBRACE
  test_sign("}", Signs::RIGHTBRACE); // RIGHTBRACE

  // Others
  test_sign("=", Signs::ASSIGN); // ASSIGN
}

#[cfg(test)]
fn test_type(value: &str, expect: Types) {
  let value = value.to_string();
  let token = Token::from_value(value.clone(), 1, 1);

  match token.token.clone().get_type() {
    Some(data_type) => {
      assert_eq!(data_type, expect);
    },
    None => {
      println!("Expect {:?}, got {:?} instead", expect, token.token);
    },
  }

  assert_eq!(token.value, value.clone());
}

#[test]
fn parser_types() {
  // Basic
  test_type("string", Types::STRING);
  test_type("number", Types::NUMBER);
  test_type("boolean", Types::BOOLEAN);

  // Function
  test_type("void", Types::VOID);
}

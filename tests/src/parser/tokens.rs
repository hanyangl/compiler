#[cfg(test)]
use sflyn_parser::tokens::*;

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

#[cfg(test)]
fn keywords() {
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

#[cfg(test)]
fn signs() {
  // Delimeters
  test_sign(",", Signs::COMMA);
  test_sign(":", Signs::COLON);
  test_sign(";", Signs::SEMICOLON);

  test_sign("(", Signs::LEFTPARENTHESES);
  test_sign(")", Signs::RIGHTPARENTHESES);

  test_sign("[", Signs::LEFTBRACKET);
  test_sign("]", Signs::RIGHTBRACKET);

  test_sign("{", Signs::LEFTBRACE);
  test_sign("}", Signs::RIGHTBRACE);

  // Assign
  test_sign("=", Signs::ASSIGN);
  test_sign("+=", Signs::PLUSASSIGN);
  test_sign("-=", Signs::MINUSASSIGN);
  test_sign("*=", Signs::MULTIPLYASSIGN);
  test_sign("/=", Signs::DIVIDEASSIGN);

  test_sign("++", Signs::PLUSPLUS);
  test_sign("--", Signs::MINUSMINUS);

  // Conditions
  test_sign("==", Signs::EQUAL);
  test_sign("===", Signs::EQUALTYPE);
  test_sign("!=", Signs::NOTEQUAL);
  test_sign("!==", Signs::NOTEQUALTYPE);

  test_sign("<", Signs::LESSTHAN);
  test_sign("<=", Signs::LESSOREQUALTHAN);
  test_sign(">", Signs::GREATERTHAN);
  test_sign(">=", Signs::GREATEROREQUALTHAN);

  test_sign("&&", Signs::AND);
  test_sign("||", Signs::OR);

  // Maths
  test_sign("+", Signs::PLUS);
  test_sign("-", Signs::MINUS);
  test_sign("*", Signs::MULTIPLY);
  test_sign("/", Signs::DIVIDE);
  test_sign("**", Signs::EMPOWERMENT);
  test_sign("%", Signs::MODULE);

  // Others
  test_sign("!", Signs::NEGATION);
  test_sign("->", Signs::ARROW);
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

#[cfg(test)]
fn types() {
  // Basic
  test_type("null", Types::NULL);
  test_type("undefined", Types::UNDEFINED);
  test_type("string", Types::STRING);
  test_type("number", Types::NUMBER);
  test_type("boolean", Types::BOOLEAN);

  // Function
  test_type("void", Types::VOID);
}

#[test]
fn parser_tokens() {
  keywords();
  signs();
  types();
}

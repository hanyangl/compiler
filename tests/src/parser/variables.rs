#[cfg(test)]
use sflyn_parser::{Parser, statements::*, tokens::*, expressions::*};

#[cfg(test)]
use super::*;

#[cfg(test)]
fn test_variable(value: &str, expect: Box<Statements>) {
  let lexer = generate_lexer(value);
  let mut parser = Parser::new(lexer);
  let statements = parser.parse_program();

  if parser.errors.len() > 0 {
    parser.show_errors();
  }

  assert_eq!(parser.errors.len(), 0);
  assert_eq!(statements.len(), 1);
  assert_eq!(statements[0].clone(), expect.clone());
}

#[cfg(test)]
fn let_string(tokens: Vec<Token>) -> Box<Statements> {
  let mut statement: Variable = Statement::new();

  statement.token = tokens[0].clone();
  statement.name = Identifier::new_box_from_token(tokens[1].clone());
  statement.data_type = tokens[3].clone();
  statement.value = Some(StringE::new_box_from_token(tokens[5].clone()));

  Box::new(Statements::VARIABLE(statement))
}

#[cfg(test)]
fn let_string_type(tokens: Vec<Token>) -> Box<Statements> {
  let mut statement: Variable = Statement::new();

  statement.token = tokens[0].clone();
  statement.name = Identifier::new_box_from_token(tokens[1].clone());
  statement.data_type = Token::from_value("string".to_string(), 0, 0);
  statement.value = Some(StringE::new_box_from_token(tokens[3].clone()));

  Box::new(Statements::VARIABLE(statement))
}

#[cfg(test)]
fn let_number(tokens: Vec<Token>) -> Box<Statements> {
  let mut statement: Variable = Statement::new();

  statement.token = tokens[0].clone();
  statement.name = Identifier::new_box_from_token(tokens[1].clone());
  statement.data_type = tokens[3].clone();
  statement.value = Some(Number::new_box_from_token(tokens[5].clone()));

  Box::new(Statements::VARIABLE(statement))
}

#[cfg(test)]
fn let_number_type(tokens: Vec<Token>) -> Box<Statements> {
  let mut statement: Variable = Statement::new();

  statement.token = tokens[0].clone();
  statement.name = Identifier::new_box_from_token(tokens[1].clone());
  statement.data_type = Token::from_value("number".to_string(), 0, 0);
  statement.value = Some(Number::new_box_from_token(tokens[3].clone()));

  Box::new(Statements::VARIABLE(statement))
}

#[cfg(test)]
fn let_boolean(tokens: Vec<Token>) -> Box<Statements> {
  let mut statement: Variable = Statement::new();

  statement.token = tokens[0].clone();
  statement.name = Identifier::new_box_from_token(tokens[1].clone());
  statement.data_type = tokens[3].clone();
  statement.value = Some(Boolean::new_box_from_token(tokens[5].clone()));

  Box::new(Statements::VARIABLE(statement))
}

#[cfg(test)]
fn let_boolean_type(tokens: Vec<Token>) -> Box<Statements> {
  let mut statement: Variable = Statement::new();

  statement.token = tokens[0].clone();
  statement.name = Identifier::new_box_from_token(tokens[1].clone());
  statement.data_type = Token::from_value("boolean".to_string(), 0, 0);
  statement.value = Some(Boolean::new_box_from_token(tokens[3].clone()));

  Box::new(Statements::VARIABLE(statement))
}

#[test]
fn parser_variables() {
  // String let
  test_variable("let lang: string = 'Sflyn';", let_string(lexer::let_lang_string("let")));
  test_variable("const lang: string = 'Sflyn';", let_string(lexer::let_lang_string("const")));

  test_variable("let lang2 = 'Sflyn';", let_string_type(lexer::let_lang2_string("let")));
  test_variable("const lang2 = 'Sflyn';", let_string_type(lexer::let_lang2_string("const")));

  // Number let
  test_variable("let two: number = 2;", let_number(lexer::let_two_number("let")));
  test_variable("const two: number = 2;", let_number(lexer::let_two_number("const")));

  test_variable("let three = 3;", let_number_type(lexer::let_three_number("let")));
  test_variable("const three = 3;", let_number_type(lexer::let_three_number("const")));

  // Boolean let
  test_variable("let is_lexer: boolean = true;", let_boolean(lexer::let_is_lexer_boolean("let")));
  test_variable("const is_lexer: boolean = true;", let_boolean(lexer::let_is_lexer_boolean("const")));

  test_variable("let is_lexer2 = true;", let_boolean_type(lexer::let_is_lexer2_boolean("let")));
  test_variable("const is_lexer2 = true;", let_boolean_type(lexer::let_is_lexer2_boolean("const")));
}

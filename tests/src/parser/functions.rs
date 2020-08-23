#[cfg(test)]
use sflyn_parser::{Parser, statements::*, tokens::*, expressions::*};

#[cfg(test)]
use super::*;

#[cfg(test)]
fn test_function(value: &str, expect: Box<Statements>) {
  let lexer = generate_lexer(value);
  let mut parser = Parser::new(lexer);
  let statements = parser.parse_program();

  if parser.errors.len() > 0 {
    parser.show_errors();
  }

  assert_eq!(parser.errors.len(), 0);
  assert_eq!(statements.len(), 1);
  assert_eq!(statements[0].clone(), expect);
}

#[cfg(test)]
fn get_box(function: Function) -> Box<Statements> {
  Box::new(Statements::FUNCTION(function))
}

#[test]
fn parser_functions() {
  let mut statements: Vec<Function> = Vec::new();

  // Empty function
  statements.push(Statement::new());

  statements[0].token = Token::from_value(String::from("function"), 1, 1);
  statements[0].name = Identifier::new_box_from_token(lexer::get_identifier("empty", 1, 10));
  statements[0].body = Block::new_box_from_token(Token::from_value(String::from("{"), 1, 18));

  test_function("function empty() {}", get_box(statements[0].clone()));
}

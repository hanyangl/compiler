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
fn get_function() -> Function {
  Statement::from_token(Token::from_value(String::from("function"), 1, 1))
}

#[cfg(test)]
fn get_box(function: Function) -> Box<Statements> {
  Box::new(Statements::FUNCTION(function))
}

#[cfg(test)]
fn empty_function() -> Box<Statements> {
  let mut function: Function = get_function();

  function.name = Identifier::new_box_from_token(lexer::get_identifier("empty", 1, 10));
  function.body = Block::new_box_from_token(Token::from_value(String::from("{"), 1, 18));

  get_box(function)
}

#[cfg(test)]
fn type_function(data_type: &str) -> Box<Statements> {
  let mut function: Function = get_function();

  function.name = Identifier::new_box_from_token(lexer::get_identifier("type", 1, 10));
  function.data_type = Token::from_value(data_type.to_string(), 1, 18);
  function.body = Block::new_box_from_token(Token::from_value(String::from("{"), 1, data_type.len() + 19));

  get_box(function)
}

#[cfg(test)]
fn arguments_function() -> Box<Statements> {
  let mut function: Function = get_function();

  function.name = Identifier::new_box_from_token(lexer::get_identifier("add", 1, 10));

  let mut x_arg = Argument::from_token(lexer::get_identifier("x", 1, 14));
  x_arg.data_type = Token::from_value(String::from("number"), 1, 17);

  function.arguments.push(Box::new(Expressions::ARGUMENT(x_arg)));

  let mut y_arg = Argument::from_token(lexer::get_identifier("y", 1, 25));
  y_arg.data_type = Token::from_value(String::from("number"), 1, 28);

  function.arguments.push(Box::new(Expressions::ARGUMENT(y_arg)));

  function.data_type = Token::from_value(String::from("number"), 1, 37);
  function.body = Block::new_box_from_token(Token::from_value(String::from("{"), 1, 44));

  get_box(function)
}

#[cfg(test)]
fn get_variable(name: &str) -> Variable {
  let mut variable: Variable = Statement::from_token(Token::from_value(String::from("let"), 1, 1));

  variable.name = Identifier::new_box_from_token(lexer::get_identifier(name, 1, 5));
  variable.data_type = Token::from_value(String::from("void"), 0, 0);

  variable
}

#[cfg(test)]
fn empty_anonymous_function_1() -> Box<Statements> {
  let mut variable: Variable = get_variable("empty");

  let mut function: AnonymousFunction = Expression::from_token(Token::from_value(String::from("function"), 1, 13));

  function.body = Block::new_box_from_token(Token::from_value(String::from("{"), 1, 25));

  variable.value = Some(Box::new(Expressions::ANONYMOUSFUNCTION(function)));

  Box::new(Statements::VARIABLE(variable))
}

#[cfg(test)]
fn empty_anonymous_function_2() -> Box<Statements> {
  let mut variable: Variable = get_variable("empty");

  let mut function: AnonymousFunction = Expression::from_token(Token::from_value(String::from("("), 1, 13));

  function.body = Block::new_box_from_token(Token::from_value(String::from("{"), 1, 19));

  variable.value = Some(Box::new(Expressions::ANONYMOUSFUNCTION(function)));

  Box::new(Statements::VARIABLE(variable))
}

#[cfg(test)]
fn type_anonymous_function_1(data_type: &str) -> Box<Statements> {
  let mut variable: Variable = get_variable("type");

  let mut function: AnonymousFunction = Expression::from_token(Token::from_value(String::from("function"), 1, 12));

  function.data_type = Token::from_value(data_type.to_string(), 1, 25);

  function.body = Block::new_box_from_token(Token::from_value(String::from("{"), 1, data_type.len() + 26));

  variable.value = Some(Box::new(Expressions::ANONYMOUSFUNCTION(function)));

  Box::new(Statements::VARIABLE(variable))
}

#[cfg(test)]
fn type_anonymous_function_2(data_type: &str) -> Box<Statements> {
  let mut variable: Variable = get_variable("type");

  let mut function: AnonymousFunction = Expression::from_token(Token::from_value(String::from("("), 1, 12));

  function.data_type = Token::from_value(data_type.to_string(), 1, 16);

  function.body = Block::new_box_from_token(Token::from_value(String::from("{"), 1, data_type.len() + 20));

  variable.value = Some(Box::new(Expressions::ANONYMOUSFUNCTION(function)));

  Box::new(Statements::VARIABLE(variable))
}

#[cfg(test)]
fn arguments_anonymous_function_1() -> Box<Statements> {
  let mut variable: Variable = get_variable("add");

  let mut function: AnonymousFunction = Expression::from_token(Token::from_value(String::from("function"), 1, 11));

  let mut x_arg = Argument::from_token(lexer::get_identifier("x", 1, 21));
  x_arg.data_type = Token::from_value(String::from("number"), 1, 24);

  function.arguments.push(Box::new(Expressions::ARGUMENT(x_arg)));

  let mut y_arg = Argument::from_token(lexer::get_identifier("y", 1, 32));
  y_arg.data_type = Token::from_value(String::from("number"), 1, 35);

  function.arguments.push(Box::new(Expressions::ARGUMENT(y_arg)));

  function.data_type = Token::from_value(String::from("number"), 1, 44);
  function.body = Block::new_box_from_token(Token::from_value(String::from("{"), 1, 51));

  variable.value = Some(Box::new(Expressions::ANONYMOUSFUNCTION(function)));

  Box::new(Statements::VARIABLE(variable))
}

#[cfg(test)]
fn arguments_anonymous_function_2() -> Box<Statements> {
  let mut variable: Variable = get_variable("add");

  let mut function: AnonymousFunction = Expression::from_token(Token::from_value(String::from("("), 1, 11));

  let mut x_arg = Argument::from_token(lexer::get_identifier("x", 1, 12));
  x_arg.data_type = Token::from_value(String::from("number"), 1, 15);

  function.arguments.push(Box::new(Expressions::ARGUMENT(x_arg)));

  let mut y_arg = Argument::from_token(lexer::get_identifier("y", 1, 23));
  y_arg.data_type = Token::from_value(String::from("number"), 1, 26);

  function.arguments.push(Box::new(Expressions::ARGUMENT(y_arg)));

  function.data_type = Token::from_value(String::from("number"), 1, 35);
  function.body = Block::new_box_from_token(Token::from_value(String::from("{"), 1, 45));

  variable.value = Some(Box::new(Expressions::ANONYMOUSFUNCTION(function)));

  Box::new(Statements::VARIABLE(variable))
}

#[test]
fn parser_functions() {
  test_function("function empty() {}", empty_function());

  test_function("function type(): string {}", type_function("string"));
  test_function("function type(): number {}", type_function("number"));
  test_function("function type(): boolean {}", type_function("boolean"));
  test_function("function type(): void {}", type_function("void"));

  test_function("function add(x: number, y: number): number {}", arguments_function());

  // Anonymous functions.
  test_function("let empty = function () {};", empty_anonymous_function_1());
  test_function("let empty = () => {};", empty_anonymous_function_2());

  test_function("let type = function (): string {};", type_anonymous_function_1("string"));
  test_function("let type = function (): number {};", type_anonymous_function_1("number"));
  test_function("let type = function (): boolean {};", type_anonymous_function_1("boolean"));
  test_function("let type = function (): void {};", type_anonymous_function_1("void"));

  test_function("let type = (): string => {};", type_anonymous_function_2("string"));
  test_function("let type = (): number => {};", type_anonymous_function_2("number"));
  test_function("let type = (): boolean => {};", type_anonymous_function_2("boolean"));
  test_function("let type = (): void => {};", type_anonymous_function_2("void"));

  test_function("let add = function (x: number, y: number): number {};", arguments_anonymous_function_1());
  test_function("let add = (x: number, y: number): number => {};", arguments_anonymous_function_2());
}

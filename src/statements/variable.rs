use crate::data;
use crate::expressions::{Identifier, Expression};
use crate::statements::{expression, Statement};
use crate::parser::{Parser, precedence::Precedence, Expressions};

#[derive(Debug, Clone)]
pub struct Variable {
  token: data::Token,
  name: Identifier,
  data_type: data::Token,
  value: Box<Expressions>,
}

impl Statement for Variable {
  fn new() -> Variable {
    Variable {
      token: data::Token::empty(),
      name: Expression::new(),
      data_type: data::Token::empty(),
      value: Box::new(Expressions::DEFAULT(Expression::new())),
    }
  }

  fn from_token(token: &data::Token) -> Variable {
    let mut variable: Variable = Statement::new();
  
    variable.token = token.clone();
  
    variable
  }

  fn string(self) -> String {
    format!(
      "{} {}: {} = {};",
      self.token.value,
      self.name.string(),
      self.data_type.value,
      self.value.string(),
    )
  }
}

/// Check if it is a valid variable statement.
/// 
/// ## Example:
/// ```sf
/// let username: string = "Sflyn";
/// let decimal: number = 6;
/// ```
pub fn parse<'a>(parser: &'a mut Parser) -> (Variable, bool) {
  let mut statement: Variable = Statement::from_token(&parser.current_token.clone());
  let token_name = &statement.token.value;

  // Check if a valid variable name.
  if parser.expect_token(&data::Tokens::IDENTIFIER) == false {
    let token: data::Token = parser.peek_token.clone();
    let line = parser.get_error_line(format!("{} ", token_name).as_str());
    let mut message = format!("{} `{}` is not a valid variable name.", line, token.value);

    if token.sign == data::Signs::COLON {
      message = format!("{} you must put the variable name.", line);
    }

    parser.errors.push(message);

    return (Statement::new(), false);
  }

  // Set the variable name to the statement.
  statement.name = Expression::from_token(&parser.current_token.clone());

  // Check if the next character is a colon (:).
  if parser.expect_sign(&data::Signs::COLON) == false {
    let line = parser.get_error_line(format!("{} {}", token_name, statement.name.string()).as_str());

    parser.errors.push(format!("{} expect `:`, got `{}` instead.", line, parser.peek_token.value));

    return (Statement::new(), false);
  }

  // Check if the next token is a valid data type.
  if parser.expect_token(&data::Tokens::TYPE) == false {
    let line = parser.get_error_line(format!("{} {}: ", token_name, statement.name.string()).as_str());

    parser.errors.push(format!("{} `{}` is not a valid data type.", line, parser.peek_token.value));

    return (Statement::new(), false);
  }

  // Set the data type to the statement.
  statement.data_type = parser.current_token.clone();

  // Check if the next character is an assign sign (=).
  if parser.expect_sign(&data::Signs::ASSIGN) == false {
    let line = parser.get_error_line(
      format!(
        "{} {}: {} ",
        token_name,
        statement.name.string(),
        statement.data_type.value
      ).as_str()
    );

    parser.errors.push(format!("{} expect `=`, got `{}` instead.", line, parser.peek_token.value));

    return (Statement::new(), false);
  }

  // Check if the next character is not a semicolon (;).
  if parser.peek_token_is_sign(&data::Signs::SEMICOLON) == true {
    let line = parser.get_error_line(
      format!(
        "{} {}: {} = ",
        token_name,
        statement.name.string(),
        statement.data_type.value
      ).as_str()
    );

    parser.errors.push(format!("{} you must set a value to the variable.", line));

    return (Statement::new(), false);
  }

  // Get the variable value.
  //let value: &data::Token = &parser.peek_token.clone();

  // Check if the variable value is the same type of the variable declaration.
  /*if expression::token_is_valid_type(&statement.data_type.data_type, value) == false {
    let line = parser.get_error_line(
      format!(
        "{} {}: {} = ",
        token_name,
        statement.name.string(),
        statement.data_type.value
      ).as_str()
    );

    parser.errors.push(format!("{} `{}` is not a valid {}", line, value.value, statement.data_type.value));

    return (Statement::new(), false);
  }*/

  // Go to the next token.
  parser.next_token();

  // Set the variable value to the statement.
  statement.value = expression::parse(parser, Precedence::LOWEST);

  // Check if the next character is a semicolon (;).
  /*if parser.expect_sign(&data::Signs::SEMICOLON) == false {
    let token = parser.peek_token.clone();

    let line = parser.get_error_line(
      format!(
        "{} {}: {} = {}",
        token_name,
        statement.name.string(),
        statement.data_type.value,
        statement.value.string(),
      ).as_str(),
    );

    if token.token == data::Tokens::EOL {
      parser.errors.push(format!("{} must be ends with `;`.", line));
    } else {
      parser.errors.push(format!("{} expect `;`, got `{}` instead.", line, token.value));
    }

    return (Statement::new(), false);
  }*/

  (statement, true)
}

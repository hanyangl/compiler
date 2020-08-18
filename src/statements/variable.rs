use crate::compiler::environment::Environment;
use crate::data;
use crate::expressions::{identifier::Identifier, Expression, Expressions, parse as expression_parse};
use crate::objects::Objects;
use crate::parser::{Parser, precedence::Precedence};
use crate::utils::{repeat_character, types::expression_is_valid_type};

use super::Statement;

#[derive(Debug, Clone, PartialEq)]
pub struct Variable {
  pub token: data::Token,
  pub name: Identifier,
  pub data_type: data::Token,
  pub value: Option<Box<Expressions>>,
}

impl Statement for Variable {
  fn new() -> Variable {
    Variable {
      token: data::Token::empty(),
      name: Expression::new(),
      data_type: data::Token::empty(),
      value: None,
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
      match self.value {
        Some(x) => x.string(),
        None => "".to_string(),
      }
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
pub fn parse<'a>(parser: &'a mut Parser, env: &mut Environment) -> Option<Variable> {
  let mut statement: Variable = Statement::from_token(&parser.current_token.clone());
  let token_name = &statement.token.value;

  // Check if a valid variable name.
  if !parser.peek_token_is(&data::Tokens::IDENTIFIER) {
    let token: data::Token = parser.peek_token.clone();
    let line = parser.get_error_line(format!("{} ", token_name).as_str());
    let mut message = format!("{} `{}` is not a valid variable name.", line, token.value);

    if token.sign == data::Signs::COLON {
      message = format!("{} you must put the variable name.", line);
    }

    parser.errors.push(message);

    return None;
  }

  // Check if the variable name is in use.
  match env.clone().get(parser.peek_token.value.clone()) {
    Some(_) => {
      let line = parser.get_error_line(format!("{} ", token_name).as_str());

      parser.errors.push(format!("{} `{}` is already in use.", line, parser.peek_token.value));

      return None;
    },
    None => {
      parser.next_token();
    },
  }

  // Set the variable name to the statement.
  statement.name = Expression::from_token(&parser.current_token);

  // Check if the next character is a colon (:).
  if parser.expect_sign(&data::Signs::COLON) == false {
    let line = parser.get_error_line(format!("{} {}", token_name, statement.name.string()).as_str());

    parser.errors.push(format!("{} expect `:`, got `{}` instead.", line, parser.peek_token.value));

    return None;
  }

  // Check if the next token is a valid data type.
  if parser.expect_token(&data::Tokens::TYPE) == false {
    let line = parser.get_error_line(format!("{} {}: ", token_name, statement.name.string()).as_str());

    parser.errors.push(format!("{} `{}` is not a valid data type.", line, parser.peek_token.value));

    return None;
  }

  // Set the data type to the statement.
  statement.data_type = parser.current_token.clone();

  // Set temp data to environment.
  env.set(statement.name.value.clone(), Objects::empty(statement.data_type.data_type.clone()));

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

    return None;
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

    return None;
  }

  // Go to the next token.
  parser.next_token();

  // Set the variable value to the statement.
  match expression_parse(parser, Precedence::LOWEST, env) {
    Some(value) => {
      if expression_is_valid_type(&statement.data_type.data_type, &value) == false {
        let left_line = format!(
          "{} | {} {}: {} = ",
          value.clone().token().line,
          token_name,
          statement.name.string(),
          statement.data_type.value
        );

        let line = format!(
          "{}{}\n{}{}",
          left_line,
          value.clone().string(),
          repeat_character(left_line.len(), " "),
          repeat_character(value.clone().string().len(), "^"),
        );

        parser.errors.push(format!("{} `{}` not satisfied the {} data type.", line, value.string(), statement.data_type.value));

        return None;
      }

      statement.value = Some(value);
    },

    None => {},
  }

  if parser.peek_token_is_sign(&data::Signs::SEMICOLON) == true {
    parser.next_token();
  }

  parser.next_token();

  Some(statement)
}

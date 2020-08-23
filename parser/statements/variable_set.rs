use crate::Environment;
use crate::expressions::{Expressions, Number, parse as parse_expression};
use crate::{Parser, Precedence};
use crate::tokens::*;
use crate::types::expression_is_type;

use super::{Statement, Statements};

#[derive(Debug, Clone, PartialEq)]
pub struct VariableSet {
  pub token: Token,
  pub assign: Token,
  pub value: Option<Box<Expressions>>,
}

impl Statement for VariableSet {
  fn new() -> VariableSet {
    VariableSet {
      token: Token::new_empty(),
      assign: Token::new_empty(),
      value: None,
    }
  }

  fn from_token(token: Token) -> VariableSet {
    let mut variable: VariableSet = Statement::new();

    variable.token = token;

    variable
  }

  fn string(self) -> String {
    if self.assign.token.clone().is_sign(Signs::PLUSPLUS) ||
      self.assign.token.clone().is_sign(Signs::MINUSMINUS) {
      return format!("{}{};", self.token.value, self.assign.value);
    }

    format!(
      "{} {} {};",
      self.token.value,
      self.assign.value,
      match self.value {
        Some(value) => value.string(),
        None => String::new(),
      },
    )
  }
}

impl VariableSet {
  pub fn parse<'a>(parser: &'a mut Parser, environment: &mut Environment) -> Option<Box<Statements>> {
    let mut variable: VariableSet = Statement::from_token(parser.current_token.clone());

    // Parse assigns signs.
    if parser.expect_token(Signs::new(Signs::ASSIGN)) ||
      parser.expect_token(Signs::new(Signs::PLUSASSIGN)) ||
      parser.expect_token(Signs::new(Signs::MINUSASSIGN)) ||
      parser.expect_token(Signs::new(Signs::MULTIPLYASSIGN)) ||
      parser.expect_token(Signs::new(Signs::DIVIDEASSIGN)) {
      // Set the variable assign token.
      variable.assign = parser.current_token.clone();

      // Get the next token.
      parser.next_token();

      // Parse the value expression.
      match parse_expression(parser, Precedence::LOWEST, environment) {
        Some(value_exp) => {
          variable.value = Some(value_exp);
        },
        None => {},
      }
    }

    // Parse ++ and -- signs.
    else if parser.expect_token(Signs::new(Signs::PLUSPLUS)) ||
      parser.expect_token(Signs::new(Signs::MINUSMINUS)) {
      // Set the variable assign token.
      variable.assign = parser.current_token.clone();

      // Get the next token.
      parser.next_token();

      // Set a one value.
      variable.value = Some(
        Number::new_box_from_token(
          Token::new(
            Box::new(Tokens::NUMBER),
            String::from("1"),
            0,
            0,
          )
        )
      );
    }

    // Check if the next token is a semicolon.
    if parser.next_token_is(Signs::new(Signs::SEMICOLON)) {
      // Get the next token.
      parser.next_token();
    }

    // Check if the next token is the end of line.
    if parser.next_token_is(Box::new(Tokens::EOL)) {
      // Get the next token.
      parser.next_token();
    }

    // Get the environment expression for the variable name.
    match environment.get_statement(variable.token.value.clone()) {
      Some(env_stmt) => {
        // Check if the statement is a variable.
        match env_stmt.clone().get_variable() {
          // Is a variable.
          Some(var) => {
            // Check if the original variable is a const.
            if var.token.token.clone().is_keyword(Keywords::CONST) {
              let line = parser.get_error_line(
                variable.token.line - 1,
                variable.token.position - 1,
                variable.token.value.len()
              );

              parser.errors.push(format!("{} you can not set a value to a const.", line));

              return None;
            }

            if !variable.assign.token.clone().is_sign(Signs::ASSIGN) &&
              !var.data_type.token.clone().is_type(Types::NUMBER) {
              let line = parser.get_error_line(
                variable.token.line - 1,
                variable.token.position - 1,
                variable.token.value.len()
              );

              parser.errors.push(format!("{} the variable is not of number type.", line));

              return None;
            }

            // Parse new value.
            match variable.value.clone() {
              Some(new_value) => {
                match var.data_type.token.clone().get_type() {
                  Some(data_type) => {
                    if !expression_is_type(data_type.clone(), new_value.clone(), environment) {
                      let line = parser.get_error_line(
                        new_value.clone().token().line - 1,
                        new_value.clone().token().position - 1,
                        new_value.clone().string().len(),
                      );

                      parser.errors.push(
                        format!(
                          "{} `{}` not satisfied the {} type.",
                          line,
                          new_value.string(),
                          var.data_type.value,
                        )
                      );

                      return None;
                    }
                  },
                  None => {},
                }
              },
              None => {},
            }
          },

          // Is not a variable.
          None => {
            let line = parser.get_error_line(
              variable.token.line - 1,
              variable.token.position - 1,
              variable.token.value.len()
            );

            parser.errors.push(format!("{} identifier is not a variable.", line));

            return None;
          },
        }
      },
      None => {
        let line = parser.get_error_line(
          variable.token.line - 1,
          variable.token.position - 1,
          variable.token.value.len()
        );

        parser.errors.push(format!("{} identifier not found.", line));

        return None;
      },
    }

    // Return variable set statement.
    Some(Box::new(Statements::VARIABLESET(variable)))
  }
}

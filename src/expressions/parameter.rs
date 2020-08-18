use crate::compiler::{environment::Environment, expression::evaluate};
use crate::data::{Token, Signs, Tokens};
use crate::objects::{Objects, error::is_error};
use crate::parser::{Parser, precedence::Precedence};
use crate::utils::{repeat_character, types::{object_is_valid_type, expression_is_valid_type}};

use super::{Expression, Expressions, parse as expression_parse};

// EXPRESSION //
#[derive(Debug, Clone, PartialEq)]
pub struct Parameter {
  pub name: Token,
  pub data_type: Token,
  pub default_value: Option<Box<Expressions>>,
}

impl Expression for Parameter {
  fn new() -> Parameter {
    Parameter {
      name: Token::empty(),
      data_type: Token::empty(),
      default_value: None,
    }
  }

  fn from_token(token: &Token) -> Parameter {
    let mut exp: Parameter = Expression::new();

    exp.name = token.clone();

    exp
  }

  fn string(self) -> String {
    let param = format!("{}: {}", self.name.value, self.data_type.value);

    match self.default_value {
      Some(default) => format!("{} = {}", param, default.string()),
      None => param,
    }
  }
}
// END EXPRESSION //


// PARSER //
pub fn parse<'a>(parser: &'a mut Parser, env: &mut Environment) -> Option<Vec<Box<Expressions>>> {
  let mut parameters: Vec<Box<Expressions>> = Vec::new();

  if parser.peek_token_is_sign(&Signs::RIGHTPARENTHESES) {
    parser.next_token();
  }

  let mut has_default = false;
  while !parser.current_token_is_sign(&Signs::RIGHTPARENTHESES) {
    if !parser.peek_token_is(&Tokens::IDENTIFIER) {
      let line = parser.get_error_line("");

      parser.errors.push(format!("{} `{}` is not a valid parameter name.", line, parser.peek_token.value));

      return None;
    }

    // Check if the parameter name is in use.
    match env.clone().get_first(parser.peek_token.value.clone()) {
      Some(_) => {
        let line = parser.get_error_line("");

        parser.errors.push(format!("{} `{}` is already in use.", line, parser.peek_token.value));

        return None;
      },
      None => {
        parser.next_token();
      },
    }

    let mut parameter: Parameter = Expression::from_token(&parser.current_token.clone());

    if !parser.expect_sign(&Signs::COLON) {
      let line = parser.get_error_line(format!("{}", parameter.name.value).as_str());

      parser.errors.push(format!("{} expect `:`, got `{}` instead.", line, parser.peek_token.value));

      return None;
    }

    if !parser.expect_token(&Tokens::TYPE) {
      let line = parser.get_error_line(format!("{}: ", parameter.name.value).as_str());

      parser.errors.push(format!("{} `{}` is not a valid data type.", line, parser.peek_token.value));

      return None;
    }

    parameter.data_type = parser.current_token.clone();

    env.set(parameter.name.value.clone(), Objects::empty(parameter.data_type.data_type.clone()));

    if parser.peek_token_is_sign(&Signs::COMMA) {
      parser.next_token();
    } else if parser.peek_token_is_sign(&Signs::ASSIGN) {
      has_default = true;
      parser.next_token();
      parser.next_token();

      match expression_parse(parser, Precedence::LOWEST, env) {
        Some(value_exp) => {
          let left_line = format!("{} | {}: {} = ", value_exp.clone().token().line, parameter.name.value, parameter.data_type.value);
  
          let line = format!(
            "{}{}\n{}{}",
            left_line,
            value_exp.clone().string(),
            repeat_character(left_line.len(), " "),
            repeat_character(value_exp.clone().string().len(), "^"),
          );

          match evaluate(value_exp.clone(), env) {
            Some(obj) => {
              if is_error(obj.clone()) {
                parser.errors.push(format!("{} {}", line, obj.string()));
    
                return None;
              }
    
              if !object_is_valid_type(&parameter.data_type.data_type, obj) {
                parser.errors.push(
                  format!(
                    "{} `{}` not satisfied the `{}` data type.",
                    line,
                    value_exp.clone().string(),
                    parameter.data_type.value
                  )
                );
        
                return None;
              }

              parser.next_token();
            },
            None => {
              if !expression_is_valid_type(&parameter.data_type.data_type, &value_exp.clone()) {    
                parser.errors.push(
                  format!(
                    "{} `{}` not satisfied the {} data type.",
                    line,
                    value_exp.clone().string(),
                    parameter.data_type.value
                  )
                );
    
                return None;
              }
            },
          }

          parameter.default_value = Some(value_exp);
        },
        None => {},
      }
    } else if has_default {
      let line = parser.get_error_line(format!("{}: {}", parameter.name.value, parameter.data_type.value).as_str());

      parser.errors.push(format!("{} `{}` must has default value.", line, parameter.name.value));

      return None;
    }

    if parser.peek_token_is_sign(&Signs::RIGHTPARENTHESES) {
      parser.next_token();
    }

    parameters.push(Box::new(Expressions::PARAMETER(parameter)));
  }

  Some(parameters)
}
// END PARSER //

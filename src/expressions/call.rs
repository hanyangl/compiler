use crate::compiler::{environment::Environment, expression::evaluate};
use crate::data::{Token, Signs};
use crate::objects::{Objects, error::is_error};
use crate::parser::Parser;
use crate::statements::expression::parse_list;
use crate::utils::{repeat_character, types::object_is_valid_type};

use super::{Expression, Expressions, identifier::Identifier};

// EXPRESSION //
#[derive(Debug, Clone, PartialEq)]
pub struct Call {
  pub token: Token,
  pub function: Box<Expressions>,
  pub arguments: Vec<Box<Expressions>>,
  pub semicolon: Option<Token>,
}

impl Expression for Call {
  fn new() -> Call {
    Call {
      token: Token::empty(),
      function: Identifier::new(),
      arguments: Vec::new(),
      semicolon: None,
    }
  }

  fn from_token(token: &Token) -> Call {
    let mut exp: Call = Expression::new();

    exp.token = token.clone();

    exp
  }

  fn string(self) -> String {
    let mut args: Vec<String> = Vec::new();

    for argument in self.arguments {
      args.push(argument.string());
    }

    format!(
      "{}({}){}",
      self.function.string(),
      args.join(", "),
      match self.semicolon {
        Some(x) => format!("{}", x.value),
        None => "".to_string(),
      }
    )
  }
}
// END EXPRESSION //


// PARSER //
pub fn parse<'a>(
  parser: &'a mut Parser,
  function: Option<Box<Expressions>>,
  env: &mut Environment,
) -> Option<Call> {
  let mut exp: Call = Expression::from_token(&parser.current_token.clone());

  match function {
    Some(x) => {
      exp.function = x;
    },
    None => {},
  }

  // Get call arguments.
  exp.arguments = parse_list(parser, Signs::RIGHTPARENTHESES, env);

  match evaluate(exp.function.clone(), env) {
    Some(obj) => match obj.get_function() {
      Some(function) => {
        let mut min_size: usize = 0;
        let mut max_size: usize = 0;

        for param_exp in function.parameters.iter() {
          match param_exp.clone().get_parameter() {
            Some(param) => {
              env.set(param.name.value, Objects::empty(param.data_type.data_type));

              match param.default_value {
                Some(_) => {
                  max_size += 1;
                },
                None => {
                  min_size += 1;
                  max_size += 1;
                },
              }
            },
            None => {},
          }
        }

        let mut args: Vec<String> = Vec::new();

        for arg_exp in exp.arguments.clone().iter() {
          args.push(arg_exp.clone().string());
        }

        let line = parser.get_error_line(format!("{}({}", function.name.value, args.join(", ")).as_str());

        if exp.arguments.len() < min_size {
          parser.errors.push(format!("{} expect minimum {} arguments, got {} instead.", line, min_size, exp.arguments.len()));

          return None;
        }

        if exp.arguments.len() > max_size {
          parser.errors.push(format!("{} expect maximum {} arguments, got {} instead.", line, max_size, exp.arguments.len()));

          return None;
        }

        let mut i: usize = 0;
        for arg_exp in exp.arguments.iter() {
          match evaluate(arg_exp.clone(), env) {
            Some(arg) => match function.parameters[i].clone().get_parameter() {
              Some(param) => {
                let mut args: Vec<String> = Vec::new();

                if i > 1 {
                  for arg_exp in exp.arguments[0..i-1].iter() {
                    args.push(arg_exp.clone().string());
                  }
                } else {
                  args.push(exp.arguments[0].clone().string());
                }

                let mut left_line = format!("{} | {}(", arg_exp.clone().token().line, function.name.value);

                if exp.arguments.len() > 1 {
                  left_line = format!("{}{}, ", left_line, args.join(", "));
                }

                let line = format!(
                  "{}{}\n{}{}",
                  left_line,
                  arg_exp.clone().string(),
                  repeat_character(left_line.len(), " "),
                  repeat_character(arg_exp.clone().string().len(), "^"),
                );

                if is_error(arg.clone()) {
                  parser.errors.push(format!("{} {}", line, arg.string()));

                  return None;
                }

                if !object_is_valid_type(&param.data_type.data_type, arg.clone()) {
                  parser.errors.push(
                    format!(
                      "{} `{}` not satisfied the `{}` data type.",
                      line,
                      arg_exp.clone().string(),
                      param.data_type.value,
                    )
                  );

                  return None;
                }
              },
              None => {},
            },
            None => {},
          }

          i += 1;
        }
      },
      None => {},
    },
    None => {},
  }

  if parser.expect_sign(&Signs::SEMICOLON) {
    exp.semicolon = Some(parser.current_token.clone());

    parser.next_token();
  }

  Some(exp)
}
// END PARSER //

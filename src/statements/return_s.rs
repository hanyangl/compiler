use crate::data::{Token, Signs, Types};
use crate::expressions::{Expressions, parse as expression_parse, function::Function};
use crate::parser::{Parser, precedence::Precedence};
use crate::utils::{repeat_character, types::expression_is_valid_type};

use super::Statement;

// EXPRESSION //
#[derive(Debug, Clone, PartialEq)]
pub struct Return {
  pub token: Token,
  pub value: Option<Box<Expressions>>,
}

impl Statement for Return {
  fn new() -> Return {
    Return {
      token: Token::empty(),
      value: None,
    }
  }

  fn from_token(token: &Token) -> Return {
    let mut statement: Return = Statement::new();

    statement.token = token.clone();

    statement
  }

  fn string(self) -> String {
    format!(
      "{}{};",
      self.token.value,
      match self.value {
        Some(x) => format!(" {}", x.string()),
        None => "".to_string(),
      },
    )
  }
}
// END EXPRESSION //


// PARSER //
pub fn parse<'a>(parser: &'a mut Parser) -> Return {
  let mut statement: Return = Statement::from_token(&parser.current_token.clone());

  parser.next_token();

  statement.value = expression_parse(parser, Precedence::LOWEST);

  if parser.peek_token_is_sign(&Signs::SEMICOLON) {
    parser.next_token();
  }

  statement
}

pub fn parse_type<'a>(parser: &'a mut Parser, return_stmt: &Return, exp: &Function) -> bool {
  match return_stmt.value.clone() {
    // With return value.
    Some(value) => {
      let left_line = format!("{} | return ", value.clone().token().line);
  
      let line = format!(
        "{}{}\n{}{}",
        left_line,
        value.clone().string(),
        repeat_character(left_line.len(), " "),
        repeat_character(value.clone().string().len(), "^"),
      );

      if exp.return_type.data_type == Types::VOID {
        parser.errors.push(format!("{} the `{}` function no return a value.", line, exp.name.value));

        return false;
      }

      if expression_is_valid_type(&exp.return_type.data_type, &value) == false {
        parser.errors.push(format!("{} `{}` not satisfied the {} data type.", line, value.clone().string(), exp.return_type.value));

        return false;
      }

      true
    },

    // No value
    None => {
      if exp.return_type.data_type != Types::VOID {
        let line = parser.get_error_line("return");

        parser.errors.push(format!("{} the '{}' function returns a {}.", line, exp.name.value, exp.return_type.value));

        return false;
      }

      true
    },
  }
}
// END PARSER //

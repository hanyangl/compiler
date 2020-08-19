use super::expressions::Expressions;
use super::tokens::*;

pub fn expression_is_type(data_type: Types, expression: Box<Expressions>) -> bool {
  match data_type {
    // String
    Types::STRING => match expression.get_string() {
      Some(string) => string.token.token.is_string(),
      None => false,
    },

    // Number
    Types::NUMBER => match expression.get_number() {
      Some(number) => number.token.token.is_number(),
      None => false,
    },

    // Default
    _ => false,
  }
}

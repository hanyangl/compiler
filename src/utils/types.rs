use crate::data;
use crate::expressions::Expressions;

/// Comprobate if a value is of a specific type.
/// 
/// ## Example
/// ```
/// use crate::data::{Token, Types};
/// use crate::utils::expression;
/// 
/// let value = Token::from_value(String::from("10"), 1, 1);
/// 
/// let is_string = expression::token_is_valid_type(&Types::STRING, &value);
/// // Return: false
/// 
/// let is_number = expression::token_is_valid_type(&Types::NUMBER, &value);
/// // Return true
/// ```
pub fn token_is_valid_type(data_type: &data::Types, token: &data::Token) -> bool {
  match data_type {
    // Undefined
    data::Types::UNDEFINED => token.token == data::Tokens::TYPE && token.data_type == data::Types::UNDEFINED,

    // Null
    data::Types::NULL => token.token == data::Tokens::TYPE && token.data_type == data::Types::NULL,

    // String
    data::Types::STRING => token.token == data::Tokens::STRING,

    // Integer
    data::Types::NUMBER => token.token == data::Tokens::INTEGER,

    // Boolean (true, false)
    data::Types::BOOLEAN => (
      token.token == data::Tokens::TYPE && (
        token.data_type == data::Types::TRUE ||
        token.data_type == data::Types::FALSE
      )
    ),

    // Default
    _ => false,
  }
}

pub fn expression_is_valid_type(data_type: &data::Types, expression: &Box<Expressions>) -> bool {
  match data_type {
    // Undefined
    data::Types::UNDEFINED => match expression.clone().get_identifier() {
      Some(default) => default.token.token == data::Tokens::TYPE && default.token.data_type == data::Types::UNDEFINED,
      None => false,
    },

    // Null
    data::Types::NULL => match expression.clone().get_identifier() {
      Some(default) => default.token.token == data::Tokens::TYPE && default.token.data_type == data::Types::NULL,
      None => false,
    },

    // String
    data::Types::STRING => match expression.clone().get_string() {
      Some(string) => string.token.token == data::Tokens::STRING,
      None => false,
    },

    // Integer
    data::Types::NUMBER => match expression.clone().get_integer() {
      Some(integer) => integer.token.token == data::Tokens::INTEGER,
      None => false,
    },

    // Boolean
    data::Types::BOOLEAN => match expression.clone().get_boolean() {
      Some(boolean) => boolean.token.token == data::Tokens::TYPE && (
        boolean.token.data_type == data::Types::TRUE ||
        boolean.token.data_type == data::Types::FALSE
      ),
      None => false,
    }

    // Default
    _ => false,
  }
}

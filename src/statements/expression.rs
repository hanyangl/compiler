use crate::data;

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
    data::Types::UNDEFINED => token.token == data::Tokens::TYPE && token.data_type == data::Types::UNDEFINED,
    data::Types::NULL => token.token == data::Tokens::TYPE && token.data_type == data::Types::NULL,

    data::Types::STRING => token.token == data::Tokens::STRING,
    data::Types::NUMBER => token.token == data::Tokens::INTEGER,

    data::Types::BOOLEAN => (
      token.token == data::Tokens::TYPE && (
        token.data_type == data::Types::TRUE ||
        token.data_type == data::Types::FALSE
      )
    ),

    _ => false,
  }
}

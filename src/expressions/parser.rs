use crate::compiler::environment::Environment;
use crate::data;
use crate::parser::{Parser, precedence::Precedence};

use super::*;

pub fn parse<'a>(parser: &'a mut Parser, precedence: Precedence, env: &mut Environment) -> Option<Box<Expressions>> {
  let token: data::Token = parser.current_token.clone();

  let mut left: Option<Box<Expressions>> = match token.token {
    // Parse identifiers.
    data::Tokens::IDENTIFIER => Some(Box::new(Expressions::IDENTIFIER(Expression::from_token(&token)))),

    // Parse strings.
    data::Tokens::STRING => Some(Box::new(Expressions::STRING(Expression::from_token(&token)))),

    // Parse numbers.
    data::Tokens::INTEGER => match integer::parse(parser) {
      Some(x) => Some(Box::new(Expressions::INTEGER(x))),
      None => None,
    },

    // Parse keywords.
    data::Tokens::KEYWORD => match token.keyword {
      // Parse if expression.
      data::Keywords::IF => match if_else::parse(parser, env) {
        Some(x) => Some(Box::new(Expressions::IFELSE(x))),
        None => None,
      },

      // Parse function.
      data::Keywords::FUNCTION => match function::parse(parser, env) {
        Some(x) => Some(Box::new(Expressions::FUNCTION(x))),
        None => None,
      },

      // Default
      _ => None,
    },

    // Parse signs.
    data::Tokens::SIGN => match token.sign {
      // Parse '!' and '-' signs.
      data::Signs::NEGATION | data::Signs::MINUS => Some(Box::new(Expressions::PREFIX(prefix::parse(parser, env)))),

      // Default
      _ => None,
    },

    // Parse data types.
    data::Tokens::TYPE => match token.data_type {
      // Parse undefined and null values.
      data::Types::UNDEFINED |
      data::Types::NULL => Some(Box::new(Expressions::IDENTIFIER(Expression::from_token(&token)))),

      // Parse true and false values.
      data::Types::TRUE |
      data::Types::FALSE => Some(Box::new(Expressions::BOOLEAN(boolean::parse(parser)))),

      // Default
      _ => None,
    },

    // Default
    _ => None,
  };

  while parser.peek_token_is_sign(&data::Signs::SEMICOLON) == false && precedence < parser.peek_precedence() {
    let peek_token: data::Token = parser.peek_token.clone();

    match peek_token.sign {
      data::Signs::PLUS |
      data::Signs::MINUS |
      data::Signs::DIVIDE |
      data::Signs::MULTIPLY |
      data::Signs::EQUAL |
      data::Signs::EQUALTYPE |
      data::Signs::NOTEQUAL |
      data::Signs::NOTEQUALTYPE |
      data::Signs::LESSTHAN |
      data::Signs::LESSOREQUALTHAN |
      data::Signs::GREATERTHAN |
      data::Signs::GREATEROREQUALTHAN => {
        parser.next_token();

        left = Some(Box::new(Expressions::INFIX(infix::parse(parser, left, env))));
      },

      data::Signs::ARROW => {
        parser.next_token();

        left = Some(Box::new(Expressions::METHOD(method::parse(parser, left, env))));
      },

      data::Signs::LEFTPARENTHESES => {
        parser.next_token();

        left = match call::parse(parser, left, env) {
          Some(call_exp) => Some(Box::new(Expressions::CALL(call_exp))),
          None => None,
        }
      }

      _ => break,
    }
  }

  left
}

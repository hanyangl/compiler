use crate::Parser;
use crate::tokens::Token;

use super::*;

pub fn parse<'a>(parser: &'a mut Parser) -> Option<Box<Expressions>> {
  let current_token: Token = parser.current_token.clone();
  let mut expression: Option<Box<Expressions>> = None;

  // Parse identifiers.
  if current_token.token.clone().is_identifier() {
    expression = Some(Identifier::new_box_from_token(current_token.clone()));
  }

  // Parse strings.
  if current_token.token.clone().is_string() {
    expression = Some(StringE::new_box_from_token(current_token.clone()));
  }

  // Parse numbers.
  if current_token.token.clone().is_number() {
    expression = Number::parse(parser);
  }

  expression
}

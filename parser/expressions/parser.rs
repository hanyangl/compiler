use crate::{Environment, Parser, Precedence};
use crate::tokens::*;

use super::*;

pub fn parse<'a>(
  parser: &'a mut Parser,
  data_type: Option<Token>,
  precedence: Precedence,
  environment: &mut Environment,
  standard_library: bool,
) -> Option<Box<Expressions>> {
  let current_token: Token = parser.current_token.clone();
  let mut expression: Option<Box<Expressions>> = None;

  // Parse identifiers.
  if current_token.token.clone().is_identifier() &&
    !parser.next_token.token.clone().expect_sign(Signs::LEFTBRACKET) &&
    !parser.next_token.token.clone().expect_sign(Signs::LEFTPARENTHESES) {
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

  // Parse booleans.
  if current_token.token.clone().expect_keyword(Keywords::TRUE) ||
    current_token.token.clone().expect_keyword(Keywords::FALSE) {
    expression = Some(Boolean::parse(parser));
  }

  // Parse prefixes.
  if current_token.token.clone().expect_sign(Signs::NEGATION) ||
    current_token.token.clone().expect_sign(Signs::MINUS) {
    expression = Prefix::parse(parser, environment, standard_library);
  }

  // Parse anonymous functions.
  if current_token.token.clone().expect_keyword(Keywords::FUNCTION) || (
    current_token.token.clone().expect_sign(Signs::LEFTPARENTHESES) && (
      parser.next_token.token.clone().is_identifier() ||
      parser.next_token.token.clone().expect_sign(Signs::RIGHTPARENTHESES)
    )
  ) {
    expression = AnonymousFunction::parse(parser, environment, standard_library);
  }

  // Parse calls.
  if current_token.token.clone().is_identifier() &&
    parser.next_token_is(Signs::new(Signs::LEFTPARENTHESES)) {
    expression = Call::parse(parser, environment, standard_library);
  }

  // Parse hashmaps.
  if current_token.token.clone().expect_sign(Signs::LEFTBRACE) {
    expression = HashMap::parse(parser, environment, standard_library);
  }

  // Parse arrays.
  if current_token.token.clone().expect_sign(Signs::LEFTBRACKET) {
    expression = Array::parse(parser, data_type, environment, standard_library);
  }

  // Parse array index.
  if current_token.token.clone().is_identifier() &&
    parser.next_token_is(Signs::new(Signs::LEFTBRACKET)) {
    expression = ArrayIndex::parse(parser, environment, standard_library);
  }

  // Parse infix expression.
  while !parser.next_token_is(Signs::new(Signs::SEMICOLON)) &&
    precedence < parser.next_precedence()
  {
    let next_token: Box<Tokens> = parser.next_token.token.clone();

    // Parse Infix
    if next_token.clone().expect_sign(Signs::PLUS) ||
      next_token.clone().expect_sign(Signs::MINUS) ||
      next_token.clone().expect_sign(Signs::DIVIDE) ||
      next_token.clone().expect_sign(Signs::MULTIPLY) ||
      next_token.clone().expect_sign(Signs::EMPOWERMENT) ||
      next_token.clone().expect_sign(Signs::MODULE) ||
      next_token.clone().expect_sign(Signs::EQUAL) ||
      next_token.clone().expect_sign(Signs::EQUALTYPE) ||
      next_token.clone().expect_sign(Signs::NOTEQUAL) ||
      next_token.clone().expect_sign(Signs::NOTEQUALTYPE) ||
      next_token.clone().expect_sign(Signs::LESSTHAN) ||
      next_token.clone().expect_sign(Signs::LESSOREQUALTHAN) ||
      next_token.clone().expect_sign(Signs::GREATERTHAN) ||
      next_token.clone().expect_sign(Signs::GREATEROREQUALTHAN) {
      // Get the next token.
      parser.next_token();

      // Set the new expression.
      expression = Some(Infix::parse(parser, expression, environment, standard_library));

      continue;
    }

    // Parse methods.
    if next_token.clone().expect_sign(Signs::ARROW) {
      // Get the next token.
      parser.next_token();

      // Set the new expression.
      expression = Method::parse(parser, expression, environment, standard_library);

      continue;
    }

    break;
  }

  // Return expression.
  expression
}

use crate::Parser;
use crate::tokens::*;

pub fn parse<'a>(parser: &'a mut Parser) -> Option<Token> {
  let current_token = parser.current_token.clone();

  // Check if the next token is an assign or a right brace.
  if current_token.token.clone().is_type() && (
    parser.next_token_is(Signs::new(Signs::ASSIGN)) ||
    parser.next_token_is(Signs::new(Signs::RIGHTBRACE))
  ) {
    return Some(parser.current_token.clone());
  }

  // Parse `<data_type>[]`
  if current_token.token.clone().is_type() &&
    parser.expect_token(Signs::new(Signs::LEFTBRACKET)) &&
    parser.expect_token(Signs::new(Signs::RIGHTBRACKET)) {
    return Some(Token::from_value(format!("{}[]", current_token.value), current_token.line, current_token.position));
  }

  None
}

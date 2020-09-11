use crate::Parser;
use crate::tokens::*;

pub fn parse_type<'a>(parser: &'a mut Parser) -> Result<Token, ()> {
  let token: Token =
    if parser.current_token.token.clone().get_type().is_some() ||
      parser.current_token.token.clone().is_identifier() {
      parser.current_token.clone()
    } else {
      match Function::parse(parser) {
        Ok(token) => token,
        Err(_) => Token::new_empty(),
      }
    };

  // Parse arrays.
  if token.token.clone().get_type().is_some() ||
    token.token.clone().is_identifier() {
    match Array::parse(parser, token.clone()) {
      Ok(token) => Ok(token),
      Err(_) => Ok(token),
    }
  } else {
    Err(())
  }
}

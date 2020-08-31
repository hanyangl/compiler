use crate::Parser;
use crate::tokens::*;

pub fn parse_type<'a>(parser: &'a mut Parser, from_group: bool) -> Result<Token, ()> {
  let token: Token = if parser.current_token.token.clone().is_type() {
    parser.current_token.clone()
  } else {
    match Group::parse(parser) {
      Ok(token) => token,
      Err(code) => if code == 0 {
        match Function::parse(parser) {
          Ok(token) => token,
          Err(code) => if code == 0 {
            match HashMap::parse(parser) {
              Ok(token) => token,
              Err(_) => Token::new_empty(),
            }
          } else {
            Token::new_empty()
          },
        }
      } else {
        Token::new_empty()
      },
    }
  };

  // Parse arrays.
  if token.token.clone().is_type() {
    match Array::parse(parser, token.clone()) {
      Ok(token) => Ok(token),
      Err(_) => if from_group {
        Ok(token)
      } else {
        match Group::parse_without_parentheses(parser, token.clone()) {
          Ok(token) => Ok(token),
          Err(code) => if code == 0 {
            Ok(token)
          } else {
            Err(())
          }
        }
      },
    }
  } else {
    Err(())
  }
}

use super::{
  Lexer,
  Precedence,
  tokens::{
    Token,
    Tokens,
  },
};

#[derive(Debug)]
pub struct Parser {
  pub lexer: Lexer,

  pub current_token: Token,
  pub next_token: Token,
}

impl Parser {
  pub fn new(lexer: Lexer) -> Parser {
    let mut parser = Parser {
      lexer,

      current_token: Token::new_empty(),
      next_token: Token::new_empty(),
    };

    parser.next_token();
    parser.next_token();

    parser
  }

  pub fn next_token(&mut self) {
    std::mem::swap(&mut self.current_token, &mut self.next_token);
    self.next_token = self.lexer.read_next_token();
  }

  pub fn current_precedence(&mut self) -> Precedence {
    if let Some(keyword) = self.current_token.token.clone().get_keyword() {
      Precedence::from_keyword(keyword)
    } else if let Some(sign) = self.current_token.token.clone().get_sign() {
      Precedence::from_sign(sign)
    } else {
      Precedence::LOWEST
    }
  }

  pub fn next_precedence(&mut self) -> Precedence {
    if let Some(keyword) = self.next_token.token.clone().get_keyword() {
      Precedence::from_keyword(keyword)
    } else if let Some(sign) = self.next_token.token.clone().get_sign() {
      Precedence::from_sign(sign)
    } else {
      Precedence::LOWEST
    }
  }

  pub fn current_token_is(&mut self, token: Box<Tokens>) -> bool {
    self.current_token.token == token
  }

  pub fn next_token_is(&mut self, token: Box<Tokens>) -> bool {
    self.next_token.token == token
  }

  pub fn expect_token(&mut self, token: Box<Tokens>) -> bool {
    if self.next_token_is(token) {
      self.next_token();
      true
    } else {
      false
    }
  }
}

use super::{
  Lexer,
  tokens::*,
  statements::*,
  Precedence,
  utils::repeat_character
};

#[derive(Debug, Clone)]
pub struct Parser {
  lexer: Lexer,
  pub errors: Vec<String>,

  pub current_token: Token,
  pub next_token: Token,
}

impl Parser {
  pub fn new(lexer: Lexer) -> Parser {
    let mut parser = Parser {
      lexer,
      errors: Vec::new(),

      current_token: Token::new_empty(),
      next_token: Token::new_empty(),
    };

    parser.next_token();
    parser.next_token();

    parser
  }

  pub fn show_errors(&mut self) {
    println!("{}", self.errors.join("\n"));
  }

  pub fn next_token(&mut self) {
    std::mem::swap(&mut self.current_token, &mut self.next_token);
    self.next_token = self.lexer.read_next_token();
  }

  pub fn current_precedence(self) -> Precedence {
    match self.current_token.token.clone().get_sign() {
      Some(sign) => Precedence::from_sign(sign),
      None => Precedence::LOWEST,
    }
  }

  pub fn next_precedence(self) -> Precedence {
    match self.next_token.token.clone().get_sign() {
      Some(sign) => Precedence::from_sign(sign),
      None => Precedence::LOWEST,
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

  pub fn get_error_line(&mut self, position: usize, size: usize) -> String {
    let line = self.lexer.clone().get_lines()[self.current_token.line - 1].clone();

    format!("{}\n{}{}", line, repeat_character(position, " "), repeat_character(size, "^"))
  }

  pub fn get_error_line_current_token(&mut self) -> String {
    self.get_error_line(self.current_token.position - 1, self.current_token.value.len())
  }

  pub fn get_error_line_next_token(&mut self) -> String {
    self.get_error_line(self.next_token.position - 1, self.next_token.value.len())
  }

  fn parse_statement(&mut self) -> Option<Box<Statements>> {
    let current_token = self.current_token.token.clone();

    // Parse variable statement.
    if current_token.clone().is_keyword(Keywords::LET) || current_token.clone().is_keyword(Keywords::CONST) {
      return Variable::parse(self);
    }

    // Parse expression statement.
    Some(ExpressionStatement::parse(self))
  }

  pub fn parse_program(&mut self) -> Vec<Box<Statements>> {
    let mut statements: Vec<Box<Statements>> = Vec::new();

    while !self.current_token_is(Box::new(Tokens::EOF)) {
      match self.parse_statement() {
        Some(statement) => {
          statements.push(statement);
        },
        None => {}
      }

      self.next_token();
    }

    statements
  }
}

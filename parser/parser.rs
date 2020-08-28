use super::{Environment, Lexer, Precedence};
use super::statements::*;
use super::tokens::*;
use super::utils::repeat_character;

#[derive(Debug, Clone)]
pub struct Parser {
  lexer: Lexer,
  pub errors: Vec<String>,

  pub last_token: Token,
  pub current_token: Token,
  pub next_token: Token,
}

impl Parser {
  pub fn new(lexer: Lexer) -> Parser {
    let mut parser = Parser {
      lexer,
      errors: Vec::new(),

      last_token: Token::new_empty(),
      current_token: Token::new_empty(),
      next_token: Token::new_empty(),
    };

    parser.next_token();
    parser.next_token();

    parser
  }

  pub fn show_errors(&mut self) {
    println!(
      "{}\n  {}  \n{}\n{}",
      repeat_character(self.lexer.file_name.len() + 4, "-"),
      self.lexer.file_name,
      repeat_character(self.lexer.file_name.len() + 4, "-"),
      self.errors.join("\n\n"),
    );
  }

  pub fn next_token(&mut self) {
    std::mem::swap(&mut self.last_token, &mut self.current_token);
    std::mem::swap(&mut self.current_token, &mut self.next_token);
    self.next_token = self.lexer.read_next_token();
  }

  pub fn current_precedence(&mut self) -> Precedence {
    match self.current_token.token.clone().get_sign() {
      Some(sign) => Precedence::from_sign(sign),
      None => Precedence::LOWEST,
    }
  }

  pub fn next_precedence(&mut self) -> Precedence {
    match self.next_token.token.clone().get_sign() {
      Some(sign) => Precedence::from_sign(sign),
      None => Precedence::LOWEST,
    }
  }

  pub fn last_token_is(&mut self, token: Box<Tokens>) -> bool {
    self.last_token.token == token
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

  pub fn get_error_line(&mut self, line: usize, position: usize, size: usize) -> String {
    let line_content = self.lexer.clone().get_lines()[line].clone();

    format!(
      "{} | {}\n{} | {}{}",
      line + 1,
      line_content,
      repeat_character((line + 1).to_string().len(), " "),
      repeat_character(position, " "),
      repeat_character(size, "^"),
    )
  }

  pub fn get_error_line_current_token(&mut self) -> String {
    self.get_error_line(self.current_token.line - 1, self.current_token.position - 1, self.current_token.value.len())
  }

  pub fn get_error_line_next_token(&mut self) -> String {
    self.get_error_line(self.next_token.line - 1, self.next_token.position - 1, self.next_token.value.len())
  }

  pub fn parse_statement(
    &mut self,
    environment: &mut Environment,
    standard_library: bool,
  ) -> Option<Box<Statements>> {
    // Parse variable statement.
    if self.current_token_is(Keywords::new(Keywords::LET)) ||
        self.current_token_is(Keywords::new(Keywords::CONST)) {
      return Variable::parse(self, environment, standard_library);
    }

    // Parse function statement.
    if self.current_token_is(Keywords::new(Keywords::FUNCTION)) {
      return Function::parse(self, environment, standard_library);
    }

    // Parse return statement.
    if self.current_token_is(Keywords::new(Keywords::RETURN)) {
      return Return::parse(self, environment, standard_library);
    }

    // Parse library statement in standard library.
    if self.current_token.value.as_str() == "library" && standard_library {
      return Library::parse(self, environment);
    }

    // Parse variable set statement.
    if self.current_token_is(Box::new(Tokens::IDENTIFIER)) &&
      !self.next_token_is(Signs::new(Signs::LEFTPARENTHESES)) &&
      !self.next_token_is(Signs::new(Signs::ARROW)) {
      return VariableSet::parse(self, environment, standard_library);
    }

    // Parse expression statement.
    ExpressionStatement::parse(self, environment, standard_library)
  }

  pub fn parse_program(
    &mut self,
    environment: &mut Environment,
    standard_library: bool,
  ) -> Vec<Box<Statements>> {
    let mut statements: Vec<Box<Statements>> = Vec::new();

    while !self.current_token_is(Box::new(Tokens::EOF)) {
      // Parse the statement.
      match self.parse_statement(environment, standard_library) {
        Some(statement) => {
          statements.push(statement);
        },
        None => {
          break;
        }
      }

      // Get the next token.
      self.next_token();
    }

    statements
  }
}

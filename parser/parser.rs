use super::{Environment, Lexer, Precedence};
use super::statements::*;
use super::tokens::*;
use super::utils::repeat_character;

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
    println!("{}", self.errors.join("\n\n"));
  }

  pub fn next_token(&mut self) {
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

  pub fn parse_statement(&mut self, environment: &mut Environment) -> Option<Box<Statements>> {
    // Parse variable statement.
    if self.current_token_is(Keywords::new(Keywords::LET)) ||
        self.current_token_is(Keywords::new(Keywords::CONST)) {
      return Variable::parse(self, environment);
    }

    // Parse variable set statement.
    if self.current_token_is(Box::new(Tokens::IDENTIFIER)) &&
      !self.next_token_is(Signs::new(Signs::LEFTPARENTHESES)) {
      return VariableSet::parse(self, environment);
    }

    // Parse function statement.
    if self.current_token_is(Keywords::new(Keywords::FUNCTION)) {
      return Function::parse(self, environment);
    }

    // Parse return statement.
    if self.current_token_is(Keywords::new(Keywords::RETURN)) {
      return Return::parse(self, environment);
    }

    // Parse expression statement.
    ExpressionStatement::parse(self, environment)
  }

  pub fn parse_program(&mut self) -> Vec<Box<Statements>> {
    let mut statements: Vec<Box<Statements>> = Vec::new();
    let mut environment = Environment::new();

    while !self.current_token_is(Box::new(Tokens::EOF)) {
      // Check if the current token is the end of line.
      if self.current_token_is(Box::new(Tokens::EOL)) {
        // Get the next token.
        self.next_token();
      }

      // Parse the statement.
      match self.parse_statement(&mut environment) {
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

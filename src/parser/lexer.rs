use crate::parser::token;
use crate::utils::texts;

#[derive(Debug)]
pub struct Lexer {
  pub current_text: String,
  pub current_position: usize,
  pub read_position: usize,
  pub current_char: u8,
  pub current_line: usize,
}

impl Lexer {
  pub fn new(current_text: String) -> Lexer {
    let mut lexer = Lexer {
      current_text,
      current_position: 0,
      read_position: 0,
      current_char: 0,
      current_line: 0,
    };

    lexer.read_char();

    lexer
  }

  pub fn peek_char(&mut self) -> u8 {
    if self.read_position >= self.current_text.len() {
      0
    } else {
      self.current_text.as_bytes()[self.read_position]
    }
  }

  pub fn read_char(&mut self) {
    self.current_position = self.read_position;
    self.read_position += 1;

    if self.current_position >= self.current_text.len() {
      self.current_char = 0;
    } else {
      self.current_char = self.current_text.as_bytes()[self.current_position];
    }
  }

  /// Skip spaces and lines separation
  pub fn skip_whitespace(&mut self) {
    loop {
      let current_char = texts::as_string(self.current_char);
      let current_char = current_char.as_str();

      if current_char != " " && current_char != "\r" && current_char != "\r" {
        break;
      }

      self.read_char();
    }
  }

  /// Read things names (classes, functions, variables, etc)
  pub fn read_identifier(&mut self) -> String {
    let position = self.current_position;

    loop {
      if texts::is_letter(self.current_char) == false {
        return self.current_text[position..self.current_position].to_string();
      }

      self.read_char();
    }
  }

  pub fn read_number(&mut self) -> String {
    let position = self.current_position;

    loop {
      if texts::is_digit(self.current_char) == false {
        return self.current_text[position..self.current_position].to_string();
      }

      self.read_char();
    }
  }

  pub fn next_token(&mut self) -> token::Token {
    self.skip_whitespace();

    if self.current_char == 0 {
      token::new(token::TokenType::EOF, String::new())
    } else {
      let mut literal = texts::as_string(self.current_char);
      let mut can_read_char: bool = true;

      let token_type = match literal.as_str() {
        "\n" => token::TokenType::EOL,

        // Operators
        "=" => (
          if texts::as_string(self.peek_char()).as_str() == "=" {
            self.read_char();
            literal = String::from("==");
            token::TokenType::EQUAL
          } else {
            token::TokenType::ASSIGN
          }
        ),
        "+" => token::TokenType::PLUS,
        "-" => token::TokenType::MINUS,
        "!" => (
          if texts::as_string(self.peek_char()).as_str() == "=" {
            self.read_char();
            literal = String::from("!=");
            token::TokenType::NOTEQUAL
          } else {
            token::TokenType::BANG
          }
        ),
        "*" => token::TokenType::ASTERISK,
        "/" => token::TokenType::SLASH,

        "<" => token::TokenType::LT,
        ">" => token::TokenType::GT,

        // Delimeters
        "," => token::TokenType::COMMA,
        ";" => token::TokenType::SEMICOLON,
        ":" => token::TokenType::COLON,

        "(" => token::TokenType::LEFTPAREN,
        ")" => token::TokenType::RIGHTPAREN,
        "{" => token::TokenType::LEFTBRACE,
        "}" => token::TokenType::RIGHTBRACE,

        // Data and keywords
        _ => (
          if texts::is_letter(self.current_char) {
            literal = self.read_identifier();
            can_read_char = false;
            token::lookup_ident(&literal)
          } else if texts::is_digit(self.current_char) {
            literal = self.read_number();
            can_read_char = false;
            token::TokenType::INT
          } else {
            token::TokenType::ILLEGAL
          }
        )
      };

      if can_read_char {
        self.read_char();
      }

      token::new(token_type, literal)
    }
  }
}
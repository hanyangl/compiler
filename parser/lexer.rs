mod file;
mod utils;

pub use file::File;

use super::tokens::*;

#[derive(Debug)]
pub struct Lexer {
  pub file: File,

  pub current_line: usize,
  pub current_line_position: usize,
  pub current_position: usize,
  pub current_character: u8,

  pub next_position: usize,
}

impl Lexer {
  pub fn new(file: File) -> Lexer {
    let mut lexer = Lexer {
      file,
      
      current_line: 1,
      current_line_position: 0,
      current_position: 0,
      current_character: 0,

      next_position: 0,
    };

    lexer.read_next_character();

    lexer
  }

  /// Read the next character and move the cursor to the next position.
  fn read_next_character(&mut self) {
    self.current_position = self.next_position;
    self.next_position += 1;
    self.current_line_position += 1;

    if self.current_position >= self.file.content.len() {
      self.current_character = 0;
    } else {
      self.current_character = self.file.content.as_bytes()[self.current_position];
    }
  }

  /// Get the next character.
  fn get_next_character(&mut self) -> &'static str {
    if self.next_position >= self.file.content.len() {
      ""
    } else {
      utils::character_to_str(self.file.content.as_bytes()[self.next_position])
    }
  }

  /// Get the two next character.
  fn get_two_next_character(&mut self) -> &'static str {
    if self.next_position + 1 >= self.file.content.len() {
      ""
    } else {
      utils::character_to_str(self.file.content.as_bytes()[self.next_position + 1])
    }
  }

  /// Ignore the whitespaces in the current line.
  fn skip_whitespace(&mut self) {
    loop {
      let current_character_str = utils::character_to_str(self.current_character.clone());

      if current_character_str == "\n" {
        self.current_line += 1;
        self.current_line_position = 0;
      }

      if current_character_str != " " && current_character_str != "\t" && current_character_str != "\n" {
        break;
      }

      self.read_next_character();
    }
  }

  /// Ignore comments.
  fn skip_comments(&mut self) {
    if utils::character_to_str(self.current_character.clone()) == "/" &&
      self.get_next_character() == "/" {
      loop {
        if utils::character_to_str(self.current_character.clone()) == "\n" {
          self.skip_whitespace();
          self.skip_comments();
          break;
        }

        self.read_next_character();
      }
    }
  }

  fn read_identifier_or_keyword(&mut self) -> String {
    let start_position = self.current_position;

    loop {
      if !utils::is_letter_identifier(self.current_character) &&
        !self.current_character.is_ascii_digit() {
        return self.file.content[start_position..self.current_position].to_string();
      }

      self.read_next_character();
    }
  }

  fn read_number(&mut self) -> String {
    let start_position = self.current_position;
    let mut has_dot = false;

    loop {
      let character_as_str = utils::character_to_str(self.current_character);

      if !has_dot && character_as_str == "." {
        has_dot = true;
        self.read_next_character();
      }

      if !utils::is_number(self.current_character) {
        return self.file.content[start_position..self.current_position].to_string();
      }

      self.read_next_character();
    }
  }

  fn read_string(&mut self, quote: u8) -> String {
    let start_position = self.current_position;
    let mut readed_first_quote = false;

    loop {
      if self.current_character == quote {
        if readed_first_quote {
          self.read_next_character();
          return self.file.content[start_position..self.current_position].to_string();
        } else {
          readed_first_quote = true;
        }
      }

      self.read_next_character();
    }
  }

  /// Read and get the next token.
  pub fn read_next_token(&mut self) -> Token {
    self.skip_whitespace();
    self.skip_comments();

    if self.current_character == 0 {
      // End Of File
      return Token::new(Box::new(Tokens::EOF), String::new(), self.current_line, self.current_line_position);
    }

    let start_position = self.current_line_position;
    let current_character_str = utils::character_to_str(self.current_character);
    let mut current_token = Token::from_value(current_character_str, self.current_line, start_position);

    // Check if the current token is a string, keyword, identifier or number.
    if current_token.token.clone().is_illegal() {
      if current_character_str == "\"" || current_character_str == "'" {
        // Read strings.
        current_token = Token::new(Box::new(Tokens::STRING), self.read_string(self.current_character), self.current_line, start_position);
      } else if utils::is_letter_identifier(self.current_character) {
        // Read identifier or keyword.
        current_token = Token::from_value(self.read_identifier_or_keyword().as_str(), self.current_line, start_position);

        // If the current token is not a keyword, set it as identifier.
        if current_token.token.clone().is_illegal() {
          current_token.token = Box::new(Tokens::IDENTIFIER);
        }
      } else if utils::is_number(self.current_character) {
        // Read numbers.
        current_token = Token::new(Box::new(Tokens::NUMBER), self.read_number(), self.current_line, start_position);
      } else {
        // Read the next character.
        self.read_next_character();
      }
    } else if let Some(sign) = current_token.token.clone().get_sign() {
      // Get the next character.
      let next_character = self.get_next_character();
      let next_two_character = self.get_two_next_character();

      // Parse "==", "===", "!=", "!==", "<=", ">=", "+=", "-=", "*=" and "/="
      if next_character == "=" && (
        sign == Signs::ASSIGN ||
        sign == Signs::NOT ||
        sign == Signs::LESSTHAN ||
        sign == Signs::GREATERTHAN ||
        sign == Signs::PLUS ||
        sign == Signs::MINUS ||
        sign == Signs::MULTIPLY ||
        sign == Signs::DIVIDE
      ) {
        // Read the next character.
        self.read_next_character();

        if next_two_character == "=" && sign != Signs::LESSTHAN && sign != Signs::GREATERTHAN &&
          sign != Signs::PLUS && sign != Signs::MINUS && sign != Signs::MULTIPLY && sign != Signs::DIVIDE {
          // Read the next character.
          self.read_next_character();

          // Set the current token to the new token.
          current_token = Token::from_value(
            format!("{}{}{}", current_token.value, next_character, next_two_character).as_str(),
            self.current_line,
            start_position,
          );
        } else {
          // Set the current token to the new token.
          current_token = Token::from_value(
            format!("{}{}", current_token.value, next_character).as_str(),
            self.current_line,
            start_position,
          );
        }
      }
      // Parse "++", "--", "**", "=>", "->", "&&" and "||"
      else if (sign == Signs::PLUS && next_character == "+") ||
        (sign == Signs::MULTIPLY && next_character == "*") ||
        (sign == Signs::ASSIGN && next_character == ">") ||
        (sign == Signs::MINUS && (next_character == "-" || next_character == ">")) ||
        (sign == Signs::BITAND && next_character == "&") ||
        (sign == Signs::BITOR && next_character == "|")
      {
        // Read the next character.
        self.read_next_character();

        // Set the current token to the new token.
        current_token = Token::from_value(
          format!("{}{}", current_token.value, next_character).as_str(),
          self.current_line,
          start_position,
        );
      }
      // Parse "..."
      else if sign == Signs::DOT && next_character == "." && next_two_character == "." {
        // Read the next character.
        self.read_next_character();

        // read the next character.
        self.read_next_character();

        // Set the current token to the new token.
        current_token = Token::from_value("...", self.current_line, start_position);
      }

      // Read the next character.
      self.read_next_character();
    }

    current_token
  }
}

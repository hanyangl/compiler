use crate::utils;
use crate::data;

#[derive(Debug, PartialEq)]
pub struct Lexer {
  pub text: String,

  pub line: usize,
  pub line_position: usize,
  pub column: usize,

  pub reading: usize,
  pub character: u8,
}

impl Lexer {
  /// Create a new lexer object in a text.
  pub fn new(text: String) -> Lexer {
    let mut object = Lexer {
      text,
      line: 1,
      line_position: 0,
      column: 0,
      reading: 0,
      character: 0,
    };

    object.read_character();

    object
  }

  /// Read the next character and move the cursor.
  pub fn read_character(&mut self) {
    self.column = self.reading;
    self.reading += 1;
    self.line_position += 1;

    if self.column >= self.text.len() {
      self.character = 0;
    } else {
      self.character = self.text.as_bytes()[self.column];
    }
  }

  /// Read the next character without move the cursor.
  pub fn peek_character(&mut self) -> u8 {
    if self.reading >= self.text.len() {
      0
    } else {
      self.text.as_bytes()[self.reading]
    }
  }

  pub fn peek_character_two(&mut self) -> u8 {
    if self.reading + 1 >= self.text.len() {
      0
    } else {
      self.text.as_bytes()[self.reading + 1]
    }
  }

  //// Skip the whitespaces in the line.
  pub fn skip_whitespace(&mut self) {
    loop {
      let current_character = utils::as_string(self.character);
      let current_character = current_character.as_str();

      if current_character != " " && current_character != "\t" {
        break;
      }

      self.read_character();
    }
  }

  /// Read identifiers (strings or numbers). If you want to read naames, use `read_identifier("string")`,
  /// otherwise use `read_identifier("number")` to read numbers.
  pub fn read_identifier(&mut self, like: &str) -> String {
    let position = self.column;

    loop {
      let value = self.text[position..self.column].to_string();

      if utils::is_letter(self.character) == false && like == "string" {
        return value;
      } else if utils::is_digit(self.character) == false && like == "number" {
        return value;
      }

      self.read_character();
    }
  }

  // Read strings with single or double quotes.
  pub fn read_string(&mut self, quote: u8) -> String {
    let position = self.column;
    let mut first_quote = false;

    loop {
      if self.character == quote {
        if first_quote == true {
          self.read_character();
          return self.text[position..self.column].to_string();
        } else {
          first_quote = true;
        }
      }

      self.read_character();
    }
  }

  /// Read and get the current token.
  pub fn read_token(&mut self) -> data::Token {
    self.skip_whitespace();

    if self.character == 0 {
      data::Token::new(data::Tokens::EOF, String::new(), self.line_position, self.line)
    } else {
      let position = self.line_position;
      let value = utils::as_string(self.character);

      let mut token = data::Token::from_value(value, position, self.line);
      if token.token == data::Tokens::ILLEGAL {
        if utils::is_quote(self.character) {
          // Check if the token is a string in single or double quotes
          token = data::Token::new(data::Tokens::STRING, self.read_string(self.character), position, self.line);
        } else if utils::is_letter(self.character) {
          // Check if the token is a keyword.
          token = data::Token::from_value(self.read_identifier("string"), position, self.line);

          // Check if the token is a string identifier like variable name or function name.
          if token.token == data::Tokens::ILLEGAL {
            token = data::Token::new(data::Tokens::IDENTIFIER, token.value, position, self.line);
          }
        } else if utils::is_digit(self.character) {
          // Check if the token is a number.
          token = data::Token::new(data::Tokens::INTEGER, self.read_identifier("number"), position, self.line);
        } else {
          // Illegal token
          self.read_character();
        }
      } else {
        if token.token == data::Tokens::SIGN {
          // Check double and triple signs like "===" or "==".
          let next_character = utils::as_string(self.peek_character());
          let next_two_character = utils::as_string(self.peek_character_two());

          if token.sign == data::Signs::ASSIGN || token.sign == data::Signs::NEGATION ||
            token.sign == data::Signs::LESSTHAN || token.sign == data::Signs::HIGHERTHAN {
            if next_character.as_str() == "=" {
              self.read_character();

              if next_two_character.as_str() == "=" && token.sign != data::Signs::LESSTHAN && token.sign != data::Signs::HIGHERTHAN {
                // Triple sign
                self.read_character();
                token = data::Token::from_value(format!("{}{}{}", token.value, next_character, next_two_character), position, self.line);
              } else {
                // Double sign
                token = data::Token::from_value(format!("{}{}", token.value, next_character), position, self.line);
              }
            }
          }
        } else if token.token == data::Tokens::EOL {
          self.line += 1;
          self.line_position = 0;
        }

        self.read_character();
      }

      token
    }
  }
}

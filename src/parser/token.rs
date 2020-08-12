#[derive(Debug)]
#[derive(PartialEq)]
pub enum TokenType {
  ILLEGAL,
  EOF, // End Of File
  EOL, // End Of Line

  // Data
  IDENTIFIER,
  INT,

  // Operators
  ASSIGN, // =
  PLUS, // +
  MINUS, // -
  BANG, // !
  ASTERISK, // *
  SLASH, // /

  LT, // <
  GT, // >

  EQUAL, // ==
  NOTEQUAL, // !=

  // Delimeters
  COMMA, // ,
  SEMICOLON, // ;
  COLON, // :

  LEFTPAREN, // (
  RIGHTPAREN, // )
  LEFTBRACE, // {
  RIGHTBRACE, // }

  // Types
  BOOLEAN,
    TRUE,
    FALSE,
  NUMBER,
  STRING,

  // Keywords
  LET,
  PRINT,
  FUNCTION,
  RETURN,
  IF,
  ELSE,
}

#[derive(Debug)]
#[derive(PartialEq)]
pub struct Token {
  pub token_type: TokenType,
  pub literal: String
}

/// Create a new token
pub fn new(token_type: TokenType, literal: String) -> Token {
  Token {
    token_type,
    literal
  }
}

pub fn lookup_ident(literal: &String) -> TokenType {
  match literal.as_str() {
    // Types
    "boolean" => TokenType::BOOLEAN,
      "true" => TokenType::TRUE,
      "false" => TokenType::FALSE,
    "number" => TokenType::NUMBER,
    "string" => TokenType::STRING,

    // Keywords
    "let" => TokenType::LET,
    "print" => TokenType::PRINT,
    "function" => TokenType::FUNCTION,
    "return" => TokenType::RETURN,
    "if" => TokenType::IF,
    "else" => TokenType::ELSE,

    // Default
    _ => TokenType::IDENTIFIER
  }
}

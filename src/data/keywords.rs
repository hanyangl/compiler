#[derive(Debug, PartialEq, Clone)]
pub enum Keywords {
  LET,
  CONST,
  PRINT,

  FUNCTION,
  RETURN,

  IF,
  ELSE,

  NONE,
}

pub fn get_keyword(value: &String) -> Keywords {
  match value.as_str() {
    "let" => Keywords::LET,
    "const" => Keywords::CONST,
    "print" => Keywords::PRINT,

    "function" => Keywords::FUNCTION,
    "return" => Keywords::RETURN,

    "if" => Keywords::IF,
    "else" => Keywords::ELSE,

    _ => Keywords::NONE,
  }
}

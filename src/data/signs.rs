#[derive(Debug, PartialEq, Clone)]
pub enum Signs {
  // Delimeters
  COMMA,
  COLON,
  SEMICOLON,

  LEFTPARENTHESES,
  RIGHTPARENTHESES,

  LEFTBRACKET,
  RIGHTBRACKET,

  LEFTBRACE,
  RIGHTBRACE,

  // Conditions  
  EQUAL,
  EQUALTYPE,
  NOTEQUAL,
  NOTEQUALTYPE,

  LESSTHAN,
  LESSOREQUALTHAN,
  HIGHERTHAN,
  HIGHEROREQUALTHAN,

  // Maths
  PLUS,
  MINUS,
  MULTIPLY,
  DIVIDE,

  // Others
  ASSIGN,
  NEGATION,

  NONE,
}

pub fn get_sign(value: &String) -> Signs {
  match value.as_str() {
    // Delimeters
    "," => Signs::COMMA,
    ":" => Signs::COLON,
    ";" => Signs::SEMICOLON,

    "(" => Signs::LEFTPARENTHESES,
    ")" => Signs::RIGHTPARENTHESES,

    "[" => Signs::LEFTBRACKET,
    "]" => Signs::RIGHTBRACKET,

    "{" => Signs::LEFTBRACE,
    "}" => Signs::RIGHTBRACKET,

    // Conditions
    "==" => Signs::EQUAL,
    "===" => Signs::EQUALTYPE,
    "!=" => Signs::NOTEQUAL,
    "!==" => Signs::NOTEQUALTYPE,

    "<" => Signs::LESSTHAN,
    "<=" => Signs::LESSOREQUALTHAN,
    ">" => Signs::HIGHERTHAN,
    ">=" => Signs::HIGHEROREQUALTHAN,

    // Maths
    "+" => Signs::PLUS,
    "-" => Signs::MINUS,
    "*" => Signs::MULTIPLY,
    "/" => Signs::DIVIDE,

    // Others
    "=" => Signs::ASSIGN,
    "!" => Signs::NEGATION,

    _ => Signs::NONE,
  }
}

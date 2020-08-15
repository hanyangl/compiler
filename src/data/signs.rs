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
  GREATERTHAN,
  GREATEROREQUALTHAN,

  // Maths
  PLUS,
  MINUS,
  MULTIPLY,
  DIVIDE,

  // Others
  ASSIGN,
  NEGATION,
  ARROW,

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
    "}" => Signs::RIGHTBRACE,

    // Conditions
    "==" => Signs::EQUAL,
    "===" => Signs::EQUALTYPE,
    "!=" => Signs::NOTEQUAL,
    "!==" => Signs::NOTEQUALTYPE,

    "<" => Signs::LESSTHAN,
    "<=" => Signs::LESSOREQUALTHAN,
    ">" => Signs::GREATERTHAN,
    ">=" => Signs::GREATEROREQUALTHAN,

    // Maths
    "+" => Signs::PLUS,
    "-" => Signs::MINUS,
    "*" => Signs::MULTIPLY,
    "/" => Signs::DIVIDE,

    // Others
    "=" => Signs::ASSIGN,
    "!" => Signs::NEGATION,
    "->" => Signs::ARROW,

    _ => Signs::NONE,
  }
}

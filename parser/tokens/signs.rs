use super::Tokens;

#[derive(Debug, Clone, PartialEq)]
pub enum Signs {
  // Delimeters
  DOT,
  DOTDOTDOT,
  COMMA,
  COLON,
  SEMICOLON,

  LEFTPARENTHESES,
  RIGHTPARENTHESES,

  LEFTBRACKET,
  RIGHTBRACKET,

  LEFTBRACE,
  RIGHTBRACE,

  // Assign
  ASSIGN,
  ASSIGNARROW,
  PLUSASSIGN,
  MINUSASSIGN,
  MULTIPLYASSIGN,
  DIVIDEASSIGN,

  PLUSPLUS,
  MINUSMINUS,

  // Conditions
  EQUAL,
  NOTEQUAL,

  LESSTHAN,
  LESSOREQUALTHAN,
  GREATERTHAN,
  GREATEROREQUALTHAN,

  AND,
  OR,

  // Operators
  PLUS,
  MINUS,
  MULTIPLY,
  DIVIDE,
  EMPOWERMENT,
  CARER,
  MODULE,

  // Others
  NOT,
  ARROW,
  AT,
  BITOR,
  BITAND,
}

impl Signs {
  pub fn new(sign: Signs) -> Box<Tokens> {
    Box::new(Tokens::SIGN(sign))
  }

  pub fn from_value(value: &str) -> Result<Signs, ()> {
    match value {
      // Delimeters
      "." => Ok(Signs::DOT),
      "..." => Ok(Signs::DOTDOTDOT),
      "," => Ok(Signs::COMMA),
      ":" => Ok(Signs::COLON),
      ";" => Ok(Signs::SEMICOLON),

      "(" => Ok(Signs::LEFTPARENTHESES),
      ")" => Ok(Signs::RIGHTPARENTHESES),

      "[" => Ok(Signs::LEFTBRACKET),
      "]" => Ok(Signs::RIGHTBRACKET),

      "{" => Ok(Signs::LEFTBRACE),
      "}" => Ok(Signs::RIGHTBRACE),

      // Assign
      "=" => Ok(Signs::ASSIGN),
      "=>" => Ok(Signs::ASSIGNARROW),
      "+=" => Ok(Signs::PLUSASSIGN),
      "-=" => Ok(Signs::MINUSASSIGN),
      "*=" => Ok(Signs::MULTIPLYASSIGN),
      "/=" => Ok(Signs::DIVIDEASSIGN),

      "++" => Ok(Signs::PLUSPLUS),
      "--" => Ok(Signs::MINUSMINUS),

      // Conditions
      "==" => Ok(Signs::EQUAL),
      "!=" => Ok(Signs::NOTEQUAL),

      "<" => Ok(Signs::LESSTHAN),
      "<=" => Ok(Signs::LESSOREQUALTHAN),
      ">" => Ok(Signs::GREATERTHAN),
      ">=" => Ok(Signs::GREATEROREQUALTHAN),

      "&&" => Ok(Signs::AND),
      "||" => Ok(Signs::OR),

      // Operators
      "+" => Ok(Signs::PLUS),
      "-" => Ok(Signs::MINUS),
      "*" => Ok(Signs::MULTIPLY),
      "/" => Ok(Signs::DIVIDE),
      "**" => Ok(Signs::EMPOWERMENT),
      "^" => Ok(Signs::CARER),
      "%" => Ok(Signs::MODULE),

      // Others
      "!" => Ok(Signs::NOT),
      "->" => Ok(Signs::ARROW),
      "@" => Ok(Signs::AT),
      "|" => Ok(Signs::BITOR),
      "&" => Ok(Signs::BITAND),

      // Default
      _ => Err(()),
    }
  }
}

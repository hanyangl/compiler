use super::tokens::Signs;

#[derive(PartialEq, PartialOrd)]
pub enum Precedence {
  LOWEST = 0,
  EQUALS = 1,
  LESSGREATER = 2,
  SUM = 3,
  PRODUCT = 4,
  PREFIX = 5,
  CALL = 6,
  INDEX = 7,
  METHOD = 8,
}

impl Precedence {
  pub fn from_sign(sign: Signs) -> Precedence {
    match sign {
      // EQUALS
      Signs::EQUAL |
      Signs::EQUALTYPE |
      Signs::NOTEQUAL |
      Signs::NOTEQUALTYPE |
      Signs::LESSOREQUALTHAN |
      Signs::GREATEROREQUALTHAN => Precedence::EQUALS,

      // LESSGREATER
      Signs::LESSTHAN |
      Signs::GREATERTHAN => Precedence::LESSGREATER,

      // SUM
      Signs::PLUS |
      Signs::MINUS => Precedence::SUM,

      // PRODUCT
      Signs::MULTIPLY |
      Signs::DIVIDE => Precedence::PRODUCT,

      // METHOD
      Signs::ARROW => Precedence::METHOD,

      // LOWEST
      _ => Precedence::LOWEST,
    }
  }
}

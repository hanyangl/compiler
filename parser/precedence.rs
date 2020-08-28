use super::tokens::Signs;

#[derive(Debug, PartialEq, PartialOrd)]
pub enum Precedence {
  LOWEST = 0,
  EQUALS = 1,
  LESSGREATER = 2,
  SUM = 3,
  PRODUCT = 4,
  EMPOWERMENT = 5,
  PREFIX = 6,
  CALL = 7,
  INDEX = 8,
  METHOD = 9,
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
      Signs::DIVIDE |
      Signs::MODULE => Precedence::PRODUCT,

      // EMPOWERMENT
      Signs::EMPOWERMENT => Precedence::EMPOWERMENT,

      // CALL
      Signs::LEFTPARENTHESES => Precedence::CALL,

      // METHOD
      Signs::ARROW => Precedence::METHOD,

      // LOWEST
      _ => Precedence::LOWEST,
    }
  }
}

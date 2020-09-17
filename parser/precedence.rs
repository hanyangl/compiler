use super::tokens::{
  Keywords,
  Signs,
};

#[derive(Debug, PartialEq, PartialOrd)]
pub enum Precedence {
  LOWEST = 0,
  OR = 1,
  AND = 2,
  EQUALS = 3,
  LESSGREATER = 4,
  SUM = 5,
  PRODUCT = 6,
  EMPOWERMENT = 7,
  PREFIX = 8,
  CALL = 9,
  METHOD = 10,
  INDEX = 11,
  ALIAS = 12,
}

impl Precedence {
  pub fn from_keyword(keyword: Keywords) -> Precedence {
    match keyword {
      // ALIAS
      Keywords::AS => Precedence::ALIAS,

      // Default
      _ => Precedence::LOWEST,
    }
  }

  pub fn from_sign(sign: Signs) -> Precedence {
    match sign {
      // OR
      Signs::OR => Precedence::OR,

      // And
      Signs::AND => Precedence::AND,

      // EQUALS
      Signs::EQUAL |
      Signs::NOTEQUAL |
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
      Signs::EMPOWERMENT |
      Signs::CARER => Precedence::EMPOWERMENT,

      // CALL
      Signs::LEFTPARENTHESES => Precedence::CALL,

      // METHOD
      Signs::ARROW => Precedence::METHOD,

      // Index
      Signs::LEFTBRACKET => Precedence::INDEX,

      // LOWEST
      _ => Precedence::LOWEST,
    }
  }
}

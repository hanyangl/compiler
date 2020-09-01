use super::tokens::{
  Keywords,
  Signs,
};

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
  OR = 10,
  AND = 11,
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
      Signs::EMPOWERMENT |
      Signs::CARER => Precedence::EMPOWERMENT,

      // CALL
      Signs::LEFTPARENTHESES => Precedence::CALL,

      // Index
      Signs::LEFTBRACKET => Precedence::INDEX,

      // METHOD
      Signs::ARROW => Precedence::METHOD,

      // OR
      Signs::OR => Precedence::OR,

      // And
      Signs::AND => Precedence::AND,

      // LOWEST
      _ => Precedence::LOWEST,
    }
  }
}

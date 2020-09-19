use super::tokens::{
  Keywords,
  Signs,
};

#[derive(Debug, PartialEq, PartialOrd)]
pub enum Precedence {
  LOWEST = 0,
  ASSIGN = 1,
  IS = 2,
  OR = 3,
  AND = 4,
  EQUALS = 5,
  LESSGREATER = 6,
  SUM = 7,
  PRODUCT = 8,
  EMPOWERMENT = 9,
  PREFIX = 10,
  SUFFIX = 11,
  INOF = 12,
  CALL = 13,
  METHOD = 14,
  INDEX = 15,
  ALIAS = 16,
}

impl Precedence {
  pub fn from_keyword(keyword: Keywords) -> Precedence {
    match keyword {
      // IS
      Keywords::IS => Precedence::IS,

      // INOF
      Keywords::IN |
      Keywords::OF => Precedence::INOF,

      // ALIAS
      Keywords::AS => Precedence::ALIAS,

      // Default
      _ => Precedence::LOWEST,
    }
  }

  pub fn from_sign(sign: Signs) -> Precedence {
    match sign {
      // ASSIGN
      Signs::ASSIGN |
      Signs::PLUSASSIGN |
      Signs::MINUSASSIGN |
      Signs::MULTIPLYASSIGN |
      Signs::DIVIDEASSIGN => Precedence::ASSIGN,

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

      // SUFFIX
      Signs::PLUSPLUS |
      Signs::MINUSMINUS => Precedence::SUFFIX,

      // CALL
      Signs::LEFTPARENTHESES => Precedence::CALL,

      // METHOD
      Signs::ARROW => Precedence::METHOD,

      // INDEX
      Signs::LEFTBRACKET => Precedence::INDEX,

      // LOWEST
      _ => Precedence::LOWEST,
    }
  }
}

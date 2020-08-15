use crate::data::Signs;

#[derive(PartialEq, PartialOrd)]
pub enum Precedence {
  LOWEST = 0,
  EQUALS = 1,
  LESSGREATER = 2,
  SUM = 3,
  PRODUCT = 4,
  PREFIX = 5,
  CALL = 6,
}

pub fn get_precedence_to_sign(sign: Signs) -> Precedence {
  match sign {
    // EQUALS
    Signs::EQUAL | Signs::EQUALTYPE | Signs::NOTEQUAL | Signs::NOTEQUALTYPE => Precedence::EQUALS,

    // LESSGREATER
    Signs::LESSTHAN | Signs::LESSOREQUALTHAN | Signs::GREATERTHAN | Signs::GREATEROREQUALTHAN => Precedence::LESSGREATER,

    // SUM
    Signs::PLUS | Signs::MINUS => Precedence::SUM,

    // PRODUCT
    Signs::DIVIDE | Signs::MULTIPLY => Precedence::PRODUCT,

    // CALL
    Signs::LEFTPARENTHESES => Precedence::CALL,

    // LOWEST
    _ => Precedence::LOWEST,
  }
}

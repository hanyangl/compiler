use crate::{
  Environment,
  typechecker::{
    check_expression,
    TTypes,
  },
};

use sflyn_parser::{
  Error,
  Suffix,
  tokens::Types,
};

pub fn check(
  suffix: &Suffix,
  environment: &mut Environment,
) -> Result<TTypes, Error> {
  match check_expression(&suffix.get_left(), environment) {
    Ok(ttype) => if ttype.get_type() != Types::NUMBER {
      Err(Error::from_token(
        String::from("is not a valid number."),
        suffix.get_left().token(),
      ))
    } else {
      Ok(ttype)
    },
    Err(error) => Err(error),
  }
}

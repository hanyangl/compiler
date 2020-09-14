use crate::{
  Environment,
  typechecker::{
    check_expression,
    TTypes,
  },
};

use sflyn_parser::{
  Error,
  Expression,
  Prefix,
  tokens::{
    Signs,
    Types,
  },
};

pub fn check(
  prefix: &Prefix,
  environment: &mut Environment,
) -> Result<TTypes, Error> {
  let right_type;

  match check_expression(&prefix.get_right(), environment) {
    Ok(token) => {
      right_type = token;
    },
    Err(error) => {
      return Err(error);
    },
  }

  if prefix.get_token().token.expect_sign(&Signs::MINUS) {
    if right_type.get_type() != Types::NUMBER {
      return Err(Error::from_token(
        String::from("only can convert numbers to negative."),
        prefix.get_token(),
      ));
    }

    return Ok(right_type);
  } else if prefix.get_token().token.expect_sign(&Signs::NOT) {
    if right_type.get_type() != Types::BOOLEAN && right_type.get_type() != Types::NULL {
      return Err(Error::from_token(
        String::from("can no be parsed to a boolean."),
        prefix.get_token(),
      ));
    }

    return Ok(TTypes::new_type(Types::BOOLEAN, String::from("boolean"), prefix.get_token()));
  }

  Err(Error::from_token(
    String::from("invalid prefix expression."),
    prefix.get_token(),
  ))
}

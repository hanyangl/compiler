use crate::{
  Environment,
  typechecker::{
    check_expression,
    equal_types,
    TTypes,
  },
};

use sflyn_parser::{
  Error,
  Variable,
  Statement,
  tokens::Keywords,
};

pub fn check(
  variable: &Variable,
  environment: &mut Environment,
) -> Result<TTypes, Error> {
  // Check if the variable name is already in use.
  if environment.store.get_type(&variable.get_name().value).is_some() {
    return Err(Error::from_token(
      format!("`{}` is already in use.", variable.get_name().value),
      variable.get_name(),
    ));
  }

  let data_type = variable.get_type();

  if let Some(value) = variable.get_value() {
    match check_expression(&value, environment) {
      Ok(token) => {
        if variable.get_token().token.expect_keyword(&Keywords::CONST) {
          environment.store.set_const(variable.get_name().value);
        }

        if data_type.value == "any" {
          environment.store.set_type(variable.get_name().value, token.clone());
          return Ok(token);
        } else if let Some(ttype) = data_type.token.get_type() {
          if equal_types(ttype, token.get_type()) || token.get_value() == "any" {
            environment.store.set_type(variable.get_name().value, token.clone());
            return Ok(token);
          }

          return Err(Error::from_token(
            format!("`{}` not satisfied the `{}` data type.", token.get_value(), data_type.value),
            value.token(),
          ));
        }
      },
      Err(error) => {
        return Err(error);
      },
    }
  }

  Err(Error::from_token(
    String::from("invalid variable statement."),
    variable.get_token(),
  ))
}

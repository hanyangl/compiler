use crate::{
  Environment,
  typechecker::{
    check_expression,
    TTypes,
  },
};

use sflyn_parser::{
  Error,
  Variable,
  Statement,
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
        if data_type.value == "any" {
          environment.store.set_type(variable.get_name().value, token.clone());
          return Ok(token);
        } else if let Some(data_type) = data_type.token.get_type() {
          if data_type == token.get_type() {
            environment.store.set_type(variable.get_name().value, token.clone());
            return Ok(token);
          }
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

use crate::{
  Environment,
  typechecker::{
    check_expression,
    check_statement,
    equal_types,
    TTypes,
  },
};

use sflyn_parser::{
  Error,
  IfElse,
  Statement,
  tokens::Types,
};

pub fn check(
  if_else: &IfElse,
  environment: &mut Environment,
) -> Result<TTypes, Error> {
  let mut data_type: Option<TTypes> = None;

  for condition in if_else.get_conditions().iter() {
    match check_expression(&condition.get_condition(), environment) {
      Ok(token) => {
        if token.get_type() != Types::BOOLEAN {
          return Err(Error::from_token(
            String::from("the condition is not a `boolean`."),
            condition.get_token(),
          ));
        }
      },
      Err(error) => {
        return Err(error);
      },
    }

    match check_statement(&condition.get_consequence(), environment) {
      Ok(token) => {
        if data_type.clone().is_some() {
          if !equal_types(data_type.clone().unwrap().get_type(), token.get_type()) {
            return Err(Error::from_token(
              format!("`{}` not satisfied the `{}` data type.", token.get_value(), data_type.unwrap().get_value()),
              token.get_token(),
            ));
          }

          continue;
        }

        data_type = Some(token);
      },
      Err(error) => {
        return Err(error);
      },
    }
  }

  if data_type.is_none() {
    return Err(Error::from_token(
      String::from("invalid data type."),
      if_else.get_token(),
    ));
  }

  if let Some(alternative) = if_else.get_alternative() {
    match check_statement(&alternative, environment) {
      Ok(token) => {
        if !equal_types(data_type.clone().unwrap().get_type(), token.get_type()) {
          return Err(Error::from_token(
            format!("`{}` not satisfied the `{}` data type.", token.get_value(), data_type.unwrap().get_value()),
            token.get_token(),
          ));
        }
      },
      Err(error) => {
        return Err(error);
      },
    }
  }

  Ok(data_type.unwrap())
}

use crate::{
  Environment,
  Store,
  typechecker::{
    check_statement,
    equal_types,
    function_arguments_to_string,
    TTypes,
  },
};

use sflyn_parser::{
  AnonymousFunction,
  Error,
  Expression,
  tokens::Token,
};

pub fn check(
  anonymous_function: &AnonymousFunction,
  environment: &mut Environment,
) -> Result<TTypes, Error> {
  // Create a new closed environment.
  let mut function_environment: Environment = environment.clone();

  function_environment.store = Store::from_store(&environment.store);

  let arguments: Vec<String>;

  match function_arguments_to_string(anonymous_function.get_arguments(), environment, &mut function_environment) {
    Ok(args) => {
      arguments = args;
    },
    Err(error) => {
      return Err(error);
    },
  }

  // Get the function data type.
  let data_type: Token = anonymous_function.get_type();

  // Get the ttypes from the function body.
  match check_statement(&anonymous_function.get_body(), &mut function_environment) {
    Ok(token) => {
      if let Some(ttoken) = data_type.token.get_type() {
        if !equal_types(ttoken, token.get_type()) && token.get_value() != "any" {
          return Err(Error::from_token(
            format!("`{}` not satisfied the `{}` data type.", token.get_token().value, data_type.value),
            token.get_token(),
          ));
        }
      } else {
        return Err(Error::from_token(
          format!("`{}` is not a valid data type.", data_type.value),
          data_type,
        ));
      }
    },
    Err(error) => {
      return Err(error);
    },
  }

  let value = format!("({}) => {}", arguments.join(", "), data_type.value);
  let token = Token::from_value(value.as_str(), 0, 0);

  if token.token.get_type().is_none() {
    return Err(Error::from_token(
      String::from("is not a valid function."),
      anonymous_function.get_token(),
    ));
  }

  return Ok(TTypes::new_function(
    token.token.get_type().unwrap(),
    token.value,
    anonymous_function.get_token(),
    anonymous_function.get_arguments(),
  ));
}

use crate::{
  Environment,
  typechecker::{
    check_expression,
    TTypes,
  },
};

use sflyn_parser::{
  Argument,
  Call,
  Error,
  Expression,
  tokens::Token,
};

pub fn check(
  call: &Call,
  environment: &mut Environment,
) -> Result<TTypes, Error> {
  let function_type = environment.store.get_type(&call.get_token().value);

  // Check if the call token exists in the environment store.
  if function_type.is_none() {
    return Err(Error::from_token(
      format!("`{}` identifier not found.", call.get_token().value),
      call.get_token(),
    ));
  }

  let function_type: TTypes = function_type.unwrap();

  if !function_type.is_function() {
    return Err(Error::from_token(
      format!("`{}` is not a function.", call.get_token().value),
      call.get_token(),
    ));
  }

  let mut min_arguments: usize = 0;
  let mut max_arguments: usize = 0;

  for argument in function_type.get_arguments().iter() {
    // Get the argument expression.
    let argument: Argument = argument.get_argument().unwrap();

    max_arguments += 1;

    // Check if the argument has a default value.
    if argument.get_value().is_none() {
      min_arguments += 1;
    }
  }

  if call.get_arguments().len() < min_arguments {
    return Err(Error::from_token(
      format!(
        "expected minimum `{}` arguments, got `{}` instead.",
        min_arguments,
        call.get_arguments().len(),
      ),
      call.get_token(),
    ));
  }

  if call.get_arguments().len() > max_arguments {
    return Err(Error::from_token(
      format!(
        "expected maximum `{}` arguments, got `{}` instead.",
        max_arguments,
        call.get_arguments().len(),
      ),
      call.get_token(),
    ));
  }

  // Get call arguments types.
  let mut call_arguments_types: Vec<TTypes> = Vec::new();

  for argument in call.get_arguments().iter() {
    match check_expression(argument, environment) {
      Ok(token) => {
        call_arguments_types.push(token);
      },
      Err(error) => {
        return Err(error);
      },
    }
  }

  // Compare arguments types.
  let mut index: usize = 0;

  for argument in call_arguments_types.iter() {
    let call_token: Token = call.get_arguments()[index].token();
    let function_argument: Token = function_type.get_arguments()[index].get_argument().unwrap().get_type();

    if function_argument.token.is_identifier() {
      if let Some(interface_type) = environment.store.get_type(&function_argument.value) {
        if interface_type.is_interface() {
          println!("Interface");
          continue;
        }

        return Err(Error::from_token(
          format!("`{}` is not a valid interface.", function_argument.value),
          call_token,
        ));
      }

      return Err(Error::from_token(
        format!("`{}` identifier not found.", function_argument.value),
        call_token,
      ));
    } else if function_argument.token.get_type().is_some() {
      if function_argument.token.get_type().unwrap() == argument.get_type() {
        continue;
      }

      return Err(Error::from_token(
        format!("`{}` not satisfied the `{}` data type.", argument.get_value(), function_argument.value),
        call_token,
      ));
    }

    index += 1;
  }

  if let Some(function) = function_type.get_type().get_function() {
    return Ok(TTypes::new_type(
      function.data_type.token.get_type().unwrap(),
      function.data_type.value,
      call.get_token(),
    ));
  }

  Err(Error::from_token(
    String::from("invalid call expression."),
    call.get_token(),
  ))
}

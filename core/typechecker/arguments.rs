use crate::Environment;

use sflyn_parser::{
  Error,
  Expression,
  Expressions,
};

pub fn function_arguments_to_string(
  function_arguments: Vec<Box<Expressions>>,
  environment: &mut Environment,
  function_environment: &mut Environment,
) -> Result<Vec<String>, Error> {
  let mut arguments_names: Vec<String> = Vec::new();
  let mut arguments: Vec<String> = Vec::new();

  for argument in function_arguments.iter() {
    let argument = argument.clone().get_argument().unwrap();
    let argument_name = argument.token.clone();

    // Check if the argument name is already in use.
    if arguments_names.contains(&argument_name.value.clone()) {
      return Err(Error::from_token(
        format!("`{}` is already in use.", argument_name.value.clone()),
        argument_name,
      ));
    }

    // Add the argument name to the arguments names list.
    arguments_names.push(argument_name.value.clone());

    let mut argument_type = argument.data_type.clone();

    // Check if the argument data type is an identifier.
    if argument_type.token.clone().is_identifier() {
      // Get the interface type from the environment store.
      match environment.store.get_interface(argument_type.value.clone()) {
        Some(data_type) => {
          argument_type = data_type;
        }
        None => {
          return Err(Error::from_token(
            format!(
              "`{}` is not a valid interface.",
              argument_type.value.clone()
            ),
            argument_type.clone(),
          ));
        }
      }
    }
    // Check if the argument data type is a type.
    else if argument_type.token.clone().get_type().is_none() {
      return Err(Error::from_token(
        format!(
          "`{}` is not a valid data type.",
          argument_type.value.clone()
        ),
        argument_type.clone(),
      ));
    }

    // Add the argument to the closed environment.
    function_environment.store.set_type(argument_name.value, argument_type);

    // Add the argument to the argumens list.
    arguments.push(argument.clone().string());
}

  Ok(arguments)
}


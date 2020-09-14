use crate::{
  Environment,
  typechecker::TTypes,
};

use sflyn_parser::{
  Argument,
  Error,
  Expression,
  Expressions,
  tokens::{
    Token,
    Tokens,
    Types,
  },
};

pub fn function_arguments_to_string(
  function_arguments: Vec<Box<Expressions>>,
  environment: &mut Environment,
  function_environment: &mut Environment,
) -> Result<Vec<String>, Error> {
  let mut arguments_names: Vec<String> = Vec::new();
  let mut arguments: Vec<String> = Vec::new();

  for argument in function_arguments.iter() {
    let argument: Argument = argument.get_argument().unwrap();
    let argument_name: Token = argument.get_token();

    // Check if the argument name is already in use.
    if arguments_names.contains(&argument_name.value.clone()) {
      return Err(Error::from_token(
        format!("`{}` is already in use.", argument_name.value.clone()),
        argument_name,
      ));
    }

    // Add the argument name to the arguments names list.
    arguments_names.push(argument_name.value.clone());

    let ttype: TTypes;

    // Check if the argument data type is an identifier.
    if argument.get_type().token.is_identifier() {
      // Get the interface type from the environment store.
      match environment.store.get_type(&argument.get_type().value) {
        Some(data_type) => {
          ttype = data_type;
        }
        None => {
          return Err(Error::from_token(
            format!(
              "`{}` is not a valid interface.",
              argument.get_type().value,
            ),
            argument.get_type(),
          ));
        }
      }
    }
    // Check if the argument data type is a type.
    else if argument.get_type().token.get_type().is_none() {
      return Err(Error::from_token(
        format!(
          "`{}` is not a valid data type.",
          argument.get_type().value,
        ),
        argument.get_type(),
      ));
    } else {
      let data_type: Types = argument.get_type().token.get_type().unwrap();

      if let Some(function) = data_type.get_function() {
        let mut arguments: Vec<Box<Expressions>> = Vec::new();

        for (key, value) in function.arguments.iter() {
          arguments.push(Argument::new_box_full(
            Token::new(Box::new(Tokens::IDENTIFIER), key.clone(), 0, 0),
            value.clone(),
            None,
          ));
        }

        ttype = TTypes::new_function(
          data_type,
          argument.get_type().value,
          argument.get_token(),
          arguments,
        );
      } else {
        ttype = TTypes::new_type(
          data_type,
          argument.get_type().value,
          argument.get_token()
        );
      }
    }

    // Add the argument to the closed environment.
    function_environment.store.set_type(argument_name.value, ttype);

    // Add the argument to the argumens list.
    arguments.push(argument.string());
  }

  Ok(arguments)
}

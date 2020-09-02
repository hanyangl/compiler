use crate::{
  Environment,
  Store,
};

use sflyn_parser::{
  Error,
  Expressions,
  tokens::Token,
};

use super::{
  equal_types,
  equal_type_and_interface,
};

pub fn check_expression(
  expression: &mut Box<Expressions>,
  environment: &mut Environment,
) -> Result<Token, Error> {
  // Anonymous function

  // Argument

  // Array

  // Array index

  // Boolean
  if expression.clone().get_boolean().is_some() {
    return Ok(Token::from_value("boolean", 0, 0));
  }

  // Call
  if let Some(call) = expression.clone().get_call().as_mut() {
    // Check if the call token exists in the environment store.
    if environment.store.get_type(call.token.value.clone()).is_none() {
      return Err(Error::from_token(
        format!("`{}` identifier not found.", call.token.value.clone()),
        call.token.clone(),
      ));
    }

    // Get the function arguments from the environment store.
    if let Some(arguments) = environment.store.get_function_arguments(call.token.value.clone()) {
      // Get the min and max arguments.
      let mut min_arguments: usize = 0;
      let mut max_arguments: usize = 0;

      for argument in arguments.clone().iter() {
        // Get the argument expression.
        let argument = argument.clone().get_argument().unwrap();
        
        max_arguments += 1;

        // Check if the argument has a default value.
        if argument.value.is_none() {
          min_arguments += 1;
        }
      }

      // Check if the call has the minimun arguments.
      if call.arguments.len() < min_arguments {
        return Err(Error::from_token(
          format!("expected minimum {} arguments, got {} instead.", min_arguments, call.arguments.len()),
          call.token.clone(),
        ));
      }

      if call.arguments.len() > max_arguments {
        return Err(Error::from_token(
          format!("expected maximum {} arguments, got {} instead.", max_arguments, call.arguments.len()),
          call.token.clone(),
        ))
      }

      // Parse call arguments types.
      let mut call_arguments_types: Vec<Token> = Vec::new();

      for argument in call.arguments.clone().iter_mut() {
        match check_expression(argument, environment) {
          Ok(data_type) => {
            call_arguments_types.push(data_type);
          },
          Err(error) => {
            return Err(error);
          },
        }
      }

      // Compare arguments types.
      let mut index: usize = 0;

      for argument in call_arguments_types.clone().iter() {
        let call_token = call.arguments[index].clone().token();
        let function_argument = arguments[index].clone().get_argument().unwrap().data_type;

        if function_argument.token.clone().is_identifier() {
          if let Some(interface) = environment.store.get_interface(function_argument.value.clone()) {
            if !equal_type_and_interface(argument.clone(), interface.clone()) {
              return Err(Error::from_token(
                format!("`{}` not satisfied the `{}` interface.", call_token.value.clone(), function_argument.value.clone()),
                call_token.clone(),
              ));
            }
          }
        } else if function_argument.token.clone().get_type().is_some() {
          if !equal_types(argument.clone(), function_argument.clone()) {
            return Err(Error::from_token(
              format!("`{}` not satisfied the `{}` data type.", call_token.value.clone(), function_argument.value.clone()),
              call_token.clone(),
            ));
          }
        }

        index += 1;
      }

      if let Some(data_type) = environment.store.get_type(call.token.value.clone()) {
        if let Some(data_type) = data_type.token.clone().get_type() {
          if let Some(function_type) = data_type.clone().get_function() {
            return Ok(function_type.data_type.clone());
          }
        }
      }
    } else {
      return Err(Error::from_token(
        format!("`{}` is not a function.", call.token.value.clone()),
        call.token.clone(),
      ));
    }
  }

  // HashMap
  if let Some(hashmap) = expression.clone().get_hashmap().as_mut() {
    let mut items: Vec<String> = Vec::new();

    for (key, value) in hashmap.items.clone().iter_mut() {
      let mut new_item = key.clone();

      // Parse item value data type.
      match check_expression(value, environment) {
        Ok(token) => {
          // Check if the token is a valid type.
          if token.token.clone().get_type().is_some() {
            new_item.push_str(": ");
            new_item.push_str(token.value.as_str());
          } else {
            return Err(Error::from_token(
              format!("`{}` is not a valid data type.", value.clone().token().value),
              value.clone().token(),
            ));
          }
        },
        Err(error) => {
          return Err(error);
        },
      }

      items.push(new_item);
    }

    let mut value = String::from("{ ");

    value.push_str(items.join(", ").as_str());
    value.push_str(" }");

    return Ok(Token::from_value(value.as_str(), 0, 0));
  }

  // Identifier
  if let Some(identifier) = expression.clone().get_identifier() {
    return match environment.store.get_type(identifier.token.value.clone()) {
      Some(data_type) => Ok(data_type),
      None => Err(Error::from_token(
        format!("`{}` identifier not found.", identifier.token.value.clone()),
        identifier.token.clone(),
      )),
    };
  }

  // Infix
  if let Some(infix) = expression.clone().get_infix().as_mut() {
    // Get the left expression data type.
    let left_type;

    match check_expression(&mut infix.left, environment) {
      Ok(data_type) => {
        left_type = data_type;
      },
      Err(error) => {
        return Err(error);
      },
    }

    // Create a new environment.
    let mut right_environment = environment.clone();

    right_environment.store = Store::from_store(environment.store.clone());

    // Check if is a method.
    if infix.clone().is_method() {
      // Get the left token type.
      if let Some(left_type) = left_type.token.clone().get_type() {
        // Get the left hashmap.
        if let Some(left_hashmap) = left_type.clone().get_hashmap() {
          for (key, value) in left_hashmap.items {
            // Add he hashmap items to the environment.
            right_environment.store.set_type(key, value);
          }
        }
      }
    }

    // Get the right expression data type.
    let right_type;

    match check_expression(&mut infix.right, &mut right_environment) {
      Ok(data_type) => {
        right_type = data_type;
      },
      Err(error) => {
        return Err(error);
      },
    }

    // Check if is a method.
    if infix.clone().is_method() {
      return Ok(right_type);
    }
  }

  // Number
  if expression.clone().get_number().is_some() {
    return Ok(Token::from_value("number", 0, 0));
  }

  // Prefix

  // String
  if expression.clone().get_string().is_some() {
    return Ok(Token::from_value("string", 0, 0));
  }

  // Default
  Err(Error::from_token(
    String::from("unknown expression."),
    expression.clone().token(),
  ))
}

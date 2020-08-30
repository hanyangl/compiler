use crate::Environment;
use crate::objects::{Objects, Error};
use crate::statements;

use sflyn_parser::expressions::Call;

use super::evaluate_expressions;

pub fn evaluate(
  file_name: String,
  call: Call,
  environment: &mut Environment,
) -> Box<Objects> {
  // Get the function object.
  let function_object = match environment.get(call.token.value.clone()) {
    Some(object) => object,
    None => Error::new(format!("Identifier not found: {}", call.token.value.clone())),
  };

  // Check if the function object is an error.
  if function_object.clone().is_error() {
    return function_object;
  }

  // Compile arguments.
  let arguments = evaluate_expressions(file_name.clone(), call.arguments.clone(), environment);

  // Check if the first argument is an error.
  if arguments.len() == 1 && arguments[0].clone().is_error() {
    return arguments[0].clone();
  }

  // Check if the function object is an anonymous function.
  if function_object.clone().is_anonymous_function() {
    // Get the anonymous function object.
    let anonymous_function = function_object.clone().get_anonymous_function().unwrap();

    // Create a new environment.
    let mut function_environment = Environment::from_environment(anonymous_function.environment);

    let mut index: usize = 0;

    // Add call arguments to the function environment.
    for argument in arguments {
      let function_argument = anonymous_function.arguments[index].clone().get_argument().unwrap();

      function_environment.set(function_argument.token.value.clone(), argument);

      index += 1;
    }

    return match statements::evaluate(
      file_name.clone(),
      anonymous_function.body.clone(),
      &mut function_environment,
    ) {
      Some(object) => object,
      None => Error::new(format!("Unknown statement: {}", anonymous_function.body.string())),
    };
  }

  Error::new(format!("Unknown function: {}", call.token.value))
}

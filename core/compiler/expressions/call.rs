use crate::{
  compiler::{
    builtins::get_builtin_for_identifier,
    Error,
    evaluate_statement,
    Objects,
  },
  Environment,
  Store,
};

use sflyn_parser::{
  Argument,
  Call,
  Expression,
};

use super::evaluate_expressions;

pub fn evaluate(
  call: Call,
  environment: &mut Environment,
) -> Box<Objects> {
  // Get the function object.
  let function_object = match environment.store.get_object(&call.get_token().value) {
    Some(object) => object.clone(),
    None => get_builtin_for_identifier(call.get_token()),
  };

  // Check if the function object is an error.
  if function_object.get_error().is_some() {
    return function_object;
  }

  // Compile arguments.
  let arguments = evaluate_expressions(call.get_arguments(), environment);

  // Check if the first argument is an error.
  if arguments.len() == 1 && arguments[0].get_error().is_some() {
    return arguments[0].clone();
  }

  // Check if the function object is an anonymous function.
  if let Some(anonymous_function) = function_object.get_anonymous_function() {
    let mut index: usize = 0;

    let mut function_environment = environment.clone();

    function_environment.store = Store::from_store(anonymous_function.store);

    // Add call arguments to the function environment.
    for argument in arguments {
      let function_argument: Argument = anonymous_function.arguments[index].get_argument().unwrap();

      function_environment.store.set_object(function_argument.get_token().value, argument);

      index += 1;
    }

    return match evaluate_statement(&anonymous_function.body, &mut function_environment) {
      Some(object) => object,
      None => Error::new(
        format!("unknown statement"),
        anonymous_function.body.token(),
      ),
    };
  }
  // Check if the function object is a builtin.
  else if let Some(builtin) = function_object.get_builtin() {
    if let Some(fun) = builtin.get_function() {
      return (fun)(call.get_token(), arguments);
    }
  }

  Error::new(
    format!("Unknown function: {}", call.get_token().value),
    call.get_token(),
  )
}

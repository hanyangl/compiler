use crate::{
  compiler::{
    AnonymousFunction,
    evaluate_expression,
    Objects,
    ReturnO,
  },
  Environment,
};

use sflyn_parser::Statements;

pub fn evaluate_statement(
  statement: Box<Statements>,
  environment: &mut Environment,
) -> Option<Box<Objects>> {
  // Block
  if let Some(block) = statement.clone().get_block() {
    let mut result_object: Option<Box<Objects>> = None;

    for statement in block.statements {
      result_object = evaluate_statement(statement.clone(), environment);

      if let Some(object) = result_object.clone() {
        // Check if the result object is an error or a return.
        if object.clone().get_error().is_some() ||
          object.clone().get_return().is_some() {
          break;
        }
      }
    }

    return result_object;
  }

  // Export

  // Expression
  if let Some(expression) = statement.clone().get_expression() {
    return Some(evaluate_expression(expression.expression, environment));
  }

  // Function
  if let Some(function) = statement.clone().get_function() {
    AnonymousFunction::add_arguments_to_environment(
      function.arguments.clone(),
      environment,
    );

    let object = AnonymousFunction::new(
      true,
      function.arguments.clone(),
      function.data_type,
      function.body,
      environment.store.clone(),
    );

    // Add function object to the environment.
    environment.store.set_object(function.name.value, object);
  }

  // If else

  // Import

  // Interface

  // Return
  if let Some(return_s) = statement.clone().get_return() {
    // Get the return value.
    if let Some(value) = return_s.value {
      // Evaluate the return value.
      let object = evaluate_expression(value, environment);

      // Check if the value object is an error.
      if object.clone().get_error().is_some() {
        return Some(object);
      }

      return Some(ReturnO::new(object));
    }
  }

  // Variable
  if let Some(variable) = statement.clone().get_variable() {
    // Get the variable value.
    if let Some(value) = variable.value {
      // Evaluate the variable value.
      let object = evaluate_expression(value, environment);

      // Check if the value object is an error.
      if object.clone().get_error().is_some() {
        return Some(object);
      }

      environment.store.set_object(variable.name.value, object);
    }
  }

  // Default
  None
}

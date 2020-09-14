mod import;

use crate::{
  compiler::{
    AnonymousFunction,
    evaluate_expression,
    Objects,
    ReturnO,
    Boolean,
    Null,
  },
  Environment,
};

use sflyn_parser::Statements;

pub fn evaluate_statement(
  statement: &Box<Statements>,
  environment: &mut Environment,
) -> Option<Box<Objects>> {
  // Block
  if let Some(block) = statement.get_block() {
    let mut result_object: Option<Box<Objects>> = None;

    for statement in block.statements.iter() {
      result_object = evaluate_statement(statement, environment);

      if let Some(object) = result_object.clone() {
        // Check if the result object is an error or a return.
        if object.get_error().is_some() || object.get_return().is_some() {
          break;
        }
      }
    }

    return result_object;
  }

  // Export
  if let Some(export) = statement.get_export() {
    return evaluate_statement(&export.value, environment);
  }

  // Expression
  if let Some(expression) = statement.get_expression() {
    return Some(evaluate_expression(&expression.expression, environment));
  }

  // Function
  if let Some(function) = statement.get_function() {
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
  if let Some(if_else) = statement.get_if_else() {
    for condition in if_else.conditions.iter() {
      let object = evaluate_expression(&condition.condition, environment);

      if Boolean::is_truthy(object) {
        return evaluate_statement(&condition.consequence, environment);
      }
    }

    if let Some(alternative) = if_else.alternative {
      return evaluate_statement(&alternative, environment);
    }
  }

  // Import
  if let Some(import_s) = statement.get_import() {
    return import::evaluate(import_s, environment);
  }

  // Interface

  // Return
  if let Some(return_s) = statement.get_return() {
    // Get the return value.
    if let Some(value) = return_s.value {
      // Evaluate the return value.
      let object = evaluate_expression(&value, environment);

      // Check if the value object is an error.
      if object.get_error().is_some() {
        return Some(object);
      }

      return Some(ReturnO::new(object));
    }

    return Some(ReturnO::new(Null::new()));
  }

  // Variable
  if let Some(variable) = statement.get_variable() {
    // Get the variable value.
    if let Some(value) = variable.value {
      // Evaluate the variable value.
      let object = evaluate_expression(&value, environment);

      // Check if the value object is an error.
      if object.get_error().is_some() {
        return Some(object);
      }

      environment.store.set_object(variable.name.value, object);
    }
  }

  // Default
  None
}

mod library;

use crate::Environment;
use crate::expressions::evaluate as evaluate_expression;
use crate::objects::*;

use sflyn_parser::statements::Statements;

pub fn evaluate(statement: Box<Statements>, environment: &mut Environment) -> Option<Box<Objects>> {
  // Block
  if statement.clone().is_block() {
    let mut result_object: Option<Box<Objects>> = None;

    for statement in statement.clone().get_block().unwrap().statements {
      result_object = evaluate(statement.clone(), environment);

      match result_object.clone() {
        Some(object) => {
          // Check if the result object is an error, return or print object.
          if object.clone().is_error() ||
            object.clone().is_return() ||
            object.clone().is_print() {
            return result_object;
          }
        },
        None => {},
      }
    }

    return result_object;
  }
  
  // Expression
  if statement.clone().is_expression() {
    return Some(evaluate_expression(
      statement.clone().get_expression().unwrap().expression,
      environment,
    ));
  }

  // Function

  // Library
  if statement.clone().is_library() {
    return Some(library::evaluate(statement.clone().get_library().unwrap(), environment));
  }

  // Return

  // Variable
  if statement.clone().is_variable() {
    let variable = statement.clone().get_variable().unwrap();

    // Evaluate variable value.
    let object = evaluate_expression(variable.value, environment);

    // Check if the object is an error.
    if object.clone().is_error() {
      return Some(object);
    }

    environment.set(variable.name.string(), object);
  }

  // Variable set

  None
}

use crate::Environment;
use crate::expressions::evaluate as evaluate_expression;
use crate::objects::*;

use sflyn_parser::statements::Statements;

pub fn evaluate(
  statement: Box<Statements>,
  environment: &mut Environment,
) -> Option<Box<Objects>> {
  // Variables
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

  None
}

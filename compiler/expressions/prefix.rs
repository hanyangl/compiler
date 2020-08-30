use crate::Environment;
use crate::objects::{Objects, Boolean, Number, Error};

use sflyn_parser::expressions::{Prefix, Expression};

use super::evaluate as evaluate_expression;

pub fn evaluate(
  file_name: String,
  prefix: Prefix,
  environment: &mut Environment,
) -> Box<Objects> {
  // Evaluate right expression.
  let right_object = evaluate_expression(file_name, prefix.right.clone(), environment);

  // Check if the object is an error.
  if right_object.clone().is_error() {
    return right_object;
  }

  // Check if the operator is a negation sign.
  if prefix.operator.clone() == "!" {
    if right_object.clone().expect_boolean(false) || right_object.clone().is_null() {
      return Boolean::new(true);
    }

    return Boolean::new(false);
  }
  // Check if the operator is a minus sign.
  else if prefix.operator.clone() == "-" && right_object.clone().is_number() {
    return Number::new(-right_object.get_number().unwrap().value)
  }

  Error::new(format!("Unknown prefix: {}", prefix.string()))
}

use crate::Environment;
use crate::objects::{Objects, Error, Number, Boolean, StringO};

use sflyn_parser::expressions::{Infix, Expression};

use super::evaluate as evaluate_expression;

pub fn evaluate(infix: Infix, environment: &mut Environment) -> Box<Objects> {
  let error = Error::new(format!("Unknown infix: {}", infix.clone().string()));

  // Evaluate left expression.
  let left_object = evaluate_expression(infix.left.clone(), environment);

  // Check if the left object is an error.
  if left_object.clone().is_error() {
    return left_object;
  }

  // Evaluate right expression.
  let right_object = evaluate_expression(infix.right.clone(), environment);

  // Check if the right object is an error.
  if right_object.clone().is_error() {
    return right_object;
  }

  // Check if left and right objects are numbers.
  if left_object.clone().is_number() && right_object.clone().is_number() {
    let left_value = left_object.clone().get_number().unwrap().value;
    let right_value = right_object.clone().get_number().unwrap().value;

    return match infix.operator.clone().as_str() {
      "+" => Number::new(left_value + right_value),
      "-" => Number::new(left_value - right_value),
      "*" => Number::new(left_value * right_value),
      "/" => Number::new(left_value / right_value),
      "**" => Number::new(left_value.powf(right_value)),
      "%" => Number::new(left_value % right_value),
      "<" => Boolean::new(left_value < right_value),
      "<=" => Boolean::new(left_value <= right_value),
      ">" => Boolean::new(left_value > right_value),
      ">=" => Boolean::new(left_value >= right_value),
      "==" | "===" => Boolean::new(left_value == right_value),
      "!=" | "!==" => Boolean::new(left_value != right_value),
      _ => error.clone(),
    };
  }
  // Check if left or right object is a string.
  else if infix.operator.clone() == "+" && (
    left_object.clone().is_string() || right_object.clone().is_string()
  ) {
    return StringO::new(left_object.string() + &right_object.string());
  }
  // Check if the operator is an equal sign.
  else if infix.operator.clone() == "==" {
    return Boolean::new(left_object.get_hashkey() == right_object.get_hashkey());
  }
  // Check if the operator is an equal type sign.
  else if infix.operator.clone() == "===" {
    return Boolean::new(left_object == right_object);
  }
  // Check if the operator is a not equal sign.
  else if infix.operator.clone() == "!=" {
    return Boolean::new(left_object.get_hashkey() != right_object.get_hashkey());
  }
  // Check if the operatir is a not equal type sign.
  else if infix.operator.clone() == "!==" {
    return Boolean::new(left_object != right_object);
  }

  error
}

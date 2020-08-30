use crate::Environment;
use crate::objects::Objects;

use sflyn_parser::expressions::Method;

use super::evaluate as evaluate_expression;

pub fn evaluate(
  file_name: String,
  method: Method,
  environment: &mut Environment,
) -> Box<Objects> {
  // Compile left expression.
  let left_object = evaluate_expression(file_name.clone(), method.left, environment);

  // Check if the left object is an error.
  if left_object.clone().is_error() {
    return left_object;
  }

  // Create a new environment.
  let mut right_environment = Environment::from_environment(environment.clone());

  // Check if the left object is an hashmap.
  if left_object.clone().is_hashmap() {
    let hashmap = left_object.get_hashmap().unwrap();

    for item in hashmap.data {
      right_environment.set(item.key, item.value);
    }
  }

  // Compile right expression.
  let right_object = evaluate_expression(file_name, method.right, &mut right_environment);

  // Check if the right object is an error.
  if right_object.clone().is_error() {
    return right_object;
  }

  right_object
}

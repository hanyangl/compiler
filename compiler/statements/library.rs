use crate::Environment;
use crate::objects::{Objects, Print, Error};
use crate::expressions::evaluate as evaluate_expression;

use sflyn_parser::statements::Library;

pub fn evaluate(library: Library, environment: &mut Environment) -> Box<Objects> {
  // Compile option expression.
  let option_object = evaluate_expression(Some(library.option.clone()), environment);

  // Check if the option object is an error.
  if option_object.clone().is_error() {
    return option_object;
  }

  // Compile value expression.
  let value_object = evaluate_expression(Some(library.value.clone()), environment);

  // Check if the value object is an error.
  if value_object.clone().is_error() {
    return value_object;
  }

  // Check if the option value is 'print'.
  if option_object.clone().string() == "print" {
    return Print::new(value_object.string());
  }

  Error::new(String::from("Unknown"))
}

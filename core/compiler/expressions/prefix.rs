use crate::{
  compiler::{
    Boolean,
    Error,
    Number,
    Objects,
  },
  Environment,
};

use sflyn_parser::{
  Expression,
  Prefix,
  tokens::Signs,
};

use super::evaluate_expression;

pub fn evaluate(
  prefix: &Prefix,
  environment: &mut Environment,
) -> Box<Objects> {
  // Evaluate right expression.
  let right_object = evaluate_expression(&prefix.get_right(), environment);

  // Check if the object is an error.
  if right_object.get_error().is_some() {
    return right_object;
  }

  // Check if the operator is a negation sign.
  if prefix.get_token().token.expect_sign(&Signs::NOT) {
    if right_object.expect_boolean(false) || right_object.get_null().is_some() {
      return Boolean::new(true);
    }

    return Boolean::new(false);
  }
  // Check if the operator is a minus sign.
  else if prefix.get_token().token.expect_sign(&Signs::MINUS) {
    if let Some(number) = right_object.get_number() {
      return Number::new(-number.get_value());
    }

    return Error::new(
      String::from("only can use the `-` prefix in number type."),
      prefix.get_token(),
    );
  }

  Error::new(
    format!("Unknown prefix: {}", prefix.string()),
    prefix.get_token(),
  )
}

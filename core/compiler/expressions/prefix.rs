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
};

use super::evaluate_expression;

pub fn evaluate(
  prefix: Prefix,
  environment: &mut Environment,
) -> Box<Objects> {
  // Evaluate right expression.
  let right_object = evaluate_expression(prefix.right.clone(), environment);

  // Check if the object is an error.
  if right_object.clone().get_error().is_some() {
    return right_object;
  }

  // Check if the operator is a negation sign.
  if prefix.operator.clone() == "!" {
    if right_object.clone().expect_boolean(false) ||
      right_object.clone().get_null().is_some() {
      return Boolean::new(true);
    }

    return Boolean::new(false);
  }
  // Check if the operator is a minus sign.
  else if prefix.operator.clone() == "-" {
    if let Some(number) = right_object.clone().get_number() {
      return Number::new(-number.value);
    }

    return Error::new(
      String::from("only can use the `-` prefix in number type."),
      prefix.token.clone(),
    );
  }

  Error::new(
    format!("Unknown prefix: {}", prefix.clone().string()),
    prefix.token.clone(),
  )
}

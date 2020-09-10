use crate::{
  compiler::{
    Boolean,
    Error,
    Number,
    Objects,
    StringO,
  },
  Environment,
  Store,
};

use sflyn_parser::{
  Expression,
  Infix,
  tokens::Signs,
};

use super::evaluate_expression;

pub fn evaluate(
  infix: Infix,
  environment: &mut Environment,
) -> Box<Objects> {
  let error = Error::new(
    format!("Unknown infix: {}", infix.clone().string()),
    infix.token.clone(),
  );

  // Evaluate left expression.
  let left_object = evaluate_expression(infix.left.clone(), environment);

  // Check if the left object is an error.
  if left_object.clone().get_error().is_some() {
    return left_object;
  }

  // Create a new environment.
  let mut right_environment = environment.clone();

  // Set the new store.
  right_environment.store = Store::from_store(environment.store.clone());

  // Check if the infix is a method.
  if infix.clone().is_method() {
    // Check if the left object is a hashmap.
    if let Some(hashmap) = left_object.clone().get_hashmap() {
      // Set the data keys to the new environment.
      for item in hashmap.data {
        right_environment.store.set_object(item.key, item.value);
      }
    }
  }

  // Evaluate right expression.
  let right_object = evaluate_expression(infix.right.clone(), &mut right_environment);

  // Check if the right object is an error.
  if right_object.clone().get_error().is_some() {
    return right_object;
  }

  // Parse method.
  if infix.clone().is_method() {
    return right_object;
  }
  // Parse infix.
  else if infix.clone().is_infix() {
    // Check if left and right objects are numbers.
    if left_object.clone().get_number().is_some() &&
      right_object.clone().get_number().is_some() {
      let left_value = left_object.clone().get_number().unwrap().value;
      let right_value = right_object.clone().get_number().unwrap().value;

      return match infix.operator.clone().as_str() {
        "+" => Number::new(left_value + right_value),
        "-" => Number::new(left_value - right_value),
        "*" => Number::new(left_value * right_value),
        "/" => Number::new(left_value / right_value),
        "**" | "^" => Number::new(left_value.powf(right_value)),
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
    else if infix.token.token.clone().expect_sign(Signs::PLUS) && (
      left_object.clone().get_string().is_some() ||
      right_object.clone().get_string().is_some()
    ) {
      return StringO::new(left_object.string() + &right_object.string());
    }
    // Check if the operator is an equal sign.
    else if infix.token.token.clone().expect_sign(Signs::EQUAL) {
      return Boolean::new(left_object.get_hashkey() == right_object.get_hashkey());
    }
    // Check if the operator is an equal type sign.
    else if infix.token.token.clone().expect_sign(Signs::EQUALTYPE) {
      return Boolean::new(left_object == right_object);
    }
    // Check if the operator is a not equal sign.
    else if infix.token.token.clone().expect_sign(Signs::NOTEQUAL) {
      return Boolean::new(left_object.get_hashkey() != right_object.get_hashkey());
    }
    // Check if the operator is a not equal type sign.
    else if infix.token.token.clone().expect_sign(Signs::NOTEQUALTYPE) {
      return Boolean::new(left_object != right_object);
    }
    // Check if the operator is an or sign.
    else if infix.token.token.clone().expect_sign(Signs::OR) {
      // TODO: The rest of the expressions.
      // Null objects.
      let mut return_right = left_object.clone().get_null().is_some();

      // Empty strings
      if let Some(string) = left_object.clone().get_string() {
        return_right = string.value.len() == 0;
      }

      // false boolean
      if let Some(boolean) = left_object.clone().get_boolean() {
        return_right = boolean.value == false;
      }

      // Return the object.
      return if return_right { right_object } else { left_object };
    }
    // Check if the operator is an and sign.
    else if infix.token.token.clone().expect_sign(Signs::AND) &&
      left_object.clone().get_boolean().is_some() &&
      right_object.clone().get_boolean().is_some() {
      return Boolean::new(
        left_object.get_boolean().unwrap().value &&
        right_object.get_boolean().unwrap().value
      );
    }
  }

  error
}

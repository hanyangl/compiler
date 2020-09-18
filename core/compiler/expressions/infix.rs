use crate::{
  compiler::{
    Array,
    Boolean,
    Error,
    ForIn,
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
  tokens::{
    Keywords,
    Signs,
  },
};

use super::evaluate_expression;

pub fn evaluate(
  infix: &Infix,
  environment: &mut Environment,
) -> Box<Objects> {
  let error = Error::new(
    format!("Unknown infix: {}", infix.string()),
    infix.get_token(),
  );

  // Evaluate left expression.
  let mut left_object: Option<Box<Objects>> = None;

  if infix.get_token().token.expect_keyword(&Keywords::IN) {
    if let Some(identifier) = infix.get_left().get_identifier() {
      if environment.store.get_object(&identifier.get_value()).is_some() {
        return Error::new(
          format!("`{}` is already in use.", identifier.get_value()),
          infix.get_left().token(),
        );
      }
    } else {
      return Error::new(
        String::from("is not a valid expression for an `in`."),
        infix.get_left().token(),
      );
    }
  }
  else if infix.get_token().token.expect_keyword(&Keywords::OF) {
    return Error::new(
      String::from("is not a valid expression for an `of`."),
      infix.get_left().token(),
    );
  } else {
    left_object = Some(evaluate_expression(&infix.get_left(), environment));

    let obj = left_object.clone().unwrap();

    // Check if the left object is an error.
    if obj.get_error().is_some() {
      return obj;
    }

    // Check if the left object is a return.
    if let Some(return_o) = obj.get_return() {
      left_object = Some(return_o.get_value());
    }
  }

  // Create a new environment.
  let mut right_environment = environment.clone();

  // Set the new store.
  right_environment.store = Store::from_store(environment.store.clone());

  // Check if the infix is a method.
  if infix.is_method() && left_object.clone().is_some() {
    let left_object: Box<Objects> = left_object.clone().unwrap();
    let mut name = "";

    // Check if the left object is a hashmap.
    if let Some(hashmap) = left_object.get_hashmap() {
      // Set the data keys to the new environment.
      for item in hashmap.get_data() {
        right_environment.store.set_object(item.key, item.value);
      }
    } else if left_object.get_number().is_some() {
      name = "Number";
    } else if left_object.get_boolean().is_some() {
      name = "Boolean";
    } else if left_object.get_array().is_some() {
      name = "Array";
    } else if left_object.get_null().is_some() {
      name = "Null";
    } else if left_object.get_string().is_some() {
      name = "String";
    }

    if !name.is_empty() {
      // Get the object from the environment.
      if let Some(obj) = environment.store.get_object(&name.to_string()) {
        if let Some(hashmap) = obj.get_hashmap() {
          // Set the data keys to the new environment.
          for item in hashmap.get_data() {
            right_environment.store.set_object(item.key, item.value);
          }
        }
      }
    }
  }

  if left_object.clone().is_some() &&
    left_object.clone().unwrap().get_string().is_some() &&
    infix.get_right().token().value == "split" &&
    infix.get_right().get_call().is_some() {
    let call_right = infix.get_right().get_call().unwrap();
    let arguments = call_right.get_arguments();

    if arguments.len() == 1 {
      let arg = arguments[0].clone();

      if let Some(string) = arg.get_string() {
        let split_value = string.get_value();
        let elements: Vec<Box<Objects>> = left_object.unwrap()
          .get_string().unwrap().get_value()
          .split(&split_value[1..split_value.len() - 1])
          .map(|x| StringO::new(x.to_string()))
          .collect();

        return Array::new(elements[1..elements.len() - 1].to_vec());
      }
    }
  }

  // Evaluate right expression.
  let mut right_object: Box<Objects> = evaluate_expression(&infix.get_right(), &mut right_environment);

  // Check if the right object is an error.
  if right_object.get_error().is_some() {
    return right_object;
  }

  // Check if the right object is a return.
  if let Some(return_o) = right_object.get_return() {
    right_object = return_o.get_value();
  }

  // Parse method.
  if infix.is_method() && left_object.is_some() {
    let left_object = left_object.unwrap();
    let right_token = infix.get_right().token();

    // Check if the method is 'toString()'.
    if right_token.value == "toString" && (
      left_object.get_number().is_some() ||
      left_object.get_boolean().is_some() ||
      left_object.get_array().is_some()
    ) {
      return StringO::new(left_object.string());
    }
    // Check if the method is 'length' in a string.
    else if right_token.value == "length" && left_object.get_string().is_some() {
      return Number::new(
        left_object
          .get_string().unwrap()
          .get_value().len()
          .to_string().parse().unwrap()
      );
    }

    return right_object;
  }
  // Parse infix without 'in' or 'of'.
  else if infix.is_infix() && left_object.is_some() {
    let left_object = left_object.unwrap();

    // Check if left and right objects are numbers.
    if left_object.get_number().is_some() &&
      right_object.get_number().is_some() {
      let left_value = left_object.get_number().unwrap().get_value();
      let right_value = right_object.get_number().unwrap().get_value();

      return match infix.get_token().token.get_sign().unwrap() {
        Signs::PLUS => Number::new(left_value + right_value),
        Signs::MINUS => Number::new(left_value - right_value),
        Signs::MULTIPLY => Number::new(left_value * right_value),
        Signs::DIVIDE => Number::new(left_value / right_value),
        Signs::EMPOWERMENT | Signs::CARER => Number::new(left_value.powf(right_value)),
        Signs::MODULE => Number::new(left_value % right_value),
        Signs::LESSTHAN => Boolean::new(left_value < right_value),
        Signs::LESSOREQUALTHAN => Boolean::new(left_value <= right_value),
        Signs::GREATERTHAN => Boolean::new(left_value > right_value),
        Signs::GREATEROREQUALTHAN => Boolean::new(left_value >= right_value),
        Signs::EQUAL => Boolean::new(left_value == right_value),
        Signs::NOTEQUAL => Boolean::new(left_value != right_value),
        _ => error.clone(),
      };
    }
    // Check if left or right object is a string.
    else if infix.get_token().token.expect_sign(&Signs::PLUS) && (
      left_object.get_string().is_some() ||
      right_object.get_string().is_some()
    ) {
      return StringO::new(left_object.string() + &right_object.string());
    }
    // Check if the operator is an equal sign.
    else if infix.get_token().token.expect_sign(&Signs::EQUAL) {
      return Boolean::new(left_object == right_object);
    }
    // Check if the operator is a not equal sign.
    else if infix.get_token().token.expect_sign(&Signs::NOTEQUAL) {
      return Boolean::new(left_object != right_object);
    }
    // Check if the operator is an or sign.
    else if infix.get_token().token.expect_sign(&Signs::OR) {
      // TODO: The rest of the expressions.
      // Null objects.
      let mut return_right = left_object.get_null().is_some();

      // Empty strings
      if let Some(string) = left_object.get_string() {
        return_right = string.get_value().len() == 0;
      }

      // false boolean
      if let Some(boolean) = left_object.get_boolean() {
        return_right = boolean.get_value() == false;
      }

      // Return the object.
      return if return_right { right_object } else { left_object };
    }
    // Check if the operator is an and sign.
    else if infix.get_token().token.expect_sign(&Signs::AND) &&
      left_object.get_boolean().is_some() &&
      right_object.get_boolean().is_some() {
      return Boolean::new(
        left_object.get_boolean().unwrap().get_value() &&
        right_object.get_boolean().unwrap().get_value()
      );
    }
  }
  // Check if is an infix with 'in' or 'of'.
  else if infix.is_infix() && left_object.clone().is_none() {
    // Check if the token is 'in'.
    if infix.get_token().token.expect_keyword(&Keywords::IN) {
      if right_object.get_array().is_some() {
        let mut name: String = String::new();

        if let Some(identifier) = infix.get_left().get_identifier() {
          name = identifier.get_value();
        }

        return ForIn::new(
          name, 
          right_object.get_array().unwrap().get_elements(),
        );
      }

      return Error::new(
        String::from("expect an array expression."),
        infix.get_right().token(),
      );
    }
    // Check if the token is 'of'.
    else if infix.get_token().token.expect_keyword(&Keywords::OF) {}
  }

  error
}

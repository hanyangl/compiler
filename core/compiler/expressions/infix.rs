use crate::{
  compiler::{
    Array,
    Boolean,
    Error,
    ForIn,
    ForOf,
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
    Types,
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

  // Check if the token is 'in'.
  if infix.get_token().token.expect_keyword(&Keywords::IN) {
    if infix.get_left().get_identifier().is_none() {
      return Error::new(
        String::from("is not a valid expression for an `in`."),
        infix.get_left().token(),
      );
    }
  }
  // Check if the token is 'of'.
  else if infix.get_token().token.expect_keyword(&Keywords::OF) {
    if infix.get_left().get_array().is_none() {
      return Error::new(
        String::from("is not a valid expression for an `of`."),
        infix.get_left().token(),
      );
    }
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
    infix.get_right().is_some() &&
    infix.get_right().unwrap().token().value == "split" &&
    infix.get_right().unwrap().get_call().is_some() {
    let call_right = infix.get_right().unwrap().get_call().unwrap();
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

  if infix.is_type() {
    let right_type = infix.get_right_type().unwrap();

    if let Some(data_type) = right_type.token.get_type() {
      let left_object: Box<Objects> = left_object.unwrap();

      return Boolean::new(
        (left_object.get_boolean().is_some() && data_type == Types::BOOLEAN) ||
        (left_object.get_null().is_some() && data_type == Types::NULL) ||
        (left_object.get_number().is_some() && data_type == Types::NUMBER) ||
        (left_object.get_string().is_some() && data_type == Types::STRING)
      );
    }

    return Boolean::new(false);
  } else {
    // Evaluate right expression.
    let mut right_object: Box<Objects> = evaluate_expression(
      &infix.get_right().unwrap(),
      &mut right_environment,
    );

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
      let right_token = infix.get_right().unwrap().token();

      // Check if the method is 'toString()'.
      if right_token.value == "toString" && (
        left_object.get_string().is_some() ||
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
          infix.get_right().unwrap().token(),
        );
      }
      // Check if the token is 'of'.
      else if infix.get_token().token.expect_keyword(&Keywords::OF) {
        if let Some(hashmap) = right_object.get_hashmap() {
          if let Some(left_array) = infix.get_left().get_array() {
            if left_array.get_data().len() != 2 {
              return Error::new(
                format!("expect `2` elements, got `{}` instead.", left_array.get_data().len()),
                infix.get_left().token(),
              );
            }

            let mut names: Vec<String> = Vec::new();

            for element in left_array.get_data().iter() {
              if let Some(identifier) = element.get_identifier() {
                names.push(identifier.get_value());
                continue;
              }

              return Error::new(
                String::from("is not a valid identifier."),
                element.token(),
              );
            }

            return ForOf::new(names, hashmap.get_data());
          }
        }

        return Error::new(
          String::from("expect an hashmap expression."),
          infix.get_right().unwrap().token(),
        );
      }
    }
    // Check if is a variable set.
    else if infix.is_variable_set() && left_object.is_some() {
      if infix.get_token().token.expect_sign(&Signs::ASSIGN) {
        if let Some(identifier) = infix.get_left().get_identifier() {
          environment.store.set_object(identifier.get_value(), right_object.clone());
          return right_object;
        } else if let Some(array_index) = infix.get_left().get_array_index() {
          if let Some(env_obj) = environment.store.get_object(&array_index.get_token().value) {
            if let Some(array_obj) = env_obj.get_array() {
              let mut index: usize = 0;
              let mut elements: Vec<Box<Objects>> = array_obj.get_elements();

              if let Some(number) = array_index.get_index().get_number() {
                index = number.string().parse().unwrap();
              } else if let Some(prefix) = array_index.get_index().get_prefix() {
                if prefix.string() == "-1" {
                  index = elements.len() - 1;
                }
              }

              elements[index] = right_object;

              let new_array = Array::new(elements);

              environment.store.set_object(array_index.get_token().value, new_array.clone());

              return new_array;
            }
          }
        }
      } else {
        if let Some(number_o) = left_object.clone().unwrap().get_number() {
          let mut value: f64 = number_o.get_value();
          let number_two: f64 = right_object.get_number().unwrap().get_value();

          if infix.get_token().token.expect_sign(&Signs::PLUSASSIGN) {
            value += number_two;
          } else if infix.get_token().token.expect_sign(&Signs::MINUSASSIGN) {
            value -= number_two;
          } else if infix.get_token().token.expect_sign(&Signs::MULTIPLYASSIGN) {
            value *= number_two;
          } else if infix.get_token().token.expect_sign(&Signs::DIVIDEASSIGN) {
            value /= number_two;
          }

          let new_object = Number::new(value);

          if let Some(identifier) = infix.get_left().get_identifier() {
            environment.store.set_object(identifier.get_value(), new_object.clone());
            return new_object;
          } else if let Some(array_index) = infix.get_left().get_array_index() {
            if let Some(env_obj) = environment.store.get_object(&array_index.get_token().value) {
              if let Some(array_obj) = env_obj.get_array() {
                let mut index: usize = 0;
                let mut elements: Vec<Box<Objects>> = array_obj.get_elements();

                if let Some(number) = array_index.get_index().get_number() {
                  index = number.string().parse().unwrap();
                } else if let Some(prefix) = array_index.get_index().get_prefix() {
                  if prefix.string() == "-1" {
                    index = elements.len() - 1;
                  }
                }

                elements[index] = new_object;

                let new_array = Array::new(elements);

                environment.store.set_object(array_index.get_token().value, new_array.clone());

                return new_array;
              }
            }
          }
        } else if let Some(string_o) = left_object.clone().unwrap().get_string() {
          let new_object = StringO::new(
            format!(
              "{}{}",
              string_o.get_value(),
              right_object.get_string().unwrap().get_value(),
            ),
          );

          if let Some(identifier) = infix.get_left().get_identifier() {
            environment.store.set_object(identifier.get_value(), new_object.clone());
            return new_object;
          } else if let Some(array_index) = infix.get_left().get_array_index() {
            if let Some(env_obj) = environment.store.get_object(&array_index.get_token().value) {
              if let Some(array_obj) = env_obj.get_array() {
                let mut index: usize = 0;
                let mut elements: Vec<Box<Objects>> = array_obj.get_elements();

                if let Some(number) = array_index.get_index().get_number() {
                  index = number.string().parse().unwrap();
                } else if let Some(prefix) = array_index.get_index().get_prefix() {
                  if prefix.string() == "-1" {
                    index = elements.len() - 1;
                  }
                }

                elements[index] = new_object;

                let new_array = Array::new(elements);

                environment.store.set_object(array_index.get_token().value, new_array.clone());

                return new_array;
              }
            }
          }
        }
      }
    }
  }

  error
}

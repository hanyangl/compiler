mod call;
mod hashmap;
mod infix;
mod method;
mod prefix;

use crate::Environment;
use crate::objects::*;

use sflyn_parser::expressions::{Expressions, Expression, Identifier};
use sflyn_parser::tokens::Keywords;

pub fn evaluate_expressions(
  expressions: Vec<Box<Expressions>>,
  environment: &mut Environment,
) -> Vec<Box<Objects>> {
  let mut objects: Vec<Box<Objects>> = Vec::new();

  for expression in expressions {
    let object = evaluate(Some(expression.clone()), environment);

    // Check if the object is an error.
    if object.clone().is_error() {
      objects.clear();
      objects.push(object);

      return objects;
    }

    objects.push(object);
  }

  objects
}

pub fn evaluate(
  expression: Option<Box<Expressions>>,
  environment: &mut Environment,
) -> Box<Objects> {
  match expression {
    // Is an expression.
    Some(exp) => {
      // Anonymous function
      if exp.clone().is_anonymous_function() {
        let function = exp.clone().get_anonymous_function().unwrap();

        // Add default arguments
        AnonymousFunction::add_arguments_to_environment(function.arguments.clone(), environment);

        return AnonymousFunction::new(
          function.token.token.clone().expect_keyword(Keywords::FUNCTION),
          function.arguments.clone(),
          function.data_type.clone(),
          function.body.clone(),
          environment.clone(),
        );
      }

      // Argument

      // Array
      if exp.clone().is_array() {
        let elements = evaluate_expressions(
          exp.clone().get_array().unwrap().data,
          environment,
        );

        if elements.len() == 1 && elements[0].clone().is_error() {
          return elements[0].clone();
        }

        return Array::new(elements);
      }

      // Array index
      if exp.clone().is_array_index() {
        let array_index = exp.clone().get_array_index().unwrap();

        // Evaluate the array name.
        let identifier_object = evaluate(
          Some(Identifier::new_box_from_token(array_index.token.clone())),
          environment,
        );

        // Check if the identifier object is an error.
        if identifier_object.clone().is_error() {
          return identifier_object;
        }

        // Evaluate the array index.
        let index_object = evaluate(array_index.index.clone(), environment);

        // Check if the index object is an error.
        if index_object.clone().is_error() {
          return index_object;
        }

        // Get array value.
        if identifier_object.clone().is_array() && index_object.clone().is_number() {
          let index: usize;
          let elements = identifier_object.get_array().unwrap().elements;
          let value = index_object.get_number().unwrap().string();

          if value == "-1" {
            index = elements.len() - 1;
          } else {
            index = value.parse().unwrap();
          }

          return elements[index].clone();
        }

        return Error::new(format!("Unknown array index: {}", array_index.string()));
      }

      // Boolean
      if exp.clone().is_boolean() {
        return Boolean::new(exp.clone().get_boolean().unwrap().value);
      }

      // Call
      if exp.clone().is_call() {
        return call::evaluate(exp.clone().get_call().unwrap(), environment);
      }

      // HashMap
      if exp.clone().is_hashmap() {
        return hashmap::evaluate(exp.clone().get_hashmap().unwrap(), environment);
      }

      // Identifier
      if exp.clone().is_identifier() {
        let identifier = exp.clone().get_identifier().unwrap();

        return match environment.get(identifier.clone().value) {
          Some(object) => object,
          None => Error::new(format!("Identifier not found: {}", identifier.string())),
        };
      }

      // Infix
      if exp.clone().is_infix() {
        return infix::evaluate(exp.clone().get_infix().unwrap(), environment);
      }

      // Method
      if exp.clone().is_method() {
        return method::evaluate(exp.clone().get_method().unwrap(), environment);
      }

      // Number
      if exp.clone().is_number() {
        return Number::new(exp.clone().get_number().unwrap().value);
      }

      // Prefix
      if exp.clone().is_prefix() {
        return prefix::evaluate(exp.clone().get_prefix().unwrap(), environment);
      }

      // String
      if exp.clone().is_string() {
        let value = exp.clone().get_string().unwrap().value;
        return StringO::new(value[1..value.len() - 1].to_string());
      }

      // Default
      Error::new(format!("Unknown expression: {}", exp.clone().string()))
    },

    // Is not an expression.
    None => Error::new(String::from("Is not a valid expression.")),
  }
}

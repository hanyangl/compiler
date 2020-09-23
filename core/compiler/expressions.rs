mod call;
mod for_condition;
mod hashmap;
mod infix;
mod prefix;

use crate::{
  compiler::{
    AnonymousFunction,
    Array,
    Boolean,
    builtins::get_builtin_for_identifier,
    Error,
    Null,
    Number,
    Object,
    Objects,
    StringO,
  },
  Environment,
};

use sflyn_parser::{
  Expression,
  Expressions,
  Identifier,
  tokens::Signs,
};

pub fn evaluate_expressions(
  expressions: Vec<Box<Expressions>>,
  environment: &mut Environment,
) -> Vec<Box<Objects>> {
  let mut objects: Vec<Box<Objects>> = Vec::new();

  for expression in expressions.iter() {
    let object = evaluate_expression(expression, environment);

    // Check if the object is an error.
    if object.get_error().is_some() {
      objects.clear();
      objects.push(object);

      return objects;
    }

    objects.push(object);
  }

  objects
}

pub fn evaluate_expression(
  expression: &Box<Expressions>,
  environment: &mut Environment,
) -> Box<Objects> {
  // Anonymous function
  if let Some(anonymous_function) = expression.get_anonymous_function() {
    AnonymousFunction::add_arguments_to_environment(
      anonymous_function.get_arguments(),
      environment,
    );

    let object = AnonymousFunction::new(
      true,
      anonymous_function.get_arguments(),
      anonymous_function.get_type(),
      anonymous_function.get_body(),
      &environment.store,
    );

    return object;
  }

  // Array
  if let Some(array) = expression.get_array() {
    // Evaluate array elements.
    let elements = evaluate_expressions(array.get_data(), environment);

    // Check if the first element is an error.
    if elements.len() == 1 && elements[0].get_error().is_some() {
      return elements[0].clone();
    }

    return Array::new(elements);
  }

  // Array index
  if let Some(array_index) = expression.get_array_index() {
    let mut identifier_obj = evaluate_expression(&Identifier::new_box_from_token(array_index.get_token()), environment);

    // Check if the identifier object is an error.
    if identifier_obj.get_error().is_some() {
      if let Some(left_exp) = array_index.get_left() {
        identifier_obj = evaluate_expression(&left_exp, environment);
      }

      if identifier_obj.get_error().is_some() {
        return identifier_obj;
      }
    }

    if let Some(return_o) = identifier_obj.get_return() {
      identifier_obj = return_o.get_value();
    }

    let index_obj = evaluate_expression(&array_index.get_index(), environment);

    // Check if the index object is an error.
    if index_obj.get_error().is_some() {
      return index_obj;
    }

    // Get string value.
    if identifier_obj.get_string().is_some() && index_obj.get_number().is_some() {
      let index: usize;
      let string: String = identifier_obj.get_string().unwrap().get_value();
      let value: String = index_obj.get_number().unwrap().string();

      if value == "-1" {
        index = string.len() - 1;
      } else {
        index = value.parse().unwrap();
      }

      if index >= string.len() {
        return Null::new();
      }

      return StringO::new(string[index..index + 1].to_string());
    }
    // Get array value.
    else if identifier_obj.get_array().is_some() && index_obj.get_number().is_some() {
      let index: usize;
      let elements = identifier_obj.get_array().unwrap().get_elements();
      let value = index_obj.get_number().unwrap().string();

      if value == "-1" {
        index = if elements.len() > 0 { elements.len() - 1 } else { 0 };
      } else {
        index = value.parse().unwrap();
      }

      if index >= elements.len() {
        return Null::new();
      }

      return elements[index].clone();
    }
  }

  // Boolean
  if let Some(boolean) = expression.get_boolean() {
    return Boolean::new(boolean.get_value());
  }

  // Call
  if let Some(call) = expression.get_call() {
    return call::evaluate(call, environment);
  }

  // For condition
  if let Some(for_condition_exp) = expression.get_for_condition() {
    return for_condition::evaluate(&for_condition_exp, environment);
  }

  // HashMap
  if let Some(hashmap_exp) = expression.get_hashmap() {
    return hashmap::evaluate(hashmap_exp, environment);
  }

  // Identifier
  if let Some(identifier) = expression.get_identifier() {
    if let Some(obj) = environment.store.get_object(&identifier.get_value()) {
      return obj.clone();
    }

    return get_builtin_for_identifier(identifier.get_token());
  }

  // Infix
  if let Some(infix_exp) = expression.get_infix() {
    return infix::evaluate(&infix_exp, environment);
  }

  // Null
  if expression.get_null().is_some() {
    return Null::new();
  }

  // Number
  if let Some(number) = expression.get_number() {
    return Number::new(number.get_value());
  }

  // Prefix
  if let Some(prefix_exp) = expression.get_prefix() {
    return prefix::evaluate(&prefix_exp, environment);
  }

  // String
  if let Some(string) = expression.get_string() {
    return StringO::new(string.get_value()[1..string.get_value().len() - 1].to_string());
  }

  // Suffix
  if let Some(suffix) = expression.get_suffix() {
    let mut left_obj = evaluate_expression(&suffix.get_left(), environment);

    if left_obj.get_error().is_some() {
      return left_obj;
    }

    if let Some(return_obj) = left_obj.get_return() {
      left_obj = return_obj.get_value();
    }

    if suffix.get_token().token.expect_sign(&Signs::PLUSPLUS) ||
      suffix.get_token().token.expect_sign(&Signs::MINUSMINUS) {
      if let Some(number_obj) = left_obj.get_number() {
        if suffix.get_token().token.expect_sign(&Signs::MINUSMINUS) {
          return Number::new(number_obj.get_value() - 1.0);
        }

        return Number::new(number_obj.get_value() + 1.0);
      }

      return Error::new(
        format!("`{}` is not a valid number.", suffix.get_left().string()),
        suffix.get_left().token(),
      );
    }
  }

  // Default
  Error::new(
    format!("`{}` is not a valid expression.", expression.string()),
    expression.token(),
  )
}

mod call;
mod hashmap;
mod infix;
mod prefix;

use crate::{
  compiler::{
    AnonymousFunction,
    Array,
    builtins::get_builtin_for_identifier,
    Boolean,
    Error,
    Number,
    Objects,
    StringO,
  },
  Environment,
};

use sflyn_parser::Expressions;

pub fn evaluate_expressions(
  expressions: Vec<Box<Expressions>>,
  environment: &mut Environment,
) -> Vec<Box<Objects>> {
  let mut objects: Vec<Box<Objects>> = Vec::new();

  for expression in expressions {
    let object = evaluate_expression(expression.clone(), environment);

    // Check if the object is an error.
    if object.clone().get_error().is_some() {
      objects.clear();
      objects.push(object);

      return objects;
    }

    objects.push(object);
  }

  objects
}

pub fn evaluate_expression(
  expression: Box<Expressions>,
  environment: &mut Environment,
) -> Box<Objects> {
  // Anonymous function
  if let Some(anonymous_function) = expression.clone().get_anonymous_function() {
    AnonymousFunction::add_arguments_to_environment(
      anonymous_function.arguments.clone(),
      environment,
    );

    let object = AnonymousFunction::new(
      true,
      anonymous_function.arguments.clone(),
      anonymous_function.data_type,
      anonymous_function.body,
      environment.store.clone(),
    );

    return object;
  }

  // Argument

  // Array
  if let Some(array) = expression.clone().get_array() {
    // Evaluate array elements.
    let elements = evaluate_expressions(array.data, environment);

    // Check if the first element is an error.
    if elements.len() == 0 && elements[0].clone().get_error().is_some() {
      return elements[0].clone();
    }

    return Array::new(elements);
  }

  // Array index

  // Boolean
  if let Some(boolean) = expression.clone().get_boolean() {
    return Boolean::new(boolean.value);
  }

  // Call
  if let Some(call) = expression.clone().get_call() {
    return call::evaluate(call, environment);
  }

  // HashMap
  if let Some(hashmap_exp) = expression.clone().get_hashmap() {
    return hashmap::evaluate(hashmap_exp, environment);
  }

  // Identifier
  if let Some(identifier) = expression.clone().get_identifier() {
    return match environment.store.get_object(identifier.value) {
      Some(object) => object.clone(),
      None => get_builtin_for_identifier(identifier.token),
    };
  }

  // Infix
  if let Some(infix_exp) = expression.clone().get_infix() {
    return infix::evaluate(infix_exp, environment);
  }

  // Number
  if let Some(number) = expression.clone().get_number() {
    return Number::new(number.value);
  }

  // Prefix
  if let Some(prefix_exp) = expression.clone().get_prefix() {
    return prefix::evaluate(prefix_exp, environment);
  }

  // String
  if let Some(string) = expression.clone().get_string() {
    return StringO::new(string.value[1..string.value.len() - 1].to_string());
  }

  // Default
  Error::new(
    String::from("is not a valid expression."),
    expression.token(),
  )
}

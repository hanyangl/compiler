mod anonymous_function;
mod argument;
mod array;
mod call;
mod hashmap;
mod infix;
mod prefix;

pub use argument::function_arguments_to_string;

use crate::{
  Environment,
  typechecker::TTypes,
};

use sflyn_parser::{
  Error,
  Expression,
  Expressions,
  tokens::Types,
};

pub fn check_expression(
  expression: &Box<Expressions>,
  environment: &mut Environment,
) -> Result<TTypes, Error> {
  // Anonymous Function
  if let Some(anonymous_function_exp) = expression.get_anonymous_function() {
    return anonymous_function::check(&anonymous_function_exp, environment);
  }

  // Argument

  // Array
  if let Some(array_exp) = expression.get_array() {
    return array::check(&array_exp, environment);
  }

  // Array Index
  if let Some(array_index) = expression.get_array_index() {
    return array::check_index(&array_index, environment);
  }

  // Boolean
  if let Some(boolean) = expression.get_boolean() {
    return Ok(TTypes::new_type(Types::BOOLEAN, String::from("boolean"), boolean.get_token()));
  }

  // Call
  if let Some(call_exp) = expression.get_call() {
    return call::check(&call_exp, environment);
  }

  // HashMap
  if let Some(hashmap_exp) = expression.get_hashmap() {
    return hashmap::check(&hashmap_exp, environment);
  }

  // Identifier
  if let Some(identifier) = expression.get_identifier() {
    return match environment.store.get_type(&identifier.get_value()) {
      Some(token) => Ok(token),
      None => Err(Error::from_token(
        format!("`{} identifier not found.`", identifier.get_value()),
        identifier.get_token(),
      )),
    }
  }

  // Infix
  if let Some(infix_exp) = expression.get_infix() {
    return infix::check(&infix_exp, environment);
  }

  // Null
  if let Some(null) = expression.get_null() {
    return Ok(TTypes::new_type(Types::NULL, String::from("null"), null.get_token()));
  }

  // Number
  if let Some(number) = expression.get_number() {
    return Ok(TTypes::new_type(Types::NUMBER, String::from("number"), number.get_token()));
  }

  // Prefix
  if let Some(prefix_exp) = expression.get_prefix() {
    return prefix::check(&prefix_exp, environment);
  }

  // String
  if let Some(string) = expression.get_string() {
    return Ok(TTypes::new_type(Types::STRING, String::from("string"), string.get_token()));
  }

  // Default
  Err(Error::from_token(
    String::from("unknown expression."),
    expression.token(),
  ))
}

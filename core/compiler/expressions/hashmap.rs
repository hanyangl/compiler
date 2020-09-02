use crate::{
  compiler::{
    HashItem,
    HashMap as HashMapO,
    Objects,
  },
  Environment,
};

use sflyn_parser::HashMap;

use super::evaluate_expression;

pub fn evaluate(
  hashmap: HashMap,
  environment: &mut Environment,
) -> Box<Objects> {
  let mut data: Vec<HashItem> = Vec::new();

  for (key, value) in hashmap.items {
    // Compile item value.
    let value_object = evaluate_expression(value, environment);

    // Check if the value object is an error.
    if value_object.clone().get_error().is_some() {
      return value_object;
    }

    // Add hash item to the hashmap data.
    data.push(HashItem {
      key,
      value: value_object.clone(),
    });
  }

  HashMapO::new(data)
}

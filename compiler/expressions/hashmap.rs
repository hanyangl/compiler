use crate::Environment;
use crate::objects::{Objects, HashItem, HashMap as HashMapO};

use sflyn_parser::expressions::HashMap;

use super::evaluate as evaluate_expression;

pub fn evaluate(
  file_name: String,
  hashmap: HashMap,
  environment: &mut Environment,
) -> Box<Objects> {
  let mut data: Vec<HashItem> = Vec::new();

  for item in hashmap.data {
    // Compile item value.
    let value_object = evaluate_expression(file_name.clone(), Some(item.value), environment);

    // Check if the value object is an error.
    if value_object.clone().is_error() {
      return value_object;
    }

    // Add hash item to the hashmap data.
    data.push(HashItem {
      key: item.key.clone(),
      value: value_object.clone(),
    });
  }

  HashMapO::new(data)
}

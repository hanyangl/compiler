use sflyn_parser::tokens::{
  Token,
  Types,
};

pub fn equal_types(one: Types, two: Types) -> bool {
  // Check if both types are functions.
  if one.get_function().is_some() && two.get_function().is_some() {
    let one_function = one.get_function().unwrap();
    let two_function = two.get_function().unwrap();

    if one_function.get_arguments().len() == two_function.get_arguments().len() &&
      equal_tokens(one_function.get_type(), two_function.get_type()) {
      for (key, value) in one_function.get_arguments().iter() {
        if let Some(value_token) = two_function.get_arguments().get(key) {
          if equal_tokens(value.clone(), value_token.clone()) {
            continue;
          }
        }

        return false;
      }

      return true;
    }
  }
  // Check if both types are hashmaps.
  else if one.get_hashmap().is_some() && two.get_hashmap().is_some() {
    let one_hashmap = one.get_hashmap().unwrap();
    let two_hashmap = two.get_hashmap().unwrap();

    if one_hashmap.get_items().len() == two_hashmap.get_items().len() {
      for (key, value) in one_hashmap.get_items().iter() {
        if let Some(value_token) = two_hashmap.get_items().get(key) {
          if equal_tokens(value.clone(), value_token.clone()) {
            continue;
          }
        }

        return false;
      }

      return true;
    }
  }

  one == two
}

pub fn equal_tokens(one: Token, two: Token) -> bool {
  if one.token.get_type().is_some() && two.token.get_type().is_some() {
    return equal_types(one.token.get_type().unwrap(), two.token.get_type().unwrap());
  }

  one == two
}

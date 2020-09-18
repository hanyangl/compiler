use crate::typechecker::TTypes;

use sflyn_parser::tokens::{
  Token,
  Types,
};

use std::collections::HashMap;

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
  // Check if both types are arrays.
  else if one.get_array().is_some() && two.get_array().is_some() {
    let one_array = one.get_array().unwrap();
    let two_array = two.get_array().unwrap();

    return equal_tokens(one_array.get_type(), two_array.get_type());
  }

  one == two
}

pub fn equal_tokens(one: Token, two: Token) -> bool {
  if one.token.get_type().is_some() && two.token.get_type().is_some() {
    return equal_types(one.token.get_type().unwrap(), two.token.get_type().unwrap());
  }

  one == two
}

pub fn get_ttypes_from_token(
  token: Token,
  token_to_ttype: Token,
) -> Option<TTypes> {
  // Get the token data type.
  if let Some(token_type) = token.token.get_type() {
    // Check if is an array.
    if token_type.get_array().is_some() {
      return Some(TTypes::new_array(
        token_type,
        token.value,
        token_to_ttype,
      ));
    }
    // Check if is an hashmap.
    else if let Some(hashmap) = token_type.get_hashmap() {
      let mut methods: HashMap<String, TTypes> = HashMap::new();

      for (key, value) in hashmap.get_items().iter() {
        if let Some(ttype) = get_ttypes_from_token(value.clone(), token_to_ttype.clone()) {
          methods.insert(key.clone(), ttype);
          continue;
        }

        return None;
      }

      return Some(TTypes::new_hashmap(
        token_type,
        token.value,
        token_to_ttype,
        methods,
      ));
    }
    // Check if is a function.
    else if let Some(function) = token_type.get_function() {
      println!("Function TType: {:?}\n", function);
      return None;
    }

    return Some(TTypes::new_type(
      token_type,
      token.value,
      token_to_ttype,
    ));
  }

  None
}

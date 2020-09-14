use crate::{
  Environment,
  typechecker::{
    check_expression,
    TTypes,
  },
};

use sflyn_parser::{
  Error,
  Expression,
  HashMap,
  tokens::Token,
};

use std::collections::HashMap as HashMapSTD;

pub fn check(
  hashmap: &HashMap,
  environment: &mut Environment,
) -> Result<TTypes, Error> {
  let mut items: Vec<String> = Vec::new();
  let mut methods: HashMapSTD<String, TTypes> = HashMapSTD::new();

  for (key, value) in hashmap.get_items().iter() {
    let mut new_item: String = key.clone();

    // Check item value data type.
    match check_expression(value, environment) {
      Ok(data_type) => {
        new_item.push_str(": ");
        new_item.push_str(data_type.get_value().as_str());
        methods.insert(key.clone(), data_type);
      },
      Err(error) => {
        return Err(error);
      },
    }

    items.push(new_item);
  }

  let mut value = String::from("{ ");

  value.push_str(&items.join(", "));
  value.push_str(" }");

  let token = Token::from_value(value.as_str(), 0, 0);

  if token.token.get_type().is_none() {
    return Err(Error::from_token(
      String::from("is not a valid hashmap."),
      hashmap.get_token(),
    ));
  }

  Ok(TTypes::new_hashmap(token.token.get_type().unwrap(), value, hashmap.get_token(), methods))
}

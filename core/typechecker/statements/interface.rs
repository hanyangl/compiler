use crate::{
  Environment,
  typechecker::TTypes,
};

use sflyn_parser::{
  Error,
  Interface,
  Statement,
  tokens::Token,
};

use std::collections::HashMap;

pub fn check(
  interface: &Interface,
  environment: &mut Environment,
) -> Result<TTypes, Error> {
  // Check if the interface name is already in use.
  if environment.store.get_type(&interface.get_name().value).is_some() {
    return Err(Error::from_token(
      format!("`{}` is already in use.", interface.get_name().value),
      interface.get_name(),
    ));
  }

  let mut values: Vec<String> = Vec::new();
  let mut methods: HashMap<String, TTypes> = HashMap::new();

  for method in interface.get_methods().iter() {
    if method.get_type().token.get_type().is_none() {
      return Err(Error::from_token(
        format!("`{}` is not a valid data type.", method.get_type().value),
        method.get_type(),
      ));
    }

    values.push(format!("{}: {}", method.get_token().value, method.get_type().value));

    methods.insert(
      method.get_token().value,
      TTypes::new_type(
        method.get_type().token.get_type().unwrap(),
        method.get_type().value,
        method.get_token(),
      ),
    );
  }

  let mut value = String::from("{");

  value.push_str(&values.join(", "));
  value.push_str("}");

  let token = Token::from_value(value.as_str(), 0, 0);

  if token.token.get_type().is_none() {
    return Err(Error::from_token(
      String::from("is not a valid interface."),
      interface.get_token(),
    ));
  }

  let ttype = TTypes::new_interface(
    token.token.get_type().unwrap(),
    value,
    interface.get_token(),
    methods,
  );

  environment.store.set_type(interface.get_name().value, ttype.clone());

  Ok(ttype)
}

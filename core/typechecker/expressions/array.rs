use crate::{
  Environment,
  typechecker::{
    check_expression,
    equal_types,
    TTypes,
  },
};

use sflyn_parser::{
  Array,
  ArrayIndex,
  Error,
  Expression,
  tokens::{
    Array as ArrayType,
    Token,
    Types,
  },
};

pub fn check(
  array: &Array,
  environment: &mut Environment,
) -> Result<TTypes, Error> {
  if array.get_data().len() == 0 {
    return Ok(TTypes::new_array(
      Types::ARRAY(ArrayType::from_value("null[]").unwrap()),
      String::from("any"),
      array.get_token(),
    ));
  }

  let mut data_type: Option<TTypes> = None;

  for item in array.get_data().iter() {
    match check_expression(item, environment) {
      Ok(token) => {
        if data_type.clone().is_some() {
          if !equal_types(data_type.clone().unwrap().get_type(), token.get_type()) {
            return Err(Error::from_token(
              format!("`{}` not satisfied the `{}` data type.", token.get_value(), data_type.unwrap().get_value()),
              token.get_token(),
            ));
          }

          continue;
        }

        data_type = Some(token);
      },
      Err(error) => {
        return Err(error);
      },
    }
  }

  if let Some(data_type) = data_type {
    let value = format!("{}[]", data_type.get_value());
    let token = Token::from_value(value.as_str(), 0, 0);

    if token.token.get_type().is_none() {
      return Err(Error::from_token(
        String::from("is not a valid array."),
        array.get_token(),
      ));
    }

    return Ok(TTypes::new_array(
      token.token.get_type().unwrap(),
      value,
      array.get_token(),
    ));
  }

  Err(Error::from_token(
    String::from("invalid array expression."),
    array.get_token(),
  ))
}

pub fn check_index(
  array_index: &ArrayIndex,
  environment: &mut Environment,
) -> Result<TTypes, Error> {
  let mut array_type = environment.store.get_type(&array_index.get_token().value);

  if array_type.is_none() {
    if let Some(left_exp) = array_index.get_left() {
      match check_expression(&left_exp, environment) {
        Ok(token) => {
          array_type = Some(token);
        },
        Err(error) => {
          return Err(error);
        },
      }
    }

    if array_type.is_none() {
      return Err(Error::from_token(
        format!("`{}` identifier not found.", array_index.get_token().value),
        array_index.get_token(),
      ));
    }
  }

  let array_type = array_type.unwrap();

  if array_type.get_type() == Types::STRING {
    return Ok(TTypes::new_type(
      array_type.get_type(),
      array_type.get_value(),
      array_index.get_token(),
    ));
  } else if !array_type.is_array() {
    return Err(Error::from_token(
      format!("`{}` is not an array.", array_index.get_token().value),
      array_index.get_token(),
    ));
  }

  let array = array_type.get_type().get_array().unwrap().get_type();

  Ok(TTypes::new_type(
    array.token.get_type().unwrap(),
    array.value,
    array_index.get_token(),
  ))
}

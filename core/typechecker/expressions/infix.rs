use crate::{
  Environment,
  Store,
  typechecker::{
    check_expression,
    equal_types,
    get_ttypes_from_token,
    TTypes,
  },
};

use sflyn_parser::{
  Error,
  Expression,
  Infix,
  tokens::{
    Keywords,
    Signs,
    Token,
    Types,
  },
};

pub fn check(
  infix: &Infix,
  environment: &mut Environment,
) -> Result<TTypes, Error> {
  // Get the left expression type.
  let mut left_type: Option<TTypes> = None;

  // Check if the token is 'in'.
  if infix.get_token().token.expect_keyword(&Keywords::IN) {
    if infix.get_left().get_identifier().is_none() {
      return Err(Error::from_token(
        String::from("is not a valid expression for an `in`."),
        infix.get_left().token(),
      ));
    }
  }
  // Check if the token is 'of'.
  else if infix.get_token().token.expect_keyword(&Keywords::OF) {
    if infix.get_left().get_array().is_none() {
      return Err(Error::from_token(
        String::from("is not a valid expression for an `of`."),
        infix.get_left().token(),
      ));
    }
  } else {
    match check_expression(&infix.get_left(), environment) {
      Ok(token) => {
        left_type = Some(token);
      },
      Err(error) => {
        return Err(error);
      },
    }

    if infix.is_variable_set() {
      let mut name = String::new();
      let mut token = Token::new_empty();

      if let Some(identifier) = infix.get_left().get_identifier() {
        name = identifier.get_value();
        token = identifier.get_token();
      } else if let Some(array_index) = infix.get_left().get_array_index() {
        name = array_index.get_token().value;
        token = array_index.get_token();
      }

      if environment.store.has_const(&name) {
        return Err(Error::from_token(
          format!("`{}` is a const.", name),
          token,
        ));
      }
    }
  } 

  // Create a new environment.
  let mut right_environment = environment.clone();

  right_environment.store = Store::from_store(&environment.store);

  // Check if is a method.
  if infix.is_method() && left_type.clone().is_some() {
    let left_type: TTypes = left_type.clone().unwrap();
    let mut from_std = "";

    // Check if the left type is null.
    if left_type.get_type() == Types::NULL {
      from_std = "Null";
    }
    // Check if the left type is a string.
    else if left_type.get_type() == Types::STRING {
      from_std = "String";
    }
    // Check if the left type is a number.
    else if left_type.get_type() == Types::NUMBER {
      from_std = "Number";
    }
    // Check if the left type is a boolean.
    else if left_type.get_type() == Types::BOOLEAN {
      from_std = "Boolean";
    }
    // Check if the left type is an array.
    else if left_type.get_type().get_array().is_some() {
      from_std = "Array";
    }
    // Check if the left type is a hashmap or an interface.
    else if left_type.is_hashmap() || left_type.is_interface() {
      for (key, value) in left_type.get_methods() {
        right_environment.store.set_type(key, value);
      }
    }

    // Check if `from_std` is not empty.
    if !from_std.is_empty() {
      if let Some(data_type) = environment.store.get_type(&from_std.to_string()) {
        if data_type.is_hashmap() {
          for (key, value) in data_type.get_methods() {
            right_environment.store.set_type(key, value);
          }
        }
      }
    }
  }

  // Get the right type.
  let right_type: TTypes;

  if infix.is_type() {
    let right_token = infix.get_right_type().unwrap();

    match get_ttypes_from_token(right_token.clone(), right_token.clone()) {
      Some(token) => {
        right_type = token;
      },
      None => {
        return Err(Error::from_token(
          String::from("is not a valid data type."),
          right_token,
        ));
      },
    }
  } else {
    match check_expression(&infix.get_right().unwrap(), &mut right_environment) {
      Ok(token) => {
        right_type = token;
      },
      Err(error) => {
        return Err(error);
      },
    }
  }

  // Check if is a method.
  if infix.is_method() {
    return Ok(right_type);
  }
  // Check if is an infix without 'in' or 'of'.
  else if infix.is_infix() && left_type.clone().is_some() {
    let left_type: TTypes = left_type.clone().unwrap();

    // Parse '-', '/', '*', '^', '**' and '%' with numbers.
    if infix.get_token().token.expect_sign(&Signs::MINUS) ||
      infix.get_token().token.expect_sign(&Signs::DIVIDE) ||
      infix.get_token().token.expect_sign(&Signs::MULTIPLY) ||
      infix.get_token().token.expect_sign(&Signs::CARER) ||
      infix.get_token().token.expect_sign(&Signs::EMPOWERMENT) ||
      infix.get_token().token.expect_sign(&Signs::MODULE) {
      if left_type.get_type() != Types::NUMBER || right_type.get_type() != Types::NUMBER {
        return Err(Error::from_token(
          String::from("only can do this with numbers."),
          infix.get_token(),
        ));
      }

      return Ok(left_type);
    }
    // Parse '<', '<=', '>' and '>=' with numbers.
    else if infix.get_token().token.expect_sign(&Signs::LESSTHAN) ||
      infix.get_token().token.expect_sign(&Signs::LESSOREQUALTHAN) ||
      infix.get_token().token.expect_sign(&Signs::GREATERTHAN) ||
      infix.get_token().token.expect_sign(&Signs::GREATEROREQUALTHAN) {
      if left_type.get_type() != Types::NUMBER || right_type.get_type() != Types::NUMBER {
        return Err(Error::from_token(
          String::from("only can do this with numbers."),
          infix.get_token(),
        ));
      }

      return Ok(TTypes::new_type(Types::BOOLEAN, String::from("boolean"), infix.get_token()));
    }
    // Parse '+' with numbers and strings.
    else if infix.get_token().token.expect_sign(&Signs::PLUS) {
      if left_type.get_type() == Types::NUMBER && right_type.get_type() == Types::NUMBER {
        return Ok(left_type);
      } else if left_type.get_type() == Types::STRING && right_type.get_type() == Types::STRING {
        return Ok(left_type);
      }

      return Err(Error::from_token(
        format!("can not concat `{}` with `{}`.", left_type.get_token().value, right_type.get_token().value),
        infix.get_token(),
      ));
    }
    // Parse '==' and '!='.
    else if infix.get_token().token.expect_sign(&Signs::EQUAL) || infix.get_token().token.expect_sign(&Signs::NOTEQUAL) {
      return Ok(TTypes::new_type(Types::BOOLEAN, String::from("boolean"), infix.get_token()));
    }
    // Parse '||'.
    else if infix.get_token().token.expect_sign(&Signs::OR) {
      if left_type.get_type() == Types::NULL {
        return Ok(right_type);
      }

      if !equal_types(left_type.get_type(), right_type.get_type()) {
        return Err(Error::from_token(
          format!("`{}` not satisfied the `{}` data type.", right_type.get_value(), left_type.get_value()),
          right_type.get_token(),
        ));
      }

      return Ok(left_type);
    }
    // Parse '&&'.
    else if infix.get_token().token.expect_sign(&Signs::AND) {
      if left_type.get_type() != Types::BOOLEAN || right_type.get_type() != Types::BOOLEAN {
        return Err(Error::from_token(
          String::from("only can compare two boolean."),
          infix.get_token(),
        ));
      }

      return Ok(left_type);
    }
  }
  // Check if is an infix with 'in' or 'of'.
  else if infix.is_infix() && left_type.clone().is_none() {
    // Check if the token is 'in'.
    if infix.get_token().token.expect_keyword(&Keywords::IN) {
      if right_type.is_array() && right_type.get_type().get_array().is_some() {
        return Ok(TTypes::new_for_in(
          right_type.get_type(),
          right_type.get_value(),
          right_type.get_token(),
          infix.get_left().get_identifier().unwrap().get_value(),
        ));
      }

      return Err(Error::from_token(
        String::from("expect an array expression."),
        infix.get_right().unwrap().token(),
      ));
    }
    // Check if the token is 'of'.
    else if infix.get_token().token.expect_keyword(&Keywords::OF) {
      if right_type.is_hashmap() && right_type.get_type().get_hashmap().is_some() {
        if let Some(left_array) = infix.get_left().get_array() {
          if left_array.get_data().len() != 2 {
            return Err(Error::from_token(
              format!("expect `2` elements, got `{}` instead.", left_array.get_data().len()),
              infix.get_left().token(),
            ));
          }

          let mut names: Vec<String> = Vec::new();

          for element in left_array.get_data().iter() {
            if let Some(identifier) = element.get_identifier() {
              names.push(identifier.get_value());
              continue;
            }

            return Err(Error::from_token(
              String::from("is not a valid identifier."),
              element.token(),
            ));
          }

          return Ok(TTypes::new_for_of(
            right_type.get_type(),
            right_type.get_value(),
            right_type.get_token(),
            right_type.get_methods(),
            names,
          ));
        }
      }

      return Err(Error::from_token(
        String::from("expect an hashmap expression."),
        infix.get_right().unwrap().token(),
      ));
    }
  }
  // Check if is a type.
  else if infix.is_type() {
    return Ok(TTypes::new_type(Types::BOOLEAN, String::from("boolean"), infix.get_token()));
  }
  // Check if is a variable set.
  else if infix.is_variable_set() && left_type.is_some() {
    let left_type: TTypes = left_type.unwrap();

    if (
      infix.get_token().token.expect_sign(&Signs::MINUSASSIGN) ||
      infix.get_token().token.expect_sign(&Signs::MULTIPLYASSIGN) ||
      infix.get_token().token.expect_sign(&Signs::DIVIDEASSIGN)
    ) && left_type.get_type() != Types::NUMBER {
      return Err(Error::from_token(
        format!("`{}` is not a number.", infix.get_left().string()),
        infix.get_left().token(),
      ));
    } else if infix.get_token().token.expect_sign(&Signs::PLUSASSIGN) &&
      left_type.get_type() != Types::NUMBER &&
      left_type.get_type() != Types::STRING {
      return Err(Error::from_token(
        format!("`{}` is not a number or a string.", infix.get_left().string()),
        infix.get_left().token(),
      ));
    }
    
    if equal_types(left_type.get_type(), right_type.get_type()) ||
      left_type.get_value() == "any" {
      return Ok(right_type);
    }

    return Err(Error::from_token(
      format!(
        "`{}` not satisfied the `{}` data type.",
        infix.get_right().unwrap().string(),
        left_type.get_value(),
      ),
      infix.get_right().unwrap().token(),
    ));
  }

  Err(Error::from_token(
    String::from("invalid infix expression."),
    infix.get_token(),
  ))
}

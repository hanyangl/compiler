use crate::{
  Environment,
  Store,
  typechecker::{
    check_expression,
    equal_types,
    TTypes,
  },
};

use sflyn_parser::{
  Error,
  Expression,
  Infix,
  tokens::{
    Signs,
    Types,
  },
};

pub fn check(
  infix: &Infix,
  environment: &mut Environment,
) -> Result<TTypes, Error> {
  // Get the left expression type.
  let left_type;

  match check_expression(&infix.get_left(), environment) {
    Ok(token) => {
      left_type = token;
    },
    Err(error) => {
      return Err(error);
    },
  }

  // Create a new environment.
  let mut right_environment = environment.clone();

  right_environment.store = Store::from_store(environment.store.clone());

  // Check if is a method.
  if infix.is_method() {
    let mut from_std = "";

    // Check if the left type is null.
    if left_type.get_type() == Types::NULL {
      from_std = "Null";
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
  let right_type;

  match check_expression(&infix.get_right(), &mut right_environment) {
    Ok(token) => {
      right_type = token;
    },
    Err(error) => {
      return Err(error);
    },
  }

  // Check if is a method.
  if infix.is_method() {
    return Ok(right_type);
  }
  // Check if is an infix.
  else if infix.is_infix() {
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

  Err(Error::from_token(
    String::from("invalid infix expression."),
    infix.get_token(),
  ))
}

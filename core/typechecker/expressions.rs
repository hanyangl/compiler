use crate::{
  Environment,
  Store,
};

use sflyn_parser::{
  Error,
  Expressions,
  tokens::{
    Signs,
    Token,
    Types,
  },
};

use super::{
  check_types,
  check_statement,
  equal_type_and_interface,
  function_arguments_to_string,
};

pub fn check_expression(
  expression: Box<Expressions>,
  environment: &mut Environment,
) -> Result<Token, Error> {
  // Anonymous function
  if let Some(anonymous_function) = expression.clone().get_anonymous_function() {
    // Create a new closed environment.
    let mut function_environment = environment.clone();

    function_environment.store = Store::from_store(environment.store.clone());

    let arguments: Vec<String>;

    match function_arguments_to_string(anonymous_function.arguments.clone(), environment, &mut function_environment) {
      Ok(args) => {
        arguments = args;
      },
      Err(error) => {
        return Err(error);
      },
    }

    // Get the function data type.
    let mut data_type = anonymous_function.data_type.clone();

    // Get the data type token from the function body.
    match check_statement(anonymous_function.body, &mut function_environment) {
      Ok(token) => {
        // Check if the token is a valid data type.
        if token.token.clone().get_type().is_some() {
          // Set the new data type.
          if data_type.line == 0 {
            data_type = token.clone();
          }
        }
      }
      Err(error) => {
        return Err(error);
      }
    }

    // Get the function string value.
    let value = format!(
      "({}) => {}",
      arguments.join(", "),
      data_type.value,
    );

    // TODO: Set function arguments to the environment.

    return Ok(Token::from_value(value.as_str(), 0, 0));
  }

  // Argument

  // Array
  if let Some(array) = expression.clone().get_array() {
    let mut types: Vec<String> = Vec::new();

    for item in array.data.iter() {
      match check_expression(item.clone(), environment) {
        Ok(data_type) => {
          if !types.contains(&data_type.value) {
            types.push(data_type.value);
          }
        }
        Err(error) => {
          return Err(error);
        }
      }
    }

    return Ok(Token::from_value(&format!("{}[]", types.join(" | ")), 0, 0));
  }

  // Array index

  // Boolean
  if expression.clone().get_boolean().is_some() {
    return Ok(Token::from_value("boolean", 0, 0));
  }

  // Call
  if let Some(call) = expression.clone().get_call() {
    // Check if the call token exists in the environment store.
    if environment.store.get_type(call.token.value.clone()).is_none() {
      return Err(Error::from_token(
        format!("`{}` identifier not found.", call.token.value.clone()),
        call.token.clone(),
      ));
    }

    // Get the function arguments from the environment store.
    if let Some(arguments) = environment.store.get_function_arguments(call.token.value.clone()) {
      // Get the min and max arguments.
      let mut min_arguments: usize = 0;
      let mut max_arguments: usize = 0;

      for argument in arguments.clone().iter() {
        // Get the argument expression.
        let argument = argument.clone().get_argument().unwrap();
        max_arguments += 1;

        // Check if the argument has a default value.
        if argument.value.is_none() {
          min_arguments += 1;
        }
      }

      // Check if the call has the minimun arguments.
      if call.arguments.len() < min_arguments {
        return Err(Error::from_token(
          format!(
            "expected minimum {} arguments, got {} instead.",
            min_arguments,
            call.arguments.len()
          ),
          call.token.clone(),
        ));
      }

      if call.arguments.len() > max_arguments {
        return Err(Error::from_token(
          format!(
            "expected maximum {} arguments, got {} instead.",
            max_arguments,
            call.arguments.len()
          ),
          call.token.clone(),
        ));
      }

      // Parse call arguments types.
      let mut call_arguments_types: Vec<Token> = Vec::new();

      for argument in call.arguments.clone().iter() {
        match check_expression(argument.clone(), environment) {
          Ok(data_type) => {
            call_arguments_types.push(data_type);
          }
          Err(error) => {
            return Err(error);
          }
        }
      }

      // Compare arguments types.
      let mut index: usize = 0;

      for argument in call_arguments_types.clone().iter() {
        let call_token = call.arguments[index].clone().token();
        let function_argument = arguments[index].clone().get_argument().unwrap().data_type;

        if function_argument.token.clone().is_identifier() {
          if let Some(interface) = environment.store.get_interface(function_argument.value.clone()) {
            if !equal_type_and_interface(argument.clone(), interface.clone()) {
              return Err(Error::from_token(
                format!(
                  "`{}` not satisfied the `{}` interface.",
                  call_token.value.clone(),
                  function_argument.value.clone()
                ),
                call_token.clone(),
              ));
            }
          }
        } else if function_argument.token.clone().get_type().is_some() {
          if !check_types(function_argument.clone(), argument.clone(), false) {
            return Err(Error::from_token(
              format!(
                "`{}` not satisfied the `{}` data type.",
                call_token.value.clone(),
                function_argument.value.clone()
              ),
              call_token.clone(),
            ));
          }
        }

        index += 1;
      }

      // Get the function return data type.
      if let Some(data_type) = environment.store.get_type(call.token.value.clone()) {
        // Check if the function token is a valid data type.
        if let Some(data_type) = data_type.token.clone().get_type() {
          // Check if the data type is a function.
          if let Some(function_type) = data_type.clone().get_function() {
            return Ok(function_type.data_type.clone());
          }

          return Err(Error::from_token(
            format!("`{}` the data type is not a function.", call.token.value.clone()),
            call.token.clone(),
          ));
        }

        return Err(Error::from_token(
          format!("`{}` the token is not a valid data type.", call.token.value.clone()),
          call.token.clone(),
        ));
      }

      return Err(Error::from_token(
        format!("`{}` does not have a data type token.", call.token.value.clone()),
        call.token.clone(),
      ));
    } else {
      return Err(Error::from_token(
        format!("`{}` is not a function.", call.token.value.clone()),
        call.token.clone(),
      ));
    }
  }

  // HashMap
  if let Some(hashmap) = expression.clone().get_hashmap().as_mut() {
    let mut items: Vec<String> = Vec::new();

    for (key, value) in hashmap.items.clone().iter() {
      let mut new_item = key.clone();

      // Parse item value data type.
      match check_expression(value.clone(), environment) {
        Ok(token) => {
          // Check if the token is a valid type.
          if token.token.clone().get_type().is_some() {
            new_item.push_str(": ");
            new_item.push_str(&token.value);
          } else {
            return Err(Error::from_token(
              format!(
                "`{}` is not a valid data type.",
                value.clone().token().value
              ),
              value.clone().token(),
            ));
          }
        }
        Err(error) => {
          return Err(error);
        }
      }

      items.push(new_item);
    }

    let mut value = String::from("{ ");

    value.push_str(&items.join(", "));
    value.push_str(" }");

    return Ok(Token::from_value(&value, 0, 0));
  }

  // Identifier
  if let Some(identifier) = expression.clone().get_identifier() {
    return match environment.store.get_type(identifier.token.value.clone()) {
      Some(data_type) => Ok(data_type),
      None => Err(Error::from_token(
        format!("`{}` identifier not found.", identifier.token.value.clone()),
        identifier.token.clone(),
      )),
    };
  }

  // Infix
  if let Some(infix) = expression.clone().get_infix() {
    // Get the left expression data type.
    let left_type;

    match check_expression(infix.left.clone(), environment) {
      Ok(data_type) => {
        left_type = data_type;
      }
      Err(error) => {
        return Err(error);
      }
    }

    // Create a new environment.
    let mut right_environment = environment.clone();

    right_environment.store = Store::from_store(environment.store.clone());

    // Check if is a method.
    if infix.clone().is_method() {
      // Get the left token type.
      if let Some(left_type) = left_type.token.clone().get_type() {
        // Get the left hashmap.
        if let Some(left_hashmap) = left_type.clone().get_hashmap() {
          for (key, value) in left_hashmap.items {
            // Add he hashmap items to the environment.
            right_environment.store.set_type(key, value);
          }
        }
      }
    }

    // Get the right expression data type.
    let right_type;

    match check_expression(infix.right.clone(), &mut right_environment) {
      Ok(data_type) => {
        right_type = data_type;
      }
      Err(error) => {
        return Err(error);
      }
    }

    // Check if is a method.
    if infix.clone().is_method() {
      return Ok(right_type);
    }
    // Check if is an infix.
    else if infix.clone().is_infix() {
      let left_tt = left_type.token.clone().get_type().unwrap();
      let right_tt = right_type.token.clone().get_type().unwrap();

      // Parse `-`, `/`, `*`, `^`, `**` and `%` with numbers.
      if infix.token.token.clone().expect_sign(Signs::MINUS) ||
        infix.token.token.clone().expect_sign(Signs::DIVIDE) ||
        infix.token.token.clone().expect_sign(Signs::MULTIPLY) ||
        infix.token.token.clone().expect_sign(Signs::EMPOWERMENT) ||
        infix.token.token.clone().expect_sign(Signs::CARER) ||
        infix.token.token.clone().expect_sign(Signs::MODULE) {
        if left_tt != Types::NUMBER || right_tt != Types::NUMBER {
          return Err(Error::from_token(
            String::from("only can do this with numbers."),
            infix.token.clone(),
          ));
        }

        return Ok(left_type);
      }
      // Parse `<` `<=`, `>` and `>=` with numbers.
      else if infix.token.token.clone().expect_sign(Signs::LESSTHAN) ||
        infix.token.token.clone().expect_sign(Signs::LESSOREQUALTHAN) ||
        infix.token.token.clone().expect_sign(Signs::GREATERTHAN) ||
        infix.token.token.clone().expect_sign(Signs::GREATEROREQUALTHAN) {
        if left_tt != Types::NUMBER || right_tt != Types::NUMBER {
          return Err(Error::from_token(
            String::from("only can do this with numbers."),
            infix.token.clone(),
          ));
        }

        return Ok(Token::from_value("boolean", 0, 0));
      }
      // Parse `+` with numbers and strings.
      else if infix.token.token.clone().expect_sign(Signs::PLUS) {
        if left_tt == Types::NUMBER && right_tt == Types::NUMBER {
          return Ok(left_type);
        } else if left_tt == Types::STRING {
          return Ok(left_type);
        } else if right_tt == Types::STRING {
          return Ok(right_type);
        }

        return Err(Error::from_token(
          format!(
            "can not concat `{}` with `{}`.",
            left_type.value,
            right_type.value
          ),
          infix.token.clone(),
        ));
      }
      // Parse `==`, `!=`, `===` and `!==`.
      else if infix.token.token.clone().expect_sign(Signs::EQUAL) ||
        infix.token.token.clone().expect_sign(Signs::NOTEQUAL) ||
        infix.token.token.clone().expect_sign(Signs::EQUALTYPE) ||
        infix.token.token.clone().expect_sign(Signs::NOTEQUALTYPE) {
        return Ok(Token::from_value("boolean", 0, 0));
      }
      // Parse `||`.
      else if infix.token.token.clone().expect_sign(Signs::OR) {
        // NOTE THIS CAN GET ERROR SOMETIMES.
        return Ok(right_type);
      }
      // Parse `&&`.
      else if infix.token.token.clone().expect_sign(Signs::AND) {
        if left_tt != Types::BOOLEAN || right_tt != Types::BOOLEAN {
          return Err(Error::from_token(
            String::from("only can do this with booleans."),
            infix.token.clone(),
          ));
        }

        return Ok(left_type);
      }
    }
  }

  // Null
  if expression.clone().get_null().is_some() {
    return Ok(Token::from_value("null", 0, 0));
  }

  // Number
  if expression.clone().get_number().is_some() {
    return Ok(Token::from_value("number", 0, 0));
  }

  // Prefix
  if let Some(prefix) = expression.clone().get_prefix() {
    let right_type;

    match check_expression(prefix.right, environment) {
      Ok(data_type) => {
        right_type = data_type;
      }
      Err(error) => {
        return Err(error);
      }
    }

    let right_tt = right_type.token.clone().get_type().unwrap();

    if prefix.token.token.clone().expect_sign(Signs::MINUS) {
      if right_tt != Types::NUMBER {
        return Err(Error::from_token(
          String::from("only can convert to negative a number."),
          prefix.token.clone(),
        ));
      }

      return Ok(right_type);
    } else if prefix.token.token.clone().expect_sign(Signs::NOT) {
      if right_tt != Types::BOOLEAN && right_tt != Types::NULL {
        return Err(Error::from_token(
          String::from("can not be parsed to a boolean."),
          prefix.token.clone(),
        ));
      }

      return Ok(Token::from_value("boolean", 0, 0));
    }

    return Err(Error::from_token(
      String::from("can not be recognized this expression."),
      prefix.token.clone(),
    ));
  }

  // String
  if expression.clone().get_string().is_some() {
    return Ok(Token::from_value("string", 0, 0));
  }

  // Default
  Err(Error::from_token(
    String::from("unknown type expression."),
    expression.clone().token(),
  ))
}

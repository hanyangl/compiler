use crate::{
  Environment,
  Store,
};

use sflyn_parser::{
  Error,
  Statements,
  tokens::{
    Token,
    Types,
  },
};

use super::{
  check_expression,
  check_types,
  function_arguments_to_string,
};

pub fn check_statement(
  statement: Box<Statements>,
  environment: &mut Environment,
) -> Result<Token, Error> {
  // Block
  if let Some(block) = statement.clone().get_block() {
    let mut values = String::new();

    for stmt in block.statements.iter() {
      // Get the data type token for the current statement.
      match check_statement(stmt.clone(), environment) {
        Ok(data_type) => {
          // Check if the current statement is a `return` or an `if else`.
          if stmt.clone().get_return().is_some() || stmt.clone().get_if_else().is_some() {
            // Check if the values len is greater than 0.
            if values.len() > 0 {
              // Add a bit or to the values.
              values.push_str(" | ");
            }

            // Add the new value data type to the values.
            values.push_str(data_type.value.as_str());
          }
        }
        Err(error) => {
          return Err(error);
        }
      }
    }

    // Return the token.
    return Ok(Token::from_value(values.as_str(), 0, 0));
  }

  // Export

  // Expression
  if let Some(expression) = statement.clone().get_expression() {
    return check_expression(expression.expression, environment);
  }

  // Function
  if let Some(function) = statement.clone().get_function() {
    // Check if the function name is already in use.
    if environment.store.get_type(function.name.value.clone()).is_some() {
      return Err(Error::from_token(
        format!("`{}` is already in use.", function.name.value.clone()),
        function.name.clone(),
      ));
    }

    // Create a new closed environment.
    let mut function_environment = environment.clone();

    function_environment.store = Store::from_store(environment.store.clone());

    let arguments: Vec<String>;

    match function_arguments_to_string(function.arguments.clone(), environment, &mut function_environment) {
      Ok(args) => {
        arguments = args;
      },
      Err(error) => {
        return Err(error);
      },
    }

    // Get the function data type.
    let mut data_type = function.data_type.clone();

    // Get the data type token from the function body.
    match check_statement(function.body, &mut function_environment) {
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

    let token = Token::from_value(value.as_str(), 0, 0);

    // Add the function data type token to the environment store.
    environment.store.set_type(function.name.value.clone(), token.clone());

    // Add the function arguments to the environment store.
    environment.store.set_function_arguments(function.name.value.clone(), function.arguments.clone());

    // Return the token.
    return Ok(token);
  }

  // If else
  if let Some(if_else) = statement.clone().get_if_else() {
    let mut tokens: Vec<Token> = Vec::new();

    for condition in if_else.conditions.iter() {
      match check_expression(condition.condition.clone(), environment) {
        Ok(data_type) => {
          if let Some(data_type) = data_type.token.get_type() {
            if data_type != Types::BOOLEAN {
              return Err(Error::from_token(
                String::from("the condition is not a `boolean`."),
                condition.token.clone(),
              ));
            }
          } else {
            return Err(Error::from_token(
              String::from("the condition is not a valid data type."),
              condition.token.clone(),
            ));
          }
        },
        Err(error) => {
          return Err(error);
        },
      }

      match check_statement(condition.consequence.clone(), environment) {
        Ok(data_type) => {
          if data_type.token.clone().get_type().is_some() {
            tokens.push(data_type);
          }
        },
        Err(error) => {
          return Err(error);
        },
      }
    }

    if let Some(alternative) = if_else.alternative {
      match check_statement(alternative, environment) {
        Ok(data_type) => {
          if data_type.token.clone().get_type().is_some() {
            tokens.push(data_type);
          }
        },
        Err(error) => {
          return Err(error);
        },
      }
    }

    return Ok(Token::from_value(tokens.iter().map(|x| x.value.clone()).collect::<Vec<String>>().join(" | ").as_str(), 0, 0));
  }

  // Import

  // Interface
  if let Some(interface) = statement.clone().get_interface() {
    // Check if the interface name is already in use.
    if environment.store.get_type(interface.name.value.clone()).is_some() {
      return Err(Error::from_token(
        format!("`{}` is already in use.", interface.name.value.clone()),
        interface.name.clone(),
      ));
    }

    let mut methods: Vec<String> = Vec::new();

    for method in interface.methods {
      methods.push(format!(
        "{}: {}",
        method.token.value,
        method.data_type.value,
      ));
    }

    let mut value = String::from("{ ");

    value.push_str(methods.join(", ").as_str());
    value.push_str(" }");

    // Get the token type for the value.
    let token = Token::from_value(value.as_str(), 0, 0);

    // Check if the token is a valid type.
    if token.token.clone().get_type().is_none() {
      return Err(Error::from_token(
        String::from("is not a valid interface type."),
        interface.name.clone(),
      ));
    }

    // Add the interface data type to the environment store.
    environment.store.set_type(interface.name.value.clone(), token.clone());
    environment.store.set_interface(interface.name.value.clone(), token.clone());

    // Return the token.
    return Ok(token);
  }

  // Return
  if let Some(return_s) = statement.clone().get_return() {
    if let Some(value) = return_s.value {
      return check_expression(value, environment);
    }

    return Ok(Token::from_value("void", 0, 0));
  }

  // Variable
  if let Some(variable) = statement.clone().get_variable() {
    // Check if the variable name is already in use.
    if environment.store.get_type(variable.name.value.clone()).is_some() {
      return Err(Error::from_token(
        format!("`{}` is already in use.", variable.name.value.clone()),
        variable.name.clone(),
      ));
    }

    // Get the variable data type.
    let mut data_type = variable.data_type.clone();

    // Get the variable value.
    if let Some(value) = variable.value.clone() {
      match check_expression(value, environment) {
        Ok(token) => {
          // Check if the token is a valid data type.
          if token.token.clone().get_type().is_some() {
            // Set the new data type.
            if data_type.line == 0 {
              data_type = token.clone();
            }
            // Compare both data types.
            else if !check_types(data_type.clone(), token, false) {
              return Err(Error::from_token(
                format!(
                  "the value does not satisfied the `{}` data type.",
                  data_type.value
                ),
                variable.token.clone(),
              ));
            }
          }
        }
        Err(error) => {
          return Err(error);
        }
      }
    }

    // Add the variable data type to the environment store.
    environment.store.set_type(variable.name.value.clone(), data_type.clone());

    // Return the token.
    return Ok(data_type);
  }

  // Default
  Err(Error::from_token(
    String::from("unknown type statement."),
    statement.clone().token(),
  ))
}

use crate::{
  Environment,
  Store,
  typechecker::{
    check_expression,
    function_arguments_to_string,
    TTypes,
  },
};

use sflyn_parser::{
  Error,
  Statement,
  Statements,
  tokens::{
    Token,
    Types,
  },
};

pub fn check_statement(
  statement: &Box<Statements>,
  environment: &mut Environment,
) -> Result<TTypes, Error> {
  // Block
  if let Some(block) = statement.get_block() {
    let mut return_token: Option<TTypes> = None;

    for statement in block.get_statements().iter() {
      // Get the token for the current statement.
      match check_statement(statement, environment) {
        Ok(token) => {
          if statement.get_return().is_some() || statement.get_if_else().is_some() {
            if let Some(rtoken) = return_token.clone() {
              if rtoken.get_type() == token.get_type() {
                continue;
              } else {
                return Err(Error::from_token(
                  format!("`{}` not satisfied the `{}` data type.", token.get_token().value, rtoken.get_token().value),
                  token.get_token(),
                ));
              }
            }

            return_token = Some(token);
          }
        },
        Err(error) => {
          return Err(error);
        },
      }
    }

    if let Some(token) = return_token {
      return Ok(token);
    }

    return Ok(TTypes::new_type(Types::VOID, String::from("void"), block.get_token()));
  }

  // Export

  // Expression
  if let Some(expression) = statement.get_expression() {
    return check_expression(&expression.get_expression(), environment);
  }

  // Function
  if let Some(function) = statement.get_function() {
    // Check if the variable name is already in use.
    if environment.store.get_type(&function.get_name().value).is_some() {
      return Err(Error::from_token(
        format!("`{}` is already in use.", function.get_name().value),
        function.get_name(),
      ));
    }

    // Create a new closed environment.
    let mut function_environment: Environment = environment.clone();

    function_environment.store = Store::from_store(environment.store.clone());

    let arguments: Vec<String>;

    match function_arguments_to_string(function.get_arguments(), environment, &mut function_environment) {
      Ok(args) => {
        arguments = args;
      },
      Err(error) => {
        return Err(error);
      },
    }

    // Get the function data type.
    let data_type: Token = function.get_type();

    // Get the ttypes from the function body.
    match check_statement(&function.get_body(), &mut function_environment) {
      Ok(token) => {
        if let Some(ttoken) = data_type.token.get_type() {
          if ttoken != token.get_type() {
            return Err(Error::from_token(
              format!("`{}` not satisfied the `{}` data type.", token.get_token().value, data_type.value),
              token.get_token(),
            ));
          }
        } else {
          return Err(Error::from_token(
            format!("`{}` is not a valid data type.", data_type.value),
            data_type,
          ));
        }
      },
      Err(error) => {
        return Err(error);
      },
    }

    let value = format!("({}) => {}", arguments.join(", "), data_type.value);
    let token = Token::from_value(value.as_str(), 0, 0);

    if token.token.get_type().is_none() {
      return Err(Error::from_token(
        String::from("is not a valid function."),
        function.get_token(),
      ));
    }

    let ttype = TTypes::new_function(
      token.token.get_type().unwrap(),
      token.value,
      function.get_token(),
      function.get_arguments(),
    );

    environment.store.set_type(function.get_name().value, ttype.clone());

    return Ok(ttype);
  }

  // If else

  // Import

  // Interface

  // Return
  if let Some(return_s) = statement.get_return() {
    if let Some(value) = return_s.get_value() {
      return check_expression(&value, environment);
    }

    return Ok(TTypes::new_type(Types::VOID, String::from("void"), return_s.get_token()));
  }

  // Variable
  if let Some(variable) = statement.get_variable() {
    // Check if the variable name is already in use.
    if environment.store.get_type(&variable.get_name().value).is_some() {
      return Err(Error::from_token(
        format!("`{}` is already in use.", variable.get_name().value),
        variable.get_name(),
      ));
    }

    let data_type = variable.get_type();

    if let Some(value) = variable.get_value() {
      match check_expression(&value, environment) {
        Ok(token) => {
          if data_type.value == "any" {
            environment.store.set_type(variable.get_name().value, token.clone());
            return Ok(token);
          } else if let Some(data_type) = data_type.token.get_type() {
            if data_type == token.get_type() {
              environment.store.set_type(variable.get_name().value, token.clone());
              return Ok(token);
            }
          }
        },
        Err(error) => {
          return Err(error);
        },
      }
    }
  }

  // Default
  Err(Error::from_token(
    String::from("unknown statement."),
    statement.token(),
  ))
}

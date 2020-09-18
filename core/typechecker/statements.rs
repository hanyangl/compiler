mod block;
mod for_s;
mod function;
mod if_else;
mod import;
mod interface;
mod variable;

use crate::{
  Environment,
  typechecker::{
    check_expression,
    TTypes,
  },
};

use sflyn_parser::{
  Error,
  Statement,
  Statements,
  tokens::Types,
};

pub fn check_statement(
  statement: &Box<Statements>,
  environment: &mut Environment,
) -> Result<TTypes, Error> {
  // Block
  if let Some(block_stmt) = statement.get_block() {
    return block::check(&block_stmt, environment);
  }

  // Export
  if let Some(export) = statement.get_export() {
    return check_statement(&export.get_value(), environment);
  }

  // Expression
  if let Some(expression) = statement.get_expression() {
    return check_expression(&expression.get_expression(), environment);
  }

  // For
  if let Some(for_stmt) = statement.get_for() {
    return for_s::check(&for_stmt, environment);
  }

  // Function
  if let Some(function_stmt) = statement.get_function() {
    return function::check(&function_stmt, environment);
  }

  // If else
  if let Some(if_else_stmt) = statement.get_if_else() {
    return if_else::check(&if_else_stmt, environment);
  }

  // Import
  if let Some(import_stmt) = statement.get_import() {
    return import::check(&import_stmt, environment);
  }

  // Interface
  if let Some(interface_stmt) = statement.get_interface() {
    return interface::check(&interface_stmt, environment);
  }

  // Return
  if let Some(return_stmt) = statement.get_return() {
    if let Some(value) = return_stmt.get_value() {
      return check_expression(&value, environment);
    }

    return Ok(TTypes::new_type(Types::VOID, String::from("void"), return_stmt.get_token()));
  }

  // Variable
  if let Some(variable_stmt) = statement.get_variable() {
    return variable::check(&variable_stmt, environment);
  }

  // Default
  Err(Error::from_token(
    String::from("unknown statement."),
    statement.token(),
  ))
}

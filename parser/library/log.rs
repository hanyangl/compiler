/// This file was replaced by /std/log.sf

use crate::expressions::{
  HashMap,
  HashMapItem,
  AnonymousFunction,
  Expressions,
  Argument,
  Identifier,
  Expression,
  Infix,
  StringE,
};

use crate::statements::{
  Block,
  Statement,
  Statements,
  Show,
  Variable,
};

use crate::tokens::{Token, Keywords};

/// Get the write method.
///
/// ```sf
/// log->write('Sflyn');
/// ```
fn get_write_function() -> HashMapItem {
  let mut write_function: AnonymousFunction = Expression::from_token(Token::from_value(String::from("("), 0, 0));

  // Add the message argument to the function.
  // `log->write(message: string)`
  write_function.arguments.push(Box::new(Expressions::ARGUMENT(
    Argument {
      token: Token::new(Box::new(Tokens::IDENTIFIER), String::from("message"), 0, 0),
      data_type: Token::from_value(String::from("string"), 0, 0),
      value: None,
    }
  )));

  // Create a new block statement.
  let mut write_function_body: Block = Statement::from_token(Token::from_value(String::from("{"), 0, 0));

  // Add the show statement to the block statement.
  write_function_body.statements.push(Box::new(Statements::SHOW(
    Show {
      token: Token::new(Keywords::new(Keywords::SHOW)), String::from("show"), 0, 0),
      value: Identifier::new_box_from_token(
        Token::new(Box::new(Tokens::IDENTIFIER), String::from("message"), 0, 0)
      ),
    }
  )));

  // Set the block statement as the function body.
  write_function.body = Box::new(Statements::BLOCK(write_function_body));

  // Return the hashmap item.
  HashMapItem {
    key: String::from("write"),
    data_type: Token::from_value(String::from("void"), 0, 0),
    value: Box::new(Expressions::ANONYMOUSFUNCTION(write_function)),
  }
}

/// Get the error method.
///
/// ## Examples
/// ```sf
/// log->error('Sflyn');
/// ```
fn get_error_function() -> HashMapItem {
  let mut error_function: AnonymousFunction = Expression::from_token(Token::from_value(String::from("("), 0, 0));

  // Add the message argument to the function.
  // `log->error(message: string)`
  error_function.arguments.push(Box::new(Expressions::ARGUMENT(
    Argument {
      token: Token::new(Box::new(Tokens::IDENTIFIER), String::from("message"), 0, 0),
      data_type: Token::from_value(String::from("string"), 0, 0),
      value: None,
    }
  )));

  // Create a new block statement.
  let mut error_function_body: Block = Statement::from_token(Token::from_value(String::from("{"), 0, 0));

  // Create a new infix expression to concat strings.
  let mut show_value: Infix = Expression::from_token(Token::from_value(String::from("+"), 0, 0));

  // Add the left expression to the infix.
  // `'[ERROR] '`
  show_value.left = Some(StringE::new_box_from_token(
    Token::new(Box::new(Tokens::STRING), String::from("'[ERROR] '"), 0, 0),
  ));

  // Add the right expression to the infix.
  // `message`
  show_value.right = Some(Identifier::new_box_from_token(
    Token::new(Box::new(Tokens::IDENTIFIER), String::from("message"), 0, 0),
  ));

  // Add the show statement to the block statement with the infix expression as value.
  error_function_body.statements.push(Box::new(Statements::SHOW(
    Show {
      token: Token::new(Box::new(Tokens::SHOW), String::from("show"), 0, 0),
      value: Box::new(Expressions::INFIX(show_value)),
    }
  )));

  // Set the block statement to the function body.
  error_function.body = Box::new(Statements::BLOCK(error_function_body));

  // Return the hashmap item.
  HashMapItem {
    key: String::from("error"),
    data_type: Token::from_value(String::from("void"), 0, 0),
    value: Box::new(Expressions::ANONYMOUSFUNCTION(error_function)),
  }
}

/// Get the warning method.
///
/// ## Examples
/// ```sf
/// log->warning('Sflyn');
/// ```
fn get_warning_function() -> HashMapItem {
  let mut warning_function: AnonymousFunction = Expression::from_token(Token::from_value(String::from("("), 0, 0));

  // Add the message argument to the function.
  // `log->warning(message: string)`
  warning_function.arguments.push(Box::new(Expressions::ARGUMENT(
    Argument {
      token: Token::new(Box::new(Tokens::IDENTIFIER), String::from("message"), 0, 0),
      data_type: Token::from_value(String::from("string"), 0, 0),
      value: None,
    }
  )));

  // Create a new block statement.
  let mut warning_function_body: Block = Statement::from_token(Token::from_value(String::from("{"), 0, 0));

  // Create a new infix expression to concat strings.
  let mut show_value: Infix = Expression::from_token(Token::from_value(String::from("+"), 0, 0));

  // Add the left expression to the infix.
  // `'[WARNING] '`
  show_value.left = Some(StringE::new_box_from_token(
    Token::new(Box::new(Tokens::STRING), String::from("'[WARNING] '"), 0, 0),
  ));

  // Add the right expression to the infix.
  // `message`
  show_value.right = Some(Identifier::new_box_from_token(
    Token::new(Box::new(Tokens::IDENTIFIER), String::from("message"), 0, 0),
  ));

  // Add the show statement to the block statement with the infix expression as value.
  warning_function_body.statements.push(Box::new(Statements::SHOW(
    Show {
      token: Token::new(Box::new(Tokens::SHOW), String::from("show"), 0, 0),
      value: Box::new(Expressions::INFIX(show_value)),
    }
  )));

  // Set the block statement to the function body.
  warning_function.body = Box::new(Statements::BLOCK(warning_function_body));

  // Return the hashmap item.
  HashMapItem {
    key: String::from("warning"),
    data_type: Token::from_value(String::from("void"), 0, 0),
    value: Box::new(Expressions::ANONYMOUSFUNCTION(warning_function)),
  }
}

/// Get the log variable value in a hashmap.
fn get_log_hashmap() -> HashMap {
  let mut log_hashmap: HashMap = Expression::from_token(Token::from_value(String::from("{"), 0, 0));

  // Add the write method. `log->write('Sflyn')`
  log_hashmap.data.push(get_write_function());

  // Add the error method. `log->error('Sflyn')`
  log_hashmap.data.push(get_error_function());

  // Add the warning method. `log->warning('Sflyn')`
  log_hashmap.data.push(get_warning_function());

  // Return the hashmap.
  log_hashmap
}

/// Get the log variable statement. This variable is used by the
/// Sflyn standar library.
/// 
/// ## Examples
/// ```sf
/// log->write('Sflyn');
/// log->error('Sflyn');
/// log->warning('Sflyn');
/// ```
pub fn get_main() -> Box<Statements> {
  let mut variable: Variable = Statement::from_token(Token::from_value(String::from("const"), 0, 0));

  // Set the variable name to `log`.
  variable.name = Identifier::new_box_from_token(
    Token::new(Box::new(Tokens::IDENTIFIER), String::from("log"), 0, 0),
  );

  // Set the variable type to `hashmap`.
  variable.data_type = Token::from_value(String::from("hashmap"), 0, 0);

  // Set the log hashmap as the variable value.
  variable.value = Some(Box::new(Expressions::HASHMAP(get_log_hashmap())));

  // Return the variable statement.
  Box::new(Statements::VARIABLE(variable))
}

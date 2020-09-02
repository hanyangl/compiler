mod expressions;
mod statements;
mod types;

pub use expressions::check_expression;
pub use statements::check_statement;
pub use types::*;

use sflyn_parser::{
  Argument,
  Expressions,
  File,
  tokens::{
    Token,
    Tokens,
  },
};

use super::{
  Environment,
  error::show_error,
};

pub fn run(
  file: &mut File,
  environment: &mut Environment,
) -> Result<(), ()> {
  // Add builtint functions.
  environment.store.set_type(String::from("print"), Token::from_value("(message: string | number | boolean) => void", 0, 0));
  environment.store.set_function_arguments(
    String::from("print"), [
      Box::new(Expressions::ARGUMENT(Argument {
        token: Token::new(Box::new(Tokens::IDENTIFIER), String::from("message"), 0, 0),
        data_type: Token::from_value("string | number | boolean", 0, 0),
        value: None,
      }))
    ].to_vec()
  );

  // Parse file statements.
  for statement in file.statements.iter_mut() {
    if let Err(error) = check_statement(statement, environment) {
      show_error(file.clone(), error);
      return Err(());
    }
  }

  Ok(())
}

mod arguments;
mod expressions;
mod statements;
mod types;

pub use arguments::function_arguments_to_string;
pub use expressions::check_expression;
pub use statements::check_statement;
pub use types::*;

use sflyn_parser::File;

use super::{
  Environment,
  error::show_error,
};

pub fn run(
  file: &mut File,
  environment: &mut Environment,
) -> Result<(), ()> {
  // Add stdlib.
  if environment.stdlibs.len() > 0 {
    for (name, file) in environment.stdlibs.clone().iter() {
      if file.statements.len() == 0 {
        println!("`{}` library is an empty file.", name);
        return Err(());
      }

      for statement in file.statements.iter() {
        if let Err(error) = check_statement(statement.clone(), environment) {
          show_error(file.clone(), error);
          return Err(());
        }
      }
    }
  }

  // Parse file statements.
  for statement in file.statements.iter() {
    if let Err(error) = check_statement(statement.clone(), environment) {
      show_error(file.clone(), error);
      return Err(());
    }
  }

  Ok(())
}

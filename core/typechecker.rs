mod expressions;
mod statements;
mod types;
mod utils;

pub use expressions::*;
pub use statements::*;
pub use types::*;
pub use utils::*;

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
        if let Err(error) = check_statement(statement, environment) {
          show_error(file.clone(), error);
          return Err(());
        }
      }
    }
  }

  // Parse file statements.
  for statement in file.statements.iter() {
    if let Err(error) = check_statement(statement, environment) {
      show_error(file.clone(), error);
      return Err(());
    }
  }

  Ok(())
}

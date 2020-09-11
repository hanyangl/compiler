use sflyn_parser::File;

use super::{
  Environment,
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

      for _statement in file.statements.iter() {
        /*if let Err(error) = check_statement(statement.clone(), environment) {
          show_error(file.clone(), error);
          return Err(());
        }*/
      }
    }
  }

  // Parse file statements.
  for _statement in file.statements.iter() {
    /*if let Err(error) = check_statement(statement.clone(), environment) {
      show_error(file.clone(), error);
      return Err(());
    }*/
  }

  Ok(())
}

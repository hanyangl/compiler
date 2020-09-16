pub mod builtins;
mod expressions;
mod objects;
mod statements;

pub use expressions::*;
pub use objects::*;
pub use statements::*;

use sflyn_parser::File;

use super::Environment;

pub fn run(
  file: File,
  environment: &mut Environment,
  with_stdlib: bool,
) {
  // Evaluate stdlib.
  if environment.stdlibs.len() > 0 && with_stdlib {
    for (name, file) in environment.stdlibs.clone().iter() {
      if name == "builtins" || file.statements.len() == 0 {
        continue;
      }

      for statement in file.statements.iter() {
        // Evaluate the statement.
        if let Some(object) = evaluate_statement(statement, environment) {
          // Check if the object is an error.
          if let Some(error) = object.get_error() {
            println!("\n{}", error.string(file.clone()));
          }
        }
      }
    }
  }

  // Evaluate file statements.
  for statement in file.statements.iter() {
    // Evaluate the statement.
    if let Some(object) = evaluate_statement(statement, environment) {
      // Check if the object is an error.
      if let Some(error) = object.get_error() {
        println!("\n{}", error.string(file.clone()));
      }
    }
  }
}

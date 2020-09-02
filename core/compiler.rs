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
) {
  for statement in file.statements.clone() {
    // Evaluate the statement.
    if let Some(object) = evaluate_statement(statement, environment) {
      // Check if the object is an error.
      if let Some(error) = object.clone().get_error() {
        println!("\n{}", error.string(file.clone()));
      }
    }
  }
}

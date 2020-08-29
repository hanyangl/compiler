use super::Environment;
use super::statements::evaluate;

use sflyn_parser::statements::Statements;

pub fn program(statements: Vec<Box<Statements>>, environment: &mut Environment) {
  if statements.len() > 0 {
    for statement in statements {
      // Evaluate statement.
      match evaluate(statement.clone(), environment) {
        Some(object) => {
          // Check if the object is an error or a print object.
          if object.clone().is_error() || object.clone().is_print() {
            println!("{}", object.string());
          }
        },
        None => {},
      }
    }
  }
}

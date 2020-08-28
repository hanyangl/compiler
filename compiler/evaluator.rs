use super::Environment;
use super::statements::evaluate;

use sflyn_parser::statements::Statements;

pub fn program(
  statements: Vec<Box<Statements>>,
  environment: &mut Environment,
) {
  if statements.len() > 0 {
    for statement in statements {
      // Evaluate statement.
      match evaluate(statement.clone(), environment) {
        Some(object) => {
          // Check if the object is an error.
          if object.clone().is_error() {
            println!("{}\n", object.string());
            break;
          }
        },
        None => {},
      }
    }
  }

  // Only for test!
  for (key, value) in environment.store.clone() {
    println!("{} = {}", key, value.string());
  }
}

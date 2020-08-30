use super::Environment;
use super::statements::evaluate;

use sflyn_parser::statements::Statements;

pub fn program(
  file_name: String,
  statements: Vec<Box<Statements>>,
  environment: &mut Environment,
) {
  if statements.len() > 0 {
    for statement in statements {
      // Evaluate statement.
      match evaluate(file_name.clone(), statement.clone(), environment) {
        Some(object) => {
          // Check if the object is an error or a print object.
          if object.clone().is_error() || object.clone().is_print() {
            let mut return_value = object.string();

            return_value = str::replace(return_value.as_str(), "\\n", "\n");
            return_value = str::replace(return_value.as_str(), "\\r", "\r");
            return_value = str::replace(return_value.as_str(), "\\t", "\t");

            println!("{}", return_value);
          }
        },
        None => {},
      }
    }
  }
}

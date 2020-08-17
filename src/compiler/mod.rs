pub mod block;
pub mod call;
pub mod environment;
pub mod expression;
pub mod if_else;
pub mod infix;
pub mod integer;
pub mod prefix;
pub mod statement;
pub mod string;

use crate::objects::{Objects, error::is_error};
use crate::statements::Statements;

pub fn evaluate(statements: Vec<Box<Statements>>, env: &mut environment::Environment) -> Option<Box<Objects>> {
  let mut result: Option<Box<Objects>> = None;

  for stmt in statements.iter() {
    result = statement::evaluate(stmt.clone(), env);

    match result.clone() {
      Some(obj) => {
        // Get return object.
        match obj.clone().get_return() {
          Some(return_obj) => {
            return Some(return_obj.value);
          },

          None => {
            // Get error object.
            if is_error(obj.clone()) {
              return Some(obj);
            }
          }
        }
      },
      None => {},
    }
  }

  result
}

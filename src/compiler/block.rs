use crate::objects::{Objects, ObjectType};
use crate::statements::block::Block;

use super::{environment::Environment, statement::evaluate as evaluate_statement};

pub fn evaluate(block: Block, env: &mut Environment) -> Option<Box<Objects>> {
  let mut result: Option<Box<Objects>> = None;

  for stmt in block.statements.iter() {
    // Evaluate current statement.
    result = evaluate_statement(stmt.clone(), env);

    match result.clone() {
      Some(obj) => {
        // Get object type.
        let object_type = obj.clone().object_type();

        // Comprobate if it is a return or error object.
        if object_type == ObjectType::RETURN || object_type == ObjectType::ERROR {
          return Some(obj);
        }
      },
      None => {}
    }
  }

  result
}

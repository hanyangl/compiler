use crate::objects::{Objects, error::is_error, return_o::Return};
use crate::statements::Statements;

use super::{environment::Environment, block, expression};

pub fn evaluate(stmt: Box<Statements>, env: &mut Environment) -> Option<Box<Objects>> {
  match stmt.clone().as_ref() {
    // Block
    Statements::BLOCK(block_stmt) => block::evaluate(block_stmt.clone(), env),

    // Expression
    Statements::EXPRESSION(exp_stmt) => match exp_stmt.clone().expression {
      Some(exp) => expression::evaluate(exp, env),
      None => None,
    },

    // Variables
    Statements::VARIABLE(var_stmt) => match var_stmt.value.clone() {
      Some(value_exp) => match expression::evaluate(value_exp.clone(), env) {
        Some(obj) => {
          if is_error(obj.clone()) {
            return Some(obj);
          }

          Some(env.set(var_stmt.name.value.clone(), obj))
        },
        None => None,
      },
      None => None,
    },

    // Return
    Statements::RETURN(return_stmt) => match return_stmt.value.clone() {
      Some(value_exp) => match expression::evaluate(value_exp.clone(), env) {
        Some(obj) => {
          if is_error(obj.clone()) {
            return Some(obj);
          }

          Some(Return::new(obj))
        },
        None => None,
      },
      None => None,
    },
  }
}

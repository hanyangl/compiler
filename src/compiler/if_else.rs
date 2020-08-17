use crate::expressions::if_else::IfElse;
use crate::objects::{Objects, error::is_error, boolean::is_truthy, null::Null};

use super::{environment::Environment, expression, statement};

pub fn evaluate(ie: IfElse, env: &mut Environment) -> Option<Box<Objects>> {
  match ie.condition.clone() {
    Some(exp) => match expression::evaluate(exp, env) {
      Some(condition) => {
        if is_error(condition.clone()) {
          return Some(condition);
        } else if is_truthy(condition) == true {
          match statement::evaluate(ie.consequence.clone(), &mut Environment::from_environment(env.clone())) {
            Some(obj) => {
              return Some(obj);
            },
            None => {},
          }
        }

        match ie.alternative.clone() {
          Some(alternative) => {
            match statement::evaluate(alternative, &mut Environment::from_environment(env.clone())) {
              Some(obj) => Some(obj),
              None => None,
            }
          },
          None => Some(Null::new()),
        }
      },
      None => None,
    },
    None => None,
  }
}

use crate::expressions::{Expressions, call::Call};
use crate::objects::{Objects, error::is_error};

use super::{environment::Environment, expression, statement};

fn evaluate_arguments(arguments: Vec<Box<Expressions>>, env: &mut Environment) -> Vec<Box<Objects>> {
  let mut result: Vec<Box<Objects>> = Vec::new();

  for exp in arguments.iter() {
    match expression::evaluate(exp.clone(), env) {
      Some(eval) => {
        if is_error(eval.clone()) {
          result.clear();
          result.push(eval.clone());

          return result;
        }

        result.push(eval.clone());
      },
      None => {},
    }
  }

  result
}

pub fn evaluate(call: Call, env: &mut Environment) -> Option<Box<Objects>> {
  match expression::evaluate(call.function.clone(), env) {
    Some(function_obj) => {
      if is_error(function_obj.clone()) {
        return Some(function_obj);
      }

      let arguments = evaluate_arguments(call.arguments, env);
      if arguments.clone().len() == 1 && is_error(arguments[0].clone()) {
        return Some(arguments[0].clone());
      }

      match function_obj.get_function() {
        Some(function) => {
          let mut env = Environment::from_environment(env.clone());
          let mut i: usize = 0;

          for param in function.parameters.clone() {
            match param.get_parameter() {
              Some(param_exp) => {
                if arguments.len() > i {
                  env.set(param_exp.name.value, arguments[i].clone());
                } else {
                  match param_exp.default_value {
                    Some(default_value) => match expression::evaluate(default_value.clone(), &mut env) {
                      Some(obj) => {
                        env.set(param_exp.name.value, obj);
                      },
                      None => {},
                    },
                    None => {},
                  }
                }
              },
              None => {},
            }

            i += 1;
          }

          match statement::evaluate(function.body, &mut env) {
            Some(obj) => match obj.clone().get_return() {
              Some(return_obj) => Some(return_obj.value),
              None => Some(obj.clone()),
            },
            None => None,
          }
        },
        None => None,
      }
    },
    None => None,
  }
}

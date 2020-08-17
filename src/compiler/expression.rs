use crate::expressions::Expressions;

use crate::objects::{
  Objects,
  integer::Integer,
  string::StringO,
  boolean::Boolean,
  function::Function,
  error::{
    is_error,
    Error
  },
};

use super::{environment::Environment, prefix, infix, if_else, call};

pub fn evaluate(exp: Box<Expressions>, env: &mut Environment) -> Option<Box<Objects>> {
  match exp.clone().as_ref() {
    // Integer
    Expressions::INTEGER(int_exp) => Some(Integer::new(int_exp.value)),

    // String
    Expressions::STRING(str_exp) => Some(StringO::new(str_exp.value[1..str_exp.value.len()-1].to_string())),

    // Boolean
    Expressions::BOOLEAN(boolean_exp) => Some(Boolean::new(boolean_exp.value.clone())),

    // Prefix
    Expressions::PREFIX(prefix_exp) => match prefix_exp.right.clone() {
      Some(right) => match evaluate(right.clone(), env) {
        Some(right_obj) => {
          if is_error(right_obj.clone()) {
            return Some(right_obj);
          }

          Some(prefix::evaluate(prefix_exp.operator.clone(), right_obj))
        },
        None => None,
      },
      None => None,
    },

    // Infix
    Expressions::INFIX(infix_exp) => match infix_exp.left.clone() {
      Some(left) => match evaluate(left.clone(), env) {
        Some(left_obj) => {
          if is_error(left_obj.clone()) {
            return Some(left_obj);
          }

          match infix_exp.right.clone() {
            Some(right) => match evaluate(right.clone(), env) {
              Some(right_obj) => {
                if is_error(right_obj.clone()) {
                  return Some(right_obj);
                }

                Some(infix::evaluate(infix_exp.operator.clone(), left_obj, right_obj))
              },
              None => None,
            },
            None => None,
          }
        },
        None => None,
      },
      None => None,
    },

    // If-else
    Expressions::IFELSE(ifelse_exp) => if_else::evaluate(ifelse_exp.clone(), env),

    // Identifier
    Expressions::IDENTIFIER(identifier) => match env.clone().get(identifier.value.clone()) {
      Some(value) => Some(value),
      None => Some(Error::new(format!("identifier not found: {}", identifier.value))),
    },

    // Function
    Expressions::FUNCTION(function_exp) => {
      let function_obj = Function::new(function_exp.clone());

      env.set(function_exp.name.value.clone(), function_obj.clone());

      Some(function_obj)
    },

    // Call
    Expressions::CALL(call_exp) => call::evaluate(call_exp.clone(), env),

    // Default
    _ => None,
  }
}

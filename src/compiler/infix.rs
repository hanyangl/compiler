use crate::objects::{Objects, ObjectType, boolean::Boolean, error::Error};

use super::{string, integer};

pub fn evaluate(operator: String, left: Box<Objects>, right: Box<Objects>) -> Box<Objects> {
  // Concat strings
  if operator.as_str() == "+" && (
    left.clone().object_type() == ObjectType::STRING ||
    right.clone().object_type() == ObjectType::STRING
  ) {
    return string::evaluate(operator, left.clone(), right.clone());
  }
  
  // Number operations
  if left.clone().object_type() == ObjectType::INTEGER && right.clone().object_type() == ObjectType::INTEGER {
    return integer::evaluate(operator, left.clone(), right.clone());
  }

  match operator.as_str() {
    // Equal value
    "==" => {
      return Boolean::new(left.string() == right.string());
    },

    // Equal type and value
    "===" => {
      return Boolean::new(left == right);
    },

    // Not equal value
    "!=" => {
      return Boolean::new(left.string() != right.string());
    },

    // Not equal type and value
    "!==" => {
      return Boolean::new(left != right);
    },

    // Default
    _ => {},
  }

  // Default
  Error::new(format!("unknown operator: {:?} {} {:?}", left.object_type(), operator, right.object_type()))
}
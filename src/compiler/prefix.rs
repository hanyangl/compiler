use crate::objects::{Objects, boolean::Boolean, null::Null, error::Error, integer::Integer, ObjectType};

pub fn evaluate(operator: String, right: Box<Objects>) -> Box<Objects> {
  match operator.as_str() {
    // Negation sign.
    "!" => {
      // False or null to true
      if right.clone() == Boolean::new(false) || right.clone() == Null::new() {
        return Boolean::new(true);
      }

      // True to false
      return Boolean::new(false);
    },

    // Negative numbers.
    "-" => {
      if right.clone().object_type() != ObjectType::INTEGER {
        return Error::new(format!("unknown operator: -{:?}", right.clone().object_type()));
      }

      return Integer::new(right.string().parse().unwrap());
    },

    // Default
    _ => Error::new(format!("unknown operator: {}{:?}", operator, right.clone().object_type())),
  }
}
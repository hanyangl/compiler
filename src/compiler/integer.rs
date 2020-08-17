use crate::objects::{Objects, integer::Integer, boolean::Boolean, error::Error};

pub fn evaluate(operator: String, left: Box<Objects>, right: Box<Objects>) -> Box<Objects> {
  let mut left_value: i64 = 0;
  let mut right_value: i64 = 0;

  match left.clone().get_integer() {
    Some(integer) => {
      left_value = integer.value;
    },
    None => {},
  }

  match right.clone().get_integer() {
    Some(integer) => {
      right_value = integer.value;
    }
    None => {},
  }

  match operator.clone().as_str() {
    // Operations
    "+" => Integer::new(left_value + right_value),
    "-" => Integer::new(left_value - right_value),
    "*" => Integer::new(left_value * right_value),
    "/" => Integer::new(left_value / right_value),

    // Conditions
    "<" => Boolean::new(left_value < right_value),
    "<=" => Boolean::new(left_value <= right_value),
    ">" => Boolean::new(left_value > right_value),
    ">=" => Boolean::new(left_value >= right_value),
    "==" | "===" => Boolean::new(left_value == right_value),
    "!=" | "!==" => Boolean::new(left_value != right_value),

    // Default
    _ => Error::new(format!("unknown operator: {:?} {} {:?}", left.object_type(), operator, right.object_type())),
  }
}

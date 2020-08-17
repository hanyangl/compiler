use crate::objects::{Objects, string::StringO, error::Error};

pub fn evaluate(operator: String, left: Box<Objects>, right: Box<Objects>) -> Box<Objects> {
  if operator.as_str() != "+" {
    return Error::new(format!("unknown operator: {:?} {} {:?}", left.object_type(), operator, right.object_type()));
  }

  StringO::new(format!("{}{}", left.string().as_str(), right.string().as_str()))
}

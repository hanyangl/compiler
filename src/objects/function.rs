use crate::data::Token;
use crate::expressions::{Expressions, function::Function as FunctionE};
use crate::statements::Statements;

use super::{Object, ObjectType, Objects};

#[derive(Debug, Clone, PartialEq)]
pub struct Function {
  pub name: Token,
  pub parameters: Vec<Box<Expressions>>,
  pub return_type: Token,
  pub body: Box<Statements>,
}

impl Object for Function {
  fn object_type(&self) -> ObjectType {
    ObjectType::FUNCTION
  }

  fn string(self) -> String {
    let mut params: Vec<String> = Vec::new();

    for param in self.parameters {
      params.push(param.string());
    }

    format!(
      "function {}({}): {} {{ {} }}",
      self.name.value,
      params.join(", "),
      self.return_type.value,
      self.body.string(),
    )
  }
}

impl Function {
  pub fn new(function: FunctionE) -> Box<Objects> {
    Box::new(
      Objects::FUNCTION(
        Function {
          name: function.name,
          parameters: function.parameters,
          return_type: function.return_type,
          body: function.body,
        }
      )
    )
  }
}

use crate::Environment;

use sflyn_parser::expressions::Expressions;
use sflyn_parser::statements::Statements;
use sflyn_parser::tokens::Token;

use super::{Object, Objects};

#[derive(Debug, Clone, PartialEq)]
pub struct AnonymousFunction {
  pub has_function: bool,
  pub arguments: Vec<Box<Expressions>>,
  pub data_type: Token,
  pub body: Box<Statements>,
  pub environment: Environment,
}

impl Object for AnonymousFunction {
  fn string(self) -> String {
    let mut arguments: Vec<String> = Vec::new();

    for argument in self.arguments {
      arguments.push(argument.string());
    }

    let function = format!(
      "({}): {}",
      arguments.join(", "),
      self.data_type.value,
    );

    if self.has_function {
      return format!("function {} {}", function, self.body.string());
    }

    format!("{} => {}", function, self.body.string())
  }
}

impl AnonymousFunction {
  pub fn new(
    has_function: bool,
    arguments: Vec<Box<Expressions>>,
    data_type: Token,
    body: Box<Statements>,
    environment: Environment,
  ) -> Box<Objects> {
    Box::new(Objects::ANONYMOUSFUNCTION(AnonymousFunction {
      has_function,
      arguments,
      data_type,
      body,
      environment,
    }))
  }
}

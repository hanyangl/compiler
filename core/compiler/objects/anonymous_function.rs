use crate::{
  compiler::evaluate_expression,
  Environment,
  Store,
};

use sflyn_parser::{
  Expressions,
  Statements,
  tokens::Token,
};

use super::{
  Object,
  Objects,
};

#[derive(Debug, Clone, PartialEq)]
pub struct AnonymousFunction {
  pub has_function: bool,
  pub arguments: Vec<Box<Expressions>>,
  pub data_type: Token,
  pub body: Box<Statements>,
  pub store: Store,
}

impl Object for AnonymousFunction {
  fn string(&self) -> String {
    let mut arguments: Vec<String> = Vec::new();

    for argument in self.arguments.iter() {
      arguments.push(argument.clone().string());
    }

    let function = format!(
      "({}): {}",
      arguments.join(", "),
      self.data_type.value,
    );

    let body = self.body.clone().string();

    if self.has_function {
      return format!("function {} {}", function, body);
    }

    format!("{} => {}", function, body)
  }
}

impl AnonymousFunction {
  pub fn add_arguments_to_environment(
    arguments: Vec<Box<Expressions>>,
    environment: &mut Environment,
  ) {
    for argument in arguments.iter() {
      let function_argument = argument.get_argument().unwrap();

      if !function_argument.has_default_value() {
        continue;
      }

      if let Some(expression) = function_argument.value {
        let object = evaluate_expression(&expression, environment);

        environment.store.set_object(function_argument.token.value.clone(), object);
      }
    }
  }

  pub fn new(
    has_function: bool,
    arguments: Vec<Box<Expressions>>,
    data_type: Token,
    body: Box<Statements>,
    store: Store,
  ) -> Box<Objects> {
    Box::new(Objects::ANONYMOUSFUNCTION(AnonymousFunction {
      has_function,
      arguments,
      data_type,
      body,
      store,
    }))
  }
}

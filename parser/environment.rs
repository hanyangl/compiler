use std::collections::HashMap;

use super::expressions::Expressions;
use super::statements::Statements;

#[derive(Debug, Clone, PartialEq)]
pub struct Environment {
  pub statements: HashMap<String, Box<Statements>>,
  pub expressions: HashMap<String, Box<Expressions>>,
  pub outer: Option<Box<Environment>>,
}

impl Environment {
  pub fn new() -> Environment {
    Environment {
      statements: HashMap::new(),
      expressions: HashMap::new(),
      outer: None,
    }
  }

  pub fn from_environment(environment: Environment) -> Environment {
    Environment {
      statements: HashMap::new(),
      expressions: HashMap::new(),
      outer: Some(Box::new(environment)),
    }
  }

  pub fn get_first_statement(&self, name: String) -> Option<Box<Statements>> {
    match self.statements.get(&name) {
      Some(statement) => Some(statement.clone()),
      None => None,
    }
  }

  pub fn get_first_expression(&self, name: String) -> Option<Box<Expressions>> {
    match self.expressions.get(&name) {
      Some(expression) => Some(expression.clone()),
      None => None,
    }
  }

  pub fn get_statement(&self, name: String) -> Option<Box<Statements>> {
    match self.get_first_statement(name.clone()) {
      Some(statement) => Some(statement),
      None => match self.outer.clone() {
        Some(outer) => outer.get_statement(name),
        None => None,
      },
    }
  }

  pub fn get_expression(&self, name: String) -> Option<Box<Expressions>> {
    match self.get_first_expression(name.clone()) {
      Some(expression) => Some(expression),
      None => match self.outer.clone() {
        Some(outer) => outer.get_expression(name),
        None => None,
      },
    }
  }

  pub fn has_statement(&mut self, name: String) -> bool {
    match self.get_statement(name) {
      Some(_) => true,
      None => false,
    }
  }

  pub fn has_expression(&mut self, name: String) -> bool {
    match self.get_expression(name) {
      Some(_) => true,
      None => false,
    }
  }

  pub fn set_statement(&mut self, name: String, statement: Box<Statements>) {
    if self.has_statement(name.clone()) {
      return;
    }

    self.statements.insert(name, statement);
  }

  pub fn set_expression(&mut self, name: String, expression: Box<Expressions>) {
    if self.has_expression(name.clone()) {
      return;
    }

    self.expressions.insert(name, expression);
  }
}

use crate::compiler::environment::Environment;
use crate::data::{Token, Signs, Tokens};
use crate::objects::function;
use crate::parser::Parser;
use crate::statements::{block, Statements};

use super::{Expression, Expressions, parameter::parse as parse_parameters};

// EXPRESSION //
#[derive(Debug, Clone, PartialEq)]
pub struct Function {
  pub token: Token,
  pub name: Token,
  pub parameters: Vec<Box<Expressions>>,
  pub return_type: Token,
  pub body: Box<Statements>,
}

impl Expression for Function {
  fn new() -> Function {
    Function {
      token: Token::empty(),
      name: Token::empty(),
      parameters: Vec::new(),
      return_type: Token::from_value(String::from("void"), 0, 0),
      body: block::Block::new(),
    }
  }

  fn from_token(token: &Token) -> Function {
    let mut exp: Function = Expression::new();

    exp.token = token.clone();

    exp
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
// END EXPRESSION //


// PARSER //
pub fn parse<'a>(parser: &'a mut Parser, env: &mut Environment) -> Option<Function> {
  let mut exp: Function = Expression::from_token(&parser.current_token.clone());

  // Get the function name.
  if !parser.peek_token_is(&Tokens::IDENTIFIER) {
    let line = parser.get_error_line("function ");

    parser.errors.push(format!("{} `{}` is not a valid function name.", line, parser.peek_token.value));

    return None;
  }

  // Check if the functtion name is in use.
  match env.clone().get(parser.peek_token.value.clone()) {
    Some(_) => {
      let line = parser.get_error_line("function ");

      parser.errors.push(format!("{} `{}` is already in use.", line, parser.peek_token.value));

      return None;
    },
    None => {
      parser.next_token();
    },
  }

  // Set the function name to the expression.
  exp.name = parser.current_token.clone();

  if !parser.expect_sign(&Signs::LEFTPARENTHESES) {
    let line = parser.get_error_line(format!("function {}", exp.name.value).as_str());

    parser.errors.push(format!("{} the functions need parameters.", line));

    return None;
  }

  let mut func_env = Environment::from_environment(env.clone());

  // Get function parameters.
  match parse_parameters(parser, &mut func_env) {
    Some(parameters) => {
      exp.parameters = parameters;
    },

    // No parameters
    None => {
      return None;
    },
  };

  // Set temp data to environment.
  env.set(exp.name.value.clone(), function::Function::new(exp.clone()));

  // Get the function return type. (Default: void).
  if parser.expect_sign(&Signs::COLON) == true {
    if parser.expect_token(&Tokens::TYPE) == false {
      let mut params: Vec<String> = Vec::new();

      for param in exp.parameters {
        params.push(param.string());
      }

      let line = parser.get_error_line(format!("function {}({}): ", exp.name.value, params.join(", ")).as_str());

      parser.errors.push(format!("{} `{}` is not a valid data type.", line, parser.current_token.value));

      return None;
    }

    // Set the function return type.
    exp.return_type = parser.current_token.clone();
  }

  if parser.expect_sign(&Signs::LEFTBRACE) == false {
    let mut params: Vec<String> = Vec::new();

    for param in exp.parameters {
      params.push(param.string());
    }

    let line = parser.get_error_line(
      format!(
        "function {}({}): {} ",
        exp.name.value,
        params.join(", "),
        exp.return_type.value
      ).as_str()
    );

    parser.errors.push(format!("{} the functions need a code block.", line));

    return None;
  }

  // Get function body.
  exp.body = block::parse(parser, env);

  match exp.body.clone().get_block() {
    Some(block) => {
      for eval_stmt in block.statements.iter() {
        if block::parse_function_block(parser, eval_stmt.clone(), &exp.clone(), &mut func_env) == false {
          return None;
        }
      }
    },
    None => {},
  }

  if parser.current_token_is_sign(&Signs::RIGHTBRACE) {
    parser.next_token();
  }

  Some(exp)
}
// END PARSER //

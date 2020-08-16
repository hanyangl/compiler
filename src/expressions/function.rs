use crate::data::{Token, Signs, Tokens};
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
pub fn parse<'a>(parser: &'a mut Parser) -> Option<Function> {
  let mut exp: Function = Expression::from_token(&parser.current_token.clone());

  if parser.expect_token(&Tokens::IDENTIFIER) == false {
    let line = parser.get_error_line("function ");

    parser.errors.push(format!("{} `{}` is not a valid function name.", line, parser.current_token.value));

    return None;
  }

  exp.name = parser.current_token.clone();

  if parser.expect_sign(&Signs::LEFTPARENTHESES) == false {
    let line = parser.get_error_line(format!("function {}", exp.name.value).as_str());

    parser.errors.push(format!("{} the functions need parameters.", line));

    return None;
  }

  // Get function parameters.
  match parse_parameters(parser) {
    Some(parameters) => {
      exp.parameters = parameters;
    },

    // No parameters
    None => {
      return None;
    },
  };

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
  exp.body = block::parse(parser);

  match exp.body.clone().get_block() {
    Some(block) => {
      for eval_stmt in block.statements.iter() {
        if block::parse_function_block(parser, eval_stmt.clone(), &exp.clone()) == false {
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

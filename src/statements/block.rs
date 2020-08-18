use crate::compiler::environment::Environment;
use crate::data::{Token, Signs, Tokens, Keywords};
use crate::expressions::function::Function;
use crate::parser::Parser;

use super::{Statement, Statements, return_s::parse_type as parse_return};

// STATEMENT //
#[derive(Debug, Clone, PartialEq)]
pub struct Block {
  pub token: Token,
  pub statements: Vec<Box<Statements>>,
}

impl Statement for Block {
  fn new() -> Block {
    Block {
      token: Token::empty(),
      statements: Vec::new(),
    }
  }

  fn from_token(token: &Token) -> Block {
    let mut statement: Block = Statement::new();

    statement.token = token.clone();

    statement
  }

  fn string(self) -> String {
    let mut string = String::new();

    for statement in self.statements {
      string.push_str(statement.string().as_str());
    }

    string
  }
}

impl Block {
  pub fn new() -> Box<Statements> {
    Box::new(Statements::BLOCK(Statement::new()))
  }
}
// END STATEMENT //


// PARSER //
pub fn parse<'a>(parser: &'a mut Parser, env: &mut Environment) -> Box<Statements> {
  let mut statement: Block = Statement::from_token(&parser.current_token.clone());

  parser.next_token();

  while parser.current_token_is_sign(&Signs::RIGHTBRACE) == false && parser.current_token_is(&Tokens::EOF) == false {
    match parser.parse_statement(env) {
      Some(x) => statement.statements.push(x),
      None => {},
    }

    parser.next_token();
  }

  Box::new(Statements::BLOCK(statement))
}

pub fn parse_function_block<'a>(
  parser: &'a mut Parser,
  eval_stmt: Box<Statements>,
  function: &Function,
  env: &mut Environment,
) -> bool {
  match eval_stmt.clone().get_return() {
    // Parse return value.
    Some(return_stmt) => {
      return parse_return(parser, &return_stmt, function, env);
    },

    // Parse expressions.
    None => match eval_stmt.clone().get_expression() {
      Some(exp_stmt) => {
        // Parse if expression.
        if exp_stmt.token.keyword == Keywords::IF {
          return match exp_stmt.expression {
            // Get if expression.
            Some(exp_exp) => {
              let mut value = true;

              match exp_exp.get_ifelse() {
                // Get first block statement.
                Some(ifelse_exp) => match ifelse_exp.consequence.clone().get_block() {
                  Some(consequence) => {
                    // Parse statements.
                    for stmt in consequence.statements.iter() {
                      value = parse_function_block(parser, stmt.clone(), function, env);
                    }

                    // Get alternative block statement.
                    match ifelse_exp.alternative {
                      Some(alternative) => match alternative.get_block() {
                        Some(else_exp) => {
                          for stmt in else_exp.statements.iter() {
                            value = parse_function_block(parser, stmt.clone(), function, env);
                          }
                        },

                        // Default
                        None => {},
                      },

                      // Default
                      None => {},
                    }
                  },

                  // Default
                  None => {},
                },

                // Default
                None => {},
              }

              value
            },

            // Default
            None => true,
          }
        }

        true
      },

      // Default
      None => true,
    },
  }
}
// END PARSER //

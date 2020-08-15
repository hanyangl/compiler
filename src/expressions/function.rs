use crate::data::{Token, Signs, Tokens, Types, Keywords};
use crate::expressions::{Identifier, Expression};
use crate::parser::Parser;
use crate::statements::{block, Statement, Statements, return_s::Return, expression::expression_is_valid_type};
use crate::utils::repeat_character;

// EXPRESSION //
#[derive(Debug, Clone)]
pub struct Function {
  pub token: Token,
  name: Token,
  parameters: Vec<Identifier>,
  return_type: Token,
  body: block::Block,
}

impl Expression for Function {
  fn new() -> Function {
    Function {
      token: Token::empty(),
      name: Token::empty(),
      parameters: Vec::new(),
      return_type: Token::from_value(String::from("void"), 0, 0),
      body: Statement::new(),
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
      params.push(format!("{}: {}", param.value, param.token.value));
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
fn parse_parameters<'a>(parser: &'a mut Parser) -> Option<Vec<Identifier>> {
  let mut parameters: Vec<Identifier> = Vec::new();

  while parser.current_token_is_sign(&Signs::RIGHTPARENTHESES) == false {
    if parser.expect_token(&Tokens::IDENTIFIER) == false {
      let line = parser.get_error_line("");

      parser.errors.push(format!("{} `{}` is not a valid parameter name.", line, parser.current_token.value));

      return None;
    }

    let mut ident: Identifier = Expression::new();

    ident.value = parser.current_token.value.clone();

    if parser.expect_sign(&Signs::COLON) == false {
      let line = parser.get_error_line(format!("{}", ident.value).as_str());

      parser.errors.push(format!("{} expect `:`, got `{}` instead.", line, parser.current_token.value));

      return None;
    }

    if parser.expect_token(&Tokens::TYPE) == false {
      let line = parser.get_error_line(format!("{}: ", ident.value).as_str());

      parser.errors.push(format!("{} `{}` is not a valid data type.", line, parser.current_token.value));

      return None;
    }

    ident.token = parser.current_token.clone();
    parameters.push(ident);

    if parser.peek_token_is_sign(&Signs::COMMA) == true {
      parser.next_token();
    }

    parser.next_token();
  }

  Some(parameters)
}

fn parse_return<'a>(parser: &'a mut Parser, return_stmt: &Return, exp: &Function) -> bool {
  match return_stmt.value.clone() {
    // With return value.
    Some(value) => {
      let left_line = format!("{} | return ", value.clone().token().line);
  
      let line = format!(
        "{}{}\n{}{}",
        left_line,
        value.clone().string(),
        repeat_character(left_line.len(), " "),
        repeat_character(value.clone().string().len(), "^"),
      );

      if exp.return_type.data_type == Types::VOID {
        parser.errors.push(format!("{} the `{}` function no return a value.", line, exp.name.value));

        return false;
      }

      if expression_is_valid_type(&exp.return_type.data_type, &value) == false {
        parser.errors.push(format!("{} `{}` not satisfied the {} data type.", line, value.clone().string(), exp.return_type.value));

        return false;
      }

      true
    },

    // No value
    None => {
      if exp.return_type.data_type != Types::VOID {
        let line = parser.get_error_line("return");

        parser.errors.push(format!("{} the '{}' function returns a {}.", line, exp.name.value, exp.return_type.value));

        return false;
      }

      true
    },
  }
}

fn parse_function_body<'a>(parser: &'a mut Parser, eval_stmt: Box<Statements>, exp: &Function) -> bool {
  match eval_stmt.clone().get_return() {
    // Get return statement.
    Some(return_stmt) => parse_return(parser, &return_stmt, exp),

    // Default
    None => match eval_stmt.clone().get_expression() {
      // Get expression statement.
      Some(exp_stmt) => {
        if exp_stmt.token.keyword == Keywords::IF {
          return match exp_stmt.expression {
            Some(exp_exp) => {
              let mut value = true;

              match exp_exp.get_ifelse() {
                Some(ifelse) => {
                  for stmt in ifelse.consequence.statements.iter() {
                    value = parse_function_body(parser, stmt.clone(), exp);
                  }

                  match ifelse.alternative {
                    Some(else_exp) => {
                      for stmt in else_exp.statements.iter() {
                        value = parse_function_body(parser, stmt.clone(), exp);
                      }
                    },
                    None => {},
                  }
                },
                None => {},
              };

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
        params.push(format!("{}: {}", param.value, param.token.value));
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
      params.push(format!("{}: {}", param.value, param.token.value));
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

  for eval_stmt in exp.body.statements.iter() {
    if parse_function_body(parser, eval_stmt.clone(), &exp) == false {
      println!("Error");
      return None;
    }
  }

  if parser.current_token_is_sign(&Signs::RIGHTBRACE) {
    parser.next_token();
  }

  Some(exp)
}
// END PARSER //

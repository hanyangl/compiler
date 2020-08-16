use crate::data::{Token, Signs, Tokens};
use crate::parser::Parser;
use crate::utils::types::token_is_valid_type;

use super::{Expression, Expressions};

// EXPRESSION //
#[derive(Debug, Clone, PartialEq)]
pub struct Parameter {
  pub name: Token,
  pub data_type: Token,
  pub default_value: Option<Token>,
}

impl Expression for Parameter {
  fn new() -> Parameter {
    Parameter {
      name: Token::empty(),
      data_type: Token::empty(),
      default_value: None,
    }
  }

  fn from_token(token: &Token) -> Parameter {
    let mut exp: Parameter = Expression::new();

    exp.name = token.clone();

    exp
  }

  fn string(self) -> String {
    let param = format!("{}: {}", self.name.value, self.data_type.value);

    match self.default_value {
      Some(default) => format!("{} = {}", param, default.value),
      None => param,
    }
  }
}
// END EXPRESSION //


// PARSER //
pub fn parse<'a>(parser: &'a mut Parser) -> Option<Vec<Box<Expressions>>> {
  let mut parameters: Vec<Box<Expressions>> = Vec::new();

  while parser.current_token_is_sign(&Signs::RIGHTPARENTHESES) == false {
    if parser.expect_token(&Tokens::IDENTIFIER) == false {
      let line = parser.get_error_line("");

      parser.errors.push(format!("{} `{}` is not a valid parameter name.", line, parser.peek_token.value));

      return None;
    }

    let mut parameter: Parameter = Expression::from_token(&parser.current_token.clone());

    if parser.expect_sign(&Signs::COLON) == false {
      let line = parser.get_error_line(format!("{}", parameter.name.value).as_str());

      parser.errors.push(format!("{} expect `:`, got `{}` instead.", line, parser.peek_token.value));

      return None;
    }

    if parser.expect_token(&Tokens::TYPE) == false {
      let line = parser.get_error_line(format!("{}: ", parameter.name.value).as_str());

      parser.errors.push(format!("{} `{}` is not a valid data type.", line, parser.peek_token.value));

      return None;
    }

    parameter.data_type = parser.current_token.clone();

    if parser.peek_token_is_sign(&Signs::COMMA) == true {
      parser.next_token();
    } else if parser.peek_token_is_sign(&Signs::ASSIGN) == true {
      parser.next_token();

      if token_is_valid_type(&parameter.data_type.data_type, &parser.peek_token) == false {
        let line = parser.get_error_line(format!("{}: {} = ", parameter.name.value, parameter.data_type.value).as_str());

        parser.errors.push(
          format!(
            "{} `{}` not satisfied the {} data type.",
            line,
            parser.peek_token.value,
            parameter.data_type.value
          )
        );

        return None;
      }

      parser.next_token();

      parameter.default_value = Some(parser.current_token.clone());
    }

    parser.next_token();

    parameters.push(Box::new(Expressions::PARAMETER(parameter)));
  }

  Some(parameters)
}
// END PARSER //

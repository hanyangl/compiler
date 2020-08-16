use crate::data::{Token, Signs, Keywords};
use crate::parser::{Parser, precedence::Precedence};
use crate::statements::{block, Statements};

use super::{Expression, Expressions, parse as expression_parse};

// EXPRESSION //
#[derive(Debug, Clone, PartialEq)]
pub struct IfElse {
  pub token: Token,
  pub condition: Option<Box<Expressions>>,
  pub consequence: Box<Statements>,
  pub alternative: Option<Box<Statements>>,
}

impl Expression for IfElse {
  fn new() -> IfElse {
    IfElse {
      token: Token::empty(),
      condition: None,
      consequence: block::Block::new(),
      alternative: None,
    }
  }

  fn from_token(token: &Token) -> IfElse {
    let mut exp: IfElse = Expression::new();

    exp.token = token.clone();

    exp
  }

  fn string(self) -> String {
    format!(
      "if {} {{ {} }}{}",
      match self.condition {
        Some(x) => x.string(),
        None => "()".to_string(),
      },
      self.consequence.string(),
      match self.alternative {
        Some(x) => format!(" else {{ {} }}", x.string()),
        None => "".to_string(),
      }
    )
  }
}
// END EXPRESSION //


// PARSER //
pub fn parse<'a>(parser: &'a mut Parser) -> Option<IfElse> {
  let mut exp: IfElse = Expression::from_token(&parser.current_token.clone());

  if parser.expect_sign(&Signs::LEFTPARENTHESES) == false {
    let line = parser.get_error_line("if ");

    parser.errors.push(format!("{} the if condition need starts with `(`", line));

    return None;
  }

  parser.next_token();

  exp.condition = expression_parse(parser, Precedence::LOWEST);

  if parser.expect_sign(&Signs::RIGHTPARENTHESES) == false {
    let line = parser.get_error_line(format!("if ({}", exp.condition.unwrap().string()).as_str());

    parser.errors.push(format!("{} the if condition need ends with `)`", line));

    return None;
  }

  if parser.expect_sign(&Signs::LEFTBRACE) == false {
    let line = parser.get_error_line(format!("if ({}) ", exp.condition.unwrap().string()).as_str());

    parser.errors.push(format!("{} the if need a block of code.", line));

    return None;
  }

  exp.consequence = block::parse(parser);

  if parser.peek_token_is_keyword(&Keywords::ELSE) == true {
    parser.next_token();

    if parser.expect_sign(&Signs::LEFTBRACE) == false {
      let line = parser.get_error_line("} else ");

      parser.errors.push(format!("{} the else need a block of code.", line));

      return None;
    }

    exp.alternative = Some(block::parse(parser));
  }

  Some(exp)
}
// END PARSER //

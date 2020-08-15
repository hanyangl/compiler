use crate::data::{Token, Signs, Tokens};
use crate::expressions::{Expressions, Expression, Identifier};
use crate::parser::Parser;
use crate::statements::expression::parse_list;

// EXPRESSION //
#[derive(Debug, Clone)]
pub struct Call {
  pub token: Token,
  function: Box<Expressions>,
  arguments: Vec<Box<Expressions>>,
  semicolon: Option<Token>,
}

impl Expression for Call {
  fn new() -> Call {
    Call {
      token: Token::empty(),
      function: Box::new(Expressions::DEFAULT(Identifier::new())),
      arguments: Vec::new(),
      semicolon: None,
    }
  }

  fn from_token(token: &Token) -> Call {
    let mut exp: Call = Expression::new();

    exp.token = token.clone();

    exp
  }

  fn string(self) -> String {
    let mut args: Vec<String> = Vec::new();

    for argument in self.arguments {
      args.push(argument.string());
    }

    format!(
      "{}({}){}",
      self.function.string(),
      args.join(", "),
      match self.semicolon {
        Some(x) => format!("{}", x.value),
        None => "".to_string(),
      }
    )
  }
}
// END EXPRESSION //


// PARSER //
pub fn parse<'a>(parser: &'a mut Parser, function: Option<Box<Expressions>>) -> Call {
  let mut exp: Call = Expression::from_token(&parser.current_token.clone());

  match function {
    Some(x) => {
      exp.function = x;
    },
    None => {},
  }

  exp.arguments = parse_list(parser, Signs::RIGHTPARENTHESES);

  if parser.expect_sign(&Signs::SEMICOLON) == true {
    exp.semicolon = Some(parser.current_token.clone());

    parser.next_token();
  } else if exp.clone().function.token().token == Tokens::IDENTIFIER {
    let token: Token = parser.current_token.clone();

    exp.semicolon = Some(Token::from_value(String::from(";"), token.position, token.line));
    parser.next_token();
  }

  exp
}
// END PARSER //

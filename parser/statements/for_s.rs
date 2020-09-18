use crate::{
  Block,
  Error,
  Expressions,
  Infix,
  parse_expression,
  Parser,
  Precedence,
  Statement,
  Statements,
  tokens::{
    Signs,
    Token,
  },
};

#[derive(Debug, Clone, PartialEq)]
pub struct For {
  token: Token,
  condition: Box<Expressions>,
  body: Box<Statements>,
}

impl Statement for For {
  fn new() -> Self {
    Self {
      token: Token::new_empty(),
      condition: Infix::new_box(),
      body: Block::new_box(),
    }
  }

  fn from_token(token: Token) -> Self {
    let mut for_s: Self = Statement::new();

    for_s.token = token;

    for_s
  }

  fn get_token(&self) -> Token {
    self.token.clone()
  }

  fn string(&self) -> String {
    format!(
      "{} ({}) {}",
      self.get_token().value,
      self.get_condition().string(),
      self.get_body().string(),
    )
  }
}

impl For {
  pub fn get_condition(&self) -> Box<Expressions> {
    self.condition.clone()
  }

  pub fn get_body(&self) -> Box<Statements> {
    self.body.clone()
  }

  pub fn parse<'a>(
    parser: &'a mut Parser,
    standard_library: bool,
    with_this: bool,
  ) -> Result<Box<Statements>, Error> {
    let mut for_s: Self = Statement::from_token(parser.get_current_token());

    // Check if the next token is a left parentheses.
    if !parser.expect_token(Signs::new(Signs::LEFTPARENTHESES)) {
      return Err(Error::from_token(
        format!("expect `(`, got `{}` instead.", parser.get_current_token().value),
        parser.get_current_token(),
      ));
    }

    // Get the next token.
    parser.next_token();

    match parse_expression(parser, Precedence::LOWEST, standard_library, with_this) {
      Ok(condition) => {
        for_s.condition = condition;
      },
      Err(error) => {
        return Err(error);
      },
    }

    // Check if the next token is a right parentheses.
    if !parser.expect_token(Signs::new(Signs::RIGHTPARENTHESES)) {
      return Err(Error::from_token(
        format!("expect `)`, got `{}` instead.", parser.get_current_token().value),
        parser.get_current_token(),
      ));
    }

    // Check if the next token is a left brace.
    if !parser.expect_token(Signs::new(Signs::LEFTBRACE)) {
      return Err(Error::from_token(
        format!("expect `{{`, got `{}` instead.", parser.get_current_token().value),
        parser.get_current_token(),
      ));
    }

    // Parse body.
    match Block::parse(parser, standard_library, false, with_this) {
      Ok(body) => {
        for_s.body = body;
      },
      Err(error) => {
        return Err(error);
      },
    }

    Ok(Box::new(Statements::FOR(for_s)))
  }
}

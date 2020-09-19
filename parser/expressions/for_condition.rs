use crate::{
  Error,
  Expression,
  Expressions,
  Identifier,
  parse_expression,
  Parser,
  Precedence,
  tokens::{
    Signs,
    Token,
  },
};

#[derive(Debug, Clone, PartialEq)]
pub struct ForCondition {
  token: Token,
  first: Box<Expressions>,
  second: Box<Expressions>,
  third: Box<Expressions>,
}

impl Expression for ForCondition {
  fn new() -> Self {
    Self {
      token: Token::new_empty(),
      first: Identifier::new_box(),
      second: Identifier::new_box(),
      third: Identifier::new_box(),
    }
  }

  fn from_token(token: Token) -> Self {
    let mut for_condition: Self = Expression::new();

    for_condition.token = token;

    for_condition
  }

  fn get_token(&self) -> Token {
    self.token.clone()
  }

  fn string(&self) -> String {
    format!(
      "{}; {}; {}",
      self.get_first().string(),
      self.get_second().string(),
      self.get_third().string(),
    )
  }
}

impl ForCondition {
  pub fn get_first(&self) -> Box<Expressions> {
    self.first.clone()
  }

  pub fn get_second(&self) -> Box<Expressions> {
    self.second.clone()
  }

  pub fn get_third(&self) -> Box<Expressions> {
    self.third.clone()
  }

  pub fn parse<'a>(
    parser: &'a mut Parser,
    first_expression: Box<Expressions>,
    standard_library: bool,
    with_this: bool,
  ) -> Result<Box<Expressions>, Error> {
    let mut for_condition: Self = Expression::from_token(first_expression.token());

    // Set the first expression.
    for_condition.first = first_expression;

    // Check if the current token is a semicolon.
    if !parser.current_token_is(Signs::new(Signs::SEMICOLON)) {
      return Err(Error::from_token(
        format!("expect `;`, got `{}` instead.", parser.get_current_token().value),
        parser.get_current_token(),
      ));
    }

    // Get the next token.
    parser.next_token();

    // Parse the second expression.
    match parse_expression(parser, Precedence::LOWEST, standard_library, with_this) {
      Ok(second_expression) => {
        for_condition.second = second_expression;
      },
      Err(error) => {
        return Err(error);
      },
    }

    // Check if the next token is a semicolon.
    if !parser.expect_token(Signs::new(Signs::SEMICOLON)) {
      return Err(Error::from_token(
        format!("expect `;`, got `{}` instead.", parser.get_next_token().value),
        parser.get_next_token(),
      ));
    }

    // Get the next token.
    parser.next_token();

    // Parse the third expression.
    match parse_expression(parser, Precedence::LOWEST, standard_library, with_this) {
      Ok(third_expression) => {
        for_condition.third = third_expression;
      },
      Err(error) => {
        return Err(error);
      },
    }

    Ok(Box::new(Expressions::FORCONDITION(for_condition)))
  }
}

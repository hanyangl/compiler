use crate::{
  Error,
  Expressions,
  Infix,
  parse_expression,
  Parser,
  Precedence,
  tokens::{
    Keywords,
    Signs,
    Token,
  },
};

use super::{
  Block,
  Statement,
  Statements,
};

#[derive(Debug, Clone, PartialEq)]
pub struct IfElseCondition {
  pub with_else: Option<Token>,
  pub token: Token,
  pub condition: Box<Expressions>,
  pub consequence: Box<Statements>,
}

impl IfElseCondition {
  pub fn new() -> Self {
    Self {
      with_else: None,
      token: Token::new_empty(),
      condition: Infix::new_box(),
      consequence: Block::new_box(),
    }
  }

  pub fn string(&self) -> String {
    format!(
      "{}{} ({}) {}",
      match self.with_else.clone() {
        Some(with_else) => format!("{} ", with_else.value),
        None => String::new(),
      },
      self.token.value,
      self.condition.string(),
      self.consequence.clone().string(),
    )
  }

  pub fn parse<'a>(
    parser: &'a mut Parser,
    standard_library: bool,
    with_this: bool,
  ) -> Result<IfElseCondition, Error> {
    let mut condition: Self = Self::new();

    // Check if the current token is an `else`.
    if parser.current_token_is(Keywords::new(Keywords::ELSE)) {
      condition.with_else = Some(parser.get_current_token());

      // Get the next token.
      parser.next_token();
    }

    // Check if the current token is an `if`.
    if parser.current_token_is(Keywords::new(Keywords::IF)) {
      condition.token = parser.get_current_token();

      // Get the next token.
      parser.next_token();
    }

    // Check if the current token is a left parentheses.
    if !parser.current_token_is(Signs::new(Signs::LEFTPARENTHESES)) {
      return Err(Error::from_token(
        format!("expect `(`, got `{}` instead.", parser.get_next_token().value),
        parser.get_next_token(),
      ));
    }

    // Get the next token.
    parser.next_token();

    // Parse condition expression.
    match parse_expression(parser, Precedence::LOWEST, standard_library, with_this) {
      Ok(expression) => {
        // Set the condition expression to the if else condition.
        condition.condition = expression;
      },
      Err(error) => {
        return Err(error);
      },
    }

    // Check if the next token is a right parentheses.
    if !parser.expect_token(Signs::new(Signs::RIGHTPARENTHESES)) {
      return Err(Error::from_token(
        format!("expect `)`, got `{}` instead.", parser.get_next_token().value),
        parser.get_next_token(),
      ));
    }

    // Check if the next token is a left brace.
    if !parser.expect_token(Signs::new(Signs::LEFTBRACE)) {
      return Err(Error::from_token(
        format!("expect `{{`, got `{}` instead.", parser.get_next_token().value),
        parser.get_next_token(),
      ));
    }

    // Parse condition body.
    match Block::parse(parser, standard_library, false, with_this) {
      Ok(body) => {
        // Set the condition consequence.
        condition.consequence = body;
      },
      Err(error) => {
        return Err(error);
      },
    }

    Ok(condition)
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct IfElse {
  token: Token,
  conditions: Vec<IfElseCondition>,
  alternative: Option<Box<Statements>>,
}

impl Statement for IfElse {
  fn new() -> Self {
    Self {
      token: Token::new_empty(),
      conditions: Vec::new(),
      alternative: None,
    }
  }

  fn from_token(token: Token) -> Self {
    let mut if_else: Self = Statement::new();

    if_else.token = token;

    if_else
  }

  fn get_token(&self) -> Token {
    self.token.clone()
  }

  fn string(&self) -> String {
    let mut conditions: Vec<String> = Vec::new();

    for condition in self.conditions.iter() {
      conditions.push(condition.string());
    }

    format!(
      "{}{}",
      conditions.join(""),
      match self.alternative.clone() {
        Some(alternative) => format!(" else {}", alternative.string()),
        None => String::new(),
      },
    )
  }
}

impl IfElse {
  pub fn get_conditions(&self) -> Vec<IfElseCondition> {
    self.conditions.clone()
  }

  pub fn get_alternative(&self) -> Option<Box<Statements>> {
    self.alternative.clone()
  }

  pub fn parse<'a>(
    parser: &'a mut Parser,
    standard_library: bool,
    with_this: bool,
  ) -> Result<Box<Statements>, Error> {
    let mut if_else: Self = Statement::from_token(parser.get_current_token());

    while !parser.current_token_is(Signs::new(Signs::RIGHTBRACE)) ||
      parser.next_token_is(Keywords::new(Keywords::ELSE)) {
      // Check if the current token is right brace.
      if parser.current_token_is(Signs::new(Signs::RIGHTBRACE)) {
        // Get the next token.
        parser.next_token();
      }

      // Check if the current token is an `if` or `else if`.
      if parser.current_token_is(Keywords::new(Keywords::IF)) || (
        parser.current_token_is(Keywords::new(Keywords::ELSE)) &&
        parser.next_token_is(Keywords::new(Keywords::IF))
      ) {
        match IfElseCondition::parse(parser, standard_library, with_this) {
          Ok(condition) => {
            // Add the condition to the if else.
            if_else.conditions.push(condition);

            continue;
          },
          Err(error) => {
            return Err(error);
          },
        }
      }

      // Check if the current token is an `else`.
      if parser.current_token_is(Keywords::new(Keywords::ELSE)) {
        // Check if the next token is an left brace.
        if !parser.expect_token(Signs::new(Signs::LEFTBRACE)) {
          return Err(Error::from_token(
            format!("expect `{{`, got `{}` instead.", parser.get_next_token().value),
            parser.get_next_token(),
          ));
        }

        // Parse else body.
        match Block::parse(parser, standard_library, false, with_this) {
          Ok(body) => {
            // Set the else body to if else alternative.
            if_else.alternative = Some(body);
          },
          Err(error) => {
            return Err(error);
          },
        }

        continue;
      }
    }

    Ok(Box::new(Statements::IFELSE(if_else)))
  }
}

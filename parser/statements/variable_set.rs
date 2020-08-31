use crate::{
  ArrayIndex,
  Error,
  Expressions,
  Number,
  parse_expression,
  Parser,
  Precedence,
  tokens::{
    Signs,
    Token,
    Tokens,
  }
};

use super::{
  Statement,
  Statements,
};

#[derive(Debug, Clone, PartialEq)]
pub struct VariableSet {
  pub this: Option<Token>,
  pub token: Token,
  pub array_position: Option<Box<Expressions>>,
  pub assign: Token,
  pub value: Box<Expressions>,
}

impl Statement for VariableSet {
  fn new() -> VariableSet {
    VariableSet {
      this: None,
      token: Token::new_empty(),
      array_position: None,
      assign: Token::new_empty(),
      value: Number::new_box(),
    }
  }

  fn from_token(token: Token) -> VariableSet {
    let mut variable: VariableSet = Statement::new();

    variable.token = token;

    variable
  }

  fn string(self) -> String {
    if self.assign.token.clone().expect_sign(Signs::PLUSPLUS) ||
      self.assign.token.clone().expect_sign(Signs::MINUSMINUS) {
      return format!(
        "{}{}{};",
        match self.this {
          Some(this) => format!("{}.", this.value),
          None => String::new(),
        },
        self.token.value,
        self.assign.value,
      );
    }

    format!(
      "{} {} {};",
      format!(
        "{}{}{}",
        match self.this {
          Some(this) => format!("{}.", this.value),
          None => String::new(),
        },
        self.token.value,
        match self.array_position {
          Some(position) => format!("[{}]", position.string()),
          None => String::new(),
        },
      ),
      self.assign.value,
      self.value.string(),
    )
  }
}

impl VariableSet {
  pub fn parse<'a>(
    parser: &'a mut Parser,
    standard_library: bool,
    this: Option<Token>,
    with_this: bool,
  ) -> Result<Box<Statements>, Error> {
    let mut variable: VariableSet = Statement::from_token(parser.current_token.clone());

    // Set the variable this.
    variable.this = this;

    // Parse (identifier)[position]
    if parser.expect_token(Signs::new(Signs::LEFTBRACKET)) {
      // Get the next token.
      parser.next_token();

      // Parse expression.
      match parse_expression(parser, Precedence::LOWEST, standard_library, with_this) {
        Ok(array_position) => {
          variable.array_position = Some(array_position.clone());

          // Parse array position.
          if let Err(error) = ArrayIndex::parse_index(array_position) {
            return Err(error);
          }
        },
        Err(error) => {
          return Err(error);
        },
      }

      // Check if the next token is a right bracket.
      if !parser.expect_token(Signs::new(Signs::RIGHTBRACKET)) {
        return Err(Error::from_token(
          format!("expect `]`, got `{}` instead.", parser.next_token.value.clone()),
          parser.next_token.clone(),
        ));
      }
    }

    // Parse assigns signs.
    if parser.expect_token(Signs::new(Signs::ASSIGN)) ||
      parser.expect_token(Signs::new(Signs::PLUSASSIGN)) ||
      parser.expect_token(Signs::new(Signs::MINUSASSIGN)) ||
      parser.expect_token(Signs::new(Signs::MULTIPLYASSIGN)) ||
      parser.expect_token(Signs::new(Signs::DIVIDEASSIGN)) {
      // Set the variable assign token.
      variable.assign = parser.current_token.clone();

      // Get the next token.
      parser.next_token();

      // Parse the value expression.
      match parse_expression(parser, Precedence::LOWEST, standard_library, with_this) {
        Ok(value) => {
          variable.value = value;
        },
        Err(error) => {
          return Err(error);
        },
      }
    }

    // Parse ++ and -- signs.
    else if parser.expect_token(Signs::new(Signs::PLUSPLUS)) ||
      parser.expect_token(Signs::new(Signs::MINUSMINUS)) {
      // Set the variable assign token.
      variable.assign = parser.current_token.clone();

      // Get the next token.
      parser.next_token();

      // Set a one value.
      variable.value = Number::new_box_from_token(
        Token::new(Box::new(Tokens::NUMBER), String::from("1"), 0, 0)
      );
    }

    // Check if the next token is a semicolon.
    if parser.next_token_is(Signs::new(Signs::SEMICOLON)) {
      // Get the next token.
      parser.next_token();
    }

    // Return variable set statement.
    Ok(Box::new(Statements::VARIABLESET(variable)))
  }
}

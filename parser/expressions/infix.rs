use crate::{
  Error,
  Expression,
  Expressions,
  Identifier,
  parse_expression,
  parse_type,
  Parser,
  tokens::{
    Keywords,
    Signs,
    Token,
  },
};

#[derive(Debug, Clone, PartialEq)]
pub enum InfixType {
  INFIX,
  ALIAS,
  METHOD,
  IS,
  VARIABLESET,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Infix {
  token: Token,
  itype: InfixType,
  left: Box<Expressions>,
  right: Option<Box<Expressions>>,
  right_type: Option<Token>,
}

impl Expression for Infix {
  fn new() -> Self {
    Self {
      token: Token::new_empty(),
      itype: InfixType::INFIX,
      left: Identifier::new_box(),
      right: None,
      right_type: None,
    }
  }

  fn from_token(token: Token) -> Self {
    let mut infix: Self = Expression::new();

    infix.token = token;

    infix
  }

  fn get_token(&self) -> Token {
    self.token.clone()
  }

  fn string(&self) -> String {
    let whitespace = if self.is_method() || self.is_type() { "" } else { " " };

    format!(
      "{}{}{}{}{}",
      self.get_left().string(),
      whitespace,
      self.get_token().value,
      whitespace,
      match self.get_right() {
        Some(right) => right.string(),
        None => match self.get_right_type() {
          Some(right_type) => right_type.value,
          None => String::new(),
        },
      },
    )
  }
}

impl Infix {
  pub fn new_box() -> Box<Expressions> {
    Box::new(Expressions::INFIX(Expression::new()))
  }

  pub fn new_box_from_token(token: Token) -> Box<Expressions> {
    Box::new(Expressions::INFIX(Expression::from_token(token)))
  }

  pub fn is_infix(&self) -> bool {
    self.itype == InfixType::INFIX
  }

  pub fn is_alias(&self) -> bool {
    self.itype == InfixType::ALIAS
  }

  pub fn is_method(&self) -> bool {
    self.itype == InfixType::METHOD
  }

  pub fn is_type(&self) -> bool {
    self.itype == InfixType::IS
  }

  pub fn is_variable_set(&self) -> bool {
    self.itype == InfixType::VARIABLESET
  }

  pub fn get_left(&self) -> Box<Expressions> {
    self.left.clone()
  }

  pub fn get_right(&self) -> Option<Box<Expressions>> {
    self.right.clone()
  }

  pub fn get_right_type(&self) -> Option<Token> {
    self.right_type.clone()
  }

  pub fn parse<'a>(
    parser: &'a mut Parser,
    left_expression: Box<Expressions>,
    standard_library: bool,
    with_this: bool,
  ) -> Result<Box<Expressions>, Error> {
    let mut infix: Infix = Expression::from_token(parser.get_current_token());

    // Check if it is an alias expression.
    if parser.current_token_is(Keywords::new(Keywords::AS)) {
      infix.itype = InfixType::ALIAS;
    }
    // Check if it is a method expression.
    else if parser.current_token_is(Signs::new(Signs::ARROW)) {
      infix.itype = InfixType::METHOD;
    }
    // Check if it is 'is' expression.
    else if parser.current_token_is(Keywords::new(Keywords::IS)) {
      infix.itype = InfixType::IS;
    }
    // Check if it is a variable set expression.
    else if parser.current_token_is(Signs::new(Signs::ASSIGN)) ||
      parser.current_token_is(Signs::new(Signs::PLUSASSIGN)) ||
      parser.current_token_is(Signs::new(Signs::MINUSASSIGN)) ||
      parser.current_token_is(Signs::new(Signs::MULTIPLYASSIGN)) ||
      parser.current_token_is(Signs::new(Signs::DIVIDEASSIGN)) {
      infix.itype = InfixType::VARIABLESET;
    }

    // Set the left expression.
    infix.left = left_expression;

    // Get the current precedence.
    let precedence = parser.current_precedence();

    // Get the next token.
    parser.next_token();

    if infix.is_type() {
      // Parse the right type.
      match parse_type(parser) {
        Ok(right_type) => {
          // Set the right type.
          infix.right_type = Some(right_type);
        },
        Err(_) => {
          return Err(Error::from_token(
            format!("`{}` is not a valid data type.", parser.get_current_token().value),
            parser.get_current_token(),
          ));
        },
      }
    } else {
      // Parse the right expression.
      match parse_expression(parser, precedence, standard_library, with_this) {
        Ok(right) => {
          // Set the right expression.
          infix.right = Some(right);
        },
        Err(error) => {
          return Err(error);
        },
      }

      if infix.is_variable_set() && parser.next_token_is(Signs::new(Signs::SEMICOLON)) {
        // Get the next token.
        parser.next_token();
      }
    }

    // Return the infix expression.
    Ok(Box::new(Expressions::INFIX(infix)))
  }
}

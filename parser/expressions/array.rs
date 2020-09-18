use crate::{
  Error,
  Parser,
  Precedence,
  tokens::{
    Signs,
    Token,
  },
};

use super::{
  Expression,
  Expressions,
  Number,
  parse_expression,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Array {
  token: Token,
  data: Vec<Box<Expressions>>,
}

impl Expression for Array {
  fn new() -> Self {
    Self {
      token: Token::new_empty(),
      data: Vec::new(),
    }
  }

  fn from_token(token: Token) -> Self {
    let mut array: Self = Expression::new();

    array.token = token;

    array
  }

  fn get_token(&self) -> Token {
    self.token.clone()
  }

  fn string(&self) -> String {
    let mut data: Vec<String> = Vec::new();

    for expression in self.get_data().iter() {
      data.push(expression.string());
    }

    format!("[{}]", data.join(", "))
  }
}

impl Array {
  pub fn get_data(&self) -> Vec<Box<Expressions>> {
    self.data.clone()
  }

  pub fn parse<'a>(
    parser: &'a mut Parser,
    standard_library: bool,
    with_this: bool,
  ) -> Result<Box<Expressions>, Error> {
    let mut array: Array = Expression::from_token(parser.get_current_token());

    // Get the next token.
    parser.next_token();

    while !parser.current_token_is(Signs::new(Signs::RIGHTBRACKET)) {
      // Parse expression.
      match parse_expression(parser, Precedence::LOWEST, standard_library, with_this) {
        Ok(expression) => {
          array.data.push(expression);
        },
        Err(error) => {
          return Err(error);
        },
      }

      // Check if the next token is a comma.
      if parser.next_token_is(Signs::new(Signs::COMMA)) {
        // Get the next token.
        parser.next_token();
      }

      // Get the next token.
      parser.next_token();
    }

    // Return the array expression.
    Ok(Box::new(Expressions::ARRAY(array)))
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ArrayIndex {
  token: Token,
  left: Option<Box<Expressions>>,
  index: Box<Expressions>,
}

impl Expression for ArrayIndex {
  fn new() -> ArrayIndex {
    ArrayIndex {
      token: Token::new_empty(),
      left: None,
      index: Number::new_box(),
    }
  }

  fn from_token(token: Token) -> ArrayIndex {
    let mut array_index: ArrayIndex = Expression::new();

    array_index.token = token;

    array_index
  }

  fn get_token(&self) -> Token {
    self.token.clone()
  }

  fn string(&self) -> String {
    format!(
      "{}[{}]",
      self.get_token().value,
      self.get_index().string(),
    )
  }
}

impl ArrayIndex {
  pub fn get_left(&self) -> Option<Box<Expressions>> {
    self.left.clone()
  }

  pub fn get_index(&self) -> Box<Expressions> {
    self.index.clone()
  }

  pub fn parse_index<'a>(
    index: Box<Expressions>,
  ) -> Result<(), Error> {
    // Check if the expression has a dot.
    if index.clone().string().contains('.') {
      return Err(Error::from_token(
        String::from("the index value can not contains a dot."),
        index.clone().token(),
      ));
    }

    // Get the prefix expression.
    if let Some(prefix) = index.clone().get_prefix() {
      if prefix.get_right().get_number().unwrap().get_value() != 1.0 {
        return Err(Error::from_token(
          String::from("the index can not be other than '-1' or a positive number."),
          index.clone().token(),
        ));
      }
    }

    Ok(())
  }

  pub fn parse<'a>(
    parser: &'a mut Parser,
    left_exp: Result<Box<Expressions>, Error>,
    standard_library: bool,
    with_this: bool,
  ) -> Result<Box<Expressions>, Error> {
    let mut array_index: ArrayIndex = Expression::from_token(parser.get_current_token());

    if let Ok(left_exp) = left_exp {
      array_index.left = Some(left_exp);
    }

    // Check if the next token is a left bracket.
    if !parser.expect_token(Signs::new(Signs::LEFTBRACKET)) {
      return Err(Error::from_token(
        format!("expect `[`, got `{}` instead.", parser.get_next_token().value),
        parser.get_next_token(),
      ));
    }

    // Get the next token.
    parser.next_token();

    // Parse expression.
    match parse_expression(parser, Precedence::LOWEST, standard_library, with_this) {
      Ok(index) => {
        array_index.index = index;
      },
      Err(error) => {
        return Err(error);
      },
    }

    // Parse array index.
    if let Err(error) = ArrayIndex::parse_index(array_index.index.clone()) {
      return Err(error);
    }

    // Check if the next token is a right bracket.
    if !parser.expect_token(Signs::new(Signs::RIGHTBRACKET)) {
      return Err(Error::from_token(
        format!("expect `]`, got `{}` instead.", parser.get_next_token().value),
        parser.get_next_token(),
      ));
    }

    // Return the box expression.
    Ok(Box::new(Expressions::ARRAYINDEX(array_index)))
  }
}

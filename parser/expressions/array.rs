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
  pub token: Token,
  pub data: Vec<Box<Expressions>>,
}

impl Expression for Array {
  fn new() -> Array {
    Array {
      token: Token::new_empty(),
      data: Vec::new(),
    }
  }

  fn from_token(token: Token) -> Array {
    let mut array: Array = Expression::new();

    array.token = token;

    array
  }

  fn string(self) -> String {
    let mut data: Vec<String> = Vec::new();

    for expression in self.data {
      data.push(expression.string());
    }

    format!("[{}]", data.join(", "))
  }
}

impl Array {
  pub fn parse<'a>(
    parser: &'a mut Parser,
    standard_library: bool,
    with_this: bool,
  ) -> Result<Box<Expressions>, Error> {
    let mut array: Array = Expression::from_token(parser.current_token.clone());

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

    // Check if the next token is a right bracket.
    if parser.current_token_is(Signs::new(Signs::RIGHTBRACKET)) {
      // Get the next token.
      parser.next_token();
    }

    // Return the array expression.
    Ok(Box::new(Expressions::ARRAY(array)))
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ArrayIndex {
  pub token: Token,
  pub index: Box<Expressions>,
}

impl Expression for ArrayIndex {
  fn new() -> ArrayIndex {
    ArrayIndex {
      token: Token::new_empty(),
      index: Number::new_box(),
    }
  }

  fn from_token(token: Token) -> ArrayIndex {
    let mut array_index: ArrayIndex = Expression::new();

    array_index.token = token;

    array_index
  }

  fn string(self) -> String {
    format!(
      "{}[{}]",
      self.token.value,
      self.index.string(),
    )
  }
}

impl ArrayIndex {
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
      if prefix.right.get_number().unwrap().value != 1.0 {
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
    standard_library: bool,
    with_this: bool,
  ) -> Result<Box<Expressions>, Error> {
    let mut array_index: ArrayIndex = Expression::from_token(parser.current_token.clone());

    // Check if the next token is a left bracket.
    if !parser.expect_token(Signs::new(Signs::LEFTBRACKET)) {
      return Err(Error::from_token(
        format!("expect `[`, got `{}` instead.", parser.next_token.value.clone()),
        parser.next_token.clone(),
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
        format!("expect `]`, got `{}` instead.", parser.next_token.value.clone()),
        parser.next_token.clone(),
      ));
    }

    // Return the box expression.
    Ok(Box::new(Expressions::ARRAYINDEX(array_index)))
  }
}

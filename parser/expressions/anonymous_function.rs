use crate::{
  Block,
  Error,
  Statements,
  Parser,
  tokens::{
    Keywords,
    Signs,
    Token,
  },
};

use super::{
  Argument,
  Expression,
  Expressions,
  parse_type,
};

#[derive(Debug, Clone, PartialEq)]
pub struct AnonymousFunction {
  token: Token,
  arguments: Vec<Box<Expressions>>,
  data_type: Token,
  body: Box<Statements>,
}

impl Expression for AnonymousFunction {
  fn new() -> Self {
    Self {
      token: Token::new_empty(),
      arguments: Vec::new(),
      data_type: Token::from_value("any", 0, 0),
      body: Block::new_box(),
    }
  }

  fn from_token(token: Token) -> Self {
    let mut function: Self = Expression::new();

    function.token = token;

    function
  }

  fn get_token(&self) -> Token {
    self.token.clone()
  }

  fn string(&self) -> String {
    let mut arguments: Vec<String> = Vec::new();

    for argument in self.get_arguments().iter() {
      arguments.push(argument.string());
    }

    let function = format!(
      "({}): {}",
      arguments.join(", "),
      self.get_type().value,
    );

    let body = self.get_body().string();

    if self.get_token().token.expect_keyword(&Keywords::FUNCTION) {
      return format!("{} {} {}", self.get_token().value, function, body);
    }

    format!("{} => {}", function, body)
  }
}

impl AnonymousFunction {
  pub fn get_arguments(&self) -> Vec<Box<Expressions>> {
    self.arguments.clone()
  }

  pub fn get_type(&self) -> Token {
    self.data_type.clone()
  }

  pub fn get_body(&self) -> Box<Statements> {
    self.body.clone()
  }

  pub fn parse<'a>(
    parser: &'a mut Parser,
    standard_library: bool,
    with_this: bool,
  ) -> Result<Box<Expressions>, Error> {
    let mut function: AnonymousFunction = Expression::from_token(parser.get_current_token());

    // Check if the current token is a left parentheses.
    if !parser.current_token_is(Signs::new(Signs::LEFTPARENTHESES)) {
      // Get the next token.
      parser.next_token();
    }

    // Parse arguments.
    match Argument::parse(parser, standard_library, with_this) {
      Ok(arguments) => {
        function.arguments = arguments;
      },
      Err(error) => {
        return Err(error);
      },
    }

    // Check if the current token is a right parentheses.
    if parser.current_token_is(Signs::new(Signs::RIGHTPARENTHESES)) {
      // Get the next token.
      parser.next_token();
    }

    // Check if the current token is a colon.
    if parser.current_token_is(Signs::new(Signs::COLON)) {
      // Get the next token.
      parser.next_token();

      // Get the return data type.
      match parse_type(parser) {
        Ok(data_type) => {
          // Set the function return data type.
          function.data_type = data_type;
        },
        Err(_) => {
          return Err(Error::from_token(
            String::from("is not a valid type."),
            parser.get_current_token(),
          ));
        },
      }

      // Get the next token.
      parser.next_token();
    }

    // Check if the function token is a left parentheses.
    if function.token.token.expect_sign(&Signs::LEFTPARENTHESES) {
      // Check if the next token is an assign arrow sign.
      if !parser.current_token_is(Signs::new(Signs::ASSIGNARROW)) {
        return Err(Error::from_token(
          format!("expect `=>`, got `{}` instead.", parser.get_current_token().value),
          parser.get_current_token(),
        ));
      }

      // Get the next token.
      parser.next_token();
    }

    // Check if the next token is a left brace.
    if !parser.current_token_is(Signs::new(Signs::LEFTBRACE)) {
      return Err(Error::from_token(
        format!("expect `{{`, got `{}` instead.", parser.get_current_token().value),
        parser.get_current_token(),
      ));
    }

    // Parse body.
    match Block::parse(parser, standard_library, false, with_this) {
      Ok(body) => {
        function.body = body;
      },
      Err(error) => {
        return Err(error);
      },
    }

    Ok(Box::new(Expressions::ANONYMOUSFUNCTION(function)))
  }
}

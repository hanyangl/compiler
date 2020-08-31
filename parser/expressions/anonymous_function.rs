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
  pub token: Token,
  pub arguments: Vec<Box<Expressions>>,
  pub data_type: Token,
  pub body: Box<Statements>,
}

impl Expression for AnonymousFunction {
  fn new() -> AnonymousFunction {
    AnonymousFunction {
      token: Token::new_empty(),
      arguments: Vec::new(),
      data_type: Token::from_value("any", 0, 0),
      body: Block::new_box(),
    }
  }

  fn from_token(token: Token) -> AnonymousFunction {
    let mut function: AnonymousFunction = Expression::new();

    function.token = token;

    function
  }

  fn string(self) -> String {
    let mut arguments: Vec<String> = Vec::new();

    for argument in self.arguments {
      arguments.push(argument.string());
    }

    let function = format!(
      "({}): {}",
      arguments.join(", "),
      self.data_type.value,
    );

    if self.token.token.clone().expect_keyword(Keywords::FUNCTION) {
      return format!("{} {} {}", self.token.value, function, self.body.string());
    }

    format!("{} => {}", function, self.body.string())
  }
}

impl AnonymousFunction {
  pub fn parse<'a>(
    parser: &'a mut Parser,
    standard_library: bool,
    with_this: bool,
  ) -> Result<Box<Expressions>, Error> {
    let mut function: AnonymousFunction = Expression::from_token(parser.current_token.clone());

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
      match parse_type(parser, false) {
        Ok(data_type) => {
          // Set the function return data type.
          function.data_type = data_type;
        },
        Err(_) => {
          return Err(Error::from_token(
            String::from("is not a valid type."),
            parser.current_token.clone(),
          ));
        },
      }

      // Get the next token.
      parser.next_token();
    }

    // Check if the function token is a left parentheses.
    if function.token.token.clone().expect_sign(Signs::LEFTPARENTHESES) {
      // Check if the next token is an assign arrow sign.
      if !parser.current_token_is(Signs::new(Signs::ASSIGNARROW)) {
        return Err(Error::from_token(
          format!("expect `=>`, got `{}` instead.", parser.current_token.value.clone()),
          parser.current_token.clone(),
        ));
      }

      // Get the next token.
      parser.next_token();
    }

    // Check if the next token is a left brace.
    if !parser.current_token_is(Signs::new(Signs::LEFTBRACE)) {
      return Err(Error::from_token(
        format!("expect `{{`, got `{}` instead.", parser.current_token.value.clone()),
        parser.current_token.clone(),
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

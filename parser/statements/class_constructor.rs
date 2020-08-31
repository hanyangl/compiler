use crate::{
  Error,
  Expressions,
  parse_expression,
  parse_type,
  Parser,
  Precedence,
  tokens::{
    Keywords,
    Signs,
    Token,
    Tokens,
  },
};

use super::{
  Block,
  Statement,
  Statements,
};

#[derive(Debug, Clone, PartialEq)]
pub struct ConstructorArgument {
  pub primary_type: Option<Token>,            // public, private or protected.
  pub secondary_type: Option<Token>,          // readonly
  pub name: Token,
  pub data_type: Token,
  pub value: Option<Box<Expressions>>,
}

impl ConstructorArgument {
  pub fn new() -> ConstructorArgument {
    ConstructorArgument {
      primary_type: None,
      secondary_type: None,
      name: Token::new_empty(),
      data_type: Token::from_value("any", 0, 0),
      value: None,
    }
  }

  pub fn string(self) -> String {
    let mut types: String = String::new();

    if let Some(primary_type) = self.primary_type {
      types.push_str(primary_type.value.as_str());
      types.push_str(" ");
    }

    if let Some(secondary_type) = self.secondary_type {
      types.push_str(secondary_type.value.as_str());
      types.push_str(" ");
    }

    let mut value: String = String::new();

    if let Some(default_value) = self.value {
      value.push_str(" = ");
      value.push_str(default_value.string().as_str());
    }

    format!(
      "{}{}: {}{}",
      types,
      self.name.value,
      self.data_type.value,
      value,
    )
  }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ClassConstructor {
  pub token: Token,
  pub arguments: Vec<ConstructorArgument>,
  pub body: Box<Statements>,
}

impl Statement for ClassConstructor {
  fn new() -> ClassConstructor {
    ClassConstructor {
      token: Token::new_empty(),
      arguments: Vec::new(),
      body: Block::new_box(),
    }
  }

  fn from_token(token: Token) -> ClassConstructor {
    let mut class_constructor: ClassConstructor = Statement::new();

    class_constructor.token = token;

    class_constructor
  }

  fn string(self) -> String {
    let mut arguments: Vec<String> = Vec::new();

    for argument in self.arguments {
      arguments.push(argument.string());
    }

    format!(
      "{} ({}) {}",
      self.token.value,
      arguments.join(", "),
      self.body.string(),
    )
  }
}

impl ClassConstructor {
  pub fn parse<'a>(
    parser: &'a mut Parser,
    standard_library: bool,
    with_this: bool,
  ) -> Result<Box<Statements>, Error> {
    let mut class_constructor: ClassConstructor = Statement::from_token(parser.current_token.clone());

    // Check if the next token is a left parentheses.
    if !parser.expect_token(Signs::new(Signs::LEFTPARENTHESES)) {
      return Err(Error::from_token(
        format!("expect `(`, got `{}` instead.", parser.next_token.value.clone()),
        parser.next_token.clone(),
      ));
    }

    // Get the next token.
    parser.next_token();

    // Parse constructor arguments.
    while !parser.current_token_is(Signs::new(Signs::RIGHTPARENTHESES)) {
      // Create a new constructor argument.
      let mut argument: ConstructorArgument = ConstructorArgument::new();

      // Check if the current token is `public`, `private` or `protected`.
      if parser.current_token_is(Keywords::new(Keywords::PUBLIC)) ||
        parser.current_token_is(Keywords::new(Keywords::PRIVATE)) ||
        parser.current_token_is(Keywords::new(Keywords::PROTECTED)) {
        // Set the argument primary type.
        argument.primary_type = Some(parser.current_token.clone());

        // Get the next token.
        parser.next_token();
      }

      // Check if the current token is `readonly`.
      if parser.current_token_is(Keywords::new(Keywords::READONLY)) {
        // Set the argument secondary type.
        argument.secondary_type = Some(parser.current_token.clone());
        
        // Get the next token.
        parser.next_token();
      }

      // Check if the current token is an identifier.
      if !parser.current_token_is(Box::new(Tokens::IDENTIFIER)) {
        return Err(Error::from_token(
          format!("`{}` is not a valid identifier.", parser.current_token.value.clone()),
          parser.current_token.clone(),
        ));
      }

      // Set the argument name.
      argument.name = parser.current_token.clone();

      // Check if the next token is a colon.
      if !parser.expect_token(Signs::new(Signs::COLON)) {
        return Err(Error::from_token(
          format!("expect `:`, got `{}` instead.", parser.next_token.value.clone()),
          parser.next_token.clone(),
        ));
      }

      // Get the next token.
      parser.next_token();

      // Parse argument data type.
      match parse_type(parser, false) {
        Ok(data_type) => {
          // Set the argument data type.
          argument.data_type = data_type;
        },
        Err(_) => {
          return Err(Error::from_token(
            format!("`{}` is not a valid data type.", parser.current_token.value.clone()),
            parser.current_token.clone(),
          ));
        },
      }

      // Check if the next token is an assign sign.
      if parser.expect_token(Signs::new(Signs::ASSIGN)) {
        // Get the next token.
        parser.next_token();

        // Parse value expression.
        match parse_expression(parser, Precedence::LOWEST, standard_library, with_this) {
          Ok(expression) => {
            // Set the argument value.
            argument.value = Some(expression);
          },
          Err(error) => {
            return Err(error);
          },
        }
      }

      // Check if the next token is a comma.
      if parser.next_token_is(Signs::new(Signs::COMMA)) {
        // Get the next token.
        parser.next_token();
      }

      // Add the argument to the class constructor.
      class_constructor.arguments.push(argument);

      // Get the next token.
      parser.next_token();
    }

    // Check if the next token is a left brace.
    if !parser.expect_token(Signs::new(Signs::LEFTBRACE)) {
      return Err(Error::from_token(
        format!("expect `{{`, got `{}` instead.", parser.next_token.value.clone()),
        parser.next_token.clone(),
      ));
    }

    // Parse constructor body.
    match Block::parse(parser, standard_library, false, true) {
      Ok(body) => {
        // Set the constructor body.
        class_constructor.body = body;
      },
      Err(error) => {
        return Err(error);
      },
    }

    Ok(Box::new(Statements::CLASSCONSTRUCTOR(class_constructor)))
  }
}

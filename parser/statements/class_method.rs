use crate::{
  Argument,
  Error,
  parse_expression,
  parse_type,
  Parser,
  Precedence,
  Expressions,
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
pub struct ClassMethod {
  pub primary_type: Token,                // public, private, protected or internal (Default: public)
  pub secondary_type: Option<Token>,      // abstract or readonly.
  pub token: Token,                       // Method name.

  pub is_function: bool,
  pub arguments: Vec<Box<Expressions>>,   // Function arguments.
  pub body: Option<Box<Statements>>,      // Function body.

  pub data_type: Token,                   // Return data type.

  pub value: Option<Box<Expressions>>,    // Variable value.
}

impl Statement for ClassMethod {
  fn new() -> ClassMethod {
    ClassMethod {
      primary_type: Token::from_value("public", 0, 0),
      secondary_type: None,
      token: Token::new_empty(),

      is_function: false,
      arguments: Vec::new(),
      body: None,

      data_type: Token::from_value("any", 0, 0),

      value: None,
    }
  }

  fn from_token(token: Token) -> ClassMethod {
    let mut class_method: ClassMethod = Statement::new();

    class_method.token = token;

    class_method
  }

  fn string(self) -> String {
    let mut types: String = self.primary_type.value;
    let mut is_abstract_or_internal = false;

    if let Some(secondary_type) = self.secondary_type {
      types.push_str(" ");
      types.push_str(secondary_type.value.as_str());

      is_abstract_or_internal = secondary_type.token.clone().expect_keyword(Keywords::ABSTRACT);

      if !is_abstract_or_internal {
        is_abstract_or_internal = secondary_type.token.clone().expect_keyword(Keywords::INTERNAL);
      }
    }

    if self.is_function {
      format!(
        "{} {}(): {}{}",
        types,
        self.token.value,
        self.data_type.value,
        match self.body {
          Some(body) => format!(" {}", body.string()),
          None => String::from(if is_abstract_or_internal { ";" } else { " { }" }),
        },
      )
    } else {
      let mut value: String = String::new();

      if let Some(default_value) = self.value {
        value.push_str(" = ");
        value.push_str(default_value.string().as_str());
      }

      format!(
        "{} {}: {}{};",
        types,
        self.token.value,
        self.data_type.value,
        value,
      )
    }
  }
}

impl ClassMethod {
  pub fn parse<'a>(
    parser: &'a mut Parser,
    standard_library: bool,
    primary_type: Option<Token>,
    with_this: bool,
  ) -> Result<Box<Statements>, Error> {
    let mut class_method: ClassMethod = Statement::new();

    // Set the primary class method type.
    if let Some(primary_type) = primary_type {
      class_method.primary_type = primary_type;
    }

    // Check if the current token is `public`, `private`, `protected` or `internal`.
    if parser.current_token_is(Keywords::new(Keywords::PUBLIC)) ||
      parser.current_token_is(Keywords::new(Keywords::PRIVATE)) ||
      parser.current_token_is(Keywords::new(Keywords::PROTECTED)) ||
      (standard_library && parser.current_token_is(Keywords::new(Keywords::INTERNAL))) {
      // Set the primary class type.
      class_method.primary_type = parser.current_token.clone();

      // Get the next token.
      parser.next_token();
    }

    // Check if the current token is `readonly` or `abstract`.
    if parser.current_token_is(Keywords::new(Keywords::READONLY)) || 
      parser.current_token_is(Keywords::new(Keywords::ABSTRACT)) {
      // Set the secondary class type.
      class_method.secondary_type = Some(parser.current_token.clone());

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

    // Set the class method name.
    class_method.token = parser.current_token.clone();

    // Check if the next token is a colon.
    if parser.expect_token(Signs::new(Signs::COLON)) {
      // Get the next token.
      parser.next_token();

      // Parse data type.
      match parse_type(parser, false) {
        Ok(data_type) => {
          // Set the class method data type.
          class_method.data_type = data_type;
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

        // Parse class method value.
        match parse_expression(parser, Precedence::LOWEST, standard_library, with_this) {
          Ok(expression) => {
            // Set the class method value.
            class_method.value = Some(expression);
          },
          Err(error) => {
            return Err(error);
          },
        }
      }

      // Check if the next token is a semicolon.
      if parser.next_token_is(Signs::new(Signs::SEMICOLON)) {
        // Get the next token.
        parser.next_token();
      }
    }
    // Check if the next token is a left parentheses.
    else if parser.expect_token(Signs::new(Signs::LEFTPARENTHESES)) {
      // Set the class method as a function.
      class_method.is_function = true;

      // Parse arguments.
      match Argument::parse(parser, standard_library, with_this) {
        Ok(arguments) => {
          // Set the class method arguments.
          class_method.arguments = arguments;
        },
        Err(error) => {
          return Err(error);
        },
      }

      // Check if the next token is a colon.
      if parser.expect_token(Signs::new(Signs::COLON)) {
        // Get the next token.
        parser.next_token();

        // Parse data type.
        match parse_type(parser, false) {
          Ok(data_type) => {
            // Set the class method data type.
            class_method.data_type = data_type;
          },
          Err(_) => {
            return Err(Error::from_token(
              format!("`{}` is not a valid data type.", parser.current_token.value.clone()),
              parser.current_token.clone(),
            ));
          },
        }
      } else {
        // Set the class method data type to void.
        class_method.data_type = Token::from_value("void", 0, 0);
      }

      let mut is_abstract_or_internal = false;

      if let Some(secondary_type) = class_method.secondary_type.clone() {
        is_abstract_or_internal = secondary_type.token.clone().expect_keyword(Keywords::ABSTRACT);

        if !is_abstract_or_internal {
          is_abstract_or_internal = secondary_type.token.clone().expect_keyword(Keywords::INTERNAL);
        }
      }

      if is_abstract_or_internal {
        // Check if the next token is a semicolon.
        if parser.next_token_is(Signs::new(Signs::SEMICOLON)) {
          // Get the next token.
          parser.next_token();
        }
      } else {
        // Check if the next token is a left brace.
        if !parser.expect_token(Signs::new(Signs::LEFTBRACE)) {
          return Err(Error::from_token(
            format!("expect `{{`, got `{}` instead.", parser.next_token.value.clone()),
            parser.next_token.clone(),
          ));
        }

        // Parse function body.
        match Block::parse(parser, standard_library, false, true) {
          Ok(body) => {
            // Set the class method body.
            class_method.body = Some(body);
          },
          Err(error) => {
            return Err(error);
          },
        }
      }
    }

    Ok(Box::new(Statements::CLASSMETHOD(class_method)))
  }
}

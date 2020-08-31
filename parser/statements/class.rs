use crate::{
  Error,
  Expressions,
  parse_expression,
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
  ClassConstructor,
  ClassMethod,
  Statement,
  Statements,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Class {
  pub primary_type: Token,                        // public, private or internal (Default public)
  pub secondary_type: Option<Token>,              // abstract
  pub token: Token,                               // class token.
  pub name: Token,

  pub extends: Vec<Box<Expressions>>,
  pub implements: Vec<Box<Expressions>>,

  pub constructor: Option<ClassConstructor>,
  pub methods: Vec<ClassMethod>,
}

impl Statement for Class {
  fn new() -> Class {
    Class {
      primary_type: Token::from_value("public", 0, 0),
      secondary_type: None,
      token: Token::new_empty(),
      name: Token::new_empty(),

      extends: Vec::new(),
      implements: Vec::new(),

      constructor: None,
      methods: Vec::new(),
    }
  }

  fn from_token(token: Token) -> Class {
    let mut class: Class = Statement::new();

    class.token = token;

    class
  }

  fn string(self) -> String {
    let mut types = self.primary_type.value;

    if let Some(secondary_type) = self.secondary_type {
      types.push_str(" ");
      types.push_str(secondary_type.value.as_str());
    }

    let mut extends: Vec<String> = Vec::new();

    for extend in self.extends {
      extends.push(extend.string());
    }

    let mut implements: Vec<String> = Vec::new();

    for implement in self.implements {
      implements.push(implement.string());
    }

    let mut methods: Vec<String> = Vec::new();

    for method in self.methods {
      methods.push(method.string());
    }

    format!(
      "{} {} {}{}{} {{\n{}{}\n}}",
      types,
      self.token.value,
      self.name.value,
      if extends.len() > 0 {
        format!(" extends {}", extends.join(", "))
      } else {
        String::new()
      },
      if implements.len() > 0 {
        format!(" implements {}", implements.join(", "))
      } else {
        String::new()
      },
      match self.constructor {
        Some(constructor) => format!("{}\n", constructor.string()),
        None => String::new(),
      },
      methods.join("\n"),
    )
  }
}

impl Class {
  pub fn parse<'a>(
    parser: &'a mut Parser,
    standard_library: bool,
    with_this: bool,
  ) -> Result<Box<Statements>, Error> {
    let mut class: Class = Statement::new();

    // Check if the current token is `public`, `private` or `internal`.
    if parser.current_token_is(Keywords::new(Keywords::PUBLIC)) ||
      parser.current_token_is(Keywords::new(Keywords::PRIVATE)) ||
      (standard_library && parser.current_token_is(Keywords::new(Keywords::INTERNAL))) {
      // Set the primary class type.
      class.primary_type = parser.current_token.clone();

      // Get the next token.
      parser.next_token();
    }

    // Check if the current token is `abstract`.
    if parser.current_token_is(Keywords::new(Keywords::ABSTRACT)) {
      // Set the secondary class type.
      class.secondary_type = Some(parser.current_token.clone());

      // Get the next token.
      parser.next_token();
    }

    // Check if the current token is `class`.
    if !parser.current_token_is(Keywords::new(Keywords::CLASS)) {
      return Err(Error::from_token(
        format!("expect `class`, got `{}` instead.", parser.current_token.value.clone()),
        parser.current_token.clone(),
      ));
    }

    // Set the class token.
    class.token = parser.current_token.clone();

    // Check if the next token is an identifier.
    if !parser.expect_token(Box::new(Tokens::IDENTIFIER)) {
      return Err(Error::from_token(
        format!("`{}` is not a valid class name.", parser.next_token.value.clone()),
        parser.next_token.clone(),
      ));
    }

    // Set the class name.
    class.name = parser.current_token.clone();

    // Check if the next token is `extends`.
    if parser.expect_token(Keywords::new(Keywords::EXTENDS)) {
      while !parser.next_token_is(Keywords::new(Keywords::IMPLEMENTS)) &&
        !parser.next_token_is(Signs::new(Signs::LEFTBRACE)) {
        // Get the next token.
        parser.next_token();

        // Parse expression.
        match parse_expression(parser, Precedence::LOWEST, standard_library, with_this) {
          Ok(expression) => {
            class.extends.push(expression);
          },
          Err(error) => {
            return Err(error);
          },
        }
      }
    }

    // Check if the next token is `implements`.
    if parser.expect_token(Keywords::new(Keywords::IMPLEMENTS)) {
      while !parser.next_token_is(Signs::new(Signs::LEFTBRACE)) {
        // Get the next token.
        parser.next_token();

        // Parse expression.
        match parse_expression(parser, Precedence::LOWEST, standard_library, with_this) {
          Ok(expression) => {
            class.implements.push(expression);
          },
          Err(error) => {
            return Err(error);
          },
        }
      }
    }

    // Check if the next token is a left brace.
    if !parser.expect_token(Signs::new(Signs::LEFTBRACE)) {
      return Err(Error::from_token(
        format!("expect `{{`, got `{}` instead.", parser.next_token.value.clone()),
        parser.next_token.clone(),
      ));
    }

    // Parse class methods.
    match Block::parse(parser, standard_library, true, false) {
      Ok(body) => {
        if let Some(body) = body.get_block() {
          for statement in body.statements {
            // Check if the statement is a class constructor.
            if let Some(class_constructor) = statement.clone().get_class_constructor() {
              class.constructor = Some(class_constructor);
              continue;
            }

            // Check if the statement is a class method.
            match statement.clone().get_class_method() {
              Some(class_method) => {
                class.methods.push(class_method);
              },
              None => {
                return Err(Error::from_token(
                  format!("`{}` is not a valid class method.", statement.clone().token().value),
                  statement.token().clone(),
                ));
              },
            }
          }
        }
      },
      Err(error) => {
        return Err(error);
      },
    }

    Ok(Box::new(Statements::CLASS(class)))
  }
}

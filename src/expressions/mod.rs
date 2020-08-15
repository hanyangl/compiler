pub mod boolean;
pub mod call;
pub mod function;
pub mod if_else;
pub mod infix;
pub mod integer;
pub mod method;
pub mod prefix;

use crate::data;
use crate::parser::{Parser, precedence::Precedence};

// IDENTIFIER //
pub trait Expression {
  /// Create a new empty expression.
  fn new() -> Self;

  /// Create a new expression from a token.
  fn from_token(token: &data::Token) -> Self;

  /// Get the expression value.
  fn string(self) -> String;
}

#[derive(Debug, Clone)]
pub struct Identifier {
  pub token: data::Token,
  value: String,
}

impl Expression for Identifier {
  fn new() -> Identifier {
    Identifier {
      token: data::Token::empty(),
      value: String::new(),
    }
  }

  fn from_token(token: &data::Token) -> Identifier {
    Identifier {
      token: token.clone(),
      value: token.value.clone(),
    }
  }

  fn string(self) -> String {
    self.value
  }
}
// END IDENTIFIER//


// EXPRESSIONS //
#[derive(Debug, Clone)]
pub enum Expressions {
  BOOLEAN(boolean::Boolean),
  CALL(call::Call),
  DEFAULT(Identifier),
  FUNCTION(function::Function),
  IFELSE(if_else::IfElse),
  INFIX(infix::Infix),
  INTEGER(integer::Integer),
  METHOD(method::Method),
  PREFIX(prefix::Prefix),
}

impl Expressions {
  /// Get the boolean expression.
  pub fn get_boolean(self) -> Option<boolean::Boolean> {
    match self {
      Expressions::BOOLEAN(boolean) => Some(boolean),
      _ => None,
    }
  }

  /// Get the default expression.
  pub fn get_default(self) -> Option<Identifier> {
    match self {
      Expressions::DEFAULT(default) => Some(default),
      _ => None,
    }
  }

  /// Get the function expression.
  pub fn get_function(self) -> Option<function::Function> {
    match self {
      Expressions::FUNCTION(function) => Some(function),
      _ => None,
    }
  }

  /// Get the if-else expression.
  pub fn get_ifelse(self) -> Option<if_else::IfElse> {
    match self {
      Expressions::IFELSE(ifelse) => Some(ifelse),
      _ => None,
    }
  }

  /// Get the integer expression.
  pub fn get_integer(self) -> Option<integer::Integer> {
    match self {
      Expressions::INTEGER(integer) => Some(integer),
      _ => None,
    }
  }

  /// Get the expression token.
  pub fn token(self) -> data::Token {
    match self {
      Expressions::BOOLEAN(boolean) => boolean.token,
      Expressions::CALL(call) => call.token,
      Expressions::DEFAULT(default) => default.token,
      Expressions::FUNCTION(function) => function.token,
      Expressions::IFELSE(ifelse) => ifelse.token,
      Expressions::INFIX(infix) => infix.token,
      Expressions::INTEGER(integer) => integer.token,
      Expressions::METHOD(method) => method.token,
      Expressions::PREFIX(prefix) => prefix.token,
    }
  }

  /// Get the expression as string.
  pub fn string(self) -> String {
    match self {
      Expressions::BOOLEAN(boolean) => boolean.string(),
      Expressions::CALL(call) => call.string(),
      Expressions::DEFAULT(default) => default.string(),
      Expressions::FUNCTION(function) => function.string(),
      Expressions::IFELSE(ifelse) => ifelse.string(),
      Expressions::INFIX(infix) => infix.string(),
      Expressions::INTEGER(integer) => integer.string(),
      Expressions::METHOD(method) => method.string(),
      Expressions::PREFIX(prefix) => prefix.string(),
    }
  }
}

pub fn parse<'a>(parser: &'a mut Parser, precedence: Precedence) -> Option<Box<Expressions>> {
  let token: data::Token = parser.current_token.clone();

  let mut left: Option<Box<Expressions>> = match token.token {    
    // Parse identifiers and strings.
    data::Tokens::IDENTIFIER |
    data::Tokens::STRING => Some(Box::new(Expressions::DEFAULT(Expression::from_token(&token)))),

    // Parse numbers.
    data::Tokens::INTEGER => match integer::parse(parser) {
      Some(x) => Some(Box::new(Expressions::INTEGER(x))),
      None => None,
    },

    // Parse keywords.
    data::Tokens::KEYWORD => match token.keyword {
      // Parse if expression.
      data::Keywords::IF => match if_else::parse(parser) {
        Some(x) => Some(Box::new(Expressions::IFELSE(x))),
        None => None,
      },

      // Parse function.
      data::Keywords::FUNCTION => match function::parse(parser) {
        Some(x) => Some(Box::new(Expressions::FUNCTION(x))),
        None => None,
      },

      // Default
      _ => None,
    },

    // Parse signs.
    data::Tokens::SIGN => match token.sign {
      // Parse '!' and '-' signs.
      data::Signs::NEGATION | data::Signs::MINUS => Some(Box::new(Expressions::PREFIX(prefix::parse(parser)))),

      // Default
      _ => None,
    },

    // Parse data types.
    data::Tokens::TYPE => match token.data_type {
      // Parse undefined and null values.
      data::Types::UNDEFINED |
      data::Types::NULL => Some(Box::new(Expressions::DEFAULT(Expression::from_token(&token)))),

      // Parse true and false values.
      data::Types::TRUE |
      data::Types::FALSE => Some(Box::new(Expressions::BOOLEAN(boolean::parse(parser)))),

      // Default
      _ => None,
    },

    // Default
    _ => None,
  };

  while parser.peek_token_is_sign(&data::Signs::SEMICOLON) == false && precedence < parser.peek_precedence() {
    let peek_token: data::Token = parser.peek_token.clone();

    match peek_token.sign {
      data::Signs::PLUS |
      data::Signs::MINUS |
      data::Signs::DIVIDE |
      data::Signs::MULTIPLY |
      data::Signs::EQUAL |
      data::Signs::EQUALTYPE |
      data::Signs::NOTEQUAL |
      data::Signs::NOTEQUALTYPE |
      data::Signs::LESSTHAN |
      data::Signs::LESSOREQUALTHAN |
      data::Signs::GREATERTHAN |
      data::Signs::GREATEROREQUALTHAN => {
        parser.next_token();

        left = Some(Box::new(Expressions::INFIX(infix::parse(parser, left))));
      },

      data::Signs::ARROW => {
        parser.next_token();

        left = Some(Box::new(Expressions::METHOD(method::parse(parser, left))));
      },

      data::Signs::LEFTPARENTHESES => {
        parser.next_token();

        left = Some(Box::new(Expressions::CALL(call::parse(parser, left))));
      }

      _ => break,
    }
  }

  left
}
// END EXPRESSIONS //

// OBJECTS
#[derive(Debug, Clone)]
pub enum ObjectType {
  NULL,
  ERROR,

  INTEGER,
  BOOLEAN,
  STRING,

  RETURNVALUE,

  FUNCTION,
}

#[derive(Debug, Clone)]
pub struct HashKey {
  object_type: ObjectType,
  value: u64,
}

pub trait Hashable {
  fn hashkey(self) -> HashKey;
}

#[derive(Debug, Clone)]
pub enum Hashables {
  BOOLEAN(boolean::BooleanObject),
  INTEGER(integer::IntegerObject),
}

pub trait Object {
  fn object_type(&self) -> ObjectType;
  fn string(self) -> String;
}
// END OBJECTS //

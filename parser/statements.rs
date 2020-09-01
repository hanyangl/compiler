mod block;
mod class;
mod class_constructor;
mod class_method;
mod export;
mod expression;
mod function;
mod if_else;
mod import;
mod return_s;
mod statement;
mod variable;
mod variable_set;

pub use block::*;
pub use class::*;
pub use class_constructor::*;
pub use class_method::*;
pub use export::*;
pub use expression::*;
pub use function::*;
pub use if_else::*;
pub use import::*;
pub use return_s::*;
pub use statement::*;
pub use variable::*;
pub use variable_set::*;

use super::{
  Error,
  Parser,
  tokens::{
    Keywords,
    Signs,
    Token,
    Tokens,
  },
};

pub fn parse_statement<'a>(
  parser: &'a mut Parser,
  standard_library: bool,
  from_class: bool,
  with_this: bool,
) -> Result<Box<Statements>, Error> {
  // Abstract
  if parser.current_token_is(Keywords::new(Keywords::ABSTRACT)) {
    // Class
    if parser.next_token_is(Keywords::new(Keywords::CLASS)) {
      return Class::parse(parser, standard_library, with_this);
    }

    // Class method.
    if parser.next_token_is(Box::new(Tokens::IDENTIFIER)) {
      return ClassMethod::parse(parser, standard_library, None, with_this);
    }
  }

  // Class
  if parser.current_token_is(Keywords::new(Keywords::CLASS)) {
    return Class::parse(parser, standard_library, with_this);
  }

  // Constructor
  if parser.current_token_is(Keywords::new(Keywords::CONSTRUCTOR)) {
    // Check if the constructor is in a class.
    if !from_class {
      return Err(Error::from_token(
        String::from("can not build a constructor here."),
        parser.current_token.clone(),
      ));
    }

    return ClassConstructor::parse(parser, standard_library, with_this);
  }

  // Enum

  // Export
  if parser.current_token_is(Keywords::new(Keywords::EXPORT)) {
    return Export::parse(parser, standard_library);
  }

  // Function
  if parser.current_token_is(Keywords::new(Keywords::FUNCTION)) {
    return Function::parse(parser, standard_library, with_this);
  }

  // If else
  if parser.current_token_is(Keywords::new(Keywords::IF)) {
    return IfElse::parse(parser, standard_library, with_this);
  }

  // Import
  if parser.current_token_is(Keywords::new(Keywords::IMPORT)) {
    return Import::parse(parser, standard_library, with_this);
  }

  // Interface

  // Internal
  if parser.current_token_is(Keywords::new(Keywords::INTERNAL)) {
    // Check if is on the standard library.
    if !standard_library {
      return Err(Error::from_token(
        String::from("the internal property only can be used by the standard library."),
        parser.current_token.clone(),
      ));
    }

    // Class
    if parser.next_token_is(Keywords::new(Keywords::CLASS)) {
      return Class::parse(parser, standard_library, with_this);
    }
  }

  // Private
  if parser.current_token_is(Keywords::new(Keywords::PRIVATE)) {
    let mut primary_token: Option<Token> = None;

    // Readonly or abstract
    if parser.next_token_is(Keywords::new(Keywords::READONLY)) ||
      parser.next_token_is(Keywords::new(Keywords::ABSTRACT)) {
      // Set the primary token.
      primary_token = Some(parser.current_token.clone());

      // Get the next token.
      parser.next_token();
    }

    // Class
    if parser.next_token_is(Keywords::new(Keywords::CLASS)) {
      return Class::parse(parser, standard_library, with_this);
    }

    // Class method.
    if parser.next_token_is(Box::new(Tokens::IDENTIFIER)) {
      return ClassMethod::parse(parser, standard_library, primary_token, with_this);
    }
  }

  // Protected
  if parser.current_token_is(Keywords::new(Keywords::PROTECTED)) {
    if !from_class {
      return Err(Error::from_token(
        String::from("can not use `protected` here."),
        parser.current_token.clone(),
      ));
    }

    let mut primary_token: Option<Token> = None;

    // Readonly or abstract
    if parser.next_token_is(Keywords::new(Keywords::READONLY)) ||
      parser.next_token_is(Keywords::new(Keywords::ABSTRACT)) {
      // Set the primary token.
      primary_token = Some(parser.current_token.clone());

      // Get the next token.
      parser.next_token();
    }

    // Class method.
    if parser.next_token_is(Box::new(Tokens::IDENTIFIER)) {
      return ClassMethod::parse(parser, standard_library, primary_token, with_this);
    }
  }

  // Public
  if parser.current_token_is(Keywords::new(Keywords::PUBLIC)) {
    let mut primary_token: Option<Token> = None;

    // Readonly or abstract
    if parser.next_token_is(Keywords::new(Keywords::READONLY)) ||
      parser.next_token_is(Keywords::new(Keywords::ABSTRACT)) {
      // Set the primary token.
      primary_token = Some(parser.current_token.clone());

      // Get the next token.
      parser.next_token();
    }

    // Class
    if parser.next_token_is(Keywords::new(Keywords::CLASS)) {
      return Class::parse(parser, standard_library, with_this);
    }

    // Class method.
    if parser.next_token_is(Box::new(Tokens::IDENTIFIER)) {
      return ClassMethod::parse(parser, standard_library, primary_token, with_this);
    }
  }

  // Return
  if parser.current_token_is(Keywords::new(Keywords::RETURN)) {
    return Return::parse(parser, standard_library, with_this);
  }

  // Variable
  if parser.current_token_is(Keywords::new(Keywords::LET)) ||
    parser.current_token_is(Keywords::new(Keywords::CONST)) {
    return Variable::parse(parser, standard_library, with_this);
  }

  // This
  if parser.current_token_is(Keywords::new(Keywords::THIS)) {
    if !with_this {
      return Err(Error::from_token(
        String::from("can not use this here."),
        parser.current_token.clone(),
      ));
    }

    // Check if the next token is a dot.
    if parser.next_token_is(Signs::new(Signs::DOT)) {
      let this = parser.current_token.clone();

      // Get the next token.
      parser.next_token();

      // Get the next token.
      parser.next_token();

      // Check if the next token is an identifier for set.
      if parser.current_token_is(Box::new(Tokens::IDENTIFIER)) &&
        !parser.next_token_is(Signs::new(Signs::LEFTPARENTHESES)) &&
        !parser.next_token_is(Signs::new(Signs::ARROW)) &&
        !parser.next_token_is(Signs::new(Signs::SEMICOLON)) {
        return VariableSet::parse(parser, standard_library, Some(this), with_this);
      }
    }
  }

  // Variable set
  if parser.current_token_is(Box::new(Tokens::IDENTIFIER)) &&
    !parser.next_token_is(Signs::new(Signs::LEFTPARENTHESES)) &&
    !parser.next_token_is(Signs::new(Signs::ARROW)) &&
    !parser.next_token_is(Signs::new(Signs::SEMICOLON)) {
    return VariableSet::parse(parser, standard_library, None, with_this);
  }

  // Default
  ExpressionStatement::parse(parser, standard_library, with_this)
}

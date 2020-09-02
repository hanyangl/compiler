mod block;
mod export;
mod expression;
mod function;
mod if_else;
mod import;
mod interface;
mod return_s;
mod statement;
mod variable;
mod variable_set;

pub use block::*;
pub use export::*;
pub use expression::*;
pub use function::*;
pub use if_else::*;
pub use import::*;
pub use interface::*;
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
    Tokens,
  },
};

pub fn parse_statement<'a>(
  parser: &'a mut Parser,
  standard_library: bool,
  _from_class: bool,
  with_this: bool,
) -> Result<Box<Statements>, Error> {
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
  if parser.current_token_is(Keywords::new(Keywords::INTERFACE)) {
    return Interface::parse(parser, standard_library, with_this);
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

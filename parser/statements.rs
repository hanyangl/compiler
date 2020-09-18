mod block;
mod export;
mod expression;
mod for_s;
mod function;
mod if_else;
mod import;
mod interface;
mod return_s;
mod statement;
mod variable;

pub use block::*;
pub use export::*;
pub use expression::*;
pub use for_s::*;
pub use function::*;
pub use if_else::*;
pub use import::*;
pub use interface::*;
pub use return_s::*;
pub use statement::*;
pub use variable::*;

use super::{
  Error,
  Parser,
  tokens::Keywords,
};

pub fn parse_statement<'a>(
  parser: &'a mut Parser,
  standard_library: bool,
  _from_class: bool,
  with_this: bool,
) -> Result<Box<Statements>, Error> {
  // Export
  if parser.current_token_is(Keywords::new(Keywords::EXPORT)) {
    return Export::parse(parser, standard_library);
  }

  // For
  if parser.current_token_is(Keywords::new(Keywords::FOR)) {
    return For::parse(parser, standard_library, with_this);
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

  // Default
  ExpressionStatement::parse(parser, standard_library, with_this)
}

mod anonymous_function;
mod argument;
mod array;
mod boolean;
mod call;
mod expression;
mod for_condition;
mod hashmap;
mod identifier;
mod infix;
mod null;
mod number;
mod prefix;
mod string;
mod suffix;
mod types;

pub use anonymous_function::*;
pub use argument::*;
pub use array::*;
pub use boolean::*;
pub use call::*;
pub use expression::*;
pub use for_condition::*;
pub use hashmap::*;
pub use identifier::*;
pub use infix::*;
pub use null::*;
pub use number::*;
pub use prefix::*;
pub use string::*;
pub use suffix::*;
pub use types::*;

use super::{
  Error,
  Parser,
  Precedence,
  tokens::{
    Keywords,
    Signs,
    Token,
    Types,
  },
};

pub fn parse_expression<'a>(
  parser: &'a mut Parser,
  precedence: Precedence,
  standard_library: bool,
  with_this: bool,
) -> Result<Box<Expressions>, Error> {
  let current_token: Token = parser.get_current_token();
  let mut expression: Result<Box<Expressions>, Error> = Err(Error::from_token(
    format!("`{}` is not a valid expression.", parser.get_current_token().value),
    parser.get_current_token(),
  ));

  // Parse identifiers.
  if current_token.token.is_identifier() &&
    !parser.get_next_token().token.expect_sign(&Signs::LEFTBRACKET) &&
    !parser.get_next_token().token.expect_sign(&Signs::LEFTPARENTHESES) {
    expression = Ok(Identifier::new_box_from_token(current_token.clone()));
  }

  // Parse nulls.
  if current_token.token.expect_type(&Types::NULL) {
    expression = Ok(Null::new_box_from_token(current_token.clone()));
  }

  // Parse strings.
  if current_token.token.is_string() {
    expression = Ok(StringE::new_box_from_token(current_token.clone()));
  }

  // Parse numbers.
  if current_token.token.is_number() {
    expression = Number::parse(parser);
  }

  // Parse booleans.
  if current_token.token.expect_keyword(&Keywords::TRUE) ||
    current_token.token.expect_keyword(&Keywords::FALSE) {
    expression = Ok(Boolean::parse(parser));
  }

  // Parse prefixes.
  if current_token.token.expect_sign(&Signs::NOT) ||
    current_token.token.expect_sign(&Signs::MINUS) ||
    current_token.token.expect_sign(&Signs::PLUSPLUS) ||
    current_token.token.expect_sign(&Signs::MINUSMINUS) {
    expression = Prefix::parse(parser, standard_library, with_this);
  }

  // Parse anonymous functions.
  if current_token.token.expect_keyword(&Keywords::FUNCTION) || (
    current_token.token.expect_sign(&Signs::LEFTPARENTHESES) && (
      parser.get_next_token().token.is_identifier() ||
      parser.get_next_token().token.expect_sign(&Signs::RIGHTPARENTHESES)
    )
  ) {
    expression = AnonymousFunction::parse(parser, standard_library, with_this);
  }

  // Parse calls.
  if current_token.token.is_identifier() &&
    parser.next_token_is(Signs::new(Signs::LEFTPARENTHESES)) {
    expression = Call::parse(parser, standard_library, with_this);
  }

  // Parse hashmaps.
  if current_token.token.expect_sign(&Signs::LEFTBRACE) {
    expression = HashMap::parse(parser, standard_library, with_this);
  }

  // Parse arrays.
  if current_token.token.expect_sign(&Signs::LEFTBRACKET) {
    expression = Array::parse(parser, standard_library, with_this);
  }

  // Parse array index.
  if current_token.token.clone().is_identifier() &&
    parser.next_token_is(Signs::new(Signs::LEFTBRACKET)) {
    expression = ArrayIndex::parse(parser, expression, standard_library, with_this);
  }

  // Parse * as identifier.
  if parser.current_token_is(Signs::new(Signs::MULTIPLY)) {
    expression = Ok(Identifier::new_box_from_token(parser.get_current_token()));
  }

  if let Err(error) = expression {
    return Err(error);
  }

  // Parse infix expression.
  while !parser.next_token_is(Signs::new(Signs::SEMICOLON)) &&
    precedence < parser.next_precedence()
  {
    // Parse Infix, Alias and method.
    if parser.next_token_is(Signs::new(Signs::PLUS)) ||
      parser.next_token_is(Signs::new(Signs::MINUS)) ||
      parser.next_token_is(Signs::new(Signs::DIVIDE)) ||
      parser.next_token_is(Signs::new(Signs::MULTIPLY)) ||
      parser.next_token_is(Signs::new(Signs::EMPOWERMENT)) ||
      parser.next_token_is(Signs::new(Signs::CARER)) ||
      parser.next_token_is(Signs::new(Signs::MODULE)) ||
      parser.next_token_is(Signs::new(Signs::EQUAL)) ||
      parser.next_token_is(Signs::new(Signs::NOTEQUAL)) ||
      parser.next_token_is(Signs::new(Signs::LESSTHAN)) ||
      parser.next_token_is(Signs::new(Signs::LESSOREQUALTHAN)) ||
      parser.next_token_is(Signs::new(Signs::GREATERTHAN)) ||
      parser.next_token_is(Signs::new(Signs::GREATEROREQUALTHAN)) ||
      parser.next_token_is(Keywords::new(Keywords::IN)) ||
      parser.next_token_is(Keywords::new(Keywords::OF)) ||
      parser.next_token_is(Keywords::new(Keywords::AS)) ||
      parser.next_token_is(Keywords::new(Keywords::IS)) ||
      parser.next_token_is(Signs::new(Signs::ARROW)) ||
      parser.next_token_is(Signs::new(Signs::OR)) ||
      parser.next_token_is(Signs::new(Signs::AND)) ||
      parser.next_token_is(Signs::new(Signs::ASSIGN)) ||
      parser.next_token_is(Signs::new(Signs::PLUSASSIGN)) ||
      parser.next_token_is(Signs::new(Signs::MINUSASSIGN)) ||
      parser.next_token_is(Signs::new(Signs::MULTIPLYASSIGN)) ||
      parser.next_token_is(Signs::new(Signs::DIVIDEASSIGN)) {
      // Get the next token.
      parser.next_token();

      // Set the new expression.
      if let Ok(left) = expression {
        expression = Infix::parse(parser, left, standard_library, with_this);

        if let Err(error) = expression {
          return Err(error);
        }
      }

      continue;
    }
    // Parse suffix expression.
    else if parser.next_token_is(Signs::new(Signs::PLUSPLUS)) ||
      parser.next_token_is(Signs::new(Signs::MINUSMINUS)) {
      // Get the next token.
      parser.next_token();

      // Set the new expression.
      if let Ok(left) = expression {
        expression = Ok(Suffix::parse(parser, left));
      }

      continue;
    }

    break;
  }

  // Return expression.
  expression
}

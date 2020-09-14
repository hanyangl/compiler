mod anonymous_function;
mod argument;
mod array;
mod boolean;
mod call;
mod expression;
mod hashmap;
mod identifier;
mod infix;
mod null;
mod number;
mod prefix;
mod string;
mod types;

pub use anonymous_function::*;
pub use argument::*;
pub use array::*;
pub use boolean::*;
pub use call::*;
pub use expression::*;
pub use hashmap::*;
pub use identifier::*;
pub use infix::*;
pub use null::*;
pub use number::*;
pub use prefix::*;
pub use string::*;
pub use types::*;

use super::{
  Error,
  Parser,
  Precedence,
  tokens::{
    Keywords,
    Signs,
    Token,
    Tokens,
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
    current_token.token.expect_sign(&Signs::MINUS) {
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
    expression = ArrayIndex::parse(parser, standard_library, with_this);
  }

  // Parse * as identifier.
  if parser.current_token_is(Signs::new(Signs::MULTIPLY)) {
    expression = Ok(Identifier::new_box_from_token(parser.get_current_token()));
  }

  // Parse this.
  if parser.current_token_is(Keywords::new(Keywords::THIS)) {
    if !with_this {
      return Err(Error::from_token(
        String::from("can not use this here."),
        parser.get_current_token(),
      ));
    }

    let this = parser.get_current_token();

    // Check if the next token is a dot.
    if !parser.expect_token(Signs::new(Signs::DOT)) {
      return Err(Error::from_token(
        format!("expect `.`, get `{}` instead,", parser.get_next_token().value),
        parser.get_next_token(),
      ));
    }

    // Check if the next token is an identifier.
    if !parser.expect_token(Box::new(Tokens::IDENTIFIER)) {
      return Err(Error::from_token(
        format!("`{}` is not a valid identifier.", parser.get_next_token().value),
        parser.get_next_token(),
      ));
    }

    let mut identifier = Identifier::from_token(parser.get_current_token());

    identifier.this = Some(this);

    // Check if the next token is a semicolon.
    if parser.next_token_is(Signs::new(Signs::SEMICOLON)) {
      // Get the next token.
      parser.next_token();
    }

    expression = Ok(Box::new(Expressions::IDENTIFIER(identifier)));
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
      parser.next_token_is(Keywords::new(Keywords::AS)) ||
      parser.next_token_is(Signs::new(Signs::ARROW)) ||
      parser.next_token_is(Signs::new(Signs::OR)) ||
      parser.next_token_is(Signs::new(Signs::AND)) {
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

    break;
  }

  // Return expression.
  expression
}

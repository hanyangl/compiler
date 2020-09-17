mod print;

use sflyn_parser::tokens::Token;

use super::{
  BuiltIn,
  Error,
  Objects,
};

pub fn get_builtin_for_identifier(identifier: Token) -> Box<Objects> {
  // Print
  if identifier.value == "print" {
    return BuiltIn::new_box(None, Some(print::print));
  }

  // Default
  Error::new(
    format!("`{}` identifier not found.", identifier.value.clone()),
    identifier.clone(),
  )
}

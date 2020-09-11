use crate::compiler::{
  Error,
  Objects,
  StringO,
};

use sflyn_parser::tokens::Token;

pub fn to_string(
  token: Token,
  arguments: Vec<Box<Objects>>,
) -> Box<Objects> {
  if arguments.len() != 1 {
    return Error::new(
      format!("expect `1` argument, got `{}` instead.", arguments.len()),
      token.clone(),
    );
  }

  StringO::new(arguments[0].clone().string())
}

use crate::compiler::{
  Error,
  Objects
};

use sflyn_parser::tokens::Token;

use std::io::{
  self,
  Write,
};

pub fn print(token: Token, arguments: Vec<Box<Objects>>) -> Box<Objects> {
  if arguments.len() != 1 {
    return Error::new(
      format!("expect `1` argument, got `{}` instead.", arguments.len()),
      token.clone(),
    );
  }

  let stdout = io::stdout();
  let mut handle = stdout.lock();

  let string =
    arguments[0].clone().string()
      .replace("\\r", "\r")
      .replace("\\n", "\n")
      .replace("\\t", "\t");

  handle.write(string.as_bytes()).unwrap();
  handle.write(b"\n").unwrap();

  arguments[0].clone()
}

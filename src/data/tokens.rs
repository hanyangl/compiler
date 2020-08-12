#[derive(Debug)]
#[derive(PartialEq)]
pub enum Tokens {
  ILLEGAL,

  IDENTIFIER,
  INTENGER,

  KEYWORD,
  SIGN,
  TYPE,

  EOL,
  EOF,
}

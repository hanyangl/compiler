#[derive(Debug)]
#[derive(PartialEq)]
pub enum Tokens {
  ILLEGAL,

  IDENTIFIER,
  STRING,
  INTENGER,

  KEYWORD,
  SIGN,
  TYPE,

  EOL,
  EOF,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Tokens {
  ILLEGAL,

  IDENTIFIER,
  STRING,
  INTEGER,

  KEYWORD,
  SIGN,
  TYPE,

  EOL,
  EOF,
}

mod functions;
mod hashmaps;
mod lexer;
mod tokens;
mod variables;

#[cfg(test)]
use sflyn_parser::Lexer;

#[cfg(test)]
pub fn generate_lexer(content: &str) -> Lexer {
  Lexer::new(String::from("test.sf"), content.to_string())
}

pub mod data;
pub mod parser;
pub mod statements;
pub mod utils;

fn main() {
  let hello_world = std::fs::read_to_string(
    format!("{}/examples/hello_world.sf", std::env::current_dir().unwrap().display())
  ).expect("File not found.");

  let lexer = parser::Lexer::new(hello_world);
  let mut parser = parser::Parser::new(lexer);

  parser.parse_program();

  if parser.errors.len() > 0 {
    println!("Parser errors:");

    for error in parser.errors {
      println!("\n{}", error);
    }
  }
}

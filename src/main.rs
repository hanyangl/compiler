pub mod compiler;
pub mod data;
pub mod expressions;
pub mod objects;
pub mod parser;
pub mod statements;
pub mod utils;

fn main() {
  let test_file = std::fs::read_to_string(
    format!("{}/test_file.sf", std::env::current_dir().unwrap().display())
  ).expect("Test file not found.");

  let lexer = parser::Lexer::new(test_file);
  let mut parser = parser::Parser::new(lexer);

  let statements = parser.parse_program();

  if parser.errors.len() > 0 {
    println!("{}", parser.errors.join("\n\n"));
  } else {
    match compiler::evaluate(statements.clone(), &mut compiler::environment::Environment::new()) {
      Some(result) => {
        println!("Result: {}", result.string());
      },
      None => {},
    }
  }
}

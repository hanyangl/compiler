use sflyn_parser;
use sflyn_compiler;

use std::{env, fs, path::Path};

pub fn main() {
  // Get the sflyn path.
  let sflyn_path = sflyn_parser::library::get_sflyn_path();

  // Check if the slfyn path exists.
  if sflyn_path.as_str() == "" {
    println!("You must set the `SFLYN_PATH` environment variable.");
    return;
  }

  // Get the command line arguments.
  let arguments: Vec<String> = env::args().collect();

  if arguments.len() > 1 {
    // Check if the first argument is a Sflyn file.
    if arguments[1].clone().ends_with(".sf") {
      let file_name = arguments[1].clone();
      let file_path = Path::new(&file_name);

      // Check if the file exists.
      if !file_path.is_file() && !file_path.exists() {
        println!("The `{}` path does not exist or is not a file.", file_name);
        return;
      }

      // Get the file content.
      let file_content = fs::read_to_string(file_path).expect("The file does not exists.");

      // Create a new lexer.
      let lexer = sflyn_parser::Lexer::new(file_name.clone(), file_content);

      // Create a new parser.
      let mut parser = sflyn_parser::Parser::new(lexer);

      // Create a new environment for the parser.
      let mut environment_parser = sflyn_parser::Environment::new();

      // Add standard libraries.
      sflyn_parser::library::add_libraries(&mut environment_parser);

      // Parse file.
      let statements = parser.parse_program(&mut environment_parser, false);

      // Check if the parser contains errors.
      if parser.errors.len() > 0 {
        parser.show_errors();
        return;
      }

      // Create a new environment for the compiler.
      let mut environment_compiler = sflyn_compiler::Environment::new();

      // Add standard libraries.
      sflyn_compiler::library::add_libraries(&mut environment_compiler);

      // Compile file.
      sflyn_compiler::evaluator::program(file_name, statements, &mut environment_compiler);
      return;
    }

    println!("Is not a valid Sflyn file.");
    return;
  }

  println!("You must enter the file path.");
}

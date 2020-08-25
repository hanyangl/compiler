use sflyn_parser::{Environment, Lexer, library, Parser};

use std::{env, fs, path::Path};

pub fn main() {
  // Get the sflyn path.
  let sflyn_path = library::get_sflyn_path();

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
      let lexer = Lexer::new(file_name, file_content);

      // Create a new parser.
      let mut parser = Parser::new(lexer);

      // Create a new environment.
      let mut environment = Environment::new();

      // Add standar libraries.
      library::add_libraries(&mut environment);

      // Parse file.
      let statements = parser.parse_program(&mut environment, false);

      // Check if the parser contains errors.
      if parser.errors.len() > 0 {
        parser.show_errors();
        return;
      }

      for stmt in statements {
        println!("{}", stmt.string());
      }

      return;
    }

    println!("Is not a valid Sflyn file.");
    return;
  }

  println!("You must enter the file path.");
}

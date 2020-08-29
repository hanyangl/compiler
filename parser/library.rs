use std::{env, fs, path::Path};

use crate::Environment;
use crate::{Lexer, Parser, statements::Statements};

/// Get the sflyn main path.
pub fn get_sflyn_path() -> String {
  match env::var(String::from("SFLYN_PATH")) {
    Ok(sflyn_path) => sflyn_path,
    Err(_) => String::new(),
  }
}

/// Get the standard library path.
fn get_std_path() -> String {
  format!("{}/std/", get_sflyn_path())
}

/// Get the library file content.
fn get_library_content(name: &str) -> Option<String> {
  let library_rute = format!("{}{}", get_std_path(), name.clone());
  let library_path = Path::new(&library_rute);

  // Check if the library path exists.
  if !library_path.is_file() || !library_path.exists() {
    return None;
  }

  // Return the library file content.
  Some(fs::read_to_string(library_rute).expect(
    format!("The library {} does not exist.", name).as_str(),
  ))
}

pub fn get_library_statements(name: &str) -> Vec<Box<Statements>> {
  // Get the library file content.
  match get_library_content(name) {
    // Library file exists.
    Some(library_content) => {
      // Create a new lexer.
      let lexer = Lexer::new(name.to_string(), library_content);

      // Create a new parser.
      let mut parser = Parser::new(lexer);

      // Parse program as standard library.
      let statements = parser.parse_program(&mut Environment::new(), true);

      // Check if the file contains syntax errors.
      if parser.errors.len() > 0 {
        parser.show_errors();
        return Vec::new();
      }

      return statements;
    },
    None => {},
  }

  Vec::new()
}

/// Add a library to the parser environment.
fn add_library(name: &str, environment: &mut Environment) {
  let statements = get_library_statements(name);

  // Add variables or functions statements to the parser environment.
  for statement in statements {
    if !statement.clone().is_variable() && !statement.clone().is_function() {
      continue;
    }

    match statement.clone().get_variable() {
      Some(variable) => {
        environment.set_statement(variable.name.clone().string(), statement.clone());
      },
      None => {
        match statement.clone().get_function() {
          Some(function) => {
            environment.set_statement(function.name.string(), statement.clone());
          },
          None => {},
        }
      },
    }
  }
}

/// Add standard libraries to the parser environment.
/// **CAUTION** Add this before parse other files.
pub fn add_libraries(environment: &mut Environment) {
  // Add log library.
  add_library("log.sf", environment);
}

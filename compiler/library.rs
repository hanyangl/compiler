use sflyn_parser::library;

use super::{Environment, evaluator};

/// Add a library to the compiler environment.
fn add_library(name: &str, environment: &mut Environment) {
  // Get library statements.
  let statements = library::get_library_statements(name);

  // Compile statements as standard library.
  evaluator::program(statements, environment);
}

/// Add standard libraries to the compiler environment.
/// **CAUTION** Add this before compile other files.
pub fn add_libraries(environment: &mut Environment) {
  // Add log library.
  add_library("log.sf", environment);
}

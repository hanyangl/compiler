mod error;
mod expressions;
mod lexer;
mod parser;
mod precedence;
mod statements;
pub mod tokens;

pub use error::*;
pub use expressions::*;
pub use lexer::*;
pub use parser::*;
pub use precedence::*;
pub use statements::*;

#[cfg(not(test))]
pub fn run(file_name: String) -> Result<File, (Error, Option<File>)> {
  let path = std::path::Path::new(&file_name);

  // Check if the path is a file.
  if !path.exists() || !path.is_file() {
    return Err((Error::new(format!("`{}` file does not exists.", file_name), 0, 0, 0), None));
  }

  // Check if the file extension is `.sf`
  if path.extension().is_none() || path.extension().unwrap() != "sf" {
    return Err((Error::new(format!("`{}` is not a Sflyn file.", file_name), 0, 0, 0), None));
  }

  // Get the file content.
  let file_content = std::fs::read_to_string(file_name.clone()).unwrap();

  // Create a new file object.
  let mut file = File::new(file_name, file_content);

  // Create a new lexer.
  let lexer = Lexer::new(file.clone());

  // Create a new parser.
  let mut parser = Parser::new(lexer);

  while !parser.current_token_is(Box::new(tokens::Tokens::EOF)) {
    match parse_statement(&mut parser, false, false, false) {
      Ok(statement) => {
        // Add the statement to the file statements.
        file.statements.push(statement.clone());

        // Check if the current statement is an export.
        if let Some(export) = statement.clone().get_export() {
          // Check if the export value is a variabe.
          if let Some(variable) = export.value.clone().get_variable() {
            file.exports.push(variable.name.value);
          }
          // Check if the export value is a function.
          else if let Some(function) = export.value.clone().get_function() {
            file.exports.push(function.name.value);
          }
          // Check if the export value is an interface.
          else if let Some(interface) = export.value.clone().get_interface() {
            file.exports.push(interface.name.value);
          }
          // Check if the export value is an expression.
          else if let Some(expression) = export.value.clone().get_expression() {
            // Check if the expression is an identifier.
            if let Some(identifier) = expression.expression.clone().get_identifier() {
              file.exports.push(identifier.value);
            }
          }
        }
      },
      Err(error) => {
        return Err((error, Some(file)));
      },
    }

    // Get the next token.
    parser.next_token();
  }

  Ok(file)
}

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

        // Check if the statement is an export.
        if let Some(export) = statement.clone().get_export() {
          file.exports.push(export);
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

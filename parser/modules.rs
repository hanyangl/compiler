use std::{path::Path, fs};

use super::Environment;
use super::{Parser, Lexer};
use super::statements::{Import, Export};

pub fn resolve_path_expression<'a>(
  parser: &'a mut Parser,
  import: Import,
  environment: &mut Environment,
  standard_library: bool,
) -> bool {
  let path = import.path.clone();

  let line = parser.get_error_line(
    path.clone().token().line - 1,
    path.clone().token().position - 1,
    path.clone().token().value.len(),
  );

  // Check if the path is a string expression.
  if !path.clone().is_string() {
    parser.errors.push(format!("{} `{}` is not a valid string.", line, path.token().value));
    return false;
  }

  match Path::new(&parser.lexer.file_name.clone()).parent() {
    Some(root_dir) => {
      let path_string = path.clone().string();

      let mut module_rute = format!(
        "{}/{}", 
        root_dir.display(),
        path_string[1..path_string.len() - 1].to_string(),
      );

      let mut module_path = Path::new(&module_rute);

      // Check if the module exists.
      if !module_path.exists() {
        parser.errors.push(format!("{} the module does not exist.", line));
        return false;
      }

      // Check if the module is a directory and get the `index.sf` file.
      if module_path.is_dir() {
        module_rute = format!("{}/index.sf", module_rute.clone());
        module_path = Path::new(&module_rute);
      }

      // Check if the module path is a file.
      if !module_path.exists() || !module_path.is_file() {
        parser.errors.push(format!("{} the module path does not exist.", line));
        return false;
      }

      // Check if the module file is a sf extension.
      if module_path.extension().unwrap() != "sf" {
        parser.errors.push(format!("{} the module path is not a Sflyn file.", line));
        return false;
      }

      let module_content = fs::read_to_string(module_rute.clone()).expect("");
      let module_lexer = Lexer::new(module_rute, module_content);
      let mut module_parser = Parser::new(module_lexer);
      let mut module_environment = Environment::new();
      let module_statements = module_parser.parse_program(&mut module_environment, standard_library);

      if module_parser.errors.len() > 0 {
        module_parser.show_errors();
        return false;
      }

      let mut exports: Vec<Export> = Vec::new();

      for statement in module_statements {
        if statement.clone().is_export() {
          exports.push(statement.clone().get_export().unwrap());
        }
      }

      if exports.len() == 0 {
        parser.errors.push(format!("{} the module does not contain exports.", line));
        return false;
      }

      for export in exports.clone() {
        // Get value statement.
        match export.value {
          Some(value) => {
            // Check if the value is an expression.
            if value.clone().is_expression() {
              match value.clone().get_expression().unwrap().expression {
                Some(value) => {
                  if value.clone().is_identifier() {
                    let identifier = value.clone().get_identifier().unwrap();

                    if module_environment.has_statement(identifier.value.clone()) {
                      environment.set_statement(identifier.value.clone(), module_environment.get_statement(identifier.value).unwrap());
                      continue;
                    }
                  }
                },
                None => {},
              }
            }

            // Check if the value is a function.
            if value.clone().is_function() {
              let function = value.clone().get_function().unwrap();
              environment.set_statement(function.name.string(), value);
              continue;
            }

            // Check if the value is a variable.
            if value.clone().is_variable() {
              let variable = value.clone().get_variable().unwrap();
              environment.set_statement(variable.name.string(), value);
              continue;
            }

            println!("TODO(Modules): {:?}", value);
          },
          None => {},
        }
      }

      for require in import.requires.clone() {
        let require_name = require.clone().string();

        if !environment.has_statement(require_name.clone()) &&
          !environment.has_expression(require_name.clone()) {
          let line = parser.get_error_line(
            require.clone().token().line - 1,
            require.clone().token().position - 1,
            require.clone().token().value.len(),
          );

          parser.errors.push(format!("{} the module does not contain a `{}` method.", line, require_name));

          return false;
        }
      }
    },
    None => {
      println!("TODO(Modules): Parent");
      return false;
    },
  }

  true
}

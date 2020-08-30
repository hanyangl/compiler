use crate::{Environment, evaluator};
use crate::objects::{Objects, Error};

use sflyn_parser::{Environment as PEnvironment, Lexer, Parser};
use sflyn_parser::statements::{Import, Export};
use std::{path::Path, fs};

pub fn evaluate(
  file_name: String,
  import: Import,
  environment: &mut Environment,
) -> Option<Box<Objects>> {
  match Path::new(&file_name.clone()).parent() {
    Some(root_dir) => {
      let path_string = import.path.clone().string();

      let mut module_rute = format!(
        "{}/{}", 
        root_dir.display(),
        path_string[1..path_string.len() - 1].to_string(),
      );

      let mut module_path = Path::new(&module_rute);

      // Check if the module exists.
      if !module_path.exists() {
        return Some(Error::new(format!("{} the module does not exist.", module_rute)));
      }

      // Check if the module is a directory and get the `index.sf` file.
      if module_path.is_dir() {
        module_rute = format!("{}/index.sf", module_rute.clone());
        module_path = Path::new(&module_rute);
      }

      // Check if the module exists.
      if !module_path.exists() || !module_path.is_file() {
        return Some(Error::new(format!("{} the module does not exist.", module_rute)));
      }

      let module_content = fs::read_to_string(module_rute.clone()).expect("");
      let module_lexer = Lexer::new(module_rute, module_content);
      let mut module_parser = Parser::new(module_lexer);
      let mut module_environment = PEnvironment::new();
      let module_statements = module_parser.parse_program(&mut module_environment, false);

      if module_parser.errors.len() > 0 {
        return None;
      }

      let mut exports: Vec<Export> = Vec::new();

      for statement in module_statements.clone() {
        if statement.clone().is_export() {
          exports.push(statement.clone().get_export().unwrap());
        }
      }

      if exports.len() == 0 {
        return None;
      }

      let mut module_environment_compiler = Environment::new();

      evaluator::program(
        file_name.clone(),
        module_statements.clone(),
        &mut module_environment_compiler,
      );

      for require in import.requires.clone() {
        let require_name = require.clone().string();

        match module_environment_compiler.get(require_name.clone()) {
          Some(object) => {
            environment.set(require_name.clone(), object);
          },
          None => {
            return None;
          }
        }
      }
    },
    None => {},
  }

  None
}

use crate::{
  compiler::{
    evaluate_expression,
    Error,
    HashItem,
    HashMap,
    Objects,
  },
  Environment,
  program::run_file,
  Store,
};

use sflyn_parser::Import;
use std::path::Path;

pub fn evaluate(
  import: Import,
  environment: &mut Environment,
) -> Option<Box<Objects>> {
  // Check if the environment has a current file.
  if environment.current_file.is_none() {
    return Some(Error::new(
      String::from("the file is not valid."),
      import.token,
    ));
  }

  // Get the current file from the environment.
  let current_file = environment.current_file.clone();

  // Get the path of the current file.
  let current_path = current_file.clone().unwrap().get_full_rute();
  let current_path = Path::new(&current_path);

  // Check if the current path has a parent directory.
  if current_path.parent().is_none() {
    return Some(Error::new(
      String::from("the file does not has a parent directory."),
      import.token,
    ));
  }

  // Get the object for the import path.
  let path_obj = evaluate_expression(import.path.clone(), environment);

  // Check if the path object is an error.
  if path_obj.clone().get_error().is_some() {
    return Some(path_obj);
  }

  // Check if the path object is not a string.
  if path_obj.clone().get_string().is_none() {
    return Some(Error::new(
      String::from("is not a valid path."),
      import.path.token(),
    ));
  }

  let mut path_to = path_obj.get_string().unwrap().value;

  if path_to.ends_with(".sf") {
    path_to = path_to[0..path_to.len() - 3].to_string();
  }

  // Get the parent directory from the current file.
  let parent_path = current_path.parent().unwrap().display();

  // Get the new path for the import.
  let new_path = format!("{}/{}.sf", parent_path, path_to);
  
  // Clone the current environment.
  let mut import_environment = environment.clone();

  // Set a new store from the current environment store.
  import_environment.store = Store::from_store(import_environment.store.clone());

  // Parse and compile the file imported.
  run_file(new_path.clone(), &mut import_environment);

  // Get the file imported from the environment.
  let new_file = import_environment.get_file(new_path);

  // Check if the file imported exists in the environment.
  if new_file.is_none() {
    return Some(Error::new(
      String::from("the file imported is not valid."),
      import.path.token(),
    ));
  }

  let file_exports = new_file.unwrap().exports;
  let mut exports_items: Vec<HashItem> = Vec::new();

  for export in file_exports.iter() {
    if let Some(env_obj) = import_environment.store.get_object(export.clone()) {
      exports_items.push(HashItem {
        key: export.clone(),
        value: env_obj,
      });

      continue;
    }

    return Some(Error::new(
      format!("`{}` is not a valid export in `{}`.", export, path_to),
      import.path.token(),
    ));
  }

  if import.modules.len() == 0 {
    for item in exports_items.iter() {
      environment.store.set_object(item.key.clone(), item.value.clone());
    }
  } else {
    let exports = HashMap::new(exports_items);

    // Evaluate import modules.
    for module in import.modules.iter() {
      // Check if the module is an identifier.
      if let Some(identifier) = module.clone().get_identifier() {
        if let Some(env_obj) = import_environment.store.get_object(identifier.value.clone()) {
          environment.store.set_object(identifier.value, env_obj);
          continue;
        }

        return Some(Error::new(
          format!("`{}` identifier not found in `{}`.", identifier.value.clone(), path_to),
          identifier.token,
        ));
      }
      // Check if the module is an infix.
      if let Some(infix) = module.clone().get_infix() {
        // Check if the infix expression is an alias.
        if infix.clone().is_alias() {
          // Get the left identifier.
          if let Some(left_identifier) = infix.left.clone().get_identifier() {
            // Get the object from the environment.
            if let Some(env_obj) = import_environment.store.get_object(left_identifier.value.clone()) {
              if let Some(right_identifier) = infix.right.clone().get_identifier() {
                environment.store.set_object(right_identifier.value, env_obj);
                continue;
              }
            }
            // Check if the left identifier is an `*`.
            else if left_identifier.value == "*" {
              if let Some(right_identifier) = infix.right.clone().get_identifier() {
                environment.store.set_object(right_identifier.value, exports.clone());
                continue;
              }
            }

            return Some(Error::new(
              format!("`{}` identifier not found in `{}`.", left_identifier.value, path_to),
              left_identifier.token,
            ));
          }
        }

        return Some(Error::new(
          String::from("only can use `as` expressions."),
          infix.token,
        ));
      }

      return Some(Error::new(
        String::from("unknown import module."),
        module.clone().token(),
      ));
    }
  }

  None
}

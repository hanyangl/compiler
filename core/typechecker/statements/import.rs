use crate::{
  Environment,
  program::run_file,
  Store,
  typechecker::{
    check_expression,
    TTypes,
  },
};

use sflyn_parser::{
  Error,
  Expression,
  Import,
  Statement,
  tokens::{
    Token,
    Types,
  },
};

use std::{
  collections::HashMap,
  path::Path,
};

pub fn check(
  import: &Import,
  environment: &mut Environment,
) -> Result<TTypes, Error> {
  // Get the current file from the environment.
  let current_file = environment.current_file.clone();

  // Get the path of the current file.
  let current_path = current_file.clone().unwrap().get_full_rute();
  let current_path = Path::new(&current_path);

  // Check if the current path has a parent directory.
  if current_path.parent().is_none() {
    return Err(Error::from_token(
      String::from("the file does not has a parent directory."),
      import.get_token(),
    ));
  }

  let path_type: TTypes;
  
  match check_expression(&import.get_path(), environment) {
    Ok(token) => {
      path_type = token;
    },
    Err(error) => {
      return Err(error);
    },
  }

  if path_type.get_type() != Types::STRING {
    return Err(Error::from_token(
      String::from("is not a valid import path."),
      path_type.get_token(),
    ));
  }

  let path_to = path_type.get_token().value;
  let mut path_to = path_to[1..path_to.len() - 1].to_string();

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
  import_environment.store = Store::from_store(&import_environment.store);

  // Parse and compile the file imported.
  run_file(new_path.clone(), &mut import_environment, true, false, false);

  // Get the file imported from the environment.
  let new_file = import_environment.get_file(new_path);

  // Check if the file imported exists in the environment.
  if new_file.is_none() {
    return Err(Error::from_token(
      String::from("the file imported is not valid."),
      import.get_path().token(),
    ));
  }

  let file_exports = new_file.unwrap().exports;

  let mut values: Vec<String> = Vec::new();
  let mut methods: HashMap<String, TTypes> = HashMap::new();

  for export in file_exports.iter() {
    if let Some(env_type) = import_environment.store.get_type(export) {
      values.push(format!("{}: {}", export.clone(), env_type.get_value()));
      methods.insert(export.clone(), env_type);
      continue;
    }

    return Err(Error::from_token(
      format!("`{}` is not a valid export in `{}`.", export, path_to),
      import.get_path().token(),
    ));
  }

  let mut value = String::from("{");

  value.push_str(&values.join(", "));
  value.push_str("}");

  let export_token = Token::from_value(value.as_str(), 0, 0);

  if export_token.token.get_type().is_none() {
    return Err(Error::from_token(
      String::from("the imported file does not contains valid exports."),
      import.get_token(),
    ));
  }

  let ttype = TTypes::new_hashmap(
    export_token.token.get_type().unwrap(),
    value,
    import.get_token(),
    methods.clone(),
  );

  if import.get_modules().len() == 0 {
    for (key, value) in methods.iter() {
      environment.store.set_type(key.clone(), value.clone());
    }
  } else {
    for module in import.get_modules().iter() {
      // Check if the module is an identifier.
      if let Some(identifier) = module.get_identifier() {
        if let Some(token) = import_environment.store.get_type(&identifier.get_value()) {
          environment.store.set_type(identifier.get_value(), token);
          continue;
        }

        return Err(Error::from_token(
          format!("`{}` identifier not found in `{}`.", identifier.get_value(), path_to),
          identifier.get_token(),
        ));
      }
      // Check if the module is an infix.
      else if let Some(infix) = module.get_infix() {
        // Check if the infix expression is an alias.
        if infix.is_alias() {
          // Get the left identifier.
          if let Some(left_identifier) = infix.get_left().get_identifier() {
            // Get the type from the environment.
            if let Some(token) = import_environment.store.get_type(&left_identifier.get_value()) {
              if let Some(right_identifier) = infix.get_right().unwrap().get_identifier() {
                environment.store.set_type(right_identifier.get_value(), token);
                continue;
              }
            }
            // Check if the left identifier is an `*`.
            else if left_identifier.get_value() == "*" {
              if let Some(right_identifier) = infix.get_right().unwrap().get_identifier() {
                environment.store.set_type(right_identifier.get_value(), ttype.clone());
                continue;
              }
            }

            return Err(Error::from_token(
              format!("`{}` identifier not found in `{}`.", left_identifier.get_value(), path_to),
              left_identifier.get_token(),
            ));
          }
        }

        return Err(Error::from_token(
          String::from("only can use `as` expressions."),
          infix.get_token(),
        ));
      }

      return Err(Error::from_token(
        String::from("unknown import module."),
        module.clone().token(),
      ));
    }
  }

  Ok(ttype)
}

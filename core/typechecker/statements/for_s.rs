use crate::{
  Environment,
  typechecker::{
    check_expression,
    check_statement,
    get_ttypes_from_token,
    TTypes,
  },
};

use sflyn_parser::{
  Error,
  For,
  Statement,
  tokens::{
    Array,
    HashMap,
    Types,
  },
};

pub fn check(
  for_s: &For,
  environment: &mut Environment,
) -> Result<TTypes, Error> {
  match check_expression(&for_s.get_condition(), environment) {
    Ok(obj) => {
      let mut for_environment: Environment = environment.clone();

      if obj.is_for_in() && obj.get_type().get_array().is_some() {
        let right_array: Array = obj.get_type().get_array().unwrap();

        if let Some(ttype) = get_ttypes_from_token(right_array.get_type(), obj.get_token()) {
          for_environment.store.set_type(
            obj.get_names()[0].clone(),
            ttype.clone(),
          );

          return check_statement(&for_s.get_body(), &mut for_environment);
        }
      } else if obj.is_for_of() && obj.get_type().get_hashmap().is_some() {
        if obj.get_names().len() == 2 {
          let hashmap: HashMap = obj.get_type().get_hashmap().unwrap();
          let mut return_ttype: Option<TTypes> = None;

          for (_, value) in hashmap.get_items().iter() {
            if let Some(ttype) = get_ttypes_from_token(value.clone(), value.clone()) {
              for_environment.store.set_type(
                obj.get_names()[0].clone(),
                TTypes::new_type(Types::STRING, String::from("string"), obj.get_token()),
              );

              for_environment.store.set_type(
                obj.get_names()[1].clone(),
                ttype.clone(),
              );

              match check_statement(&for_s.get_body(), &mut for_environment) {
                Ok(token) => {
                  return_ttype = Some(token);
                },
                Err(error) => {
                  return Err(error);
                },
              }
            }
          }

          if let Some(ttype) = return_ttype {
            return Ok(ttype);
          }
        }
      }

      Err(Error::from_token(
        String::from("invalid for."),
        for_s.get_token(),
      ))
    },
    Err(error) => Err(error),
  }
}

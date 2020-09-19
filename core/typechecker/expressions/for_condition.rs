use crate::{
  Environment,
  typechecker::{
    check_expression,
    TTypes,
  },
};

use sflyn_parser::{
  Error,
  Expression,
  ForCondition,
  tokens::{
    Token,
    Types,
  },
};

pub fn check(
  for_condition: &ForCondition,
  environment: &mut Environment,
) -> Result<TTypes, Error> {
  // Get the first infix expression.
  if let Some(first_infix) = for_condition.get_first().get_infix() {
    // Check if the first infix expression is a variable set.
    if first_infix.is_variable_set() {
      // Check if the left expression is an identifier.
      if first_infix.get_left().get_identifier().is_none() {
        return Err(Error::from_token(
          String::from("expect an identifier."),
          first_infix.get_left().token(),
        ));
      }

      let key_value = first_infix.get_left().get_identifier().unwrap().get_value();

      // Evaluate the right expression.
      match check_expression(&first_infix.get_right().unwrap(), environment) {
        Ok(first_ttype) => {
          // Set the data type to the left identifier.
          environment.store.set_type(key_value.clone(), first_ttype);

          // Evaluate the second infix expression.
          match check_expression(&for_condition.get_second(), environment) {
            Ok(second_ttype) => {
              // Check if the data type is a boolean.
              if second_ttype.get_type() != Types::BOOLEAN {
                return Err(Error::from_token(
                  String::from("this expression must be a boolean."),
                  for_condition.get_second().token(),
                ));
              }

              // Evaluate the third expression.
              match check_expression(&for_condition.get_third(), environment) {
                Ok(third_ttype) => {
                  let token = Token::from_value(format!("{}[]", third_ttype.get_value()).as_str(), 0, 0);

                  return Ok(TTypes::new_for_in(
                    token.token.get_type().unwrap(),
                    token.value,
                    for_condition.get_third().token(),
                    key_value.clone(),
                  ));
                },
                Err(error) => {
                  return Err(error);
                },
              }
            },
            Err(error) => {
              return Err(error);
            },
          }
        },
        Err(error) => {
          return Err(error);
        },
      }
    }

    return Err(Error::from_token(
      format!("`{}` is not a valid expression.", first_infix.get_token().value),
      first_infix.get_token(),
    ));
  }

  Err(Error::from_token(
    String::from("invalid for condition."),
    for_condition.get_token(),
  ))
}

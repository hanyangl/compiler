use crate::{
  Environment,
  typechecker::{
    check_statement,
    TTypes,
  },
};

use sflyn_parser::{
  Block,
  Error,
  Statement,
  tokens::Types,
};

pub fn check(
  block: &Block,
  environment: &mut Environment,
) -> Result<TTypes, Error> {
  let mut return_token: Option<TTypes> = None;

  for statement in block.get_statements().iter() {
    // Get the token for the current statement.
    match check_statement(statement, environment) {
      Ok(token) => {
        if statement.get_return().is_some() || statement.get_if_else().is_some() {
          if let Some(rtoken) = return_token.clone() {
            if rtoken.get_type() == token.get_type() {
              continue;
            } else {
              return Err(Error::from_token(
                format!("`{}` not satisfied the `{}` data type.", token.get_token().value, rtoken.get_token().value),
                token.get_token(),
              ));
            }
          }

          return_token = Some(token);
        }
      },
      Err(error) => {
        return Err(error);
      },
    }
  }

  if let Some(token) = return_token {
    return Ok(token);
  }

  Ok(TTypes::new_type(Types::VOID, String::from("void"), block.get_token()))
}

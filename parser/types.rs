use super::Environment;
use super::expressions::Expressions;
use super::tokens::Types;

pub fn expression_is_type(
  data_type: Types,
  expression: Box<Expressions>,
  environment: &mut Environment,
) -> bool {
  let data_type_exp = Types::from_expression(expression, environment);

  // Parse array
  match data_type.clone().get_array() {
    Some(array) => {
      if data_type_exp.token.clone().is_type() && !data_type_exp.token.clone().get_type().unwrap().is_array() {
        return array.types.contains(&data_type_exp);
      }
    },
    None => {},
  }

  match data_type_exp.token.get_type() {
    Some(data_type_exp) => data_type_exp == data_type,
    None => false,
  }
}

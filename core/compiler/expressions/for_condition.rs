use crate::{
  Environment,
  compiler::{
    Error,
    evaluate_expression,
    ForIn,
    Objects,
  },
};

use sflyn_parser::{
  Expression,
  Expressions,
  ForCondition,
};

fn cycle(
  key: String,
  second_expression: &Box<Expressions>,
  third_expression: &Box<Expressions>,
  environment: &mut Environment,
  elements: &mut Vec<Box<Objects>>,
) -> Box<Objects> {
  let mut third_obj = evaluate_expression(third_expression, environment);

  if third_obj.get_error().is_some() {
    return third_obj;
  }

  if let Some(return_obj) = third_obj.get_return() {
    third_obj = return_obj.get_value();
  }

  environment.store.set_object(key.clone(), third_obj.clone());

  let mut second_obj_bool = evaluate_expression(second_expression, environment);

  if second_obj_bool.get_error().is_some() {
    return second_obj_bool;
  }

  if let Some(return_obj) = second_obj_bool.get_return() {
    second_obj_bool = return_obj.get_value();
  }

  if let Some(boolean_obj) = second_obj_bool.get_boolean() {
    if boolean_obj.get_value() == true {
      elements.push(third_obj);

      return cycle(key, second_expression, third_expression, environment, elements);
    }
  }

  ForIn::new(key.clone(), elements.clone())
}

pub fn evaluate(
  for_condition: &ForCondition,
  environment: &mut Environment,
) -> Box<Objects> {
  if let Some(first_infix) = for_condition.get_first().get_infix() {
    if first_infix.is_variable_set() {
      let key: String;

      if let Some(identifier) = first_infix.get_left().get_identifier() {
        key = identifier.get_value();
      } else {
        return Error::new(
          format!("`{}` is not a valid expression.", first_infix.get_left().string()),
          first_infix.get_left().token(),
        );
      }

      let mut first_obj = evaluate_expression(&first_infix.get_right().unwrap(), environment);

      if first_obj.get_error().is_some() {
        return first_obj;
      }

      if let Some(return_obj) = first_obj.get_return() {
        first_obj = return_obj.get_value();
      }

      environment.store.set_object(key.clone(), first_obj.clone());

      return cycle(
        key,
        &for_condition.get_second(),
        &for_condition.get_third(),
        environment,
        &mut vec!(first_obj),
      );
    }
  }

  Error::new(
    String::from("invalid for condition."),
    for_condition.get_token(),
  )
}

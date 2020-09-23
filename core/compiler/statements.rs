mod import;

use crate::{
  compiler::{
    AnonymousFunction,
    Break,
    Boolean,
    Continue,
    evaluate_expression,
    Null,
    Objects,
    ReturnO,
    StringO,
  },
  Environment,
  Store,
};

use sflyn_parser::{
  Statement,
  Statements,
  tokens::Keywords,
};

pub fn evaluate_statement(
  statement: &Box<Statements>,
  environment: &mut Environment,
) -> Option<Box<Objects>> {
  // Block
  if let Some(block) = statement.get_block() {
    let mut result_object: Option<Box<Objects>> = None;

    for statement in block.get_statements().iter() {
      result_object = evaluate_statement(statement, environment);

      if let Some(object) = result_object.clone() {
        // Check if the result object is an error, return, break or continue.
        if object.get_error().is_some() ||
          object.get_return().is_some() ||
          object.get_break().is_some() ||
          object.get_continue().is_some() {
          break;
        }
      }
    }

    return result_object;
  }

  // Continue and break
  if let Some(continue_break) = statement.get_continue_break() {
    if continue_break.get_token().token.expect_keyword(&Keywords::CONTINUE) {
      return Some(Continue::new());
    } else if continue_break.get_token().token.expect_keyword(&Keywords::BREAK) {
      return Some(Break::new());
    }
  }

  // Export
  if let Some(export) = statement.get_export() {
    return evaluate_statement(&export.get_value(), environment);
  }

  // Expression
  if let Some(expression) = statement.get_expression() {
    return Some(evaluate_expression(&expression.get_expression(), environment));
  }

  // For
  if let Some(for_s) = statement.get_for() {
    let mut for_environment: Environment = environment.clone();

    for_environment.store = Store::from_store(&for_environment.store);

    let condition_obj = evaluate_expression(&for_s.get_condition(), &mut for_environment);

    if condition_obj.get_error().is_some() {
      return Some(condition_obj);
    }

    if let Some(for_in) = condition_obj.get_for_in() {
      if for_in.get_elements().len() > 0 {
        for obj in for_in.get_elements().iter() {
          let mut new_environment: Environment = environment.clone();

          new_environment.store = Store::from_store(&new_environment.store);

          new_environment.store.set_object(for_in.get_name(), obj.clone());

          if let Some(obj) = evaluate_statement(&for_s.get_body(), &mut new_environment) {
            if obj.get_error().is_some() {
              return Some(obj);
            }

            if obj.get_continue().is_some() {
              continue;
            }

            if obj.get_break().is_some() {
              break;
            }
          }

          new_environment.store.delete_object(&for_in.get_name());

          if let Some(outer) = new_environment.store.get_outer() {
            environment.store = outer;
          }
        }
      }
    } else if let Some(for_of) = condition_obj.get_for_of() {
      if for_of.get_names().len() == 2 {
        for item in for_of.get_data().iter() {
          let mut new_environment: Environment = environment.clone();

          new_environment.store = Store::from_store(&environment.store);

          new_environment.store.set_object(
            for_of.get_names()[0].clone(),
            StringO::new(item.key.clone()),
          );

          new_environment.store.set_object(
            for_of.get_names()[1].clone(),
            item.value.clone(),
          );

          if let Some(obj) = evaluate_statement(&for_s.get_body(), &mut new_environment) {
            if obj.get_error().is_some() {
              return Some(obj);
            }

            if obj.get_continue().is_some() {
              continue;
            }

            if obj.get_break().is_some() {
              break;
            }
          }

          if let Some(outer) = new_environment.store.get_outer() {
            environment.store = outer.clone();
          }
        }
      }
    }
  }

  // Function
  if let Some(function) = statement.get_function() {
    AnonymousFunction::add_arguments_to_environment(
      function.get_arguments(),
      environment,
    );

    let object = AnonymousFunction::new(
      true,
      function.get_arguments(),
      function.get_type(),
      function.get_body(),
      &environment.store,
    );

    // Add function object to the environment.
    environment.store.set_object(function.get_name().value, object);
  }

  // If else
  if let Some(if_else) = statement.get_if_else() {
    for condition in if_else.get_conditions().iter() {
      let object = evaluate_expression(&condition.get_condition(), environment);

      if Boolean::is_truthy(object) {
        return evaluate_statement(&condition.get_consequence(), environment);
      }
    }

    if let Some(alternative) = if_else.get_alternative() {
      return evaluate_statement(&alternative, environment);
    }
  }

  // Import
  if let Some(import_s) = statement.get_import() {
    return import::evaluate(import_s, environment);
  }

  // Return
  if let Some(return_s) = statement.get_return() {
    // Get the return value.
    if let Some(value) = return_s.get_value() {
      // Evaluate the return value.
      let object = evaluate_expression(&value, environment);

      // Check if the value object is an error.
      if object.get_error().is_some() {
        return Some(object);
      }

      return Some(ReturnO::new(object));
    }

    return Some(ReturnO::new(Null::new()));
  }

  // Variable
  if let Some(variable) = statement.get_variable() {
    // Get the variable value.
    if let Some(value) = variable.get_value() {
      // Evaluate the variable value.
      let object = evaluate_expression(&value, environment);

      // Check if the value object is an error.
      if object.get_error().is_some() {
        return Some(object);
      }

      environment.store.set_object(variable.get_name().value, object);
    }
  }

  // Default
  None
}

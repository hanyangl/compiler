use crate::data;
use crate::statements::{Identifier, expression};
use crate::parser::Parser;

#[derive(Debug, Clone)]
pub struct Variable {
  token: data::Token,
  name: Identifier,
  data_type: data::Token,
  value: Identifier,
}

impl Variable {
  /// Create an empty variable statement.
  pub fn empty() -> Variable {
    Variable {
      token: data::Token::empty(),
      name: Identifier::new(),
      data_type: data::Token::empty(),
      value: Identifier::new(),
    }
  }

  /// Create a variable statement with a token.
  pub fn new(token: data::Token) -> Variable {
    let mut variable = Variable::empty();

    variable.token = token;

    variable
  }

  /// Check if it is a valid variable statement.
  /// 
  /// ## Example:
  /// ```sf
  /// let username: string = "Sflyn";
  /// let decimal: number = 6;
  /// ```
  pub fn parse<'a>(parser: &'a mut Parser) -> (Variable, bool) {
    let mut statement = Variable::new(parser.current_token.clone());

    // Check if a valid variable name.
    if parser.expect_token(&data::Tokens::IDENTIFIER) == false {
      let token: data::Token = parser.peek_token.clone();
      let line = parser.get_error_line("let ");
      let mut message = format!("{} `{}` is not a valid variable name.", line, token.value);

      if token.sign == data::Signs::COLON {
        message = format!("{} you must put the variable name.", line);
      }

      parser.errors.push(message);

      return (Variable::empty(), false);
    }

    // Set the variable name to the statement.
    statement.name = Identifier::from_token(&parser.current_token.clone());

    // Check if the next character is a colon (:).
    if parser.expect_sign(&data::Signs::COLON) == false {
      let line = parser.get_error_line(format!("let {}", statement.name.value).as_str());

      parser.errors.push(format!("{} expect `:`, got `{}` instead.", line, parser.peek_token.value));

      return (Variable::empty(), false);
    }

    // Check if the next token is a valid data type.
    if parser.expect_token(&data::Tokens::TYPE) == false {
      let line = parser.get_error_line(format!("let {}: ", statement.name.value).as_str());

      parser.errors.push(format!("{} `{}` is not a valid data type.", line, parser.peek_token.value));

      return (Variable::empty(), false);
    }

    // Set the data type to the statement.
    statement.data_type = parser.current_token.clone();

    // Check if the next character is an assign sign (=).
    if parser.expect_sign(&data::Signs::ASSIGN) == false {
      let line = parser.get_error_line(format!("let {}: {} ", statement.name.value, statement.data_type.value).as_str());

      parser.errors.push(format!("{} expect `=`, got `{}` instead.", line, parser.peek_token.value));

      return (Variable::empty(), false);
    }

    // Check if the next character is not a semicolon (;).
    if parser.peek_token_is_sign(&data::Signs::SEMICOLON) == true {
      let line = parser.get_error_line(format!("let {}: {} = ", statement.name.value, statement.data_type.value).as_str());

      parser.errors.push(format!("{} you must set a value to the variable.", line));

      return (Variable::empty(), false);
    }

    // Get the variable value.
    let value: &data::Token = &parser.peek_token.clone();

    // Check if the variable value is the same type of the variable declaration.
    if expression::token_is_valid_type(&statement.data_type.data_type, value) == false {
      let line = parser.get_error_line(format!("let {}: {} = ", statement.name.value, statement.data_type.value).as_str());

      parser.errors.push(format!("{} `{}` is not a valid {}", line, value.value, statement.data_type.value));

      return (Variable::empty(), false);
    }

    // Go to the next token.
    parser.next_token();

    // Set the variable value to the statement.
    statement.value = Identifier::from_token(&parser.current_token.clone());

    // Check if the next character is a semicolon (;).
    if parser.expect_sign(&data::Signs::SEMICOLON) == false {
      let token = parser.peek_token.clone();
  
      let line = parser.get_error_line(
        format!(
          "let {}: {} = {}",
          statement.name.value,
          statement.data_type.value,
          statement.value.value,
        ).as_str(),
      );

      if token.token == data::Tokens::EOL {
        parser.errors.push(format!("{} must be ends with `;`.", line));
      } else {
        parser.errors.push(format!("{} expect `;`, got `{}` instead.", line, token.value));
      }

      return (Variable::empty(), false);
    }

    (statement, true)
  }

  /// Get the variable value.
  pub fn get_value(self) -> String {
    self.value.value
  }

  /// Parse the variable statement to a string.
  pub fn to_string(&self) -> String {
    format!(
      "{} {}: {} = {};",
      self.token.value,
      self.name.value,
      self.data_type.value,
      self.value.value,
    )
  }
}

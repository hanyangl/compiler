use crate::data;
use crate::expressions;
use crate::expressions::Expression;
use crate::parser::{Parser, precedence::Precedence, Expressions};
use crate::statements::Statement;

#[derive(Debug, Clone)]
pub struct ExpressionStatement {
  token: data::Token,
  expression: expressions::Identifier,
}

impl Statement for ExpressionStatement {
  fn new() -> ExpressionStatement {
    ExpressionStatement {
      token: data::Token::empty(),
      expression: expressions::Expression::new(),
    }
  }

  fn from_token(token: &data::Token) -> ExpressionStatement {
    let mut expression: ExpressionStatement = Statement::new();

    expression.token = token.clone();

    expression
  }

  fn string(self) -> String {
    self.expression.string()
  }
}

pub fn parse<'a>(parser: &'a mut Parser, precedence: Precedence) -> Box<Expressions> {
  let token: data::Token = parser.current_token.clone();

  let mut left: Box<Expressions> = Box::new(match token.token {
    // Parse identifiers and strings.
    data::Tokens::IDENTIFIER | data::Tokens::STRING => Expressions::DEFAULT(Expression::from_token(&token)),

    // Parse numbers.
    data::Tokens::INTEGER => {
      let (identifier, _) = expressions::integer::parse(parser);
      Expressions::INTEGER(identifier)
    },

    // Parse signs.
    data::Tokens::SIGN => match token.sign {
      // Parse '!' and '-' signs.
      data::Signs::NEGATION | data::Signs::MINUS => {
        let prefix = expressions::prefix::parser(parser);
        Expressions::PREFIX(prefix)
      },

      // Default
      _ => Expressions::DEFAULT(Expression::new()),
    },

    // Parse data types.
    data::Tokens::TYPE => match token.data_type {
      // Parse true and false values.
      data::Types::TRUE | data::Types::FALSE => Expressions::BOOLEAN(expressions::boolean::parse(parser)),

      // Default
      _ => Expressions::DEFAULT(Expression::new()),
    },

    // Default
    _ => Expressions::DEFAULT(Expression::new()),
  });

  while parser.peek_token_is_sign(&data::Signs::SEMICOLON) == false && precedence < parser.peek_precedence() {
    let peek_token: data::Token = parser.peek_token.clone();

    match peek_token.sign {
      data::Signs::PLUS |
      data::Signs::MINUS |
      data::Signs::DIVIDE |
      data::Signs::MULTIPLY |
      data::Signs::EQUAL |
      data::Signs::EQUALTYPE |
      data::Signs::NOTEQUAL |
      data::Signs::NOTEQUALTYPE |
      data::Signs::LESSTHAN |
      data::Signs::LESSOREQUALTHAN |
      data::Signs::GREATERTHAN |
      data::Signs::GREATEROREQUALTHAN => {
        parser.next_token();

        left = Box::new(Expressions::INFIX(expressions::infix::parse(parser, left)));
      },

      _ => break,
    }
  }

  left
}

/// Comprobate if a value is of a specific type.
/// 
/// ## Example
/// ```
/// use crate::data::{Token, Types};
/// use crate::utils::expression;
/// 
/// let value = Token::from_value(String::from("10"), 1, 1);
/// 
/// let is_string = expression::token_is_valid_type(&Types::STRING, &value);
/// // Return: false
/// 
/// let is_number = expression::token_is_valid_type(&Types::NUMBER, &value);
/// // Return true
/// ```
pub fn token_is_valid_type(data_type: &data::Types, token: &data::Token) -> bool {
  match data_type {
    data::Types::UNDEFINED => token.token == data::Tokens::TYPE && token.data_type == data::Types::UNDEFINED,
    data::Types::NULL => token.token == data::Tokens::TYPE && token.data_type == data::Types::NULL,

    data::Types::STRING => token.token == data::Tokens::STRING,
    data::Types::NUMBER => token.token == data::Tokens::INTEGER,

    data::Types::BOOLEAN => (
      token.token == data::Tokens::TYPE && (
        token.data_type == data::Types::TRUE ||
        token.data_type == data::Types::FALSE
      )
    ),

    _ => false,
  }
}

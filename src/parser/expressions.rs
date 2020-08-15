use crate::expressions;
use crate::expressions::Expression;

#[derive(Debug, Clone)]
pub enum Expressions {
  BOOLEAN(expressions::boolean::Boolean),
  DEFAULT(expressions::Identifier),
  INFIX(expressions::infix::Infix),
  INTEGER(expressions::integer::Integer),
  PREFIX(expressions::prefix::Prefix),
}

impl Expressions {
  pub fn string(self) -> String {
    match self {
      Expressions::BOOLEAN(x) => x.string(),
      Expressions::DEFAULT(x) => x.string(),
      Expressions::INFIX(x) => x.string(),
      Expressions::INTEGER(x) => x.string(),
      Expressions::PREFIX(x) => x.string(),
    }
  }
}

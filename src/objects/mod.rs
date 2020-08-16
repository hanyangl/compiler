pub mod boolean;
pub mod integer;

#[derive(Debug, Clone, PartialEq)]
pub enum ObjectType {
  NULL,
  ERROR,

  INTEGER,
  BOOLEAN,
  STRING,

  RETURNVALUE,

  FUNCTION,
}

#[derive(Debug, Clone, PartialEq)]
pub struct HashKey {
  object_type: ObjectType,
  value: u64,
}

pub trait Hashable {
  fn hashkey(self) -> HashKey;
}

pub trait Object {
  fn object_type(&self) -> ObjectType;
  fn string(self) -> String;
}

#[derive(Debug, Clone, PartialEq)]
pub enum Objects {
  INTEGER(integer::Integer),
  BOOLEAN(boolean::Boolean),
}

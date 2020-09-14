use sflyn_parser::{
  Expressions,
  tokens::{
    Token,
    Types,
  },
};

use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum TType {
  NONE,
  INTERFACE,
  FUNCTION,
  HASHMAP,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TTypes {
  ttype: TType,
  data_type: Types,
  type_value: String,
  token: Token,

  arguments: Vec<Box<Expressions>>,
  methods: HashMap<String, TTypes>,
}

impl TTypes {
  pub fn new(
    ttype: TType,
    data_type: Types,
    type_value: String,
    token: Token,
    arguments: Vec<Box<Expressions>>,
    methods: HashMap<String, TTypes>,
  ) -> Self {
    Self {
      ttype,
      data_type,
      type_value,
      token,
      arguments,
      methods,
    }
  }

  pub fn new_type(data_type: Types, type_value: String, token: Token) -> Self {
    Self::new(TType::NONE, data_type, type_value, token, Vec::new(), HashMap::new())
  }

  pub fn new_interface(data_type: Types, type_value: String, token: Token, methods: HashMap<String, TTypes>) -> Self {
    Self::new(TType::INTERFACE, data_type, type_value, token, Vec::new(), methods)
  } 

  pub fn new_function(data_type: Types, type_value: String, token: Token, arguments: Vec<Box<Expressions>>) -> Self {
    Self::new(TType::FUNCTION, data_type, type_value, token, arguments, HashMap::new())
  }

  pub fn new_hashmap(data_type: Types, type_value: String, token: Token, methods: HashMap<String, TTypes>) -> Self {
    Self::new(TType::HASHMAP, data_type, type_value, token, Vec::new(), methods)
  }

  pub fn is_interface(&self) -> bool {
    self.ttype == TType::INTERFACE
  }

  pub fn is_function(&self) -> bool {
    self.ttype == TType::FUNCTION
  }

  pub fn is_hashmap(&self) -> bool {
    self.ttype == TType::HASHMAP
  }

  pub fn get_type(&self) -> Types {
    self.data_type.clone()
  }

  pub fn get_value(&self) -> String {
    self.type_value.clone()
  }

  pub fn get_token(&self) -> Token {
    self.token.clone()
  }

  pub fn get_arguments(&self) -> Vec<Box<Expressions>> {
    self.arguments.clone()
  }

  pub fn get_methods(&self) -> HashMap<String, TTypes> {
    self.methods.clone()
  }
}

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
  ARRAY,

  FORIN,
  FOROF,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TTypes {
  ttype: TType,
  data_type: Types,
  type_value: String,
  token: Token,

  names: Vec<String>,
  arguments: Vec<Box<Expressions>>,
  methods: HashMap<String, TTypes>,
}

impl TTypes {
  pub fn new(
    ttype: TType,
    data_type: Types,
    type_value: String,
    token: Token,
    names: Vec<String>,
    arguments: Vec<Box<Expressions>>,
    methods: HashMap<String, TTypes>,
  ) -> Self {
    Self {
      ttype,
      data_type,
      type_value,
      token,
      names,
      arguments,
      methods,
    }
  }

  pub fn new_type(
    data_type: Types,
    type_value: String,
    token: Token,
  ) -> Self {
    Self::new(
      TType::NONE,
      data_type,
      type_value,
      token,
      Vec::new(),
      Vec::new(),
      HashMap::new(),
    )
  }

  pub fn new_interface(
    data_type: Types,
    type_value: String,
    token: Token,
    methods: HashMap<String, TTypes>,
  ) -> Self {
    Self::new(
      TType::INTERFACE,
      data_type,
      type_value,
      token,
      Vec::new(),
      Vec::new(),
      methods,
    )
  } 

  pub fn new_function(
    data_type: Types,
    type_value: String,
    token: Token,
    arguments: Vec<Box<Expressions>>,
  ) -> Self {
    Self::new(
      TType::FUNCTION,
      data_type,
      type_value,
      token,
      Vec::new(),
      arguments,
      HashMap::new(),
    )
  }

  pub fn new_hashmap(
    data_type: Types,
    type_value: String,
    token: Token,
    methods: HashMap<String, TTypes>,
  ) -> Self {
    Self::new(
      TType::HASHMAP,
      data_type,
      type_value,
      token,
      Vec::new(),
      Vec::new(),
      methods,
    )
  }

  pub fn new_array(
    data_type: Types,
    type_value: String,
    token: Token,
  ) -> Self {
    Self::new(
      TType::ARRAY,
      data_type,
      type_value,
      token,
      Vec::new(),
      Vec::new(),
      HashMap::new(),
    )
  }

  pub fn new_for_in(
    data_type: Types,
    type_value: String,
    token: Token,
    name: String,
  ) -> Self {
    Self::new(
      TType::FORIN,
      data_type,
      type_value,
      token,
      [name].to_vec(),
      Vec::new(),
      HashMap::new(),
    )
  }

  pub fn new_for_of(
    data_type: Types,
    type_value: String,
    token: Token,
    methods: HashMap<String, TTypes>,
    names: Vec<String>,
  ) -> Self {
    Self::new(
      TType::FOROF,
      data_type,
      type_value,
      token,
      names,
      Vec::new(),
      methods,
    )
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

  pub fn is_array(&self) -> bool {
    self.ttype == TType::ARRAY
  }

  pub fn is_for_in(&self) -> bool {
    self.ttype == TType::FORIN
  }

  pub fn is_for_of(&self) -> bool {
    self.ttype == TType::FOROF
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

  pub fn get_names(&self) -> Vec<String> {
    self.names.clone()
  }

  pub fn get_arguments(&self) -> Vec<Box<Expressions>> {
    self.arguments.clone()
  }

  pub fn get_methods(&self) -> HashMap<String, TTypes> {
    self.methods.clone()
  }
}

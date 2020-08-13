use crate::data;
use crate::parser::Lexer;
use crate::statements;
use crate::utils::repeat_character;

#[derive(Debug)]
pub struct Parser {
  lexer: Lexer,
  pub errors: Vec<String>,

  pub current_token: data::Token,
  pub peek_token: data::Token,
}

impl Parser {
  pub fn new(lexer: Lexer) -> Parser {
    let mut parser = Parser {
      lexer,
      errors: Vec::new(),

      current_token: data::Token::empty(),
      peek_token: data::Token::empty(),
    };

    parser.next_token();
    parser.next_token();

    parser
  }

  pub fn next_token(&mut self) {
    std::mem::swap(&mut self.current_token, &mut self.peek_token);
    self.peek_token = self.lexer.read_token();
  }

  pub fn current_token_is<'a>(&mut self, token: &'a data::Tokens) -> bool {
    &self.current_token.token == token
  }

  pub fn peek_token_is<'a>(&mut self, token: &'a data::Tokens) -> bool {
    &self.peek_token.token == token
  }

  pub fn expect_token<'a>(&mut self, token: &'a data::Tokens) -> bool {
    if self.peek_token_is(token) == true {
      self.next_token();
      true
    } else {
      false
    }
  }

  pub fn current_token_is_keyword<'a>(&mut self, keyword: &'a data::Keywords) -> bool {
    self.current_token_is(&data::Tokens::KEYWORD) && &self.current_token.keyword == keyword
  }

  pub fn peek_token_is_keyword<'a>(&mut self, keyword: &'a data::Keywords) -> bool {
    self.peek_token_is(&data::Tokens::KEYWORD) && &self.peek_token.keyword == keyword
  }

  pub fn expect_keyword<'a>(&mut self, keyword: &'a data::Keywords) -> bool {
    if self.peek_token_is_keyword(keyword) == true {
      self.next_token();
      true
    } else {
      false
    }
  }

  pub fn current_token_is_sign<'a>(&mut self, sign: &'a data::Signs) -> bool {
    self.current_token_is(&data::Tokens::SIGN) && &self.current_token.sign == sign
  }

  pub fn peek_token_is_sign<'a>(&mut self, sign: &'a data::Signs) -> bool {
    self.peek_token_is(&data::Tokens::SIGN) && &self.peek_token.sign == sign
  }

  pub fn expect_sign<'a>(&mut self, sign: &'a data::Signs) -> bool {
    if self.peek_token_is_sign(sign) == true {
      self.next_token();
      true
    } else {
      false
    }
  }

  pub fn current_token_is_type<'a>(&mut self, data_type: &'a data::Types) -> bool {
    self.current_token_is(&data::Tokens::TYPE) && &self.current_token.data_type == data_type
  }

  pub fn peek_token_is_type<'a>(&mut self, data_type: &'a data::Types) -> bool {
    self.peek_token_is(&data::Tokens::TYPE) && &self.peek_token.data_type == data_type
  }

  pub fn expect_type<'a>(&mut self, data_type: &'a data::Types) -> bool {
    if self.peek_token_is_type(data_type) == true {
      self.next_token();
      true
    } else {
      false
    }
  }

  pub fn get_error_line(&mut self, line_message: &str) -> String {
    let token = self.peek_token.clone();
    let line = format!("{} | {}", token.line, line_message);
    let mut value = token.value;

    if token.token == data::Tokens::EOL {
      value = String::from(" ");
    }

    format!("{}{}\n{}{}", line, value, repeat_character(line.len(), " "), repeat_character(value.len(), "^"))
  }

  pub fn parse_program(&mut self) {
    while self.current_token.token != data::Tokens::EOF {
      self.parse_statement();
      self.next_token();
    }
  }

  pub fn parse_statement(&mut self) {
    match self.current_token.token {
      data::Tokens::KEYWORD => {
        match self.current_token.keyword {
          data::Keywords::LET => {
            statements::Variable::parse(self);
          },
          _ => {},
        }
      },
      _ => {},
    }
  }
}

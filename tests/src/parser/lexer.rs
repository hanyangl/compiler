#[cfg(test)]
use sflyn_parser::tokens::{Token, Tokens, Signs};

#[cfg(test)]
use super::generate_lexer;

#[cfg(test)]
pub fn test_lexer(value: String, expects: Vec<Token>) {
  let mut lexer = generate_lexer(value.as_str());

  for expect in expects.iter() {
    assert_eq!(lexer.read_next_token(), expect.clone());
  }

  assert_eq!(lexer.current_position, lexer.file_content.len());
  assert_eq!(lexer.current_character, 0);
}

#[cfg(test)]
fn lexer_sign(value: &str, sign: Signs) {
  let mut tokens = Vec::new();

  tokens.push(Token::new(Box::new(Tokens::SIGN(sign)), value.to_string(), 1, 1));

  test_lexer(value.to_string(), tokens);
}

#[cfg(test)]
fn get_identifier(value: &str, line: usize, position: usize) -> Token {
  Token::new(Box::new(Tokens::IDENTIFIER), value.to_string(), line, position)
}

#[cfg(test)]
fn get_string(value: &str, line: usize, position: usize) -> Token {
  Token::new(Box::new(Tokens::STRING), value.to_string(), line, position)
}

#[cfg(test)]
fn get_number(value: u64, line: usize, position: usize) -> Token {
  Token::new(Box::new(Tokens::NUMBER), value.to_string(), line, position)
}

#[cfg(test)]
fn get_eof(line: usize, position: usize) -> Token {
  Token::new(Box::new(Tokens::EOF), String::new(), line, position)
}

#[cfg(test)]
pub fn let_lang_string(keyword: &str) -> Vec<Token> {
  let mut let_tokens = Vec::new();

  let_tokens.push(Token::from_value(keyword.to_string(), 1, 1));
  let_tokens.push(get_identifier("lang", 1, keyword.len() + 2));
  let_tokens.push(Token::from_value(":".to_string(), 1, keyword.len() + 6));
  let_tokens.push(Token::from_value("string".to_string(), 1, keyword.len() + 8));
  let_tokens.push(Token::from_value("=".to_string(), 1, keyword.len() + 15));
  let_tokens.push(get_string("'Sflyn'", 1, keyword.len() + 17));
  let_tokens.push(Token::from_value(";".to_string(), 1, keyword.len() + 24));
  let_tokens.push(get_eof(1, keyword.len() + 25));

  let_tokens
}

#[cfg(test)]
pub fn let_lang2_string(keyword: &str) -> Vec<Token> {
  let mut let_tokens = Vec::new();

  let_tokens.push(Token::from_value(keyword.to_string(), 1, 1));
  let_tokens.push(get_identifier("lang2", 1, keyword.len() + 2));
  let_tokens.push(Token::from_value("=".to_string(), 1, keyword.len() + 8));
  let_tokens.push(get_string("'Sflyn'", 1, keyword.len() + 10));
  let_tokens.push(Token::from_value(";".to_string(), 1, keyword.len() + 17));
  let_tokens.push(get_eof(1, keyword.len() + 18));

  let_tokens
}

#[cfg(test)]
pub fn let_lang3_string(keyword: &str) -> Vec<Token> {
  let mut let_tokens = Vec::new();

  let_tokens.push(Token::from_value(keyword.to_string(), 1, 1));
  let_tokens.push(get_identifier("lang3", 1, keyword.len() + 2));
  let_tokens.push(Token::from_value("=".to_string(), 1, keyword.len() + 8));
  let_tokens.push(get_string("'Sflyn'", 1, keyword.len() + 10));
  let_tokens.push(Token::from_value("+".to_string(), 1, keyword.len() + 18));
  let_tokens.push(get_number(10, 1, keyword.len() + 20));
  let_tokens.push(Token::from_value(";".to_string(), 1, keyword.len() + 22));
  let_tokens.push(get_eof(1, keyword.len() + 23));

  let_tokens
}

#[cfg(test)]
pub fn let_two_number(keyword: &str) -> Vec<Token> {
  let mut let_tokens = Vec::new();

  let_tokens.push(Token::from_value(keyword.to_string(), 1, 1));
  let_tokens.push(get_identifier("two", 1, keyword.len() + 2));
  let_tokens.push(Token::from_value(":".to_string(), 1, keyword.len() + 5));
  let_tokens.push(Token::from_value("number".to_string(), 1, keyword.len() + 7));
  let_tokens.push(Token::from_value("=".to_string(), 1, keyword.len() + 14));
  let_tokens.push(get_number(2, 1, keyword.len() + 16));
  let_tokens.push(Token::from_value(";".to_string(), 1, keyword.len() + 17));
  let_tokens.push(get_eof(1, keyword.len() + 18));

  let_tokens
}

#[cfg(test)]
pub fn let_three_number(keyword: &str) -> Vec<Token> {
  let mut let_tokens = Vec::new();

  let_tokens.push(Token::from_value(keyword.to_string(), 1, 1));
  let_tokens.push(get_identifier("three", 1, keyword.len() + 2));
  let_tokens.push(Token::from_value("=".to_string(), 1, keyword.len() + 8));
  let_tokens.push(get_number(3, 1, keyword.len() + 10));
  let_tokens.push(Token::from_value(";".to_string(), 1, keyword.len() + 11));
  let_tokens.push(get_eof(1, keyword.len() + 12));

  let_tokens
}

#[cfg(test)]
pub fn let_four_number(keyword: &str) -> Vec<Token> {
  let mut let_tokens = Vec::new();

  let_tokens.push(Token::from_value(keyword.to_string(), 1, 1));
  let_tokens.push(get_identifier("four", 1, keyword.len() + 2));
  let_tokens.push(Token::from_value("=".to_string(), 1, keyword.len() + 7));
  let_tokens.push(get_number(3, 1, keyword.len() + 9));
  let_tokens.push(Token::from_value("+".to_string(), 1, keyword.len() + 11));
  let_tokens.push(get_number(1, 1, keyword.len() + 13));
  let_tokens.push(Token::from_value(";".to_string(), 1, keyword.len() + 14));
  let_tokens.push(get_eof(1, keyword.len() + 15));

  let_tokens
}

#[cfg(test)]
pub fn let_is_lexer_boolean(keyword: &str) -> Vec<Token> {
  let mut let_tokens = Vec::new();

  let_tokens.push(Token::from_value(keyword.to_string(), 1, 1));
  let_tokens.push(get_identifier("is_lexer", 1, keyword.len() + 2));
  let_tokens.push(Token::from_value(":".to_string(), 1, keyword.len() + 10));
  let_tokens.push(Token::from_value("boolean".to_string(), 1, keyword.len() + 12));
  let_tokens.push(Token::from_value("=".to_string(), 1, keyword.len() + 20));
  let_tokens.push(Token::from_value("true".to_string(), 1, keyword.len() + 22));
  let_tokens.push(Token::from_value(";".to_string(), 1, keyword.len() + 26));
  let_tokens.push(get_eof(1, keyword.len() + 27));

  let_tokens
}

#[cfg(test)]
pub fn let_is_lexer2_boolean(keyword: &str) -> Vec<Token> {
  let mut let_tokens = Vec::new();

  let_tokens.push(Token::from_value(keyword.to_string(), 1, 1));
  let_tokens.push(get_identifier("is_lexer2", 1, keyword.len() + 2));
  let_tokens.push(Token::from_value("=".to_string(), 1, keyword.len() + 12));
  let_tokens.push(Token::from_value("true".to_string(), 1, keyword.len() + 14));
  let_tokens.push(Token::from_value(";".to_string(), 1, keyword.len() + 18));
  let_tokens.push(get_eof(1, keyword.len() + 19));

  let_tokens
}

#[cfg(test)]
fn lexer_variable(keyword: &str) {
  // String variables
  test_lexer(format!("{} lang: string = 'Sflyn';", keyword), let_lang_string(keyword));
  test_lexer(format!("{} lang2 = 'Sflyn';", keyword), let_lang2_string(keyword));
  test_lexer(format!("{} lang3 = 'Sflyn' + 10;", keyword), let_lang3_string(keyword));

  // Number variables
  test_lexer(format!("{} two: number = 2;", keyword), let_two_number(keyword)); 
  test_lexer(format!("{} three = 3;", keyword), let_three_number(keyword));
  test_lexer(format!("{} four = 3 + 1;", keyword), let_four_number(keyword));

  // Boolean variable
  test_lexer(format!("{} is_lexer: boolean = true;", keyword), let_is_lexer_boolean(keyword));
  test_lexer(format!("{} is_lexer2 = true;", keyword), let_is_lexer2_boolean(keyword));
}

#[cfg(test)]
fn lexer_function() {
  let mut function_tokens = Vec::new();

  function_tokens.push(Token::from_value("function".to_string(), 1, 1));
  function_tokens.push(get_identifier("say_hi", 1, 10));
  function_tokens.push(Token::from_value("(".to_string(), 1, 16));
  function_tokens.push(get_identifier("name", 1, 17));
  function_tokens.push(Token::from_value(":".to_string(), 1, 21));
  function_tokens.push(Token::from_value("string".to_string(), 1, 23));
  function_tokens.push(Token::from_value(")".to_string(), 1, 29));
  function_tokens.push(Token::from_value(":".to_string(), 1, 30));
  function_tokens.push(Token::from_value("string".to_string(), 1, 32));
  function_tokens.push(Token::from_value("{".to_string(), 1, 39));
  function_tokens.push(Token::from_value("\n".to_string(), 1, 40));

  function_tokens.push(Token::from_value("return".to_string(), 2, 1));
  function_tokens.push(get_string("'Hello, '", 2, 8));
  function_tokens.push(Token::from_value("+".to_string(), 2, 18));
  function_tokens.push(get_identifier("name", 2, 20));
  function_tokens.push(Token::from_value("+".to_string(), 2, 25));
  function_tokens.push(get_string("'!'", 2, 27));
  function_tokens.push(Token::from_value(";".to_string(), 2, 30));
  function_tokens.push(Token::from_value("\n".to_string(), 2, 31));

  function_tokens.push(Token::from_value("}".to_string(), 3, 1));
  function_tokens.push(get_eof(3, 2));

  test_lexer(
    format!(
      "{}\n{}\n{}",
      "function say_hi(name: string): string {",
      "return 'Hello, ' + name + '!';",
      "}",
    ),
    function_tokens.clone(),
  );
}

#[test]
fn parser_lexer() {
  // Signs
  lexer_sign(",", Signs::COMMA);
  lexer_sign(":", Signs::COLON);
  lexer_sign(";", Signs::SEMICOLON);

  lexer_sign("(", Signs::LEFTPARENTHESES);
  lexer_sign(")", Signs::RIGHTPARENTHESES);

  lexer_sign("[", Signs::LEFTBRACKET);
  lexer_sign("]", Signs::RIGHTBRACKET);

  lexer_sign("{", Signs::LEFTBRACE);
  lexer_sign("}", Signs::RIGHTBRACE);

  lexer_sign("=", Signs::ASSIGN);
  lexer_sign("+=", Signs::PLUSASSIGN);
  lexer_sign("-=", Signs::MINUSASSIGN);
  lexer_sign("*=", Signs::MULTIPLYASSIGN);
  lexer_sign("/=", Signs::DIVIDEASSIGN);

  lexer_sign("++", Signs::PLUSPLUS);
  lexer_sign("--", Signs::MINUSMINUS);

  lexer_sign("==", Signs::EQUAL);
  lexer_sign("===", Signs::EQUALTYPE);
  lexer_sign("!=", Signs::NOTEQUAL);
  lexer_sign("!==", Signs::NOTEQUALTYPE);

  lexer_sign("<", Signs::LESSTHAN);
  lexer_sign("<=", Signs::LESSOREQUALTHAN);
  lexer_sign(">", Signs::GREATERTHAN);
  lexer_sign(">=", Signs::GREATEROREQUALTHAN);

  lexer_sign("&&", Signs::AND);
  lexer_sign("||", Signs::OR);

  lexer_sign("+", Signs::PLUS);
  lexer_sign("-", Signs::MINUS);
  lexer_sign("*", Signs::MULTIPLY);
  lexer_sign("/", Signs::DIVIDE);
  lexer_sign("**", Signs::EMPOWERMENT);
  lexer_sign("%", Signs::MODULE);

  lexer_sign("!", Signs::NEGATION);
  lexer_sign("->", Signs::ARROW);

  // Variables
  lexer_variable("let");
  lexer_variable("const");

  // Function
  lexer_function();
}

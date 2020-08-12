pub mod parser;
pub mod utils;

fn main() {
  let text = String::from("let x: number = 10;\nprint(x);");
  let mut lexer = parser::Lexer::new(text);

  loop {
    let next_token = lexer.next_token();
    println!("{:?}", next_token);

    if next_token.token_type == parser::token::TokenType::EOF {
      break;
    }
  }
}
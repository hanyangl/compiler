pub mod data;
pub mod parser;
pub mod utils;

fn main() {
  let text = String::from("let x: number = 10;\nprint(x);\n\nif (x == 10) { print('Is ten'); } else { x = 10; }");
  let mut cursor = parser::Cursor::new(text);

  loop {
    let token = cursor.read_token();
    println!("{:?}", token);

    if token.token == data::Tokens::EOF {
      break;
    }
  }
}

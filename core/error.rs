use sflyn_parser::{
  Error,
  File,
};

use super::utils::repeat_character;

pub fn show_error(
  file: File,
  error: Error,
) {
  if error.line > 0 {
    let line = file.get_lines()[error.line - 1].to_string();

    println!(
      "{} | {}\n{} | {}{} {}",
      error.line,
      line,
      repeat_character(error.line.to_string().len(), " "),
      repeat_character(error.start_position - 1, " "),
      repeat_character(error.end_position - error.start_position, "^"),
      error.message,
    );
  } else {
    println!("{}", error.message);
  }
}

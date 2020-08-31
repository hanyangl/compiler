use super::{Environment, utils::repeat_character};

pub fn start() -> i32 {
  let mut environment = Environment::new();

  if environment.arguments.flag_version {
    println!("Sflyn v1.0.0");
    return 0;
  }

  match sflyn_parser::run(environment.arguments.file) {
    Ok(file) => {
      environment.files.push(file.clone());

      println!("File parsed!");
    },

    Err((error, file)) => {
      if error.line == 0 {
        println!("{}", error.message);
      } else if let Some(file) = file {
        let line = file.get_lines()[error.line - 1].clone();

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

      return 1;
    },
  }

  0
}

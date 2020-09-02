
#[derive(Debug, Clone, PartialEq)]
pub struct Arguments {
  pub file: String,

  pub flag_version: bool,
}

impl Arguments {
  pub fn new() -> Arguments {
    Arguments {
      file: String::new(),

      flag_version: false,
    }
  }

  pub fn from_console() -> Arguments {
    let arguments: Vec<String> = std::env::args().collect();

    if arguments.len() == 1 {
      Arguments::new()
    } else {
      Arguments {
        file: if std::path::Path::new(&arguments[1].clone()).exists() {
          arguments[1].clone()
        } else {
          String::new()
        },

        flag_version: arguments.contains(&String::from("--version")),
      }
    }
  }
}

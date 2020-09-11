use super::{
  compiler,
  Environment,
  error::show_error,
  typechecker,
  utils::get_sflyn_path,
};

pub fn run_file(
  file_name: String,
  environment: &mut Environment,
) -> i32 {
  match sflyn_parser::run(file_name) {
    Ok(file) => {
      environment.current_file = Some(file.clone());

      let mut file = file.clone();

      if file.statements.len() > 0 {
        if let Err(_) = typechecker::run(&mut file, environment) {
          return 1;
        }

        compiler::run(file.clone(), environment);
      }

      environment.files.push(file.clone());
    },

    Err((error, file)) => {
      if let Some(file) = file {
        show_error(file, error);
      } else {
        println!("{}", error.message);
      }

      return 1;
    },
  }

  0
}

pub fn start() -> i32 {
  let mut environment = Environment::new();

  let sflyn_path: String = get_sflyn_path();

  if sflyn_path.len() == 0 {
    println!("You need set the `SFLYN_PATH` variable in the environment.");
    return 1;
  }

  if environment.arguments.flag_version {
    println!("Sflyn v1.0.0");
    return 0;
  }

  if environment.load_stdlibs(sflyn_path) != 0 {
    return 1;
  }

  run_file(environment.arguments.file.clone(), &mut environment)
}

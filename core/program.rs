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
  with_typechecker: bool,
  with_compiler: bool,
  with_stdlib: bool,
) -> i32 {
  match sflyn_parser::run(file_name) {
    Ok(file) => {
      environment.current_file = Some(file.clone());

      let mut file = file.clone();

      if file.statements.len() > 0 {
        if with_typechecker {
          if let Err(_) = typechecker::run(&mut file, environment, with_stdlib) {
            return 1;
          }
        }

        if with_compiler {
          compiler::run(file.clone(), environment, with_stdlib);
        }
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

  run_file(environment.arguments.file.clone(), &mut environment, true, true, true)
}

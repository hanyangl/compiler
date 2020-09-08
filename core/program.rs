use super::{
  compiler,
  Environment,
  error::show_error,
  typechecker,
  utils::get_sflyn_path,
};

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

  match sflyn_parser::run(environment.arguments.file.clone()) {
    Ok(file) => {
      let mut file = file.clone();

      if file.statements.len() > 0 {
        if let Err(_) = typechecker::run(&mut file, &mut environment) {
          return 1;
        }

        compiler::run(file.clone(), &mut environment);
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

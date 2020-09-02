use super::{
  compiler,
  Environment,
  error::show_error,
  typechecker,
};

pub fn start() -> i32 {
  let mut environment = Environment::new();

  if environment.arguments.flag_version {
    println!("Sflyn v1.0.0");
    return 0;
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
      }

      return 1;
    },
  }

  0
}

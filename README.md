# Sflyn Compiler
Compiler for the **Sflyn** programming language.

## [Documentation](https://github.com/sflynlang/docs)

## Sflyn path
You need set the root directory in the `SFLYN_PATH` environment variable. In this directory you need put the standard library in a directory called `std`. For example, if you `SFLYN_PATH` is `/home/sflyn/.sflyn/`, the standard library must be in `/home/sflyn/.sflyn/std/`.

## Testing examples files
You must have the `SFLYN_PATH` configured, then run the following command:

```sh
cd cli && cargo run (file path)

# Examples
cd cli && cargo run ../examples/hello_world.sf
cd cli && cargo run ../examples/variables/let.sf
```

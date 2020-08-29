# Sflyn Compiler

[![Twitter](https://img.shields.io/twitter/follow/sflynlang?style=social)](https://twitter.com/sflynlang)
![Stars](https://img.shields.io/github/stars/sflynlang/compiler?style=social)
![Forks](https://img.shields.io/github/forks/sflynlang/compiler?style=social)

[![Discord](https://img.shields.io/discord/743142851642261644?label=Discord)](https://discord.gg/zkTMFSr)
![License](https://img.shields.io/github/license/sflynlang/compiler)
![Downloads](https://img.shields.io/github/downloads/sflynlang/compiler/total)
![Code Size](https://img.shields.io/github/languages/code-size/sflynlang/compiler)

Compiler for the **Sflyn** programming language.

## [Documentation](https://github.com/sflynlang/docs)

## Sflyn path
You need set the root directory in the `SFLYN_PATH` environment variable. In this directory you need put the standard library in a directory called `std`. For example, if you `SFLYN_PATH` is `/home/sflyn/.sflyn/`, the standard library must be in `/home/sflyn/.sflyn/std/`.

## Testing examples files
You must have the `SFLYN_PATH` configured, then run the following command:

```sh
cargo run (file path)

# Examples
cargo run ./examples/hello_world.sf
cargo run ./examples/variables/let.sf
```

# Sflynlang Compiler
![Build State](https://img.shields.io/github/workflow/status/sflynlang/compiler/Rust%20CI)
![Code Size](https://img.shields.io/github/languages/code-size/sflynlang/compiler)
![Last Release](https://img.shields.io/github/v/release/sflynlang/compiler)
![Downloads](https://img.shields.io/github/downloads/sflynlang/compiler/total)

This repository contains source code for the Sflynlang compiler. It is written in Rustlang.

## What is Sflynlang?
Sflynlang is a multiparadigm and cross-platform programming language. The principal focus is to have syntax like TypeScript but natively on the browser or more applications; the syntax strives be easier and familiar to everyone.

## Pre-requisites
* [Rustlang](https://www.rust-lang.org/)
* [Rustup](https://rustup.rs/)
* [Rustlang VS Code Extension (Optional)](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust)

## Installation
1. Go to [releases](https://github.com/sflynlang/compiler/releases) and click on the first release you find.

2. Download the compiler binary file for your operating system:

* Linux: `sflyn-x86_64-unknown-linux-gnu.zip`
* Windows: `sflyn-x86_64-pc-windows-msvc.zip`
* MacOS: `sflyn-x86_64-apple-darwin.zip`

3. Download the STD library and the rest of dependencies (the `sflyn-src.zip` file)

4. Extract the `sflyn-src.zip` to `C:/sflynlang`, `$HOME/sflynlang`, or a directory of your choice.

5. Set the path with the STD as `SFLYN_PATH` in your the environment.

> Example: `export SFLYN_PATH=$HOME/sflynlang`.

6. Add the sflynlang compiler binary file to `SFLYN_PATH/bin/`.

7. Set the executable path in your environment's PATH.

> Example: `export PATH=$PATH:$HOME/sflynlang/bin`.

8. Now you can run Sflyn code! See [Getting Started](#Getting-Started).

## How to Contribute
Please read the [Contribution Guidelines](./CONTRIBUTING.md) and [Code of Conduct](./CODE_OF_CONDUCT.md) before contributing.

## Changelog
View the latest changes on [CHANGELOG.md](./CHANGELOG.md)

## Getting Started
To start coding in Sflyn, you can build your first `Hello world!` program:

1. Create a new file called `index.sf` and append to it the following lines:

```sf
print('Hello World!');
```

2. To run the file, use the following command:
```bash
$ sflyn /path/to/index.sf
# Output: Hello world!
```

3. Congratulations! You have created your first code in Sflyn.

## Wiki
You can find more about how to works Sflynlang and its syntax on our [wiki](https://github.com/sflynlang/compiler/wiki).

## Code examples
* [Hello World](./examples/hello_world.sf)
* Classes
  * [Sflyn Class](./examples/classes/Sflyn.sf)
* For
  * [For in an array](./examples/for/array.sf)
  * [For in a hashmap](./examples/for/hashmap.sf)
* Functions
  * [Callback](./examples/functions/callback.sf))
  * [Double Number](./examples/functions/double.sf)
  * [Less or Greater](./examples/functions/less_or_greater.sf)
  * [Say Hi](./examples/functions/say_hi.sf)
* Interfaces
  * [Label](./examples/interfaces/label.sf)
* Modules
  * [Basic](./examples/modules/basic/index.sf)
* Variables
  * [Array](./examples/variables/arrays.sf)
  * [Booleans](./examples/variables/booleans.sf)
  * [Consts](./examples/variables/const.sf)
  * [Numbers](./examples/variables/numbers.sf)
  * [Strings](./examples/variables/strings.sf)

## Social Networks
* [Discord Server](https://discord.gg/XdeRFHt)
* [Twitter (@sflynlang)](https://twitter.com/sflynlang)
* [Facebook (@sflynlang)](https://facebook.com/sflynlang)

## Contributors
* **Daniel Solarte** - Initial Work - [GitHub](https://github.com/danielsolartech)
* **Maria Antonella** - Icon Design - [Instagram](https://www.instagram.com/raccon_324/)
* **LemonCod3** - Emotional Support - [GitHub Organization](https://github.com/LemonCod3)

You can also view the [list of contributors](https://github.com/sflynlang/compiler/contributors) here.

## Licensing
This project is under the MIT License. Please see the [LICENSE](./LICENSE) file for more information.

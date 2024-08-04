# Yew Calculator & Rust Math Parser

Welcome to my mini project! This initiative allowed me to delve into [Yew], a compelling Rust framework that serves as an alternative to React. I chose [Yew] to further my development skills in Rust and explore its potential in building web applications.

Along the way, I decided to create my own math parser from scratch. This involved implementing an [Abstract Syntax Tree][AST] (AST) and utilizing the [Shunting Yard algorithm][SY].

## Math Parser

### Capabilities

The parser provides the following capabilities:

- Tokenizing the input code into a stream of tokens.
- Parsing the token stream to build an AST.
- Checking the syntax and structure of the code.
- Resolving variable and function references.
- Reporting any syntax errors or semantic issues.

It supports the following operators:

- Addition: `+`
- Subtraction: `-`
- Multiplication: `*`
- Division: `/`
- Exponentiation: `^`
- Modulo: `%`

These operators can be used to perform basic arithmetic operations within the math parser. Additionally, parentheses `()` can be used to group expressions and control the order of operations.

Please note that the math parser follows the standard precedence rules for operators, where exponentiation has the highest precedence, followed by multiplication, division, and modulo, and finally addition and subtraction.

### Usage as a library 

This parser is used in the website's code as a library (written in [src/parser][parser]) that is used as follows: 

```rust
use crate::parser::parse;

let str_value = "3*(4+5)";
let result = parse(str_value);
assert_eq!(result, Some(27));
```

## Website

### Browsing it online

This website is available as a GitHub page [here](https://nschiffmacher.github.io/RustMathParser/).

### Running it locally

If you don't already have it installed, it's time to install Rust: <https://www.rust-lang.org/tools/install>.
The rest of this guide assumes a typical Rust installation which contains both `rustup` and Cargo.

To compile Rust to WASM, we need to have the `wasm32-unknown-unknown` target installed.
If you don't already have it, install it with the following command:

```bash
rustup target add wasm32-unknown-unknown
```

Now that we have our basics covered, it's time to install the star of the show: [Trunk].
Simply run the following command to install it:

```bash
cargo install trunk wasm-bindgen-cli
```

That's it, we're done!

### Running

```bash
trunk serve
```

[trunk]: https://github.com/thedodd/trunk
[SY]: https://en.wikipedia.org/wiki/Shunting_yard_algorithm
[AST]: https://en.wikipedia.org/wiki/Abstract_syntax_tree
[Yew]: https://yew.rs
[parser]: https://github.com/NSchiffmacher/RustMathParser/tree/main/src/parser
<h1 align="center">
    <!-- <img src="resources/img/hex.gif" alt="Algorithm" width="192"> -->
    <img src="resources/img/algorithm.png" alt="Algorithm" width="192">
    <div align="center">Logic Tracer</div>
</h1>

[<img alt="github" src="https://img.shields.io/badge/github-Yrrrrrf%2Flogic__tracer-58A6FF?style=for-the-badge&logo=github" height="24">](https://github.com/Yrrrrrf/logic_tracer)
[<img alt="crates.io" src="https://img.shields.io/crates/v/logic__tracer.svg?style=for-the-badge&logo=rust" height="24">](https://crates.io/crates/logic_tracer)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-logic__tracer-66c2a5?style=for-the-badge&labelColor=555555" height="24">](https://docs.rs/logic_tracer)

Logic Traces is a simple crate that reads a logical proposition and interprets it to **build the truth table and the AST of the proposition**.  

Makes use of regular expressions are used to validate the input and to tokenize the proposition.  
Also implements the [Shunting Yard Algorithm](https://en.wikipedia.org/wiki/Shunting-yard_algorithm) (not yet) to build the AST.

## Setup
- Use `cargo build` to build the project
- Use `cargo test` to run the tests
-To run the project, use the command:
```bash
cargo run  # run the project
```

## [Examples](./examples/README.md)

```bash
cargo run --example <example name>
```

## Roadmap
- [x] Validate brackets
- [x] Validate terms (only numbers, variables, variable with subindex)
- [ ] Validate Logical Operators (and, or, not, implies, iff)
- [ ] Read a logical proposition
- [ ] Tokenize the proposition
- [ ] Parse the proposition (validate)
- [ ] Build the AST (Shunting Yard Algorithm) (use postfix notation)
- [ ] **Solve proposition** (evaluate the AST)
- [ ] Add good looking output (truth table, ast, function, etc.)
- [ ] Reduce a proposition to its simplest form (boolean algebra) 

### Bonus (if I have time)
- [ ] Validate mathematical operators (add, sub, mul, div, pow)
- [ ] Add MathPropositions support (examples, tests, etc.)

----

## [License](LICENSE)

This project is licensed under the terms of the [MIT license](./LICENSE)

`note for me: use cargo publish to publish the crate`

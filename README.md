<h1 align="center">
  <img src="https://raw.githubusercontent.com/Yrrrrrf/logic_tracer/main/resources/img/framework.png" alt="Axion Icon" width="128" height="128">
  <div align="center">Logic Tracer</div>
</h1>

<div align="center">

[![GitHub: Repo](https://img.shields.io/badge/github-Yrrrrrf%2Flogic__tracer-58A6FF?&logo=github)](https://github.com/Yrrrrrf/logic_tracer)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow)](./LICENSE)

[![Crates.io](https://img.shields.io/crates/v/logic_tracer.svg?logo=rust)](https://crates.io/crates/logic_tracer)
[![Crates.io Downloads](https://img.shields.io/crates/d/logic_tracer)](https://crates.io/crates/logic_tracer)
[![docs.rs](https://img.shields.io/badge/docs.rs-logic__tracer-66c2a5)](https://docs.rs/logic_tracer)

</div>

> A Rust library that reads logical propositions and interprets them to build truth tables and ASTs.

`Logic Tracer` is a Rust library designed to parse and interpret logical propositions, automatically generating truth tables and Abstract Syntax Trees (ASTs) from input expressions. Makes use of regular expressions to validate the input and tokenize the proposition.

## 🚦 Getting Started

<!--
### Quick Start

Here's a minimal example to get you started:

```rust
use logic_tracer::Proposition;

// Create a new proposition
let proposition = Proposition::new("A & B");
// TODO: Add actual usage examples when functionality is implemented
```
-->

### Examples

Check the [examples](./examples/) directory to see how to use the crate.

```bash
cargo run --example <example name>
```

## Features

### Current
- [x] Read a logical proposition
- [x] Tokenize the proposition
- [x] Validate Input Tokens (only valid tokens)
    - [x] Variables (only letters)
    - [x] Logical Operators (and, or, not, implies, iff)
    - [x] Mathematical Operators (add, sub, mul, div, pow)
    - [x] Compounds (only numbers, variables, variable with subindex)

### Planned
- [ ] Parse the proposition (make sure it is valid)
- [ ] Build the AST (Shunting Yard Algorithm) (use postfix notation)
- [ ] Add notation to the AST (prefix, infix, postfix)
- [ ] **Solve proposition** (evaluate the AST)
- [ ] Add good looking output for truth table, ast, function, etc.
- [ ] Reduce a proposition to its simplest form (boolean algebra) 
    - [ ] Reagrupate Compounds (boolean algebra)
    - [ ] Apply De Morgan's laws (boolean algebra)
    - [ ] Apply distributive, associative and commutative laws (boolean algebra)
- [ ] Add more examples
- [ ] Add more tests
- [ ] Improve documentation
- [ ] Improve the import/export of the crate (to improve it's usability as a library)

<!-- ### Considerations
- [ ] Add some parallel computation (to improve the performance) -->

<!-- ### Future
- [ ] Add complex math iteration (sums, products, etc.) (to be able to solve more complex propositions)
    - [ ] Test a proposition with complex math iteration
- [ ] Add combinational logic (multiplexers, decoders, etc.)
- [ ] Add sequential logic (flip-flops, registers, etc.)
- [ ] Add some geometric demonstration (to be able to solve geometric problems)
$$
\sum_{i=1}^{n} i = \frac{n(n+1)}{2}
$$ -->

> `I want to make this crate completely independent of any other crate.`
`So, I will implement the parser myself...`

> Check [PEST](https://docs.rs/pest/latest/pest/) [grammar declaration using `.pest` files](https://docs.rs/pest/latest/pest/index.html#pest-files)

## 📄 License

This project is licensed under the [**MIT License**](./LICENSE).
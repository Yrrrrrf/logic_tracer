<h1 align="center">
    <img src="resources/img/algorithm.png" alt="Algorithm" width="192">
    <div align="center">Logic Tracer</div>
</h1>

Logic Traces is a simple crate that reads a logical proposition and interprets it to **build the truth table and the AST of the proposition**.  

Makes use of regular expressions are used to validate the input and to tokenize the proposition.  
Also implements the [Shunting Yard Algorithm](https://en.wikipedia.org/wiki/Shunting-yard_algorithm) (not yet) to build the AST.

## Setup

- Use `cargo build src/main.rs` to build the project.  
To run the project, use the command:

```bash
cargo run src/main.rs  # run main (for testing build)
cargo run  # run main
```

## Examples

```bash
cargo run --example <example name>
# examples list:
# - parser
# - regex
# - ast
# - kmap
# - truth_table
# - shunting_yard
# - logic_operations
```

## [License](LICENSE)

This project is licensed under the terms of the [MIT license](./LICENSE)

## References
- [iced gui calculator](https://codinginformer.com/blog/rust-iced-calculator-tutorial)

## Roadmap
- [ ] Read a logical proposition
- [ ] Validate brackets
- [ ] Tokenize the proposition
- [ ] Parse the proposition
- [ ] Validate the proposition
- [ ] **Solve proposition** (Shunting Yard Algorithm)
- [ ] Build the truth table
- [ ] Build the AST (parse tree)
- [ ] Reduce a proposition to its simplest form (boolean algebra) 

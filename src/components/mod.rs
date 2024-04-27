// * Primitive Token Types
// todo: Create some usage for the operators & numbers modules...
// todo: Because they're primitive, they can be used to build more complex types...
pub mod operators;  // operators of the language (LogicOp, MathOp, RelOp, ...)
pub mod numbers;  // numbers of the language (Natural, Integer, Real, ...)
 // pub mod variables;  // variables of the language (Variable, ...)


// * Compiler Components
// * Lexer (Lexical Analysis) -> TokenStream
#[macro_export]
pub mod lexer;

// * Parser (Syntactic Analysis) -> AST
// todo: Implement a simple propositional logic language...
// Parser also includes the grammar rules of the language
// #[macro_export]
// pub mod parser;

// * Semantic Analysis -> Type checking AST
// #[macro_export]
// pub mod semantic;


// todo: Implement a simple propositional logic language...
// * Proposition
// pub mod proposition;  // proposition builder (Proposition)

// * Primitive Token types
pub mod tokens;


// * Compiler Components
// * Lexer (Lexical Analysis) -> TokenStream
#[macro_export]
pub mod lexer;

// * Parser (Syntactic Analysis) -> AST
// * It includes the grammar rules of the language
// todo: Implement a simple propositional logic language...
#[macro_export]
pub mod parser;

// * Semantic Analysis -> Type checking AST
// #[macro_export]
// pub mod semantic;




// * Proposition
// pub mod proposition;  // proposition builder (Proposition)
